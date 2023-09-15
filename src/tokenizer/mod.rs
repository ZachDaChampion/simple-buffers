pub mod error;

use lazy_static::lazy_static;
use regex::Regex;
use regex_macro::regex;

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

/// A token in the input stream.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    /// The type of the token.
    pub token_type: TokenType,

    /// The line number where the token was found.
    pub line: usize,

    /// The column number where the token was found.
    pub column: usize,
}

/// A tokenizer that lazily tokenizes a string.
pub struct Tokenizer<'a> {
    /// The source string to tokenize.
    source: &'a str,

    /// The name of the file being tokenized.
    file: &'a str,

    /// The next token in the source string.
    next_token: Option<Token>,

    /// The current line number.
    line: usize,

    /// The current column number.
    column: usize,

    /// The current position in the source string.
    cursor: usize,
}

impl<'a> Tokenizer<'a> {
    /// Returns a Tokenizer at the beginning of the source string.
    ///
    /// # Arguments
    ///
    /// * `source` - The source string to tokenize.
    /// * `file` - The name of the file being tokenized.
    pub fn new(source: &'a str, file: &'a str) -> Result<Self, TokenizerError> {
        let mut tokenizer = Self {
            source,
            file,
            next_token: None,
            line: 1,
            column: 1,
            cursor: 0,
        };
        tokenizer.advance(true)?;

        Ok(tokenizer)
    }

    /// Returns the next token in the source string and advances the tokenizer.
    ///
    /// # Returns
    ///
    /// * `Some(token)` if there is a next token, `None` otherwise.
    pub fn pop(&mut self) -> Result<Option<Token>, TokenizerError> {
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
                            token_type,
                            line: self.line,
                            column: self.column,
                        });
                    }

                    // Count newlines and columns
                    for c in val.as_str().chars() {
                        if c == '\n' {
                            self.line += 1;
                            self.column = 1;
                        } else {
                            self.column += 1;
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
        let slice_before = &self.source[..self.cursor];
        let slice_after = &self.source[self.cursor..];
        let line_start = slice_before.rfind('\n').map_or(0, |i| i + 1);
        let line_end = slice_after.find('\n').unwrap_or(slice_after.len()) + self.cursor;
        self.next_token = None;
        Err(TokenizerError::new(
            self.file.to_string(),
            self.line,
            self.column,
            self.source[line_start..line_end].to_string(),
        ))
    }
}

pub type TokenIterator<'a> = dyn Iterator<Item = Result<Token, TokenizerError>> + 'a;
impl Iterator for Tokenizer<'_> {
    type Item = Result<Token, TokenizerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop().transpose()
    }
}
