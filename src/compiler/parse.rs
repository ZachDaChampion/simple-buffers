//! This file parses a SyntaxTree into a series of sequences and enums.

use colored::Colorize;

use super::CompilerError;
use crate::ast::{SyntaxTree, TaggedSyntaxTree, TreeTraversal};
use core::fmt;
use std::collections::HashMap;

/// Array of primitive names and internal representations.
const PRIMITIVES: [(&str, Primitive); 13] = [
    ("bool", Primitive::Bool),
    ("i8", Primitive::I8),
    ("i16", Primitive::I16),
    ("i32", Primitive::I32),
    ("i64", Primitive::I64),
    ("u8", Primitive::U8),
    ("u16", Primitive::U16),
    ("u32", Primitive::U32),
    ("u64", Primitive::U64),
    ("f32", Primitive::F32),
    ("f64", Primitive::F64),
    ("string", Primitive::String),
    ("bytes", Primitive::Bytes),
];

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
    String,
    Bytes,
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
            Primitive::String => 2, // 16-bit offset to actual string
            Primitive::Bytes => 2,  // 16-bit offset to actual bytes
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
            Primitive::String => write!(f, "string"),
            Primitive::Bytes => write!(f, "bytes"),
        }
    }
}

/// A sequence of fields.
pub struct Sequence {
    /// The name of the sequence.
    pub name: String,

    /// The fields of the sequence.
    pub fields: Vec<Field>,
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

/// A type.
#[derive(Clone, Debug)]
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
    OneOf(Vec<Field>),
}

impl Type {
    /// Get the size of the type in bytes. This is the fixed size that the type will take up in a
    /// sequence or oneof. It does not account for any dynamic sizes such as the size of a string
    /// that are added to the end of the structure.
    pub fn size(&self) -> usize {
        match self {
            Self::Primitive(p) => p.size(),
            Self::Sequence(_) => 2, // 16-bit offset to actual sequence
            Self::Enum(_) => 1,     // 8-bit enum value
            Self::Array(_) => 4,    // 16-bit array length + 16-bit offset to actual array
            Self::OneOf(_) => 3,    // 8-bit index + 16-bit offset to actual field
        }
    }
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
    pub value: u8,
}

/// Determines whether a structure is a sequence or an enum.
#[derive(Clone, Copy, PartialEq)]
enum StructType {
    Sequence,
    Enum,
}

/// The result of parsing a SyntaxTree.
pub struct ParseResult {
    /// The sequences in the SyntaxTree.
    pub sequences: Vec<Sequence>,

    /// The enums in the SyntaxTree.
    pub enums: Vec<Enum>,
}

/// Parse a SyntaxTree into a series of sequences and enums. This will verify that all types are
/// valid.
///
/// # Arguments
///
/// * `root` - The root of the SyntaxTree.
///
/// # Returns
///
/// The result of parsing the SyntaxTree or an error if the SyntaxTree is invalid.
pub fn parse_ast<'a>(
    root: &'a TaggedSyntaxTree<'a>,
) -> Result<ParseResult, Box<CompilerError<'a>>> {
    // make a map from strings to sequences and enums. This is used to verify that all types are
    // valid and unique.
    let mut struct_map: HashMap<String, StructType> = HashMap::new();
    for node in root.iter_depth_first() {
        let (name, struct_type) = match &node.data {
            SyntaxTree::Sequence(name, _) => (name.clone(), StructType::Sequence),
            SyntaxTree::Enum(name, _) => (name.clone(), StructType::Enum),
            _ => continue,
        };
        if let Err(message) = verify_struct_name(name.as_str(), &struct_map) {
            return Err(Box::new(CompilerError::<'a>::new(
                node.token.clone(),
                message,
            )));
        }
        struct_map.insert(name.clone(), struct_type);
    }

    // Construct the result.
    let mut result = ParseResult {
        sequences: Vec::new(),
        enums: Vec::new(),
    };
    let file_contents = match &root.data {
        SyntaxTree::File(file) => file,
        _ => unreachable!("Root node is not a file"),
    };

    // All top level nodes must be sequences or enums. Parse them.
    for top_level in file_contents {
        match &top_level.data {
            SyntaxTree::Sequence(name, fields) => {
                result
                    .sequences
                    .push(parse_sequence(name.clone(), fields, &struct_map)?)
            }
            SyntaxTree::Enum(name, entries) => {
                result.enums.push(parse_enum(name.clone(), entries)?)
            }
            _ => unreachable!("Top level node is not a sequence or enum"),
        }
    }

    Ok(result)
}

/// Verifies that a struct name is not reserved and is unique.
fn verify_struct_name(name: &str, struct_map: &HashMap<String, StructType>) -> Result<(), String> {
    // Check if the name is reserved.
    for (primitive_name, _) in PRIMITIVES.iter() {
        if name == *primitive_name {
            return Err(format!("Name \"{}\" is reserved", name));
        }
    }

    // Check if the name is already used.
    if struct_map.contains_key(name) {
        return Err(format!(
            "A structure with the name \"{}\" already exists",
            name
        ));
    }

    Ok(())
}

/// Parse a sequence.
fn parse_sequence<'a>(
    name: String,
    fields: &Vec<TaggedSyntaxTree<'a>>,
    struct_map: &HashMap<String, StructType>,
) -> Result<Sequence, Box<CompilerError<'a>>> {
    let mut res = Vec::with_capacity(fields.len());
    let mut field_names = Vec::<String>::with_capacity(fields.len());

    // Parse all the fields and ensure that all field names are unique.
    let mut offset = 0;
    for field in fields {
        if let SyntaxTree::Field(field_name, field_type) = &field.data {
            // Check if the field name is unique.
            if field_names.contains(field_name) {
                return Err(Box::new(CompilerError::new(
                    field.token.clone(),
                    format!(
                        "Field \"{}\" already exists in sequence \"{}\"",
                        field_name.cyan().bold(),
                        name.cyan().bold()
                    ),
                )));
            }
            field_names.push(field_name.clone());

            // Parse the field type.
            let field_type = parse_type(field_type, struct_map)?;
            let field_size = field_type.size();
            res.push(Field {
                name: field_name.clone(),
                ty: field_type,
                index: offset,
            });
            offset += field_size;
        } else {
            unreachable!("Field is not a field")
        }
    }

    Ok(Sequence { name, fields: res })
}

/// Parse a type.
fn parse_type<'a>(
    ty: &TaggedSyntaxTree<'a>,
    struct_map: &HashMap<String, StructType>,
) -> Result<Type, Box<CompilerError<'a>>> {
    match &ty.data {
        // Type is a simple named type. This can be a primitive, sequence, or enum. Verify that the
        // type is valid and parse it.
        SyntaxTree::Type(name) => {
            // Check if the type is a sequence or enum.
            if let Some(struct_type) = struct_map.get(name) {
                match struct_type {
                    StructType::Sequence => Ok(Type::Sequence(name.clone())),
                    StructType::Enum => Ok(Type::Enum(name.clone())),
                }
            }
            // Type is not a sequence or enum. Check if it is a primitive.
            else {
                for (primitive_name, primitive) in PRIMITIVES.iter() {
                    if name == *primitive_name {
                        return Ok(Type::Primitive(primitive.clone()));
                    }
                }

                // Type is not a primitive, sequence, or enum. Error.
                Err(Box::new(CompilerError::new(
                    ty.token.clone(),
                    format!("Type \"{}\" is not a valid type", name),
                )))
            }
        }

        // Type is an array. Parse the type of the array.
        SyntaxTree::Array(ty) => Ok(Type::Array(Box::new(parse_type(ty, struct_map)?))),

        // Type is a oneof. Parse all the types in the oneof.
        SyntaxTree::OneOf(fields) => {
            let mut res = Vec::with_capacity(fields.len());
            let mut field_names = Vec::<String>::with_capacity(fields.len());

            // Parse all fields and ensure that all field names are unique.
            for (i, field) in fields.iter().enumerate() {
                if let SyntaxTree::Field(field_name, field_type) = &field.data {
                    // Check if the field name is unique.
                    if field_names.contains(field_name) {
                        return Err(Box::new(CompilerError::new(
                            field.token.clone(),
                            format!(
                                "Field \"{}\" already exists in oneof",
                                field_name.cyan().bold()
                            ),
                        )));
                    }
                    field_names.push(field_name.clone());

                    // Parse the field type.
                    let field_type = parse_type(field_type, struct_map)?;
                    res.push(Field {
                        name: field_name.clone(),
                        ty: field_type,
                        index: i,
                    });
                } else {
                    unreachable!("Field is not a field")
                }
            }

            Ok(Type::OneOf(res))
        }

        _ => unreachable!("Type is not a type"),
    }
}

/// Parse an enum.
fn parse_enum<'a>(
    name: String,
    entries: &Vec<TaggedSyntaxTree<'a>>,
) -> Result<Enum, Box<CompilerError<'a>>> {
    let mut variants = Vec::<EnumVariant>::new();

    // Parse all the entries.
    for entry in entries {
        if let SyntaxTree::EnumEntry(entry_name, entry_value) = &entry.data {
            // Check if the entry value is a valid integer.
            let parsed_value = match entry_value.parse::<u8>() {
                Ok(value) => value,
                Err(_) => {
                    let full_name = format!("{}:{}", name, entry_name);
                    return Err(Box::new(CompilerError::new(
                        entry.token.clone(),
                        format!(
                            "Value \"{}\" for enum entry \"{}\" is not a valid integer",
                            entry_value.cyan().bold(),
                            full_name.cyan().bold()
                        ),
                    )));
                }
            };

            // Make sure entry does not have a duplicate name or value.
            for variant in variants.iter() {
                if variant.name == *entry_name {
                    return Err(Box::new(CompilerError::new(
                        entry.token.clone(),
                        format!(
                            "Enum entry \"{}\" already exists in enum \"{}\"",
                            entry_name.cyan().bold(),
                            name.cyan().bold()
                        ),
                    )));
                }
                if variant.value == parsed_value {
                    let full_name_1 = format!("{}:{}", name, variant.name);
                    let full_name_2 = format!("{}:{}", name, entry_name);
                    return Err(Box::new(CompilerError::new(
                        entry.token.clone(),
                        format!(
                            "Enum entries \"{}\" and \"{}\" have the same value",
                            full_name_1.cyan().bold(),
                            full_name_2.cyan().bold()
                        ),
                    )));
                }
            }

            // Add the entry to the enum.
            variants.push(EnumVariant {
                name: entry_name.clone(),
                value: parsed_value,
            });
        } else {
            unreachable!("Entry is not an entry")
        }
    }

    Ok(Enum { name, variants })
}
