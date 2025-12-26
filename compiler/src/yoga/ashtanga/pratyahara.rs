//! Pratyāhāra - Withdrawal/Encapsulation (प्रत्याहार)
//!
//! The fifth limb of Ashtanga Yoga - Withdrawal of the senses.
//! In software, this means information hiding and encapsulation:
//!
//! - Private by default
//! - Minimal public API surface
//! - Implementation details hidden
//! - Controlled access to internals

use std::collections::{HashMap, HashSet};

/// Visibility levels (degree of withdrawal)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Visibility {
    /// Private - fully withdrawn (only self)
    Private,
    /// Crate-internal - visible within crate
    Internal,
    /// Protected - visible to submodules
    Protected,
    /// Public - exposed to the world
    Public,
}

impl Visibility {
    /// Sanskrit mapping
    pub fn sanskrit(&self) -> &'static str {
        match self {
            Self::Private => "आत्मन् (Ātman) - Inner self only",
            Self::Internal => "कुल (Kula) - Family/crate",
            Self::Protected => "गोत्र (Gotra) - Lineage/hierarchy",
            Self::Public => "लोक (Loka) - World/public",
        }
    }

    /// Is this more exposed than another?
    pub fn is_more_exposed(&self, other: &Self) -> bool {
        *self > *other
    }
}

/// Symbol with visibility
#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub visibility: Visibility,
    pub kind: SymbolKind,
    pub location: String,
    pub accessed_by: HashSet<String>,
}

/// Kinds of symbols
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SymbolKind {
    Function,
    Type,
    Constant,
    Variable,
    Module,
    Trait,
    Field,
    Method,
}

/// Pratyāhāra analyzer - encapsulation checker
pub struct PratyaharaAnalyzer {
    /// All symbols
    symbols: HashMap<String, Symbol>,
    /// Violations
    violations: Vec<PratyaharaViolation>,
    /// Configuration
    config: PratyaharaConfig,
}

/// Configuration for encapsulation checks
#[derive(Debug, Clone)]
pub struct PratyaharaConfig {
    /// Default visibility for new symbols
    pub default_visibility: Visibility,
    /// Maximum public API surface (as percentage)
    pub max_public_percentage: f64,
    /// Warn on unused public symbols?
    pub warn_unused_public: bool,
    /// Minimum encapsulation score
    pub min_encapsulation_score: f64,
}

impl Default for PratyaharaConfig {
    fn default() -> Self {
        Self {
            default_visibility: Visibility::Private,
            max_public_percentage: 20.0,
            warn_unused_public: true,
            min_encapsulation_score: 0.7,
        }
    }
}

/// Encapsulation violation
#[derive(Debug, Clone)]
pub struct PratyaharaViolation {
    pub kind: ViolationKind,
    pub symbol: String,
    pub description: String,
    pub suggestion: String,
}

/// Types of encapsulation violations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationKind {
    /// Public API too large
    ExcessivePublicApi,
    /// Internal detail exposed
    LeakedImplementation,
    /// Unused public symbol
    UnusedPublic,
    /// Field directly exposed (should use getter)
    ExposedField,
    /// Dependency on implementation detail
    DependsOnInternal,
}

impl PratyaharaAnalyzer {
    pub fn new(config: PratyaharaConfig) -> Self {
        Self {
            symbols: HashMap::new(),
            violations: Vec::new(),
            config,
        }
    }

    /// Add a symbol for analysis
    pub fn add_symbol(&mut self, symbol: Symbol) {
        self.symbols.insert(symbol.name.clone(), symbol);
    }

    /// Record an access to a symbol
    pub fn record_access(&mut self, symbol_name: &str, accessor: &str) {
        if let Some(symbol) = self.symbols.get_mut(symbol_name) {
            symbol.accessed_by.insert(accessor.to_string());
        }
    }

    /// Check public API surface
    pub fn check_api_surface(&mut self, module: &str) {
        let module_symbols: Vec<_> = self.symbols.values()
            .filter(|s| s.location.starts_with(module))
            .collect();

        let total = module_symbols.len();
        let public_count = module_symbols.iter()
            .filter(|s| s.visibility == Visibility::Public)
            .count();

        if total > 0 {
            let percentage = (public_count as f64 / total as f64) * 100.0;
            if percentage > self.config.max_public_percentage {
                self.violations.push(PratyaharaViolation {
                    kind: ViolationKind::ExcessivePublicApi,
                    symbol: module.to_string(),
                    description: format!(
                        "Public API is {:.1}% (max: {:.1}%)",
                        percentage, self.config.max_public_percentage
                    ),
                    suggestion: "Reduce public surface, use internal APIs".to_string(),
                });
            }
        }
    }

    /// Check for unused public symbols
    pub fn check_unused_public(&mut self) {
        if !self.config.warn_unused_public {
            return;
        }

        for symbol in self.symbols.values() {
            if symbol.visibility == Visibility::Public && symbol.accessed_by.is_empty() {
                self.violations.push(PratyaharaViolation {
                    kind: ViolationKind::UnusedPublic,
                    symbol: symbol.name.clone(),
                    description: "Public symbol is never accessed externally".to_string(),
                    suggestion: "Consider making private or internal".to_string(),
                });
            }
        }
    }

    /// Check for exposed fields
    pub fn check_exposed_fields(&mut self) {
        for symbol in self.symbols.values() {
            if symbol.kind == SymbolKind::Field && symbol.visibility == Visibility::Public {
                self.violations.push(PratyaharaViolation {
                    kind: ViolationKind::ExposedField,
                    symbol: symbol.name.clone(),
                    description: "Field is directly exposed".to_string(),
                    suggestion: "Use getter/setter methods for encapsulation".to_string(),
                });
            }
        }
    }

    /// Check for leaked implementation details
    pub fn check_leaked_implementation(&mut self, impl_patterns: &[&str]) {
        for symbol in self.symbols.values() {
            if symbol.visibility == Visibility::Public {
                for pattern in impl_patterns {
                    if symbol.name.contains(pattern) {
                        self.violations.push(PratyaharaViolation {
                            kind: ViolationKind::LeakedImplementation,
                            symbol: symbol.name.clone(),
                            description: format!("Implementation detail '{}' in public API", pattern),
                            suggestion: "Rename to hide implementation or make internal".to_string(),
                        });
                    }
                }
            }
        }
    }

    /// Run all checks
    pub fn analyze(&mut self) {
        self.check_unused_public();
        self.check_exposed_fields();
        self.check_leaked_implementation(&["impl", "internal", "_inner", "_raw"]);
    }

    /// Calculate encapsulation score (0.0 to 1.0)
    pub fn encapsulation_score(&self) -> f64 {
        if self.symbols.is_empty() {
            return 1.0;
        }

        let total = self.symbols.len() as f64;
        let private = self.symbols.values()
            .filter(|s| s.visibility == Visibility::Private)
            .count() as f64;

        private / total
    }

    /// Get violations
    pub fn violations(&self) -> &[PratyaharaViolation] {
        &self.violations
    }

    /// Get API surface statistics
    pub fn api_stats(&self) -> ApiStats {
        let mut stats = ApiStats::default();

        for symbol in self.symbols.values() {
            match symbol.visibility {
                Visibility::Private => stats.private += 1,
                Visibility::Internal => stats.internal += 1,
                Visibility::Protected => stats.protected += 1,
                Visibility::Public => stats.public += 1,
            }
        }

        stats.encapsulation_score = self.encapsulation_score();
        stats
    }

    /// Generate visibility report
    pub fn report(&self) -> String {
        let stats = self.api_stats();
        let mut report = String::new();

        report.push_str("=== Pratyāhāra Report ===\n\n");
        report.push_str(&format!("Encapsulation Score: {:.1}%\n\n", stats.encapsulation_score * 100.0));

        report.push_str("Visibility Distribution:\n");
        report.push_str(&format!("  Private:   {} (आत्मन्)\n", stats.private));
        report.push_str(&format!("  Internal:  {} (कुल)\n", stats.internal));
        report.push_str(&format!("  Protected: {} (गोत्र)\n", stats.protected));
        report.push_str(&format!("  Public:    {} (लोक)\n\n", stats.public));

        if !self.violations.is_empty() {
            report.push_str("Violations:\n");
            for v in &self.violations {
                report.push_str(&format!("  - {} ({})\n", v.symbol, v.description));
            }
        }

        report
    }

    /// Clear for reuse
    pub fn clear(&mut self) {
        self.symbols.clear();
        self.violations.clear();
    }
}

/// API surface statistics
#[derive(Debug, Default)]
pub struct ApiStats {
    pub private: usize,
    pub internal: usize,
    pub protected: usize,
    pub public: usize,
    pub encapsulation_score: f64,
}

impl Default for PratyaharaAnalyzer {
    fn default() -> Self {
        Self::new(PratyaharaConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encapsulation_score() {
        let mut analyzer = PratyaharaAnalyzer::default();

        // 3 private, 1 public = 75% encapsulated
        for i in 0..3 {
            analyzer.add_symbol(Symbol {
                name: format!("private_{}", i),
                visibility: Visibility::Private,
                kind: SymbolKind::Function,
                location: "test.rs".to_string(),
                accessed_by: HashSet::new(),
            });
        }

        analyzer.add_symbol(Symbol {
            name: "public_fn".to_string(),
            visibility: Visibility::Public,
            kind: SymbolKind::Function,
            location: "test.rs".to_string(),
            accessed_by: HashSet::new(),
        });

        let score = analyzer.encapsulation_score();
        assert!((score - 0.75).abs() < 0.01);
    }

    #[test]
    fn test_unused_public() {
        let mut analyzer = PratyaharaAnalyzer::default();

        analyzer.add_symbol(Symbol {
            name: "unused_public".to_string(),
            visibility: Visibility::Public,
            kind: SymbolKind::Function,
            location: "test.rs".to_string(),
            accessed_by: HashSet::new(), // Never accessed
        });

        analyzer.check_unused_public();
        assert!(analyzer.violations.iter().any(|v| v.kind == ViolationKind::UnusedPublic));
    }
}
