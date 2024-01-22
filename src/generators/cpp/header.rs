//! This file contains the code for generating C++ header files.

use indoc::{formatdoc, indoc};

use crate::{
    compiler::{Field, Sequence},
    generators::cpp::primitive_to_cpp_type,
};

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

/// Returns the code for a sequence.
fn sequence(sequence: Sequence) -> String {
    let mut result = formatdoc! {r#"
        class {name} {{
           public:
        "#,
        name = sequence.name
    };

    todo!("Add fields");

    result
}

/// Returns the code for a field.
fn field(field: Field) -> String {
    use crate::compiler::Type as T;
    match field.ty {
        T::Primitive(primitive) => format!(
            "{ty} read_{name}() const;",
            ty = primitive_to_cpp_type(primitive),
            name = field.name
        ),
        _ => todo!("Add support for non-primitive types"),
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
