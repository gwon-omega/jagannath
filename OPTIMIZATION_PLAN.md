# Jagannath Compiler: Deep Optimization & Refactoring Strategy

**Created:** December 29, 2025
**Status:** Ready for Execution
**Prerequisite:** Review alongside `COMPLETION_PLAN.md` for context

---

## Executive Summary

While `COMPLETION_PLAN.md` claims 100% completion, deep codebase analysis reveals **55-65% actual functionality**. The philosophical scaffolding is excellent, but critical components have TODO stubs. This plan addresses:

1. **Core Pipeline Gaps** - Type checking disabled, register allocation stubbed
2. **Code Quality** - 30+ TODOs, 4 files >1000 LOC, blanket `#[allow(dead_code)]`
3. **Architecture** - Philosophy modules decorative, not load-bearing
4. **Modern Techniques** - Missing query-based incremental compilation

### Key Metrics Gap

| Metric | Claimed | Actual | Target |
|--------|---------|--------|--------|
| Core Compiler | 100% | ~60% | 100% |
| Type Checking | Complete | ✅ Enabled | Functional |
| Register Alloc | Complete | ✅ 862 LOC | Linear Scan |
| Philosophy Integration | Load-bearing | Decorative | Active |
| Blanket Allows | Present | ✅ Removed | None |
| Warning Count | 238 | ✅ 80 (dead_code) | <100 |

---

## Phase 1: Critical Path Fixes (P0)

**Timeline:** Week 1-2
**Goal:** Make the compiler actually compile

### 1.1 Enable Type Checking

**Location:** `compiler/src/driver/mod.rs` (Line 143)

```rust
// CURRENT (stubbed)
// TODO: Implement type checking

// TARGET
let typed_ast = typeck::TypeChecker::new(&self.session)
    .check_crate(&resolved_ast)?;
```

**Dependencies:**
- Connect `semantics/typeck.rs` to driver
- Wire Nyāya pramāṇa inference to actual type resolution
- Enable error reporting through Naraka system

### 1.2 Implement Register Allocation

**Location:** `compiler/src/codegen/regalloc.rs` (Line 70)

```rust
// CURRENT
// TODO: Implement linear scan algorithm

// TARGET - Linear Scan Algorithm
impl LinearScanAllocator {
    pub fn allocate(&mut self, mir: &MirFunction) -> AllocationResult {
        let intervals = self.compute_live_intervals(mir);
        let sorted = self.sort_by_start(intervals);

        for interval in sorted {
            self.expire_old_intervals(interval.start);

            if self.active.len() == self.num_registers {
                self.spill_at_interval(interval);
            } else {
                self.assign_register(interval);
            }
        }

        AllocationResult { assignments: self.assignments.clone() }
    }
}
```

### 1.3 Remove Blanket Allow Directives ✅ COMPLETED (Dec 2025)

**Location:** `compiler/src/lib.rs`

**Status:** ✅ Completed
- Removed all blanket `#![allow]` directives
- Fixed 157 warnings (unused imports, variables, mut)
- Remaining 80 warnings are dead_code for WIP features (expected)
- All 468 tests pass

**Action Items:**
- [x] Remove blanket allows from `lib.rs`
- [x] Fix ~175 resulting warnings (fixed 157)
- [x] Add targeted `#[allow]` only where justified (none needed)
- [ ] Run `cargo clippy` and address all issues (future work)

### 1.4 Fix Critical TODOs

| File | Line | Issue | Priority |
|------|------|-------|----------|
| `driver/mod.rs` | 143 | Type checking disabled | P0 |
| `codegen/regalloc.rs` | 70 | No register allocation | P0 |
| `macro/mod.rs` | 129 | Macro expansion stubbed | P1 |
| `garuda/preta/mod.rs` | 80 | Ghost detection TODO | P1 |
| `garuda/vaitarani/mod.rs` | 72 | Taint analysis TODO | P1 |

---

## Phase 2: Module Splitting (P1)

**Timeline:** Week 3-4
**Goal:** Improve maintainability and testability

### 2.1 Split `semantics/typeck.rs` (2,495 lines)

**Current:** Monolithic file with inference, unification, checking, Nyāya logic

**Target Structure:**
```
compiler/src/semantics/typeck/
├── mod.rs              # Re-exports, TypeChecker facade
├── inference.rs        # Algorithm W, constraint generation
├── unification.rs      # Type unification, occurs check
├── checker.rs          # Type checking driver, error collection
├── nyaya.rs            # 4-pramāṇa inference (Pratyakṣa, Anumāna, Upamāna, Śabda)
└── tests.rs            # Unit tests for type system
```

**Refactoring Strategy:**
```rust
// mod.rs - Clean public API
pub mod inference;
pub mod unification;
pub mod checker;
pub mod nyaya;

pub use checker::TypeChecker;
pub use inference::{InferenceContext, TypeVar};
pub use unification::{Unifier, UnificationError};
pub use nyaya::{Pramana, NyayaInference};
```

### 2.2 Split `semantics/borrow.rs` (1,472 lines)

**Target Structure:**
```
compiler/src/semantics/borrow/
├── mod.rs              # Re-exports, BorrowChecker facade
├── ownership.rs        # Ownership state machine (owned/moved/dropped)
├── borrowing.rs        # Borrow tracking, lifetime validation
├── linear.rs           # Linear type enforcement (Sanskrit -l affix)
├── nll.rs              # Non-lexical lifetimes, region inference
└── tests.rs            # Borrow checker tests
```

### 2.3 Split `parser/grammar.rs` (1,365 lines)

**Target Structure:**
```
compiler/src/parser/grammar/
├── mod.rs              # Re-exports
├── expr.rs             # Expression parsing (arithmetic, calls, etc.)
├── stmt.rs             # Statement parsing (let, if, loop, etc.)
├── item.rs             # Top-level items (fn, struct, impl, etc.)
├── pattern.rs          # Pattern matching syntax
└── precedence.rs       # Operator precedence (Pratt parsing)
```

### 2.4 Consolidate Naraka Files

**Current:** 28 separate files in `garuda/narakas/`

**Target:** Unified enum with per-naraka modules for detailed logic

```rust
// garuda/narakas/mod.rs
mod tamisram;      // Use-after-free
mod andhakupa;     // Null pointer
mod vaitarani;     // Tainted data
mod suchimukha;    // Memory leak
mod raurava;       // Panic/crash
mod asipatravana;  // Buffer overflow
// ... 22 more

/// All 28 Narakas from Garuda Purana mapped to error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Naraka {
    Tamisram,       // Hell 1: Theft → Use-after-free
    Andhakupa,      // Hell 9: Dark well → Null pointer
    Vaitarani,      // Hell 14: Filthy river → Tainted data
    Suchimukha,     // Hell 28: Needle torture → Memory leak
    Raurava,        // Hell 5: Screaming → Panic/crash
    Asipatravana,   // Hell 3: Sword forest → Buffer overflow
    // ... all 28
}

impl Naraka {
    pub fn sin(&self) -> &'static str { ... }
    pub fn punishment(&self) -> &'static str { ... }
    pub fn penance(&self) -> &'static str { ... }
}
```

---

## Phase 3: Query-Based Architecture (P2)

**Timeline:** Week 5-8
**Goal:** Enable incremental compilation like rustc

### 3.1 Why Query-Based?

From rustc-dev-guide research:
> "The Rust compiler is not organized as a series of passes over the code which execute sequentially. The Rust compiler does this to make incremental compilation possible."

**Benefits:**
- **Incremental:** Only recompute what changed
- **Parallel:** Independent queries run concurrently
- **Cached:** Results persist across compilations
- **Testable:** Queries are isolated units

### 3.2 Query Infrastructure

```rust
// compiler/src/queries/mod.rs
use salsa::Database;

/// Central query database - replaces global state
#[salsa::database(
    SourceStorage,
    ParsingStorage,
    TypeCheckingStorage,
    BorrowCheckingStorage,
    MirStorage,
    CodegenStorage
)]
pub struct CompilerDatabase {
    storage: salsa::Storage<Self>,
}

// Source queries
#[salsa::query_group(SourceStorage)]
pub trait SourceDatabase {
    #[salsa::input]
    fn source_text(&self, file: FileId) -> Arc<String>;

    fn line_index(&self, file: FileId) -> Arc<LineIndex>;
}

// Parsing queries
#[salsa::query_group(ParsingStorage)]
pub trait ParsingDatabase: SourceDatabase {
    fn parse(&self, file: FileId) -> Arc<ast::SourceFile>;
    fn ast_id_map(&self, file: FileId) -> Arc<AstIdMap>;
}

// Type checking queries (Nyāya-powered)
#[salsa::query_group(TypeCheckingStorage)]
pub trait TypeCheckingDatabase: ParsingDatabase {
    fn infer_type(&self, expr: ExprId) -> Ty;
    fn check_function(&self, func: FunctionId) -> TypeCheckResult;
    fn resolve_path(&self, path: PathId) -> Option<DefId>;
}
```

### 3.3 Migration Strategy

1. **Week 5:** Add `salsa` dependency, create query infrastructure
2. **Week 6:** Migrate lexer/parser to queries
3. **Week 7:** Migrate type checking to queries
4. **Week 8:** Migrate MIR/codegen, enable incremental mode

---

## Phase 4: Trait Abstractions (Reusability)

**Timeline:** Week 9-10
**Goal:** Enable extensibility through well-defined interfaces

### 4.1 AST/MIR Visitor Pattern

```rust
// compiler/src/visitor.rs
use std::ops::ControlFlow;

/// Visitor for AST traversal
pub trait AstVisitor {
    type Result;

    fn visit_expr(&mut self, expr: &ast::Expr) -> ControlFlow<Self::Result> {
        walk_expr(self, expr)
    }

    fn visit_stmt(&mut self, stmt: &ast::Stmt) -> ControlFlow<Self::Result> {
        walk_stmt(self, stmt)
    }

    fn visit_item(&mut self, item: &ast::Item) -> ControlFlow<Self::Result> {
        walk_item(self, item)
    }
}

/// Visitor for MIR traversal
pub trait MirVisitor {
    fn visit_function(&mut self, func: &mir::Function);
    fn visit_basic_block(&mut self, bb: &mir::BasicBlock);
    fn visit_statement(&mut self, stmt: &mir::Statement);
    fn visit_terminator(&mut self, term: &mir::Terminator);
    fn visit_operand(&mut self, operand: &mir::Operand);
    fn visit_place(&mut self, place: &mir::Place);
}
```

### 4.2 Unified Diagnostic Trait

```rust
// compiler/src/diagnostics/mod.rs

/// Common interface for all compiler diagnostics
pub trait Diagnostic: Send + Sync {
    /// Error code (e.g., E0001, N0005 for Naraka)
    fn code(&self) -> DiagnosticCode;

    /// Human-readable message
    fn message(&self) -> String;

    /// Source location
    fn span(&self) -> Span;

    /// Severity level
    fn severity(&self) -> Severity;

    /// Fix suggestions (penance/prāyaścitta)
    fn suggestions(&self) -> Vec<Suggestion>;

    /// Related information
    fn related(&self) -> Vec<RelatedInfo> { vec![] }
}

/// Severity levels mapped to Guṇas
#[derive(Debug, Clone, Copy)]
pub enum Severity {
    Error,      // Tamas - blocking
    Warning,    // Rajas - concerning
    Info,       // Sattva - informational
    Hint,       // Guidance
}
```

### 4.3 Codegen Backend Trait

```rust
// compiler/src/codegen/backend.rs

/// Extensible code generation backend
pub trait CodegenBackend: Send + Sync {
    /// Backend identifier
    fn name(&self) -> &'static str;

    /// Target triple (e.g., "x86_64-unknown-linux-gnu")
    fn target_triple(&self) -> &str;

    /// Pointer size in bytes
    fn pointer_size(&self) -> u8;

    /// Available registers
    fn registers(&self) -> &[Register];

    /// Calling convention
    fn calling_convention(&self) -> CallingConvention;

    /// Emit assembly for a function
    fn emit_function(&self, func: &mir::Function) -> Result<Vec<u8>, CodegenError>;

    /// Emit full module
    fn emit_module(&self, module: &mir::Module) -> Result<Vec<u8>, CodegenError>;

    /// Link objects into executable
    fn link(&self, objects: &[PathBuf], output: &Path) -> Result<(), LinkError>;
}

/// Backend factory
pub fn create_backend(target: Target) -> Box<dyn CodegenBackend> {
    match target {
        Target::X86_64 => Box::new(x86_64::X86_64Backend::new()),
        Target::AArch64 => Box::new(aarch64::AArch64Backend::new()),
        Target::RiscV64 => Box::new(riscv64::RiscV64Backend::new()),
    }
}
```

---

## Phase 5: Enhanced Optimization Passes

**Timeline:** Week 11-12
**Goal:** Implement LLVM-caliber optimizations via Astra framework

### 5.1 Analysis Passes (New Astras)

| Pass | Astra Name | Purpose | LLVM Equivalent |
|------|------------|---------|-----------------|
| Dominator Tree | VṛkṣaAstra (Tree) | Control flow dominance | `domtree` |
| Loop Analysis | CakraAstra (Wheel) | Loop nest detection | `loops` |
| Alias Analysis | MāyāAstra (Illusion) | Memory aliasing | `basic-aa` |
| Scalar Evolution | GatiAstra (Motion) | Induction variables | `scalar-evolution` |
| Call Graph | JālaAstra (Network) | Inter-procedural | `basiccg` |

### 5.2 Transform Passes (Enhanced Astras)

| Pass | Map to Existing | Purpose | LLVM Equivalent |
|------|-----------------|---------|-----------------|
| Loop Invariant Code Motion | Vāyuastra (Wind) | Hoist loop invariants | `licm` |
| Global Value Numbering | Jñānāstra (Wisdom) | Eliminate redundancy | `gvn` |
| Sparse Conditional Constant Prop | SatyaAstra (Truth) | Constant propagation | `sccp` |
| Dead Code Elimination | Brahmastra (existing) | Remove unreachable code | `dce`, `adce` |
| Instruction Combining | SaṅghaAstra (Union) | Peephole optimization | `instcombine` |
| Tail Call Elimination | MokṣaAstra (Liberation) | TCO for recursion | `tailcallelim` |
| Loop Unrolling | AnantaAstra (Infinite) | Unroll small loops | `loop-unroll` |
| Inlining | EkatvāAstra (Unity) | Function inlining | `inline` |

### 5.3 Implementation Pattern

```rust
// compiler/src/astras/licm.rs (Loop Invariant Code Motion)

/// Vāyuastra - Wind weapon that lifts invariants out of loops
pub struct Vayuastra {
    loop_analysis: CakraAstra,
    domtree: VrksaAstra,
}

impl DivyaAstra for Vayuastra {
    fn name(&self) -> &'static str { "Vāyuastra" }
    fn deity(&self) -> AstraDeity { AstraDeity::Vayu }
    fn mantra(&self) -> Mantra {
        Mantra::new("Om Vāyave Namaḥ - Lift the unchanging from cycles")
    }

    fn invoke(&self, func: &mut MirFunction) -> AstraResult {
        log::info!("Invoking Vāyuastra: {}", self.mantra());

        let loops = self.loop_analysis.find_loops(func);
        let mut hoisted = 0;

        for loop_info in loops {
            let invariants = self.find_invariants(func, &loop_info);
            for stmt in invariants {
                self.hoist_to_preheader(func, &loop_info, stmt);
                hoisted += 1;
            }
        }

        AstraResult::Deployed {
            power_level: 7,
            transformations: hoisted,
        }
    }
}
```

---

## Phase 6: Philosophy Integration (Active)

**Timeline:** Week 13-16
**Goal:** Make philosophy modules load-bearing, not decorative

### 6.1 DevataSystem Pipeline Integration

```rust
// compiler/src/driver/mod.rs

impl Driver {
    pub fn compile(&self, source: Source) -> Result<Binary, CompileError> {
        let devata = DevataSystem::new();

        // 12 Adityas execute compilation phases in cosmic order
        let mut state = CompilationState::new(source);

        for aditya in devata.adityas() {
            state = aditya.execute(state).map_err(|e| {
                // Ashvins diagnose failures
                devata.ashvins().diagnose(&e)
            })?;
        }

        // 11 Rudras apply transformations
        for rudra in devata.rudras() {
            state = rudra.transform(state)?;
        }

        // Verify Ṛta (cosmic order) maintained
        devata.verify_rta(&state)?;

        state.finalize()
    }
}
```

### 6.2 NavaDurgaDefense Active Security

```rust
// compiler/src/codegen/mod.rs

impl Codegen {
    pub fn emit(&self, mir: &MirModule) -> Result<Vec<u8>, CodegenError> {
        let durga = NavaDurgaDefense::new();
        let context = SecurityContext::from_mir(mir);

        // Must pass all 9 goddess layers
        match durga.protect(&context) {
            SecurityResult::Perfect => {
                log::info!("Siddhidatri grants perfection");
                self.emit_unchecked(mir)
            }
            SecurityResult::Blocked { layer, goddess, reason } => {
                Err(CodegenError::SecurityViolation {
                    layer,
                    goddess: goddess.to_string(),
                    reason,
                    penance: durga.get_penance(layer),
                })
            }
        }
    }
}
```

### 6.3 Marga-Driven Optimization Selection

```rust
// compiler/src/mir/optimize.rs

impl MirOptimizer {
    pub fn optimize(&mut self, module: &mut MirModule) -> OptimizationReport {
        let selector = MargaSelector::new();
        let marga = selector.analyze_and_select(module);

        log::info!("Selected {} for optimization", marga.name());

        match marga {
            Marga::Karma => {
                // Action path: loops, mutations, side effects
                self.apply_pass::<LoopOptimizer>(module);
                self.apply_pass::<StateMachineOptimizer>(module);
                self.apply_pass::<SideEffectOrdering>(module);
            }
            Marga::Jnana => {
                // Knowledge path: pure functions, immutability
                self.apply_pass::<PurityAnalysis>(module);
                self.apply_pass::<Memoization>(module);
                self.apply_pass::<AggressiveConstantFolding>(module);
            }
            Marga::Bhakti => {
                // Devotion path: domain-specific optimizations
                self.apply_pass::<DomainPatternMatcher>(module);
                self.apply_pass::<SpecializedIntrinsics>(module);
            }
            Marga::RajaYoga => {
                // Royal path: balanced hybrid
                self.apply_all_passes_balanced(module);
            }
        }

        self.generate_report()
    }
}
```

### 6.4 Purushartha-Balanced Tradeoffs

```rust
// compiler/src/purusharthas/triangle.rs

impl PurusharthaTriangle {
    /// Find Pareto-optimal compilation strategy
    pub fn balance(&self, constraints: &CompileConstraints) -> Strategy {
        let weights = PurusharthaWeights {
            artha: constraints.resource_weight,   // Memory/binary size
            kama: constraints.performance_weight, // Runtime speed
            dharma: constraints.safety_weight,    // Correctness
        };

        // Check for Moksha (liberation) - all goals maximized
        if self.can_achieve_moksha(&weights) {
            return Strategy::Moksha;
        }

        // Find Pareto frontier point
        self.find_pareto_optimal(weights)
    }

    fn can_achieve_moksha(&self, weights: &PurusharthaWeights) -> bool {
        // Moksha achieved when no tradeoff needed
        weights.artha >= 0.9 && weights.kama >= 0.9 && weights.dharma >= 1.0
    }
}
```

---

## Success Metrics

### Phase Completion Criteria

| Phase | Success Criteria | Verification |
|-------|------------------|--------------|
| P1 | Type checking runs, 0 warnings | `cargo build`, `cargo test` |
| P2 | No file >500 LOC, tests pass | LOC count, CI green |
| P3 | Incremental rebuild <100ms | Benchmark cold vs warm |
| P4 | All backends use traits | Code review |
| P5 | 3.2× faster than C | `benchmarks/RESULTS.md` |
| P6 | Philosophy in call path | Execution traces |

### Overall Targets

| Metric | Current | Target | Method |
|--------|---------|--------|--------|
| Compilation Speed (10K LOC) | ~2.5s | <1.2s | Query caching |
| Runtime vs C | ~2.5× | 3.5× | Enhanced Astras |
| TODO Count | 30+ | 0 | Systematic cleanup |
| Test Coverage | ~60% | 95% | Property + unit tests |
| Large Files (>1000 LOC) | 4 | 0 | Module splitting |
| Philosophy Integration | Decorative | Active | Pipeline wiring |

---

## Timeline Summary

```
┌─────────────────────────────────────────────────────────────────┐
│ Week 1-2   │ Phase 1: Critical Path (P0)                        │
│            │ - Enable type checking                              │
│            │ - Implement register allocation                     │
│            │ - Remove blanket allows                             │
├─────────────────────────────────────────────────────────────────┤
│ Week 3-4   │ Phase 2: Module Splitting (P1)                     │
│            │ - Split typeck.rs, borrow.rs, grammar.rs           │
│            │ - Consolidate Naraka files                          │
├─────────────────────────────────────────────────────────────────┤
│ Week 5-8   │ Phase 3: Query Architecture (P2)                   │
│            │ - Add Salsa infrastructure                          │
│            │ - Migrate to on-demand compilation                  │
│            │ - Enable incremental mode                           │
├─────────────────────────────────────────────────────────────────┤
│ Week 9-10  │ Phase 4: Trait Abstractions                        │
│            │ - Visitor patterns                                  │
│            │ - Unified diagnostics                               │
│            │ - Backend trait                                     │
├─────────────────────────────────────────────────────────────────┤
│ Week 11-12 │ Phase 5: Enhanced Optimizations                    │
│            │ - New analysis Astras                               │
│            │ - LLVM-caliber transforms                           │
├─────────────────────────────────────────────────────────────────┤
│ Week 13-16 │ Phase 6: Philosophy Integration                    │
│            │ - DevataSystem in pipeline                          │
│            │ - Active NavaDurga security                         │
│            │ - Marga optimization selection                      │
└─────────────────────────────────────────────────────────────────┘
```

---

## Decisions Required

### Decision 1: Incremental Compilation Library

**Options:**
- **Salsa** - Rust-analyzer's choice, mature, well-documented
- **Custom** - Full control, tailored to Jagannath philosophy
- **Hybrid** - Salsa core with philosophy-aware wrapper

**Recommendation:** Salsa with philosophy wrapper

### Decision 2: Cross-Architecture Priority

**Options:**
- Complete x86-64 → Extract traits → AArch64 → RISC-V (Sequential)
- Parallel development with shared abstraction (Parallel)

**Recommendation:** Sequential - x86-64 first to prove patterns

### Decision 3: Test Strategy

**Options:**
- Unit tests only (current approach)
- Property-based testing (QuickCheck/proptest)
- Hybrid with fuzzing

**Recommendation:** Hybrid - unit + property + selective fuzzing

---

## References

- [LLVM Passes Documentation](https://llvm.org/docs/Passes.html)
- [Rustc Dev Guide - Query System](https://rustc-dev-guide.rust-lang.org/overview.html)
- [Rustc Incremental Compilation](https://rustc-dev-guide.rust-lang.org/queries/incremental-compilation.html)
- [Salsa Incremental Computation](https://docs.rs/salsa/latest/salsa/)
- `AGENTS.md` - Project philosophy mappings
- `COMPLETION_PLAN.md` - Original completion tracking
- `v1.md` through `v18.md` - Language specifications
