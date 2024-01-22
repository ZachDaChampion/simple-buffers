use crate::compiler::ParseResult;

pub mod cpp;

/// Parameters for code generators.
pub struct GeneratorParams<T> {
    /// The name of the generated file.
    pub file_name: String,

    /// Additional arguments that are specific to the generator.
    pub additional_args: Box<T>,
}

/// A code generator.
pub trait Generator<T> {
    /// Generate code for encoding and decoding messages.
    /// 
    /// # Arguments
    /// 
    /// * `params` - The parameters for the generator.
    /// 
    /// # Returns
    /// 
    /// A `Result` indicating whether the operation was successful.
    fn generate(data: ParseResult, params: GeneratorParams<T>) -> Result<(), String>;
}