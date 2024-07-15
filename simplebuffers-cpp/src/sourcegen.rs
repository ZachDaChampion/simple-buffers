//! Generates a C++ source file.

use crate::annotate::CppOneOf;
use crate::annotate::CppOneOfField;
use crate::annotate::CppSchema;
use crate::annotate::CppSequence;
use crate::annotate::CppSequenceField;
use crate::annotate::CppType;
use crate::annotate::SizeToType;
use crate::annotate::ToReaderWriterString;
use crate::argparse::CppGeneratorParams;
use indent::indent_by;
use indoc::formatdoc;
use itertools::Itertools;

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
    let namespace = format!("simplebuffers_{}", params.global.file_name);

    // Generate full implementations for sequence writers.
    let sequence_writers = schema
        .sequences
        .iter()
        .map(impl_sequence_writer)
        .join("\n\n");

    let sequence_readers = schema
        .sequences
        .iter()
        .map(impl_sequence_reader)
        .join("\n\n");

    // Generate the full source file.
    formatdoc! {
        r#"
        #include "{header_name}"

        namespace {namespace} {{

        {sequence_writers}

        {sequence_readers}
        
        }} // namespace {namespace}"#
    }
    .replace("\n\n\n", "\n")
}

//                                                                                                //
// ================================= Generate Writer Components ================================= //
//                                                                                                //

/// Generates the C++ code for implementing a sequence writer.
fn impl_sequence_writer(sequence: &CppSequence) -> String {
    // The full name of the sequence writer class, in the form "SequenceWriter".
    let class_name = sequence.to_writer_string();

    // Comment that indicates the beginning of this sequence's section.
    let section_comment = section_comment(&class_name);

    // Generate the parameter list for the constructor.
    let param_list = sequence
        .fields
        .iter()
        .map(|f| format!("{} {}", f.ty.to_writer_string(), f.name))
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
                    dyn_cursor = simplebuffers::write_field(dest, dest_end, dyn_cursor, {cast});
                    if (dyn_cursor == nullptr) return nullptr;
                    dest += simplebuffers::get_static_size({cast});",
                    cast = f.cast()
                }
            } else {
                // Last field, don't need to increment dest.
                formatdoc! {r"
                    dyn_cursor = simplebuffers::write_field(dest, dest_end, dyn_cursor, {cast});
                    if (dyn_cursor == nullptr) return nullptr;",
                    cast = f.cast()
                }
            }
        })
        .join("\n");

    // Generate all implementation code for oneof fields in this sequence.
    let oneofs = impl_oneof_writers(sequence);

    // Generate sequence code.
    // TODO: Find out if we should be comparing to `static_size` or `static_size - 1`.
    formatdoc! {
        r"
        {section_comment}

        {class_name}::{class_name}({param_list}):
            {init_list} {{}}

        uint16_t {class_name}::static_size() const {{ return {static_size}; }}
        
        uint8_t* {class_name}::write_component(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor) const {{
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
fn impl_oneof_writers(sequence: &CppSequence) -> String {
    enum Visitor<'a> {
        Visit(&'a CppOneOf),
        PopName,
    }

    let mut generated = String::new();
    let mut name_stack = vec![sequence.to_writer_string()];
    let mut visit_stack = sequence.oneofs().rev().map(Visitor::Visit).collect_vec();

    while let Some(visit) = visit_stack.pop() {
        match visit {
            Visitor::Visit(oneof) => {
                name_stack.push(oneof.to_writer_string());
                let full_name = name_stack.join("::");
                generated += &format!("{}\n\n", section_comment(&full_name));
                generated += &format!("{}\n\n", visit_oneof_writer(oneof, &name_stack));
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

fn visit_oneof_writer(oneof: &CppOneOf, name_stack: &[String]) -> String {
    // The full name of the oneof writer class, in the form "namespace::SequenceWriter".
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
                field_type = f.ty.to_writer_string(),
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
                    return simplebuffers::write_oneof_field(dest, dest_end, dyn_cursor, {index}, {write_cast});",
                tag = f.tag,
                index = f.index,
                write_cast = f.write_cast()
            }
        })
        .join("\n");

    formatdoc! {r"
        {public_constructors}
        
        uint8_t* {full_class_name}::write_component(uint8_t* dest, const uint8_t* dest_end, uint8_t* dyn_cursor) const {{
            switch (tag_) {{
                {switch_cases}
                default:
                    return nullptr;
            }}
        }}
        
        {full_class_name}::{class_name}(Tag tag, Value value) : tag_(tag), value_(value) {{}}",
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

//                                                                                                //
// ================================= Generate Reader Components ================================= //
//                                                                                                //

/// Generates the C++ code for implementing a sequence reader.
fn impl_sequence_reader(sequence: &CppSequence) -> String {
    // The full name of the sequence reader class, in the form "SequenceReader".
    let class_name = sequence.to_reader_string();

    // Comment that indicates the beginning of this sequence's section.
    let section_comment = section_comment(&class_name);

    // Get the size of the sequence's static data.
    let static_size = sequence.size;

    // Generate code that writes fields to the buffer.
    let field_accessors = sequence
        .fields
        .iter()
        .map(|f| impl_sequence_field_reader(sequence.to_reader_string().as_str(), f))
        .join("\n");

    // Generate all implementation code for oneof fields in this sequence.
    let oneofs = impl_oneof_readers(sequence);

    // Generate sequence code.
    // TODO: Find out if we should be comparing to `static_size` or `static_size - 1`.
    formatdoc! {
        r"
        {section_comment}

        {class_name}::{class_name}(const uint8_t* data_ptr, size_t idx) : simplebuffers::SimpleBufferReader(data_ptr + {static_size} * idx) {{}}

        uint16_t {class_name}::static_size() const {{ return {static_size}; }}
        
        {field_accessors}
        
        {oneofs}",
    }
}

fn impl_sequence_field_reader(namespace: &str, field: &CppSequenceField) -> String {
    // Position of the field in the sequence.
    let pos = field.pos;

    // Name of the field.
    let name = field.name.as_str();

    // Read-formatted name of the field's type.
    let type_name = field.ty.to_reader_string();

    // Function to execute to access the data.
    match &field.ty {
        CppType::Primitive(p) => {
            formatdoc! {
                r"
                {type_name} {namespace}::{name}() const {{
                    return simplebuffers::read_field<{p}>(data_ptr_ + {pos});
                }}"
            }
        }

        CppType::Sequence(_) => {
            formatdoc! {
                r"
                {type_name} {namespace}::{name}() const {{
                    return {type_name}(data_ptr_ + {pos}, 0);
                }}"
            }
        }

        CppType::Enum(_, size) => {
            let dtype = size.to_type();
            formatdoc! {
                r"
                {type_name} {namespace}::{name}() const {{
                    return static_cast<{type_name}>(simplebuffers::read_field<{dtype}>(data_ptr_ + {pos}));
                }}"
            }
        }

        CppType::Array(t) => {
            let template_type = match t.as_ref() {
                CppType::OneOf(o) => {
                    format!(
                        "{namespace}::{oneof_name}",
                        oneof_name = o.to_reader_string()
                    )
                }
                CppType::Enum(_, size) => {
                    format!(
                        "{ret}, {read}",
                        ret = t.to_reader_string(),
                        read = size.to_type()
                    )
                }
                _ => t.to_reader_string(),
            };
            formatdoc! {
                r"
                simplebuffers::ListReader<{template_type}> {namespace}::{name}() const {{
                    return simplebuffers::ListReader<{template_type}>(static_cast<const uint8_t*>(data_ptr_ + {pos}), 0);
                }}"
            }
        }

        CppType::OneOf(o) => {
            let full_type_name = format!(
                "{namespace}::{oneof_name}",
                oneof_name = o.to_reader_string()
            );
            formatdoc! {
                r"
                {full_type_name} {namespace}::{name}() const {{
                    return {type_name}(static_cast<const uint8_t*>(data_ptr_ + {pos}), 0);
                }}"
            }
        }
    }
}

fn impl_oneof_readers(sequence: &CppSequence) -> String {
    enum Visitor<'a> {
        Visit(&'a CppOneOf),
        PopName,
    }

    let mut generated = String::new();
    let mut name_stack = vec![sequence.to_reader_string()];
    let mut visit_stack = sequence.oneofs().rev().map(Visitor::Visit).collect_vec();

    while let Some(visit) = visit_stack.pop() {
        match visit {
            Visitor::Visit(oneof) => {
                name_stack.push(oneof.to_reader_string());
                let full_name = name_stack.join("::");
                generated += &format!("{}\n\n", section_comment(&full_name));
                generated += &format!("{}\n\n", visit_oneof_reader(oneof, &name_stack));
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

fn visit_oneof_reader(oneof: &CppOneOf, name_stack: &[String]) -> String {
    // The full name of the oneof reader class, in the form "namespace::SequenceReader".
    let class_name = name_stack
        .last()
        .expect("C++ name stack unexpectedly empty");
    let full_class_name = name_stack.join("::");

    let fields = oneof
        .fields
        .iter()
        .map(|f| impl_oneof_field_reader(&full_class_name, f))
        .join("\n\n");

    formatdoc! {r"
        {full_class_name}::{class_name}(const uint8_t* data_ptr, size_t idx) : OneOfReader(data_ptr, idx) {{
            const uint16_t offset = simplebuffers::read_field<uint16_t>(data_ptr + 1);
            tag_ = static_cast<Tag>(simplebuffers::read_field<uint8_t>(data_ptr));
            val_ptr_ = data_ptr + offset;
        }}
        
        {full_class_name}::Tag {full_class_name}::tag() const {{
            return tag_;
        }}
        
        {fields}"
    }
}

fn impl_oneof_field_reader(namespace: &str, field: &CppOneOfField) -> String {
    // Name of the field.
    let name = field.name.as_str();

    // Tag of the field.
    let tag = field.tag.as_str();

    // Read-formatted name of the field's type.
    let type_name = field.ty.to_reader_string();

    // Function to execute to access the data.
    match &field.ty {
        CppType::Primitive(p) => {
            let null_val = match *p {
                "const char*" => "\"\\0\"",
                "bool" => "false",
                _ => "0",
            };
            formatdoc! {
                r"
                {type_name} {namespace}::{name}() const {{
                    if (tag_ != Tag::{tag}) return {null_val};
                    return simplebuffers::read_field<{p}>(val_ptr_);
                }}"
            }
        }

        CppType::Sequence(_) => {
            formatdoc! {
                r"
                {type_name} {namespace}::{name}() const {{
                    return {type_name}(val_ptr_, 0);
                }}"
            }
        }

        CppType::Enum(_, size) => {
            let dtype = size.to_type();
            formatdoc! {
                r"
                {type_name} {namespace}::{name}() const {{
                    if (tag_ != Tag::{tag}) return static_cast<{type_name}>(0);
                    return static_cast<{type_name}>(simplebuffers::read_field<{dtype}>(val_ptr_));
                }}"
            }
        }

        CppType::Array(t) => {
            let template_type = match t.as_ref() {
                CppType::OneOf(o) => {
                    format!(
                        "{namespace}::{oneof_name}",
                        oneof_name = o.to_reader_string()
                    )
                }
                CppType::Enum(_, size) => {
                    format!(
                        "{ret}, {read}",
                        ret = t.to_reader_string(),
                        read = size.to_type()
                    )
                }
                _ => t.to_reader_string(),
            };
            formatdoc! {
                r"
                simplebuffers::ListReader<{template_type}> {namespace}::{name}() const {{
                    if (tag_ != Tag::{tag}) return simplebuffers::ListReader<{template_type}>(nullptr, 0);
                    return simplebuffers::ListReader<{template_type}>(static_cast<const uint8_t*>(val_ptr_, 0);
                }}"
            }
        }

        CppType::OneOf(o) => {
            let full_type_name = format!(
                "{namespace}::{oneof_name}",
                oneof_name = o.to_reader_string()
            );
            formatdoc! {
                r"
                {full_type_name} {namespace}::{name}() const {{
                    if (tag_ != Tag::{tag}) return {type_name}(nullptr, 0);
                    return {type_name}(val_ptr_, 0);
                }}"
            }
        }
    }
}
