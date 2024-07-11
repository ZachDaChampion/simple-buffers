use convert_case::Casing;
use indent::indent_by;
use indoc::formatdoc;

use itertools::Itertools;

use crate::annotate::CppOneOf;
use crate::annotate::CppSchema;
use crate::annotate::CppSequence;
use crate::annotate::ToReaderWriterString;
use crate::argparse::CppGeneratorParams;

//                                                                                                //
// ======================================= Main Function ======================================== //
//                                                                                                //

/// Generates a source file from a given schema.
///
/// # Arguments
///
/// * `params` - Generator params.
/// * `schema` - The schema to generate from.
///
/// # Returns
///
/// The code for a source file, as a String.
pub(crate) fn generate_source(params: &CppGeneratorParams, schema: &CppSchema) -> String {
    // Generate the name of the associated header file.
    let header_name = format!("{}.hpp", params.global.file_name);

    // Generate namespace name.
    let namespace = format!("simplebuffers_{}", params.global.file_name).to_case(params.ns_case);

    // Generate full implementations for sequence writers.
    let sequence_writers = schema
        .sequences
        .iter()
        .map(|s| impl_sequence_writer(params, s))
        .join("\n\n");

    // Generate the full source file.
    formatdoc! {
        r#"
        #include "{header_name}"

        namespace simplebuffers = _sb;
        namespace {namespace} = _sbs;

        {sequence_writers}"#
    }
    .replace("\n\n\n", "\n")
}

//                                                                                                //
// ================================== Generate File Components ================================== //
//                                                                                                //

/// Generates the C++ code for implementing a sequence.
fn impl_sequence_writer(params: &CppGeneratorParams, sequence: &CppSequence) -> String {
    // The full name of the sequence writer class, in the form "SequenceWriter" (formatted for
    // casing preference).
    let class_name = sequence.to_writer_string(params);

    // Comment that indicates the beginning of this sequence's section.
    let section_comment = section_comment(&class_name);

    // Generate the parameter list for the constructor.
    let param_list = sequence
        .fields
        .iter()
        .map(|f| format!("{} {}", f.ty.to_writer_string(params), f.name))
        .join(", ");

    // Generate the initialization list for the constructor.
    let init_list = sequence
        .fields
        .iter()
        .map(|f| format!("{}({})", f.name, f.name))
        .join(", ");

    // Get the size of the sequence's static data.
    let static_size = sequence.size;

    // Generate code that writes fields to the buffer.
    let write_fields = sequence
        .fields
        .iter()
        .enumerate()
        .map(|(i, f)| {
            if i < sequence.fields.len() - 1 {
                // Not last field, increment dest.
                formatdoc! {r"
                    dyn_cursor = __sb::write_field(dest, dest_end, dyn_cursor, {cast});
                    if (dyn_cursor == nullptr) return nullptr;
                    dest += __sb::get_static_size({cast});",
                    cast = f.cast()
                }
            } else {
                // Last field, don't need to increment dest.
                formatdoc! {r"
                    dyn_cursor = __sb::write_field(dest, dest_end, dyn_cursor, {cast});
                    if (dyn_cursor == nullptr) return nullptr;",
                    cast = f.cast()
                }
            }
        })
        .join("\n");

    // Generate all implementation code for oneof fields in this sequence.
    let oneofs = generate_oneofs(params, sequence);

    // Generate sequence code.
    // TODO: Find out if we should be comparing to `static_size` or `static_size - 1`.
    formatdoc! {
        r"
        {section_comment}

        {class_name}::{class_name}({param_list}):
            {init_list} {{}}

        uint16_t {class_name}::static_size() const {{ return {static_size}; }}
        
        uint8_t* {class_name}::write_component(uint8_t* dest, const uint8_t* dest_end,
                                 uint8_t* dyn_cursor) const {{
            if (dest_end - dest < {static_size}) return nullptr;
            {write_fields}
            return dyn_cursor;
        }}
        
        {oneofs}",
        write_fields = indent_by(4, write_fields)
    }
}

/// Implements a visitor pattern for oneof fields. Each oneof is first visited. In this step,
/// code is generated for it and its name is pushed to the namespace stack. Later, when all of
/// the oneof's oneof fields are fully processed, its name is popped from the namespace stack.
///
/// # Returns
///
/// A string with code for all of the sequence's oneof fields.
fn generate_oneofs(params: &CppGeneratorParams, sequence: &CppSequence) -> String {
    enum Visitor<'a> {
        Visit(&'a CppOneOf),
        PopName,
    }

    let mut generated = String::new();
    let mut name_stack = vec![sequence.to_writer_string(params)];
    let mut visit_stack = sequence.oneofs().rev().map(Visitor::Visit).collect_vec();

    while let Some(visit) = visit_stack.pop() {
        match visit {
            Visitor::Visit(oneof) => {
                name_stack.push(oneof.to_writer_string(params));
                let full_name = name_stack.join("::");
                generated += &format!("{}\n\n", section_comment(&full_name));
                generated += &format!("{}\n\n", visit_oneof(params, oneof, &name_stack));
                visit_stack.push(Visitor::PopName);
                for sub_oneof in oneof.oneofs().rev() {
                    visit_stack.push(Visitor::Visit(sub_oneof));
                }
            }
            Visitor::PopName => {
                name_stack.pop();
                generated += "\n";
            }
        }
    }

    generated
}

fn visit_oneof(params: &CppGeneratorParams, oneof: &CppOneOf, name_stack: &[String]) -> String {
    // The full name of the oneof writer class, in the form "namespace::SequenceWriter" (formatted
    // for casing preference).
    let class_name = name_stack
        .last()
        .expect("C++ name stack unexpectedly empty");
    let full_class_name = name_stack.join("::");

    let public_constructors = oneof
        .fields
        .iter()
        .map(|f| {
            formatdoc! {"
                {full_class_name} {full_class_name}::{constructor}({field_type}* val) {{
                    Value v;
                    v.{name} = val;
                    return {class_name}(Tag::{tag}, v);
                }}",
                constructor = f.constructor,
                field_type = f.ty.to_writer_string(params),
                name = f.name,
                tag = f.tag
            }
        })
        .join("\n\n");

    // Switch cases for use in `write_component`.
    let switch_cases = oneof
        .fields
        .iter()
        .map(|f| {
            formatdoc! {"
                case Tag::{tag}:
                    return __sb::write_oneof_field(dest, dest_end, dyn_cursor, {index}, *value.{name});",
                tag = f.tag,
                index = f.index,
                name = f.name
            }
        })
        .join("\n");

    formatdoc! {r"
        {public_constructors}
        
        uint8_t* {full_class_name}::write_component(uint8_t* dest, const uint8_t* dest_end,
                                 uint8_t* dyn_cursor) const {{
            switch (tag) {{
                {switch_cases}
                default:
                    return nullptr;
            }}
        }}
        
        {full_class_name}::{class_name}(Tag tag, Value value) : tag(tag), value(value) {{}}",
        public_constructors = public_constructors,
        switch_cases = indent_by(8, switch_cases),
    }
}

/// Generates a comment that indicates a section of code.
///
/// Generated comments look like:
///
/// ```cpp
/// /*
///  * <label>
///  */
/// ```
fn section_comment(label: &str) -> String {
    formatdoc! {
        r"
        /*
         * {label}
         */"
    }
}
