use colored::Colorize;
use std::fmt;

use super::TokenLocation;

/// An error that occured during tokenization.
#[derive(Debug)]
pub struct TokenizerError<'a> {
    /// The location of the error.
    pub location: TokenLocation<'a>,
}

impl<'a> TokenizerError<'a> {
    /// Creates a new TokenizerError.
    pub fn new(location: &TokenLocation<'a>) -> Self {
        Self {
            location: location.clone(),
        }
    }
}

impl<'a> std::error::Error for TokenizerError<'a> {}

impl<'a> fmt::Display for TokenizerError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(line_text) = self.location.line_text {
            // Get the string from of the offending token. As of now this is only a single
            // character, but it may be more in the future.
            let start_idx = self.location.col_num;
            let end_idx = self.location.col_num + self.location.width;
            let token_str = &line_text[start_idx..end_idx];

            write!(
                f,
                "{error_str} Invalid character `{token_str}`\n{location}",
                error_str = "ERROR:".red().bold(),
                token_str = token_str.blue().bold(),
                location = self.location
            )
        } else {
            write!(
                f,
                "{error_str} Invalid character\n{location}",
                error_str = "ERROR:".red().bold(),
                location = self.location
            )
        }
    }
}
