use colored::Colorize;
use std::fmt;

use super::TokenLocation;

/// An error that occured during tokenization.
#[derive(Debug)]
pub struct TokenizerError {
    /// The location of the error.
    location: TokenLocation,
}

impl TokenizerError {
    /// Creates a new TokenizerError.
    pub fn new(location: TokenLocation) -> Self {
        Self { location }
    }
}

impl std::error::Error for TokenizerError {}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Get the string from of the offending token.
        let token_str = &self.location.line_text
            [self.location.col_num..self.location.col_num + self.location.width];

        // Display the error message.
        write!(
            f,
            "{error_str} Invalid character `{token_str}`\n{location}",
            error_str = "ERROR:".red().bold(),
            token_str = token_str.blue().bold(),
            location = self.location
        )?;

        Ok(())
    }
}
