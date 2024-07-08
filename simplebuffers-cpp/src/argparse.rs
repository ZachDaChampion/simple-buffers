use clap::{Parser, ValueEnum};

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// A struct that holds generator-specific arguments for the C++ generator.
#[derive(Parser, Debug)]
#[command(name = "SimpleBuffers C++ Code Generator")]
#[command(version = VERSION)]
#[command(about = "Generate C++ code from a SimpleBuffers schema.")]
pub(crate) struct CPPGeneratorArgs {
    /// The directory to write generated header files to. Source files will be written to `dstdir`.
    /// If `headerdir` is not specified, header files will be written to `dstdir` as well.
    #[arg(long)]
    headerdir: Option<String>,
}

/// Parse generator-specific arguments from an input string.
pub(crate) fn parse_args(args_str: &str) -> CPPGeneratorArgs {
    CPPGeneratorArgs::parse_from(args_str.split_ascii_whitespace())
}
