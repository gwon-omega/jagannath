//! Integration tests for the Jagannath compiler semantic analysis
//!
//! Tests type checking, kāraka analysis, and lifetime inference including:
//! - Nyāya 4-pramāṇa type inference
//! - Kāraka role verification
//! - Borrow checking
//! - Lifetime region analysis

use jagannath_compiler::semantics::{TypeChecker, KarakaAnalyzer, BorrowChecker};

/// Test basic type inference (Pratyakṣa - explicit)
#[test]
fn test_pratyaksha_inference() {
    let source = r#"
kāryakrama parikṣā() {
    māna x: saṅkhyā32-a = 42
    māna y: bhinna64-a = 3.14
    māna s: sūtra-a = "namaste"
}
"#;
    let result = TypeChecker::check(source);
    assert!(result.is_ok());
}

/// Test type inference (Anumāna - from context)
#[test]
fn test_anumana_inference() {
    let source = r#"
kāryakrama parikṣā() {
    māna x = 42           // Infer saṅkhyā from literal
    māna y = 3.14         // Infer bhinna from literal
    māna z = x + 10       // Infer saṅkhyā from operation
}
"#;
    let result = TypeChecker::check(source);
    assert!(result.is_ok());

    let types = result.unwrap();
    assert!(types.get("x").unwrap().is_integer());
    assert!(types.get("y").unwrap().is_float());
    assert!(types.get("z").unwrap().is_integer());
}

/// Test kāraka agent (kartṛ) role verification
#[test]
fn test_karaka_kartr() {
    let source = r#"
kāryakrama prayoga(@kartṛ processor: Processor-b) {
    // kartṛ should be in callee-saved register
    // kartṛ performs the action but is not modified
    processor.process()
}
"#;
    let result = KarakaAnalyzer::analyze(source);
    assert!(result.is_ok());

    let karaka = result.unwrap();
    assert_eq!(karaka.get("processor"), Some(&Karaka::Kartr));
}

/// Test kāraka patient (karman) role verification
#[test]
fn test_karaka_karman() {
    let source = r#"
kāryakrama saṃskaraṇa(@karman data: Bufara-ā) {
    // karman is the patient - what gets modified
    data.transform()
}
"#;
    let result = KarakaAnalyzer::analyze(source);
    assert!(result.is_ok());

    let karaka = result.unwrap();
    assert_eq!(karaka.get("data"), Some(&Karaka::Karman));
}

/// Test kāraka instrument (karaṇa) role verification
#[test]
fn test_karaka_karana() {
    let source = r#"
kāryakrama likha(@kartṛ writer: Writer-b, @karaṇa pen: Pen-b, @karman paper: Paper-ā) {
    // karaṇa is the instrument - used to perform action
    writer.write_with(pen, paper)
}
"#;
    let result = KarakaAnalyzer::analyze(source);
    assert!(result.is_ok());

    let karaka = result.unwrap();
    assert_eq!(karaka.get("writer"), Some(&Karaka::Kartr));
    assert_eq!(karaka.get("pen"), Some(&Karaka::Karana));
    assert_eq!(karaka.get("paper"), Some(&Karaka::Karman));
}

/// Test linear type checking (single ownership)
#[test]
fn test_linear_types() {
    let source = r#"
kāryakrama parikṣā() {
    māna x-l = Peṭī::nirmā(42)  // Linear box
    māna y = x                    // Move x to y
    // x is now consumed, using it should fail
}
"#;
    let result = TypeChecker::check(source);
    assert!(result.is_ok());

    // Using x after move should fail
    let bad_source = r#"
kāryakrama parikṣā() {
    māna x-l = Peṭī::nirmā(42)
    māna y = x
    mudraṇa!(x)  // ERROR: x was moved
}
"#;
    let result = TypeChecker::check(bad_source);
    assert!(result.is_err());
    assert!(result.unwrap_err().is_use_after_move());
}

/// Test borrowed reference checking
#[test]
fn test_borrow_checking() {
    let source = r#"
kāryakrama parikṣā() {
    māna x = 42
    māna y-b = &x        // Immutable borrow
    mudraṇa!(y)
    mudraṇa!(x)          // OK: x still accessible
}
"#;
    let result = BorrowChecker::check(source);
    assert!(result.is_ok());
}

/// Test mutable borrow exclusivity
#[test]
fn test_mutable_borrow_exclusive() {
    let source = r#"
kāryakrama parikṣā() {
    māna x-ā = 42
    māna y-b = &ā x      // Mutable borrow
    // x cannot be accessed while y exists
}
"#;
    let result = BorrowChecker::check(source);
    assert!(result.is_ok());

    // Accessing x while mutable borrow exists should fail
    let bad_source = r#"
kāryakrama parikṣā() {
    māna x-ā = 42
    māna y-b = &ā x
    mudraṇa!(x)  // ERROR: x is mutably borrowed
    y
}
"#;
    let result = BorrowChecker::check(bad_source);
    assert!(result.is_err());
}

/// Test lifetime region analysis
#[test]
fn test_lifetime_regions() {
    let source = r#"
kāryakrama parikṣā() {
    māna x^1 = 42        // Region 1
    {
        māna y^2 = &x    // Region 2, borrows from Region 1
        mudraṇa!(y)
    }                     // Region 2 ends, borrow released
    mudraṇa!(x)          // OK: no more borrows
}
"#;
    let result = BorrowChecker::check(source);
    assert!(result.is_ok());
}

/// Test lifetime region escaping detection
#[test]
fn test_lifetime_escape() {
    // Reference should not escape its region
    let bad_source = r#"
kāryakrama escape() -> &saṅkhyā-a {
    māna x^1 = 42
    phera &x  // ERROR: x does not live long enough
}
"#;
    let result = BorrowChecker::check(bad_source);
    assert!(result.is_err());
    assert!(result.unwrap_err().is_lifetime_escape());
}

/// Test type coercion rules
#[test]
fn test_type_coercion() {
    let source = r#"
kāryakrama parikṣā() {
    māna x: saṅkhyā32-a = 42
    māna y: saṅkhyā64-a = x  // Widening OK

    māna a: bhinna32-a = 3.14
    māna b: bhinna64-a = a   // Widening OK
}
"#;
    let result = TypeChecker::check(source);
    assert!(result.is_ok());
}

/// Test trait bound checking
#[test]
fn test_trait_bounds() {
    let source = r#"
guṇa Gaṇita {
    kāryakrama yoga(sva-b, anya: Sva-b) -> Sva
}

kāryakrama sum<T: Gaṇita>(items: Sūcī<T>-b) -> T {
    māna result = T::śūnya()
    cala item antargatam items {
        result = result.yoga(item)
    }
    phera result
}
"#;
    let result = TypeChecker::check(source);
    assert!(result.is_ok());
}

/// Test invalid trait bound detection
#[test]
fn test_invalid_trait_bound() {
    let bad_source = r#"
kāryakrama sum<T>(items: Sūcī<T>-b) -> T {
    māna result = T::śūnya()  // ERROR: T has no trait bounds
    phera result
}
"#;
    let result = TypeChecker::check(bad_source);
    assert!(result.is_err());
}

/// Test suffix compatibility checking
#[test]
fn test_suffix_compatibility() {
    // -l (linear) and -b (borrowed) are incompatible
    let bad_source = r#"
prakāra Invalid-l-b {  // ERROR: cannot be both linear and borrowed
    value: saṅkhyā-a,
}
"#;
    let result = TypeChecker::check(bad_source);
    assert!(result.is_err());
    assert!(result.unwrap_err().is_incompatible_suffixes());
}

/// Test Pancha Kosha memory tier analysis
#[test]
fn test_kosha_tier_analysis() {
    let source = r#"
kāryakrama parikṣā() {
    māna hot-anna = 42           // Should be in register (annamaya)
    māna warm-prāṇa = large_data // Should be in L2 (prāṇamaya)
    māna cold-manas = huge_data  // Should be in RAM (manomaya)
}
"#;
    let result = TypeChecker::check(source);
    assert!(result.is_ok());

    let tiers = result.unwrap().kosha_tiers();
    assert_eq!(tiers.get("hot"), Some(&Kosha::Annamaya));
    assert_eq!(tiers.get("warm"), Some(&Kosha::Pranamaya));
    assert_eq!(tiers.get("cold"), Some(&Kosha::Manomaya));
}

/// Test Guṇa mode selection
#[test]
fn test_guna_mode() {
    // Sattva mode: maximum correctness
    let sattva_source = r#"
#[guṇa(sattva)]
kāryakrama safe_division(x: saṅkhyā-a, y: saṅkhyā-a) -> Parināma<saṅkhyā-a, Doṣa> {
    yad y == 0 {
        phera Asaphala(Doṣa::DivideByZero)
    }
    phera Saphala(x / y)
}
"#;
    let result = TypeChecker::check(sattva_source);
    assert!(result.is_ok());

    // Rajas mode: maximum performance
    let rajas_source = r#"
#[guṇa(rajas)]
kāryakrama fast_division(x: saṅkhyā-a, y: saṅkhyā-a) -> saṅkhyā-a {
    // Assumes y != 0 (no check)
    phera x / y
}
"#;
    let result = TypeChecker::check(rajas_source);
    assert!(result.is_ok());
}
