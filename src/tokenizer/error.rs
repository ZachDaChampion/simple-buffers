use colored::Colorize;
use std::fmt;

/// An error that occured during tokenization.
#[derive(Debug)]
pub struct TokenizerError {
    /// The name of the file where the error occured.
    file: String,

    /// The line number where the error occured.
    line: usize,

    /// The column number where the error occured.
    column: usize,

    /// An excerpt of the source code where the error occured. This is a string containing the line
    /// where the error occured.
    excerpt: String,
}

impl TokenizerError {
    /// Creates a new TokenizerError.
    pub fn new(file: String, line: usize, column: usize, excerpt: String) -> Self {
        Self {
            file,
            line,
            column,
            excerpt,
        }
    }
}

impl std::error::Error for TokenizerError {}

impl fmt::Display for TokenizerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Get the width of the terminal.
        let term_width = match termsize::get() {
            Some(size) => size.cols,
            None => 0,
        };

        // Get the length of the line number when printed as a string.
        let line_num_len = self.line.to_string().len() + 3;

        // Determine how the maximum length of the excerpt that will be displayed. If there is no
        // terminal width, display the entire excerpt.
        let excerpt_width = if term_width == 0 {
            self.excerpt.len()
        } else {
            term_width as usize - line_num_len
        };

        // Truncate the excerpt if it is too long, padding both sides with ellipses.
        let adjust_left = self.column - 1 > excerpt_width / 2;
        let adjust_right = self.excerpt.len() - self.column - 1 > excerpt_width / 2;
        let (except_slice, overflow_left, overflow_right) = match (adjust_left, adjust_right) {
            (true, true) => {
                let start = self.column - 1 - excerpt_width / 2 + 3;
                let end = self.column - 1 + excerpt_width / 2 - 3;
                (&self.excerpt[start..end], "...", "...")
            }
            (true, false) => {
                let start = self.excerpt.len() - excerpt_width + 3;
                let end = self.excerpt.len();
                (&self.excerpt[start..end], "...", "")
            }
            (false, true) => {
                let start = 0;
                let end = excerpt_width - 3;
                (&self.excerpt[start..end], "", "...")
            }
            (false, false) => (&self.excerpt[..], "", ""),
        };

        // Construct strings for parts of the error message.
        let excerpt = format!("{}{}{}", overflow_left, except_slice, overflow_right);
        let pointer = format!(
            "{}{}",
            " ".repeat(self.column - 1 + line_num_len),
            "^".yellow()
        );
        let problem_char = self.excerpt.chars().nth(self.column - 1);
        let excerpt_prefix = format!("{} | ", self.line).cyan();

        // Construct the error message.
        match problem_char {
            Some(c) => write!(
                f,
                concat!(
                    "{error_header} ",
                    "Unexpected character \"{char}\" in {file} at line {line}, col {column}\n\n",
                    "{excerpt_prefix}{excerpt}\n",
                    "{pointer}"
                ),
                error_header = "ERROR:".bright_red().bold(),
                char = c.to_string().bold(),
                file = self.file.green(),
                line = self.line.to_string().cyan(),
                column = self.column.to_string().yellow(),
                excerpt_prefix = excerpt_prefix,
                excerpt = excerpt,
                pointer = pointer
            ),
            None => write!(
                f,
                concat!(
                    "{error_header} ",
                    "Unexpected end of file in {file} at line {line}, col {column}\n\n",
                    "{excerpt_prefix}{excerpt}\n",
                    "{pointer}"
                ),
                error_header = "ERROR:".bright_red().bold(),
                file = self.file.green(),
                line = self.line.to_string().cyan(),
                column = self.column.to_string().cyan(),
                excerpt_prefix = excerpt_prefix,
                excerpt = excerpt,
                pointer = pointer
            ),
        }
    }
}
