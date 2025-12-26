//! LSP Handlers

use tower_lsp::lsp_types::*;

/// Handle completion requests
pub fn handle_completion(
    params: &CompletionParams,
    // TODO: Add document state
) -> Option<CompletionResponse> {
    let items = vec![
        // Sanskrit keywords
        CompletionItem {
            label: "kāryakrama".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Function definition".to_string()),
            documentation: Some(Documentation::String(
                "कार्यक्रम - Define a function".to_string()
            )),
            insert_text: Some("kāryakrama ${1:name}(${2:params}) -> ${3:type} {\n\t$0\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "yad".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("If statement".to_string()),
            documentation: Some(Documentation::String(
                "यद् - Conditional statement".to_string()
            )),
            insert_text: Some("yad ${1:condition} {\n\t$0\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "cala".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Loop statement".to_string()),
            documentation: Some(Documentation::String(
                "चल - Loop construct".to_string()
            )),
            insert_text: Some("cala ${1:iterator} {\n\t$0\n}".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
        CompletionItem {
            label: "phera".to_string(),
            kind: Some(CompletionItemKind::KEYWORD),
            detail: Some("Return statement".to_string()),
            documentation: Some(Documentation::String(
                "फेर - Return from function".to_string()
            )),
            insert_text: Some("phera $0".to_string()),
            insert_text_format: Some(InsertTextFormat::SNIPPET),
            ..Default::default()
        },
    ];

    Some(CompletionResponse::Array(items))
}

/// Handle hover requests
pub fn handle_hover(
    params: &HoverParams,
    // TODO: Add document state
) -> Option<Hover> {
    // TODO: Look up symbol at position
    None
}

/// Handle goto definition requests
pub fn handle_goto_definition(
    params: &GotoDefinitionParams,
    // TODO: Add document state
) -> Option<GotoDefinitionResponse> {
    // TODO: Find definition location
    None
}
