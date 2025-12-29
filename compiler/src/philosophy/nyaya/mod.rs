//! Nyāya Logic System
//!
//! Implements the 4 pramāṇas (means of valid knowledge) for type inference:
//! 1. Pratyakṣa (perception) - explicit type annotations
//! 2. Anumāna (inference) - logical deduction
//! 3. Upamāna (comparison) - pattern matching
//! 4. Śabda (testimony) - documentation/contracts
//!
//! Implements v10.0 unified traits: SanskritNamed, SanskritDescribed, PhilosophicalEnum

pub mod anumana;
pub mod pratyaksha;
pub mod shabda;
pub mod upamana;

pub use anumana::{AnumanaEngine, InferenceContext, VyaptiRule};
pub use pratyaksha::PratyakshaResolver;
pub use shabda::ShabdaAnalyzer;
pub use upamana::UpamanaEngine;

use crate::traits::{PhilosophicalEnum, SanskritDescribed, SanskritNamed};
use std::collections::HashMap;

/// The 4 Nyāya pramāṇas (valid means of knowledge)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pramana {
    /// Pratyakṣa - Direct perception (explicit annotation)
    /// Certainty: 100%
    Pratyaksha,

    /// Anumāna - Inference (logical deduction)
    /// Certainty: 95%
    Anumana,

    /// Śabda - Testimony (documentation/contract)
    /// Certainty: 90%
    Shabda,

    /// Upamāna - Comparison (pattern matching)
    /// Certainty: 85%
    Upamana,
}

impl Pramana {
    /// Get the certainty level of this pramāṇa
    pub fn certainty(&self) -> f32 {
        match self {
            Self::Pratyaksha => 1.00,
            Self::Anumana => 0.95,
            Self::Shabda => 0.90,
            Self::Upamana => 0.85,
        }
    }

    /// Get Sanskrit name (Devanagari script)
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::Pratyaksha => "प्रत्यक्ष",
            Self::Anumana => "अनुमान",
            Self::Shabda => "शब्द",
            Self::Upamana => "उपमान",
        }
    }

    /// Get IAST transliteration
    pub fn iast(&self) -> &'static str {
        match self {
            Self::Pratyaksha => "Pratyakṣa",
            Self::Anumana => "Anumāna",
            Self::Shabda => "Śabda",
            Self::Upamana => "Upamāna",
        }
    }

    /// Get English name
    pub fn english(&self) -> &'static str {
        match self {
            Self::Pratyaksha => "Perception",
            Self::Anumana => "Inference",
            Self::Shabda => "Testimony",
            Self::Upamana => "Comparison",
        }
    }

    /// Get all pramāṇas in epistemological order
    pub fn all() -> &'static [Pramana] {
        &[
            Pramana::Pratyaksha,
            Pramana::Anumana,
            Pramana::Shabda,
            Pramana::Upamana,
        ]
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// v10.0 UNIFIED TRAIT IMPLEMENTATIONS
// ═══════════════════════════════════════════════════════════════════════════════

impl SanskritNamed for Pramana {
    fn sanskrit(&self) -> &'static str {
        self.sanskrit_name()
    }

    fn iast(&self) -> &'static str {
        self.iast()
    }

    fn english(&self) -> &'static str {
        self.english()
    }
}

impl SanskritDescribed for Pramana {
    fn meaning(&self) -> &'static str {
        match self {
            Self::Pratyaksha => "Direct perception through the senses - explicit type annotations",
            Self::Anumana => "Logical inference through reasoning - type deduction",
            Self::Shabda => "Reliable testimony from authority - documentation/contracts",
            Self::Upamana => "Knowledge through comparison/analogy - pattern matching",
        }
    }

    fn explanation(&self) -> &'static str {
        match self {
            Self::Pratyaksha => "The most reliable pramāṇa with 100% certainty, used when type is explicitly declared",
            Self::Anumana => "Uses vyāpti (universal concomitance) for logical deduction with 95% certainty",
            Self::Shabda => "Relies on āpta (reliable source) for type information with 90% certainty",
            Self::Upamana => "Establishes type through sādṛśya (similarity) with 85% certainty",
        }
    }

    fn mantra(&self) -> Option<&'static str> {
        Some(match self {
            Self::Pratyaksha => "इन्द्रियार्थसन्निकर्षोत्पन्नं ज्ञानं प्रत्यक्षम् (Indriyārtha-sannikarṣotpannam jñānam pratyakṣam)",
            Self::Anumana => "व्याप्तिज्ञानं लिङ्गपरामर्शः (Vyāptijñānaṃ liṅgaparāmarśaḥ)",
            Self::Shabda => "आप्तोपदेशः शब्दः (Āptopadeśaḥ śabdaḥ)",
            Self::Upamana => "सादृश्यज्ञानं संज्ञासंज्ञिसम्बन्धः (Sādṛśyajñānaṃ saṃjñā-saṃjñi-sambandhaḥ)",
        })
    }

    fn category(&self) -> &'static str {
        "Nyāya Epistemology (न्याय प्रमाण)"
    }
}

impl PhilosophicalEnum for Pramana {
    fn all() -> &'static [Self] {
        Pramana::all()
    }

    fn count() -> usize {
        4
    }

    fn index(&self) -> usize {
        match self {
            Self::Pratyaksha => 0,
            Self::Anumana => 1,
            Self::Shabda => 2,
            Self::Upamana => 3,
        }
    }

    fn ordinal(&self) -> usize {
        self.index() + 1
    }

    fn next(&self) -> Self {
        let idx = self.index();
        Self::all()[(idx + 1) % 4]
    }

    fn prev(&self) -> Self {
        let idx = self.index();
        Self::all()[(idx + 4 - 1) % 4]
    }

    fn from_index(index: usize) -> Option<Self> {
        Self::all().get(index).copied()
    }
}

/// Nyāya inference engine
pub struct NyayaInference {
    /// Known types (pratyakṣa)
    known_types: HashMap<String, TypeEvidence>,
    /// Inference rules (anumāna)
    inference_rules: Vec<InferenceRule>,
    /// Pattern database (upamāna)
    patterns: Vec<TypePattern>,
}

/// Evidence for a type assignment
#[derive(Debug, Clone)]
pub struct TypeEvidence {
    /// The inferred type
    pub type_name: String,
    /// Which pramāṇa was used
    pub pramana: Pramana,
    /// Certainty level
    pub certainty: f32,
    /// Evidence chain
    pub evidence: Vec<String>,
}

/// Inference rule for anumāna
#[derive(Debug, Clone)]
pub struct InferenceRule {
    /// Rule name
    pub name: String,
    /// Premises
    pub premises: Vec<Premise>,
    /// Conclusion
    pub conclusion: String,
}

/// Premise for inference
#[derive(Debug, Clone)]
pub struct Premise {
    /// Pattern to match
    pub pattern: String,
    /// Type constraint
    pub constraint: String,
}

/// Type pattern for upamāna
#[derive(Debug, Clone)]
pub struct TypePattern {
    /// Pattern name
    pub name: String,
    /// Example expressions
    pub examples: Vec<String>,
    /// Resulting type
    pub result_type: String,
}

impl NyayaInference {
    pub fn new() -> Self {
        Self {
            known_types: HashMap::new(),
            inference_rules: Vec::new(),
            patterns: Vec::new(),
        }
    }

    /// Try pratyakṣa (explicit type)
    pub fn pratyaksha(&self, name: &str) -> Option<TypeEvidence> {
        self.known_types.get(name).cloned()
    }

    /// Record explicit type (pratyakṣa)
    pub fn record_pratyaksha(&mut self, name: String, type_name: String) {
        self.known_types.insert(
            name.clone(),
            TypeEvidence {
                type_name,
                pramana: Pramana::Pratyaksha,
                certainty: 1.0,
                evidence: vec![format!("Explicit annotation on '{}'", name)],
            },
        );
    }

    /// Try anumāna (inference)
    /// Uses the classic Nyāya 5-step syllogism:
    /// 1. Pratijñā (Proposition): "x has type T"
    /// 2. Hetu (Reason): "because x is used as T"
    /// 3. Udāharaṇa (Example): "like other T values"
    /// 4. Upanaya (Application): "x is similar"
    /// 5. Nigamana (Conclusion): "therefore x: T"
    pub fn anumana(&self, expression: &str, context: &[(&str, &str)]) -> Option<TypeEvidence> {
        // Try each inference rule
        for rule in &self.inference_rules {
            if let Some(ty) = self.apply_rule(rule, expression, context) {
                return Some(TypeEvidence {
                    type_name: ty,
                    pramana: Pramana::Anumana,
                    certainty: 0.95,
                    evidence: vec![
                        format!("Pratijñā: '{}' has type T", expression),
                        format!("Hetu: Rule '{}' applies", rule.name),
                        "Nigamana: Type inferred".to_string(),
                    ],
                });
            }
        }
        None
    }

    /// Apply an inference rule
    fn apply_rule(&self, rule: &InferenceRule, expr: &str, ctx: &[(&str, &str)]) -> Option<String> {
        // Check if all premises match
        for premise in &rule.premises {
            if !self.premise_matches(&premise, expr, ctx) {
                return None;
            }
        }
        Some(rule.conclusion.clone())
    }

    /// Check if a premise matches
    fn premise_matches(&self, _premise: &Premise, _expr: &str, _ctx: &[(&str, &str)]) -> bool {
        // TODO: Implement pattern matching
        false
    }

    /// Try śabda (documentation/contract)
    pub fn shabda(&self, _name: &str, docs: &str) -> Option<TypeEvidence> {
        // Parse documentation for type hints
        // Look for patterns like "@type T" or "returns T"
        if docs.contains("@returns") || docs.contains("-> ") {
            // TODO: Extract type from documentation
        }
        None
    }

    /// Try upamāna (pattern matching)
    pub fn upamana(&self, expression: &str) -> Option<TypeEvidence> {
        for pattern in &self.patterns {
            for example in &pattern.examples {
                if self.expressions_similar(expression, example) {
                    return Some(TypeEvidence {
                        type_name: pattern.result_type.clone(),
                        pramana: Pramana::Upamana,
                        certainty: 0.85,
                        evidence: vec![
                            format!("Pattern '{}' matches", pattern.name),
                            format!("Similar to: {}", example),
                        ],
                    });
                }
            }
        }
        None
    }

    /// Check if two expressions are similar (for upamāna)
    fn expressions_similar(&self, _a: &str, _b: &str) -> bool {
        // TODO: Implement similarity check
        false
    }

    /// Full inference trying all pramāṇas in order
    pub fn infer(
        &self,
        name: &str,
        expression: &str,
        context: &[(&str, &str)],
        docs: &str,
    ) -> Option<TypeEvidence> {
        // 1. Pratyakṣa (highest certainty)
        if let Some(ty) = self.pratyaksha(name) {
            return Some(ty);
        }

        // 2. Anumāna
        if let Some(ty) = self.anumana(expression, context) {
            return Some(ty);
        }

        // 3. Śabda
        if let Some(ty) = self.shabda(name, docs) {
            return Some(ty);
        }

        // 4. Upamāna (lowest certainty)
        if let Some(ty) = self.upamana(expression) {
            return Some(ty);
        }

        None
    }

    /// Add an inference rule
    pub fn add_rule(&mut self, rule: InferenceRule) {
        self.inference_rules.push(rule);
    }

    /// Add a type pattern
    pub fn add_pattern(&mut self, pattern: TypePattern) {
        self.patterns.push(pattern);
    }
}

impl Default for NyayaInference {
    fn default() -> Self {
        Self::new()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pramana_certainty() {
        assert_eq!(Pramana::Pratyaksha.certainty(), 1.00);
        assert_eq!(Pramana::Anumana.certainty(), 0.95);
        assert_eq!(Pramana::Shabda.certainty(), 0.90);
        assert_eq!(Pramana::Upamana.certainty(), 0.85);
    }

    #[test]
    fn test_pramana_ordering() {
        // Pramanas should be ordered by certainty (highest first)
        assert!(Pramana::Pratyaksha < Pramana::Anumana);
        assert!(Pramana::Anumana < Pramana::Shabda);
        assert!(Pramana::Shabda < Pramana::Upamana);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // v10.0 TRAIT IMPLEMENTATION TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_pramana_sanskrit_named_trait() {
        let p = Pramana::Pratyaksha;
        assert_eq!(SanskritNamed::sanskrit(&p), "प्रत्यक्ष");
        assert_eq!(SanskritNamed::iast(&p), "Pratyakṣa");
        assert_eq!(SanskritNamed::english(&p), "Perception");

        let a = Pramana::Anumana;
        assert_eq!(SanskritNamed::sanskrit(&a), "अनुमान");
        assert_eq!(SanskritNamed::iast(&a), "Anumāna");
        assert_eq!(SanskritNamed::english(&a), "Inference");
    }

    #[test]
    fn test_pramana_sanskrit_described_trait() {
        let s = Pramana::Shabda;
        assert!(s.meaning().contains("testimony"));
        assert!(s.explanation().contains("āpta"));
        assert!(s.mantra().is_some());
        assert_eq!(s.category(), "Nyāya Epistemology (न्याय प्रमाण)");
    }

    #[test]
    fn test_pramana_philosophical_enum_trait() {
        // Test count
        assert_eq!(Pramana::count(), 4);

        // Test index
        assert_eq!(Pramana::Pratyaksha.index(), 0);
        assert_eq!(Pramana::Upamana.index(), 3);

        // Test ordinal
        assert_eq!(Pramana::Pratyaksha.ordinal(), 1);
        assert_eq!(Pramana::Upamana.ordinal(), 4);

        // Test navigation (wrapping)
        assert_eq!(Pramana::Pratyaksha.next(), Pramana::Anumana);
        assert_eq!(Pramana::Upamana.next(), Pramana::Pratyaksha);
        assert_eq!(Pramana::Pratyaksha.prev(), Pramana::Upamana);

        // Test from_index
        assert_eq!(Pramana::from_index(0), Some(Pramana::Pratyaksha));
        assert_eq!(Pramana::from_index(3), Some(Pramana::Upamana));
        assert_eq!(Pramana::from_index(4), None);
    }

    #[test]
    fn test_pramana_all_have_mantras() {
        for pramana in Pramana::all() {
            assert!(
                pramana.mantra().is_some(),
                "Missing mantra for {:?}",
                pramana
            );
        }
    }

    #[test]
    fn test_nyaya_inference_creation() {
        let inference = NyayaInference::new();
        assert!(inference.known_types.is_empty());
    }

    #[test]
    fn test_nyaya_record_pratyaksha() {
        let mut inference = NyayaInference::new();
        inference.record_pratyaksha("x".to_string(), "i32".to_string());

        let evidence = inference.pratyaksha("x").unwrap();
        assert_eq!(evidence.type_name, "i32");
        assert_eq!(evidence.pramana, Pramana::Pratyaksha);
        assert_eq!(evidence.certainty, 1.0);
    }
}
