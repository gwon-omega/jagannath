//! Karma Dependency Tracking
//!
//! Tracks causal dependencies (karma) between code elements.
//! Every action (code) has consequences (dependencies).

use std::collections::{HashMap, HashSet};

/// Karma dependency graph
pub struct KarmaGraph {
    /// Nodes (code elements)
    nodes: HashMap<String, KarmaNode>,
    /// Edges (dependencies)
    edges: Vec<KarmaDependency>,
}

/// Node in the karma graph
#[derive(Debug, Clone)]
pub struct KarmaNode {
    pub name: String,
    pub kind: NodeKind,
    /// Saṃskāra (accumulated impressions) - usage count
    pub samskara: u64,
}

/// Kind of node
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Function,
    Type,
    Variable,
    Module,
    File,
}

/// Dependency (karmic link)
#[derive(Debug, Clone)]
pub struct KarmaDependency {
    /// Source (cause)
    pub from: String,
    /// Target (effect)
    pub to: String,
    /// Kind of dependency
    pub kind: DependencyKind,
    /// Strength (how strong the karmic bond)
    pub strength: f32,
}

/// Kind of dependency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyKind {
    /// Calls
    Calls,
    /// Uses type
    UsesType,
    /// Reads value
    Reads,
    /// Writes value
    Writes,
    /// Imports
    Imports,
    /// Implements
    Implements,
}

impl KarmaGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: Vec::new(),
        }
    }

    /// Add a node
    pub fn add_node(&mut self, name: String, kind: NodeKind) {
        self.nodes.insert(name.clone(), KarmaNode {
            name,
            kind,
            samskara: 0,
        });
    }

    /// Add a dependency (karmic link)
    pub fn add_dependency(&mut self, from: String, to: String, kind: DependencyKind) {
        self.edges.push(KarmaDependency {
            from: from.clone(),
            to: to.clone(),
            kind,
            strength: 1.0,
        });

        // Increase saṃskāra (usage impressions)
        if let Some(node) = self.nodes.get_mut(&to) {
            node.samskara += 1;
        }
    }

    /// Get all dependencies of a node
    pub fn dependencies_of(&self, name: &str) -> Vec<&KarmaDependency> {
        self.edges.iter().filter(|e| e.from == name).collect()
    }

    /// Get all dependents of a node
    pub fn dependents_of(&self, name: &str) -> Vec<&KarmaDependency> {
        self.edges.iter().filter(|e| e.to == name).collect()
    }

    /// Compute karma score (how much this change affects others)
    pub fn karma_score(&self, name: &str) -> f32 {
        let direct_dependents = self.dependents_of(name).len() as f32;
        let samskara = self.nodes.get(name).map(|n| n.samskara).unwrap_or(0) as f32;

        // Karma = direct dependents + accumulated usage
        direct_dependents + (samskara * 0.1)
    }

    /// Find all nodes affected by changing this node (transitive karma)
    pub fn transitive_karma(&self, name: &str) -> HashSet<String> {
        let mut affected = HashSet::new();
        let mut queue = vec![name.to_string()];

        while let Some(current) = queue.pop() {
            if affected.contains(&current) {
                continue;
            }
            affected.insert(current.clone());

            for dep in self.dependents_of(&current) {
                queue.push(dep.from.clone());
            }
        }

        affected.remove(name);
        affected
    }

    /// Find cycle (karmic loop)
    pub fn find_cycles(&self) -> Vec<Vec<String>> {
        // TODO: Implement cycle detection
        Vec::new()
    }

    /// Get compilation order (topological sort respecting karma)
    pub fn compilation_order(&self) -> Vec<String> {
        // Kahn's algorithm for topological sort
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        let mut result = Vec::new();
        let mut queue = Vec::new();

        // Initialize in-degrees
        for name in self.nodes.keys() {
            in_degree.insert(name.clone(), 0);
        }
        for edge in &self.edges {
            *in_degree.entry(edge.from.clone()).or_insert(0) += 1;
        }

        // Find nodes with no incoming edges
        for (name, degree) in &in_degree {
            if *degree == 0 {
                queue.push(name.clone());
            }
        }

        // Process queue
        while let Some(name) = queue.pop() {
            result.push(name.clone());

            for dep in self.dependencies_of(&name) {
                if let Some(degree) = in_degree.get_mut(&dep.to) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push(dep.to.clone());
                    }
                }
            }
        }

        result
    }

    /// Cleanse karma (remove unused dependencies)
    pub fn cleanse(&mut self) {
        // Remove edges to nodes with zero saṃskāra
        let unused: HashSet<_> = self
            .nodes
            .iter()
            .filter(|(_, n)| n.samskara == 0)
            .map(|(name, _)| name.clone())
            .collect();

        self.edges.retain(|e| !unused.contains(&e.to));
    }
}

impl Default for KarmaGraph {
    fn default() -> Self {
        Self::new()
    }
}
