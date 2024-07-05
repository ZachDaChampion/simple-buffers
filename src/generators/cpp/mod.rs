//! C++ code generator for encoding and decoding SimpleBuffer messages.

use crate::compiler::{Field, ParseResult, Primitive, Sequence, Type};

use super::{Generator, GeneratorParams};

mod header;

/// Parameters specific to the C++ code generator.
pub struct CppGeneratorParams {}

/// A C++ code generator.
pub struct CppGenerator {}

/// A ParseResult, annotated with C++-specific data.
pub struct CppParseResult {
    pub sequences: Vec<CppSequence>,
    pub enums: Vec<CppEnum>,
}

/// A C++ enum.
pub struct CppEnum {
    /// The name of the enum.
    pub name: String,

    /// The variants of the enum in the form (name, value).
    pub variants: Vec<(String, u8)>,
}

pub struct CppSequence {
    /// The name of the sequence.
    pub local_name: String,

    /// The full name of the sequence, including any namespaces and nested classes.
    pub full_name: Vec<String>,

    /// The fields of the sequence.
    pub fields: Vec<CppField>,
}

pub struct CppOneOf {
    /// The name of the oneof.
    pub local_name: String,

    /// The full name of the oneof, including any namespaces and nested classes.
    pub full_name: Vec<String>,

    /// The possible fields of the oneof.
    pub fields: Vec<CppField>,
}

pub struct CppField {
    /// The name of the field.
    pub name: String,

    /// The type of the field.
    pub ty: CppType,

    /// The index of the field. For sequences, this is the offset in bytes from the start of the
    /// sequence. For oneofs, this is the index of the field in the oneof.
    pub index: usize,
}

pub enum CppType {
    Primitive(String),
    Sequence(String),
    Enum(String),
    Array(Box<CppType>),
    OneOf(CppOneOf),
}

fn annotate_sequence(seq: &Sequence, full_name: &Option<Vec<String>>) -> CppSequence {
    let local_name = seq.name.clone();
    let full_name_updated = match full_name {
        Some(f) => {
            let mut n = f.clone();
            n.push(local_name.clone());
            n
        }
        None => vec![local_name.clone()],
    };

    let mut fields = seq.fields.iter().map(|f| match f.ty {
        Type::Primitive(p) => match p {
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
            Primitive::String => "char*",
            Primitive::Bytes => "uint8_t"
        };
    });

    CppSequence {
        local_name,
        full_name: full_name_updated,
        fields: Vec::new(),
    }
}

/// Takes a ParseResult and annotates it with C++-specific data.
pub fn annotate_parse_result(data: ParseResult) -> CppParseResult {
    let enums = data
        .enums
        .iter()
        .map(|e| CppEnum {
            name: e.name.clone(),
            variants: e
                .variants
                .iter()
                .map(|v| (v.name.clone(), v.value))
                .collect(),
        })
        .collect();

    let sequences = Vec::new();

    CppParseResult { sequences, enums }
}

impl Generator<CppGeneratorParams> for CppGenerator {
    fn generate(
        _data: ParseResult,
        _params: GeneratorParams<CppGeneratorParams>,
    ) -> Result<(), String> {
        Ok(())
    }
}
