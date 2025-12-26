//! Diagnostics handling

use tower_lsp::lsp_types::*;

/// Generate diagnostics for a document
pub fn generate_diagnostics(uri: &Url, source: &str) -> Vec<Diagnostic> {
    let mut diagnostics = Vec::new();

    // TODO: Integrate with compiler for real diagnostics
    // This is a placeholder

    diagnostics
}

/// Convert compiler error to LSP diagnostic
pub fn compiler_error_to_diagnostic(
    message: &str,
    line: u32,
    column: u32,
    severity: DiagnosticSeverity,
) -> Diagnostic {
    Diagnostic {
        range: Range {
            start: Position { line, character: column },
            end: Position { line, character: column + 1 },
        },
        severity: Some(severity),
        code: None,
        code_description: None,
        source: Some("jagannath".to_string()),
        message: message.to_string(),
        related_information: None,
        tags: None,
        data: None,
    }
}

/// Diagnostic codes for Jagannath
pub mod codes {
    pub const SYNTAX_ERROR: i32 = 1000;
    pub const TYPE_ERROR: i32 = 2000;
    pub const KARAKA_ERROR: i32 = 3000;
    pub const LIFETIME_ERROR: i32 = 4000;
    pub const BORROW_ERROR: i32 = 5000;
    pub const AFFIX_ERROR: i32 = 6000;
    pub const SANDHI_WARNING: i32 = 7000;
}
