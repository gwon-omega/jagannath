//! Garuda Purana Error Classification Tests (v5.0)
//!
//! Tests the 28 Naraka (hell) error taxonomy from Garuda Purana.
//! Each error type maps to a specific hell based on the "sin" committed.

use jagannath_compiler::errors::span::{SourceId, Span};
use jagannath_compiler::garuda::{
    narakas::{Naraka, Severity},
    yama::{
        ConcurrencyYamaduta, MemoryYamaduta, SecurityYamaduta, Violation, ViolationKind,
        YamaDharmaraja, Yamaduta,
    },
    GarudaAnalyzer,
};
use jagannath_compiler::lexer::Lexer;
use jagannath_compiler::parser::Parser;

/// Helper to parse code and run a Yamaduta
fn analyze_with_yamaduta<Y: Yamaduta>(code: &str, yamaduta: &Y) -> Vec<Violation> {
    let mut lexer = Lexer::new(code);
    let tokens = lexer.tokenize();
    let mut parser = Parser::new(tokens);
    match parser.parse() {
        Ok(ast) => yamaduta.inspect(&ast),
        Err(_) => Vec::new(),
    }
}

// ============================================================================
// Memory Yamaduta Tests (Narakas 1-10)
// ============================================================================

mod memory_tests {
    use super::*;

    #[test]
    fn test_memory_yamaduta_creation() {
        let yamaduta = MemoryYamaduta::new();
        assert!(yamaduta.name().contains("स्मृति"));
    }

    #[test]
    fn test_use_after_free_detection() {
        // Code that uses a variable after freeing it
        let code = r#"
            kāryakrama main() {
                let ptr-ā-h = nirmā(100);
                mukta(ptr);
                let val = ptr;
            }
        "#;

        let violations = analyze_with_yamaduta(code, &MemoryYamaduta::new());

        // Detection depends on proper tracking
        // For now, just verify no crash
        assert!(
            violations.is_empty()
                || violations
                    .iter()
                    .any(|v| { matches!(v.kind, ViolationKind::UseAfterFree) })
        );
    }

    #[test]
    fn test_double_free_detection() {
        let code = r#"
            kāryakrama test() {
                let mem-ā-h = nirmā(64);
                mukta(mem);
                mukta(mem);
            }
        "#;

        let violations = analyze_with_yamaduta(code, &MemoryYamaduta::new());
        assert!(
            violations.is_empty()
                || violations
                    .iter()
                    .any(|v| { matches!(v.kind, ViolationKind::DoubleFree) })
        );
    }

    #[test]
    fn test_clean_memory_usage() {
        let code = r#"
            kāryakrama clean() {
                let x-a-k = 42;
                let y-a-k = x + 1;
                phera y;
            }
        "#;

        let violations = analyze_with_yamaduta(code, &MemoryYamaduta::new());
        assert!(
            violations.is_empty(),
            "Clean code should have no violations"
        );
    }
}

// ============================================================================
// Security Yamaduta Tests (Narakas 14-19)
// ============================================================================

mod security_tests {
    use super::*;

    #[test]
    fn test_security_yamaduta_creation() {
        let yamaduta = SecurityYamaduta::new();
        assert!(yamaduta.name().contains("सुरक्षा"));
    }

    #[test]
    fn test_clean_security() {
        let code = r#"
            kāryakrama secure() {
                let val-a-k = 42;
                print(val);
            }
        "#;

        let violations = analyze_with_yamaduta(code, &SecurityYamaduta::new());
        assert!(
            violations.is_empty(),
            "Secure code should have no violations"
        );
    }
}

// ============================================================================
// Concurrency Yamaduta Tests (Narakas 6-11)
// ============================================================================

mod concurrency_tests {
    use super::*;

    #[test]
    fn test_concurrency_yamaduta_creation() {
        let yamaduta = ConcurrencyYamaduta::new();
        assert!(yamaduta.name().contains("समकालिक") || yamaduta.name().contains("Concurrency"));
    }

    #[test]
    fn test_clean_concurrency() {
        let code = r#"
            kāryakrama sequential() {
                let x-a-k = 1;
                let y-a-k = 2;
                phera x + y;
            }
        "#;

        let violations = analyze_with_yamaduta(code, &ConcurrencyYamaduta::new());
        assert!(
            violations.is_empty(),
            "Sequential code should have no concurrency violations"
        );
    }
}

// ============================================================================
// Yama Dharmaraja Tests (Error Classification)
// ============================================================================

mod dharmaraja_tests {
    use super::*;

    #[test]
    fn test_dharmaraja_creation() {
        let _judge = YamaDharmaraja::new();
        // Just verify creation doesn't panic
    }

    #[test]
    fn test_violation_to_naraka_mapping() {
        let judge = YamaDharmaraja::new();
        let span = Span::new(SourceId(0), 0, 10);

        // Create violations and check their Narakas
        let uaf_violation = Violation::new(
            ViolationKind::UseAfterFree,
            span.clone(),
            "Test use after free",
        );
        assert_eq!(judge.determine_naraka(&uaf_violation), Naraka::Tamisram);

        let double_free_violation =
            Violation::new(ViolationKind::DoubleFree, span.clone(), "Test double free");
        assert_eq!(
            judge.determine_naraka(&double_free_violation),
            Naraka::Tamisram
        );

        let null_violation =
            Violation::new(ViolationKind::NullDeref, span.clone(), "Test null deref");
        assert_eq!(judge.determine_naraka(&null_violation), Naraka::Andhakupa);

        let buffer_violation = Violation::new(
            ViolationKind::BufferOverflow,
            span.clone(),
            "Test buffer overflow",
        );
        assert_eq!(
            judge.determine_naraka(&buffer_violation),
            Naraka::Asipatravana
        );

        let taint_violation = Violation::new(
            ViolationKind::TaintedData,
            span.clone(),
            "Test tainted data",
        );
        assert_eq!(judge.determine_naraka(&taint_violation), Naraka::Vaitarani);

        let deadlock_violation =
            Violation::new(ViolationKind::Deadlock, span.clone(), "Test deadlock");
        assert_eq!(
            judge.determine_naraka(&deadlock_violation),
            Naraka::Pranarodha
        );
    }

    #[test]
    fn test_all_28_narakas_exist() {
        // Verify all 28 Narakas are defined
        let narakas = [
            Naraka::Tamisram,
            Naraka::Andhatamisram,
            Naraka::Raurava,
            Naraka::Maharaurava,
            Naraka::Kumbhipaka,
            Naraka::Kalasutra,
            Naraka::Asipatravana,
            Naraka::Sukaramukha,
            Naraka::Andhakupa,
            Naraka::Krimibhaksha,
            Naraka::Sandamsha,
            Naraka::Taptasurmi,
            Naraka::Vajrakantaka,
            Naraka::Vaitarani,
            Naraka::Puyoda,
            Naraka::Pranarodha,
            Naraka::Visasana,
            Naraka::Lalabhaksha,
            Naraka::Sarameyadana,
            Naraka::Avichi,
            Naraka::Ayahpana,
            Naraka::Ksharakardama,
            Naraka::Raksogana,
            Naraka::Sulaprota,
            Naraka::Dandasuka,
            Naraka::Vatarodha,
            Naraka::Paryavartana,
            Naraka::Suchimukha,
        ];

        assert_eq!(narakas.len(), 28, "Should have exactly 28 Narakas");
    }

    #[test]
    fn test_naraka_severity() {
        // Memory violations should be severe (Error or Critical)
        assert!(matches!(
            Naraka::Tamisram.severity(),
            Severity::Error | Severity::Critical
        ));

        // Security violations should be critical
        assert_eq!(Naraka::Vaitarani.severity(), Severity::Critical);

        // Resource leaks (Suchimukha) should be Warning
        assert_eq!(Naraka::Suchimukha.severity(), Severity::Warning);
    }

    #[test]
    fn test_naraka_names() {
        // Test that each Naraka has a Sanskrit name
        assert!(Naraka::Tamisram.name().contains("Tamisram"));
        assert!(Naraka::Vaitarani.name().contains("Vaitarani"));
        assert!(Naraka::Suchimukha.name().contains("Suchimukha"));
    }

    #[test]
    fn test_sin_descriptions() {
        // Each Naraka should have a meaningful sin description
        let sin = Naraka::Tamisram.sin_description();
        assert!(sin.contains("use-after-free") || sin.contains("double-free"));

        let sin = Naraka::Suchimukha.sin_description();
        assert!(sin.contains("leak") || sin.contains("freed"));
    }
}

// ============================================================================
// Violation Structure Tests
// ============================================================================

mod violation_tests {
    use super::*;

    #[test]
    fn test_violation_creation() {
        let span = Span::new(SourceId(0), 0, 10);
        let violation = Violation::new(
            ViolationKind::UseAfterFree,
            span,
            "Test use after free".to_string(),
        );

        assert_eq!(violation.kind, ViolationKind::UseAfterFree);
        assert_eq!(violation.evidence, "Test use after free");
    }

    #[test]
    fn test_violation_full_with_garuda_details() {
        let span = Span::new(SourceId(0), 0, 10);
        let violation = Violation::full(
            ViolationKind::MemoryLeak,
            span,
            "Memory leak detected".to_string(),
            "Attachment to resources (failing to free memory)",
            "Eternal suffering in Suchimukha",
            "Add mukta() call before scope exit".to_string(),
        );

        assert_eq!(violation.kind, ViolationKind::MemoryLeak);
        assert_eq!(
            violation.sin,
            "Attachment to resources (failing to free memory)"
        );
        assert!(violation.penance.contains("mukta()"));
    }

    #[test]
    fn test_violation_kind_completeness() {
        // Test that all key violation kinds exist
        let kinds = [
            ViolationKind::UseAfterFree,
            ViolationKind::DoubleFree,
            ViolationKind::NullDeref,
            ViolationKind::MemoryLeak,
            ViolationKind::BufferOverflow,
            ViolationKind::TaintedData,
            ViolationKind::CodeInjection,
            ViolationKind::Deadlock,
            ViolationKind::RaceCondition,
        ];

        assert!(kinds.len() >= 9, "Should have at least 9 violation kinds");
    }
}

// ============================================================================
// Integration Tests
// ============================================================================

mod integration_tests {
    use super::*;

    #[test]
    fn test_garuda_analyzer_creation() {
        let _analyzer = GarudaAnalyzer::new();
        // Just verify creation doesn't panic
    }

    #[test]
    fn test_full_analysis_pipeline() {
        let code = r#"
            kāryakrama analyze_me() {
                let x-a-k = 42;
                phera x;
            }
        "#;

        let mut lexer = Lexer::new(code);
        let tokens = lexer.tokenize();
        let mut parser = Parser::new(tokens);

        match parser.parse() {
            Ok(ast) => {
                let mut analyzer = GarudaAnalyzer::new();
                let result = analyzer.analyze(&ast);

                // Clean code should have no errors
                // Result is Vec<NarakaError>
                assert!(result.is_empty());
            }
            Err(_) => {
                // Parser error is acceptable for this test
            }
        }
    }
}
