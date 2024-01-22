//! C++ code generator for encoding and decoding SimpleBuffer messages.

use crate::compiler::{ParseResult, Primitive, Sequence};

use super::{Generator, GeneratorParams};

mod header;

/// Parameters specific to the C++ code generator.
pub struct CppGeneratorParams {}

/// A C++ code generator.
pub struct CppGenerator {}

/// Convert a primitive to the corresponding C++ type.
fn primitive_to_cpp_type(primitive: Primitive) -> &'static str {
    match primitive {
        Primitive::Bool => "bool",
        Primitive::U8 => "uint8_t",
        Primitive::U16 => "uint16_t",
        Primitive::U32 => "uint32_t",
        Primitive::U64 => "uint64_t",
        Primitive::I8 => "int8_t",
        Primitive::I16 => "int16_t",
        Primitive::I32 => "int32_t",
        Primitive::I64 => "int64_t",
        Primitive::F32 => "float",
        Primitive::F64 => "double",
        Primitive::String => "char const*",
        other => todo!("Handle primitive {:?}", other),
    }
}

/// Generate code for encoding a sequence.
fn sequence_encoder(sequence: Sequence) -> String {
    let mut result = format!(
        r"
class {name} {{
public:
    {name}() = default;
    
        ",
        name = sequence.name
    );

    result
}

impl Generator<CppGeneratorParams> for CppGenerator {
    fn generate(
        _data: ParseResult,
        _params: GeneratorParams<CppGeneratorParams>,
    ) -> Result<(), String> {
        Ok(())
    }
}
