use colored::Colorize;
use std::fmt;

/// Types of errors that can occur during parsing or tokenization.
pub enum ErrType {
    /// Error occurred during tokenization
    Tokenizer,
    /// Error occurred during parsing
    Parser,
    /// Error ocurred during interpretation
    Interpreter,
    /// Error unrelated to input
    Internal,
}

impl ErrType {
    /// Returns the name of the error type
    pub fn name(&self) -> &'static str {
        match self {
            ErrType::Tokenizer => "Tokenizer error",
            ErrType::Parser => "Parser error",
            ErrType::Interpreter => "Interpreter error",
            ErrType::Internal => "Internal error",
        }
    }
}

impl fmt::Display for ErrType {
    /// Returns the name of the error type
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name().red())
    }
}

/// An error that can occur during parsing or tokenization.
pub struct Error {
    /// The type of the error
    pub err_type: ErrType,
    /// The message of the error
    pub message: Option<String>,
    /// The file in which the error occurred
    pub file: String,
    /// The line number that the error occurred on
    pub line: usize,
    /// The column number that the error occurred on
    pub column: usize,
    /// The upstream error
    pub upstream: Option<Box<Error>>,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Check if this error is the source of the error chain
        let is_source = self.upstream.is_none();

        // Check if the upstream error is the source of the error chain
        let is_upstream_source = match &self.upstream {
            Some(e) => e.upstream.is_none(),
            None => false,
        };

        write!(
            f,
            "{upstream}{err}",
            // Print upstream error if it exists
            upstream = match &self.upstream {
                Some(e) => format!("{}", e),
                None => String::new(),
            },
            // If this is the source of the error chain, print the full error
            err = format!(
                "{called_from}{err_type} in {file} at Ln {line}, Col {col}{msg}\n",
                called_from = if is_upstream_source {
                    format!("{}\n", "Called from:".bright_red())
                } else {
                    String::new()
                },
                err_type = self.err_type,
                file = self.file.cyan(),
                line = self.line.to_string().cyan(),
                col = self.column.to_string().cyan(),
                msg = match &self.message {
                    Some(m) =>
                        if is_source {
                            format!(":\n\n\t{}\n", m.replace('\n', "\n\t"))
                        } else {
                            format!(":\n\t{}", m.replace('\n', "\n\t"))
                        },
                    None => String::new(),
                },
            )
        )
    }
}

/// Construct an internal error with a message and reference to the current file, line, and column.
///
/// # Usage
///
/// ```
/// internal_error!();
/// internal_error!("Something went wrong");
/// internal_error!("Something went wrong: {}", "Invalid syntax");
/// internal_error!("Something went wrong: {}", "Invalid syntax" => upstream_error);
/// ```
#[macro_export]
macro_rules! internal_error {
    () => {
        crate::error::Error {
            err_type: crate::error::ErrType::Internal,
            message: None,
            file: file!(),
            line: line!(),
            column: column!(),
            upstream: None,
        }
    };

    ($msg:expr) => {
        crate::error::Error {
            err_type: crate::error::ErrType::Internal,
            message: Some(format!($msg)),
            file: file!(),
            line: line!(),
            column: column!(),
            upstream: None,
        }
    };

    ($msg:expr => $upstream:expr) => {
        crate::error::Error {
            err_type: crate::error::ErrType::Internal,
            message: Some(format!($msg)),
            file: file!(),
            line: line!(),
            column: column!(),
            upstream: Some(Box::new($upstream)),
        }
    };

    ($msg:expr, $($arg:tt)*) => {
        crate::error::Error {
            err_type: crate::error::ErrType::Internal,
            message: Some(format!($msg, $($arg)*)),
            file: file!(),
            line: line!(),
            column: column!(),
            upstream: None,
        }
    };

    ($msg:expr, $($arg:tt)* => $upstream:expr) => {
        crate::error::Error {
            err_type: crate::error::ErrType::Internal,
            message: Some(format!($msg, $($arg)*)),
            file: file!(),
            line: line!(),
            column: column!(),
            upstream: Some(Box::new($upstream)),
        }
    };
}
