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
    let enum_definitions = schema
        .enums
        .iter()
        .map(|e| define_enum(params, e))
        .join("\n\n");

    let forward_declarations = schema
        .sequences
        .iter()
        .map(|s| forward_declare_sequence_writer(params, s))
        .join("\n");

    let sequence_definitions = schema
        .sequences
        .iter()
        .map(|s| define_sequence_writer(params, s))
        .join("\n\n");

    formatdoc! {
        r"
            {header}

            {enums}

            {forward_declarations}

            {sequences}

            {footer}",
        header = file_header(params, true),
        enums = enum_definitions,
        forward_declarations = forward_declarations,
        sequences = sequence_definitions,
        footer = file_footer(params),
    }
    .replace("\n\n\n", "\n")
}

/// Generates the code that goes at the top of the header file. This contains include guards,
/// file imports, and namespace declarations.
fn file_header(params: &CppGeneratorParams, hpp: bool) -> String {
    let include_guards = format!(
        "SIMPLEBUFFERS_GENERATED__{file_name}_{suffix}",
        file_name = params.global.file_name.to_case(Case::UpperSnake),
        suffix = if hpp { "HPP" } else { "H" }
    );

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
    let variants = data
        .variants
        .iter()
        .map(|v| format!("{} = {}", v.0, v.1))
        .join(",\n");

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
    let case_corrected_name = format!("{}_Writer", seq.name).to_case(params.class_case);
    format!("class {};", case_corrected_name)
}

/// Generates the C++ code for defining a sequence writer.
fn define_sequence_writer(params: &CppGeneratorParams, seq: &CppSequence) -> String {
    let class_name = to_writer_string(seq.name.as_str(), params);

    let param_list = seq
        .fields
        .iter()
        .map(|f| format!("{} {}", f.ty.to_writer_string(params), f.name))
        .join(", ");

    let oneofs = seq
        .fields
        .iter()
        .filter_map(|f| match &f.ty {
            CppType::OneOf(o) => Some(define_oneof_writer(params, o)),
            _ => None,
        })
        .join("\n\n");

    let members = seq
        .fields
        .iter()
        .map(|f| format!("{} {};", f.ty.to_writer_string(params), f.name))
        .join("\n");

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
/// sequence writer.
fn define_oneof_writer(params: &CppGeneratorParams, oneof: &CppOneOf) -> String {
    let class_name = to_writer_string(oneof.name.as_str(), params);

    let oneofs = oneof
        .fields
        .iter()
        .filter_map(|f| match &f.ty {
            CppType::OneOf(o) => Some(define_oneof_writer(params, o)),
            _ => None,
        })
        .join("\n\n");

    let tags = oneof
        .fields
        .iter()
        .enumerate()
        .map(|f| format!("{} = {}", f.1.name.to_case(params.enum_var_case), f.0))
        .join(",\n");

    let values = oneof
        .fields
        .iter()
        .map(|f| format!("{}* {};", f.ty.to_writer_string(params), f.name))
        .join("\n");

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

    let private_body = formatdoc! {
        r"
            {class_name}(Tag tag, Value value);
            
            Tag tag;
            Value value;",
        class_name = class_name
    };

    formatdoc! {
            r"
            class {class_name} : public OneOfWriter {{
               public:
                {public_body}

               private:
                {private_body}
            }};",
            class_name = class_name,
            public_body = indent_by(4, public_body.trim()),
            private_body = indent_by(4, private_body)
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

/// Turns a sequence name into a SimpleBufferWriter name, following the casing set in params.
fn to_writer_string(name: &str, params: &CppGeneratorParams) -> String {
    format!("{}_Writer", name).to_case(params.class_case)
}
