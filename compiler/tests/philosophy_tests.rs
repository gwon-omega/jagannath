//! Integration tests for Hindu Philosophy systems in Jagannath compiler

use jagannath_compiler::driver::options::CompilerOptions;
use jagannath_compiler::driver::CompilerSession;
use jagannath_compiler::philosophy::guna::Guna;

/// Helper to check if source compiles without errors
fn compiles_ok(source: &str) -> bool {
    let mut options = CompilerOptions::new();
    options.emit_asm = true; // Only generate assembly, don't try to link
    let mut session = CompilerSession::new(options);
    session.compile(source).is_ok()
}

/// Test Guṇa mode compilation - Sattva (maximum safety)
#[test]
fn test_guna_sattva_mode() {
    let mut options = CompilerOptions::new();
    options.guna = Guna::Sattva;
    options.debug_info = true;
    options.emit_asm = true; // Only generate assembly
    let mut session = CompilerSession::new(options);
    let source = r#"
kāryakrama test() -> saṅkhyā-a-k-t32 {
    phera 42
}
"#;
    let result = session.compile(source);
    assert!(result.is_ok(), "Sattva mode should compile successfully");
}

/// Test Guṇa mode compilation - Rajas (maximum performance)
#[test]
fn test_guna_rajas_mode() {
    let mut options = CompilerOptions::new();
    options.guna = Guna::Rajas;
    options.opt_level = 3;
    options.emit_asm = true; // Only generate assembly
    let mut session = CompilerSession::new(options);
    let source = r#"
kāryakrama fast_add(a: saṅkhyā-a-k-t32, b: saṅkhyā-a-k-t32) -> saṅkhyā-a-k-t32 {
    phera a + b
}
"#;
    let result = session.compile(source);
    assert!(result.is_ok(), "Rajas mode should compile successfully");
}

/// Test Guṇa mode compilation - Tamas (maximum control)
#[test]
fn test_guna_tamas_mode() {
    let mut options = CompilerOptions::new();
    options.guna = Guna::Tamas;
    options.emit_asm = true; // Only generate assembly
    let mut session = CompilerSession::new(options);
    let source = r#"
kāryakrama low_level() -> saṅkhyā-a-k-t32 {
    phera 0
}
"#;
    let result = session.compile(source);
    assert!(result.is_ok(), "Tamas mode should compile successfully");
}

/// Test philosophy concepts are integrated in the type system
#[test]
fn test_philosophy_integration() {
    // Test that Sanskrit names work as identifiers
    let source = r#"
kāryakrama test() {
    let value = 42;
    let float_val = 3.14;
    let text = "namaste";
}
"#;
    assert!(
        compiles_ok(source),
        "Sanskrit names should work as identifiers"
    );
}
