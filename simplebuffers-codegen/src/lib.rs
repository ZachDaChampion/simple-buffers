use simplebuffers_core::SBSchema;

/// Parameters for code generators.
pub struct GeneratorParams {
    /// The name of the generated file.
    pub file_name: String,

    /// Additional arguments passed to the compiler. These are likely to be generator-specific
    /// parameters and should be parsed appropriately. The [clap](https://docs.rs/clap/latest/clap/)
    /// library is recommended for this.
    pub additional_args: String,
}

/// A SimpleBuffers code generator.
pub trait CodeGenerator {
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
    fn generate(&mut self, schema: SBSchema, params: GeneratorParams) -> Result<(), String>;
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
/// impl MyRustGenerator {
///     fn new() -> Self {
///         Self
///     }
/// }
///
/// impl CodeGenerator for MyRustGenerator {
///     fn generate(schema: SBSchema, params: GeneratorParams) -> Result<(), String> {
///         // Custom generation logic here...
///     }
/// }
///
/// register_generator!(rust, MyRustGenerator);
/// register_generator!(rs, MyRustGenerator);
/// ```
macro_rules! register_generator {
    ($name:ident, $generator:ty) => {
        #[no_mangle]
        pub extern "C" fn $name() -> Box<dyn CodeGenerator> {
            Box::new(<$generator>::new())
        }
    };
}
