//! This file parses a SyntaxTree into a series of sequences and enums.

use crate::ast::{SyntaxTree, TaggedSyntaxTree, TreeTraversal};

use super::CompilerError;

/// A sequence of fields.
pub struct Sequence {
    /// The name of the sequence.
    pub name: String,

    /// The fields of the sequence.
    pub fields: Vec<Field>,
}

/// A field of a sequence.
pub struct Field {
    /// The name of the field.
    pub name: String,

    /// The type of the field.
    pub ty: Type,
}

/// A type.
pub enum Type {
    /// A primitive type.
    Primitive(Primitive),

    /// A sequence type.
    Sequence(String),

    /// An enum type.
    Enum(String),

    /// An array type.
    Array(Box<Type>),

    /// A oneof type. This is a type that can be one of several types.
    OneOf(Vec<Type>),
}

/// A primitive type.
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
    String,
    Bytes,
}

/// An enum.
pub struct Enum {
    /// The name of the enum.
    pub name: String,

    /// The variants of the enum.
    pub variants: Vec<EnumVariant>,
}

/// A variant of an enum.
pub struct EnumVariant {
    /// The name of the variant.
    pub name: String,

    /// The value of the variant.
    pub value: i32,
}

/// The result of parsing a SyntaxTree.
pub struct ParseResult {
    /// The sequences in the SyntaxTree.
    pub sequences: Vec<Sequence>,

    /// The enums in the SyntaxTree.
    pub enums: Vec<Enum>,
}

/// Parse a SyntaxTree into a series of sequences and enums.
pub fn parse_ast(root: &TaggedSyntaxTree) -> Result<ParseResult, CompilerError> {
    // Make lists of all the sequence and enum names. This is used to verify types later on.
    let mut sequence_names = Vec::new();
    let mut enum_names = Vec::new();
    for node in root.iter_depth_first() {
        if let SyntaxTree::Sequence(name, _) = &node.data {
            sequence_names.push(name);
        } else if let SyntaxTree::Enum(name, _) = &node.data {
            enum_names.push(name);
        }
    }

    // Construct the result.
    let mut result = ParseResult {
        sequences: Vec::new(),
        enums: Vec::new(),
    };
    let file_contents = match &root.data {
        SyntaxTree::File(file) => file,
        _ => {
            return Err(CompilerError::InvalidNode(
                "Root node is not a file".to_string(),
            ))
        }
    };
    for top_level in file_contents {
        // match top_level {
        //     SyntaxTree::Sequence(sequence_name, sequence_fields)
        // }
    }

    Ok(result)
}
