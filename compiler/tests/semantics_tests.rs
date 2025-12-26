//! Integration tests for Jagannath compiler semantic analysis

use jagannath_compiler::driver::options::CompilerOptions;
use jagannath_compiler::driver::CompilerSession;

/// Helper to check if source compiles without errors
fn compiles_ok(source: &str) -> bool {
    let options = CompilerOptions::new();
    let mut session = CompilerSession::new(options);
    session.compile(source).is_ok()
}

/// Test basic type annotations work
#[test]
fn test_explicit_type_annotations() {
    let source = r#"
kāryakrama test() {
    let x: saṅkhyā-a-k-t32 = 42;
}
"#;
    assert!(
        compiles_ok(source),
        "Explicit type annotations should compile"
    );
}

/// Test type inference from literals
#[test]
fn test_type_inference() {
    let source = r#"
kāryakrama test() {
    let x = 42;
    let y = 3.14;
    let s = "namaste";
}
"#;
    assert!(compiles_ok(source), "Type inference should work");
}

/// Test kāraka annotations on parameters
#[test]
fn test_karaka_annotations() {
    let source = r#"
kāryakrama process(data[kartṛ]: saṅkhyā-a-k-t32) -> saṅkhyā-a-k-t32 {
    phera data + 1
}
"#;
    assert!(compiles_ok(source), "Kāraka annotations should parse");
}

/// Test if-else type checking
#[test]
fn test_conditional_types() {
    let source = r#"
kāryakrama max(a: saṅkhyā-a-k-t32, b: saṅkhyā-a-k-t32) -> saṅkhyā-a-k-t32 {
    yad a > b {
        phera a
    } anyathā {
        phera b
    }
}
"#;
    assert!(
        compiles_ok(source),
        "Conditional branches should type check"
    );
}

/// Test loop variable scoping
#[test]
fn test_loop_scoping() {
    let source = r#"
kāryakrama count() -> saṅkhyā-a-k-t32 {
    let sum = 0;
    cala i madhye 0..10 {
        sum = sum + i;
    }
    phera sum
}
"#;
    assert!(
        compiles_ok(source),
        "Loop variables should be properly scoped"
    );
}

/// Test function call type checking
#[test]
fn test_function_calls() {
    // Note: Using Sanskrit names (dviguna = double, mukhya = main)
    // since 'main' is a keyword (pradhāna) in Jagannath
    let source = r#"
kāryakrama dviguna(x: saṅkhyā-a-k-t32) -> saṅkhyā-a-k-t32 {
    phera x * 2
}

kāryakrama mukhya() -> saṅkhyā-a-k-t32 {
    phera dviguna(21)
}
"#;
    assert!(compiles_ok(source), "Function calls should type check");
}
