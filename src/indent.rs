//! This file contains a trait for indenting strings.

/// A trait for indenting strings.
pub trait Indent {
    /// Indent a string.
    ///
    /// # Arguments
    ///
    /// * `self` - The string to indent.
    /// * `indent` - The number of spaces to indent by.
    ///
    /// # Returns
    ///
    /// The indented string.
    fn indent(&self, indent: usize) -> String;
}

impl Indent for String {
    fn indent(&self, indent: usize) -> String {
        let mut result = String::new();

        for line in self.lines() {
            result.push_str(&format!("{:indent$}{}", "", line, indent = indent));
            result.push('\n');
        }

        result
    }
}
