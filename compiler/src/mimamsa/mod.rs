//! Mīmāṃsā - 6 Pramāṇas for Extended Type Inference
//!
//! Extends Nyāya's 4 pramāṇas with 2 more:
//! 5. Arthāpatti (presumption) - Contextual inference
//! 6. Anupalabdhi (non-apprehension) - Negative type proof

// Submodules
pub mod anupalabdhi;
pub mod arthapatti;

// Re-exports
pub use anupalabdhi::AbsenceRule as AnupalabdhiAbsenceRule;
pub use anupalabdhi::{AbsenceConclusion, AbsenceProof, AnupalabdhiEngine, SoughtFeature};
pub use arthapatti::{
    ArthapattEngine, ArthapattRule, Conclusion, Impossibility, InferredType, Observed,
};

use super::philosophy::nyaya::{NyayaInference, Pramana, TypeEvidence};

/// Extended pramāṇas (Mīmāṃsā system)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MimamsaPramana {
    /// Original 4 from Nyāya
    Pratyaksha,
    Anumana,
    Upamana,
    Shabda,

    /// Arthāpatti (presumption) - Contextual inference
    /// "If A and not-B would be impossible, and A is true, then B"
    Arthapatti,

    /// Anupalabdhi (non-apprehension) - Absence as evidence
    /// "The absence of X proves not-X"
    Anupalabdhi,
}

impl MimamsaPramana {
    pub fn certainty(&self) -> f32 {
        match self {
            Self::Pratyaksha => 1.00,
            Self::Anumana => 0.95,
            Self::Shabda => 0.90,
            Self::Upamana => 0.85,
            Self::Arthapatti => 0.80,
            Self::Anupalabdhi => 0.75,
        }
    }

    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::Pratyaksha => "प्रत्यक्ष",
            Self::Anumana => "अनुमान",
            Self::Upamana => "उपमान",
            Self::Shabda => "शब्द",
            Self::Arthapatti => "अर्थापत्ति",
            Self::Anupalabdhi => "अनुपलब्धि",
        }
    }
}

/// Mīmāṃsā inference engine
pub struct MimamsaInference {
    /// Base Nyāya inference
    nyaya: NyayaInference,
    /// Contextual rules for arthāpatti
    contextual_rules: Vec<ContextualRule>,
    /// Absence rules for anupalabdhi
    absence_rules: Vec<AbsenceRule>,
}

/// Contextual inference rule
#[derive(Debug, Clone)]
pub struct ContextualRule {
    pub name: String,
    /// Condition that must be true
    pub condition: String,
    /// What would be impossible if conclusion false
    pub impossibility: String,
    /// Conclusion to draw
    pub conclusion: String,
}

/// Absence-based inference rule
#[derive(Debug, Clone)]
pub struct AbsenceRule {
    pub name: String,
    /// What to look for
    pub sought: String,
    /// What absence proves
    pub proves: String,
}

impl MimamsaInference {
    pub fn new() -> Self {
        Self {
            nyaya: NyayaInference::new(),
            contextual_rules: Vec::new(),
            absence_rules: Vec::new(),
        }
    }

    /// Try arthāpatti (presumption)
    /// Example: "x is used in numeric operation" + "x has no type annotation"
    /// → Presume: "x must be numeric type"
    pub fn arthapatti(&self, context: &str) -> Option<TypeEvidence> {
        for rule in &self.contextual_rules {
            if context.contains(&rule.condition) {
                return Some(TypeEvidence {
                    type_name: rule.conclusion.clone(),
                    pramana: Pramana::Upamana, // Closest Nyāya equivalent
                    certainty: 0.80,
                    evidence: vec![
                        format!("Arthāpatti: {}", rule.name),
                        format!("Condition: {}", rule.condition),
                        format!("Conclusion: {}", rule.conclusion),
                    ],
                });
            }
        }
        None
    }

    /// Try anupalabdhi (non-apprehension)
    /// Example: "No error type is declared" → "Function cannot fail"
    pub fn anupalabdhi(&self, expression: &str, available_info: &[&str]) -> Option<TypeEvidence> {
        for rule in &self.absence_rules {
            let sought_present = available_info
                .iter()
                .any(|info| info.contains(&rule.sought));
            if !sought_present {
                return Some(TypeEvidence {
                    type_name: rule.proves.clone(),
                    pramana: Pramana::Upamana,
                    certainty: 0.75,
                    evidence: vec![
                        format!("Anupalabdhi: absence of '{}'", rule.sought),
                        format!("Proves: {}", rule.proves),
                    ],
                });
            }
        }
        None
    }

    /// Add contextual rule
    pub fn add_contextual_rule(&mut self, rule: ContextualRule) {
        self.contextual_rules.push(rule);
    }

    /// Add absence rule
    pub fn add_absence_rule(&mut self, rule: AbsenceRule) {
        self.absence_rules.push(rule);
    }

    /// Full 6-pramāṇa inference
    pub fn infer_6_pramana(
        &self,
        name: &str,
        expression: &str,
        context: &[(&str, &str)],
        docs: &str,
        available_info: &[&str],
    ) -> Option<TypeEvidence> {
        // Try Nyāya 4 pramāṇas first
        if let Some(ty) = self.nyaya.infer(name, expression, context, docs) {
            return Some(ty);
        }

        // 5. Arthāpatti
        if let Some(ty) = self.arthapatti(expression) {
            return Some(ty);
        }

        // 6. Anupalabdhi
        if let Some(ty) = self.anupalabdhi(expression, available_info) {
            return Some(ty);
        }

        None
    }
}

impl Default for MimamsaInference {
    fn default() -> Self {
        Self::new()
    }
}
