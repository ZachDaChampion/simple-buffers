use colored::Colorize;
use std::fmt;

use crate::tokenizer::{Token, TokenLocation, TokenType};

/// An error that is returned by the AST builder.
#[derive(Debug)]
pub enum AstBuilderError {
    /// An unexpected token was encountered.
    UnexpectedToken {
        /// The token type that was encountered.
        token_type: TokenType,

        /// An optional error message.
        message: Option<String>,

        /// The name of the file where the token was found.
        file: String,

        /// The line number where the token was found (0-indexed).
        line_num: usize,

        /// The column number where the token was found (0-indexed).
        col_num: usize,

        /// The width of the token in characters.
        width: usize,

        /// The line above the line where the token was found.
        prev_line_text: Option<String>,

        /// The line of text where the token was found.
        line_text: Option<String>,

        /// The line below the line where the token was found.
        next_line_text: Option<String>,
    },

    /// An unexpected end of file was encountered.
    UnexpectedEof {
        /// The file where the unexpected end of file was encountered.
        file: String,
    },
}

impl AstBuilderError {
    /// Generate a new `AstBuilderError::UnexpectedToken` variant with the given token and message.
    pub fn unexpected_token(token: &Token, message: Option<String>) -> AstBuilderError {
        Self::UnexpectedToken {
            token_type: token.token_type.clone(),
            message,
            file: token.location.file.to_string(),
            line_num: token.location.line_num,
            col_num: token.location.col_num,
            width: token.location.width,
            prev_line_text: token.location.prev_line_text.map(|s| s.to_string()),
            line_text: token.location.line_text.map(|s| s.to_string()),
            next_line_text: token.location.next_line_text.map(|s| s.to_string()),
        }
    }
}

impl std::error::Error for AstBuilderError {}

impl fmt::Display for AstBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedToken {
                token_type,
                message,
                file,
                line_num,
                col_num,
                width,
                prev_line_text,
                line_text,
                next_line_text,
            } => {
                // Construct a `TokenLocation` from the `TokenizerError` so we can use its `Display`
                // implementation.
                let location = TokenLocation {
                    file,
                    line_num: *line_num,
                    col_num: *col_num,
                    width: *width,
                    prev_line_text: prev_line_text.as_deref(),
                    line_text: line_text.as_deref(),
                    next_line_text: next_line_text.as_deref(),
                };

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
