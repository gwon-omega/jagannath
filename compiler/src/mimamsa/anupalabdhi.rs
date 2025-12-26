//! Anupalabdhi - Non-Apprehension/Negative Proof (अनुपलब्धि)
//!
//! The 6th pramāṇa in Mīmāṃsā - Knowledge from absence.
//! "The absence of X in a situation where X would be perceivable if present
//! proves that X is not present."
//!
//! In type inference:
//! - Absence of error handling → function cannot fail
//! - Absence of mutable operations → variable is immutable
//! - Absence of generic bounds → concrete type expected

use std::collections::{HashMap, HashSet};

/// Anupalabdhi inference engine
pub struct AnupalabdhiEngine {
    /// Absence rules
    rules: Vec<AbsenceRule>,
    /// What we've observed (to check for absence)
    observations: HashSet<String>,
    /// Scope observations
    scoped_observations: HashMap<String, HashSet<String>>,
}

/// Absence-based inference rule
#[derive(Debug, Clone)]
pub struct AbsenceRule {
    /// Rule name
    pub name: String,
    /// What to look for (if absent, conclusion follows)
    pub sought_feature: SoughtFeature,
    /// Where it should be if present
    pub expected_location: Location,
    /// What absence proves
    pub proves_on_absence: AbsenceConclusion,
    /// Certainty
    pub certainty: f32,
}

/// Features to seek
#[derive(Debug, Clone)]
pub enum SoughtFeature {
    /// Error type declaration
    ErrorType,
    /// Mutable modifier
    MutableModifier,
    /// Lifetime annotation
    LifetimeAnnotation,
    /// Generic bound
    GenericBound,
    /// Return statement
    ReturnStatement,
    /// Side effect
    SideEffect,
    /// Null/None check
    NullCheck,
    /// Lock acquisition
    LockAcquisition,
    /// Custom feature
    Custom(String),
}

/// Where to look
#[derive(Debug, Clone)]
pub enum Location {
    /// Function signature
    FunctionSignature(String),
    /// Type definition
    TypeDefinition(String),
    /// Variable declaration
    VariableDeclaration(String),
    /// Block scope
    BlockScope(String),
    /// Entire module
    Module(String),
    /// Custom location
    Custom(String),
}

/// Conclusion from absence
#[derive(Debug, Clone)]
pub enum AbsenceConclusion {
    /// Type is infallible
    Infallible,
    /// Value is immutable
    Immutable,
    /// Has no lifetime constraints
    StaticLifetime,
    /// Is concrete (not generic)
    ConcreteType,
    /// Has no side effects (pure)
    Pure,
    /// Cannot be null
    NonNull,
    /// Is thread-safe
    ThreadSafe,
    /// Custom conclusion
    Custom(String),
}

/// Absence proof result
#[derive(Debug, Clone)]
pub struct AbsenceProof {
    pub conclusion: AbsenceConclusion,
    pub certainty: f32,
    pub via_rule: String,
    pub reasoning: Vec<String>,
}

impl AnupalabdhiEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            rules: Vec::new(),
            observations: HashSet::new(),
            scoped_observations: HashMap::new(),
        };

        engine.add_default_rules();
        engine
    }

    fn add_default_rules(&mut self) {
        // No error type → infallible
        self.add_rule(AbsenceRule {
            name: "no_error_means_infallible".to_string(),
            sought_feature: SoughtFeature::ErrorType,
            expected_location: Location::FunctionSignature("return".to_string()),
            proves_on_absence: AbsenceConclusion::Infallible,
            certainty: 0.85,
        });

        // No mut → immutable
        self.add_rule(AbsenceRule {
            name: "no_mut_means_immutable".to_string(),
            sought_feature: SoughtFeature::MutableModifier,
            expected_location: Location::VariableDeclaration("binding".to_string()),
            proves_on_absence: AbsenceConclusion::Immutable,
            certainty: 0.95,
        });

        // No lifetime → static
        self.add_rule(AbsenceRule {
            name: "no_lifetime_means_static".to_string(),
            sought_feature: SoughtFeature::LifetimeAnnotation,
            expected_location: Location::TypeDefinition("struct".to_string()),
            proves_on_absence: AbsenceConclusion::StaticLifetime,
            certainty: 0.75,
        });

        // No side effects → pure
        self.add_rule(AbsenceRule {
            name: "no_side_effects_means_pure".to_string(),
            sought_feature: SoughtFeature::SideEffect,
            expected_location: Location::BlockScope("function_body".to_string()),
            proves_on_absence: AbsenceConclusion::Pure,
            certainty: 0.80,
        });

        // No null check needed → non-null
        self.add_rule(AbsenceRule {
            name: "no_null_check_means_nonnull".to_string(),
            sought_feature: SoughtFeature::NullCheck,
            expected_location: Location::BlockScope("dereference".to_string()),
            proves_on_absence: AbsenceConclusion::NonNull,
            certainty: 0.70,
        });

        // No locks → thread-safe by default
        self.add_rule(AbsenceRule {
            name: "no_locks_means_threadsafe".to_string(),
            sought_feature: SoughtFeature::LockAcquisition,
            expected_location: Location::Module("concurrent".to_string()),
            proves_on_absence: AbsenceConclusion::ThreadSafe,
            certainty: 0.65,
        });
    }

    /// Add custom rule
    pub fn add_rule(&mut self, rule: AbsenceRule) {
        self.rules.push(rule);
    }

    /// Record an observation
    pub fn observe(&mut self, feature: &str) {
        self.observations.insert(feature.to_string());
    }

    /// Record scoped observation
    pub fn observe_in_scope(&mut self, scope: &str, feature: &str) {
        self.scoped_observations
            .entry(scope.to_string())
            .or_default()
            .insert(feature.to_string());
    }

    /// Clear observations
    pub fn clear_observations(&mut self) {
        self.observations.clear();
        self.scoped_observations.clear();
    }

    /// Check if feature is absent
    pub fn is_absent(&self, feature: &SoughtFeature) -> bool {
        let feature_str = match feature {
            SoughtFeature::ErrorType => "error_type",
            SoughtFeature::MutableModifier => "mut",
            SoughtFeature::LifetimeAnnotation => "lifetime",
            SoughtFeature::GenericBound => "generic_bound",
            SoughtFeature::ReturnStatement => "return",
            SoughtFeature::SideEffect => "side_effect",
            SoughtFeature::NullCheck => "null_check",
            SoughtFeature::LockAcquisition => "lock",
            SoughtFeature::Custom(s) => s.as_str(),
        };

        !self.observations.contains(feature_str)
    }

    /// Check absence in scope
    pub fn is_absent_in_scope(&self, scope: &str, feature: &SoughtFeature) -> bool {
        let feature_str = match feature {
            SoughtFeature::ErrorType => "error_type",
            SoughtFeature::MutableModifier => "mut",
            SoughtFeature::LifetimeAnnotation => "lifetime",
            SoughtFeature::GenericBound => "generic_bound",
            SoughtFeature::ReturnStatement => "return",
            SoughtFeature::SideEffect => "side_effect",
            SoughtFeature::NullCheck => "null_check",
            SoughtFeature::LockAcquisition => "lock",
            SoughtFeature::Custom(s) => s.as_str(),
        };

        self.scoped_observations
            .get(scope)
            .map(|obs| !obs.contains(feature_str))
            .unwrap_or(true) // Absent if scope doesn't exist
    }

    /// Prove by absence
    pub fn prove_by_absence(&self, feature: &SoughtFeature) -> Option<AbsenceProof> {
        if !self.is_absent(feature) {
            return None; // Feature is present, no absence proof
        }

        for rule in &self.rules {
            if std::mem::discriminant(&rule.sought_feature) == std::mem::discriminant(feature) {
                return Some(AbsenceProof {
                    conclusion: rule.proves_on_absence.clone(),
                    certainty: rule.certainty,
                    via_rule: rule.name.clone(),
                    reasoning: vec![
                        format!("Sought: {:?}", feature),
                        format!("Expected at: {:?}", rule.expected_location),
                        "Feature is absent".to_string(),
                        format!("Conclusion: {:?}", rule.proves_on_absence),
                    ],
                });
            }
        }

        None
    }

    /// Prove by absence in scope
    pub fn prove_in_scope(&self, scope: &str, feature: &SoughtFeature) -> Option<AbsenceProof> {
        if !self.is_absent_in_scope(scope, feature) {
            return None;
        }

        for rule in &self.rules {
            if std::mem::discriminant(&rule.sought_feature) == std::mem::discriminant(feature) {
                return Some(AbsenceProof {
                    conclusion: rule.proves_on_absence.clone(),
                    certainty: rule.certainty * 0.9, // Slightly less certain for scoped
                    via_rule: rule.name.clone(),
                    reasoning: vec![
                        format!("Sought: {:?} in scope '{}'", feature, scope),
                        "Feature is absent in this scope".to_string(),
                        format!("Conclusion: {:?}", rule.proves_on_absence),
                    ],
                });
            }
        }

        None
    }

    /// Run all absence checks
    pub fn analyze(&self) -> Vec<AbsenceProof> {
        let mut proofs = Vec::new();

        let features = vec![
            SoughtFeature::ErrorType,
            SoughtFeature::MutableModifier,
            SoughtFeature::LifetimeAnnotation,
            SoughtFeature::SideEffect,
            SoughtFeature::LockAcquisition,
        ];

        for feature in features {
            if let Some(proof) = self.prove_by_absence(&feature) {
                proofs.push(proof);
            }
        }

        proofs
    }
}

impl Default for AnupalabdhiEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_immutability_from_absence() {
        let engine = AnupalabdhiEngine::new();

        // No 'mut' observed
        let proof = engine.prove_by_absence(&SoughtFeature::MutableModifier);
        assert!(proof.is_some());

        let p = proof.unwrap();
        assert!(matches!(p.conclusion, AbsenceConclusion::Immutable));
    }

    #[test]
    fn test_presence_prevents_proof() {
        let mut engine = AnupalabdhiEngine::new();

        // Observe 'mut'
        engine.observe("mut");

        // Should not prove immutable
        let proof = engine.prove_by_absence(&SoughtFeature::MutableModifier);
        assert!(proof.is_none());
    }

    #[test]
    fn test_infallible_from_no_error() {
        let engine = AnupalabdhiEngine::new();

        let proof = engine.prove_by_absence(&SoughtFeature::ErrorType);
        assert!(proof.is_some());

        let p = proof.unwrap();
        assert!(matches!(p.conclusion, AbsenceConclusion::Infallible));
    }
}
