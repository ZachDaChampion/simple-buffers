//! Interface for SimpleBuffers code generators.
//!
//! This module contains [CodeGenerator] trait, which must be implemented by all code generators. A
//! generator should be implemented as a crate that compiles to a `dylib`, which allows the compiler
//! to link to the generator as a dynamic library. Generators that are packaged into the compiler
//! executable must also compile to a `lib`:
//!
//! ```toml
//! [lib]
//! crate-type = ["lib", "dylib"]
//! ```
//!
//! Once a custom generator is created, it must be registered. This can be done with the
//! [register_generator!] macro.
//!
//! # Example
//!
//! ```
//! // lib.rs
//!
//! pub struct MyCodeGenerator;
//!
//! impl CodeGenerator for MyCodeGenerator {
//!     fn new() -> Self
//!     where
//!         Self: Sized,
//!     {
//!         Self
//!     }
//!
//!     fn generate(
//!         &mut self,
//!         schema: &simplebuffers_core::SBSchema,
//!         _params: &simplebuffers_codegen::GeneratorParams,
//!     ) -> Result<(), String> {
//!         // Custom generation code goes here...
//!     }
//!
//!     fn reserved_identifiers(
//!         &mut self,
//!         _params: &simplebuffers_codegen::GeneratorParams,
//!     ) -> Vec<String> {
//!         vec![
//!             "int",
//!             "if",
//!             "while",
//!             // ...
//!         ]
//!         .iter()
//!         .map(|s| s.to_string())
//!         .collect()
//!     }
//! }
//!
//! register_generator!(mygen: MyCodeGenerator);
//! ```
//!
//! Assuming this generator is compiled to `my_code_generator.so`, it can be invoked with:
//! ```sh
//! simplebuffers-compiler --lib="my_code_generator.so" mygen "my_schema.sb"
//! ```

pub use simplebuffers_core::SBSchema;

/// Parameters for code generators.
#[derive(Debug, Clone)]
pub struct GeneratorParams {
    /// The desired name of the generated file, without a file extension.
    pub file_name: String,

    /// The directory to write generated files to.
    pub dest_dir: String,

    /// Additional arguments passed to the compiler.
    ///
    /// These are likely to be generator-specific parameters and should be parsed appropriately. The
    /// [clap](https://docs.rs/clap/latest/clap/) library is recommended for this.
    ///
    /// `additional_args` begins with the name of the generator being invoked, followed by a series
    /// of arguments passed by the user.
    ///
    /// # Example
    ///
    /// ```
    /// // simplebuffers-compiler --dstdir='src' cpp 'my_schema.sb' --headerdir='include'
    ///
    /// // Results in:
    /// additional_args: "cpp --headerdir='include'"
    /// ```
    pub additional_args: String,
}

/// A SimpleBuffers code generator.
pub trait CodeGenerator {
    /// Construct a new instance of your CodeGenerator.
    ///
    /// This is used when registering your generator from the main compiler and should produce a
    /// fully functional instance. If parameters are required, they should be parsed dynamically
    /// from the `generate` function.
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
    /// # Errors
    ///
    /// A human-readable string. This will be reported to the user by the compiler, so it should be
    /// a useful message describing the cause of the issue or how it may be fixed.
    fn generate(&mut self, schema: &SBSchema, params: &GeneratorParams) -> Result<(), String>;

    /// Returns a list of reserved identifiers in the generated language. The compiler will ensure
    /// that these identifiers are not used anywhere in the schema before calling `generate`.
    fn reserved_identifiers(&mut self, params: &GeneratorParams) -> Vec<String>;
}

#[macro_export]
/// This macro is used to register code generators with the SimpleBuffers compiler. Call this in
/// your `lib.rs`.
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
/// register_generator!(cpp: MyCppGenerator);
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
