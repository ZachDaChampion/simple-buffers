use std::path::Path;

use clap::Parser;
use internal_generators::get_internal_generator;
use libloading::{Library, Symbol};
use simplebuffers_codegen::{CodeGenerator, GeneratorParams};
use simplebuffers_core::SBSchema;

mod ast;
mod compiler;
mod internal_generators;
mod tokenizer;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// A struct that holds CLI parameters.
#[derive(Parser, Debug)]
#[command(name = "SimpleBuffers Compiler")]
#[command(version = VERSION)]
#[command(about = "Compile a SimpleBuffers schema into your chosen language.")]
struct Cli {
    /// A custom library to load. This can be used to load third-party generators.
    #[arg(short, long)]
    lib: Option<String>,

    /// The directory where your SimpleBuffers schema lives.
    #[arg(short, long)]
    srcdir: Option<String>,

    /// The directory to write generated files to.
    #[arg(short, long)]
    dstdir: Option<String>,

    /// The name of the code generator to use.
    generator: String,

    /// The SimpleBuffers file to parse.
    file: String,

    /// Additional arguments that are specific to the code generator.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    generator_args: Vec<String>,
}

/// Load a generator from a shared library and run it.
fn run_from_lib(
    schema: SBSchema,
    params: GeneratorParams,
    path: &str,
    gen_name: &str,
) -> Result<(), String> {
    let loaded_lib =
        unsafe { Library::new(path).map_err(|_| format!("Failed to load library at '{}'", path)) }?;
    let loaded_constructor: Symbol<fn() -> Box<dyn CodeGenerator>> = unsafe {
        loaded_lib
            .get(gen_name.as_bytes())
            .map_err(|_| format!("Failed to load generator from '{}'", path))?
    };
    let mut generator = loaded_constructor();
    generator
        .generate(schema, params)
        .map_err(|e| format!("GENERATOR ERROR: {}", e))
}

/// Search for a generator bundled with the SimpleBuffers compiler and run it if found.
fn run_internal(schema: SBSchema, params: GeneratorParams, gen_name: &str) -> Result<(), String> {
    if let Some(mut generator) = get_internal_generator(gen_name) {
        generator
            .generate(schema, params)
            .map_err(|e| format!("GENERATOR ERROR: {}", e))
    } else {
        Err(format!("No generators found for target {}", gen_name))
    }
}

fn main_impl() -> Result<(), String> {
    let cli = Cli::parse();
    let generator_args = format!("{} {}", cli.generator.clone(), cli.generator_args.join(" "));

    let raw_schema = std::fs::read_to_string(cli.file.clone())
        .map_err(|_| format!("Failed to read '{}'", cli.file))?;
    let mut parser =
        ast::AstBuilder::new(raw_schema.as_str(), "test").map_err(|_| "Failed to create parser")?;
    let ast = parser.parse().map_err(|_| "Failed to parse into AST")?;
    let schema = compiler::parse_ast(&ast).map_err(|_| "Failed to compile AST")?;

    let filename = {
        let ostr = Path::new(&cli.file)
            .file_name()
            .ok_or("Path to schema file is invalid")?;
        let raw_str = ostr
            .to_str()
            .ok_or("Path to schema file contains invalid Unicode")?;
        raw_str.to_string()
    };

    let generator_params = GeneratorParams {
        file_name: filename,
        additional_args: generator_args,
    };

    if let Some(lib_path) = cli.lib {
        run_from_lib(schema, generator_params, &lib_path, &cli.generator)
    } else {
        run_internal(schema, generator_params, &cli.generator)
    }
}

fn main() {
    if let Err(e) = main_impl() {
        println!("{}", e);
    }
}
