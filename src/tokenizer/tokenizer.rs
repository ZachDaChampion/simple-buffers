use lazy_static::lazy_static;
use regex::Regex;
use regex_macro::regex;

use super::error::TokenizerError;

type OptionalTokenGenerator = Option<fn(String) -> TokenType>;
lazy_static! {
    static ref SPEC: Vec<(&'static Regex, OptionalTokenGenerator)> = vec![
        (regex!(r"^\s+"), None), // Ignore whitespace
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

        // Capture comments and strip the leading slashes and whitespace
        (regex!(r"^//.*?(\r|\n|\r\n)"), Some(|cmt|
            TokenType::Comment(cmt.as_str()[2..].trim().to_string())
        )),
    ];
}

/// A specific token type and associated data.
#[derive(Clone, Debug, PartialEq)]
pub enum TokenType {
    /// A 'sequence' keyword.
    Sequence,

    /// A `oneof` keyword.
    Oneof,

    /// A `enum` keyword.
    Enum,

    /// An opening brace (`{`).
    OpenBrace,

    /// A closing brace (`}`).
    CloseBrace,

    /// An opening bracket (`[`).
    OpenBracket,

    /// A closing bracket (`]`).
    CloseBracket,

    /// A colon (`:`).
    Colon,

    /// A semicolon (`;`).
    Semicolon,

    /// An equals sign (`=`).
    Equals,

    /// Number literal.
    Number(String),

    /// An identifier.
    Identifier(String),

    /// A comment.
    Comment(String),
}

/// A token in the input stream.
#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    /// The type of the token.
    token_type: TokenType,

    /// The line number where the token was found.
    line: usize,

    /// The column number where the token was found.
    column: usize,
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
        tokenizer.advance()?;

        Ok(tokenizer)
    }

    /// Returns whether or not the tokenizer has reached the end of the source string.
    ///
    /// # Returns
    ///
    /// * `true` if the tokenizer has reached the end of the source string, `false` otherwise.
    pub fn is_eof(&self) -> bool {
        self.next_token.is_none()
    }

    /// Peeks at the next token in the source string.
    ///
    /// # Returns
    ///
    /// * `Some(token)` if there is a next token, `None` otherwise.
    pub fn peek(&self) -> Option<&Token> {
        self.next_token.as_ref()
    }

    /// Returns the next token in the source string and advances the tokenizer.
    ///
    /// # Returns
    ///
    /// * `Some(token)` if there is a next token, `None` otherwise.
    pub fn pop(&mut self) -> Result<Option<Token>, TokenizerError> {
        let token = self.next_token.clone();
        self.advance()?;
        Ok(token)
    }

    /// Advances the tokenizer to the next token. This will replace the `next_token` field.
    fn advance(&mut self) -> Result<(), TokenizerError> {
        // Check if the tokenizer has reached the end of the source string
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
                        self.advance()?;
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
        self.cursor = self.source.len();
        Err(TokenizerError::new(
            self.file.to_string(),
            self.line,
            self.column,
            self.source[line_start..line_end].to_string(),
        ))
    }
}

impl Iterator for Tokenizer<'_> {
    type Item = Result<Token, TokenizerError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.pop().transpose()
    }
}
