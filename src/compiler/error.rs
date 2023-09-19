use colored::Colorize;
use std::fmt;

use crate::tokenizer::Token;

#[derive(Debug)]
pub struct CompilerError<'a> {
    /// The token that best represents the error location.
    token: Option<Token<'a>>,

    /// An optional error message.
    message: Option<String>,
}

impl<'a> CompilerError<'a> {
    /// Generate a new `CompilerError::InvalidNode` variant with the given token and message.
    pub fn new(token: Option<Token<'a>>, message: String) -> CompilerError<'a> {
        Self {
            token,
            message: Some(message),
        }
    }
}

impl<'a> std::error::Error for CompilerError<'a> {}

impl<'a> fmt::Display for CompilerError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(message) = &self.message {
            write!(
                f,
                "{error_str} {message}",
                error_str = "ERROR:".red().bold(),
                message = message,
            )?;
        } else {
            write!(
                f,
                "{error_str} Compiler error",
                error_str = "ERROR:".red().bold(),
            )?;
        }
        if let Some(token) = &self.token {
            write!(f, "\n{}", token.location,)
        } else {
            Ok(())
        }
    }
}
