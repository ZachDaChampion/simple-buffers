use ast::{SyntaxTree, TaggedSyntaxTree};
use compiler::ParseResult;
use std::{collections::LinkedList, error::Error};

use crate::compiler::{Enum, EnumVariant};

mod ast;
mod compiler;
mod indent;
mod tokenizer;

fn main() {
    // Load test.sb into a string.
    let source = std::fs::read_to_string("test.sb").expect("Failed to read test.sb");

    // Run the compiler.
    println!();
    if let Err(e) = do_stuff(source) {
        eprintln!("{}", e);
    }
}

enum Visitor<'a, T>
where
    T: 'a,
{
    Visit(&'a T),
    Cleanup(&'a T),
}

fn do_stuff(source: String) -> Result<(), String> {
    let mut parser =
        ast::AstBuilder::new(source.as_str(), "test.sb").expect("Failed to create parser");
    let ast = parser.parse().map_err(|e| e.to_string())?;

    print_ast(&ast).map_err(|e| e.to_string())?;

    let parsed = compiler::parse_ast(&ast).map_err(|e| e.to_string())?;
    print_parsed(parsed).map_err(|e| e.to_string())?;

    Ok(())
}

fn print_ast(ast: &TaggedSyntaxTree<'_>) -> Result<(), Box<dyn Error>> {
    let mut stack = LinkedList::new();
    stack.push_back(Visitor::Visit(ast));
    let mut indent = 0;

    print!(concat!(
        "=========================\n",
        "|      SYNTAX TREE      |\n",
        "=========================\n\n"
    ));

    while let Some(action) = stack.pop_back() {
        match action {
            Visitor::Visit(node) => {
                print!("{}", "|  ".repeat(indent));
                match &node.data {
                    SyntaxTree::File(children) => {
                        println!("File");
                        stack.push_back(Visitor::Cleanup(node));
                        for child in children.iter().rev() {
                            stack.push_back(Visitor::Visit(child));
                        }
                        indent += 1;
                    }
                    SyntaxTree::Sequence(identifier, fields) => {
                        println!("Sequence: {}", identifier);
                        stack.push_back(Visitor::Cleanup(node));
                        for field in fields.iter().rev() {
                            stack.push_back(Visitor::Visit(field));
                        }
                        indent += 1;
                    }
                    SyntaxTree::Field(identifier, field_type) => {
                        println!("Field: {}", identifier);
                        stack.push_back(Visitor::Cleanup(node));
                        stack.push_back(Visitor::Visit(field_type));
                        indent += 1;
                    }
                    SyntaxTree::Enum(identifier, entries) => {
                        println!("Enum: {}", identifier);
                        stack.push_back(Visitor::Cleanup(node));
                        for entry in entries.iter().rev() {
                            stack.push_back(Visitor::Visit(entry));
                        }
                        indent += 1;
                    }
                    SyntaxTree::EnumEntry(identifier, value) => {
                        println!("EnumEntry: {} = {}", identifier, value);
                        stack.push_back(Visitor::Cleanup(node));
                    }
                    SyntaxTree::Type(name) => {
                        println!("Type: {}", name);
                        stack.push_back(Visitor::Cleanup(node));
                    }
                    SyntaxTree::Array(inner) => {
                        println!("Array");
                        stack.push_back(Visitor::Cleanup(node));
                        stack.push_back(Visitor::Visit(inner));
                        indent += 1;
                    }
                    SyntaxTree::OneOf(fields) => {
                        println!("OneOf");
                        stack.push_back(Visitor::Cleanup(node));
                        for field in fields.iter().rev() {
                            stack.push_back(Visitor::Visit(field));
                        }
                        indent += 1;
                    }
                }
            }
            Visitor::Cleanup(node) => match node.data {
                SyntaxTree::File(_)
                | SyntaxTree::Sequence(_, _)
                | SyntaxTree::Field(_, _)
                | SyntaxTree::Enum(_, _)
                | SyntaxTree::Array(_)
                | SyntaxTree::OneOf(_) => {
                    indent -= 1;
                }
                _ => {}
            },
        }
    }

    println!();
    Ok(())
}

fn print_parsed(parsed: ParseResult) -> Result<(), Box<dyn Error>> {
    print!(concat!(
        "=========================\n",
        "|         ENUMS         |\n",
        "=========================\n\n"
    ));
    for Enum { name, variants } in parsed.enums.iter() {
        println!("{}:", name);
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
    for sequence in parsed.sequences.iter() {
        println!("{}:", sequence.name);
        for field in sequence.fields.iter() {
            let mut indent = 1;
            let mut stack = LinkedList::new();
            stack.push_back((Some(field.name.clone()), &field.ty, field.index));
            while let Some((field_name, field_type, field_offset)) = stack.pop_back() {
                if let Some(n) = field_name {
                    print!(
                        "{indent}{offset} | {name}: ",
                        offset = field_offset,
                        indent = "  ".repeat(indent),
                        name = n
                    );
                }
                match &field_type {
                    compiler::Type::Primitive(name) => println!("{} (primitive)", name),
                    compiler::Type::Sequence(name) => println!("{} (sequence)", name),
                    compiler::Type::Enum(name) => println!("{} (enum)", name),
                    compiler::Type::Array(ty) => {
                        print!("ARRAY OF ");
                        stack.push_back((None, ty, 0));
                    }
                    compiler::Type::String => println!("string"),
                    compiler::Type::OneOf(f) => {
                        println!("ONE OF:");
                        for field in f.iter().rev() {
                            stack.push_back((Some(field.name.clone()), &field.ty, field.index));
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
