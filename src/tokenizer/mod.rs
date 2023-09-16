pub mod error;

use colored::Colorize;
use lazy_static::lazy_static;
use regex::Regex;
use regex_macro::regex;
use std::fmt;

use error::TokenizerError;

type OptionalTokenGenerator = Option<fn(String) -> TokenType>;
lazy_static! {
    static ref SPEC: Vec<(&'static Regex, OptionalTokenGenerator)> = vec![
        (regex!(r"^\s+"), None), // Ignore whitespace
        (regex!(r"^//.*?(\r|\n|\r\n)"), None), // Ignore comments
        (regex!(r"^sequence"), Some(|_| TokenType::Sequence)), // Capture sequence keyword
        (regex!(r"^oneof"), Some(|_| TokenType::Oneof)), // Capture oneof keyword
        (regex!(r"^enum"), Some(|_| TokenType::Enum)), // Capture enum keyword
        (regex!(r"^\{"), Some(|_| TokenType::OpenBrace)), // Capture opening brace
        (regex!(r"^\}"), Some(|_| TokenType::CloseBrace)), // Capture closing brace
        (regex!(r"^\["), Some(|_| TokenType::OpenBracket)), // Capture opening bracket
        (regex!(r"^\]"), Some(|_| TokenType::CloseBracket)), // Capture closing bracket
        (regex!(r"^:"), Some(|_| TokenType::Colon)), // Capture colon
        (regex!(r"^;"), Some(|_| TokenType::Semicolon)), // Capture semicolon
        (regex!(r"^="), Some(|_| TokenType::Equals)), // Capture equals sign
        (regex!(r"^[0-9_]+(?:\.[0-9_]+)?"), Some(TokenType::Number)), // Capture numbers
        (regex!(r"^[a-zA-Z_][a-zA-Z0-9_]*"), Some(TokenType::Identifier)), // Capture identifiers
    ];
}

/// A specific token type and associated data.
#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    Sequence,
    Oneof,
    Enum,
    OpenBrace,
    CloseBrace,
    OpenBracket,
    CloseBracket,
    Colon,
    Semicolon,
    Equals,
    Number(String),
    Identifier(String),
}

impl TokenType {
    /// Returns the width of the token in characters.
    pub fn width(&self) -> usize {
        match self {
            TokenType::Sequence => "sequence".len(),
            TokenType::Oneof => "oneof".len(),
            TokenType::Enum => "enum".len(),
            TokenType::OpenBrace => "{".len(),
            TokenType::CloseBrace => "}".len(),
            TokenType::OpenBracket => "[".len(),
            TokenType::CloseBracket => "]".len(),
            TokenType::Colon => ":".len(),
            TokenType::Semicolon => ";".len(),
            TokenType::Equals => "=".len(),
            TokenType::Number(val) => val.len(),
            TokenType::Identifier(val) => val.len(),
        }
    }
}

impl fmt::Display for TokenType {
    /// Displays the token type in a human-readable format.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenType::Sequence => write!(f, "sequence"),
            TokenType::Oneof => write!(f, "oneof"),
            TokenType::Enum => write!(f, "enum"),
            TokenType::OpenBrace => write!(f, "{{"),
            TokenType::CloseBrace => write!(f, "}}"),
            TokenType::OpenBracket => write!(f, "["),
            TokenType::CloseBracket => write!(f, "]"),
            TokenType::Colon => write!(f, ":"),
            TokenType::Semicolon => write!(f, ";"),
            TokenType::Equals => write!(f, "="),
            TokenType::Number(val) => write!(f, "{}", val),
            TokenType::Identifier(val) => write!(f, "{}", val),
        }
    }
}

/// Information needed to locate a token in the source string.
#[derive(Clone, Debug, PartialEq)]
pub struct TokenLocation<'a> {
    /// The name of the file where the token was found.
    pub file: &'a str,

    /// The line number where the token was found (0-indexed).
    pub line_num: usize,

    /// The column number where the token was found (0-indexed).
    pub col_num: usize,

    /// The width of the token in characters.
    pub width: usize,

    /// The line above the line where the token was found.
    pub prev_line_text: Option<&'a str>,

    /// The line of text where the token was found.
    pub line_text: Option<&'a str>,

    /// The line below the line where the token was found.
    pub next_line_text: Option<&'a str>,
}

impl<'a> fmt::Display for TokenLocation<'a> {
    /// Displays the token location in a human-readable format with context.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Print the file name, line number, and column number.
        let path = format!("{}:{}:{}", self.file, self.line_num + 1, self.col_num + 1);
        write!(
            f,
            "  {arrow} {path}",
            arrow = "-->".cyan().bold(),
            path = path.green().underline(),
        )?;

        writeln!(f)?;

        // Figure out how wide the line number will be when it is displayed in the error message.
        // This will be the widest of the three displayed lines. For instance:
        //
        //     9  | let x = 1;
        //     10 | let y = 2;
        //     11 | let z = 3;
        //     ^^^^^
        //    width=5
        let line_num_dis_width = if self.next_line_text.is_some() {
            (self.line_num + 2).to_string().len()
        } else {
            (self.line_num + 1).to_string().len()
        };

        // Construct an arrow pointing to the problematic token.
        let arrow_str = format!("{}{}", " ".repeat(self.col_num), "^".repeat(self.width));

        // Display lines of code around the problematic token.
        for (i, line_str) in [
            self.prev_line_text.as_ref(),
            self.line_text.as_ref(),
            self.next_line_text.as_ref(),
        ]
        .iter()
        .enumerate()
        {
            if let Some(line_str) = line_str {
                if i == 0 {
                    write!(
                        f,
                        "\n{padding} {cyan_bar}",
                        padding = " ".repeat(line_num_dis_width),
                        cyan_bar = "|".cyan().bold(),
                    )?;
                }
                write!(
                    f,
                    "\n{line_num: >width$} {cyan_bar} {line_str}",
                    line_num = (self.line_num + i).to_string().cyan().bold(),
                    width = line_num_dis_width,
                    cyan_bar = "|".cyan().bold(),
                    line_str = line_str,
                )?;
                if i == 1 {
                    write!(
                        f,
                        "\n{padding} {cyan_bar} {arrow_str}",
                        padding = " ".repeat(line_num_dis_width),
                        cyan_bar = "|".cyan().bold(),
                        arrow_str = arrow_str.yellow(),
                    )?;
                }
                if i == 2 {
                    write!(
                        f,
                        "\n{padding} {cyan_bar}",
                        padding = " ".repeat(line_num_dis_width),
                        cyan_bar = "|".cyan().bold(),
                    )?;
                }
            }
        }

        Ok(())
    }
}

/// A token in the input stream.
#[derive(Clone, Debug, PartialEq)]
pub struct Token<'a> {
    /// The type of the token.
    pub token_type: TokenType,

    /// The location of the token in the source string.
    pub location: TokenLocation<'a>,
}

/// A tokenizer that lazily tokenizes a string.
pub struct Tokenizer<'a> {
    /// The source string to tokenize.
    source: &'a str,

    /// The name of the file being tokenized.
    file: &'a str,

    /// The next token in the source string.
    next_token: Option<Token<'a>>,

    /// The current position in the source string.
    cursor: usize,

    /// The current line number (0-indexed).
    line_num: usize,

    /// The current column number (0-indexed).
    col_num: usize,

    /// An iterator over the lines of the source string. This is used to display context when an
    /// error occurs.
    lines_iter: std::str::Lines<'a>,

    /// The previous line of text. This is used to display context when an error occurs.
    prev_line_text: Option<&'a str>,

    /// The current line of text. This is used to display context when an error occurs.
    line_text: Option<&'a str>,

    /// The next line of text. This is used to display context when an error occurs.
    next_line_text: Option<&'a str>,
}

impl<'a> Tokenizer<'a> {
    /// Returns a Tokenizer at the beginning of the source string.
    ///
    /// # Arguments
    ///
    /// * `source` - The source string to tokenize.
    /// * `file` - The name of the file being tokenized.
    pub fn new(source: &'a str, file: &'a str) -> Result<Self, TokenizerError> {
        let mut lines_iter = source.lines();
        let first_line = lines_iter.next();
        let second_line = lines_iter.next();

        let mut tokenizer = Self {
            source,
            file,
            next_token: None,
            cursor: 0,
            line_num: 0,
            col_num: 0,
            lines_iter,
            prev_line_text: None,
            line_text: first_line,
            next_line_text: second_line,
        };
        tokenizer.advance(true)?;

        Ok(tokenizer)
    }

    /// Returns the next token in the source string and advances the tokenizer.
    ///
    /// # Returns
    ///
    /// * `Some(token)` if there is a next token, `None` otherwise.
    pub fn pop(&mut self) -> Result<Option<Token<'a>>, TokenizerError> {
        let token = self.next_token.clone();
        self.advance(false)?;
        Ok(token)
    }

    /// Advances the tokenizer to the next token. This will replace the `next_token` field.
    ///
    /// # Arguments
    ///
    /// * `first` - Whether or not this is the first call to `advance`. If this is true, the current
    ///             token will be ignored.
    fn advance(&mut self, first: bool) -> Result<(), TokenizerError> {
        // Check if the tokenizer has reached the end of the source string
        if self.next_token.is_none() && !first {
            return Ok(());
        }
        if self.cursor >= self.source.len() {
            self.next_token = None;
            return Ok(());
        }

        // Iterate through patterns and try to match them
        let sliced = &self.source[self.cursor..];
        for (re, token_fn) in SPEC.iter() {
            match re.find(sliced) {
                // Pattern doesn't match, continue to next pattern
                None => continue,

                // Pattern matches, create token
                Some(val) => {
                    self.cursor += val.end();

                    // If there is a token function, call it and set the next token
                    if let Some(token_fn) = token_fn {
                        let token_type = token_fn(val.as_str().to_string());
                        self.next_token = Some(Token {
                            token_type: token_type.clone(),
                            location: TokenLocation {
                                file: self.file,
                                line_num: self.line_num,
                                col_num: self.col_num,
                                width: token_type.width(),
                                prev_line_text: self.prev_line_text,
                                line_text: self.line_text,
                                next_line_text: self.next_line_text,
                            },
                        });
                    }

                    // Count newlines and columns
                    for c in val.as_str().chars() {
                        if c == '\n' {
                            self.line_num += 1;
                            self.col_num = 0;
                            self.prev_line_text = self.line_text;
                            self.line_text = self.next_line_text;
                            self.next_line_text = self.lines_iter.next();
                        } else {
                            self.col_num += 1;
                        }
                    }

                    // If there is no token function, advance to the next token
                    if token_fn.is_none() {
                        self.advance(false)?;
                    }

                    return Ok(());
                }
            }
        }

        // If no patterns match, return an error
        self.next_token = None;
        Err(TokenizerError::new(&TokenLocation {
            file: self.file,
            line_num: self.line_num,
            col_num: self.col_num,
            width: 1,
            prev_line_text: self.prev_line_text,
            line_text: self.line_text,
            next_line_text: self.next_line_text,
        }))
    }
}

pub type TokenIterator<'a> = dyn Iterator<Item = Result<Token<'a>, TokenizerError>> + 'a;
impl<'a> Iterator for Tokenizer<'a> {
    type Item = Result<Token<'a>, TokenizerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop().transpose()
    }
}
