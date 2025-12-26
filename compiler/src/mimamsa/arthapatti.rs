//! Arthāpatti - Presumption/Contextual Inference (अर्थापत्ति)
//!
//! The 5th pramāṇa in Mīmāṃsā - Presumption based on impossibility.
//! "If A is true and B being false would make A impossible, then B must be true."
//!
//! In type inference:
//! - If expression is used in numeric context, and no type → presume numeric
//! - If function is called with argument X, and no param type → presume compatible type

use std::collections::HashMap;

/// Arthāpatti inference engine
pub struct ArthapattEngine {
    /// Contextual rules
    rules: Vec<ArthapattRule>,
    /// Context stack
    context_stack: Vec<ContextFrame>,
    /// Cached inferences
    cache: HashMap<String, InferredType>,
}

/// Contextual inference rule
#[derive(Debug, Clone)]
pub struct ArthapattRule {
    /// Rule name
    pub name: String,
    /// The observed fact (A)
    pub observed: Observed,
    /// What would be impossible if conclusion is false
    pub impossibility: Impossibility,
    /// The conclusion (B)
    pub conclusion: Conclusion,
    /// Certainty of this inference
    pub certainty: f32,
}

/// Observed fact types
#[derive(Debug, Clone)]
pub enum Observed {
    /// Expression used in operation
    UsedInOperation(String),
    /// Variable passed to function
    PassedToFunction(String, usize),
    /// Value compared with
    ComparedWith(String),
    /// Assigned from expression
    AssignedFrom(String),
    /// Custom observation
    Custom(String),
}

/// Impossibility types
#[derive(Debug, Clone)]
pub enum Impossibility {
    /// Type mismatch would occur
    TypeMismatch(String, String),
    /// Operation would be undefined
    UndefinedOperation(String),
    /// Trait bound would be violated
    MissingTrait(String),
    /// Custom impossibility
    Custom(String),
}

/// Conclusion types
#[derive(Debug, Clone)]
pub enum Conclusion {
    /// Has this type
    HasType(String),
    /// Implements trait
    ImplementsTrait(String),
    /// Has capability
    HasCapability(String),
    /// Custom conclusion
    Custom(String),
}

/// Context frame for scoped inference
#[derive(Debug, Clone)]
pub struct ContextFrame {
    pub scope: String,
    pub known_types: HashMap<String, String>,
    pub known_traits: HashMap<String, Vec<String>>,
}

/// Inferred type result
#[derive(Debug, Clone)]
pub struct InferredType {
    pub type_name: String,
    pub certainty: f32,
    pub via_rule: String,
    pub reasoning: Vec<String>,
}

impl ArthapattEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            rules: Vec::new(),
            context_stack: Vec::new(),
            cache: HashMap::new(),
        };

        // Add default rules
        engine.add_default_rules();
        engine
    }

    fn add_default_rules(&mut self) {
        // Arithmetic operation implies numeric type
        self.add_rule(ArthapattRule {
            name: "arithmetic_implies_numeric".to_string(),
            observed: Observed::UsedInOperation("+|-|*|/|%".to_string()),
            impossibility: Impossibility::UndefinedOperation("arithmetic on non-numeric".to_string()),
            conclusion: Conclusion::HasType("Numeric".to_string()),
            certainty: 0.85,
        });

        // Comparison implies Ord trait
        self.add_rule(ArthapattRule {
            name: "comparison_implies_ord".to_string(),
            observed: Observed::UsedInOperation("<|>|<=|>=".to_string()),
            impossibility: Impossibility::MissingTrait("Ord".to_string()),
            conclusion: Conclusion::ImplementsTrait("Ord".to_string()),
            certainty: 0.90,
        });

        // Index access implies array/slice
        self.add_rule(ArthapattRule {
            name: "index_implies_indexable".to_string(),
            observed: Observed::UsedInOperation("[]".to_string()),
            impossibility: Impossibility::MissingTrait("Index".to_string()),
            conclusion: Conclusion::ImplementsTrait("Index".to_string()),
            certainty: 0.88,
        });

        // String concatenation
        self.add_rule(ArthapattRule {
            name: "concat_implies_string".to_string(),
            observed: Observed::UsedInOperation("++".to_string()),
            impossibility: Impossibility::TypeMismatch("String".to_string(), "unknown".to_string()),
            conclusion: Conclusion::HasType("String".to_string()),
            certainty: 0.80,
        });
    }

    /// Add a custom rule
    pub fn add_rule(&mut self, rule: ArthapattRule) {
        self.rules.push(rule);
    }

    /// Push a context frame
    pub fn push_context(&mut self, frame: ContextFrame) {
        self.context_stack.push(frame);
    }

    /// Pop context frame
    pub fn pop_context(&mut self) -> Option<ContextFrame> {
        self.context_stack.pop()
    }

    /// Infer type from operation context
    pub fn infer_from_operation(&self, expr: &str, operation: &str) -> Option<InferredType> {
        for rule in &self.rules {
            if let Observed::UsedInOperation(ops) = &rule.observed {
                if ops.split('|').any(|op| operation.contains(op)) {
                    let type_name = match &rule.conclusion {
                        Conclusion::HasType(t) => t.clone(),
                        Conclusion::ImplementsTrait(t) => format!("impl {}", t),
                        _ => continue,
                    };

                    return Some(InferredType {
                        type_name,
                        certainty: rule.certainty,
                        via_rule: rule.name.clone(),
                        reasoning: vec![
                            format!("Observed: {} used in '{}'", expr, operation),
                            format!("Impossibility: {:?}", rule.impossibility),
                            format!("Conclusion: {:?}", rule.conclusion),
                        ],
                    });
                }
            }
        }
        None
    }

    /// Infer type from function call context
    pub fn infer_from_call(&self, arg_expr: &str, function: &str, param_idx: usize) -> Option<InferredType> {
        for rule in &self.rules {
            if let Observed::PassedToFunction(func_pattern, idx) = &rule.observed {
                if function.contains(func_pattern) && param_idx == *idx {
                    let type_name = match &rule.conclusion {
                        Conclusion::HasType(t) => t.clone(),
                        _ => continue,
                    };

                    return Some(InferredType {
                        type_name,
                        certainty: rule.certainty,
                        via_rule: rule.name.clone(),
                        reasoning: vec![
                            format!("Passed {} to {}() at position {}", arg_expr, function, param_idx),
                        ],
                    });
                }
            }
        }
        None
    }

    /// Infer type from comparison context
    pub fn infer_from_comparison(&self, expr: &str, compared_with: &str, known_type: &str) -> Option<InferredType> {
        // If compared with a known type, presume same type
        Some(InferredType {
            type_name: known_type.to_string(),
            certainty: 0.82,
            via_rule: "comparison_type_propagation".to_string(),
            reasoning: vec![
                format!("'{}' compared with '{}' (known: {})", expr, compared_with, known_type),
                "Comparison requires compatible types".to_string(),
            ],
        })
    }

    /// Clear cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }
}

impl Default for ArthapattEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_inference() {
        let engine = ArthapattEngine::new();

        let result = engine.infer_from_operation("x", "+");
        assert!(result.is_some());

        let inferred = result.unwrap();
        assert_eq!(inferred.type_name, "Numeric");
    }

    #[test]
    fn test_comparison_inference() {
        let engine = ArthapattEngine::new();

        let result = engine.infer_from_operation("x", "<");
        assert!(result.is_some());

        let inferred = result.unwrap();
        assert!(inferred.type_name.contains("Ord"));
    }
}
