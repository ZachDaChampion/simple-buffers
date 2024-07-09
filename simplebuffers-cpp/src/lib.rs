mod annotate;
mod argparse;
mod headergen;

use annotate::annotate_schema;
use argparse::parse_args;
use headergen::generate_header;
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
        let generator_params = parse_args(params);
        println!("C++ Generator! Args: {:?}", generator_params);

        let annotated = annotate_schema(&generator_params, schema);
        println!("\n\nAnnotated:\n\n{:?}", annotated);

        let header = generate_header(&generator_params, &annotated);
        println!("\n\nHeader:\n\n{}", header);

        Ok(())
    }
}
