use colored::Colorize;
use std::fmt;

use crate::tokenizer::{Token, TokenLocation, TokenType};

/// An error that is returned by the AST builder.
#[derive(Debug)]
pub enum AstBuilderError<'a> {
    /// An unexpected token was encountered.
    UnexpectedToken {
        /// The token type that was encountered.
        token_type: TokenType,

        /// An optional error message.
        message: Option<String>,

        /// The location of the token.
        location: TokenLocation<'a>,
    },

    /// An unexpected end of file was encountered.
    UnexpectedEof {
        /// The file where the unexpected end of file was encountered.
        file: String,
    },
}

impl<'a> AstBuilderError<'a> {
    /// Generate a new `AstBuilderError::UnexpectedToken` variant with the given token and message.
    pub fn unexpected_token(token: &Token<'a>, message: Option<String>) -> AstBuilderError<'a> {
        Self::UnexpectedToken {
            token_type: token.token_type.clone(),
            message,
            location: token.location.clone(),
        }
    }
}

impl<'a> std::error::Error for AstBuilderError<'a> {}

impl<'a> fmt::Display for AstBuilderError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedToken {
                token_type,
                message,
                location,
            } => {
                if let Some(message) = message {
                    write!(
                        f,
                        "{error_str} Unexpected token `{token}` ({message})\n{location}",
                        error_str = "ERROR:".red().bold(),
                        token = token_type.to_string().blue().bold(),
                        message = message,
                        location = location,
                    )
                } else {
                    write!(
                        f,
                        "{error_str} Unexpected token `{token}`\n{location}",
                        error_str = "ERROR:".red().bold(),
                        token = token_type.to_string().blue().bold(),
                        location = location,
                    )
                }
            }

            Self::UnexpectedEof { file } => write!(
                f,
                "{error_str} Unexpected end of file in {file}",
                error_str = "ERROR:".red().bold(),
                file = file.green().underline(),
            ),
        }
    }
}
