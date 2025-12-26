//! Nyāya Logic System
//!
//! Implements the 4 pramāṇas (means of valid knowledge) for type inference:
//! 1. Pratyakṣa (perception) - explicit type annotations
//! 2. Anumāna (inference) - logical deduction
//! 3. Upamāna (comparison) - pattern matching
//! 4. Śabda (testimony) - documentation/contracts

pub mod pratyaksha;
pub mod anumana;
pub mod upamana;
pub mod shabda;

pub use pratyaksha::PratyakshaResolver;
pub use anumana::{AnumanaEngine, InferenceContext, VyaptiRule};
pub use upamana::UpamanaEngine;
pub use shabda::ShabdaAnalyzer;

use std::collections::HashMap;

/// The 4 Nyāya pramāṇas
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

    /// Get Sanskrit name
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::Pratyaksha => "प्रत्यक्ष",
            Self::Anumana => "अनुमान",
            Self::Shabda => "शब्द",
            Self::Upamana => "उपमान",
        }
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
        self.known_types.insert(name.clone(), TypeEvidence {
            type_name,
            pramana: Pramana::Pratyaksha,
            certainty: 1.0,
            evidence: vec![format!("Explicit annotation on '{}'", name)],
        });
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
    fn premise_matches(&self, premise: &Premise, expr: &str, ctx: &[(&str, &str)]) -> bool {
        // TODO: Implement pattern matching
        false
    }

    /// Try śabda (documentation/contract)
    pub fn shabda(&self, name: &str, docs: &str) -> Option<TypeEvidence> {
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
    fn expressions_similar(&self, a: &str, b: &str) -> bool {
        // TODO: Implement similarity check
        false
    }

    /// Full inference trying all pramāṇas in order
    pub fn infer(&self, name: &str, expression: &str, context: &[(&str, &str)], docs: &str) -> Option<TypeEvidence> {
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
