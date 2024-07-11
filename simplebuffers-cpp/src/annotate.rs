use convert_case::Casing;
use simplebuffers_core::Enum;
use simplebuffers_core::Field;
use simplebuffers_core::Primitive;
use simplebuffers_core::SBSchema;
use simplebuffers_core::Sequence;
use simplebuffers_core::Type;

use crate::argparse::CppGeneratorParams;

#[derive(Debug)]
pub(crate) struct CppSchema {
    pub sequences: Vec<CppSequence>,
    pub enums: Vec<CppEnum>,
}

/// An enum, annotated and adjusted for C++ conventions.
#[derive(Debug)]
pub(crate) struct CppEnum {
    /// The name of the enum.
    pub name: String,

    /// The size, in bytes, of the enum.
    pub size: u8,

    /// The variants of the enum in the form (name, value).
    pub variants: Vec<(String, u64)>,
}

/// A sequence, annotated and adjusted for C++ conventions.
#[derive(Debug)]
pub(crate) struct CppSequence {
    /// The name of the sequence.
    pub name: String,

    /// The fields of the sequence.
    pub fields: Vec<CppSequenceField>,

    /// The size of the sequence in bytes.
    pub size: usize,
}

/// A field, annotated and adjusted for C++ conventions.
#[derive(Debug)]
pub(crate) struct CppSequenceField {
    /// The name of the field.
    pub name: String,

    /// The type of the field.
    pub ty: CppType,
}

/// A field, annotated and adjusted for C++ conventions.
#[derive(Debug)]
pub(crate) struct CppOneOfField {
    /// The name of the field.
    pub name: String,

    // The tag of the field, used as an enum variant.
    pub tag: String,

    // The name of the field's dedicated constructor.
    pub constructor: String,

    /// The type of the field.
    pub ty: CppType,

    /// The index of the field in the oneof.
    pub index: usize,
}

/// A type, annotated and adjusted for C++ conventions.
#[derive(Debug)]
pub(crate) enum CppType {
    Primitive(&'static str),
    Sequence(String),
    Enum(String, usize),
    Array(Box<CppType>),
    OneOf(CppOneOf),
}

/// A oneof, annotated and adjusted for C++ conventions.
#[derive(Debug)]
pub(crate) struct CppOneOf {
    /// The name of the oneof.
    pub name: String,

    /// The fields of the oneof.
    pub fields: Vec<CppOneOfField>,
}

impl CppEnum {
    /// Returns the C++ type that corresponds with this enum. This is used as the base of a C++
    /// enum class, so we prefer fast data types over minimal ones.
    pub(crate) fn size_to_type(&self) -> &str {
        match self.size {
            1 => "uint_fast8_t",
            2 => "uint_fast16_t",
            4 => "uint_fast32_t",
            8 => "uint_fast64_t",
            _ => panic!("Invalid size {} for enum {}", self.size, self.name),
        }
    }
}

impl CppSequence {
    /// Return an iterator over all of the oneof fields contained within this sequence.
    pub(crate) fn oneofs(&self) -> impl DoubleEndedIterator<Item = &CppOneOf> {
        self.fields.iter().filter_map(|f| match &f.ty {
            CppType::OneOf(o) => Some(o),
            _ => None,
        })
    }
}

impl CppSequenceField {
    /// Returns a version of this field's name, cast to the appropriate type for serialization. This
    /// only affects enums.
    pub(crate) fn cast(&self) -> String {
        match self.ty {
            CppType::Enum(_, size) => format!(
                "static_cast<{}>({})",
                match size {
                    1 => "uint8_t",
                    2 => "uint16_t",
                    4 => "uint32_t",
                    8 => "uint64_t",
                    _ => panic!("Invalid size {} for enum {}", size, self.name),
                },
                self.name
            ),
            _ => self.name.to_string(),
        }
    }
}

impl CppOneOf {
    /// Return an iterator over all of the oneof fields contained within this oneof.
    pub(crate) fn oneofs(&self) -> impl DoubleEndedIterator<Item = &CppOneOf> {
        self.fields.iter().filter_map(|f| match &f.ty {
            CppType::OneOf(o) => Some(o),
            _ => None,
        })
    }
}

impl CppOneOfField {
    /// Returns a version of this field's name, cast to the appropriate type for serialization. This
    /// only affects enums.
    pub(crate) fn cast(&self) -> String {
        match self.ty {
            CppType::Enum(_, size) => format!(
                "static_cast<{}>({})",
                match size {
                    1 => "uint8_t",
                    2 => "uint16_t",
                    4 => "uint32_t",
                    8 => "uint64_t",
                    _ => panic!("Invalid size {} for enum {}", size, self.name),
                },
                self.name
            ),
            _ => self.name.to_string(),
        }
    }
}

/// This trait is implemented for fields and types that must be represented in specific ways for
/// Readers and Writers.
pub(crate) trait ToReaderWriterString {
    /// Convert to a Writer name.
    fn to_writer_string(&self, params: &CppGeneratorParams) -> String;

    /// Convert to a Reader name.
    fn to_reader_string(&self, params: &CppGeneratorParams) -> String;
}

impl ToReaderWriterString for CppType {
    fn to_writer_string(&self, params: &CppGeneratorParams) -> String {
        match self {
            CppType::Primitive(p) => p.to_string(),
            CppType::Sequence(s) => format!("{}_Writer", s).to_case(params.class_case),
            CppType::Enum(e, _) => e.clone(),
            CppType::Array(t) => {
                format!("simplebuffers::ArrayWriter<{}>", t.to_writer_string(params))
            }
            CppType::OneOf(o) => format!("{}_Writer", o.name).to_case(params.class_case),
        }
    }

    fn to_reader_string(&self, params: &CppGeneratorParams) -> String {
        match self {
            CppType::Primitive(p) => p.to_string(),
            CppType::Sequence(s) => format!("{}_Reader", s).to_case(params.class_case),
            CppType::Enum(e, _) => e.clone(),
            CppType::Array(t) => {
                format!("simplebuffers::ArrayReader<{}>", t.to_writer_string(params))
            }
            CppType::OneOf(o) => format!("{}_Reader", o.name).to_case(params.class_case),
        }
    }
}

impl ToReaderWriterString for CppSequence {
    fn to_writer_string(&self, params: &CppGeneratorParams) -> String {
        format!("{}_Writer", self.name).to_case(params.class_case)
    }

    fn to_reader_string(&self, params: &CppGeneratorParams) -> String {
        format!("{}_Reader", self.name).to_case(params.class_case)
    }
}

impl ToReaderWriterString for CppOneOf {
    fn to_writer_string(&self, params: &CppGeneratorParams) -> String {
        format!("{}_Writer", self.name).to_case(params.class_case)
    }

    fn to_reader_string(&self, params: &CppGeneratorParams) -> String {
        format!("{}_Reader", self.name).to_case(params.class_case)
    }
}

/// Take a schema and annotate it for use with C++. This will adjust naming to match C++ convention,
/// and will add extra data that is necessary for C++ code generation.
pub(crate) fn annotate_schema(params: &CppGeneratorParams, schema: &SBSchema) -> CppSchema {
    CppSchema {
        sequences: schema
            .sequences
            .iter()
            .map(|s| annotate_sequence(params, s))
            .collect(),
        enums: schema
            .enums
            .iter()
            .map(|e| annotate_enum(params, e))
            .collect(),
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
fn annotate_enum(params: &CppGeneratorParams, original: &Enum) -> CppEnum {
    CppEnum {
        name: original.name.to_case(params.class_case),
        size: original.size,
        variants: original
            .variants
            .iter()
            .map(|v| (v.name.to_case(params.enum_var_case), v.value))
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
fn annotate_sequence(params: &CppGeneratorParams, seq: &Sequence) -> CppSequence {
    CppSequence {
        name: seq.name.to_case(params.class_case),
        fields: seq
            .fields
            .iter()
            .map(|f| CppSequenceField {
                name: f.name.to_case(params.field_case),
                ty: annotate_type(params, &f.ty, f.name.as_str()),
            })
            .collect(),
        size: seq.fields.iter().fold(0, |acc, f| acc + f.ty.size()),
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
fn annotate_type(params: &CppGeneratorParams, ty: &Type, field_name: &str) -> CppType {
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
        Type::Sequence(s) => CppType::Sequence(s.to_case(params.class_case)),
        Type::Enum(e, s) => CppType::Enum(e.to_case(params.class_case), *s),
        Type::Array(t) => CppType::Array(Box::new(annotate_type(params, t, field_name))),
        Type::String => CppType::Primitive("char*"),
        Type::OneOf(o) => CppType::OneOf(annotate_oneof(params, o, field_name)),
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
fn annotate_oneof(params: &CppGeneratorParams, subfields: &[Field], field_name: &str) -> CppOneOf {
    CppOneOf {
        name: field_name.to_case(params.class_case),
        fields: (subfields
            .iter()
            .map(|f| CppOneOfField {
                name: f.name.to_case(params.field_case),
                tag: f.name.to_case(params.enum_var_case),
                constructor: f.name.to_case(params.class_case),
                ty: annotate_type(params, &f.ty, f.name.as_str()),
                index: f.index,
            })
            .collect()),
    }
}
