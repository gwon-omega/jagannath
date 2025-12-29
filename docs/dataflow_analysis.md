# Dataflow Analysis Framework - Karma-Bandha Architecture

## Overview

The Jagannath compiler implements a generic lattice-based dataflow analysis framework
in the `garuda/dataflow/` module. This enables multiple analysis passes (Preta ghost
detection, Vaitarani security boundaries, etc.) to share common infrastructure.

## Sanskrit Foundation

| Concept | Sanskrit | Meaning | Usage in Compiler |
|---------|----------|---------|-------------------|
| Dataflow | कर्म-बन्ध (Karma Bandha) | Action-bond | Dependencies between computations |
| Lattice | शृङ्खला (Śṛṅkhalā) | Chain | Ordered domain of abstract values |
| Join | संगम (Saṅgama) | Confluence | Merging values at control flow joins |
| Meet | मिलन (Milana) | Meeting | Computing greatest lower bound |
| Fixpoint | स्थिर-बिन्दु (Sthira-Bindu) | Stable point | Converged analysis result |

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                  DataflowSolver                              │
│  ┌──────────────┐  ┌─────────────────┐  ┌────────────────┐ │
│  │ Direction    │  │ Worklist        │  │ Fixpoint       │ │
│  │ (Forward/    │  │ Algorithm       │  │ Iteration      │ │
│  │  Backward)   │  │                 │  │                │ │
│  └──────────────┘  └─────────────────┘  └────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                  Lattice Trait                               │
│  ┌──────────────┐  ┌─────────────────┐  ┌────────────────┐ │
│  │ bottom()     │  │ join(&self,     │  │ leq(&self,     │ │
│  │ ⊥ element    │  │      &Self)     │  │     &Self)     │ │
│  └──────────────┘  └─────────────────┘  └────────────────┘ │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│              Domain Implementations                          │
│  ┌──────────────┐  ┌─────────────────┐  ┌────────────────┐ │
│  │ SetLattice   │  │ ResourceLattice │  │ TaintLattice   │ │
│  │ Powerset     │  │ Alloc/Free/     │  │ Clean/Tainted/ │ │
│  │ Domain       │  │ Escaped         │  │ Purified       │ │
│  └──────────────┘  └─────────────────┘  └────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## Core Traits

### Lattice

```rust
pub trait Lattice: Clone + Eq {
    /// Bottom element (⊥) - empty/no information
    fn bottom() -> Self;

    /// Join operation (⊔) - least upper bound
    fn join(&self, other: &Self) -> Self;

    /// Partial ordering - is self ≤ other in the lattice?
    fn leq(&self, other: &Self) -> bool;

    /// Meet operation (⊓) - greatest lower bound (optional)
    fn meet(&self, other: &Self) -> Self {
        // Default: self if leq, else bottom
    }
}
```

### SetLattice

Generic powerset domain for collecting facts:

```rust
pub struct SetLattice<T: Clone + Eq + std::hash::Hash> {
    elements: HashSet<T>,
}
```

- `bottom()` = empty set
- `join(a, b)` = a ∪ b (union)
- `leq(a, b)` = a ⊆ b (subset)

### ResourceLattice

Tracks resource allocation states:

```rust
pub enum ResourceState {
    Unknown,     // Initial state
    Allocated,   // Resource acquired
    Freed,       // Resource released
    Escaped,     // Resource passed out (e.g., returned)
}
```

## Control Flow Graph

```rust
pub struct ControlFlowGraph {
    num_blocks: usize,
    successors: Vec<Vec<usize>>,
    predecessors: Vec<Vec<usize>>,
}
```

CFG construction:
1. Each basic block gets an index
2. `add_edge(from, to)` adds flow edge
3. `compute_predecessors()` builds reverse edges

## Worklist Algorithm

The solver uses iterative worklist with predecessor-driven updates:

```
Initialize:
  ∀ block b: state[b] = ⊥
  worklist = {entry} for forward, {exit} for backward

Iterate:
  while worklist not empty:
    b = worklist.pop()
    new_state = transfer(b, join of predecessor states)
    if new_state ≠ state[b]:
      state[b] = new_state
      add successors to worklist

Result:
  state[] contains fixpoint solution
```

## Usage Example: Preta Ghost Detection

```rust
use crate::garuda::dataflow::{ResourceLattice, DataflowSolver, Direction};

// Track allocations forward through the function
let mut solver = DataflowSolver::new(Direction::Forward);
let initial = ResourceLattice::new();

// Transfer function: update resource states
let transfer = |block, state: &ResourceLattice| {
    let mut new_state = state.clone();
    for stmt in block.stmts {
        match stmt {
            Stmt::Let { value: Call("alloc", _), .. } =>
                new_state.allocate(var),
            Stmt::Expr(Call("free", var)) =>
                new_state.free(var),
            Stmt::Return { value, .. } =>
                new_state.escape(value),
        }
    }
    new_state
};

let result = solver.solve(&cfg, initial, transfer);

// At function exit, any still-allocated resources are leaks
for (var, state) in result.exit_state {
    if state == ResourceState::Allocated {
        report_leak(var);
    }
}
```

## Research Connections

### Academic Foundations

1. **Kildall's Algorithm** (1973): Original worklist dataflow
2. **Tarski's Fixpoint Theorem**: Guarantees termination on finite lattices
3. **Abstract Interpretation** (Cousot & Cousot, 1977): Sound approximation

### Sanskrit Philosophical Mapping

| Dataflow Concept | Sanskrit Parallel |
|-----------------|-------------------|
| Forward analysis | प्रवृत्ति (Pravṛtti) - outward action flow |
| Backward analysis | निवृत्ति (Nivṛtti) - withdrawal/return |
| Fixpoint | मोक्ष (Mokṣa) - liberation/stable state |
| Lattice ordering | धर्म क्रम (Dharma Krama) - righteous order |
| Join operation | संयोग (Saṃyoga) - union/conjunction |
| Transfer function | कर्म (Karma) - action transformation |

### Garuda Purana Mapping

The dataflow framework detects "sins" (bugs) that lead to specific Narakas:

| Analysis | Detects | Naraka |
|----------|---------|--------|
| Resource tracking | Memory leaks | Suchimukha (needle torture) |
| Taint analysis | Injection attacks | Vaitarani (filthy river) |
| Use-after-free | Dangling pointers | Tamisram (darkness) |
| Deadlock detection | Lock ordering | Kalasutra (time binding) |

## Performance Characteristics

| Metric | Value |
|--------|-------|
| Time complexity | O(n × h × e) where n=nodes, h=lattice height, e=edges |
| Space complexity | O(n × |domain|) |
| Typical iterations | 2-3 for structured programs |

## Future Enhancements

1. **Sparse Analysis**: Only track modified variables per block
2. **Demand-Driven**: Compute only for queried program points
3. **Widening**: For infinite-height lattices (numeric analysis)
4. **Context-Sensitivity**: Track calling context for precision

## References

- Muchnick, S. S., "Advanced Compiler Design and Implementation" (1997)
- Nielson, F., et al., "Principles of Program Analysis" (2005)
- Garuda Purana, Chapter 5-15: Classifications of Hell
