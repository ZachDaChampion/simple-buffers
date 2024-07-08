use simplebuffers_codegen::{register_generator, CodeGenerator};
use simplebuffers_core::{Enum, EnumVariant, Type};

#[derive(Debug)]
pub struct SanityCheckCodeGenerator;

impl CodeGenerator for SanityCheckCodeGenerator {
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
        let _ = params;

        print!(concat!(
            "=========================\n",
            "|         ENUMS         |\n",
            "=========================\n\n"
        ));
        for Enum {
            name,
            size,
            variants,
        } in schema.enums.iter()
        {
            println!("{} ({} bytes):", name, size);
            for EnumVariant { name, value } in variants.iter() {
                println!("  {} = {}", name, value);
            }
            println!();
        }

        print!(concat!(
            "=========================\n",
            "|       SEQUENCES       |\n",
            "=========================\n\n"
        ));
        for sequence in schema.sequences.iter() {
            println!("{}:", sequence.name);
            for field in sequence.fields.iter() {
                let mut indent = 1;
                let mut stack = Vec::new();
                stack.push((Some(field.name.clone()), &field.ty, field.index));
                while let Some((field_name, field_type, field_offset)) = stack.pop() {
                    if let Some(n) = field_name {
                        print!(
                            "{indent}{offset} | {name}: ",
                            offset = field_offset,
                            indent = "  ".repeat(indent),
                            name = n
                        );
                    }
                    match &field_type {
                        Type::Primitive(name) => println!("{} (primitive)", name),
                        Type::Sequence(name) => println!("{} (sequence)", name),
                        Type::Enum(name) => println!("{} (enum)", name),
                        Type::Array(ty) => {
                            print!("ARRAY OF ");
                            stack.push((None, ty, 0));
                        }
                        Type::String => println!("string"),
                        Type::OneOf(f) => {
                            println!("ONE OF:");
                            for field in f.iter().rev() {
                                stack.push((Some(field.name.clone()), &field.ty, field.index));
                            }
                            indent += 1;
                        }
                    }
                }
            }
            println!();
        }

        Ok(())
    }
}

register_generator!(sanitycheck: SanityCheckCodeGenerator);
