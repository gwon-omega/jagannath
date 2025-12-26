//! # Purification Functions
//!
//! Sanitizers that cleanse tainted data for Vaitarani crossing.

use super::TaintLevel;

/// A purifier function
pub trait Purifier {
    /// Name of this purifier
    fn name(&self) -> &str;

    /// Sanskrit name (śuddhi-kri variant)
    fn sanskrit_name(&self) -> &str;

    /// Purify the data
    fn purify(&self, data: &str) -> Result<String, PurificationError>;
}

/// Error during purification
#[derive(Debug)]
pub struct PurificationError {
    pub message: String,
}

/// SQL injection purifier
pub struct SqlPurifier;

impl Purifier for SqlPurifier {
    fn name(&self) -> &str { "sql_escape" }
    fn sanskrit_name(&self) -> &str { "śuddhi-kri-sql" }

    fn purify(&self, data: &str) -> Result<String, PurificationError> {
        // Escape SQL special characters
        let escaped = data
            .replace("'", "''")
            .replace("\\", "\\\\")
            .replace("\0", "")
            .replace("\n", "\\n")
            .replace("\r", "\\r")
            .replace("\x1a", "\\Z");
        Ok(escaped)
    }
}

/// HTML/XSS purifier
pub struct HtmlPurifier;

impl Purifier for HtmlPurifier {
    fn name(&self) -> &str { "html_escape" }
    fn sanskrit_name(&self) -> &str { "śuddhi-kri-html" }

    fn purify(&self, data: &str) -> Result<String, PurificationError> {
        // Escape HTML special characters
        let escaped = data
            .replace("&", "&amp;")
            .replace("<", "&lt;")
            .replace(">", "&gt;")
            .replace("\"", "&quot;")
            .replace("'", "&#x27;");
        Ok(escaped)
    }
}

/// Shell command injection purifier
pub struct ShellPurifier;

impl Purifier for ShellPurifier {
    fn name(&self) -> &str { "shell_escape" }
    fn sanskrit_name(&self) -> &str { "śuddhi-kri-shell" }

    fn purify(&self, data: &str) -> Result<String, PurificationError> {
        // Very strict: only allow alphanumeric and some safe chars
        if data.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '.') {
            Ok(data.to_string())
        } else {
            // Quote the entire string
            Ok(format!("'{}'", data.replace("'", "'\\''")))
        }
    }
}
