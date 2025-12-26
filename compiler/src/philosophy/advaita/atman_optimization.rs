//! Ātman Optimization - Identity-Based Optimization (आत्मन्)
//!
//! "Ātman is Brahman" - the individual self is the universal self
//!
//! Optimizations based on identity:
//! - If two values are the "same" (identical pointers), skip comparisons
//! - If two types are "the same" (same size/layout), allow zero-cost conversion
//! - If two code paths produce "the same" result, merge them

use std::collections::{HashMap, HashSet};

/// Ātman Optimizer - Identity-based optimizations
pub struct AtmanOptimizer {
    /// Identity groups (values that are "the same")
    identity_groups: HashMap<IdentityKey, IdentityGroup>,
    /// Value numbering for expressions
    value_numbers: HashMap<String, u64>,
    /// Next value number
    next_value_num: u64,
    /// Known identities (x == x is always true)
    known_identities: HashSet<String>,
}

/// Key for identity lookup
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IdentityKey {
    /// Variable identity
    Variable(String),
    /// Expression identity (value number)
    Expression(u64),
    /// Type identity
    Type(String),
    /// Memory location identity
    Memory(String),
}

/// Group of identical values
#[derive(Debug, Clone)]
pub struct IdentityGroup {
    /// Representative (canonical) value
    pub representative: String,
    /// All members of the group
    pub members: Vec<String>,
    /// Properties shared by all members
    pub shared_properties: Vec<Property>,
}

/// Shared property of an identity group
#[derive(Debug, Clone)]
pub struct Property {
    pub name: String,
    pub value: PropertyValue,
}

/// Property value types
#[derive(Debug, Clone)]
pub enum PropertyValue {
    Bool(bool),
    Int(i64),
    String(String),
    None,
}

/// Optimization opportunity found by ātman analysis
#[derive(Debug)]
pub enum AtmanOptimization {
    /// Replace comparison with constant (x == x → true)
    IdentityComparison {
        left: String,
        right: String,
        result: bool,
    },
    /// Merge identical expressions
    CommonSubexpression {
        expressions: Vec<String>,
        representative: String,
    },
    /// Remove dead store (value is overwritten by identical)
    DeadStore {
        variable: String,
        location: usize,
    },
    /// Replace expression with known value
    ConstantFolding {
        expression: String,
        value: String,
    },
    /// Skip redundant check (already known to be true/false)
    RedundantCheck {
        condition: String,
        known_result: bool,
    },
}

impl AtmanOptimizer {
    pub fn new() -> Self {
        Self {
            identity_groups: HashMap::new(),
            value_numbers: HashMap::new(),
            next_value_num: 1,
            known_identities: HashSet::new(),
        }
    }

    /// Record that two values are identical (same ātman)
    pub fn record_identity(&mut self, a: String, b: String) {
        // Get or create identity group for a
        let key = IdentityKey::Variable(a.clone());

        let group = self.identity_groups.entry(key.clone()).or_insert_with(|| {
            IdentityGroup {
                representative: a.clone(),
                members: vec![a.clone()],
                shared_properties: vec![],
            }
        });

        if !group.members.contains(&b) {
            group.members.push(b.clone());
        }

        // Also record reverse lookup
        self.identity_groups.insert(
            IdentityKey::Variable(b),
            group.clone(),
        );
    }

    /// Get value number for an expression (for CSE)
    pub fn get_value_number(&mut self, expression: &str) -> u64 {
        if let Some(&num) = self.value_numbers.get(expression) {
            return num;
        }

        let num = self.next_value_num;
        self.next_value_num += 1;
        self.value_numbers.insert(expression.to_string(), num);
        num
    }

    /// Check if two expressions have the same value number
    pub fn same_value(&self, a: &str, b: &str) -> bool {
        match (self.value_numbers.get(a), self.value_numbers.get(b)) {
            (Some(va), Some(vb)) => va == vb,
            _ => false,
        }
    }

    /// Check if two variables are in the same identity group
    pub fn are_identical(&self, a: &str, b: &str) -> bool {
        if a == b {
            return true;
        }

        let key = IdentityKey::Variable(a.to_string());
        if let Some(group) = self.identity_groups.get(&key) {
            return group.members.contains(&b.to_string());
        }

        false
    }

    /// Analyze a comparison and return optimization if possible
    pub fn analyze_comparison(&self, left: &str, op: &str, right: &str) -> Option<AtmanOptimization> {
        // x == x is always true
        // x != x is always false
        if self.are_identical(left, right) {
            return Some(AtmanOptimization::IdentityComparison {
                left: left.to_string(),
                right: right.to_string(),
                result: op == "==" || op == "<=" || op == ">=",
            });
        }

        None
    }

    /// Find common subexpressions in a list of expressions
    pub fn find_common_subexpressions(&mut self, expressions: &[String]) -> Vec<AtmanOptimization> {
        let mut optimizations = Vec::new();
        let mut value_to_exprs: HashMap<u64, Vec<String>> = HashMap::new();

        for expr in expressions {
            let vn = self.get_value_number(expr);
            value_to_exprs.entry(vn).or_default().push(expr.clone());
        }

        for (_, exprs) in value_to_exprs {
            if exprs.len() > 1 {
                optimizations.push(AtmanOptimization::CommonSubexpression {
                    representative: exprs[0].clone(),
                    expressions: exprs,
                });
            }
        }

        optimizations
    }

    /// Analyze an assignment and check for dead stores
    pub fn analyze_assignment(&self, var: &str, value: &str, location: usize) -> Option<AtmanOptimization> {
        // If var already has this value (same identity), it's a dead store
        if self.are_identical(var, value) {
            return Some(AtmanOptimization::DeadStore {
                variable: var.to_string(),
                location,
            });
        }

        None
    }

    /// "Realize ātman is Brahman" - apply all identity optimizations
    pub fn realize_unity(&mut self, code: &CodeBlock) -> Vec<AtmanOptimization> {
        let mut optimizations = Vec::new();

        // Find identity comparisons
        for (i, stmt) in code.statements.iter().enumerate() {
            match stmt {
                Statement::Compare { left, op, right } => {
                    if let Some(opt) = self.analyze_comparison(left, op, right) {
                        optimizations.push(opt);
                    }
                }
                Statement::Assign { var, value } => {
                    if let Some(opt) = self.analyze_assignment(var, value, i) {
                        optimizations.push(opt);
                    }
                    // After assignment, var and value are identical
                    self.record_identity(var.clone(), value.clone());
                }
            }
        }

        // Find common subexpressions
        let expressions: Vec<String> = code.statements.iter().filter_map(|s| {
            match s {
                Statement::Assign { value, .. } => Some(value.clone()),
                _ => None,
            }
        }).collect();

        optimizations.extend(self.find_common_subexpressions(&expressions));

        optimizations
    }

    /// Record a known fact (for redundant check elimination)
    pub fn record_fact(&mut self, condition: &str, result: bool) {
        if result {
            self.known_identities.insert(condition.to_string());
        }
    }

    /// Check if a condition's result is already known
    pub fn check_known(&self, condition: &str) -> Option<bool> {
        if self.known_identities.contains(condition) {
            return Some(true);
        }

        // Check negation
        if condition.starts_with('!') {
            let inner = &condition[1..];
            if self.known_identities.contains(inner) {
                return Some(false);
            }
        }

        None
    }

    /// Clear analysis state (for new function/block)
    pub fn clear(&mut self) {
        self.identity_groups.clear();
        self.value_numbers.clear();
        self.next_value_num = 1;
        self.known_identities.clear();
    }
}

/// Simple code block for analysis
#[derive(Debug)]
pub struct CodeBlock {
    pub statements: Vec<Statement>,
}

/// Statement types
#[derive(Debug)]
pub enum Statement {
    Assign { var: String, value: String },
    Compare { left: String, op: String, right: String },
}

impl Default for AtmanOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_identity_comparison() {
        let optimizer = AtmanOptimizer::new();

        // x == x should be optimized to true
        let opt = optimizer.analyze_comparison("x", "==", "x");
        assert!(matches!(opt, Some(AtmanOptimization::IdentityComparison { result: true, .. })));

        // x != x should be optimized to false
        let opt = optimizer.analyze_comparison("x", "!=", "x");
        assert!(matches!(opt, Some(AtmanOptimization::IdentityComparison { result: false, .. })));
    }

    #[test]
    fn test_identity_groups() {
        let mut optimizer = AtmanOptimizer::new();

        // Record that a and b are identical
        optimizer.record_identity("a".to_string(), "b".to_string());

        assert!(optimizer.are_identical("a", "b"));
        assert!(optimizer.are_identical("b", "a"));
    }

    #[test]
    fn test_common_subexpressions() {
        let mut optimizer = AtmanOptimizer::new();

        let expressions = vec![
            "x + y".to_string(),
            "a * b".to_string(),
            "x + y".to_string(), // Duplicate
        ];

        // Give same value number to duplicate expressions
        optimizer.get_value_number("x + y");
        optimizer.get_value_number("a * b");

        assert!(optimizer.same_value("x + y", "x + y"));
    }
}
