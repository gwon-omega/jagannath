//! Integration tests for the Jagannath compiler code generation

use jagannath_compiler::driver::options::CompilerOptions;
use jagannath_compiler::driver::CompilerSession;

/// Helper function to compile source to assembly
fn compile_to_asm(source: &str) -> String {
    let options = CompilerOptions::new();
    let mut session = CompilerSession::new(options);
    let result = session.compile(source);
    match result {
        Ok(compile_result) => String::from_utf8(compile_result.output).unwrap_or_default(),
        Err(_) => String::new(),
    }
}

/// Test basic x86-64 code generation
#[test]
fn test_x86_64_basic() {
    let source = r#"
kāryakrama yoga(x: saṅkhyā-a-k-t64, y: saṅkhyā-a-k-t64) -> saṅkhyā-a-k-t64 {
    phera x + y
}
"#;
    let asm = compile_to_asm(source);

    // Should contain x86-64 add instruction
    assert!(
        asm.contains("add"),
        "Assembly should contain add instruction"
    );
    // Should have proper function prologue/epilogue
    assert!(asm.contains("push rbp"), "Assembly should have prologue");
    assert!(asm.contains("ret"), "Assembly should have return");
}

/// Test function with loop
#[test]
fn test_loop_codegen() {
    let source = r#"
kāryakrama count() -> saṅkhyā-a-k-t32 {
    let sum = 0;
    cala i madhye 0..10 {
        sum = sum + i;
    }
    phera sum
}
"#;
    let asm = compile_to_asm(source);

    // Should have loop structure
    assert!(
        asm.contains("jmp") || asm.contains("cmp"),
        "Assembly should have control flow"
    );
}

/// Test conditional code generation
#[test]
fn test_if_codegen() {
    let source = r#"
kāryakrama max(a: saṅkhyā-a-k-t32, b: saṅkhyā-a-k-t32) -> saṅkhyā-a-k-t32 {
    yad a > b {
        phera a
    } anyathā {
        phera b
    }
}
"#;
    let asm = compile_to_asm(source);

    // Should have conditional jumps
    assert!(
        asm.contains("cmp") || asm.contains("je") || asm.contains("jg"),
        "Assembly should have comparison"
    );
}

/// Test basic fibonacci compilation
#[test]
fn test_fibonacci_codegen() {
    let source = r#"
kāryakrama fib(n: saṅkhyā-a-k-t32) -> saṅkhyā-a-k-t32 {
    yad n <= 1 {
        phera n
    }
    phera fib(n - 1) + fib(n - 2)
}
"#;
    let asm = compile_to_asm(source);

    // Should have recursive call structure
    assert!(asm.contains("call fib"), "Should have recursive call");
    assert!(asm.contains("sub"), "Should have subtraction");
    assert!(asm.contains("add"), "Should have addition");
}
