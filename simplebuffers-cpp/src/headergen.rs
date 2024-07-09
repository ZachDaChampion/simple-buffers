use convert_case::{Case, Casing};
use indent::indent_by;
use indoc::formatdoc;

use itertools::Itertools;

use crate::annotate::CppEnum;
use crate::annotate::CppOneOf;
use crate::annotate::CppSchema;
use crate::annotate::CppSequence;
use crate::annotate::CppType;
use crate::annotate::ToReaderWriterString;
use crate::argparse::CppGeneratorParams;

/// Generates a header file from a given schema.
///
/// # Arguments
///
/// * `params` - Generator params.
/// * `schema` - The schema to generate from.
///
/// # Returns
///
/// The code for a header file, as a String.
pub(crate) fn generate_header(params: &CppGeneratorParams, schema: &CppSchema) -> String {
    // Generate enum definitions.
    let enum_definitions = schema
        .enums
        .iter()
        .map(|e| define_enum(params, e))
        .join("\n\n");

    // Generate forward declarations for sequence writers.
    let writer_forward_declarations = schema
        .sequences
        .iter()
        .map(|s| forward_declare_sequence_writer(params, s))
        .join("\n");

    // Generate full descriptions for sequence writers.
    let sequence_writer_definitions = schema
        .sequences
        .iter()
        .map(|s| define_sequence_writer(params, s))
        .join("\n\n");

    // Generate the full header file.
    formatdoc! {
        r"
            {header}

            {enums}

            {writer_forward_declarations}

            {sequence_writers}

            {footer}",
        header = file_header(params, true),
        enums = enum_definitions,
        writer_forward_declarations = writer_forward_declarations,
        sequence_writers = sequence_writer_definitions,
        footer = file_footer(params),
    }
    .replace("\n\n\n", "\n")
}

/// Generates the code that goes at the top of the header file. This contains include guards,
/// file imports, and namespace declarations.
fn file_header(params: &CppGeneratorParams, hpp: bool) -> String {
    // Generate the name of the include guards.
    let include_guards = format!(
        "SIMPLEBUFFERS_GENERATED__{file_name}_{suffix}",
        file_name = params.global.file_name.to_case(Case::UpperSnake),
        suffix = if hpp { "HPP" } else { "H" }
    );

    // Generate the full file header code.
    formatdoc! {
        r#"
            #ifndef {include_guards}
            #define {include_guards}

            #include "simplebuffers.hpp"

            namespace {ns} {{"#,
        include_guards = include_guards,
        ns = format!("simplebuffers_{}", params.global.file_name).to_case(params.ns_case)
    }
}

/// Generates the C++ code for defining an enum.
fn define_enum(_params: &CppGeneratorParams, data: &CppEnum) -> String {
    // Generate the code for defining individual enum variants.
    let variants = data
        .variants
        .iter()
        .map(|v| format!("{} = {}", v.0, v.1))
        .join(",\n");

    // Generate the full enum code.
    formatdoc! {
        r"
            enum class {name} : {dtype} {{
                {variants}
            }};",
        name = data.name,
        dtype = match data.size {
            1 => "uint_fast8_t",
            2 => "uint_fast16_t",
            4 => "uint_fast32_t",
            8 => "uint_fast64_t",
            _ => panic!("Invalid size {} for enum {}", data.size, data.name),
        },
        variants = indent_by(4, variants)
    }
}

/// Generates the C++ code for forward declaring sequence writers.
fn forward_declare_sequence_writer(params: &CppGeneratorParams, seq: &CppSequence) -> String {
    let case_corrected_name = seq.to_writer_string(params);
    format!("class {};", case_corrected_name)
}

/// Generates the C++ code for defining a sequence writer.
fn define_sequence_writer(params: &CppGeneratorParams, seq: &CppSequence) -> String {
    // The full name of the sequence writer class, in the form "SequenceWriter" (formatted for
    // casing preference).
    let class_name = seq.to_writer_string(params);

    // Generate the parameter list for the constructor.
    let param_list = seq
        .fields
        .iter()
        .map(|f| format!("{} {}", f.ty.to_writer_string(params), f.name))
        .join(", ");

    // Generate class definitions of any oneof fields contained in the sequence. These are
    // subclasses of this sequence class.
    let oneofs = seq
        .fields
        .iter()
        .filter_map(|f| match &f.ty {
            CppType::OneOf(o) => Some(define_oneof_writer(params, o)),
            _ => None,
        })
        .join("\n\n");

    // Generate member declarations for all fields.
    let members = seq
        .fields
        .iter()
        .map(|f| format!("{} {};", f.ty.to_writer_string(params), f.name))
        .join("\n");

    // Generate class body. We do this separately from the final class generation so that we can
    // trim the body and remove the extra whitespace present when there are no oneof fields.
    let body = formatdoc! {
        r"
            {oneofs}

            {class_name}({param_list});

            {members}

            uint16_t static_size() const override;
            uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                                     uint8_t* dyn_cursor) const override;",
        class_name = class_name,
        oneofs = oneofs,
        param_list = param_list,
        members = members
    };

    // Generate full class code.
    formatdoc! {
        r"
            class {class_name} : public SimpleBufferWriter {{
               public:
                {body}
            }};",
        class_name = class_name,
        body = indent_by(4, body.trim())
    }
}

/// Generates the C++ code for defining a oneof writer. This should be written as a subclass of a
/// sequence writer. Because oneofs can contain other oneofs as fields, we must recursively define
/// any oneof writers we find.
fn define_oneof_writer(params: &CppGeneratorParams, oneof: &CppOneOf) -> String {
    // The full name of the oneof writer class, in the form "OneOfWriter" (formatted for casing
    // preference).
    let class_name = oneof.to_writer_string(params);

    // Generate class definitions of any oneof fields. These are subclasses of this oneof class and
    // are generated recursively.
    let oneofs = oneof
        .fields
        .iter()
        .filter_map(|f| match &f.ty {
            CppType::OneOf(o) => Some(define_oneof_writer(params, o)),
            _ => None,
        })
        .join("\n\n");

    // Generate a list of tags for the fields. These are members of the `Tag` enum class.
    let tags = oneof
        .fields
        .iter()
        .map(|f| format!("{} = {}", f.name.to_case(params.enum_var_case), f.index))
        .join(",\n");

    // Generate a list of possible values for each oneof field. These are held in a union and are
    // stored as pointers.
    let values = oneof
        .fields
        .iter()
        .map(|f| format!("{}* {};", f.ty.to_writer_string(params), f.name))
        .join("\n");

    // Generate static "constructors" for this oneof class. There is one constructor for each field.
    // The true class constructor is private, so one of these must be called to instantiate a class
    // instance.
    let constructors = oneof
        .fields
        .iter()
        .map(|f| {
            format!(
                "static {} {}({}* val);",
                class_name,
                f.name.to_case(params.class_case),
                f.ty.to_writer_string(params)
            )
        })
        .join("\n");

    // Generate the public portion of the body. This is done separately so that we can trim it and
    // remove the extra white space generated when there are no oneof fields.
    let public_body = formatdoc! {
        r"
            {oneofs}

            enum class Tag : uint8_t {{
                {tags}
            }};
            
            union Value {{
                {values}
            }};
            
            {constructors}

            uint8_t* write_component(uint8_t* dest, const uint8_t* dest_end,
                                     uint8_t* dyn_cursor) const override;",
        oneofs = indent_by(4, oneofs),
        tags = indent_by(4, tags),
        values = indent_by(4, values),
        constructors = constructors
    };

    // Generate full class code.
    formatdoc! {
            r"
            class {class_name} : public OneOfWriter {{
               public:
                {public_body}

               private:
                {class_name}(Tag tag, Value value);
            
                Tag tag;
                Value value;
            }};",
            class_name = class_name,
            public_body = indent_by(4, public_body.trim()),
    }
}

/// Generates the code that goes at the bottom of the header file. This contains the closing
/// brace for the namespace and closes the include guards.
fn file_footer(params: &CppGeneratorParams) -> String {
    formatdoc! {
        r"
            }} // namespace {ns}

            #endif",
        ns = format!("simplebuffers_{}", params.global.file_name).to_case(params.ns_case)
    }
    .to_string()
}
