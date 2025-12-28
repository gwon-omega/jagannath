//! Code Actions - Automated fixes and refactoring
//!
//! Based on the Yoga concept of Kriyā (क्रिया - action) for transformations.

use tower_lsp::lsp_types::*;

// ============================================================================
// Action Kinds (Kriyā Prakāra - क्रिया प्रकार)
// ============================================================================

/// Code action categories
pub mod kriya_prakara {
    use tower_lsp::lsp_types::CodeActionKind;

    /// Quick fix (शीघ्र सुधार - śīghra sudhāra)
    pub const SHIGHRA_SUDHARA: CodeActionKind = CodeActionKind::QUICKFIX;

    /// Refactor (पुनर्निर्माण - punarnirmāṇa)
    pub const PUNARNIRMANA: CodeActionKind = CodeActionKind::REFACTOR;

    /// Extract (निष्कर्षण - niṣkarṣaṇa)
    pub const NISHKARSHANA: CodeActionKind = CodeActionKind::REFACTOR_EXTRACT;

    /// Inline (समाविष्ट - samāviṣṭa)
    pub const SAMAVISHTA: CodeActionKind = CodeActionKind::REFACTOR_INLINE;

    /// Rewrite (पुनर्लेखन - punarlekhana)
    pub const PUNARLEKHANA: CodeActionKind = CodeActionKind::REFACTOR_REWRITE;

    /// Source (स्रोत - srota)
    pub const SROTA: CodeActionKind = CodeActionKind::SOURCE;

    /// Organize imports (आयात व्यवस्था - āyāta vyavasthā)
    pub const AYATA_VYAVASTHA: CodeActionKind = CodeActionKind::SOURCE_ORGANIZE_IMPORTS;
}

// ============================================================================
// Code Action Builder
// ============================================================================

/// Builds code actions
pub struct KriyaBuilder {
    actions: Vec<CodeActionOrCommand>,
}

impl KriyaBuilder {
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    /// Add a quick fix action
    pub fn quick_fix(
        &mut self,
        title: impl Into<String>,
        uri: Url,
        edits: Vec<TextEdit>,
        diagnostics: Option<Vec<Diagnostic>>,
    ) {
        let edit = WorkspaceEdit {
            changes: Some([(uri, edits)].into_iter().collect()),
            ..Default::default()
        };

        self.actions.push(CodeActionOrCommand::CodeAction(CodeAction {
            title: title.into(),
            kind: Some(CodeActionKind::QUICKFIX),
            diagnostics,
            edit: Some(edit),
            is_preferred: Some(true),
            ..Default::default()
        }));
    }

    /// Add a refactoring action
    pub fn refactor(
        &mut self,
        title: impl Into<String>,
        kind: CodeActionKind,
        command: Command,
    ) {
        self.actions.push(CodeActionOrCommand::CodeAction(CodeAction {
            title: title.into(),
            kind: Some(kind),
            command: Some(command),
            ..Default::default()
        }));
    }

    /// Build the actions list
    pub fn build(self) -> Vec<CodeActionOrCommand> {
        self.actions
    }
}

impl Default for KriyaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Quick Fixes (Śīghra Sudhāra - शीघ्र सुधार)
// ============================================================================

/// Generate quick fixes for common errors
pub fn generate_quick_fixes(
    uri: &Url,
    diagnostic: &Diagnostic,
    source_text: &str,
) -> Vec<CodeActionOrCommand> {
    let mut builder = KriyaBuilder::new();

    // Match on diagnostic code if available
    if let Some(code) = &diagnostic.code {
        match code {
            NumberOrString::String(code_str) => {
                generate_fixes_for_code(&mut builder, uri.clone(), diagnostic, code_str, source_text);
            }
            NumberOrString::Number(code_num) => {
                let code_str = format!("E{:04}", code_num);
                generate_fixes_for_code(&mut builder, uri.clone(), diagnostic, &code_str, source_text);
            }
        }
    }

    // Generate fixes based on diagnostic message
    generate_fixes_from_message(&mut builder, uri.clone(), diagnostic, source_text);

    builder.build()
}

fn generate_fixes_for_code(
    builder: &mut KriyaBuilder,
    uri: Url,
    diagnostic: &Diagnostic,
    code: &str,
    _source_text: &str,
) {
    match code {
        // Type mismatch - suggest type conversion
        "E0200" | "TYPE_MISMATCH" => {
            // TODO: Generate type conversion fix
        }
        // Undefined variable - suggest declaration
        "E0100" | "UNDEFINED" => {
            builder.quick_fix(
                "Declare variable with 'mana'",
                uri.clone(),
                vec![TextEdit {
                    range: Range {
                        start: Position {
                            line: diagnostic.range.start.line,
                            character: 0,
                        },
                        end: Position {
                            line: diagnostic.range.start.line,
                            character: 0,
                        },
                    },
                    new_text: "mana ".to_string(),
                }],
                Some(vec![diagnostic.clone()]),
            );
        }
        // Missing semicolon
        "E0300" | "MISSING_SEMICOLON" => {
            builder.quick_fix(
                "Add semicolon",
                uri.clone(),
                vec![TextEdit {
                    range: Range {
                        start: diagnostic.range.end,
                        end: diagnostic.range.end,
                    },
                    new_text: ";".to_string(),
                }],
                Some(vec![diagnostic.clone()]),
            );
        }
        // Use after move - suggest clone
        "E0400" | "USE_AFTER_MOVE" => {
            builder.quick_fix(
                "Clone the value instead of moving",
                uri.clone(),
                vec![TextEdit {
                    range: diagnostic.range,
                    new_text: ".pratilipi()".to_string(), // clone in Sanskrit
                }],
                Some(vec![diagnostic.clone()]),
            );
        }
        _ => {}
    }
}

fn generate_fixes_from_message(
    builder: &mut KriyaBuilder,
    uri: Url,
    diagnostic: &Diagnostic,
    _source_text: &str,
) {
    let message = &diagnostic.message;

    // Missing import
    if message.contains("not found") || message.contains("undefined") {
        if let Some(symbol) = extract_symbol_from_message(message) {
            builder.quick_fix(
                format!("Import '{}'", symbol),
                uri.clone(),
                vec![TextEdit {
                    range: Range {
                        start: Position { line: 0, character: 0 },
                        end: Position { line: 0, character: 0 },
                    },
                    new_text: format!("āyāta {};\n", symbol),
                }],
                Some(vec![diagnostic.clone()]),
            );
        }
    }

    // Unused variable - suggest removal or underscore prefix
    if message.contains("unused") {
        if let Some(var_name) = extract_symbol_from_message(message) {
            // Suggest prefixing with underscore
            builder.quick_fix(
                format!("Prefix '{}' with underscore", var_name),
                uri.clone(),
                vec![TextEdit {
                    range: diagnostic.range,
                    new_text: format!("_{}", var_name),
                }],
                Some(vec![diagnostic.clone()]),
            );
        }
    }

    // Mutable not needed
    if message.contains("does not need to be mutable") {
        builder.quick_fix(
            "Remove 'mana mut'",
            uri,
            vec![TextEdit {
                range: Range {
                    start: Position {
                        line: diagnostic.range.start.line,
                        character: diagnostic.range.start.character.saturating_sub(4),
                    },
                    end: diagnostic.range.start,
                },
                new_text: String::new(),
            }],
            Some(vec![diagnostic.clone()]),
        );
    }
}

fn extract_symbol_from_message(message: &str) -> Option<String> {
    // Look for `symbol` or 'symbol' in message
    let patterns = [('`', '`'), ('\'', '\''), ('"', '"')];

    for (open, close) in patterns {
        if let Some(start) = message.find(open) {
            if let Some(end) = message[start + 1..].find(close) {
                return Some(message[start + 1..start + 1 + end].to_string());
            }
        }
    }

    None
}

// ============================================================================
// Refactoring Actions (Punarnirmāṇa - पुनर्निर्माण)
// ============================================================================

/// Extract function refactoring
pub fn extract_function_action(
    selection: Range,
    function_name: &str,
) -> CodeActionOrCommand {
    CodeActionOrCommand::CodeAction(CodeAction {
        title: format!("Extract to function '{}'", function_name),
        kind: Some(CodeActionKind::REFACTOR_EXTRACT),
        command: Some(Command {
            title: "Extract Function".to_string(),
            command: "jagannath.extractFunction".to_string(),
            arguments: Some(vec![
                serde_json::to_value(&selection).unwrap(),
                serde_json::Value::String(function_name.to_string()),
            ]),
        }),
        ..Default::default()
    })
}

/// Extract variable refactoring
pub fn extract_variable_action(
    selection: Range,
    var_name: &str,
) -> CodeActionOrCommand {
    CodeActionOrCommand::CodeAction(CodeAction {
        title: format!("Extract to variable '{}'", var_name),
        kind: Some(CodeActionKind::REFACTOR_EXTRACT),
        command: Some(Command {
            title: "Extract Variable".to_string(),
            command: "jagannath.extractVariable".to_string(),
            arguments: Some(vec![
                serde_json::to_value(&selection).unwrap(),
                serde_json::Value::String(var_name.to_string()),
            ]),
        }),
        ..Default::default()
    })
}

/// Inline variable refactoring
pub fn inline_variable_action(position: Position) -> CodeActionOrCommand {
    CodeActionOrCommand::CodeAction(CodeAction {
        title: "Inline variable".to_string(),
        kind: Some(CodeActionKind::REFACTOR_INLINE),
        command: Some(Command {
            title: "Inline Variable".to_string(),
            command: "jagannath.inlineVariable".to_string(),
            arguments: Some(vec![
                serde_json::to_value(&position).unwrap(),
            ]),
        }),
        ..Default::default()
    })
}

/// Convert to karaka annotation
pub fn add_karaka_action(
    uri: Url,
    param_range: Range,
    param_name: &str,
    karaka: &str,
) -> CodeActionOrCommand {
    let edit = WorkspaceEdit {
        changes: Some([(
            uri,
            vec![TextEdit {
                range: Range {
                    start: param_range.start,
                    end: param_range.start,
                },
                new_text: format!("-{}:", karaka),
            }],
        )].into_iter().collect()),
        ..Default::default()
    };

    CodeActionOrCommand::CodeAction(CodeAction {
        title: format!("Add kāraka '-{}:' to '{}'", karaka, param_name),
        kind: Some(CodeActionKind::REFACTOR),
        edit: Some(edit),
        ..Default::default()
    })
}

/// Organize imports
pub fn organize_imports_action(uri: Url) -> CodeActionOrCommand {
    CodeActionOrCommand::CodeAction(CodeAction {
        title: "Organize imports (āyāta)".to_string(),
        kind: Some(CodeActionKind::SOURCE_ORGANIZE_IMPORTS),
        command: Some(Command {
            title: "Organize Imports".to_string(),
            command: "jagannath.organizeImports".to_string(),
            arguments: Some(vec![serde_json::to_value(&uri).unwrap()]),
        }),
        ..Default::default()
    })
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_symbol() {
        assert_eq!(
            extract_symbol_from_message("variable `foo` not found"),
            Some("foo".to_string())
        );
        assert_eq!(
            extract_symbol_from_message("unused variable 'bar'"),
            Some("bar".to_string())
        );
    }

    #[test]
    fn test_quick_fix_builder() {
        let mut builder = KriyaBuilder::new();
        builder.quick_fix(
            "Test fix",
            Url::parse("file:///test.jag").unwrap(),
            vec![],
            None,
        );

        let actions = builder.build();
        assert_eq!(actions.len(), 1);
    }

    #[test]
    fn test_extract_function_action() {
        let action = extract_function_action(
            Range::default(),
            "new_function",
        );

        match action {
            CodeActionOrCommand::CodeAction(ca) => {
                assert!(ca.title.contains("new_function"));
            }
            _ => panic!("Expected code action"),
        }
    }
}
