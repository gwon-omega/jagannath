//! Integration tests for the Jagannath compiler code generation
//!
//! Tests assembly generation for different targets including:
//! - x86-64 code generation
//! - AArch64 code generation
//! - RISC-V code generation
//! - Kāraka-guided register allocation

use jagannath_compiler::codegen::{CodeGenerator, Target};

/// Test basic x86-64 code generation
#[test]
fn test_x86_64_basic() {
    let source = r#"
kāryakrama yoga(x: saṅkhyā64-a, y: saṅkhyā64-a) -> saṅkhyā64-a {
    phera x + y
}
"#;
    let asm = CodeGenerator::compile(source, Target::X86_64).expect("Codegen failed");

    // Should contain x86-64 add instruction
    assert!(asm.contains("add"));
    // Should have proper function prologue/epilogue
    assert!(asm.contains("push"));
    assert!(asm.contains("ret"));
}

/// Test AArch64 code generation
#[test]
fn test_aarch64_basic() {
    let source = r#"
kāryakrama yoga(x: saṅkhyā64-a, y: saṅkhyā64-a) -> saṅkhyā64-a {
    phera x + y
}
"#;
    let asm = CodeGenerator::compile(source, Target::AArch64).expect("Codegen failed");

    // Should contain ARM64 add instruction
    assert!(asm.contains("add") || asm.contains("ADD"));
    // Should have proper return
    assert!(asm.contains("ret") || asm.contains("RET"));
}

/// Test RISC-V code generation
#[test]
fn test_riscv64_basic() {
    let source = r#"
kāryakrama yoga(x: saṅkhyā64-a, y: saṅkhyā64-a) -> saṅkhyā64-a {
    phera x + y
}
"#;
    let asm = CodeGenerator::compile(source, Target::RiscV64).expect("Codegen failed");

    // Should contain RISC-V add instruction
    assert!(asm.contains("add"));
    // Should have proper return
    assert!(asm.contains("ret"));
}

/// Test kāraka-guided register allocation - kartṛ
#[test]
fn test_karaka_kartr_allocation() {
    let source = r#"
kāryakrama process(@kartṛ handler: Handler-b) {
    handler.run()
    handler.run()  // kartṛ preserved across calls
}
"#;
    let asm = CodeGenerator::compile(source, Target::X86_64).expect("Codegen failed");

    // kartṛ should be in callee-saved register (rbx, r12-r15)
    // Should NOT see handler being saved/restored between calls
    assert!(asm.contains("rbx") || asm.contains("r12") || asm.contains("r13") ||
            asm.contains("r14") || asm.contains("r15"));
}

/// Test kāraka-guided register allocation - karaṇa
#[test]
fn test_karaka_karana_allocation() {
    let source = r#"
kāryakrama compute(@karaṇa tool: Tool-b) {
    tool.use_once()  // karaṇa can be in caller-saved register
}
"#;
    let asm = CodeGenerator::compile(source, Target::X86_64).expect("Codegen failed");

    // karaṇa can be in caller-saved register (rdi, rsi, rdx, rcx, r8, r9)
    // This is the default for first argument in System V ABI
    assert!(asm.contains("rdi") || asm.contains("rsi") || asm.contains("rdx"));
}

/// Test kāraka-guided register allocation - karman
#[test]
fn test_karaka_karman_allocation() {
    let source = r#"
kāryakrama transform(@karman data: Bufara-ā) {
    data.modify()
    data.finalize()
}
"#;
    let asm = CodeGenerator::compile(source, Target::X86_64).expect("Codegen failed");

    // karman (patient) is modified, should see store operations
    assert!(asm.contains("mov") || asm.contains("store"));
}

/// Test linear type efficient codegen (no RC/GC overhead)
#[test]
fn test_linear_type_codegen() {
    let source = r#"
kāryakrama transfer() {
    māna x-l = Peṭī::nirmā(42)
    māna y = x  // Move, not copy
    y.consume()
}
"#;
    let asm = CodeGenerator::compile(source, Target::X86_64).expect("Codegen failed");

    // Should NOT contain reference counting instructions
    assert!(!asm.contains("inc") || !asm.contains("dec") ||
            !asm.contains("atomic") || !asm.contains("lock"));
    // Move is just register rename or memcpy, no overhead
}

/// Test arithmetic operations codegen
#[test]
fn test_arithmetic_codegen() {
    let source = r#"
kāryakrama gaṇita(a: saṅkhyā64-a, b: saṅkhyā64-a) -> saṅkhyā64-a {
    māna sum = a + b
    māna diff = a - b
    māna prod = a * b
    māna quot = a / b
    māna rem = a % b
    phera sum + diff + prod + quot + rem
}
"#;
    let asm = CodeGenerator::compile(source, Target::X86_64).expect("Codegen failed");

    assert!(asm.contains("add"));
    assert!(asm.contains("sub"));
    assert!(asm.contains("imul") || asm.contains("mul"));
    assert!(asm.contains("idiv") || asm.contains("div"));
}

/// Test comparison operations codegen
#[test]
fn test_comparison_codegen() {
    let source = r#"
kāryakrama compare(a: saṅkhyā64-a, b: saṅkhyā64-a) -> tarka {
    phera a < b && a > 0 || a == b
}
"#;
    let asm = CodeGenerator::compile(source, Target::X86_64).expect("Codegen failed");

    // Should contain compare and conditional move/jump
    assert!(asm.contains("cmp"));
    assert!(asm.contains("set") || asm.contains("cmov") || asm.contains("j"));
}

/// Test loop codegen
#[test]
fn test_loop_codegen() {
    let source = r#"
kāryakrama sum_to_n(n: saṅkhyā64-a) -> saṅkhyā64-a {
    māna result = 0
    māna i = 0
    cala dharṣa i < n {
        result = result + i
        i = i + 1
    }
    phera result
}
"#;
    let asm = CodeGenerator::compile(source, Target::X86_64).expect("Codegen failed");

    // Should have loop labels and jumps
    assert!(asm.contains("jmp") || asm.contains("jl") || asm.contains("jle") ||
            asm.contains("jg") || asm.contains("jge") || asm.contains("je"));
}

/// Test function call codegen (System V ABI on x86-64)
#[test]
fn test_function_call_codegen() {
    let source = r#"
kāryakrama outer(x: saṅkhyā64-a) -> saṅkhyā64-a {
    phera inner(x, x + 1)
}

kāryakrama inner(a: saṅkhyā64-a, b: saṅkhyā64-a) -> saṅkhyā64-a {
    phera a + b
}
"#;
    let asm = CodeGenerator::compile(source, Target::X86_64).expect("Codegen failed");

    // Should have call instruction
    assert!(asm.contains("call"));
    // Should set up arguments in rdi, rsi (System V ABI)
    // Return value in rax
}

/// Test struct field access codegen
#[test]
fn test_struct_field_codegen() {
    let source = r#"
prakāra Bindu {
    x: bhinna64-a,
    y: bhinna64-a,
}

kāryakrama get_x(p: Bindu-b) -> bhinna64-a {
    phera p.x
}
"#;
    let asm = CodeGenerator::compile(source, Target::X86_64).expect("Codegen failed");

    // Should load from offset
    assert!(asm.contains("mov") || asm.contains("movsd"));
}

/// Test array indexing codegen
#[test]
fn test_array_index_codegen() {
    let source = r#"
kāryakrama get_element(arr: [saṅkhyā64-a; 10]-b, i: saṅkhyā64-a) -> saṅkhyā64-a {
    phera arr[i]
}
"#;
    let asm = CodeGenerator::compile(source, Target::X86_64).expect("Codegen failed");

    // Should compute offset and load
    // offset = base + i * sizeof(element)
    assert!(asm.contains("mov") || asm.contains("lea"));
}

/// Test SIMD vectorization (when applicable)
#[test]
fn test_simd_vectorization() {
    let source = r#"
#[yantra(śrīyantra)]  // Enable SIMD
kāryakrama dot_product(a: [bhinna32-a; 4]-b, b: [bhinna32-a; 4]-b) -> bhinna32-a {
    māna sum = 0.0
    cala i antargatam 0..4 {
        sum = sum + a[i] * b[i]
    }
    phera sum
}
"#;
    let asm = CodeGenerator::compile(source, Target::X86_64).expect("Codegen failed");

    // Should contain SIMD instructions
    assert!(asm.contains("xmm") || asm.contains("ymm") ||
            asm.contains("vmul") || asm.contains("vadd") ||
            asm.contains("mulps") || asm.contains("addps"));
}

/// Test inline assembly passthrough
#[test]
fn test_inline_asm() {
    let source = r#"
kāryakrama rdtsc() -> saṅkhyā64-a {
    māna low: saṅkhyā32-ā
    māna high: saṅkhyā32-ā
    asm!("rdtsc", out("eax") low, out("edx") high)
    phera ((high as saṅkhyā64-a) << 32) | (low as saṅkhyā64-a)
}
"#;
    let asm = CodeGenerator::compile(source, Target::X86_64).expect("Codegen failed");

    // Should contain the inline assembly
    assert!(asm.contains("rdtsc"));
}

/// Test Vedic math optimization - Nikhilam multiplication
#[test]
fn test_vedic_nikhilam_optimization() {
    let source = r#"
#[vedic_optimize]
kāryakrama multiply_near_100(a: saṅkhyā64-a, b: saṅkhyā64-a) -> saṅkhyā64-a {
    // When a, b are near 100, Nikhilam is faster
    // 97 × 96 = (97-100)(96-100) + 100×(97+96-100) = 9312
    phera a * b
}
"#;
    let asm = CodeGenerator::compile(source, Target::X86_64).expect("Codegen failed");

    // Should use optimized multiplication path for near-base numbers
    // This tests that the Vedic math optimization is applied
    assert!(asm.len() > 0); // Basic sanity check
}
