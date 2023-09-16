use colored::Colorize;
use std::fmt;

use super::TokenLocation;

/// An error that occured during tokenization.
#[derive(Debug)]
pub struct TokenizerError {
    /// The name of the file where the token was found.
    pub file: String,

    /// The line number where the token was found (0-indexed).
    pub line_num: usize,

    /// The column number where the token was found (0-indexed).
    pub col_num: usize,

    /// The width of the token in characters.
    pub width: usize,

    /// The line above the line where the token was found.
    pub prev_line_text: Option<String>,

    /// The line of text where the token was found.
    pub line_text: Option<String>,

    /// The line below the line where the token was found.
    pub next_line_text: Option<String>,
}

impl TokenizerError {
    /// Creates a new TokenizerError.
    pub fn new(location: &TokenLocation) -> Self {
        Self {
            file: location.file.to_string(),
            line_num: location.line_num,
            col_num: location.col_num,
            width: location.width,
            prev_line_text: location.prev_line_text.map(|s| s.to_string()),
            line_text: location.line_text.map(|s| s.to_string()),
            next_line_text: location.next_line_text.map(|s| s.to_string()),
        }
    }
}

impl std::error::Error for TokenizerError {}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Construct a `TokenLocation` from the `TokenizerError` so we can use its `Display`
        // implementation.
        let location = TokenLocation {
            file: self.file.as_str(),
            line_num: self.line_num,
            col_num: self.col_num,
            width: self.width,
            prev_line_text: self.prev_line_text.as_deref(),
            line_text: self.line_text.as_deref(),
            next_line_text: self.next_line_text.as_deref(),
        };

        if let Some(line_text) = location.line_text {
            // Get the string from of the offending token. As of now this is only a single
            // character, but it may be more in the future.
            let token_str = &line_text[location.col_num..location.col_num + location.width];

            write!(
                f,
                "{error_str} Invalid character `{token_str}`\n{location}",
                error_str = "ERROR:".red().bold(),
                token_str = token_str.blue().bold(),
                location = location
            )
        } else {
            write!(
                f,
                "{error_str} Invalid character\n{location}",
                error_str = "ERROR:".red().bold(),
                location = location
            )
        }
    }
}
