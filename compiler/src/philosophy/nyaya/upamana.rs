//! Upamāna - Comparison/Analogy (उपमान)
//!
//! The fourth pramāṇa: knowledge by comparison/similarity.
//! "This is like that" - pattern matching and structural similarity.
//!
//! In type inference:
//! - If expression A is similar to expression B (whose type is known),
//!   then A likely has the same type as B.

use super::{Pramana, TypeEvidence};
use std::collections::HashMap;

/// Upamāna pattern matcher
pub struct UpamanaEngine {
    /// Known expression patterns and their types
    patterns: Vec<ExpressionPattern>,
    /// Structural similarity rules
    similarity_rules: Vec<SimilarityRule>,
    /// Type families (groups of related types)
    type_families: HashMap<String, TypeFamily>,
}

/// Expression pattern for matching
#[derive(Debug, Clone)]
pub struct ExpressionPattern {
    /// Pattern name
    pub name: String,
    /// Pattern template (with wildcards)
    pub template: PatternTemplate,
    /// Result type
    pub result_type: String,
    /// Example expressions
    pub examples: Vec<String>,
    /// Certainty when matched
    pub certainty: f32,
}

/// Pattern template with wildcards
#[derive(Debug, Clone)]
pub enum PatternTemplate {
    /// Literal pattern
    Literal(String),
    /// Wildcard (matches anything)
    Wildcard,
    /// Typed wildcard (matches specific type)
    TypedWildcard(String),
    /// Sequence pattern
    Sequence(Vec<PatternTemplate>),
    /// Alternative patterns
    Alternative(Vec<PatternTemplate>),
    /// Binary operation pattern
    BinaryOp {
        left: Box<PatternTemplate>,
        op: String,
        right: Box<PatternTemplate>,
    },
    /// Function call pattern
    Call {
        name: Box<PatternTemplate>,
        args: Vec<PatternTemplate>,
    },
    /// Method call pattern
    MethodCall {
        receiver: Box<PatternTemplate>,
        method: String,
        args: Vec<PatternTemplate>,
    },
}

/// Similarity rule between expressions
#[derive(Debug, Clone)]
pub struct SimilarityRule {
    /// Rule name
    pub name: String,
    /// Feature to compare
    pub feature: SimilarityFeature,
    /// Weight (0.0 - 1.0)
    pub weight: f32,
}

/// Features that can indicate similarity
#[derive(Debug, Clone)]
pub enum SimilarityFeature {
    /// Same operator used
    Operator,
    /// Same number of arguments
    Arity,
    /// Same literal type
    LiteralType,
    /// Same structure (AST shape)
    Structure,
    /// Same naming convention
    Naming,
    /// Same return type
    ReturnType,
}

/// Family of related types
#[derive(Debug, Clone)]
pub struct TypeFamily {
    /// Family name
    pub name: String,
    /// Member types
    pub members: Vec<String>,
    /// Common operations
    pub operations: Vec<String>,
}

impl UpamanaEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            patterns: Vec::new(),
            similarity_rules: Vec::new(),
            type_families: HashMap::new(),
        };
        engine.add_builtin_patterns();
        engine.add_builtin_type_families();
        engine
    }

    /// Add built-in patterns
    fn add_builtin_patterns(&mut self) {
        // List literal pattern
        self.patterns.push(ExpressionPattern {
            name: "list_literal".to_string(),
            template: PatternTemplate::Sequence(vec![
                PatternTemplate::Literal("[".to_string()),
                PatternTemplate::Wildcard,
                PatternTemplate::Literal("]".to_string()),
            ]),
            result_type: "Sūci".to_string(), // List type
            examples: vec!["[1, 2, 3]".to_string(), "[\"a\", \"b\"]".to_string()],
            certainty: 0.90,
        });

        // Map literal pattern
        self.patterns.push(ExpressionPattern {
            name: "map_literal".to_string(),
            template: PatternTemplate::Sequence(vec![
                PatternTemplate::Literal("{".to_string()),
                PatternTemplate::Wildcard,
                PatternTemplate::Literal("}".to_string()),
            ]),
            result_type: "Sāraṇī".to_string(), // Map type
            examples: vec!["{\"key\": \"value\"}".to_string()],
            certainty: 0.85,
        });

        // Range pattern
        self.patterns.push(ExpressionPattern {
            name: "range".to_string(),
            template: PatternTemplate::BinaryOp {
                left: Box::new(PatternTemplate::TypedWildcard("saṅkhyā".to_string())),
                op: "..".to_string(),
                right: Box::new(PatternTemplate::TypedWildcard("saṅkhyā".to_string())),
            },
            result_type: "Śreṇī".to_string(), // Range type
            examples: vec!["0..10".to_string(), "1..=100".to_string()],
            certainty: 0.90,
        });

        // Iterator method pattern
        self.patterns.push(ExpressionPattern {
            name: "iterator".to_string(),
            template: PatternTemplate::MethodCall {
                receiver: Box::new(PatternTemplate::Wildcard),
                method: "cala".to_string(), // iterate
                args: vec![],
            },
            result_type: "Calana".to_string(), // Iterator type
            examples: vec!["sūci.cala()".to_string()],
            certainty: 0.85,
        });
    }

    /// Add built-in type families
    fn add_builtin_type_families(&mut self) {
        // Numeric family
        self.type_families.insert("saṅkhyā".to_string(), TypeFamily {
            name: "Saṅkhyā (Numeric)".to_string(),
            members: vec![
                "t8".to_string(), "t16".to_string(), "t32".to_string(), "t64".to_string(),
                "u8".to_string(), "u16".to_string(), "u32".to_string(), "u64".to_string(),
                "d32".to_string(), "d64".to_string(),
            ],
            operations: vec!["+".to_string(), "-".to_string(), "*".to_string(), "/".to_string()],
        });

        // Collection family
        self.type_families.insert("saṅgraha".to_string(), TypeFamily {
            name: "Saṅgraha (Collection)".to_string(),
            members: vec![
                "Sūci".to_string(),   // List
                "Sāraṇī".to_string(), // Map
                "Gaṇa".to_string(),   // Set
            ],
            operations: vec!["dairghya".to_string(), "cala".to_string()], // length, iterate
        });
    }

    /// Find matching pattern using upamāna
    pub fn find_similar(&self, expression: &str) -> Option<TypeEvidence> {
        for pattern in &self.patterns {
            let similarity = self.compute_similarity(expression, pattern);
            if similarity >= 0.7 { // 70% threshold
                return Some(TypeEvidence {
                    type_name: pattern.result_type.clone(),
                    pramana: Pramana::Upamana,
                    certainty: pattern.certainty * similarity,
                    evidence: vec![
                        format!("उपमान (comparison): Similar to pattern '{}'", pattern.name),
                        format!("Matched examples: {:?}", pattern.examples),
                        format!("Similarity score: {:.0}%", similarity * 100.0),
                    ],
                });
            }
        }
        None
    }

    /// Compute similarity between expression and pattern
    fn compute_similarity(&self, expression: &str, pattern: &ExpressionPattern) -> f32 {
        let mut total_score = 0.0;
        let mut count = 0;

        // Compare with template
        let template_score = self.match_template(expression, &pattern.template);
        total_score += template_score;
        count += 1;

        // Compare with examples
        for example in &pattern.examples {
            let example_score = self.string_similarity(expression, example);
            total_score += example_score;
            count += 1;
        }

        total_score / count as f32
    }

    /// Match expression against template
    fn match_template(&self, expression: &str, template: &PatternTemplate) -> f32 {
        match template {
            PatternTemplate::Literal(lit) => {
                if expression.contains(lit) { 1.0 } else { 0.0 }
            }
            PatternTemplate::Wildcard => 0.5, // Partial match
            PatternTemplate::TypedWildcard(_) => 0.5, // Partial match
            PatternTemplate::Sequence(parts) => {
                let mut score = 0.0;
                for part in parts {
                    score += self.match_template(expression, part);
                }
                score / parts.len() as f32
            }
            PatternTemplate::Alternative(alts) => {
                alts.iter()
                    .map(|alt| self.match_template(expression, alt))
                    .max_by(|a, b| a.partial_cmp(b).unwrap())
                    .unwrap_or(0.0)
            }
            PatternTemplate::BinaryOp { left, op, right } => {
                if expression.contains(op) {
                    let parts: Vec<&str> = expression.split(op.as_str()).collect();
                    if parts.len() == 2 {
                        let left_score = self.match_template(parts[0], left);
                        let right_score = self.match_template(parts[1], right);
                        return (left_score + right_score + 1.0) / 3.0;
                    }
                }
                0.0
            }
            PatternTemplate::Call { name: _name, args: _args } => {
                // Check for function call syntax
                if expression.contains('(') && expression.contains(')') {
                    0.7 // Basic match
                } else {
                    0.0
                }
            }
            PatternTemplate::MethodCall { receiver: _receiver, method, args: _args } => {
                if expression.contains('.') && expression.contains(method) {
                    0.8
                } else {
                    0.0
                }
            }
        }
    }

    /// Compute string similarity (Levenshtein-based)
    fn string_similarity(&self, a: &str, b: &str) -> f32 {
        if a == b {
            return 1.0;
        }
        if a.is_empty() || b.is_empty() {
            return 0.0;
        }

        let max_len = a.len().max(b.len());
        let distance = self.levenshtein_distance(a, b);

        1.0 - (distance as f32 / max_len as f32)
    }

    /// Levenshtein distance for similarity computation
    fn levenshtein_distance(&self, a: &str, b: &str) -> usize {
        let a_chars: Vec<char> = a.chars().collect();
        let b_chars: Vec<char> = b.chars().collect();
        let len_a = a_chars.len();
        let len_b = b_chars.len();

        let mut matrix = vec![vec![0; len_b + 1]; len_a + 1];

        for i in 0..=len_a {
            matrix[i][0] = i;
        }
        for j in 0..=len_b {
            matrix[0][j] = j;
        }

        for i in 1..=len_a {
            for j in 1..=len_b {
                let cost = if a_chars[i - 1] == b_chars[j - 1] { 0 } else { 1 };
                matrix[i][j] = (matrix[i - 1][j] + 1)
                    .min(matrix[i][j - 1] + 1)
                    .min(matrix[i - 1][j - 1] + cost);
            }
        }

        matrix[len_a][len_b]
    }

    /// Infer type from similar expressions
    pub fn infer_from_context(&self, expression: &str, known_types: &HashMap<String, String>) -> Option<TypeEvidence> {
        let mut best_match: Option<(f32, TypeEvidence)> = None;

        for (known_expr, known_type) in known_types {
            let similarity = self.string_similarity(expression, known_expr);
            if similarity >= 0.6 {
                let evidence = TypeEvidence {
                    type_name: known_type.clone(),
                    pramana: Pramana::Upamana,
                    certainty: 0.85 * similarity,
                    evidence: vec![
                        format!("उपमान: Similar to known expression '{}'", known_expr),
                        format!("Known type: {}", known_type),
                        format!("Similarity: {:.0}%", similarity * 100.0),
                    ],
                };

                match &best_match {
                    Some((score, _)) if similarity > *score => {
                        best_match = Some((similarity, evidence));
                    }
                    None => {
                        best_match = Some((similarity, evidence));
                    }
                    _ => {}
                }
            }
        }

        best_match.map(|(_, e)| e)
    }

    /// Add custom pattern
    pub fn add_pattern(&mut self, pattern: ExpressionPattern) {
        self.patterns.push(pattern);
    }
}

impl Default for UpamanaEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_pattern() {
        let engine = UpamanaEngine::new();

        let result = engine.find_similar("[1, 2, 3]");
        assert!(result.is_some());
        let evidence = result.unwrap();
        assert_eq!(evidence.type_name, "Sūci");
        assert_eq!(evidence.pramana, Pramana::Upamana);
    }

    #[test]
    fn test_range_pattern() {
        let engine = UpamanaEngine::new();

        let result = engine.find_similar("0..10");
        assert!(result.is_some());
        let evidence = result.unwrap();
        assert_eq!(evidence.type_name, "Śreṇī");
    }

    #[test]
    fn test_string_similarity() {
        let engine = UpamanaEngine::new();

        assert!((engine.string_similarity("hello", "hello") - 1.0).abs() < 0.01);
        assert!((engine.string_similarity("hello", "hallo") - 0.8).abs() < 0.1);
    }
}
