mod codegen;

use std::{fs::File, io::Write};

use simplebuffers_codegen::CodeGenerator;

use crate::codegen::generate_code;

#[derive(Debug)]
pub struct TsCodeGenerator;

impl CodeGenerator for TsCodeGenerator {
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
        let code = generate_code(schema);

        // Write code to file.
        {
            let mut target_file =
                File::create(format!("{}/{}.ts", params.dest_dir, params.file_name))
                    .expect("Failed to open target file");
            target_file
                .write_all(code.as_bytes())
                .expect("Failed to write target file.");
        }

        // Copy corelib to target directory.
        {
            let corelib = include_str!("../corelib/simplebuffers.ts");
            let mut corelib_file = File::create(format!("{}/simplebuffers.ts", params.dest_dir))
                .expect("Failed to open corelib header file");
            corelib_file
                .write_all(corelib.as_bytes())
                .expect("Failed to write header file.");
        }

        Ok(())
    }

    fn reserved_identifiers(
        &mut self,
        _params: &simplebuffers_codegen::GeneratorParams,
    ) -> Vec<String> {
        vec![
            "abstract",
            "arguments",
            "async",
            "await",
            "boolean",
            "break",
            "byte",
            "case",
            "catch",
            "char",
            "class",
            "const",
            "continue",
            "debugger",
            "default",
            "delete",
            "do",
            "double",
            "else",
            "enum",
            "eval",
            "export",
            "extends",
            "false",
            "final",
            "finally",
            "float",
            "for",
            "function",
            "goto",
            "if",
            "implements",
            "function",
            "import",
            "in",
            "instanceof",
            "int",
            "interface",
            "let",
            "long",
            "native",
            "new",
            "null",
            "package",
            "private",
            "protected",
            "public",
            "return",
            "short",
            "static",
            "super",
            "switch",
            "synchronized",
            "this",
            "throw",
            "throws",
            "transient",
            "true",
            "try",
            "typeof",
            "using",
            "var",
            "void",
            "volatile",
            "while",
            "with",
            "yield",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect()
    }
}
