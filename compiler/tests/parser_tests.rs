//! Integration tests for the Jagannath compiler parser
//!
//! Tests AST construction including:
//! - Function declarations (kāryakrama)
//! - Type declarations (prakāra)
//! - Control flow (yad, cala)
//! - Expression parsing

use jagannath_compiler::parser::{Parser, ast::*};

/// Test function declaration parsing
#[test]
fn test_function_declaration() {
    let source = r#"
kāryakrama yoga(@kartṛ x: saṅkhyā-a, @karman y: saṅkhyā-a) -> saṅkhyā-a {
    phera x + y
}
"#;
    let ast = Parser::parse(source).expect("Failed to parse");

    assert_eq!(ast.items.len(), 1);
    match &ast.items[0] {
        Item::Function(func) => {
            assert_eq!(func.name.as_str(), "yoga");
            assert_eq!(func.params.len(), 2);
            assert!(func.return_type.is_some());
        }
        _ => panic!("Expected function declaration"),
    }
}

/// Test struct declaration parsing
#[test]
fn test_struct_declaration() {
    let source = r#"
prakāra Bindu-a-l-k {
    x: bhinna-a,
    y: bhinna-a,
}
"#;
    let ast = Parser::parse(source).expect("Failed to parse");

    match &ast.items[0] {
        Item::Struct(s) => {
            assert_eq!(s.name.as_str(), "Bindu");
            assert_eq!(s.fields.len(), 2);
            // Check suffix parsing
            assert!(s.suffixes.contains(&Suffix::Immutable));
            assert!(s.suffixes.contains(&Suffix::Linear));
            assert!(s.suffixes.contains(&Suffix::Stack));
        }
        _ => panic!("Expected struct declaration"),
    }
}

/// Test enum declaration parsing
#[test]
fn test_enum_declaration() {
    let source = r#"
gaṇa Vikalpa<T> {
    Kiñcit(T),
    Śūnya,
}
"#;
    let ast = Parser::parse(source).expect("Failed to parse");

    match &ast.items[0] {
        Item::Enum(e) => {
            assert_eq!(e.name.as_str(), "Vikalpa");
            assert_eq!(e.type_params.len(), 1);
            assert_eq!(e.variants.len(), 2);
        }
        _ => panic!("Expected enum declaration"),
    }
}

/// Test if-else parsing
#[test]
fn test_if_else() {
    let source = r#"
kāryakrama parikṣā(x: saṅkhyā-a) -> saṅkhyā-a {
    yad x > 0 {
        phera x
    } anyathā {
        phera -x
    }
}
"#;
    let ast = Parser::parse(source).expect("Failed to parse");

    match &ast.items[0] {
        Item::Function(func) => {
            // Body should contain an if-else expression
            assert!(func.body.stmts.iter().any(|s| matches!(s, Stmt::Expr(Expr::If { .. }))));
        }
        _ => panic!("Expected function"),
    }
}

/// Test loop parsing
#[test]
fn test_loops() {
    let source = r#"
kāryakrama gaṇanā() {
    // While loop
    cala dharṣa x < 10 {
        x = x + 1
    }

    // For loop
    cala i antargatam 0..10 {
        mudraṇa!(i)
    }

    // Infinite loop
    cala {
        virama
    }
}
"#;
    let ast = Parser::parse(source).expect("Failed to parse");

    match &ast.items[0] {
        Item::Function(func) => {
            // Should have multiple loop statements
            let loop_count = func.body.stmts.iter()
                .filter(|s| matches!(s, Stmt::Expr(Expr::Loop { .. }) | Stmt::Expr(Expr::While { .. }) | Stmt::Expr(Expr::For { .. })))
                .count();
            assert!(loop_count >= 2);
        }
        _ => panic!("Expected function"),
    }
}

/// Test pattern matching
#[test]
fn test_match_expression() {
    let source = r#"
kāryakrama melana(v: Vikalpa<saṅkhyā-a>) -> saṅkhyā-a {
    melana v {
        Kiñcit(x) => x,
        Śūnya => 0,
    }
}
"#;
    let ast = Parser::parse(source).expect("Failed to parse");

    match &ast.items[0] {
        Item::Function(func) => {
            assert!(func.body.stmts.iter().any(|s| matches!(s, Stmt::Expr(Expr::Match { .. }))));
        }
        _ => panic!("Expected function"),
    }
}

/// Test binary expressions
#[test]
fn test_binary_expressions() {
    let source = r#"
kāryakrama gaṇita() -> saṅkhyā-a {
    māna a = 1 + 2 * 3 - 4 / 2
    māna b = (1 + 2) * (3 - 4)
    māna c = a == b && a != 0 || b > 5
    phera a + b + c
}
"#;
    let ast = Parser::parse(source).expect("Failed to parse");

    match &ast.items[0] {
        Item::Function(func) => {
            // Should have let bindings with complex expressions
            let let_count = func.body.stmts.iter()
                .filter(|s| matches!(s, Stmt::Let { .. }))
                .count();
            assert_eq!(let_count, 3);
        }
        _ => panic!("Expected function"),
    }
}

/// Test field access and method calls
#[test]
fn test_field_and_method() {
    let source = r#"
kāryakrama prayoga(p: Bindu-b) {
    māna x = p.x
    māna len = p.dūram()
    māna moved = p.calana(1.0, 2.0)
}
"#;
    let ast = Parser::parse(source).expect("Failed to parse");

    match &ast.items[0] {
        Item::Function(func) => {
            // Should parse field access and method calls
            assert!(func.body.stmts.iter().any(|s| matches!(s, Stmt::Let { value: Expr::Field { .. }, .. })));
            assert!(func.body.stmts.iter().any(|s| matches!(s, Stmt::Let { value: Expr::MethodCall { .. }, .. })));
        }
        _ => panic!("Expected function"),
    }
}

/// Test array and index expressions
#[test]
fn test_arrays_and_indexing() {
    let source = r#"
kāryakrama sūcī_prayoga() {
    māna arr = [1, 2, 3, 4, 5]
    māna first = arr[0]
    māna slice = arr[1..4]
}
"#;
    let ast = Parser::parse(source).expect("Failed to parse");

    match &ast.items[0] {
        Item::Function(func) => {
            assert!(func.body.stmts.iter().any(|s| matches!(s, Stmt::Let { value: Expr::Array { .. }, .. })));
            assert!(func.body.stmts.iter().any(|s| matches!(s, Stmt::Let { value: Expr::Index { .. }, .. })));
        }
        _ => panic!("Expected function"),
    }
}

/// Test lambda expressions
#[test]
fn test_lambda() {
    let source = r#"
kāryakrama lambda_prayoga() {
    māna dviguṇa = |x| x * 2
    māna phala = dviguṇa(21)
}
"#;
    let ast = Parser::parse(source).expect("Failed to parse");

    match &ast.items[0] {
        Item::Function(func) => {
            assert!(func.body.stmts.iter().any(|s| matches!(s, Stmt::Let { value: Expr::Lambda { .. }, .. })));
        }
        _ => panic!("Expected function"),
    }
}

/// Test trait/impl parsing
#[test]
fn test_trait_impl() {
    let source = r#"
guṇa Gaṇita {
    kāryakrama yoga(sva-b, anya: Sva-b) -> Sva
}

pūrti Gaṇita krte Saṅkhyā {
    kāryakrama yoga(sva-b, anya: Saṅkhyā-b) -> Saṅkhyā {
        phera sva + anya
    }
}
"#;
    let ast = Parser::parse(source).expect("Failed to parse");

    assert!(ast.items.iter().any(|i| matches!(i, Item::Trait(_))));
    assert!(ast.items.iter().any(|i| matches!(i, Item::Impl(_))));
}

/// Test generic type parameters
#[test]
fn test_generics() {
    let source = r#"
prakāra Peṭī<T> {
    mūlya: T,
}

kāryakrama sāmānya<T: Gaṇita>(x: T, y: T) -> T {
    phera x.yoga(y)
}
"#;
    let ast = Parser::parse(source).expect("Failed to parse");

    match &ast.items[0] {
        Item::Struct(s) => {
            assert_eq!(s.type_params.len(), 1);
        }
        _ => panic!("Expected struct"),
    }

    match &ast.items[1] {
        Item::Function(f) => {
            assert_eq!(f.type_params.len(), 1);
            assert!(f.type_params[0].bounds.len() > 0);
        }
        _ => panic!("Expected function"),
    }
}

/// Test module and use declarations
#[test]
fn test_modules() {
    let source = r#"
vibhāga gaṇita {
    kāryakrama yoga(x: saṅkhyā-a, y: saṅkhyā-a) -> saṅkhyā-a {
        phera x + y
    }
}

upayoga gaṇita::yoga
"#;
    let ast = Parser::parse(source).expect("Failed to parse");

    assert!(ast.items.iter().any(|i| matches!(i, Item::Module(_))));
    assert!(ast.items.iter().any(|i| matches!(i, Item::Use(_))));
}

/// Test error recovery in parser
#[test]
fn test_parser_error_recovery() {
    // Missing semicolon, should still parse rest
    let source = r#"
kāryakrama eka() {
    māna x = 1
    māna y = 2  // missing semicolon should be auto-inserted
    phera x + y
}
"#;
    // Should parse with warnings, not hard fail
    let result = Parser::parse(source);
    assert!(result.is_ok() || result.unwrap_err().is_recoverable());
}
