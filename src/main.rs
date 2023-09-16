use ast::SyntaxTree;
use std::{collections::LinkedList, error::Error};

mod ast;
mod tokenizer;

fn main() {
    // Load test.sb into a string.
    let source = std::fs::read_to_string("test.sb").expect("Failed to read test.sb");

    if let Err(e) = print_ast(source) {
        println!("\n{}\n", e);
    }
}

enum Visitor<'a> {
    Visit(&'a SyntaxTree),
    Cleanup(&'a SyntaxTree),
}

fn print_ast(source: String) -> Result<(), Box<dyn Error>> {
    let mut parser =
        ast::AstBuilder::new(source.as_str(), "test.sb").expect("Failed to create parser");

    let ast = parser.parse()?;
    let mut stack = LinkedList::new();
    stack.push_back(Visitor::Visit(&ast));
    let mut indent = 0;

    while let Some(action) = stack.pop_back() {
        match action {
            Visitor::Visit(node) => {
                print!("{}", "|  ".repeat(indent));
                match node {
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
                    SyntaxTree::Oneof(fields) => {
                        println!("OneOf");
                        stack.push_back(Visitor::Cleanup(node));
                        for field in fields.iter().rev() {
                            stack.push_back(Visitor::Visit(field));
                        }
                        indent += 1;
                    }
                }
            }
            Visitor::Cleanup(node) => match node {
                SyntaxTree::File(_)
                | SyntaxTree::Sequence(_, _)
                | SyntaxTree::Field(_, _)
                | SyntaxTree::Enum(_, _)
                | SyntaxTree::Array(_)
                | SyntaxTree::Oneof(_) => {
                    indent -= 1;
                }
                _ => {}
            },
        }
    }

    Ok(())
}
