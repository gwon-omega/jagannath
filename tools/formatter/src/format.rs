//! Core formatting logic

use super::config::{FormatConfig, BraceStyle};

/// Code formatter
pub struct Formatter {
    config: FormatConfig,
}

impl Formatter {
    /// Create new formatter
    pub fn new(config: FormatConfig) -> Self {
        Self { config }
    }

    /// Format source code
    pub fn format(&self, source: &str) -> String {
        let mut output = String::new();
        let mut indent_level = 0;
        let mut in_string = false;
        let mut in_comment = false;

        for line in source.lines() {
            let trimmed = line.trim();

            // Skip empty lines but preserve them
            if trimmed.is_empty() {
                output.push('\n');
                continue;
            }

            // Handle comments
            if trimmed.starts_with("//") {
                output.push_str(&self.config.indent(indent_level));
                output.push_str(trimmed);
                output.push('\n');
                continue;
            }

            // Decrease indent for closing braces
            if trimmed.starts_with('}') || trimmed.starts_with(')') || trimmed.starts_with(']') {
                indent_level = indent_level.saturating_sub(1);
            }

            // Write indented line
            output.push_str(&self.config.indent(indent_level));
            output.push_str(&self.format_line(trimmed));
            output.push('\n');

            // Increase indent for opening braces
            if trimmed.ends_with('{') || trimmed.ends_with('(') || trimmed.ends_with('[') {
                indent_level += 1;
            }
        }

        // Trim trailing whitespace from each line
        if self.config.trim_trailing_whitespace {
            output = output
                .lines()
                .map(|line| line.trim_end())
                .collect::<Vec<_>>()
                .join("\n");
        }

        // Add trailing newline
        if self.config.trailing_newline && !output.ends_with('\n') {
            output.push('\n');
        }

        output
    }

    /// Format a single line
    fn format_line(&self, line: &str) -> String {
        let mut result = String::new();
        let mut chars = line.chars().peekable();
        let mut prev_char = ' ';

        while let Some(c) = chars.next() {
            match c {
                // Ensure space around operators
                '+' | '-' | '*' | '/' | '=' | '<' | '>' => {
                    if prev_char != ' ' && prev_char != '(' {
                        result.push(' ');
                    }
                    result.push(c);
                    if chars.peek().map_or(false, |&next| {
                        next != ' ' && next != ')' && next != '=' && next != c
                    }) {
                        result.push(' ');
                    }
                }
                // Remove space before comma/semicolon, ensure space after
                ',' | ';' => {
                    // Remove trailing space before
                    while result.ends_with(' ') {
                        result.pop();
                    }
                    result.push(c);
                    if chars.peek().map_or(false, |&next| next != ' ' && next != '\n') {
                        result.push(' ');
                    }
                }
                // Opening brackets - no space after
                '(' | '[' | '{' => {
                    result.push(c);
                }
                // Closing brackets - no space before
                ')' | ']' | '}' => {
                    // Remove trailing space before
                    while result.ends_with(' ') {
                        result.pop();
                    }
                    result.push(c);
                }
                // Colon - space after for type annotations
                ':' => {
                    result.push(c);
                    if chars.peek().map_or(false, |&next| next != ' ' && next != ':') {
                        result.push(' ');
                    }
                }
                // Default
                _ => {
                    result.push(c);
                }
            }
            prev_char = c;
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_formatting() {
        let formatter = Formatter::new(FormatConfig::default());
        let source = "kāryakrama main(){\nphera 0\n}";
        let formatted = formatter.format(source);
        assert!(formatted.contains("    phera")); // Should be indented
    }
}
