# Query System - Karma Kosha Architecture

## Overview

The Jagannath compiler implements a demand-driven incremental computation system
inspired by Salsa and aligned with Sanskrit Karma philosophy in `queries/`.

## Sanskrit Foundation

| Concept | Sanskrit | Meaning | Usage in Compiler |
|---------|----------|---------|-------------------|
| Query database | कर्म कोष (Karma Kosha) | Action repository | Central memoization store |
| Query | कर्मन् (Karman) | Action/deed | Computation that produces a result |
| Result | फल (Phala) | Fruit | Cached computation output |
| Cache | स्मृति (Smṛti) | Memory | Storage of previous results |
| Dependency | कर्म-बन्ध (Karma Bandha) | Action-bond | Relationships between queries |
| Revision | संस्कार (Saṃskāra) | Impression | Change tracking number |
| Cycle | संसार (Saṃsāra) | World-cycle | Cyclic dependency error |

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     KarmaKosha                               │
│  ┌──────────────┐  ┌─────────────────┐  ┌────────────────┐ │
│  │ QueryCache   │  │ DependencyGraph │  │ Revision       │ │
│  │ (Smṛti)      │  │ (Karma Bandha)  │  │ (Saṃskāra)     │ │
│  └──────────────┘  └─────────────────┘  └────────────────┘ │
└─────────────────────────────────────────────────────────────┘
         │                    │                    │
         ▼                    ▼                    ▼
┌─────────────┐    ┌─────────────────┐    ┌────────────────┐
│ CacheEntry  │    │ Forward edges   │    │ Atomic counter │
│ - value     │    │ (dependencies)  │    │ for change     │
│ - revision  │    │ Reverse edges   │    │ tracking       │
│ - deps      │    │ (dependents)    │    │                │
└─────────────┘    └─────────────────┘    └────────────────┘
```

## Core Components

### KarmaKosha (Query Database)

```rust
pub struct KarmaKosha {
    /// Current revision
    revision: RwLock<Revision>,
    /// Query definitions
    queries: RwLock<HashMap<QueryId, Arc<dyn QueryExecutor>>>,
    /// Query cache (smṛti)
    smriti: RwLock<QueryCache>,
    /// Dependency graph (karma-bandha)
    karma_bandha: RwLock<DependencyGraph>,
    /// Active query stack (cycle detection)
    active_stack: RwLock<Vec<QueryKey>>,
}
```

### Query Execution Flow

```
User calls db.query("parse", "main.jag")
              │
              ▼
┌─────────────────────────────┐
│ Check cache (smṛti)         │
│ Is (parse, main.jag) cached │
│ and revision matches?       │
└─────────────────────────────┘
         │ Yes: Return cached    │ No
         ▼                       ▼
┌──────────────┐    ┌─────────────────────────────┐
│ Cache hit!   │    │ Check for cycles (saṃsāra)  │
│ Return value │    │ Is "parse" on active stack? │
└──────────────┘    └─────────────────────────────┘
                              │ No cycle
                              ▼
                    ┌─────────────────────────────┐
                    │ Push to active stack        │
                    │ Execute query               │
                    │ Track dependencies          │
                    │ Pop from stack              │
                    │ Cache result                │
                    └─────────────────────────────┘
```

### Invalidation Strategy

When an input changes:
1. Increment global revision (saṃskāra)
2. Find all transitive dependents via karma-bandha
3. Remove stale cache entries
4. Next query will recompute

## Usage Examples

### Defining Input Queries

```rust
let kosha = KarmaKosha::new();

// Set source file content (input query)
kosha.set_input("source", "main.jag".to_string(), source_code);
kosha.set_input("source", "lib.jag".to_string(), lib_code);
```

### Defining Derived Queries

```rust
// Register parse query
kosha.register_query(
    QueryId::new("parse"),
    Arc::new(ParseQuery),
);

struct ParseQuery;

impl QueryExecutor for ParseQuery {
    fn execute(&self, db: &KarmaKosha, key: &dyn Any) -> QueryResult<...> {
        let path = key.downcast_ref::<String>().unwrap();

        // Read dependency (automatically tracked)
        let source: Arc<String> = db.query("source", path.clone())?;

        // Parse and return
        Ok(Box::new(Arc::new(parse(&source)?)))
    }

    fn name(&self) -> &str { "parse" }
}
```

### Query Usage

```rust
// First call: executes and caches
let ast1: Arc<Ast> = kosha.query("parse", "main.jag".to_string())?;

// Second call: returns cached
let ast2: Arc<Ast> = kosha.query("parse", "main.jag".to_string())?;

// Modify input: invalidates dependents
kosha.set_input("source", "main.jag".to_string(), new_source);

// Third call: recomputes
let ast3: Arc<Ast> = kosha.query("parse", "main.jag".to_string())?;
```

## Cycle Detection

Cyclic dependencies (saṃsāra) are runtime errors:

```rust
// A depends on B, B depends on A
QueryError::CyclicDependency {
    query: "typecheck",
    cycle: vec!["parse", "resolve", "typecheck", "parse"],
}
// Error message: "Cyclic dependency (saṃsāra) detected..."
```

The active_stack tracks in-progress queries to detect cycles before they cause infinite loops.

## Performance Statistics

```rust
pub struct QueryStats {
    pub total_queries: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub invalidations: u64,
    pub cycles_detected: u64,
}

let stats = kosha.stats();
println!("Hit ratio: {:.2}%", stats.hit_ratio() * 100.0);
```

## Compiler Pipeline Integration

```
┌──────────────────────────────────────────────────────────────┐
│                    Compilation Query Graph                    │
│                                                               │
│  source ─────► parse ─────► resolve ─────► typecheck         │
│    │             │            │              │                │
│    │             │            │              ▼                │
│  config ────────►├────────────┤          lower_mir           │
│                  │            │              │                │
│                  ▼            ▼              ▼                │
│               macros ───► imports       optimize              │
│                                              │                │
│                                              ▼                │
│                                           codegen             │
└──────────────────────────────────────────────────────────────┘
```

Each node is a query; edges are dependencies tracked via karma-bandha.

## Sanskrit Philosophy Alignment

### Karma Theory Mapping

| Sanskrit Concept | Query System |
|-----------------|--------------|
| Karma (action) | Query execution |
| Phala (fruit) | Computed result |
| Karmāśaya (action reservoir) | Cache |
| Saṃskāra (impression) | Revision number |
| Prārabdha (initiated karma) | In-progress queries |
| Kriyamāṇa (current karma) | Active computation |
| Saṃcita (accumulated karma) | All cached results |

### Saṃsāra (Cycle) as Anti-Pattern

In Hindu philosophy, saṃsāra is the cycle of rebirth that one seeks to escape.
In the compiler, cyclic dependencies are similarly undesirable - they represent
infinite loops in computation that must be broken.

The query system detects saṃsāra and reports it as an error, encouraging
developers to break the cycle through proper layering.

## Future Enhancements

1. **Parallel Query Execution**: Multiple independent queries in parallel
2. **Durability**: Persist cache to disk for faster incremental builds
3. **Query Tracing**: Debug why a query was recomputed
4. **Memory Pressure**: LRU eviction when cache grows too large
5. **Verified Inputs**: Hash-based input change detection

## References

- Salsa: Incremental recomputation (rustc)
- Adapton: Demand-driven incremental computation
- Bhagavad Gita Chapter 3: Karma Yoga (action philosophy)
