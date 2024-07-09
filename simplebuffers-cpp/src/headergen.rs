use convert_case::{Case, Casing};
use indoc::formatdoc;

use itertools::Itertools;

use crate::annotate::CppEnum;
use crate::annotate::CppSchema;
use crate::annotate::CppSequence;
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

    formatdoc! {
        r#"
            {header}
            {enums}
            {forward_declarations}
            {sequences}
            {footer}
        "#,
        header = file_header(params, true),
        enums = enum_definitions,
        forward_declarations = forward_declarations,
        sequences = "",
        footer = file_footer(params),
    }
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

            namespace {ns} {{
        "#,
        include_guards = include_guards,
        ns = format!("simplebuffers_{}", params.global.file_name).to_case(params.ns_case)
    }
}

/// Generates the C++ code for defining an enum.
fn define_enum(_params: &CppGeneratorParams, data: &CppEnum) -> String {
    formatdoc! {
        r#"
            enum class {name} : {dtype} {{
                {variants}
            }};
        "#,
        name = data.name,
        dtype = match data.size {
            1 => "uint_fast8_t",
            2 => "uint_fast16_t",
            4 => "uint_fast32_t",
            8 => "uint_fast64_t",
            _ => panic!("Invalid size {} for enum {}", data.size, data.name),
        },
        variants = data
        .variants
        .iter()
        .map(|v| format!("{} = {}", v.0, v.1))
        .join(",\n    ")
    }
}

/// Generates the C++ code for forward declaring sequence writers.
fn forward_declare_sequence_writer(params: &CppGeneratorParams, seq: &CppSequence) -> String {
    let case_corrected_name = format!("{}_Writer", seq.name).to_case(params.class_case);
    format!("class {} : public SimpleBufferWriter;", case_corrected_name)
}

/// Generates the code that goes at the bottom of the header file. This contains the closing
/// brace for the namespace and closes the include guards.
fn file_footer(params: &CppGeneratorParams) -> String {
    formatdoc! {
        r"
            }} // namespace {ns}

            #endif
        ",
        ns = format!("simplebuffers_{}", params.global.file_name).to_case(params.ns_case)
    }
    .to_string()
}
