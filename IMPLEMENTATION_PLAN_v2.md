# Jagannath Compiler - Reality-Based Implementation Plan
## From Architectural Prototype to Production Compiler

**Author:** Senior Systems Engineer Analysis
**Date:** December 28, 2025
**Status:** Implementation Ready

---

## EXECUTIVE SUMMARY

### Current State Assessment

After deep code analysis, here's an honest assessment:

| Component | Current State | Lines of Code | Real Completion |
|-----------|---------------|---------------|-----------------|
| **Lexer** | Functional with sandhi stubs | ~700 LOC | **70%** |
| **Parser** | Grammar implemented, compounds partial | ~1000 LOC | **60%** |
| **Type Checker** | Algorithm W started, inference partial | ~2100 LOC | **30%** |
| **Borrow Checker** | Framework exists, analysis partial | ~1400 LOC | **40%** |
| **MIR** | Types defined, passes are stubs | ~600 LOC | **25%** |
| **Codegen** | x86-64 structure exists, emission partial | ~1400 LOC | **35%** |
| **Philosophy Layers** | Excellent scaffolding, integration minimal | ~8000 LOC | **20%** |

**Total Real Work Done:** ~35% of core compiler
**Time to Production:** 18-24 months with focused effort

### What Works Now
- ✅ Tokenization of basic Jagannath syntax
- ✅ AST construction for functions, types, expressions
- ✅ Basic type resolution (explicit types)
- ✅ Simple borrow checking (obvious violations)
- ✅ x86-64 assembly structure (prologue/epilogue)
- ✅ Excellent error taxonomy (28 Narakas)

### What Doesn't Work Yet
- ❌ Real Sanskrit morphological analysis (sandhi splitting)
- ❌ Full type inference (Hindley-Milner Algorithm W)
- ❌ Complete borrow checking (Rust-level analysis)
- ❌ MIR optimization passes (all are stubs with `// TODO`)
- ❌ Complete assembly emission (many instructions missing)
- ❌ End-to-end compilation to executable

---

## PHASE 1: FOUNDATION COMPLETION (3-4 months)

### 1.1 Lexer Completion

**Current State:** Scanner works, sandhi/dhātu are stubs
**Goal:** Real Sanskrit morphological analysis

#### Tasks:
```
Priority | Task                                    | Estimated | Complexity
=========|=========================================|===========|===========
P0       | Complete FST-based sandhi splitting     | 3 weeks   | High
P0       | Implement compound word (samāsa) parser | 2 weeks   | High
P1       | Build dhātu dictionary (500 roots)      | 2 weeks   | Medium
P1       | Affix sequence validation               | 1 week    | Medium
P2       | Unicode normalization (NFC/NFD)         | 1 week    | Low
```

**Key File Changes:**
```
compiler/src/lexer/
├── sandhi.rs       # REWRITE: Real FST implementation
├── dhatu.rs        # EXPAND: Complete root dictionary
├── affixes.rs      # COMPLETE: Validation & semantics
└── compounds.rs    # NEW: Samāsa decomposition
```

**Implementation Strategy - sandhi.rs:**
```rust
// Current (stub):
impl SandhiFst {
    pub fn split(&self, word: &str) -> Vec<String> {
        vec![word.to_string()] // Just returns word unchanged
    }
}

// Required (real FST):
impl SandhiFst {
    /// Build finite-state transducer for sandhi rules
    pub fn build() -> Self {
        // Implement 20+ sandhi rules from Aṣṭādhyāyī
        // Example: ā + i → e (vowel sandhi)
        // Example: t + d → d (consonant sandhi)
        let mut fst = FstBuilder::new();
        fst.add_rule(SandhiRule::VowelCoalescence {
            first: 'ā', second: 'i', result: "e"
        });
        fst.add_rule(SandhiRule::ConsonantAssimilation {
            first: 't', second: 'd', result: "d"
        });
        // ... 50+ rules
        Self { fst: fst.build() }
    }

    pub fn split(&self, word: &str) -> Vec<SandhiSplit> {
        self.fst.analyze(word) // Returns all possible splits with scores
    }
}
```

### 1.2 Parser Completion

**Current State:** Basic grammar works, error recovery weak
**Goal:** Complete grammar + excellent error recovery

#### Tasks:
```
Priority | Task                                    | Estimated | Complexity
=========|=========================================|===========|===========
P0       | Pattern matching (pratyabhijñā)         | 2 weeks   | High
P0       | Macro expansion system                  | 2 weeks   | High
P1       | Error recovery with synchronization     | 1 week    | Medium
P1       | Expression precedence (Pratt parser)    | 1 week    | Medium
P2       | Documentation comment extraction        | 1 week    | Low
```

**Key Implementation - Pattern Matching:**
```rust
// New: compiler/src/parser/patterns.rs
pub enum Pattern {
    /// Wildcard: _
    Wildcard,
    /// Variable binding: x
    Binding { name: Identifier, mutable: bool },
    /// Literal: 42, "hello", true
    Literal(Literal),
    /// Tuple: (a, b, c)
    Tuple(Vec<Pattern>),
    /// Struct: Point { x, y }
    Struct { name: Identifier, fields: Vec<(Identifier, Pattern)> },
    /// Enum variant: Some(x) | None
    Variant { enum_name: Option<Identifier>, variant: Identifier, fields: Vec<Pattern> },
    /// Array: [a, b, c]
    Array(Vec<Pattern>),
    /// Slice: [head, ..tail]
    Slice { head: Vec<Pattern>, tail: Option<Box<Pattern>> },
    /// Range: 0..10
    Range { start: Option<Box<Pattern>>, end: Option<Box<Pattern>> },
    /// Or pattern: A | B | C
    Or(Vec<Pattern>),
    /// Guard: pattern if condition
    Guard { pattern: Box<Pattern>, condition: Box<Expr> },
}

impl Parser {
    pub fn parse_match_expr(&mut self) -> Result<Expr, ParseError> {
        self.expect(&TokenKind::Match)?; // pratyabhijñā
        let scrutinee = self.parse_expr()?;
        self.expect(&TokenKind::LeftBrace)?;

        let mut arms = Vec::new();
        while !self.check(&TokenKind::RightBrace) {
            let pattern = self.parse_pattern()?;
            self.expect(&TokenKind::FatArrow)?;
            let body = self.parse_expr()?;
            arms.push(MatchArm { pattern, body });
            self.match_token(&TokenKind::Comma);
        }

        self.expect(&TokenKind::RightBrace)?;
        Ok(Expr::Match { scrutinee: Box::new(scrutinee), arms })
    }
}
```

---

## PHASE 2: TYPE SYSTEM & SAFETY (4-5 months)

### 2.1 Complete Type Inference

**Current State:** Basic Algorithm W, no unification
**Goal:** Full Hindley-Milner with Nyāya integration

#### Tasks:
```
Priority | Task                                    | Estimated | Complexity
=========|=========================================|===========|===========
P0       | Complete unification algorithm          | 3 weeks   | Very High
P0       | Let-polymorphism (generalization)       | 2 weeks   | High
P0       | Trait constraint solving                | 3 weeks   | Very High
P1       | Higher-kinded types (basic)             | 2 weeks   | High
P1       | Associated types                        | 2 weeks   | High
P2       | Type error diagnostics                  | 2 weeks   | Medium
```

**Key Implementation - Unification:**
```rust
// compiler/src/semantics/typeck.rs - Complete implementation
impl TypeInference {
    /// Unify two types, returning substitution or error
    pub fn unify(&mut self, t1: &ResolvedType, t2: &ResolvedType) -> Result<(), TypeError> {
        let t1 = self.apply_substitution(t1);
        let t2 = self.apply_substitution(t2);

        match (&t1, &t2) {
            // Same type - trivial unification
            _ if t1 == t2 => Ok(()),

            // Type variable - occurs check + substitution
            (ResolvedType::TypeVar(v), t) | (t, ResolvedType::TypeVar(v)) => {
                if self.occurs_in(*v, t) {
                    Err(TypeError::InfiniteType { var: *v, ty: t.clone() })
                } else {
                    self.substitutions.insert(*v, t.clone());
                    Ok(())
                }
            }

            // Function types - unify params and return
            (ResolvedType::Function { params: p1, return_type: r1 },
             ResolvedType::Function { params: p2, return_type: r2 }) => {
                if p1.len() != p2.len() {
                    return Err(TypeError::ArityMismatch { expected: p1.len(), found: p2.len() });
                }
                for (a, b) in p1.iter().zip(p2.iter()) {
                    self.unify(a, b)?;
                }
                self.unify(r1, r2)
            }

            // Named types with generics
            (ResolvedType::Named { name: n1, generics: g1 },
             ResolvedType::Named { name: n2, generics: g2 }) => {
                if n1 != n2 {
                    return Err(TypeError::TypeMismatch { expected: t1.clone(), found: t2.clone() });
                }
                for (a, b) in g1.iter().zip(g2.iter()) {
                    self.unify(a, b)?;
                }
                Ok(())
            }

            // Reference types - check mutability + inner
            (ResolvedType::Reference { inner: i1, mutable: m1, .. },
             ResolvedType::Reference { inner: i2, mutable: m2, .. }) => {
                if *m1 != *m2 {
                    return Err(TypeError::MutabilityMismatch { expected: *m1, found: *m2 });
                }
                self.unify(i1, i2)
            }

            // Array types
            (ResolvedType::Array { element: e1, size: s1 },
             ResolvedType::Array { element: e2, size: s2 }) => {
                if s1 != s2 && s1.is_some() && s2.is_some() {
                    return Err(TypeError::ArraySizeMismatch { expected: *s1, found: *s2 });
                }
                self.unify(e1, e2)
            }

            // Tuple types
            (ResolvedType::Tuple(t1), ResolvedType::Tuple(t2)) => {
                if t1.len() != t2.len() {
                    return Err(TypeError::TupleSizeMismatch { expected: t1.len(), found: t2.len() });
                }
                for (a, b) in t1.iter().zip(t2.iter()) {
                    self.unify(a, b)?;
                }
                Ok(())
            }

            // No unification possible
            _ => Err(TypeError::TypeMismatch { expected: t1.clone(), found: t2.clone() })
        }
    }

    /// Occurs check: prevent infinite types like τ = List<τ>
    fn occurs_in(&self, var: TypeVar, ty: &ResolvedType) -> bool {
        match ty {
            ResolvedType::TypeVar(v) => *v == var,
            ResolvedType::Function { params, return_type } => {
                params.iter().any(|p| self.occurs_in(var, p))
                    || self.occurs_in(var, return_type)
            }
            ResolvedType::Named { generics, .. } => {
                generics.iter().any(|g| self.occurs_in(var, g))
            }
            ResolvedType::Reference { inner, .. } => self.occurs_in(var, inner),
            ResolvedType::Array { element, .. } => self.occurs_in(var, element),
            ResolvedType::Tuple(ts) => ts.iter().any(|t| self.occurs_in(var, t)),
            _ => false,
        }
    }
}
```

### 2.2 Complete Borrow Checker

**Current State:** Basic ownership tracking, no NLL
**Goal:** Rust-equivalent borrow checking with NLL

#### Tasks:
```
Priority | Task                                    | Estimated | Complexity
=========|=========================================|===========|===========
P0       | Control flow graph construction         | 2 weeks   | High
P0       | Liveness analysis                       | 2 weeks   | High
P0       | Non-Lexical Lifetimes (NLL)             | 4 weeks   | Very High
P1       | Two-phase borrows                       | 1 week    | Medium
P1       | Polonius-style analysis                 | 4 weeks   | Very High
P2       | Better error diagnostics                | 2 weeks   | Medium
```

**Key Implementation - NLL:**
```rust
// compiler/src/semantics/borrow.rs - NLL implementation
pub struct NllBorrowChecker {
    /// Control flow graph
    cfg: ControlFlowGraph,
    /// Liveness information for each variable
    liveness: HashMap<String, LivenessInfo>,
    /// Active borrows at each program point
    borrows_at: HashMap<ProgramPoint, HashSet<BorrowInfo>>,
    /// Region constraints
    region_constraints: Vec<RegionConstraint>,
}

impl NllBorrowChecker {
    /// Build control flow graph from MIR
    pub fn build_cfg(&mut self, func: &MirFunction) {
        for (idx, block) in func.blocks.iter().enumerate() {
            let node = CfgNode {
                id: idx,
                instructions: block.instructions.clone(),
                terminator: block.terminator.clone(),
            };
            self.cfg.add_node(node);

            // Add edges based on terminator
            match &block.terminator {
                MirTerminator::Goto { target } => {
                    self.cfg.add_edge(idx, *target);
                }
                MirTerminator::SwitchInt { targets, otherwise, .. } => {
                    for (_, target) in targets {
                        self.cfg.add_edge(idx, *target);
                    }
                    self.cfg.add_edge(idx, *otherwise);
                }
                MirTerminator::Call { target, .. } => {
                    self.cfg.add_edge(idx, *target);
                }
                MirTerminator::Return | MirTerminator::Unreachable => {}
                MirTerminator::Unwind => {}
            }
        }
    }

    /// Compute liveness for all variables
    pub fn compute_liveness(&mut self) {
        // Iterate until fixed point
        let mut changed = true;
        while changed {
            changed = false;

            // Process blocks in reverse postorder
            for block_id in self.cfg.reverse_postorder() {
                let block = &self.cfg.nodes[block_id];

                // live_out = union of live_in of all successors
                let mut live_out: HashSet<String> = HashSet::new();
                for succ in self.cfg.successors(block_id) {
                    live_out.extend(self.liveness.get(&format!("live_in_{}", succ))
                        .map(|l| l.variables.clone())
                        .unwrap_or_default());
                }

                // live_in = (live_out - defs) ∪ uses
                let mut live_in = live_out.clone();
                for inst in block.instructions.iter().rev() {
                    match inst {
                        MirInstruction::Assign { dest, value } => {
                            // Remove def
                            live_in.remove(&format!("_{}", dest.local));
                            // Add uses from value
                            for var in self.extract_uses(value) {
                                live_in.insert(var);
                            }
                        }
                        _ => {}
                    }
                }

                let key = format!("live_in_{}", block_id);
                if self.liveness.get(&key).map(|l| &l.variables) != Some(&live_in) {
                    self.liveness.insert(key, LivenessInfo { variables: live_in });
                    changed = true;
                }
            }
        }
    }

    /// Check borrows using NLL
    pub fn check_nll(&mut self, func: &MirFunction) -> Result<(), Vec<BorrowError>> {
        self.build_cfg(func);
        self.compute_liveness();

        let mut errors = Vec::new();

        // For each borrow, compute its live range
        for block in &func.blocks {
            for inst in &block.instructions {
                if let MirInstruction::Assign { dest, value: MirRvalue::Ref { place, .. } } = inst {
                    // Borrow starts here
                    let borrow_var = format!("_{}", dest.local);
                    let borrowed_place = format!("_{}", place.local);

                    // Find all program points where borrow is live
                    let live_range = self.compute_live_range(&borrow_var);

                    // Check for conflicts
                    for point in &live_range {
                        if let Some(conflict) = self.check_conflict_at(point, &borrowed_place) {
                            errors.push(conflict);
                        }
                    }
                }
            }
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }
}
```

---

## PHASE 3: MIR & OPTIMIZATION PASSES (3-4 months)

### 3.1 Complete MIR Infrastructure

**Current State:** Types defined, builder partial
**Goal:** Full MIR with all transformations

#### Tasks:
```
Priority | Task                                    | Estimated | Complexity
=========|=========================================|===========|===========
P0       | AST → MIR lowering (complete)           | 3 weeks   | High
P0       | MIR verification pass                   | 1 week    | Medium
P1       | MIR pretty printing                     | 1 week    | Low
P1       | MIR serialization                       | 1 week    | Medium
```

**Key Implementation - AST to MIR:**
```rust
// compiler/src/mir/builder.rs - Complete lowering
impl MirBuilder {
    /// Lower a function to MIR
    pub fn lower_function(&mut self, func: &FunctionDef) -> MirFunction {
        let mut mir_func = MirFunction {
            name: func.name.name.clone(),
            params: Vec::new(),
            return_type: self.lower_type(&func.return_type),
            blocks: Vec::new(),
            locals: Vec::new(),
            karaka_hints: HashMap::new(),
        };

        // Lower parameters
        for (idx, param) in func.params.iter().enumerate() {
            let mir_param = MirParam {
                index: idx,
                ty: self.lower_type(&Some(param.ty.clone())),
                karaka: param.karaka.clone(),
            };
            mir_func.params.push(mir_param);

            // Create local for parameter
            let local_idx = self.create_local(&mir_func, param.name.name.clone(), mir_param.ty.clone());

            // Add kāraka hint if present
            if let Some(karaka) = &param.karaka {
                mir_func.karaka_hints.insert(local_idx, KarakaHint {
                    karaka: karaka.clone(),
                    register_class: self.karaka_to_register_class(karaka),
                });
            }
        }

        // Create entry block
        let entry_block = self.create_block();
        self.current_block = Some(entry_block);

        // Lower function body
        self.lower_block(&func.body, &mut mir_func);

        // Ensure function has return
        self.ensure_terminator(&mut mir_func);

        mir_func
    }

    /// Lower a statement to MIR
    fn lower_stmt(&mut self, stmt: &Stmt, func: &mut MirFunction) {
        match stmt {
            Stmt::Let { name, ty, value, .. } => {
                let mir_ty = self.lower_type(ty);
                let local = self.create_local(func, name.name.clone(), mir_ty);

                if let Some(init) = value {
                    let rvalue = self.lower_expr_to_rvalue(init, func);
                    self.emit(MirInstruction::Assign {
                        dest: MirPlace::local(local),
                        value: rvalue,
                    }, func);
                }
            }

            Stmt::Assign { target, value, .. } => {
                let place = self.lower_expr_to_place(target, func);
                let rvalue = self.lower_expr_to_rvalue(value, func);
                self.emit(MirInstruction::Assign { dest: place, value: rvalue }, func);
            }

            Stmt::If { condition, then_block, else_block, .. } => {
                let cond = self.lower_expr_to_operand(condition, func);

                let then_bb = self.create_block();
                let else_bb = self.create_block();
                let join_bb = self.create_block();

                // Emit conditional branch
                self.terminate_block(MirTerminator::SwitchInt {
                    discriminant: cond,
                    targets: vec![(1, then_bb)], // true → then
                    otherwise: else_bb,          // false → else
                }, func);

                // Lower then branch
                self.current_block = Some(then_bb);
                self.lower_block(then_block, func);
                self.terminate_block(MirTerminator::Goto { target: join_bb }, func);

                // Lower else branch
                self.current_block = Some(else_bb);
                if let Some(else_b) = else_block {
                    self.lower_block(else_b, func);
                }
                self.terminate_block(MirTerminator::Goto { target: join_bb }, func);

                self.current_block = Some(join_bb);
            }

            Stmt::While { condition, body, .. } => {
                let header_bb = self.create_block();
                let body_bb = self.create_block();
                let exit_bb = self.create_block();

                // Jump to header
                self.terminate_block(MirTerminator::Goto { target: header_bb }, func);

                // Header: check condition
                self.current_block = Some(header_bb);
                let cond = self.lower_expr_to_operand(condition, func);
                self.terminate_block(MirTerminator::SwitchInt {
                    discriminant: cond,
                    targets: vec![(1, body_bb)],
                    otherwise: exit_bb,
                }, func);

                // Body
                self.current_block = Some(body_bb);
                self.lower_block(body, func);
                self.terminate_block(MirTerminator::Goto { target: header_bb }, func);

                self.current_block = Some(exit_bb);
            }

            Stmt::Return { value, .. } => {
                if let Some(expr) = value {
                    let operand = self.lower_expr_to_operand(expr, func);
                    // Store in return place
                    self.emit(MirInstruction::Assign {
                        dest: MirPlace::local(0), // _0 is return place
                        value: MirRvalue::Use(operand),
                    }, func);
                }
                self.terminate_block(MirTerminator::Return, func);
            }

            _ => {}
        }
    }
}
```

### 3.2 Implement Real Optimization Passes (Astras)

**Current State:** All passes are stubs with `// TODO`
**Goal:** Working optimization passes

#### Priority Passes:
```
Priority | Astra Name      | Pass                      | Estimated | Impact
=========|=================|===========================|===========|=======
P0       | Brahmastra      | Dead Code Elimination     | 2 weeks   | High
P0       | Varunastra      | Copy Propagation          | 2 weeks   | High
P0       | Agneyastra      | Constant Folding          | 2 weeks   | High
P1       | Vayuastra       | CFG Simplification        | 1 week    | Medium
P1       | Nagastra        | Common Subexpr Elim       | 2 weeks   | Medium
P2       | Sudarshana      | Loop Invariant Code Motion| 3 weeks   | High
P2       | Pashupatastra   | Function Inlining         | 3 weeks   | High
```

**Key Implementation - Dead Code Elimination (Brahmastra):**
```rust
// compiler/src/mir/passes.rs - Real implementation
pub struct DeadCodeElimination {
    /// Variables that are used
    used: HashSet<usize>,
    /// Instructions to remove
    dead: HashSet<(usize, usize)>, // (block, instruction)
}

impl MirPass for DeadCodeElimination {
    fn name(&self) -> &'static str {
        "brahmastra_dce"
    }

    fn run(&mut self, func: &mut MirFunction) {
        // Phase 1: Mark all variables that are used
        self.mark_used(func);

        // Phase 2: Remove instructions whose results are unused
        self.remove_dead(func);

        // Phase 3: Remove unreachable blocks
        self.remove_unreachable_blocks(func);
    }
}

impl DeadCodeElimination {
    fn mark_used(&mut self, func: &MirFunction) {
        // Start with return value and terminators
        self.used.insert(0); // Return place always used

        // Mark variables used in terminators
        for block in &func.blocks {
            match &block.terminator {
                MirTerminator::SwitchInt { discriminant, .. } => {
                    self.mark_operand_used(discriminant);
                }
                MirTerminator::Call { func: f, args, .. } => {
                    self.mark_operand_used(f);
                    for arg in args {
                        self.mark_operand_used(arg);
                    }
                }
                _ => {}
            }
        }

        // Iterate until fixed point
        let mut changed = true;
        while changed {
            changed = false;

            for block in &func.blocks {
                for inst in &block.instructions {
                    match inst {
                        MirInstruction::Assign { dest, value } => {
                            // If dest is used, mark all operands in value as used
                            if self.used.contains(&dest.local) {
                                if self.mark_rvalue_used(value) {
                                    changed = true;
                                }
                            }
                        }
                        MirInstruction::Store { ptr, value } => {
                            // Stores are always live (side effects)
                            self.mark_operand_used(ptr);
                            self.mark_operand_used(value);
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn remove_dead(&mut self, func: &mut MirFunction) {
        for block in &mut func.blocks {
            block.instructions.retain(|inst| {
                match inst {
                    MirInstruction::Assign { dest, .. } => {
                        self.used.contains(&dest.local)
                    }
                    // Keep all other instructions (stores, drops, etc.)
                    _ => true,
                }
            });
        }
    }

    fn remove_unreachable_blocks(&mut self, func: &mut MirFunction) {
        // Build reachability from entry block
        let mut reachable = HashSet::new();
        let mut worklist = vec![0usize];

        while let Some(block_id) = worklist.pop() {
            if reachable.contains(&block_id) {
                continue;
            }
            reachable.insert(block_id);

            if let Some(block) = func.blocks.get(block_id) {
                match &block.terminator {
                    MirTerminator::Goto { target } => {
                        worklist.push(*target);
                    }
                    MirTerminator::SwitchInt { targets, otherwise, .. } => {
                        for (_, target) in targets {
                            worklist.push(*target);
                        }
                        worklist.push(*otherwise);
                    }
                    MirTerminator::Call { target, .. } => {
                        worklist.push(*target);
                    }
                    _ => {}
                }
            }
        }

        // Remove unreachable blocks
        let unreachable_ids: Vec<_> = (0..func.blocks.len())
            .filter(|id| !reachable.contains(id))
            .collect();

        for id in unreachable_ids.into_iter().rev() {
            func.blocks.remove(id);
        }
    }
}
```

---

## PHASE 4: CODE GENERATION (4-5 months)

### 4.1 Complete Assembly Emission

**Current State:** Basic structure, many instructions missing
**Goal:** Complete x86-64/ARM64/RISC-V emission

#### Tasks:
```
Priority | Task                                    | Estimated | Complexity
=========|=========================================|===========|===========
P0       | Complete x86-64 instruction emission    | 4 weeks   | High
P0       | Register allocation (linear scan)       | 3 weeks   | High
P0       | Stack frame layout                      | 2 weeks   | Medium
P1       | Calling convention handling             | 2 weeks   | Medium
P1       | ARM64 backend                           | 4 weeks   | High
P2       | RISC-V backend                          | 4 weeks   | High
P2       | SIMD instruction emission               | 3 weeks   | High
```

**Key Implementation - Linear Scan Register Allocation:**
```rust
// compiler/src/codegen/regalloc.rs
pub struct LinearScanAllocator {
    /// Live intervals for each virtual register
    intervals: Vec<LiveInterval>,
    /// Available physical registers
    available: Vec<PhysReg>,
    /// Active intervals (sorted by end point)
    active: BinaryHeap<Reverse<(usize, usize)>>, // (end, interval_id)
    /// Spilled intervals
    spilled: HashSet<usize>,
    /// Assignment: virtual → physical
    assignment: HashMap<usize, PhysReg>,
    /// Stack slots for spills
    stack_slots: HashMap<usize, i32>,
    /// Next stack slot offset
    next_stack_slot: i32,
}

impl LinearScanAllocator {
    pub fn allocate(&mut self, func: &MirFunction) -> AllocationResult {
        // 1. Compute live intervals
        self.compute_live_intervals(func);

        // 2. Sort intervals by start point
        self.intervals.sort_by_key(|i| i.start);

        // 3. Process each interval
        for (idx, interval) in self.intervals.iter().enumerate() {
            // Expire old intervals
            self.expire_old_intervals(interval.start);

            if self.available.is_empty() {
                // Need to spill
                self.spill_at_interval(idx);
            } else {
                // Allocate register
                let reg = self.available.pop().unwrap();
                self.assignment.insert(interval.vreg, reg);

                // Add to active set
                self.active.push(Reverse((interval.end, idx)));
            }
        }

        AllocationResult {
            assignment: self.assignment.clone(),
            spilled: self.spilled.clone(),
            stack_slots: self.stack_slots.clone(),
            stack_size: self.next_stack_slot.abs() as usize,
        }
    }

    fn compute_live_intervals(&mut self, func: &MirFunction) {
        // For each block, track variable definitions and uses
        for (block_idx, block) in func.blocks.iter().enumerate() {
            for (inst_idx, inst) in block.instructions.iter().enumerate() {
                let point = block_idx * 1000 + inst_idx;

                match inst {
                    MirInstruction::Assign { dest, value } => {
                        // Definition
                        self.extend_interval(dest.local, point, true);

                        // Uses
                        self.mark_rvalue_uses(value, point);
                    }
                    MirInstruction::Store { ptr, value } => {
                        self.mark_operand_use(ptr, point);
                        self.mark_operand_use(value, point);
                    }
                    _ => {}
                }
            }
        }
    }

    fn expire_old_intervals(&mut self, current: usize) {
        while let Some(&Reverse((end, idx))) = self.active.peek() {
            if end >= current {
                break;
            }
            self.active.pop();

            // Return register to available pool
            if let Some(&reg) = self.assignment.get(&self.intervals[idx].vreg) {
                self.available.push(reg);
            }
        }
    }

    fn spill_at_interval(&mut self, idx: usize) {
        // Spill the interval with the longest remaining range
        if let Some(&Reverse((_, spill_idx))) = self.active.peek() {
            if self.intervals[spill_idx].end > self.intervals[idx].end {
                // Spill the active interval, give its register to current
                let vreg = self.intervals[spill_idx].vreg;
                let reg = self.assignment.remove(&vreg).unwrap();

                // Mark as spilled
                self.spilled.insert(spill_idx);
                self.stack_slots.insert(vreg, self.allocate_stack_slot());

                // Give register to current interval
                self.assignment.insert(self.intervals[idx].vreg, reg);
                self.active.pop();
                self.active.push(Reverse((self.intervals[idx].end, idx)));
                return;
            }
        }

        // Spill current interval
        self.spilled.insert(idx);
        self.stack_slots.insert(
            self.intervals[idx].vreg,
            self.allocate_stack_slot()
        );
    }
}
```

### 4.2 Linker Integration

**Current State:** Stub
**Goal:** Real object file generation and linking

#### Implementation:
```rust
// compiler/src/codegen/linker.rs
pub struct Linker {
    /// Object files to link
    objects: Vec<PathBuf>,
    /// Libraries to link
    libraries: Vec<String>,
    /// Output path
    output: PathBuf,
    /// Target triple
    target: Target,
}

impl Linker {
    pub fn link(&self) -> Result<(), LinkError> {
        match self.target {
            Target::X86_64 => self.link_elf_x86_64(),
            Target::AArch64 => self.link_elf_aarch64(),
            Target::RiscV64 => self.link_elf_riscv64(),
        }
    }

    fn link_elf_x86_64(&self) -> Result<(), LinkError> {
        // Use system linker (ld) or lld
        let mut cmd = Command::new("ld");

        // Add runtime
        cmd.arg("-dynamic-linker")
           .arg("/lib64/ld-linux-x86-64.so.2");

        // Add CRT
        cmd.arg("/usr/lib/x86_64-linux-gnu/crt1.o")
           .arg("/usr/lib/x86_64-linux-gnu/crti.o");

        // Add our objects
        for obj in &self.objects {
            cmd.arg(obj);
        }

        // Add libraries
        for lib in &self.libraries {
            cmd.arg(format!("-l{}", lib));
        }

        // Add libc and CRT end
        cmd.arg("-lc")
           .arg("/usr/lib/x86_64-linux-gnu/crtn.o");

        // Output
        cmd.arg("-o").arg(&self.output);

        let output = cmd.output()?;
        if output.status.success() {
            Ok(())
        } else {
            Err(LinkError::LinkerFailed(
                String::from_utf8_lossy(&output.stderr).to_string()
            ))
        }
    }
}
```

---

## PHASE 5: RUNTIME & STANDARD LIBRARY (3-4 months)

### 5.1 Runtime Implementation

**Current State:** Basic allocator stub
**Goal:** Full runtime with Pancha Kosha memory

#### Tasks:
```
Priority | Task                                    | Estimated | Complexity
=========|=========================================|===========|===========
P0       | Arena allocator (Annamaya)              | 2 weeks   | Medium
P0       | Stack allocator (Prāṇamaya)             | 1 week    | Medium
P1       | Object system (Manomaya)                | 2 weeks   | Medium
P1       | Heap with GC support (Vijñānamaya)      | 4 weeks   | High
P2       | Static memory (Ānandamaya)              | 1 week    | Low
```

### 5.2 Standard Library

**Current State:** Stubs with Sanskrit names
**Goal:** Working implementations

#### Priority:
```
Priority | Module              | Contents                  | Estimated
=========|=====================|===========================|==========
P0       | sankhya (numbers)   | Integer/float operations  | 2 weeks
P0       | sutra (strings)     | UTF-8 string handling     | 3 weeks
P0       | suci (collections)  | Vec, HashMap, HashSet     | 4 weeks
P1       | smriti (memory)     | Allocators, Box, Rc, Arc  | 3 weeks
P1       | tantu (threads)     | Threading, channels       | 4 weeks
P1       | tala (sync)         | Mutex, RwLock, atomics    | 3 weeks
P2       | jala (networking)   | TCP/UDP, HTTP basics      | 4 weeks
```

---

## TIMELINE SUMMARY

```
Phase    | Description                    | Duration  | Cumulative
=========|================================|===========|===========
Phase 1  | Foundation (Lexer/Parser)      | 3-4 months| 3-4 months
Phase 2  | Safety (Types/Borrows)         | 4-5 months| 7-9 months
Phase 3  | Optimization (MIR/Passes)      | 3-4 months| 10-13 months
Phase 4  | Codegen (Assembly/Linking)     | 4-5 months| 14-18 months
Phase 5  | Runtime/Stdlib                 | 3-4 months| 17-22 months
Buffer   | Integration & Polish           | 2-3 months| 19-25 months
```

**Total Estimated Time: 20-24 months** (with one experienced full-time developer)

---

## MILESTONES & DELIVERABLES

### Milestone 1: Self-Hosting Lexer (Month 4)
- Lexer can tokenize its own source code
- All Sanskrit morphological features working
- Benchmark: 1M tokens/second

### Milestone 2: Type-Safe Subset (Month 9)
- Complete type inference for a subset
- Borrow checker passes simple programs
- Can compile fibonacci without runtime errors

### Milestone 3: End-to-End Compilation (Month 14)
- Full pipeline: .jag → .exe
- All optimization passes implemented
- Benchmark: <1s for 10K LOC

### Milestone 4: Standard Library (Month 18)
- Core modules implemented and tested
- Can build non-trivial programs
- Documentation complete

### Milestone 5: Production Ready (Month 24)
- Self-hosting compiler
- All targets (x86-64, ARM64, RISC-V)
- Performance target: 3× faster than C baseline

---

## IMMEDIATE NEXT STEPS (Week 1-2)

1. **Fix MIR passes** - Replace all `// TODO` stubs with real implementations
2. **Complete unification** - Finish Algorithm W in type checker
3. **Add comprehensive tests** - Target 80% code coverage
4. **Benchmark realistic programs** - Not scaffolding, real compilation

---

## APPENDIX: KEY TECHNICAL DECISIONS

### A1. Why Not Use LLVM?

The specification calls for "direct assembly, no C middleman." While LLVM would be faster to implement, we lose:
- Kāraka-guided register allocation (LLVM doesn't understand Sanskrit semantics)
- Philosophy-based optimization ordering
- Educational value of understanding codegen
- 3.35× performance claim requires our own optimizations

**Decision:** Keep direct assembly, but consider LLVM backend as optional tier-2 target.

### A2. Borrow Checker Strategy

Options:
1. **NLL (Non-Lexical Lifetimes)** - Current Rust approach
2. **Polonius** - Next-gen Rust approach
3. **Affine types** - Simpler, less expressive

**Decision:** Implement NLL first (matches Rust semantics), add Polonius later.

### A3. Optimization Pass Ordering

The Sāṃkhya tattva ordering provides a framework:
1. Buddhi (high-level analysis) → Dead code, constant prop
2. Ahaṃkāra (boundaries) → Inlining
3. Manas (control) → CFG simplification
4. Indriyas (I/O) → Memory optimization
5. Tanmātras (layout) → Field reordering

This is philosophically sound AND practically efficient.

---

*End of Implementation Plan v2*
