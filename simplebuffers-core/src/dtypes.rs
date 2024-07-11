use std::fmt;

/// A primitive type.
#[derive(Clone, Debug)]
pub enum Primitive {
    Bool,
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
    F32,
    F64,
}

impl Primitive {
    /// Get the size of the primitive in bytes.
    pub fn size(&self) -> usize {
        match self {
            Primitive::Bool => 1,
            Primitive::I8 => 1,
            Primitive::I16 => 2,
            Primitive::I32 => 4,
            Primitive::I64 => 8,
            Primitive::U8 => 1,
            Primitive::U16 => 2,
            Primitive::U32 => 4,
            Primitive::U64 => 8,
            Primitive::F32 => 4,
            Primitive::F64 => 8,
        }
    }
}

impl fmt::Display for Primitive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Primitive::Bool => write!(f, "bool"),
            Primitive::I8 => write!(f, "i8"),
            Primitive::I16 => write!(f, "i16"),
            Primitive::I32 => write!(f, "i32"),
            Primitive::I64 => write!(f, "i64"),
            Primitive::U8 => write!(f, "u8"),
            Primitive::U16 => write!(f, "u16"),
            Primitive::U32 => write!(f, "u32"),
            Primitive::U64 => write!(f, "u64"),
            Primitive::F32 => write!(f, "f32"),
            Primitive::F64 => write!(f, "f64"),
        }
    }
}

/// A field of a sequence.
#[derive(Clone, Debug)]
pub struct Field {
    /// The name of the field.
    pub name: String,

    /// The type of the field.
    pub ty: Type,

    /// The index of the field. For sequences, this is the offset in bytes from the start of the
    /// sequence. For oneofs, this is the index of the field in the oneof.
    pub index: usize,
}

/// A sequence of fields.
pub struct Sequence {
    /// The name of the sequence.
    pub name: String,

    /// The fields of the sequence.
    pub fields: Vec<Field>,
}

/// A type.
#[derive(Clone, Debug)]
pub enum Type {
    /// A primitive type.
    Primitive(Primitive),

    /// A sequence type.
    Sequence(String),

    /// An enum type. This contains its name and its size when stored.
    Enum(String, usize),

    /// An array type.
    Array(Box<Type>),

    /// A string type.
    String,

    /// A oneof type. This is a type that can be one of several types.
    OneOf(Vec<Field>),
}

impl Type {
    /// Get the size of the type in bytes. This is the fixed size that the type will take up in a
    /// sequence or oneof. It does not account for any dynamic sizes such as the size of a string
    /// that are added to the end of the structure.
    pub fn size(&self) -> usize {
        match self {
            Self::Primitive(p) => p.size(),
            Self::Sequence(_) => 2, // 16-bit offset to actual sequence.
            Self::Enum(_, s) => *s, // Size depends on enum values.
            Self::Array(_) => 4,    // 16-bit array length + 16-bit offset to actual array.
            Self::String => 2,      // 16-bit offset.
            Self::OneOf(_) => 3,    // 8-bit index + 16-bit offset to actual field.
        }
    }
}

/// An enum.
pub struct Enum {
    /// The name of the enum.
    pub name: String,

    /// The size of the enum, in bytes. This matches the smallest data type that can fully represent
    /// the enum and should not be greater than 8 (equivalent to a u64).
    pub size: u8,

    /// The variants of the enum.
    pub variants: Vec<EnumVariant>,
}

/// A variant of an enum.
pub struct EnumVariant {
    /// The name of the variant.
    pub name: String,

    /// The value of the variant.
    pub value: u64,
}

/// A fully parsed SimpleBuffers schema.
pub struct SBSchema {
    /// The sequences in the SyntaxTree.
    pub sequences: Vec<Sequence>,

    /// The enums in the SyntaxTree.
    pub enums: Vec<Enum>,
}
