//! This file parses a SyntaxTree into a series of sequences and enums.

use super::CompilerError;
use crate::ast::{SyntaxTree, TaggedSyntaxTree, TreeTraversal};
use colored::Colorize;
use simplebuffers_core::*;
use std::collections::HashMap;

/// Array of primitive names and internal representations.
const PRIMITIVES: [(&str, Primitive); 11] = [
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
];

/// Determines whether a structure is a sequence or an enum.
#[derive(Clone, Copy, PartialEq)]
enum StructType {
    Sequence,
    Enum,
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
pub fn parse_ast<'a>(root: &'a TaggedSyntaxTree<'a>) -> Result<SBSchema, Box<CompilerError<'a>>> {
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
    let mut result = SBSchema {
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

    // Inject enum size into all enum types.
    for enm in &result.enums {
        for sequence in &mut result.sequences {
            inject_enum_size_into(&enm.name, enm.size.into(), &mut sequence.fields);
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
            // Check if the type is a string.
            if name == "string" {
                Ok(Type::String)
            }
            // Check if the type is a sequence or enum.
            else if let Some(struct_type) = struct_map.get(name) {
                match struct_type {
                    StructType::Sequence => Ok(Type::Sequence(name.clone())),
                    StructType::Enum => Ok(Type::Enum(name.clone(), 0)),
                }
            }
            // Type is not a string, sequence, or enum. Check if it is a primitive.
            else if let Some(found) = PRIMITIVES.iter().find(|&x| x.0 == name) {
                Ok(Type::Primitive(found.1.clone()))
            }
            // Type is not a primitive, string, sequence, or enum. Error.
            else {
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
    let mut enum_size = 1;
    for entry in entries {
        if let SyntaxTree::EnumEntry(entry_name, entry_value) = &entry.data {
            // Check if the entry value is a valid integer.
            let parsed_value = match entry_value.parse::<u64>() {
                Ok(value) => value,
                Err(e) => {
                    let full_name = format!("{}:{}", name, entry_name);
                    return Err(Box::new(CompilerError::new(
                        entry.token.clone(),
                        format!(
                            "Value \"{}\" for enum entry \"{}\" is not a valid integer: {}",
                            entry_value.cyan().bold(),
                            full_name.cyan().bold(),
                            e.to_string().italic()
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

            // Check if we must increase the size of the enum to accommodate this new value. We do
            // not have to handle overflows here, since that is checked when we first call
            // `entry_value.parse::<u64>`. If the provided value is larger than 64 bits, it would
            // not have been parsed.
            for (size, max_val) in [
                (1, u8::MAX.into()),
                (2, u16::MAX.into()),
                (4, u32::MAX.into()),
                (8, u64::MAX),
            ] {
                if enum_size > size {
                    continue;
                }
                if parsed_value <= max_val {
                    enum_size = size;
                    break;
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

    Ok(Enum {
        name,
        size: enum_size,
        variants,
    })
}

/// Finds any enum fields that match the given name and injects the given size into them.
fn inject_enum_size_into(enum_name: &str, enum_size: usize, fields: &mut [Field]) {
    /// Injects the enum into a single Type instance. Returns the size that the field increased by
    /// so that it can be added to `adjust_by`.
    fn process_type(enum_name: &str, enum_size: usize, ty: &mut Type) -> usize {
        match ty {
            Type::Enum(found_name, found_size) => {
                if found_name == enum_name {
                    *found_size = enum_size;
                    enum_size
                } else {
                    0
                }
            }
            Type::Array(b) => {
                process_type(enum_name, enum_size, b.as_mut());
                0
            }
            Type::OneOf(subfields) => {
                inject_enum_size_into(enum_name, enum_size, subfields);
                0
            }
            _ => 0,
        }
    }

    let mut adjust_by = 0;
    for field in fields {
        field.index += adjust_by;
        adjust_by += process_type(enum_name, enum_size, &mut field.ty);
    }
}
