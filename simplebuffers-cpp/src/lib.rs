mod annotate;
mod argparse;
mod headergen;
mod sourcegen;

use annotate::annotate_schema;
use argparse::parse_args;
use headergen::generate_header;
use simplebuffers_codegen::CodeGenerator;
use sourcegen::generate_source;

#[derive(Debug)]
pub struct CPPCodeGenerator;

impl CodeGenerator for CPPCodeGenerator {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn generate(
        &mut self,
        schema: &simplebuffers_core::SBSchema,
        params: &simplebuffers_codegen::GeneratorParams,
    ) -> Result<(), String> {
        let generator_params = parse_args(params);
        println!("C++ Generator! Args: {:?}", generator_params);

        let annotated = annotate_schema(&generator_params, schema);
        println!("\n\nAnnotated:\n\n{:?}", annotated);

        let header = generate_header(&generator_params, &annotated);
        println!("\n\nHeader:\n\n{}", header);

        let source = generate_source(&generator_params, &annotated);
        println!("\n\nSource:\n\n{}", source);

        Ok(())
    }

    fn reserved_identifiers(
        &mut self,
        _params: &simplebuffers_codegen::GeneratorParams,
    ) -> Vec<String> {
        vec![
            "__sb",
            "__sbs",
            "alignas",
            "alignof",
            "and",
            "and_eq",
            "asm",
            "atomic_cancel",
            "atomic_commit",
            "atomic_noexcept",
            "auto",
            "bitand",
            "bitor",
            "bool",
            "break",
            "case",
            "catch",
            "char",
            "char8_t",
            "char16_t",
            "char32_t",
            "class",
            "compl",
            "concept",
            "const",
            "consteval",
            "constexpr",
            "constinit",
            "const_cast",
            "continue",
            "co_await",
            "co_return",
            "co_yield",
            "decltype",
            "default",
            "delete",
            "do",
            "double",
            "dynamic_cast",
            "else",
            "enum",
            "explicit",
            "export",
            "extern",
            "false",
            "float",
            "for",
            "friend",
            "goto",
            "if",
            "inline",
            "int",
            "long",
            "mutable",
            "namespace",
            "new",
            "noexcept",
            "not",
            "not_eq",
            "nullptr",
            "operator",
            "or",
            "or_eq",
            "private",
            "protected",
            "public",
            "reflexpr",
            "register",
            "reinterpret_cast",
            "requires",
            "return",
            "short",
            "signed",
            "sizeof",
            "static",
            "static_assert",
            "static_cast",
            "struct",
            "switch",
            "synchronized",
            "template",
            "this",
            "thread_local",
            "throw",
            "true",
            "try",
            "typedef",
            "typeid",
            "typename",
            "union",
            "unsigned",
            "using",
            "virtual",
            "void",
            "volatile",
            "wchar_t",
            "while",
            "xor",
            "xor_eq",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }
}
