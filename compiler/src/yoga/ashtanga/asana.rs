//! Āsana - Posture/Architecture (आसन)
//!
//! The third limb of Ashtanga Yoga - Stable foundation.
//! Āsana provides the structural stability for software:
//!
//! - Stable foundation for the system
//! - Proper module organization
//! - Clean dependency management
//! - Layered architecture

use std::collections::{HashMap, HashSet};

/// Architecture layer (like body positions in āsana)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Layer {
    /// Foundation layer - Core types, utilities
    Foundation,
    /// Domain layer - Business logic
    Domain,
    /// Application layer - Use cases
    Application,
    /// Interface layer - APIs, UI
    Interface,
    /// Infrastructure layer - External systems
    Infrastructure,
}

impl Layer {
    /// Can this layer depend on another?
    pub fn can_depend_on(&self, other: &Layer) -> bool {
        // Dependencies should flow inward (higher → lower)
        *self >= *other
    }

    /// Sanskrit name (āsana position metaphor)
    pub fn asana_name(&self) -> &'static str {
        match self {
            Self::Foundation => "Padma (Lotus) - Root stability",
            Self::Domain => "Vīra (Hero) - Core strength",
            Self::Application => "Trikona (Triangle) - Balance",
            Self::Interface => "Tāḍa (Mountain) - Outward facing",
            Self::Infrastructure => "Śava (Corpse) - External, grounded",
        }
    }
}

/// Module in the architecture
#[derive(Debug, Clone)]
pub struct Module {
    pub name: String,
    pub layer: Layer,
    pub dependencies: Vec<String>,
    pub exports: Vec<String>,
    pub cohesion_score: f64,
}

/// Architecture analyzer
pub struct AsanaAnalyzer {
    /// All modules
    modules: HashMap<String, Module>,
    /// Detected violations
    violations: Vec<AsanaViolation>,
    /// Dependency graph
    dep_graph: HashMap<String, HashSet<String>>,
}

/// Architecture violation
#[derive(Debug, Clone)]
pub struct AsanaViolation {
    pub kind: ViolationKind,
    pub source: String,
    pub target: Option<String>,
    pub description: String,
    pub suggestion: String,
}

/// Types of architecture violations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ViolationKind {
    /// Dependency flows wrong direction
    WrongDependencyDirection,
    /// Circular dependency detected
    CircularDependency,
    /// Skip-layer dependency
    SkipLayer,
    /// God module (too many responsibilities)
    GodModule,
    /// Orphan module (no connections)
    OrphanModule,
    /// Low cohesion
    LowCohesion,
}

impl AsanaAnalyzer {
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            violations: Vec::new(),
            dep_graph: HashMap::new(),
        }
    }

    /// Add a module
    pub fn add_module(&mut self, module: Module) {
        let name = module.name.clone();
        for dep in &module.dependencies {
            self.dep_graph
                .entry(name.clone())
                .or_default()
                .insert(dep.clone());
        }
        self.modules.insert(name, module);
    }

    /// Analyze dependency directions
    pub fn check_dependency_directions(&mut self) {
        for (name, module) in &self.modules {
            for dep_name in &module.dependencies {
                if let Some(dep_module) = self.modules.get(dep_name) {
                    if !module.layer.can_depend_on(&dep_module.layer) {
                        self.violations.push(AsanaViolation {
                            kind: ViolationKind::WrongDependencyDirection,
                            source: name.clone(),
                            target: Some(dep_name.clone()),
                            description: format!(
                                "{:?} layer cannot depend on {:?} layer",
                                module.layer, dep_module.layer
                            ),
                            suggestion: "Invert dependency using abstraction".to_string(),
                        });
                    }
                }
            }
        }
    }

    /// Check for circular dependencies
    pub fn check_circular_dependencies(&mut self) {
        for module_name in self.modules.keys() {
            if let Some(cycle) = self.find_cycle(module_name) {
                self.violations.push(AsanaViolation {
                    kind: ViolationKind::CircularDependency,
                    source: module_name.clone(),
                    target: None,
                    description: format!("Circular dependency: {}", cycle.join(" → ")),
                    suggestion: "Break cycle with dependency inversion or extraction".to_string(),
                });
            }
        }
    }

    /// Find cycle starting from a node using DFS
    fn find_cycle(&self, start: &str) -> Option<Vec<String>> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        self.dfs_cycle(start, &mut visited, &mut path)
    }

    fn dfs_cycle(
        &self,
        node: &str,
        visited: &mut HashSet<String>,
        path: &mut Vec<String>,
    ) -> Option<Vec<String>> {
        if path.contains(&node.to_string()) {
            let cycle_start = path.iter().position(|n| n == node)?;
            let mut cycle: Vec<String> = path[cycle_start..].to_vec();
            cycle.push(node.to_string());
            return Some(cycle);
        }

        if visited.contains(node) {
            return None;
        }

        visited.insert(node.to_string());
        path.push(node.to_string());

        if let Some(deps) = self.dep_graph.get(node) {
            for dep in deps {
                if let Some(cycle) = self.dfs_cycle(dep, visited, path) {
                    return Some(cycle);
                }
            }
        }

        path.pop();
        None
    }

    /// Check for god modules
    pub fn check_god_modules(&mut self, max_dependencies: usize, max_exports: usize) {
        for (name, module) in &self.modules {
            if module.dependencies.len() > max_dependencies {
                self.violations.push(AsanaViolation {
                    kind: ViolationKind::GodModule,
                    source: name.clone(),
                    target: None,
                    description: format!(
                        "Module has {} dependencies (max: {})",
                        module.dependencies.len(),
                        max_dependencies
                    ),
                    suggestion: "Split module into smaller, focused modules".to_string(),
                });
            }

            if module.exports.len() > max_exports {
                self.violations.push(AsanaViolation {
                    kind: ViolationKind::GodModule,
                    source: name.clone(),
                    target: None,
                    description: format!(
                        "Module exports {} items (max: {})",
                        module.exports.len(),
                        max_exports
                    ),
                    suggestion: "Module has too many responsibilities".to_string(),
                });
            }
        }
    }

    /// Check cohesion
    pub fn check_cohesion(&mut self, min_cohesion: f64) {
        for (name, module) in &self.modules {
            if module.cohesion_score < min_cohesion {
                self.violations.push(AsanaViolation {
                    kind: ViolationKind::LowCohesion,
                    source: name.clone(),
                    target: None,
                    description: format!(
                        "Module cohesion {:.2} below minimum {:.2}",
                        module.cohesion_score, min_cohesion
                    ),
                    suggestion: "Group related functionality together".to_string(),
                });
            }
        }
    }

    /// Check for orphan modules
    pub fn check_orphan_modules(&mut self) {
        let mut referenced: HashSet<&str> = HashSet::new();

        for module in self.modules.values() {
            for dep in &module.dependencies {
                referenced.insert(dep.as_str());
            }
        }

        for name in self.modules.keys() {
            let module = &self.modules[name];
            if module.dependencies.is_empty() && !referenced.contains(name.as_str()) {
                self.violations.push(AsanaViolation {
                    kind: ViolationKind::OrphanModule,
                    source: name.clone(),
                    target: None,
                    description: "Module has no dependencies and is not referenced".to_string(),
                    suggestion: "Integrate module or remove if unused".to_string(),
                });
            }
        }
    }

    /// Run all architecture checks
    pub fn analyze(&mut self) {
        self.check_dependency_directions();
        self.check_circular_dependencies();
        self.check_god_modules(10, 20);
        self.check_cohesion(0.6);
        self.check_orphan_modules();
    }

    /// Get violations
    pub fn violations(&self) -> &[AsanaViolation] {
        &self.violations
    }

    /// Check if architecture is stable (no violations)
    pub fn is_stable(&self) -> bool {
        self.violations.is_empty()
    }

    /// Generate architecture diagram (DOT format)
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph Architecture {\n");
        dot.push_str("  rankdir=TB;\n");
        dot.push_str("  node [shape=box];\n\n");

        // Group by layer
        for layer in [Layer::Interface, Layer::Application, Layer::Domain, Layer::Infrastructure, Layer::Foundation] {
            dot.push_str(&format!("  subgraph cluster_{:?} {{\n", layer));
            dot.push_str(&format!("    label=\"{:?}\";\n", layer));

            for (name, module) in &self.modules {
                if module.layer == layer {
                    dot.push_str(&format!("    \"{}\";\n", name));
                }
            }

            dot.push_str("  }\n\n");
        }

        // Add edges
        for (name, module) in &self.modules {
            for dep in &module.dependencies {
                dot.push_str(&format!("  \"{}\" -> \"{}\";\n", name, dep));
            }
        }

        dot.push_str("}\n");
        dot
    }

    /// Clear for reuse
    pub fn clear(&mut self) {
        self.modules.clear();
        self.violations.clear();
        self.dep_graph.clear();
    }
}

impl Default for AsanaAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layer_dependencies() {
        assert!(Layer::Application.can_depend_on(&Layer::Domain));
        assert!(Layer::Application.can_depend_on(&Layer::Foundation));
        assert!(!Layer::Foundation.can_depend_on(&Layer::Application));
    }

    #[test]
    fn test_circular_dependency_detection() {
        let mut analyzer = AsanaAnalyzer::new();

        analyzer.add_module(Module {
            name: "A".to_string(),
            layer: Layer::Domain,
            dependencies: vec!["B".to_string()],
            exports: vec![],
            cohesion_score: 0.8,
        });

        analyzer.add_module(Module {
            name: "B".to_string(),
            layer: Layer::Domain,
            dependencies: vec!["A".to_string()], // Circular!
            exports: vec![],
            cohesion_score: 0.8,
        });

        analyzer.check_circular_dependencies();
        assert!(analyzer.violations.iter().any(|v| v.kind == ViolationKind::CircularDependency));
    }
}
