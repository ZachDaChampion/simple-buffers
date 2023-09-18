//! An AST builder can parse a string into a syntax tree. This parser implements recursive descent
//! parsing. See: <http://craftinginterpreters.com/parsing-expressions.html>
//!
//! # Grammar
//! - file       ->  (sequence | enum)* EOF
//! - sequence   ->  sequence" IDENTIFIER "{" (field ";")* "}"
//! - field      ->  IDENTIFIER ":" type
//! - enum       ->  enum" IDENTIFIER "{" (enum_entry ";")* "}"
//! - enum_entry ->  IDENTIFIER "=" NUMBER
//! - type       ->  IDENTIFIER | array | oneof
//! - array      ->  "[" type "]"
//! - oneof      ->  "oneof" "{" (field ";")* "}"

mod error;
mod traverse;
pub use self::error::AstBuilderError;
pub use traverse::*;

use crate::tokenizer::{Token, TokenIterator, TokenLocation, TokenType, Tokenizer};
use colored::Colorize;
use std::error::Error;

pub struct TaggedSyntaxTree<'a> {
    /// The data of this node.
    pub data: SyntaxTree<'a>,

    /// The main token associated with this node.
    pub token: Option<Token<'a>>,
}

impl<'a> TaggedSyntaxTree<'a> {
    /// Creates a new syntax tree.
    ///
    /// # Arguments
    ///
    /// * `data` - The data of this node.
    /// * `token` - The main token associated with this node.
    ///
    /// # Returns
    ///
    /// A new syntax tree.
    pub fn new(data: SyntaxTree<'a>, token: Token<'a>) -> Self {
        Self {
            data,
            token: Some(token),
        }
    }
}

/// The data of a syntax tree node.
pub enum SyntaxTree<'a> {
    File(Vec<TaggedSyntaxTree<'a>>),
    Sequence(String, Vec<TaggedSyntaxTree<'a>>),
    Field(String, Box<TaggedSyntaxTree<'a>>),
    Enum(String, Vec<TaggedSyntaxTree<'a>>),
    EnumEntry(String, i32),
    Type(String),
    Array(Box<TaggedSyntaxTree<'a>>),
    OneOf(Vec<TaggedSyntaxTree<'a>>),
}

impl<'a> SyntaxTree<'a> {
    pub fn tag(self, token: Token<'a>) -> TaggedSyntaxTree<'a> {
        TaggedSyntaxTree::new(self, token)
    }
}

impl<'a> From<SyntaxTree<'a>> for TaggedSyntaxTree<'a> {
    fn from(data: SyntaxTree<'a>) -> Self {
        Self { data, token: None }
    }
}

/// An AST builder that lazily parses a string into a syntax tree.
pub struct AstBuilder<'a> {
    /// The name of the file being parsed.
    file: &'a str,

    /// An iterator that yields tokens from the source string.
    tokens: Box<TokenIterator<'a>>,

    /// The current token.
    current_token: Option<Token<'a>>,
}

/// A result type for parsing. This is a convenience type alias.
pub type AstBuildResult<'a> = Result<TaggedSyntaxTree<'a>, Box<dyn Error + 'a>>;

impl<'a> AstBuilder<'a> {
    /// Creates an AstBuilder at the beginning of the source string. This will construct a Tokenizer
    /// internally.
    ///
    /// # Arguments
    ///
    /// * `source` - The source string to parse.
    /// * `file` - The name of the file being parsed.
    ///
    /// # Returns
    ///
    /// An AstBuilder at the beginning of the source string.
    pub fn new(source: &'a str, file: &'a str) -> Result<Self, Box<dyn Error + 'a>> {
        let mut tokens = Box::new(Tokenizer::new(source, file)?);
        let current_token = tokens.next().transpose()?;
        Ok(Self {
            file,
            tokens,
            current_token,
        })
    }

    /// Parses the source string into a syntax tree.
    pub fn parse(&mut self) -> AstBuildResult<'a> {
        self.parse_file()
    }

    /// Parses the file rule.
    /// file -> (sequence | enum)* EOF
    fn parse_file(&mut self) -> AstBuildResult<'a> {
        let mut file = Vec::new();
        while let Some(token) = &self.current_token {
            match token.token_type {
                TokenType::Sequence => file.push(self.parse_sequence()?),
                TokenType::Enum => file.push(self.parse_enum()?),
                _ => {
                    return Err(Box::new(AstBuilderError::unexpected_token(
                        token,
                        Some("expected a \"sequence\" or \"enum\"".to_string()),
                    )));
                }
            }
        }
        Ok(SyntaxTree::File(file).into())
    }

    /// Parses the sequence rule.
    /// sequence -> "sequence" IDENTIFIER "{" (field ";")* "}"
    fn parse_sequence(&mut self) -> AstBuildResult<'a> {
        let tag = self.expect(TokenType::Sequence)?;
        let name = self.expect_identifier()?;
        self.expect(TokenType::OpenBrace)?;
        let mut fields = Vec::new();
        loop {
            match &self.current_token {
                Some(token) => match token.token_type {
                    TokenType::Identifier(_) => {
                        fields.push(self.parse_field()?);
                        self.expect(TokenType::Semicolon)?;
                    }
                    TokenType::CloseBrace => break,
                    _ => {
                        return Err(Box::new(AstBuilderError::unexpected_token(
                            token,
                            Some("expected an identifier or \"}\"".to_string()),
                        )));
                    }
                },
                None => {
                    return Err(Box::new(AstBuilderError::UnexpectedEof {
                        file: self.file.to_string(),
                    }))
                }
            }
        }
        self.expect(TokenType::CloseBrace)?;
        Ok(SyntaxTree::Sequence(name, fields).tag(tag))
    }

    /// Parses the field rule.
    /// field -> IDENTIFIER ":" type
    fn parse_field(&mut self) -> AstBuildResult<'a> {
        let (name, tag) = self.expect_identifier_with_token()?;
        self.expect(TokenType::Colon)?;
        let field_type = self.parse_type()?;
        Ok(SyntaxTree::Field(name, Box::new(field_type)).tag(tag))
    }

    /// Parses the enum rule.
    /// enum -> "enum" IDENTIFIER "{" (enum_entry ";")* "}"
    fn parse_enum(&mut self) -> AstBuildResult<'a> {
        let tag = self.expect(TokenType::Enum)?;
        let name = self.expect_identifier()?;
        self.expect(TokenType::OpenBrace)?;
        let mut entries = Vec::new();
        loop {
            match &self.current_token {
                Some(token) => match token.token_type {
                    TokenType::Identifier(_) => {
                        entries.push(self.parse_enum_entry()?);
                        self.expect(TokenType::Semicolon)?;
                    }
                    TokenType::CloseBrace => break,
                    _ => {
                        return Err(Box::new(AstBuilderError::unexpected_token(
                            token,
                            Some("expected an identifier or \"}\"".to_string()),
                        )));
                    }
                },
                None => {
                    return Err(Box::new(AstBuilderError::UnexpectedEof {
                        file: self.file.to_string(),
                    }))
                }
            }
        }
        self.expect(TokenType::CloseBrace)?;
        Ok(SyntaxTree::Enum(name, entries).tag(tag))
    }

    /// Parses the enum_entry rule.
    /// enum_entry -> IDENTIFIER "=" NUMBER
    fn parse_enum_entry(&mut self) -> AstBuildResult<'a> {
        let (name, tag) = self.expect_identifier_with_token()?;
        self.expect(TokenType::Equals)?;
        let value = self.expect_number()?;
        let value_num =
            value
                .parse::<i32>()
                .or(Err(Box::new(AstBuilderError::unexpected_token(
                    &Token {
                        token_type: TokenType::Number(value),
                        location: TokenLocation {
                            file: self.file,
                            line_num: 0,
                            col_num: 0,
                            width: 0,
                            prev_line_text: None,
                            line_text: None,
                            next_line_text: None,
                        },
                    },
                    Some("expected a number literal".to_string()),
                ))))?;
        Ok(SyntaxTree::EnumEntry(name, value_num).tag(tag))
    }

    /// Parses the type rule.
    /// type -> IDENTIFIER | array | oneof
    fn parse_type(&mut self) -> AstBuildResult<'a> {
        match &self.current_token {
            Some(token) => match token.token_type {
                TokenType::Identifier(_) => {
                    let tag = token.clone();
                    let name = self.expect_identifier()?;
                    Ok(SyntaxTree::Type(name).tag(tag))
                }
                TokenType::OpenBracket => self.parse_array(),
                TokenType::Oneof => self.parse_oneof(),
                _ => Err(Box::new(AstBuilderError::unexpected_token(
                    token,
                    Some("expected a type or \"oneof\"".to_string()),
                ))),
            },
            None => Err(Box::new(AstBuilderError::UnexpectedEof {
                file: self.file.to_string(),
            })),
        }
    }

    /// Parses the array rule.
    /// array -> "[" type "]"
    fn parse_array(&mut self) -> AstBuildResult<'a> {
        let tag = self.expect(TokenType::OpenBracket)?;
        let array_type = self.parse_type()?;
        self.expect(TokenType::CloseBracket)?;
        Ok(SyntaxTree::Array(Box::new(array_type)).tag(tag))
    }

    /// Parses the oneof rule.
    /// oneof -> "oneof" "{" (field ";")* "}"
    fn parse_oneof(&mut self) -> AstBuildResult<'a> {
        let tag = self.expect(TokenType::Oneof)?;
        self.expect(TokenType::OpenBrace)?;
        let mut fields = Vec::new();
        loop {
            match &self.current_token {
                Some(token) => match token.token_type {
                    TokenType::Identifier(_) => {
                        fields.push(self.parse_field()?);
                        self.expect(TokenType::Semicolon)?;
                    }
                    TokenType::CloseBrace => break,
                    _ => {
                        return Err(Box::new(AstBuilderError::unexpected_token(
                            token,
                            Some("expected an identifier or \"}\"".to_string()),
                        )));
                    }
                },
                None => {
                    return Err(Box::new(AstBuilderError::UnexpectedEof {
                        file: self.file.to_string(),
                    }))
                }
            }
        }
        self.expect(TokenType::CloseBrace)?;
        Ok(SyntaxTree::OneOf(fields).tag(tag))
    }

    /// Advances the parser to the next token.
    fn advance(&mut self) -> Result<(), Box<dyn Error + 'a>> {
        self.current_token = self.tokens.next().transpose()?;
        Ok(())
    }

    /// Expects the current token to be of the provided type. If it is not, an error is returned.
    /// If the current token is of the provided type, it is consumed and the next token is loaded.
    ///
    /// # Arguments
    ///
    /// * `token_type` - The type of token to expect.
    ///
    /// # Returns
    ///
    /// The current token if it is of the provided type.
    fn expect(&mut self, token_type: TokenType) -> Result<Token<'a>, Box<dyn Error + 'a>> {
        match &self.current_token {
            Some(token) => {
                if token.token_type != token_type {
                    return Err(Box::new(AstBuilderError::unexpected_token(
                        token,
                        Some(format!("expected: \"{}\"", token_type.to_string().bold())),
                    )));
                }
                let res = token.clone();
                self.advance()?;
                Ok(res)
            }
            None => Err(Box::new(AstBuilderError::UnexpectedEof {
                file: self.file.to_string(),
            })),
        }
    }

    /// Expects the current token to be an identifier. If it is not, an error is returned. If the
    /// current token is an identifier, it is consumed and the next token is loaded.
    ///
    /// # Returns
    ///
    /// The identifier if the current token is an identifier.
    fn expect_identifier(&mut self) -> Result<String, Box<dyn Error + 'a>> {
        match &self.current_token {
            Some(token) => {
                if let TokenType::Identifier(identifier) = &token.token_type {
                    let res = identifier.clone();
                    self.advance()?;
                    Ok(res)
                } else {
                    Err(Box::new(AstBuilderError::unexpected_token(
                        token,
                        Some("expected an identifier".to_string()),
                    )))
                }
            }
            None => Err(Box::new(AstBuilderError::UnexpectedEof {
                file: self.file.to_string(),
            })),
        }
    }

    /// Expects the current token to be an identifier and returns the token in addition to the
    /// identifier. If the current token is not an identifier, an error is returned. If the current
    /// token is an identifier, it is consumed and the next token is loaded.
    ///
    /// # Returns
    ///
    /// A tuple containing the identifier and the token.
    fn expect_identifier_with_token(&mut self) -> Result<(String, Token<'a>), Box<dyn Error + 'a>> {
        match &self.current_token {
            Some(token) => {
                if let TokenType::Identifier(identifier) = &token.token_type {
                    let res = (identifier.clone(), token.clone());
                    self.advance()?;
                    Ok(res)
                } else {
                    Err(Box::new(AstBuilderError::unexpected_token(
                        token,
                        Some("expected an identifier".to_string()),
                    )))
                }
            }
            None => Err(Box::new(AstBuilderError::UnexpectedEof {
                file: self.file.to_string(),
            })),
        }
    }

    /// Expects a number literal. If the current token is not a number literal, an error is
    /// returned. If the current token is a number literal, it is consumed and the next token is
    /// loaded.
    ///
    /// # Returns
    ///
    /// The number literal if the current token is a number literal.
    fn expect_number(&mut self) -> Result<String, Box<dyn Error + 'a>> {
        match &self.current_token {
            Some(token) => {
                if let TokenType::Number(number) = &token.token_type {
                    let res = number.clone();
                    self.advance()?;
                    Ok(res)
                } else {
                    Err(Box::new(AstBuilderError::unexpected_token(
                        token,
                        Some("expected a number literal".to_string()),
                    )))
                }
            }
            None => Err(Box::new(AstBuilderError::UnexpectedEof {
                file: self.file.to_string(),
            })),
        }
    }
}
