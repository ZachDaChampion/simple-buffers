//! Compiles a tagged syntax tree into a [simplebuffers_core::SBSchema].

mod error;
mod parse;
pub use error::CompilerError;
pub use parse::*;
