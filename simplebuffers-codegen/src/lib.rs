use simplebuffers_core::SBSchema;

/// Parameters for code generators.
#[derive(Debug, Clone)]
pub struct GeneratorParams {
    /// The name of the generated file.
    pub file_name: String,

    /// The directory to write generated files to.
    pub dest_dir: String,

    /// Additional arguments passed to the compiler. These are likely to be generator-specific
    /// parameters and should be parsed appropriately. The [clap](https://docs.rs/clap/latest/clap/)
    /// library is recommended for this.
    pub additional_args: String,
}

/// A SimpleBuffers code generator.
pub trait CodeGenerator {
    /// Construct a new instance of your CodeGenerator. This is used when registering your generator
    /// from the main compiler and should produce a fully functional instance. If parameters are
    /// required, they should be parsed dynamically from the `generate` function.
    fn new() -> Self
    where
        Self: Sized;

    /// Generate code for encoding and decoding messages.
    ///
    /// # Arguments
    ///
    /// * `schema` - The schema to generate code for.
    /// * `params` - The parameters for the generator.
    ///
    /// # Returns
    ///
    /// A `Result` indicating whether the operation was successful.
    fn generate(&mut self, schema: &SBSchema, params: &GeneratorParams) -> Result<(), String>;

    /// Returns a list of reserved identifiers in the generated language. The compiler will ensure
    /// that these identifiers are not used anywhere in the schema before calling `generate`.
    fn reserved_identifiers(&mut self, params: &GeneratorParams) -> Vec<String>;
}

#[macro_export]
/// This macro is used to register code generators with the SimpleBuffers compiler. Call this at the
/// end of your `lib.rs`.
///
/// # Arguments
///
/// * `name` - The name of the generator (e.g. 'cpp', 'python', etc.).
/// * `generator` - The generator struct being registered. This must implement the `Generator`
///   trait.
///
/// # Example
///
/// ```
/// // lib.rs
///
/// pub struct MyGenerator;
///
/// impl CodeGenerator for MyRustGenerator {
///     fn new() -> Self {
///         Self
///     }
///
///     fn generate(&mut self, schema: &SBSchema, params: &GeneratorParams) -> Result<(), String> {
///         // Custom generation logic here...
///     }
///
///     fn reserved_identifiers(&mut self, _params: &GeneratorParams) -> Vec<String> {
///         vec!["if", "for", "while"]
///             .iter()
///             .map(|s| s.to_string())
///             .collect()
///     }
/// }
///
/// register_generator!(rust: MyRustGenerator);
/// register_generator!(rs: MyRustGenerator);
/// ```
macro_rules! register_generator {
    ($name:ident : $generator:ty) => {
        #[no_mangle]
        pub extern "C" fn $name() -> Box<dyn CodeGenerator> {
            Box::new(<$generator as CodeGenerator>::new())
        }
    };
}
