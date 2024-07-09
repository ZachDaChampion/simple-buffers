use clap::{Parser, ValueEnum};
use convert_case::Case;
use simplebuffers_codegen::GeneratorParams;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum CaseOption {
    /// PascalCase
    Pascal,

    /// camelCase
    Camel,

    /// snake_case
    Snake,

    /// UPPERFLATCASE
    UpperFlat,

    /// UPPER_SNAKE_CASE
    UpperSnake,
}

// use Case::Pascal as SequenceCase;
// use Case::Pascal as EnumCase;
// use Case::UpperSnake as EnumVarCase;
// use Case::Snake as FieldCase;
// use Case::Pascal as OneOfCase;

/// Struct to parse CLI args into.
#[derive(Parser, Debug)]
#[command(name = "SimpleBuffers C++ Code Generator")]
#[command(version = VERSION)]
#[command(about = "Generate C++ code from a SimpleBuffers schema.")]
struct CLI {
    /// The directory to write generated header files to. Source files will be written to `dstdir`.
    /// If `headerdir` is not specified, header files will be written to `dstdir` as well.
    #[arg(long)]
    headerdir: Option<String>,

    /// The type of casing for class names. Default is PascalCase.
    #[arg(long, value_enum)]
    classcase: Option<CaseOption>,

    /// The type of casing for enum variant names. Default is UPPER_SNAKE_CASE.
    #[arg(long, value_enum)]
    enumvarcase: Option<CaseOption>,

    /// The type of casing for field names. Default is snake_case.
    #[arg(long, value_enum)]
    fieldcase: Option<CaseOption>,

    /// The type of casing for namespace names. Default is snake_case.
    #[arg(long, value_enum)]
    nscase: Option<CaseOption>,
}

impl From<CaseOption> for Case {
    fn from(value: CaseOption) -> Self {
        match value {
            CaseOption::Pascal => Case::Pascal,
            CaseOption::Camel => Case::Camel,
            CaseOption::Snake => Case::Snake,
            CaseOption::UpperFlat => Case::UpperFlat,
            CaseOption::UpperSnake => Case::UpperSnake,
        }
    }
}

/// A struct that holds generator-specific arguments for the C++ generator.
#[derive(Debug)]
pub(crate) struct CppGeneratorParams {
    /// The directory to write generated header files to.
    pub header_dir: String,

    /// The type of casing for class names.
    pub class_case: Case,

    /// The type of casing for enum variant names.
    pub enum_var_case: Case,

    /// The type of casing for field names.
    pub field_case: Case,

    /// The type of casing for namespace names.
    pub ns_case: Case,

    /// The global compiler parameters.
    pub global: GeneratorParams,
}

/// Parse generator-specific arguments from an input string.
pub(crate) fn parse_args(generator_params: GeneratorParams) -> CppGeneratorParams {
    let cli = CLI::parse_from(generator_params.additional_args.split_ascii_whitespace());
    CppGeneratorParams {
        header_dir: cli.headerdir.unwrap_or(generator_params.dest_dir.clone()),
        class_case: cli.classcase.unwrap_or(CaseOption::Pascal).into(),
        enum_var_case: cli.enumvarcase.unwrap_or(CaseOption::UpperSnake).into(),
        field_case: cli.fieldcase.unwrap_or(CaseOption::Snake).into(),
        ns_case: cli.nscase.unwrap_or(CaseOption::Snake).into(),

        global: generator_params,
    }
}
