use colored::Colorize;
use std::fmt;

use crate::tokenizer::Token;

/// An error that is returned by the AST builder.
#[derive(Debug)]
pub enum AstBuilderError {
    /// An unexpected token was encountered.
    UnexpectedToken {
        /// The token that was encountered.
        token: Token,

        /// An optional error message.
        message: Option<String>,
    },

    /// An unexpected end of file was encountered.
    UnexpectedEof {
        /// The file where the unexpected end of file was encountered.
        file: String,
    },
}

impl std::error::Error for AstBuilderError {}

impl fmt::Display for AstBuilderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedToken { token, message } => {
                if let Some(message) = message {
                    write!(
                        f,
                        "{error_str} Unexpected token `{token}` ({message})\n{location}",
                        error_str = "ERROR:".red().bold(),
                        token = token.token_type.to_string().blue().bold(),
                        message = message,
                        location = token.location,
                    )
                } else {
                    write!(
                        f,
                        "{error_str} Unexpected token `{token}`\n{location}",
                        error_str = "ERROR:".red().bold(),
                        token = token.token_type.to_string().blue().bold(),
                        location = token.location,
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
