//! # Query Dependency Tracking - Karma Bandha
//!
//! Tracks dependencies between queries for invalidation.
//!
//! # Sanskrit Foundation
//!
//! **Karma Bandha** (कर्म बन्ध) - "action bond":
//! The chain of cause and effect between actions.
//! When one computation depends on another, they are bound.

use super::query::QueryKey;
use std::collections::{HashMap, HashSet};

/// Dependency graph for queries
pub struct DependencyGraph {
    /// Forward edges: query -> queries it depends on
    dependencies: HashMap<QueryKey, HashSet<QueryKey>>,

    /// Reverse edges: query -> queries that depend on it
    dependents: HashMap<QueryKey, HashSet<QueryKey>>,
}

impl DependencyGraph {
    /// Create a new dependency graph
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            dependents: HashMap::new(),
        }
    }

    /// Add a dependency edge
    pub fn add_dependency(&mut self, from: QueryKey, to: QueryKey) {
        // Forward edge
        self.dependencies
            .entry(from.clone())
            .or_default()
            .insert(to.clone());

        // Reverse edge
        self.dependents.entry(to).or_default().insert(from);
    }

    /// Get all dependencies of a query
    pub fn get_dependencies(&self, key: &QueryKey) -> Vec<QueryKey> {
        self.dependencies
            .get(key)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Get all queries that depend on a given query
    pub fn get_dependents(&self, key: &QueryKey) -> Vec<QueryKey> {
        self.dependents
            .get(key)
            .map(|s| s.iter().cloned().collect())
            .unwrap_or_default()
    }

    /// Get all transitive dependents (for cascading invalidation)
    pub fn get_transitive_dependents(&self, key: &QueryKey) -> HashSet<QueryKey> {
        let mut result = HashSet::new();
        let mut worklist = vec![key.clone()];

        while let Some(current) = worklist.pop() {
            if let Some(deps) = self.dependents.get(&current) {
                for dep in deps {
                    if result.insert(dep.clone()) {
                        worklist.push(dep.clone());
                    }
                }
            }
        }

        result
    }

    /// Remove a query and its edges
    pub fn remove(&mut self, key: &QueryKey) {
        // Remove forward edges
        if let Some(deps) = self.dependencies.remove(key) {
            for dep in deps {
                if let Some(rev) = self.dependents.get_mut(&dep) {
                    rev.remove(key);
                }
            }
        }

        // Remove reverse edges
        if let Some(deps) = self.dependents.remove(key) {
            for dep in deps {
                if let Some(fwd) = self.dependencies.get_mut(&dep) {
                    fwd.remove(key);
                }
            }
        }
    }

    /// Clear all dependencies
    pub fn clear(&mut self) {
        self.dependencies.clear();
        self.dependents.clear();
    }

    /// Check if there's a cyclic dependency
    pub fn has_cycle(&self, start: &QueryKey) -> bool {
        let mut visited = HashSet::new();
        let mut stack = HashSet::new();

        self.dfs_cycle(start, &mut visited, &mut stack)
    }

    fn dfs_cycle(
        &self,
        key: &QueryKey,
        visited: &mut HashSet<QueryKey>,
        stack: &mut HashSet<QueryKey>,
    ) -> bool {
        if stack.contains(key) {
            return true; // Cycle detected (saṃsāra!)
        }

        if visited.contains(key) {
            return false; // Already processed
        }

        visited.insert(key.clone());
        stack.insert(key.clone());

        if let Some(deps) = self.dependencies.get(key) {
            for dep in deps {
                if self.dfs_cycle(dep, visited, stack) {
                    return true;
                }
            }
        }

        stack.remove(key);
        false
    }

    /// Number of tracked queries
    pub fn len(&self) -> usize {
        self.dependencies.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.dependencies.is_empty()
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

/// Tracks dependencies during query execution
pub struct DependencyTracker {
    /// Current query being executed
    current: Option<QueryKey>,

    /// Collected dependencies
    dependencies: Vec<QueryKey>,
}

impl DependencyTracker {
    /// Create a new tracker
    pub fn new() -> Self {
        Self {
            current: None,
            dependencies: Vec::new(),
        }
    }

    /// Start tracking for a query
    pub fn start(&mut self, query: QueryKey) {
        self.current = Some(query);
        self.dependencies.clear();
    }

    /// Record a dependency
    pub fn record(&mut self, dependency: QueryKey) {
        if self.current.is_some() {
            self.dependencies.push(dependency);
        }
    }

    /// Finish tracking and return dependencies
    pub fn finish(&mut self) -> (Option<QueryKey>, Vec<QueryKey>) {
        let current = self.current.take();
        let deps = std::mem::take(&mut self.dependencies);
        (current, deps)
    }
}

impl Default for DependencyTracker {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::super::QueryId;
    use super::*;

    fn make_key(name: &str, key: i32) -> QueryKey {
        QueryKey::new(QueryId::new(name), Box::new(key))
    }

    #[test]
    fn test_dependency_graph_creation() {
        let graph = DependencyGraph::new();
        assert!(graph.is_empty());
    }

    #[test]
    fn test_add_dependency() {
        let mut graph = DependencyGraph::new();
        let a = make_key("parse", 1);
        let b = make_key("typecheck", 1);

        graph.add_dependency(a.clone(), b.clone());

        let deps = graph.get_dependencies(&a);
        assert_eq!(deps.len(), 1);

        let dependents = graph.get_dependents(&b);
        assert_eq!(dependents.len(), 1);
    }

    #[test]
    fn test_transitive_dependents() {
        let mut graph = DependencyGraph::new();
        let a = make_key("source", 1);
        let b = make_key("parse", 1);
        let c = make_key("typecheck", 1);

        graph.add_dependency(b.clone(), a.clone()); // parse depends on source
        graph.add_dependency(c.clone(), b.clone()); // typecheck depends on parse

        let transitive = graph.get_transitive_dependents(&a);
        assert_eq!(transitive.len(), 2); // Both b and c depend on a
    }

    #[test]
    fn test_cycle_detection() {
        let mut graph = DependencyGraph::new();
        let a = make_key("a", 1);
        let b = make_key("b", 1);
        let c = make_key("c", 1);

        graph.add_dependency(a.clone(), b.clone());
        graph.add_dependency(b.clone(), c.clone());

        // No cycle yet
        assert!(!graph.has_cycle(&a));

        // Create cycle: c -> a
        graph.add_dependency(c.clone(), a.clone());
        assert!(graph.has_cycle(&a));
    }

    #[test]
    fn test_dependency_tracker() {
        let mut tracker = DependencyTracker::new();
        let query = make_key("parse", 1);
        let dep1 = make_key("source", 1);
        let dep2 = make_key("config", 1);

        tracker.start(query.clone());
        tracker.record(dep1);
        tracker.record(dep2);

        let (current, deps) = tracker.finish();
        assert_eq!(current, Some(query));
        assert_eq!(deps.len(), 2);
    }
}
