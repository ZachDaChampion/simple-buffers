//! This file contains the code for generating C++ header files.

use indoc::{formatdoc, indoc};
use itertools::Itertools;

use crate::compiler::{Enum, Field, ParseResult, Sequence, Type};

/// Returns the code that goes at the top of a header file.
fn file_header(file_name: String, hpp: bool) -> String {
    let include_guards = format!(
        "SIMPLEBUFFER_GENERATED__{file_name}_{suffix}",
        file_name = file_name.to_uppercase(),
        suffix = if hpp { "HPP" } else { "H" }
    );

    formatdoc! {r#"
            #ifndef {include_guards}
            #define {include_guards}

            #include "simplebuffers.hpp"

            namespace simplebuffers_{file_name} {{
            "#,
        include_guards = include_guards,
        file_name = file_name
    }
}

/// Returns the code that goes at the bottom of a header file.
fn file_footer() -> String {
    indoc! {r"
            }
            
            #endif
        "}
    .to_string()
}

/// Generate the C++ code for defining an enum.
fn define_enum(data: &Enum) -> String {
    formatdoc! {r#"
        enum class {name} {{
            {variants}
        }};
        "#,
    name = data.name,
    variants = data
        .variants
        .iter()
        .map(|v| format!("{} = {}", v.name, v.value))
        .join(",\n")
    }
}

/// Generate the C++ code for sequence writer forward declarations.
fn forward_declare_writer(data: &Sequence) -> String {
    format!("class {}Writer : public SimpleBufferWriter;", data.name)
}

/// Generate the code for a C++ header file.
pub fn generate(file_name: String, data: ParseResult, hpp: bool) -> String {
    let enums = data.enums.iter().map(define_enum).join("\n");
    let writer_forward_declarations = data.sequences.iter().map(forward_declare_writer).join("\n");

    todo!("Define classes");
    let member_specifications = "";

    formatdoc! {r#"
        {header}
        {enums}
        {forward_declarations}
        {member_specifications}
        {footer}
    "#,
    header = file_header(file_name, hpp),
    enums = enums,
    forward_declarations = writer_forward_declarations,
    member_specifications = member_specifications,
    footer = file_footer()
    }
}
