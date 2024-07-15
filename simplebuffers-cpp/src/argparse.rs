//! Parses generator-specific arguments contained in [GeneratorParams::additional_args].

use clap::Parser;
use simplebuffers_codegen::GeneratorParams;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Struct to parse CLI args into.
#[derive(Parser, Debug)]
#[command(name = "SimpleBuffers C++ Code Generator")]
#[command(version = VERSION)]
#[command(about = "Generate C++ code from a SimpleBuffers schema.")]
struct Cli {
    /// The directory to write generated header files to. Source files will be written to `dstdir`.
    /// If `headerdir` is not specified, header files will be written to `dstdir` as well.
    #[arg(long)]
    headerdir: Option<String>,
}

/// A struct that holds generator-specific arguments for the C++ generator.
#[derive(Debug)]
pub(crate) struct CppGeneratorParams {
    /// The directory to write generated header files to.
    pub header_dir: String,

    /// The global compiler parameters.
    pub global: GeneratorParams,
}

/// Parse generator-specific arguments from an input string.
pub(crate) fn parse_args(generator_params: &GeneratorParams) -> CppGeneratorParams {
    let cli = Cli::parse_from(generator_params.additional_args.split_ascii_whitespace());
    CppGeneratorParams {
        header_dir: cli.headerdir.unwrap_or(generator_params.dest_dir.clone()),
        global: generator_params.clone(),
    }
}
