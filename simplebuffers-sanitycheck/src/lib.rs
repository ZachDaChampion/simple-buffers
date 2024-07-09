use simplebuffers_codegen::{register_generator, CodeGenerator};
use simplebuffers_core::{Enum, EnumVariant, Sequence, Type};

//                                                                                                //
// ========================================== Register ========================================== //
//                                                                                                //

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
        _params: simplebuffers_codegen::GeneratorParams,
    ) -> Result<(), String> {
        print_enums(&schema.enums);
        print_sequences(&schema.sequences);
        Ok(())
    }
}

register_generator!(sanitycheck: SanityCheckCodeGenerator);

//                                                                                                //
// ===================================== Printing Functions ===================================== //
//                                                                                                //

/// Print a list of enums.
fn print_enums(enums: &[Enum]) {
    print!(concat!(
        "=========================\n",
        "|         ENUMS         |\n",
        "=========================\n\n"
    ));

    for Enum {
        name,
        size,
        variants,
    } in enums.iter()
    {
        println!("{} ({} bytes):", name, size);
        for EnumVariant { name, value } in variants.iter() {
            println!("  {} = {}", name, value);
        }
        println!();
    }
}

/// Print a list of sequences.
fn print_sequences(sequences: &[Sequence]) {
    print!(concat!(
        "=========================\n",
        "|       SEQUENCES       |\n",
        "=========================\n\n"
    ));

    for sequence in sequences.iter() {
        println!("{}:", sequence.name);
        for field in sequence.fields.iter() {
            // For each field in a root-level sequence, reset local indentation and create a new
            // stack.
            let mut indent = 1;
            let mut stack = Vec::new();
            stack.push((Some(field.name.clone()), &field.ty, field.index));

            // Run through the stack of the current field.
            while let Some((field_name, field_type, field_offset)) = stack.pop() {
                // For named fields, print the name.
                if let Some(n) = field_name {
                    print!(
                        "{indent}{offset} | {name}: ",
                        offset = field_offset,
                        indent = "  ".repeat(indent),
                        name = n
                    );
                }

                // Print and maybe add to stack depending on the field type.
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
}
