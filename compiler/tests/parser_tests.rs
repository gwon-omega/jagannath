//! Integration tests for the Jagannath compiler parser
//!
//! Tests AST construction including:
//! - Function declarations (kāryakrama)
//! - Type declarations (prakāra)
//! - Control flow (yad, cala)
//! - Expression parsing

use jagannath_compiler::parser::{ast::*, Parser};

/// Test function declaration parsing
#[test]
fn test_function_declaration() {
    let source = r#"
kāryakrama yoga(x: saṅkhyā, y: saṅkhyā) -> saṅkhyā {
    phera x + y
}
"#;
    let ast = Parser::parse_str(source).expect("Failed to parse");

    assert_eq!(ast.items.len(), 1);
    match &ast.items[0] {
        Item::Function(func) => {
            assert_eq!(func.name.name, "yoga");
            assert_eq!(func.params.len(), 2);
            assert!(func.return_type.is_some());
        }
        _ => panic!("Expected function declaration"),
    }
}

/// Test type declaration parsing
#[test]
fn test_type_declaration() {
    let source = r#"
prakāra Bindu {
    x: bhinna,
    y: bhinna,
}
"#;
    let ast = Parser::parse_str(source).expect("Failed to parse");

    match &ast.items[0] {
        Item::TypeDef(t) => {
            assert_eq!(t.name.name, "Bindu");
        }
        _ => panic!("Expected type declaration"),
    }
}

/// Test if-else parsing
#[test]
fn test_if_else() {
    let source = r#"
kāryakrama parikṣā(x: saṅkhyā) -> saṅkhyā {
    yad x > 0 {
        phera x
    } anyathā {
        phera 0
    }
}
"#;
    let ast = Parser::parse_str(source).expect("Failed to parse");

    match &ast.items[0] {
        Item::Function(func) => {
            // Body should contain an if statement
            assert!(func.body.stmts.iter().any(|s| matches!(s, Stmt::If { .. })));
        }
        _ => panic!("Expected function"),
    }
}

/// Test loop parsing
#[test]
fn test_loops() {
    let source = r#"
kāryakrama gaṇanā() {
    cala x : data {
        mudraṇa!(x)
    }
}
"#;
    let ast = Parser::parse_str(source).expect("Failed to parse");

    match &ast.items[0] {
        Item::Function(func) => {
            // Should have loop statement
            assert!(func
                .body
                .stmts
                .iter()
                .any(|s| matches!(s, Stmt::Loop { .. })));
        }
        _ => panic!("Expected function"),
    }
}

/// Test binary expressions
#[test]
fn test_binary_expressions() {
    let source = r#"
kāryakrama gaṇita() -> saṅkhyā {
    māna a = 1 + 2 * 3 - 4 / 2
    phera a
}
"#;
    let ast = Parser::parse_str(source).expect("Failed to parse");

    match &ast.items[0] {
        Item::Function(func) => {
            // Should parse function body successfully
            assert!(
                !func.body.stmts.is_empty(),
                "Function body should not be empty"
            );
        }
        _ => panic!("Expected function"),
    }
}

/// Test field access and method calls
#[test]
fn test_field_and_method() {
    let source = r#"
kāryakrama prayoga(p: Bindu) {
    māna x = p.x
    māna len = p.dūram()
}
"#;
    let ast = Parser::parse_str(source).expect("Failed to parse");

    match &ast.items[0] {
        Item::Function(func) => {
            // Should have let bindings (may include additional parsed elements)
            assert!(func.body.stmts.len() >= 2);
        }
        _ => panic!("Expected function"),
    }
}

/// Test array expressions
#[test]
fn test_arrays() {
    let source = r#"
kāryakrama sūcī_prayoga() {
    māna arr = [1, 2, 3, 4, 5]
    māna first = arr[0]
}
"#;
    let ast = Parser::parse_str(source).expect("Failed to parse");

    match &ast.items[0] {
        Item::Function(func) => {
            assert!(func.body.stmts.len() >= 2);
        }
        _ => panic!("Expected function"),
    }
}

/// Test return statement
#[test]
fn test_return() {
    let source = r#"
kāryakrama test() -> saṅkhyā {
    phera 42
}
"#;
    let ast = Parser::parse_str(source).expect("Failed to parse");

    match &ast.items[0] {
        Item::Function(func) => {
            assert!(func
                .body
                .stmts
                .iter()
                .any(|s| matches!(s, Stmt::Return { .. })));
        }
        _ => panic!("Expected function"),
    }
}

/// Test multiple functions
#[test]
fn test_multiple_functions() {
    let source = r#"
kāryakrama eka() -> saṅkhyā {
    phera 1
}

kāryakrama dvi() -> saṅkhyā {
    phera 2
}

kāryakrama tri() -> saṅkhyā {
    phera 3
}
"#;
    let ast = Parser::parse_str(source).expect("Failed to parse");
    assert_eq!(ast.items.len(), 3);
}

/// Test import parsing
#[test]
fn test_import() {
    let source = r#"
āyāti std::io
āyāti collections::Vec
"#;
    let ast = Parser::parse_str(source).expect("Failed to parse");

    assert_eq!(ast.items.len(), 2);
    for item in &ast.items {
        assert!(matches!(item, Item::Import(_)));
    }
}

/// Test nested expressions with parentheses
#[test]
fn test_nested_expressions() {
    let source = r#"
kāryakrama nested() -> saṅkhyā {
    māna a = (1 + 2) * (3 + 4)
    phera a
}
"#;
    let ast = Parser::parse_str(source).expect("Failed to parse");

    match &ast.items[0] {
        Item::Function(func) => {
            assert!(func.body.stmts.len() >= 2);
        }
        _ => panic!("Expected function"),
    }
}

/// Test function call parsing
#[test]
fn test_function_call() {
    let source = r#"
kāryakrama caller() {
    māna x = other_function(1, 2, 3)
}
"#;
    let ast = Parser::parse_str(source).expect("Failed to parse");

    match &ast.items[0] {
        Item::Function(func) => {
            assert!(func.body.stmts.len() >= 1);
        }
        _ => panic!("Expected function"),
    }
}

/// Test unary expressions
#[test]
fn test_unary_expressions() {
    let source = r#"
kāryakrama unary() {
    māna a = -5
    māna b = !satya
}
"#;
    let ast = Parser::parse_str(source).expect("Failed to parse");

    match &ast.items[0] {
        Item::Function(func) => {
            assert!(func.body.stmts.len() >= 2);
        }
        _ => panic!("Expected function"),
    }
}

/// Test comparison operators
#[test]
fn test_comparison_operators() {
    let source = r#"
kāryakrama compare(a: saṅkhyā, b: saṅkhyā) -> saṅkhyā {
    māna lt = a < b
    phera lt
}
"#;
    let ast = Parser::parse_str(source).expect("Failed to parse");

    match &ast.items[0] {
        Item::Function(func) => {
            // let binding + return
            assert!(func.body.stmts.len() >= 2);
        }
        _ => panic!("Expected function"),
    }
}
