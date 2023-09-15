use colored::Colorize;
use std::fmt;

use crate::tokenizer::Token;

/// An error that is returned by the parser.
#[derive(Debug)]
pub enum ParserError {
    /// An unexpected token was encountered.
    UnexpectedToken {
        /// The file where the unexpected token was encountered.
        file: String,

        /// The token that was encountered.
        token: Token,

        /// An optional error message.
        message: Option<String>,
    },

    /// An unexpected end of file was encountered.
    UnexpectedEof {
        /// The file where the unexpected end of file was encountered.
        file: String,

        /// The line number where the unexpected end of file was encountered.
        line: usize,

        /// The column number where the unexpected end of file was encountered.
        column: usize,
    },
}

impl std::error::Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnexpectedToken {
                file,
                token,
                message,
            } => {
                let mut msg = format!(
                    "Unexpected token {token:?} in {file} at line {line}, column {col}",
                    token = token.token_type,
                    file = file.green(),
                    line = token.line.to_string().cyan(),
                    col = token.column.to_string().yellow(),
                );
                if let Some(message) = message {
                    msg.push_str(&format!(" ({})", message));
                }
                write!(f, "{}", msg)
            }

            Self::UnexpectedEof { file, line, column } => write!(
                f,
                "Unexpected end of file in {file} at line {line}, column {col}",
                file = file.green(),
                line = line.to_string().cyan(),
                col = column.to_string().yellow(),
            ),
        }
    }
}
