//! Module Graph (Khaṇḍa Jāla - खण्ड जाल)
//!
//! Sanskrit: जाल (jāla) = network, graph
//!
//! Implements a directed graph of module dependencies with:
//! - Kahn's algorithm for topological sorting
//! - Kosaraju's algorithm for strongly connected component detection
//! - Cycle detection via DFS coloring
//!
//! Research basis:
//! - "Depth-First Search and Linear Graph Algorithms" (Tarjan, 1972)
//! - "Topological Sorting of Large Networks" (Kahn, 1962)
//! - "Linear Time Algorithms for Finding and Representing All the Cuts of a Graph"

use super::{Module, ModuleError};
use std::collections::{HashMap, HashSet, VecDeque};

/// Unique module identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ModuleId(pub u32);

impl ModuleId {
    pub const ROOT: ModuleId = ModuleId(0);
}

/// Module dependency graph
#[derive(Debug, Clone)]
pub struct ModuleGraph {
    /// All modules indexed by ID
    pub modules: HashMap<ModuleId, Module>,
    /// Forward edges: module → dependencies
    pub dependencies: HashMap<ModuleId, Vec<ModuleId>>,
    /// Reverse edges: module ← dependents
    pub dependents: HashMap<ModuleId, Vec<ModuleId>>,
    /// Path to module ID mapping for fast lookup
    path_index: HashMap<String, ModuleId>,
    /// Next available module ID
    next_id: u32,
}

impl ModuleGraph {
    /// Create an empty module graph
    pub fn new() -> Self {
        Self {
            modules: HashMap::new(),
            dependencies: HashMap::new(),
            dependents: HashMap::new(),
            path_index: HashMap::new(),
            next_id: 0,
        }
    }

    /// Add a module to the graph
    pub fn add_module(&mut self, mut module: Module) -> ModuleId {
        let id = ModuleId(self.next_id);
        self.next_id += 1;
        module.id = id;

        // Index by path
        let path_key = module.path.join("::");
        self.path_index.insert(path_key, id);

        // Initialize edge lists
        self.dependencies.insert(id, Vec::new());
        self.dependents.insert(id, Vec::new());

        self.modules.insert(id, module);
        id
    }

    /// Add a dependency edge (from depends on to)
    pub fn add_dependency(&mut self, from: ModuleId, to: ModuleId) {
        self.dependencies.entry(from).or_default().push(to);
        self.dependents.entry(to).or_default().push(from);
    }

    /// Find module by path
    pub fn find_by_path(&self, path: &[String]) -> Option<ModuleId> {
        let key = path.join("::");
        self.path_index.get(&key).copied()
    }

    /// Find module by name (last component of path)
    pub fn find_by_name(&self, name: &str) -> Option<ModuleId> {
        self.modules
            .iter()
            .find(|(_, m)| m.name == name)
            .map(|(id, _)| *id)
    }

    /// Get all dependencies of a module
    pub fn get_dependencies(&self, id: ModuleId) -> &[ModuleId] {
        self.dependencies
            .get(&id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get all dependents of a module
    pub fn get_dependents(&self, id: ModuleId) -> &[ModuleId] {
        self.dependents
            .get(&id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Detect circular dependencies using DFS coloring
    /// Returns the cycle if found, None otherwise
    ///
    /// Algorithm: Three-color DFS
    /// - White (0): Not visited
    /// - Gray (1): Currently in recursion stack
    /// - Black (2): Completely processed
    pub fn find_cycle(&self) -> Option<Vec<ModuleId>> {
        #[derive(Clone, Copy, PartialEq)]
        enum Color {
            White,
            Gray,
            Black,
        }

        let mut colors: HashMap<ModuleId, Color> =
            self.modules.keys().map(|&id| (id, Color::White)).collect();

        let mut parent: HashMap<ModuleId, ModuleId> = HashMap::new();

        fn dfs(
            graph: &ModuleGraph,
            node: ModuleId,
            colors: &mut HashMap<ModuleId, Color>,
            parent: &mut HashMap<ModuleId, ModuleId>,
        ) -> Option<(ModuleId, ModuleId)> {
            colors.insert(node, Color::Gray);

            for &dep in graph.get_dependencies(node) {
                match colors.get(&dep) {
                    Some(Color::Gray) => {
                        // Found cycle: back edge to gray node
                        return Some((node, dep));
                    }
                    Some(Color::White) => {
                        parent.insert(dep, node);
                        if let Some(cycle) = dfs(graph, dep, colors, parent) {
                            return Some(cycle);
                        }
                    }
                    _ => {}
                }
            }

            colors.insert(node, Color::Black);
            None
        }

        // Try DFS from each unvisited node
        for &id in self.modules.keys() {
            if colors.get(&id) == Some(&Color::White) {
                if let Some((from, to)) = dfs(self, id, &mut colors, &mut parent) {
                    // Reconstruct cycle path
                    let mut cycle = vec![to];
                    let mut current = from;
                    while current != to {
                        cycle.push(current);
                        current = *parent.get(&current).unwrap_or(&to);
                    }
                    cycle.push(to);
                    cycle.reverse();
                    return Some(cycle);
                }
            }
        }

        None
    }

    /// Topological sort using Kahn's algorithm
    /// Returns modules in dependency-first order
    ///
    /// Algorithm (Kahn, 1962):
    /// 1. Compute in-degree for all nodes
    /// 2. Add all zero in-degree nodes to queue
    /// 3. Remove node from queue, add to result, decrement neighbors' in-degree
    /// 4. Repeat until queue empty
    pub fn topological_order(&self) -> Result<Vec<ModuleId>, ModuleError> {
        // Calculate in-degrees
        let mut in_degree: HashMap<ModuleId, usize> =
            self.modules.keys().map(|&id| (id, 0)).collect();

        for deps in self.dependencies.values() {
            for &dep in deps {
                *in_degree.entry(dep).or_insert(0) += 1;
            }
        }

        // Initialize queue with zero in-degree nodes
        let mut queue: VecDeque<ModuleId> = in_degree
            .iter()
            .filter(|(_, &deg)| deg == 0)
            .map(|(&id, _)| id)
            .collect();

        let mut result = Vec::with_capacity(self.modules.len());

        while let Some(node) = queue.pop_front() {
            result.push(node);

            // Decrease in-degree of dependencies
            for &dep in self.get_dependencies(node) {
                if let Some(deg) = in_degree.get_mut(&dep) {
                    *deg -= 1;
                    if *deg == 0 {
                        queue.push_back(dep);
                    }
                }
            }
        }

        // If we couldn't sort all nodes, there's a cycle
        if result.len() != self.modules.len() {
            // Find cycle using DFS
            if let Some(cycle) = self.find_cycle() {
                return Err(ModuleError::CircularDependency(cycle));
            }
        }

        // Reverse to get dependencies first
        result.reverse();
        Ok(result)
    }

    /// Find strongly connected components using Kosaraju's algorithm
    /// Each SCC with >1 node represents a circular dependency group
    ///
    /// Algorithm (Kosaraju, 1978):
    /// 1. DFS on original graph, record finish times
    /// 2. Transpose the graph
    /// 3. DFS on transpose in reverse finish order
    pub fn find_strongly_connected_components(&self) -> Vec<Vec<ModuleId>> {
        let mut visited: HashSet<ModuleId> = HashSet::new();
        let mut finish_order: Vec<ModuleId> = Vec::new();

        // First DFS pass: record finish times
        fn dfs_first(
            graph: &ModuleGraph,
            node: ModuleId,
            visited: &mut HashSet<ModuleId>,
            finish_order: &mut Vec<ModuleId>,
        ) {
            if visited.contains(&node) {
                return;
            }
            visited.insert(node);

            for &dep in graph.get_dependencies(node) {
                dfs_first(graph, dep, visited, finish_order);
            }

            finish_order.push(node);
        }

        for &id in self.modules.keys() {
            dfs_first(self, id, &mut visited, &mut finish_order);
        }

        // Second DFS pass on transposed graph
        fn dfs_second(
            graph: &ModuleGraph,
            node: ModuleId,
            visited: &mut HashSet<ModuleId>,
            component: &mut Vec<ModuleId>,
        ) {
            if visited.contains(&node) {
                return;
            }
            visited.insert(node);
            component.push(node);

            // Use reverse edges (dependents instead of dependencies)
            for &dep in graph.get_dependents(node) {
                dfs_second(graph, dep, visited, component);
            }
        }

        visited.clear();
        let mut components: Vec<Vec<ModuleId>> = Vec::new();

        for &node in finish_order.iter().rev() {
            if !visited.contains(&node) {
                let mut component = Vec::new();
                dfs_second(self, node, &mut visited, &mut component);
                components.push(component);
            }
        }

        components
    }

    /// Get modules that have no dependencies (root modules)
    pub fn get_roots(&self) -> Vec<ModuleId> {
        self.modules
            .keys()
            .filter(|&id| {
                self.dependencies
                    .get(id)
                    .map(|deps| deps.is_empty())
                    .unwrap_or(true)
            })
            .copied()
            .collect()
    }

    /// Get modules that nothing depends on (leaf modules)
    pub fn get_leaves(&self) -> Vec<ModuleId> {
        self.modules
            .keys()
            .filter(|&id| {
                self.dependents
                    .get(id)
                    .map(|deps| deps.is_empty())
                    .unwrap_or(true)
            })
            .copied()
            .collect()
    }

    /// Print module graph in DOT format for visualization
    pub fn to_dot(&self) -> String {
        let mut dot = String::from("digraph ModuleGraph {\n");
        dot.push_str("  rankdir=BT;\n"); // Bottom to top

        for (id, module) in &self.modules {
            dot.push_str(&format!(
                "  m{} [label=\"{}\"];\n",
                id.0,
                module.path.join("::")
            ));
        }

        for (from, deps) in &self.dependencies {
            for to in deps {
                dot.push_str(&format!("  m{} -> m{};\n", from.0, to.0));
            }
        }

        dot.push_str("}\n");
        dot
    }
}

impl Default for ModuleGraph {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::symbol::SymbolTable;
    use std::path::PathBuf;

    fn make_module(name: &str) -> Module {
        Module {
            id: ModuleId(0),
            name: name.to_string(),
            path: vec![name.to_string()],
            file_path: PathBuf::from(format!("{}.jag", name)),
            ast: None,
            exports: SymbolTable::new(),
            imports: Vec::new(),
            compiled: false,
        }
    }

    #[test]
    fn test_add_module() {
        let mut graph = ModuleGraph::new();
        let m1 = graph.add_module(make_module("main"));
        let m2 = graph.add_module(make_module("utils"));

        assert_eq!(graph.modules.len(), 2);
        assert_eq!(m1.0, 0);
        assert_eq!(m2.0, 1);
    }

    #[test]
    fn test_find_by_path() {
        let mut graph = ModuleGraph::new();
        let m1 = graph.add_module(make_module("main"));

        assert_eq!(graph.find_by_path(&["main".to_string()]), Some(m1));
        assert_eq!(graph.find_by_path(&["other".to_string()]), None);
    }

    #[test]
    fn test_topological_order_simple() {
        let mut graph = ModuleGraph::new();
        let m1 = graph.add_module(make_module("main"));
        let m2 = graph.add_module(make_module("utils"));
        let m3 = graph.add_module(make_module("core"));

        // main -> utils -> core
        graph.add_dependency(m1, m2);
        graph.add_dependency(m2, m3);

        let order = graph.topological_order().unwrap();
        assert_eq!(order, vec![m3, m2, m1]);
    }

    #[test]
    fn test_cycle_detection() {
        let mut graph = ModuleGraph::new();
        let m1 = graph.add_module(make_module("a"));
        let m2 = graph.add_module(make_module("b"));
        let m3 = graph.add_module(make_module("c"));

        // a -> b -> c -> a (cycle)
        graph.add_dependency(m1, m2);
        graph.add_dependency(m2, m3);
        graph.add_dependency(m3, m1);

        let cycle = graph.find_cycle();
        assert!(cycle.is_some());
    }

    #[test]
    fn test_no_cycle() {
        let mut graph = ModuleGraph::new();
        let m1 = graph.add_module(make_module("main"));
        let m2 = graph.add_module(make_module("utils"));

        graph.add_dependency(m1, m2);

        let cycle = graph.find_cycle();
        assert!(cycle.is_none());
    }

    #[test]
    fn test_scc_detection() {
        let mut graph = ModuleGraph::new();
        let m1 = graph.add_module(make_module("a"));
        let m2 = graph.add_module(make_module("b"));
        let m3 = graph.add_module(make_module("c"));
        let m4 = graph.add_module(make_module("d"));

        // SCC1: a <-> b
        graph.add_dependency(m1, m2);
        graph.add_dependency(m2, m1);

        // SCC2: c -> d (no back edge)
        graph.add_dependency(m3, m4);

        // Cross edge: a -> c
        graph.add_dependency(m1, m3);

        let sccs = graph.find_strongly_connected_components();

        // Should have 3 SCCs: {a,b}, {c}, {d}
        assert_eq!(sccs.len(), 3);

        // Find the SCC with multiple nodes
        let multi_scc = sccs.iter().find(|scc| scc.len() > 1);
        assert!(multi_scc.is_some());
    }

    #[test]
    fn test_dot_output() {
        let mut graph = ModuleGraph::new();
        let m1 = graph.add_module(make_module("main"));
        let m2 = graph.add_module(make_module("utils"));
        graph.add_dependency(m1, m2);

        let dot = graph.to_dot();
        assert!(dot.contains("digraph"));
        assert!(dot.contains("main"));
        assert!(dot.contains("utils"));
    }
}
