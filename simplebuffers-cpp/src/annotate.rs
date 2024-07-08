use convert_case::Case;
use convert_case::Casing;
use simplebuffers_core::Enum;
use simplebuffers_core::Field;
use simplebuffers_core::Primitive;
use simplebuffers_core::SBSchema;
use simplebuffers_core::Sequence;
use simplebuffers_core::Type;

use Case::Pascal as SequenceCase;
use Case::Pascal as EnumCase;
use Case::Pascal as OneOfCase;
use Case::Snake as FieldCase;
use Case::UpperSnake as EnumVarCase;

pub(crate) struct CppSchema {
    pub sequences: Vec<CppSequence>,
    pub enums: Vec<CppEnum>,
}

/// An enum, annotated and adjusted for C++ conventions.
pub(crate) struct CppEnum {
    /// The name of the enum.
    pub name: String,

    /// The size, in bytes, of the enum.
    pub size: u8,

    /// The variants of the enum in the form (name, value).
    pub variants: Vec<(String, u64)>,
}

/// A sequence, annotated and adjusted for C++ conventions.
pub(crate) struct CppSequence {
    /// The name of the sequence.
    pub name: String,

    /// The fields of the sequence.
    pub fields: Vec<CppField>,
}

/// A field, annotated and adjusted for C++ conventions.
pub(crate) struct CppField {
    /// The name of the field.
    pub name: String,

    /// The type of the field.
    pub ty: CppType,

    /// The index of the field. For sequences, this is the offset in bytes from the start of the
    /// sequence. For oneofs, this is the index of the field in the oneof.
    pub index: usize,
}

/// A type, annotated and adjusted for C++ conventions.
pub(crate) enum CppType {
    Primitive(&'static str),
    Sequence(String),
    Enum(String),
    Array(Box<CppType>),
    OneOf(CppOneOf),
}

/// A oneof, annotated and adjusted for C++ conventions.
pub(crate) struct CppOneOf {
    /// The name of the oneof.
    pub name: String,

    /// The fields of the oneof.
    pub fields: Vec<CppField>,
}

/// Take a schema and annotate it for use with C++. This will adjust naming to match C++ convention,
/// and will add extra data that is necessary for C++ code generation.
pub(crate) fn annotate_schema(schema: SBSchema) -> CppSchema {
    CppSchema {
        sequences: schema.sequences.iter().map(annotate_sequence).collect(),
        enums: schema.enums.iter().map(annotate_enum).collect(),
    }
}

/// Annotate a single enum.
///
/// # Arguments
///
/// * `original` - The enum to annotate.
///
/// # Returns
///
/// An enum, formatted for C++ code generation.
fn annotate_enum(original: &Enum) -> CppEnum {
    CppEnum {
        name: original.name.to_case(EnumCase),
        size: original.size,
        variants: original
            .variants
            .iter()
            .map(|v| (v.name.to_case(EnumVarCase), v.value))
            .collect(),
    }
}

/// Recursively annotate a single sequence.
///
/// # Arguments
///
/// * `seq` - The sequence to annotate.
///
/// # Returns
///
/// A sequence, formatted for C++ code generation.
fn annotate_sequence(seq: &Sequence) -> CppSequence {
    CppSequence {
        name: seq.name.to_case(SequenceCase),
        fields: seq.fields.iter().map(annotate_field).collect(),
    }
}

/// Annotates a single field.
///
/// # Arguments
///
/// * `field` - The field to annotate.
///
/// # Returns
///
/// An annotated CppField.
fn annotate_field(field: &Field) -> CppField {
    CppField {
        name: field.name.to_case(FieldCase),
        ty: annotate_type(&field.ty, field.name.as_str()),
        index: field.index,
    }
}

/// Annotate a Type.
///
/// # Arguments
///
/// * `ty` - The type to annotate.
/// * `field_name` - The name of the field that the type is associated with.
///
/// # Returns
///
/// An annotated CppType.
fn annotate_type(ty: &Type, field_name: &str) -> CppType {
    match ty {
        Type::Primitive(p) => CppType::Primitive(match p {
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
        }),
        Type::Sequence(s) => CppType::Sequence(s.to_case(SequenceCase)),
        Type::Enum(e) => CppType::Enum(e.to_case(EnumCase)),
        Type::Array(t) => CppType::Array(Box::new(annotate_type(t, field_name))),
        Type::String => CppType::Primitive("char*"),
        Type::OneOf(o) => CppType::OneOf(annotate_oneof(o, field_name)),
    }
}

/// Annotate a OneOf type.
///
/// # Arguments
///
/// * `subfields` - A list of the oneof's fields.
/// * `field_name` - The name of the field holding the oneof.
///
/// # Returns
///
/// An annotated CppOneOf.
fn annotate_oneof(subfields: &Vec<Field>, field_name: &str) -> CppOneOf {
    CppOneOf {
        name: field_name.to_case(OneOfCase),
        fields: (subfields
            .iter()
            .map(|f| CppField {
                name: f.name.to_case(FieldCase),
                ty: annotate_type(&f.ty, f.name.as_str()),
                index: f.index,
            })
            .collect()),
    }
}
