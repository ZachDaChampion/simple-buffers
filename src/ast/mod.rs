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

use std::error::Error;

use crate::tokenizer::{Token, TokenIterator, TokenType, Tokenizer};

use self::error::AstBuilderError;

mod error;

pub enum SyntaxTree {
    File(Vec<SyntaxTree>),
    Sequence(String, Vec<SyntaxTree>),
    Field(String, Box<SyntaxTree>),
    Enum(String, Vec<SyntaxTree>),
    EnumEntry(String, i32),
    Type(String),
    Array(Box<SyntaxTree>),
    Oneof(Vec<SyntaxTree>),
}

/// An AST builder that lazily parses a string into a syntax tree.
pub struct AstBuilder<'a> {
    /// The source string to parse.
    source: &'a str,

    /// The name of the file being parsed.
    file: &'a str,

    /// An iterator that yields tokens from the source string.
    tokens: Box<TokenIterator<'a>>,

    /// The current token.
    current_token: Option<Token>,
}

/// A result type for parsing. This is a convenience type alias.
pub type AstBuildResult = Result<SyntaxTree, Box<dyn Error>>;

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
    pub fn new(source: &'a str, file: &'a str) -> Result<Self, Box<dyn Error>> {
        let mut tokens = Box::new(Tokenizer::new(source, file)?);
        let current_token = tokens.next().transpose()?;
        Ok(Self {
            source,
            file,
            tokens,
            current_token,
        })
    }

    /// Parses the source string into a syntax tree.
    pub fn parse(&mut self) -> AstBuildResult {
        self.parse_file()
    }

    /// Parses the file rule.
    /// file -> (sequence | enum)* EOF
    fn parse_file(&mut self) -> AstBuildResult {
        let mut file = Vec::new();
        while let Some(token) = &self.current_token {
            match token.token_type {
                TokenType::Sequence => file.push(self.parse_sequence()?),
                TokenType::Enum => file.push(self.parse_enum()?),
                _ => {
                    return Err(Box::new(AstBuilderError::UnexpectedToken {
                        file: self.file.to_string(),
                        token: token.clone(),
                        message: Some("Expected a sequence or enum".to_string()),
                    }))
                }
            }
        }
        Ok(SyntaxTree::File(file))
    }

    /// Parses the sequence rule.
    /// sequence -> "sequence" IDENTIFIER "{" (field ";")* "}"
    fn parse_sequence(&mut self) -> AstBuildResult {
        self.expect(TokenType::Sequence)?;
        let name = self.expect_identifier()?;
        self.expect(TokenType::OpenBrace)?;
        let mut fields = Vec::new();
        loop {
            match &self.current_token {
                Some(token) => match token.token_type {
                    TokenType::Identifier(_) => fields.push(self.parse_field()?),
                    TokenType::CloseBrace => break,
                    _ => {
                        return Err(Box::new(AstBuilderError::UnexpectedToken {
                            file: self.file.to_string(),
                            token: token.clone(),
                            message: Some("Expected an identifier or right brace".to_string()),
                        }))
                    }
                },
                None => {
                    return Err(Box::new(AstBuilderError::UnexpectedEof {
                        file: self.file.to_string(),
                        line: 0,
                        column: 0,
                    }))
                }
            }
            self.advance()?;
        }
        self.expect(TokenType::CloseBrace)?;
        Ok(SyntaxTree::Sequence(name, fields))
    }

    /// Parses the field rule.
    /// field -> IDENTIFIER ":" type
    fn parse_field(&mut self) -> AstBuildResult {
        let name = self.expect_identifier()?;
        self.expect(TokenType::Colon)?;
        let field_type = self.parse_type()?;
        Ok(SyntaxTree::Field(name, Box::new(field_type)))
    }

    /// Parses the enum rule.
    /// enum -> "enum" IDENTIFIER "{" (enum_entry ";")* "}"
    fn parse_enum(&mut self) -> AstBuildResult {
        self.expect(TokenType::Enum)?;
        let name = self.expect_identifier()?;
        self.expect(TokenType::OpenBrace)?;
        let mut entries = Vec::new();
        loop {
            match &self.current_token {
                Some(token) => match token.token_type {
                    TokenType::Identifier(_) => entries.push(self.parse_enum_entry()?),
                    TokenType::CloseBrace => break,
                    _ => {
                        return Err(Box::new(AstBuilderError::UnexpectedToken {
                            file: self.file.to_string(),
                            token: token.clone(),
                            message: Some("Expected an identifier or right brace".to_string()),
                        }))
                    }
                },
                None => {
                    return Err(Box::new(AstBuilderError::UnexpectedEof {
                        file: self.file.to_string(),
                        line: 0,
                        column: 0,
                    }))
                }
            }
            self.advance()?;
        }
        self.expect(TokenType::CloseBrace)?;
        Ok(SyntaxTree::Enum(name, entries))
    }

    /// Parses the enum_entry rule.
    /// enum_entry -> IDENTIFIER "=" NUMBER
    fn parse_enum_entry(&mut self) -> AstBuildResult {
        let name = self.expect_identifier()?;
        self.expect(TokenType::Equals)?;
        let value = self.expect_number()?;
        let value_num =
            value
                .parse::<i32>()
                .or(Err(Box::new(AstBuilderError::UnexpectedToken {
                    file: self.file.to_string(),
                    token: Token {
                        token_type: TokenType::Number(value),
                        line: 0,
                        column: 0,
                    },
                    message: Some("Expected a number literal".to_string()),
                })))?;
        Ok(SyntaxTree::EnumEntry(name, value_num))
    }

    /// Parses the type rule.
    /// type -> IDENTIFIER | array | oneof
    fn parse_type(&mut self) -> AstBuildResult {
        match &self.current_token {
            Some(token) => match token.token_type {
                TokenType::Identifier(_) => {
                    let name = self.expect_identifier()?;
                    Ok(SyntaxTree::Type(name))
                }
                TokenType::OpenBracket => self.parse_array(),
                TokenType::Oneof => self.parse_oneof(),
                _ => Err(Box::new(AstBuilderError::UnexpectedToken {
                    file: self.file.to_string(),
                    token: token.clone(),
                    message: Some("Expected an identifier, open bracket, or oneof".to_string()),
                })),
            },
            None => Err(Box::new(AstBuilderError::UnexpectedEof {
                file: self.file.to_string(),
                line: 0,
                column: 0,
            })),
        }
    }

    /// Parses the array rule.
    /// array -> "[" type "]"
    fn parse_array(&mut self) -> AstBuildResult {
        self.expect(TokenType::OpenBracket)?;
        let array_type = self.parse_type()?;
        self.expect(TokenType::CloseBracket)?;
        Ok(SyntaxTree::Array(Box::new(array_type)))
    }

    /// Parses the oneof rule.
    /// oneof -> "oneof" "{" (field ";")* "}"
    fn parse_oneof(&mut self) -> AstBuildResult {
        self.expect(TokenType::Oneof)?;
        self.expect(TokenType::OpenBrace)?;
        let mut fields = Vec::new();
        loop {
            match &self.current_token {
                Some(token) => match token.token_type {
                    TokenType::Identifier(_) => fields.push(self.parse_field()?),
                    TokenType::CloseBrace => break,
                    _ => {
                        return Err(Box::new(AstBuilderError::UnexpectedToken {
                            file: self.file.to_string(),
                            token: token.clone(),
                            message: Some("Expected an identifier or right brace".to_string()),
                        }))
                    }
                },
                None => {
                    return Err(Box::new(AstBuilderError::UnexpectedEof {
                        file: self.file.to_string(),
                        line: 0,
                        column: 0,
                    }))
                }
            }
            self.advance()?;
        }
        self.expect(TokenType::CloseBrace)?;
        Ok(SyntaxTree::Oneof(fields))
    }

    /// Advances the parser to the next token.
    fn advance(&mut self) -> Result<(), Box<dyn Error>> {
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
    fn expect(&mut self, token_type: TokenType) -> Result<Token, Box<dyn Error>> {
        match &self.current_token {
            Some(token) => {
                if token.token_type != token_type {
                    return Err(Box::new(AstBuilderError::UnexpectedToken {
                        file: self.file.to_string(),
                        token: token.clone(),
                        message: Some(format!("Expected a {:?}", token_type)),
                    }));
                }
                let res = token.clone();
                self.advance()?;
                Ok(res)
            }
            None => Err(Box::new(AstBuilderError::UnexpectedEof {
                file: self.file.to_string(),
                line: 0,
                column: 0,
            })),
        }
    }

    /// Expects the current token to be an identifier. If it is not, an error is returned. If the
    /// current token is an identifier, it is consumed and the next token is loaded.
    ///
    /// # Returns
    ///
    /// The identifier if the current token is an identifier.
    fn expect_identifier(&mut self) -> Result<String, Box<dyn Error>> {
        match &self.current_token {
            Some(token) => {
                if let TokenType::Identifier(identifier) = &token.token_type {
                    let res = identifier.clone();
                    self.advance()?;
                    Ok(res)
                } else {
                    Err(Box::new(AstBuilderError::UnexpectedToken {
                        file: self.file.to_string(),
                        token: token.clone(),
                        message: Some("Expected an identifier".to_string()),
                    }))
                }
            }
            None => Err(Box::new(AstBuilderError::UnexpectedEof {
                file: self.file.to_string(),
                line: 0,
                column: 0,
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
    fn expect_number(&mut self) -> Result<String, Box<dyn Error>> {
        match &self.current_token {
            Some(token) => {
                if let TokenType::Number(number) = &token.token_type {
                    let res = number.clone();
                    self.advance()?;
                    Ok(res)
                } else {
                    Err(Box::new(AstBuilderError::UnexpectedToken {
                        file: self.file.to_string(),
                        token: token.clone(),
                        message: Some("Expected a number literal".to_string()),
                    }))
                }
            }
            None => Err(Box::new(AstBuilderError::UnexpectedEof {
                file: self.file.to_string(),
                line: 0,
                column: 0,
            })),
        }
    }
}
