mod argparse;
mod annotate;
mod headergen;

use argparse::parse_args;
use simplebuffers_codegen::CodeGenerator;

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
        schema: simplebuffers_core::SBSchema,
        params: simplebuffers_codegen::GeneratorParams,
    ) -> Result<(), String> {
        let generator_params = parse_args(&params.additional_args);
        println!("C++ Generator! Args: {:?}", generator_params);
        Ok(())
    }
}
