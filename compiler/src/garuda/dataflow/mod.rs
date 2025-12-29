//! # Dataflow Analysis Framework
//!
//! Generic dataflow analysis infrastructure for the Garuda system.
//!
//! # Sanskrit Foundation
//! The dataflow framework embodies **Prāṇa-vāha** (प्राण-वाह - life-force flow):
//! - Data flows through the code like prāṇa through nāḍīs (channels)
//! - Analysis tracks this flow to detect anomalies (doṣas)
//!
//! # Philosophy: Sāṃkhya Flow
//! Like the evolution from Prakṛti through 25 tattvas,
//! data transforms as it flows through the program.
//!
//! ```text
//! Source (Prakṛti) → Transformations (Tattvas) → Sink (Puruṣa observes)
//! ```
//!
//! # Design
//! Based on the classic dataflow framework:
//! - Forward analysis: propagates from entry to exit
//! - Backward analysis: propagates from exit to entry
//! - Lattice-based: monotonic transfer functions ensure termination

use std::collections::{HashMap, HashSet, VecDeque};

/// Direction of dataflow analysis
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// Forward analysis (from entry to exit)
    /// Like prāṇa flowing outward (prāṇa-vāyu)
    Forward,
    /// Backward analysis (from exit to entry)
    /// Like apāna-vāyu returning inward
    Backward,
}

/// A lattice element for dataflow analysis
///
/// Must form a bounded semi-lattice with:
/// - `bottom()`: least element (⊥)
/// - `join()`: least upper bound (⊔)
/// - `leq()`: partial ordering (⊑)
pub trait Lattice: Clone + PartialEq + Eq + std::fmt::Debug {
    /// The bottom element (⊥) - no information
    fn bottom() -> Self;

    /// The top element (⊤) - maximum information (optional)
    fn top() -> Self {
        Self::bottom() // Default: no top
    }

    /// Join two elements: compute least upper bound
    /// a ⊔ b = smallest c where a ⊑ c and b ⊑ c
    fn join(&self, other: &Self) -> Self;

    /// Partial ordering: is self ⊑ other?
    fn leq(&self, other: &Self) -> bool;

    /// Meet two elements: compute greatest lower bound (optional)
    fn meet(&self, _other: &Self) -> Self {
        // Default: no meet operation
        self.clone()
    }
}

/// A generic set lattice (powerset domain)
/// Used for many analyses: live variables, reaching definitions, etc.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetLattice<T: Clone + Eq + std::hash::Hash> {
    elements: HashSet<T>,
}

impl<T: Clone + Eq + std::hash::Hash + std::fmt::Debug> SetLattice<T> {
    pub fn new() -> Self {
        Self {
            elements: HashSet::new(),
        }
    }

    pub fn singleton(element: T) -> Self {
        let mut s = Self::new();
        s.elements.insert(element);
        s
    }

    pub fn from_iter(iter: impl IntoIterator<Item = T>) -> Self {
        Self {
            elements: iter.into_iter().collect(),
        }
    }

    pub fn insert(&mut self, element: T) {
        self.elements.insert(element);
    }

    pub fn remove(&mut self, element: &T) {
        self.elements.remove(element);
    }

    pub fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.elements.iter()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn len(&self) -> usize {
        self.elements.len()
    }

    pub fn union(&self, other: &Self) -> Self {
        Self {
            elements: self.elements.union(&other.elements).cloned().collect(),
        }
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self {
            elements: self
                .elements
                .intersection(&other.elements)
                .cloned()
                .collect(),
        }
    }

    pub fn difference(&self, other: &Self) -> Self {
        Self {
            elements: self.elements.difference(&other.elements).cloned().collect(),
        }
    }
}

impl<T: Clone + Eq + std::hash::Hash + std::fmt::Debug> Default for SetLattice<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone + Eq + std::hash::Hash + std::fmt::Debug> Lattice for SetLattice<T> {
    fn bottom() -> Self {
        Self::new()
    }

    fn join(&self, other: &Self) -> Self {
        self.union(other)
    }

    fn leq(&self, other: &Self) -> bool {
        self.elements.is_subset(&other.elements)
    }

    fn meet(&self, other: &Self) -> Self {
        self.intersection(other)
    }
}

/// A transfer function for dataflow analysis
///
/// Computes the effect of a statement/instruction on the dataflow facts.
pub trait TransferFunction<L: Lattice> {
    /// The type of statement/node being analyzed
    type Node;

    /// Apply transfer function: out = f(in)
    /// For forward analysis: out = gen ∪ (in - kill)
    /// For backward analysis: in = gen ∪ (out - kill)
    fn transfer(&self, node: &Self::Node, input: &L) -> L;
}

/// Basic block identifier
pub type BlockId = usize;

/// Control flow graph for dataflow analysis
#[derive(Debug, Clone)]
pub struct ControlFlowGraph {
    /// Entry block
    pub entry: BlockId,
    /// Exit blocks
    pub exits: Vec<BlockId>,
    /// Successors of each block
    pub successors: HashMap<BlockId, Vec<BlockId>>,
    /// Predecessors of each block (computed from successors)
    pub predecessors: HashMap<BlockId, Vec<BlockId>>,
    /// Number of blocks
    pub num_blocks: usize,
}

impl ControlFlowGraph {
    pub fn new(num_blocks: usize) -> Self {
        Self {
            entry: 0,
            exits: vec![num_blocks.saturating_sub(1)],
            successors: HashMap::new(),
            predecessors: HashMap::new(),
            num_blocks,
        }
    }

    pub fn add_edge(&mut self, from: BlockId, to: BlockId) {
        self.successors.entry(from).or_default().push(to);
        self.predecessors.entry(to).or_default().push(from);
    }

    pub fn compute_predecessors(&mut self) {
        self.predecessors.clear();
        for (&from, tos) in &self.successors {
            for &to in tos {
                self.predecessors.entry(to).or_default().push(from);
            }
        }
    }
}

/// Result of dataflow analysis
#[derive(Debug)]
pub struct DataflowResult<L: Lattice> {
    /// Facts at entry of each block
    pub in_facts: HashMap<BlockId, L>,
    /// Facts at exit of each block
    pub out_facts: HashMap<BlockId, L>,
    /// Number of iterations to reach fixpoint
    pub iterations: usize,
}

/// Generic dataflow analysis solver
///
/// Uses worklist algorithm to compute fixpoint.
pub struct DataflowSolver<L: Lattice> {
    direction: Direction,
    _phantom: std::marker::PhantomData<L>,
}

impl<L: Lattice> DataflowSolver<L> {
    pub fn new(direction: Direction) -> Self {
        Self {
            direction,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Solve dataflow analysis to fixpoint
    ///
    /// # Parameters
    /// - `cfg`: Control flow graph
    /// - `block_facts`: Transfer function results for each block
    /// - `boundary`: Initial facts at entry (forward) or exit (backward)
    pub fn solve<F>(
        &self,
        cfg: &ControlFlowGraph,
        mut transfer: F,
        boundary: L,
    ) -> DataflowResult<L>
    where
        F: FnMut(BlockId, &L) -> L,
    {
        let mut in_facts: HashMap<BlockId, L> = HashMap::new();
        let mut out_facts: HashMap<BlockId, L> = HashMap::new();

        // Initialize all facts to bottom
        for block in 0..cfg.num_blocks {
            in_facts.insert(block, L::bottom());
            out_facts.insert(block, L::bottom());
        }

        // Set boundary condition
        match self.direction {
            Direction::Forward => {
                in_facts.insert(cfg.entry, boundary);
            }
            Direction::Backward => {
                for &exit in &cfg.exits {
                    out_facts.insert(exit, boundary.clone());
                }
            }
        }

        // Worklist algorithm
        let mut worklist: VecDeque<BlockId> = (0..cfg.num_blocks).collect();
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 1000;

        while let Some(block) = worklist.pop_front() {
            iterations += 1;
            if iterations > MAX_ITERATIONS {
                break; // Prevent infinite loops
            }

            match self.direction {
                Direction::Forward => {
                    // in[B] = ⊔ out[P] for all predecessors P
                    let predecessors = cfg.predecessors.get(&block).cloned().unwrap_or_default();
                    let new_in = if block == cfg.entry {
                        in_facts.get(&block).cloned().unwrap_or_else(L::bottom)
                    } else {
                        predecessors.iter().fold(L::bottom(), |acc, &pred| {
                            acc.join(&out_facts.get(&pred).cloned().unwrap_or_else(L::bottom))
                        })
                    };

                    // Only update if changed
                    if new_in != *in_facts.get(&block).unwrap_or(&L::bottom()) {
                        in_facts.insert(block, new_in.clone());
                    }

                    // out[B] = transfer(in[B])
                    let new_out = transfer(block, &new_in);
                    if new_out != *out_facts.get(&block).unwrap_or(&L::bottom()) {
                        out_facts.insert(block, new_out);

                        // Add successors to worklist
                        if let Some(succs) = cfg.successors.get(&block) {
                            for &succ in succs {
                                if !worklist.contains(&succ) {
                                    worklist.push_back(succ);
                                }
                            }
                        }
                    }
                }
                Direction::Backward => {
                    // out[B] = ⊔ in[S] for all successors S
                    let successors = cfg.successors.get(&block).cloned().unwrap_or_default();
                    let new_out = if cfg.exits.contains(&block) {
                        out_facts.get(&block).cloned().unwrap_or_else(L::bottom)
                    } else {
                        successors.iter().fold(L::bottom(), |acc, &succ| {
                            acc.join(&in_facts.get(&succ).cloned().unwrap_or_else(L::bottom))
                        })
                    };

                    if new_out != *out_facts.get(&block).unwrap_or(&L::bottom()) {
                        out_facts.insert(block, new_out.clone());
                    }

                    // in[B] = transfer(out[B])
                    let new_in = transfer(block, &new_out);
                    if new_in != *in_facts.get(&block).unwrap_or(&L::bottom()) {
                        in_facts.insert(block, new_in);

                        // Add predecessors to worklist
                        if let Some(preds) = cfg.predecessors.get(&block) {
                            for &pred in preds {
                                if !worklist.contains(&pred) {
                                    worklist.push_back(pred);
                                }
                            }
                        }
                    }
                }
            }
        }

        DataflowResult {
            in_facts,
            out_facts,
            iterations,
        }
    }
}

/// Resource state for tracking allocations/deallocations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceState {
    /// Resource has been allocated
    Allocated,
    /// Resource has been deallocated/freed
    Freed,
    /// Resource has escaped (returned, stored globally)
    Escaped,
    /// Resource state is unknown
    Unknown,
}

/// Resource tracking fact
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ResourceFact {
    /// Variable/resource name
    pub name: String,
    /// Current state
    pub state: ResourceState,
    /// Allocation location (for diagnostics)
    pub alloc_location: Option<usize>,
}

/// Resource tracking lattice for leak detection
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResourceLattice {
    /// Tracked resources
    resources: HashMap<String, ResourceFact>,
}

impl ResourceLattice {
    pub fn new() -> Self {
        Self {
            resources: HashMap::new(),
        }
    }

    pub fn allocate(&mut self, name: String, location: Option<usize>) {
        self.resources.insert(
            name.clone(),
            ResourceFact {
                name,
                state: ResourceState::Allocated,
                alloc_location: location,
            },
        );
    }

    pub fn free(&mut self, name: &str) {
        if let Some(fact) = self.resources.get_mut(name) {
            fact.state = ResourceState::Freed;
        }
    }

    pub fn escape(&mut self, name: &str) {
        if let Some(fact) = self.resources.get_mut(name) {
            fact.state = ResourceState::Escaped;
        }
    }

    pub fn get_state(&self, name: &str) -> Option<ResourceState> {
        self.resources.get(name).map(|f| f.state)
    }

    /// Get all resources that are still allocated (potential leaks)
    pub fn get_leaks(&self) -> Vec<&ResourceFact> {
        self.resources
            .values()
            .filter(|f| f.state == ResourceState::Allocated)
            .collect()
    }

    /// Get all resources that were double-freed
    pub fn get_double_frees(&self, freed_names: &HashSet<String>) -> Vec<&ResourceFact> {
        self.resources
            .values()
            .filter(|f| f.state == ResourceState::Freed && freed_names.contains(&f.name))
            .collect()
    }
}

impl Default for ResourceLattice {
    fn default() -> Self {
        Self::new()
    }
}

impl Lattice for ResourceLattice {
    fn bottom() -> Self {
        Self::new()
    }

    fn join(&self, other: &Self) -> Self {
        let mut result = self.clone();
        for (name, fact) in &other.resources {
            match result.resources.get(name) {
                Some(existing) => {
                    // Join states: Allocated + Freed = Unknown, etc.
                    let joined_state = match (existing.state, fact.state) {
                        (a, b) if a == b => a,
                        (ResourceState::Unknown, _) | (_, ResourceState::Unknown) => {
                            ResourceState::Unknown
                        }
                        (ResourceState::Escaped, _) | (_, ResourceState::Escaped) => {
                            ResourceState::Escaped
                        }
                        _ => ResourceState::Unknown,
                    };
                    result.resources.insert(
                        name.clone(),
                        ResourceFact {
                            name: name.clone(),
                            state: joined_state,
                            alloc_location: existing.alloc_location.or(fact.alloc_location),
                        },
                    );
                }
                None => {
                    result.resources.insert(name.clone(), fact.clone());
                }
            }
        }
        result
    }

    fn leq(&self, other: &Self) -> bool {
        // self ⊑ other if self.resources ⊆ other.resources
        self.resources
            .iter()
            .all(|(k, v)| other.resources.get(k).map_or(false, |ov| v == ov))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_lattice_operations() {
        let a: SetLattice<i32> = SetLattice::from_iter(vec![1, 2, 3]);
        let b: SetLattice<i32> = SetLattice::from_iter(vec![2, 3, 4]);

        let union = a.join(&b);
        assert!(union.contains(&1));
        assert!(union.contains(&2));
        assert!(union.contains(&3));
        assert!(union.contains(&4));

        let intersect = a.meet(&b);
        assert!(!intersect.contains(&1));
        assert!(intersect.contains(&2));
        assert!(intersect.contains(&3));
        assert!(!intersect.contains(&4));
    }

    #[test]
    fn test_set_lattice_ordering() {
        let a: SetLattice<i32> = SetLattice::from_iter(vec![1, 2]);
        let b: SetLattice<i32> = SetLattice::from_iter(vec![1, 2, 3]);

        assert!(a.leq(&b));
        assert!(!b.leq(&a));
    }

    #[test]
    fn test_resource_lattice() {
        let mut r = ResourceLattice::new();
        r.allocate("x".to_string(), Some(10));

        assert_eq!(r.get_state("x"), Some(ResourceState::Allocated));
        assert_eq!(r.get_leaks().len(), 1);

        r.free("x");
        assert_eq!(r.get_state("x"), Some(ResourceState::Freed));
        assert_eq!(r.get_leaks().len(), 0);
    }

    #[test]
    fn test_cfg_construction() {
        let mut cfg = ControlFlowGraph::new(3);
        cfg.add_edge(0, 1);
        cfg.add_edge(0, 2);
        cfg.add_edge(1, 2);

        assert_eq!(cfg.successors.get(&0), Some(&vec![1, 2]));
        assert_eq!(cfg.predecessors.get(&2), Some(&vec![0, 1]));
    }

    #[test]
    fn test_forward_dataflow() {
        let mut cfg = ControlFlowGraph::new(3);
        cfg.add_edge(0, 1);
        cfg.add_edge(1, 2);
        cfg.compute_predecessors();

        let solver: DataflowSolver<SetLattice<i32>> = DataflowSolver::new(Direction::Forward);

        // Simple reaching definitions: gen = {block_id}, kill = {}
        let result = solver.solve(
            &cfg,
            |block, input| {
                let mut out = input.clone();
                out.insert(block as i32);
                out
            },
            SetLattice::new(),
        );

        // Block 2 should have definitions from all blocks
        assert!(result.in_facts.get(&2).unwrap().contains(&0));
        assert!(result.in_facts.get(&2).unwrap().contains(&1));
    }
}
