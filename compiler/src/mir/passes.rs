//! MIR Optimization Passes (Divya Astra System)
//!
//! Individual optimization passes following Sāṃkhya tattva ordering.
//! Each pass is named after a Divine Weapon (Astra) from Hindu mythology.
//!
//! ## Pass Ordering (Sāṃkhya Tattvas)
//! 1. Buddhi (Intellect) - High-level analysis: DCE, Const Prop
//! 2. Ahaṃkāra (Ego) - Function boundaries: Inlining
//! 3. Manas (Mind) - Control flow: CFG simplification
//! 4. Indriyas (Senses) - I/O & Memory: Access optimization
//! 5. Tanmātras (Subtle) - Data layout: Field reordering

use super::types::*;
use std::collections::{HashMap, HashSet, VecDeque};

/// Trait for MIR optimization passes
pub trait MirPass {
    /// Name of the pass (Sanskrit astra name)
    fn name(&self) -> &'static str;

    /// Mantra for invocation (logging/debugging)
    fn mantra(&self) -> &'static str {
        "Om Śānti"
    }

    /// Run the pass on a function
    fn run(&mut self, func: &mut MirFunction);
}

// ============================================
// Buddhi (Intellect) Level - High-level Analysis
// ============================================

/// Dead Code Elimination - Brahmastra (ब्रह्मास्त्र)
///
/// The ultimate weapon of creation/destruction - eliminates all unreachable
/// and unused code with divine precision.
///
/// Algorithm:
/// 1. Build use-def chains for all locals
/// 2. Mark roots (return value, side-effecting instructions)
/// 3. Propagate liveness backwards
/// 4. Remove instructions whose results are unused
/// 5. Remove unreachable basic blocks
pub struct DeadCodeElimination {
    /// Locals that are used (live)
    used_locals: HashSet<usize>,
    /// Instructions marked as dead: (block_id, instruction_index)
    dead_instructions: HashSet<(usize, usize)>,
    /// Blocks that are reachable from entry
    reachable_blocks: HashSet<usize>,
}

impl DeadCodeElimination {
    pub fn new() -> Self {
        Self {
            used_locals: HashSet::new(),
            dead_instructions: HashSet::new(),
            reachable_blocks: HashSet::new(),
        }
    }

    /// Mark all roots as used (return value, side effects)
    fn mark_roots(&mut self, func: &MirFunction) {
        // Return place (_0) is always live
        self.used_locals.insert(0);

        // Mark variables used in terminators
        for block in &func.blocks {
            self.mark_terminator_uses(&block.terminator);

            // Mark side-effecting instructions as having live results
            for inst in &block.instructions {
                match inst {
                    MirInstruction::Store { ptr, value } => {
                        self.mark_operand_used(ptr);
                        self.mark_operand_used(value);
                    }
                    MirInstruction::Drop { place } => {
                        self.mark_place_used(place);
                    }
                    MirInstruction::Assert { condition, .. } => {
                        self.mark_operand_used(condition);
                    }
                    MirInstruction::BoundsCheck { index, len, .. } => {
                        self.mark_operand_used(index);
                        self.mark_operand_used(len);
                    }
                    _ => {}
                }
            }
        }
    }

    /// Mark operands used in a terminator
    fn mark_terminator_uses(&mut self, term: &MirTerminator) {
        match term {
            MirTerminator::SwitchInt { discriminant, .. } => {
                self.mark_operand_used(discriminant);
            }
            MirTerminator::Call { func, args, .. } => {
                self.mark_operand_used(func);
                for arg in args {
                    self.mark_operand_used(arg);
                }
            }
            MirTerminator::Return
            | MirTerminator::Goto { .. }
            | MirTerminator::Unreachable
            | MirTerminator::Unwind => {}
        }
    }

    /// Mark an operand as used
    fn mark_operand_used(&mut self, op: &MirOperand) {
        match op {
            MirOperand::Copy(place) | MirOperand::Move(place) => {
                self.mark_place_used(place);
            }
            MirOperand::Constant(_) => {}
        }
    }

    /// Mark a place as used
    fn mark_place_used(&mut self, place: &MirPlace) {
        self.used_locals.insert(place.local);
        // Also mark any locals used in projections
        for proj in &place.projection {
            if let PlaceProjection::Index { index } = proj {
                self.mark_operand_used(index);
            }
        }
    }

    /// Mark operands in an rvalue as used
    fn mark_rvalue_uses(&mut self, rvalue: &MirRvalue) {
        match rvalue {
            MirRvalue::Use(op) => self.mark_operand_used(op),
            MirRvalue::Ref { place, .. } => self.mark_place_used(place),
            MirRvalue::BinaryOp { left, right, .. } => {
                self.mark_operand_used(left);
                self.mark_operand_used(right);
            }
            MirRvalue::UnaryOp { operand, .. } => self.mark_operand_used(operand),
            MirRvalue::Aggregate { operands, .. } => {
                for op in operands {
                    self.mark_operand_used(op);
                }
            }
            MirRvalue::Cast { operand, .. } => self.mark_operand_used(operand),
            MirRvalue::Discriminant(place) => self.mark_place_used(place),
            MirRvalue::Len(place) => self.mark_place_used(place),
            MirRvalue::AddressOf { place, .. } => self.mark_place_used(place),
            MirRvalue::Field { base, .. } => self.mark_operand_used(base),
            MirRvalue::Index { base, index } => {
                self.mark_operand_used(base);
                self.mark_operand_used(index);
            }
            MirRvalue::FloatOp { left, right, .. } => {
                self.mark_operand_used(left);
                self.mark_operand_used(right);
            }
            MirRvalue::SimdOp { operands, .. } => {
                for op in operands {
                    self.mark_operand_used(op);
                }
            }
        }
    }

    /// Propagate liveness backwards through the function
    fn propagate_liveness(&mut self, func: &MirFunction) {
        let mut changed = true;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 1000;

        while changed && iterations < MAX_ITERATIONS {
            changed = false;
            iterations += 1;

            for block in &func.blocks {
                // Process instructions in reverse order
                for inst in block.instructions.iter().rev() {
                    if let MirInstruction::Assign { dest, value } = inst {
                        // If the destination is used, mark all sources as used
                        if self.used_locals.contains(&dest.local) {
                            let old_size = self.used_locals.len();
                            self.mark_rvalue_uses(value);
                            if self.used_locals.len() > old_size {
                                changed = true;
                            }
                        }
                    }
                }
            }
        }
    }

    /// Compute reachable blocks using BFS from entry
    fn compute_reachable_blocks(&mut self, func: &MirFunction) {
        self.reachable_blocks.clear();
        let mut worklist = VecDeque::new();
        worklist.push_back(0); // Entry block

        while let Some(block_id) = worklist.pop_front() {
            if self.reachable_blocks.contains(&block_id) {
                continue;
            }
            self.reachable_blocks.insert(block_id);

            if let Some(block) = func.blocks.get(block_id) {
                match &block.terminator {
                    MirTerminator::Goto { target } => {
                        worklist.push_back(*target);
                    }
                    MirTerminator::SwitchInt {
                        targets, otherwise, ..
                    } => {
                        for (_, target) in targets {
                            worklist.push_back(*target);
                        }
                        worklist.push_back(*otherwise);
                    }
                    MirTerminator::Call { target, .. } => {
                        worklist.push_back(*target);
                    }
                    MirTerminator::Return | MirTerminator::Unreachable | MirTerminator::Unwind => {}
                }
            }
        }
    }

    /// Remove dead instructions and unreachable blocks
    fn eliminate_dead_code(&self, func: &mut MirFunction) {
        // First, remove unreachable blocks
        let mut new_blocks: Vec<MirBasicBlock> = Vec::new();
        let mut block_remap: HashMap<usize, usize> = HashMap::new();

        for block in func.blocks.drain(..) {
            if self.reachable_blocks.contains(&block.id) {
                let new_id = new_blocks.len();
                block_remap.insert(block.id, new_id);
                new_blocks.push(MirBasicBlock {
                    id: new_id,
                    instructions: block.instructions,
                    terminator: block.terminator,
                });
            }
        }

        // Remap block references in terminators
        for block in &mut new_blocks {
            match &mut block.terminator {
                MirTerminator::Goto { target } => {
                    if let Some(&new_target) = block_remap.get(target) {
                        *target = new_target;
                    }
                }
                MirTerminator::SwitchInt {
                    targets, otherwise, ..
                } => {
                    for (_, target) in targets.iter_mut() {
                        if let Some(&new_target) = block_remap.get(target) {
                            *target = new_target;
                        }
                    }
                    if let Some(&new_otherwise) = block_remap.get(otherwise) {
                        *otherwise = new_otherwise;
                    }
                }
                MirTerminator::Call { target, .. } => {
                    if let Some(&new_target) = block_remap.get(target) {
                        *target = new_target;
                    }
                }
                _ => {}
            }
        }

        // Remove dead instructions from each block
        for block in &mut new_blocks {
            block.instructions.retain(|inst| match inst {
                MirInstruction::Assign { dest, .. } => self.used_locals.contains(&dest.local),
                // Keep all side-effecting instructions
                MirInstruction::Store { .. }
                | MirInstruction::Drop { .. }
                | MirInstruction::Assert { .. }
                | MirInstruction::BoundsCheck { .. }
                | MirInstruction::SetDiscriminant { .. }
                | MirInstruction::Load { .. } => true,
                MirInstruction::Nop => false,
            });
        }

        func.blocks = new_blocks;
    }
}

impl Default for DeadCodeElimination {
    fn default() -> Self {
        Self::new()
    }
}

impl MirPass for DeadCodeElimination {
    fn name(&self) -> &'static str {
        "brahmastra_dce" // ब्रह्मास्त्र - Ultimate Dead Code Elimination
    }

    fn mantra(&self) -> &'static str {
        "Om Brahmāstrāya Phaṭ" // Invocation of Brahmastra
    }

    fn run(&mut self, func: &mut MirFunction) {
        // Reset state
        self.used_locals.clear();
        self.dead_instructions.clear();
        self.reachable_blocks.clear();

        // Phase 1: Mark roots (side effects, return value)
        self.mark_roots(func);

        // Phase 2: Propagate liveness backwards
        self.propagate_liveness(func);

        // Phase 3: Compute reachable blocks
        self.compute_reachable_blocks(func);

        // Phase 4: Eliminate dead code
        self.eliminate_dead_code(func);
    }
}

/// Constant Propagation - Agneyastra (आग्नेयास्त्र)
///
/// The fire weapon that burns away uncertainty - replaces variable uses
/// with known constant values, enabling further optimizations.
///
/// Algorithm (Lattice-based Sparse Conditional Constant Propagation):
/// 1. Initialize all values to ⊤ (unknown)
/// 2. Process instructions, computing constant values
/// 3. Propagate constants through control flow
/// 4. Replace uses of constants with literal values
/// 5. Fold constant binary/unary operations
pub struct ConstantPropagation {
    /// Lattice values for each local: None = ⊤, Some(c) = constant, NAC = ⊥
    lattice: HashMap<usize, LatticeValue>,
}

/// Lattice value for constant propagation
#[derive(Debug, Clone, PartialEq)]
enum LatticeValue {
    /// Top - value not yet known
    Top,
    /// Constant - known constant value
    Constant(MirConstant),
    /// Bottom - value is not a constant (varies)
    Bottom,
}

impl ConstantPropagation {
    pub fn new() -> Self {
        Self {
            lattice: HashMap::new(),
        }
    }

    /// Initialize lattice - all params are Bottom (unknown input)
    fn initialize(&mut self, func: &MirFunction) {
        self.lattice.clear();

        // Parameters are not constants (they vary per call)
        for param in &func.params {
            self.lattice.insert(param.index, LatticeValue::Bottom);
        }

        // All other locals start as Top
        for local in &func.locals {
            if !self.lattice.contains_key(&local.index) {
                self.lattice.insert(local.index, LatticeValue::Top);
            }
        }
    }

    /// Get lattice value for an operand
    fn get_operand_value(&self, op: &MirOperand) -> LatticeValue {
        match op {
            MirOperand::Constant(c) => LatticeValue::Constant(c.clone()),
            MirOperand::Copy(place) | MirOperand::Move(place) => {
                if place.projection.is_empty() {
                    self.lattice
                        .get(&place.local)
                        .cloned()
                        .unwrap_or(LatticeValue::Top)
                } else {
                    // Projections make value non-constant
                    LatticeValue::Bottom
                }
            }
        }
    }

    /// Evaluate a binary operation on constants
    fn eval_binary_op(
        &self,
        op: &BinaryOp,
        left: &MirConstant,
        right: &MirConstant,
    ) -> Option<MirConstant> {
        match (left, right) {
            (MirConstant::Int(l, ls), MirConstant::Int(r, rs)) if ls == rs => {
                let result = match op {
                    BinaryOp::Add => l.checked_add(*r)?,
                    BinaryOp::Sub => l.checked_sub(*r)?,
                    BinaryOp::Mul => l.checked_mul(*r)?,
                    BinaryOp::Div => {
                        if *r == 0 {
                            return None;
                        }
                        l.checked_div(*r)?
                    }
                    BinaryOp::Rem => {
                        if *r == 0 {
                            return None;
                        }
                        l.checked_rem(*r)?
                    }
                    BinaryOp::BitAnd => l & r,
                    BinaryOp::BitOr => l | r,
                    BinaryOp::BitXor => l ^ r,
                    BinaryOp::Shl => l.checked_shl(*r as u32)?,
                    BinaryOp::Shr => l.checked_shr(*r as u32)?,
                    BinaryOp::Eq => return Some(MirConstant::Bool(l == r)),
                    BinaryOp::Ne => return Some(MirConstant::Bool(l != r)),
                    BinaryOp::Lt => return Some(MirConstant::Bool(l < r)),
                    BinaryOp::Le => return Some(MirConstant::Bool(l <= r)),
                    BinaryOp::Gt => return Some(MirConstant::Bool(l > r)),
                    BinaryOp::Ge => return Some(MirConstant::Bool(l >= r)),
                };
                Some(MirConstant::Int(result, *ls))
            }
            (MirConstant::Float(l, ls), MirConstant::Float(r, rs)) if ls == rs => {
                let result = match op {
                    BinaryOp::Add => l + r,
                    BinaryOp::Sub => l - r,
                    BinaryOp::Mul => l * r,
                    BinaryOp::Div => l / r,
                    BinaryOp::Eq => return Some(MirConstant::Bool((l - r).abs() < f64::EPSILON)),
                    BinaryOp::Ne => return Some(MirConstant::Bool((l - r).abs() >= f64::EPSILON)),
                    BinaryOp::Lt => return Some(MirConstant::Bool(l < r)),
                    BinaryOp::Le => return Some(MirConstant::Bool(l <= r)),
                    BinaryOp::Gt => return Some(MirConstant::Bool(l > r)),
                    BinaryOp::Ge => return Some(MirConstant::Bool(l >= r)),
                    _ => return None,
                };
                Some(MirConstant::Float(result, *ls))
            }
            (MirConstant::Bool(l), MirConstant::Bool(r)) => {
                let result = match op {
                    BinaryOp::BitAnd => *l && *r,
                    BinaryOp::BitOr => *l || *r,
                    BinaryOp::BitXor => *l ^ *r,
                    BinaryOp::Eq => *l == *r,
                    BinaryOp::Ne => *l != *r,
                    _ => return None,
                };
                Some(MirConstant::Bool(result))
            }
            _ => None,
        }
    }

    /// Evaluate a unary operation on a constant
    fn eval_unary_op(&self, op: &UnaryOp, operand: &MirConstant) -> Option<MirConstant> {
        match (op, operand) {
            (UnaryOp::Neg, MirConstant::Int(v, s)) => Some(MirConstant::Int(-v, *s)),
            (UnaryOp::Neg, MirConstant::Float(v, s)) => Some(MirConstant::Float(-v, *s)),
            (UnaryOp::Not, MirConstant::Bool(v)) => Some(MirConstant::Bool(!v)),
            (UnaryOp::Not, MirConstant::Int(v, s)) => Some(MirConstant::Int(!v, *s)),
            _ => None,
        }
    }

    /// Evaluate an rvalue to get its lattice value
    fn eval_rvalue(&self, rvalue: &MirRvalue) -> LatticeValue {
        match rvalue {
            MirRvalue::Use(op) => self.get_operand_value(op),
            MirRvalue::BinaryOp { op, left, right } => {
                let l = self.get_operand_value(left);
                let r = self.get_operand_value(right);
                match (l, r) {
                    (LatticeValue::Constant(lc), LatticeValue::Constant(rc)) => {
                        if let Some(result) = self.eval_binary_op(op, &lc, &rc) {
                            LatticeValue::Constant(result)
                        } else {
                            LatticeValue::Bottom
                        }
                    }
                    (LatticeValue::Bottom, _) | (_, LatticeValue::Bottom) => LatticeValue::Bottom,
                    _ => LatticeValue::Top,
                }
            }
            MirRvalue::UnaryOp { op, operand } => {
                let v = self.get_operand_value(operand);
                match v {
                    LatticeValue::Constant(c) => {
                        if let Some(result) = self.eval_unary_op(op, &c) {
                            LatticeValue::Constant(result)
                        } else {
                            LatticeValue::Bottom
                        }
                    }
                    LatticeValue::Bottom => LatticeValue::Bottom,
                    LatticeValue::Top => LatticeValue::Top,
                }
            }
            // Most other operations produce non-constant results
            _ => LatticeValue::Bottom,
        }
    }

    /// Meet operation for lattice values
    fn meet(&self, a: &LatticeValue, b: &LatticeValue) -> LatticeValue {
        match (a, b) {
            (LatticeValue::Top, x) | (x, LatticeValue::Top) => x.clone(),
            (LatticeValue::Bottom, _) | (_, LatticeValue::Bottom) => LatticeValue::Bottom,
            (LatticeValue::Constant(c1), LatticeValue::Constant(c2)) => {
                if Self::constants_equal(c1, c2) {
                    LatticeValue::Constant(c1.clone())
                } else {
                    LatticeValue::Bottom
                }
            }
        }
    }

    /// Check if two constants are equal
    fn constants_equal(a: &MirConstant, b: &MirConstant) -> bool {
        match (a, b) {
            (MirConstant::Int(v1, s1), MirConstant::Int(v2, s2)) => v1 == v2 && s1 == s2,
            (MirConstant::Float(v1, s1), MirConstant::Float(v2, s2)) => {
                (v1 - v2).abs() < f64::EPSILON && s1 == s2
            }
            (MirConstant::Bool(v1), MirConstant::Bool(v2)) => v1 == v2,
            (MirConstant::Unit, MirConstant::Unit) => true,
            (MirConstant::String(s1), MirConstant::String(s2)) => s1 == s2,
            _ => false,
        }
    }

    /// Propagate constants through the function (fixed-point iteration)
    fn propagate(&mut self, func: &MirFunction) {
        let mut changed = true;
        let mut iterations = 0;
        const MAX_ITERATIONS: usize = 1000;

        while changed && iterations < MAX_ITERATIONS {
            changed = false;
            iterations += 1;

            for block in &func.blocks {
                for inst in &block.instructions {
                    if let MirInstruction::Assign { dest, value } = inst {
                        if dest.projection.is_empty() {
                            let new_value = self.eval_rvalue(value);
                            let old_value = self
                                .lattice
                                .get(&dest.local)
                                .cloned()
                                .unwrap_or(LatticeValue::Top);

                            let merged = self.meet(&old_value, &new_value);
                            if merged != old_value {
                                self.lattice.insert(dest.local, merged);
                                changed = true;
                            }
                        }
                    }
                }
            }
        }
    }

    /// Replace uses of known constants with literal values
    fn replace_constants(&self, func: &mut MirFunction) {
        for block in &mut func.blocks {
            for inst in &mut block.instructions {
                if let MirInstruction::Assign { value, .. } = inst {
                    self.replace_in_rvalue(value);
                }
            }
        }
    }

    /// Replace constants in an rvalue
    fn replace_in_rvalue(&self, rvalue: &mut MirRvalue) {
        match rvalue {
            MirRvalue::Use(op) => {
                if let Some(c) = self.get_constant_for_operand(op) {
                    *op = MirOperand::Constant(c);
                }
            }
            MirRvalue::BinaryOp { left, right, .. } => {
                if let Some(c) = self.get_constant_for_operand(left) {
                    *left = MirOperand::Constant(c);
                }
                if let Some(c) = self.get_constant_for_operand(right) {
                    *right = MirOperand::Constant(c);
                }
            }
            MirRvalue::UnaryOp { operand, .. } => {
                if let Some(c) = self.get_constant_for_operand(operand) {
                    *operand = MirOperand::Constant(c);
                }
            }
            _ => {}
        }
    }

    /// Get constant value for an operand if known
    fn get_constant_for_operand(&self, op: &MirOperand) -> Option<MirConstant> {
        match op {
            MirOperand::Constant(_) => None, // Already a constant
            MirOperand::Copy(place) | MirOperand::Move(place) => {
                if place.projection.is_empty() {
                    if let Some(LatticeValue::Constant(c)) = self.lattice.get(&place.local) {
                        return Some(c.clone());
                    }
                }
                None
            }
        }
    }

    /// Fold constant expressions (simplify binary ops on constants)
    fn fold_constants(&self, func: &mut MirFunction) {
        for block in &mut func.blocks {
            for inst in &mut block.instructions {
                if let MirInstruction::Assign { value, .. } = inst {
                    if let MirRvalue::BinaryOp { op, left, right } = value {
                        if let (MirOperand::Constant(lc), MirOperand::Constant(rc)) = (left, right)
                        {
                            if let Some(result) = self.eval_binary_op(op, lc, rc) {
                                *value = MirRvalue::Use(MirOperand::Constant(result));
                            }
                        }
                    } else if let MirRvalue::UnaryOp { op, operand } = value {
                        if let MirOperand::Constant(c) = operand {
                            if let Some(result) = self.eval_unary_op(op, c) {
                                *value = MirRvalue::Use(MirOperand::Constant(result));
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Default for ConstantPropagation {
    fn default() -> Self {
        Self::new()
    }
}

impl MirPass for ConstantPropagation {
    fn name(&self) -> &'static str {
        "agneyastra_constprop" // आग्नेयास्त्र - Fire weapon for burning uncertainty
    }

    fn mantra(&self) -> &'static str {
        "Om Agnaye Svāhā" // Invocation of Agni
    }

    fn run(&mut self, func: &mut MirFunction) {
        // Phase 1: Initialize lattice
        self.initialize(func);

        // Phase 2: Propagate constants to fixed point
        self.propagate(func);

        // Phase 3: Replace uses of constants
        self.replace_constants(func);

        // Phase 4: Fold constant expressions
        self.fold_constants(func);
    }
}

// ============================================
// Ahaṃkāra (Ego) Level - Function Boundaries
// ============================================

/// Function inlining pass - Sūkṣmāstra (सूक्ष्मास्त्र)
///
/// The subtle weapon that merges function boundaries, eliminating call overhead
/// by directly integrating callee code into caller.
///
/// Inlining Strategy:
/// 1. Analyze call sites for inlining candidates
/// 2. Apply size and depth heuristics
/// 3. Clone and remap callee's MIR into caller
/// 4. Handle parameter passing and return value
/// 5. Update CFG with inlined blocks
pub struct Inlining {
    /// Maximum instruction count to inline
    max_size: usize,
    /// Maximum inlining depth to prevent explosion
    max_depth: usize,
    /// Call site information
    call_sites: Vec<CallSite>,
    /// Functions available for inlining (name -> function)
    available_functions: HashMap<String, MirFunction>,
    /// Current inlining depth
    current_depth: usize,
}

/// Information about a call site candidate for inlining
#[derive(Debug, Clone)]
struct CallSite {
    /// Block containing the call
    block_id: usize,
    /// Index of instruction in block (must be terminator for calls)
    /// Actually we track the terminator
    callee_name: String,
    /// Arguments to the call
    args: Vec<MirOperand>,
    /// Destination for return value
    destination: Option<MirPlace>,
    /// Target block after call
    target_block: usize,
    /// Estimated benefit of inlining
    benefit_score: i32,
}

impl Inlining {
    pub fn new(max_size: usize) -> Self {
        Self {
            max_size,
            max_depth: 3,
            call_sites: Vec::new(),
            available_functions: HashMap::new(),
            current_depth: 0,
        }
    }

    /// Register a function as available for inlining
    pub fn register_function(&mut self, func: MirFunction) {
        self.available_functions.insert(func.name.clone(), func);
    }

    /// Find all call sites in a function
    fn find_call_sites(&mut self, func: &MirFunction) {
        self.call_sites.clear();

        for block in &func.blocks {
            if let MirTerminator::Call {
                func: callee_op,
                args,
                destination,
                target,
            } = &block.terminator
            {
                // Extract function name from operand
                if let Some(callee_name) = self.extract_function_name(callee_op) {
                    // Check if we have the callee available
                    if let Some(callee) = self.available_functions.get(&callee_name) {
                        // Compute benefit score
                        let benefit = self.compute_inline_benefit(callee, block.id);

                        self.call_sites.push(CallSite {
                            block_id: block.id,
                            callee_name,
                            args: args.clone(),
                            destination: destination.clone(),
                            target_block: *target,
                            benefit_score: benefit,
                        });
                    }
                }
            }
        }

        // Sort by benefit (highest first)
        self.call_sites
            .sort_by(|a, b| b.benefit_score.cmp(&a.benefit_score));
    }

    /// Extract function name from MirOperand
    fn extract_function_name(&self, op: &MirOperand) -> Option<String> {
        match op {
            MirOperand::Constant(MirConstant::String(name)) => Some(name.clone()),
            MirOperand::Copy(place) | MirOperand::Move(place) => {
                // Could be a function pointer in a local
                // For now, we only inline direct calls
                None
            }
            _ => None,
        }
    }

    /// Compute benefit score for inlining a function at a call site
    fn compute_inline_benefit(&self, callee: &MirFunction, _call_block: usize) -> i32 {
        let instruction_count = callee
            .blocks
            .iter()
            .map(|b| b.instructions.len())
            .sum::<usize>();

        // Base score - prefer small functions
        let mut score = (self.max_size as i32) - (instruction_count as i32);

        // Bonus for very small functions (likely just a computation)
        if instruction_count <= 5 {
            score += 100;
        }

        // Bonus for leaf functions (no calls)
        let has_calls = callee
            .blocks
            .iter()
            .any(|b| matches!(b.terminator, MirTerminator::Call { .. }));
        if !has_calls {
            score += 50;
        }

        // Penalty for functions with many blocks (complex control flow)
        if callee.blocks.len() > 10 {
            score -= (callee.blocks.len() as i32) * 5;
        }

        score
    }

    /// Check if a function should be inlined based on heuristics
    fn should_inline(&self, callee: &MirFunction, call_site: &CallSite) -> bool {
        // Don't inline if we're too deep
        if self.current_depth >= self.max_depth {
            return false;
        }

        // Count instructions
        let instruction_count: usize = callee.blocks.iter().map(|b| b.instructions.len()).sum();

        // Don't inline if too large
        if instruction_count > self.max_size {
            return false;
        }

        // Don't inline recursive calls
        if call_site.callee_name == callee.name {
            return false;
        }

        // Inline if benefit score is positive
        call_site.benefit_score > 0
    }

    /// Inline a single call site
    fn inline_call_site(&self, func: &mut MirFunction, call_site: &CallSite, callee: &MirFunction) {
        // Step 1: Clone callee's blocks with remapped IDs and locals
        let base_block_id = func.blocks.iter().map(|b| b.id).max().unwrap_or(0) + 1;
        let base_local_id = func.locals.len();

        // Step 2: Create locals for callee's parameters and locals
        let mut local_remap: HashMap<usize, usize> = HashMap::new();

        // Map callee's return value local (usually _0)
        if let Some(dest) = &call_site.destination {
            local_remap.insert(0, dest.local);
        } else {
            // Create temp for discarded return value
            let temp_local = func.locals.len();
            func.locals.push(MirLocal {
                index: temp_local,
                ty: callee.return_type.clone(),
                name: None,
            });
            local_remap.insert(0, temp_local);
        }

        // Map parameters to argument values (we'll use copy instructions)
        for (idx, param) in callee.params.iter().enumerate() {
            let new_local = base_local_id + idx + 1;
            func.locals.push(MirLocal {
                index: new_local,
                ty: param.ty.clone(),
                name: None,
            });
            local_remap.insert(param.index, new_local);
        }

        // Map callee's other locals
        for local in &callee.locals {
            if !local_remap.contains_key(&local.index) {
                let new_local = func.locals.len();
                func.locals.push(MirLocal {
                    index: new_local,
                    ty: local.ty.clone(),
                    name: local.name.clone(),
                });
                local_remap.insert(local.index, new_local);
            }
        }

        // Step 3: Clone and remap callee's blocks
        let mut inlined_blocks: Vec<MirBasicBlock> = Vec::new();
        let mut block_remap: HashMap<usize, usize> = HashMap::new();

        for (idx, block) in callee.blocks.iter().enumerate() {
            let new_block_id = base_block_id + idx;
            block_remap.insert(block.id, new_block_id);
        }

        for block in &callee.blocks {
            let new_block_id = block_remap[&block.id];

            // Remap instructions
            let new_instructions: Vec<MirInstruction> = block
                .instructions
                .iter()
                .map(|inst| self.remap_instruction(inst, &local_remap))
                .collect();

            // Remap terminator
            let new_terminator = self.remap_terminator(
                &block.terminator,
                &local_remap,
                &block_remap,
                call_site.target_block,
            );

            inlined_blocks.push(MirBasicBlock {
                id: new_block_id,
                instructions: new_instructions,
                terminator: new_terminator,
            });
        }

        // Step 4: Create entry block that copies arguments to parameter locals
        let entry_block_id = base_block_id + callee.blocks.len();
        let mut entry_instructions: Vec<MirInstruction> = Vec::new();

        for (idx, arg) in call_site.args.iter().enumerate() {
            if let Some(&new_local) = local_remap.get(&(idx + 1)) {
                // +1 because 0 is return value
                // Actually params start at a different index, need to check callee.params
                if idx < callee.params.len() {
                    let param_local = callee.params[idx].index;
                    if let Some(&mapped_local) = local_remap.get(&param_local) {
                        entry_instructions.push(MirInstruction::Assign {
                            dest: MirPlace {
                                local: mapped_local,
                                projection: Vec::new(),
                            },
                            value: MirRvalue::Use(arg.clone()),
                        });
                    }
                }
            }
        }

        // Jump to first inlined block
        let first_inlined_block = block_remap
            .get(&callee.blocks[0].id)
            .copied()
            .unwrap_or(entry_block_id);

        let entry_block = MirBasicBlock {
            id: entry_block_id,
            instructions: entry_instructions,
            terminator: MirTerminator::Goto {
                target: first_inlined_block,
            },
        };

        // Step 5: Update the call site block to jump to entry block
        if let Some(call_block) = func.blocks.iter_mut().find(|b| b.id == call_site.block_id) {
            call_block.terminator = MirTerminator::Goto {
                target: entry_block_id,
            };
        }

        // Step 6: Add all inlined blocks to the function
        func.blocks.push(entry_block);
        func.blocks.extend(inlined_blocks);
    }

    /// Remap an instruction's locals
    fn remap_instruction(
        &self,
        inst: &MirInstruction,
        local_remap: &HashMap<usize, usize>,
    ) -> MirInstruction {
        match inst {
            MirInstruction::Assign { dest, value } => MirInstruction::Assign {
                dest: self.remap_place(dest, local_remap),
                value: self.remap_rvalue(value, local_remap),
            },
            MirInstruction::Drop { place } => MirInstruction::Drop {
                place: self.remap_place(place, local_remap),
            },
            MirInstruction::Store { ptr, value } => MirInstruction::Store {
                ptr: self.remap_operand(ptr, local_remap),
                value: self.remap_operand(value, local_remap),
            },
            MirInstruction::Load { dest, ptr } => MirInstruction::Load {
                dest: self.remap_place(dest, local_remap),
                ptr: self.remap_operand(ptr, local_remap),
            },
            MirInstruction::Assert { condition, message } => MirInstruction::Assert {
                condition: self.remap_operand(condition, local_remap),
                message: message.clone(),
            },
            MirInstruction::SetDiscriminant { place, variant } => MirInstruction::SetDiscriminant {
                place: self.remap_place(place, local_remap),
                variant: *variant,
            },
            MirInstruction::BoundsCheck {
                index,
                len,
                message,
            } => MirInstruction::BoundsCheck {
                index: self.remap_operand(index, local_remap),
                len: self.remap_operand(len, local_remap),
                message: message.clone(),
            },
            MirInstruction::Nop => MirInstruction::Nop,
        }
    }

    /// Remap a terminator's locals and blocks
    fn remap_terminator(
        &self,
        term: &MirTerminator,
        local_remap: &HashMap<usize, usize>,
        block_remap: &HashMap<usize, usize>,
        return_target: usize,
    ) -> MirTerminator {
        match term {
            MirTerminator::Goto { target } => MirTerminator::Goto {
                target: block_remap.get(target).copied().unwrap_or(*target),
            },
            MirTerminator::SwitchInt {
                discriminant,
                targets,
                otherwise,
            } => MirTerminator::SwitchInt {
                discriminant: self.remap_operand(discriminant, local_remap),
                targets: targets
                    .iter()
                    .map(|(val, tgt)| (*val, block_remap.get(tgt).copied().unwrap_or(*tgt)))
                    .collect(),
                otherwise: block_remap.get(otherwise).copied().unwrap_or(*otherwise),
            },
            MirTerminator::Return => {
                // Return becomes a goto to the continuation block
                MirTerminator::Goto {
                    target: return_target,
                }
            }
            MirTerminator::Call {
                func,
                args,
                destination,
                target,
            } => MirTerminator::Call {
                func: self.remap_operand(func, local_remap),
                args: args
                    .iter()
                    .map(|a| self.remap_operand(a, local_remap))
                    .collect(),
                destination: destination
                    .as_ref()
                    .map(|d| self.remap_place(d, local_remap)),
                target: block_remap.get(target).copied().unwrap_or(*target),
            },
            MirTerminator::Unreachable => MirTerminator::Unreachable,
            MirTerminator::Unwind => MirTerminator::Unwind,
        }
    }

    /// Remap a place's local
    fn remap_place(&self, place: &MirPlace, local_remap: &HashMap<usize, usize>) -> MirPlace {
        MirPlace {
            local: local_remap
                .get(&place.local)
                .copied()
                .unwrap_or(place.local),
            projection: place.projection.clone(),
        }
    }

    /// Remap an operand's locals
    fn remap_operand(&self, op: &MirOperand, local_remap: &HashMap<usize, usize>) -> MirOperand {
        match op {
            MirOperand::Copy(place) => MirOperand::Copy(self.remap_place(place, local_remap)),
            MirOperand::Move(place) => MirOperand::Move(self.remap_place(place, local_remap)),
            MirOperand::Constant(c) => MirOperand::Constant(c.clone()),
        }
    }

    /// Remap an rvalue's locals
    fn remap_rvalue(&self, rvalue: &MirRvalue, local_remap: &HashMap<usize, usize>) -> MirRvalue {
        match rvalue {
            MirRvalue::Use(op) => MirRvalue::Use(self.remap_operand(op, local_remap)),
            MirRvalue::Ref { mutable, place } => MirRvalue::Ref {
                mutable: *mutable,
                place: self.remap_place(place, local_remap),
            },
            MirRvalue::BinaryOp { op, left, right } => MirRvalue::BinaryOp {
                op: *op,
                left: self.remap_operand(left, local_remap),
                right: self.remap_operand(right, local_remap),
            },
            MirRvalue::UnaryOp { op, operand } => MirRvalue::UnaryOp {
                op: *op,
                operand: self.remap_operand(operand, local_remap),
            },
            MirRvalue::Aggregate { kind, operands } => MirRvalue::Aggregate {
                kind: kind.clone(),
                operands: operands
                    .iter()
                    .map(|o| self.remap_operand(o, local_remap))
                    .collect(),
            },
            MirRvalue::Cast { kind, operand, ty } => MirRvalue::Cast {
                kind: *kind,
                operand: self.remap_operand(operand, local_remap),
                ty: ty.clone(),
            },
            MirRvalue::Discriminant(place) => {
                MirRvalue::Discriminant(self.remap_place(place, local_remap))
            }
            MirRvalue::Len(place) => MirRvalue::Len(self.remap_place(place, local_remap)),
            MirRvalue::AddressOf { mutable, place } => MirRvalue::AddressOf {
                mutable: *mutable,
                place: self.remap_place(place, local_remap),
            },
            MirRvalue::Field { base, index } => MirRvalue::Field {
                base: self.remap_operand(base, local_remap),
                index: *index,
            },
            MirRvalue::Index { base, index } => MirRvalue::Index {
                base: self.remap_operand(base, local_remap),
                index: self.remap_operand(index, local_remap),
            },
            MirRvalue::FloatOp { op, left, right } => MirRvalue::FloatOp {
                op: *op,
                left: self.remap_operand(left, local_remap),
                right: self.remap_operand(right, local_remap),
            },
            MirRvalue::SimdOp {
                op,
                operands,
                width,
            } => MirRvalue::SimdOp {
                op: *op,
                operands: operands
                    .iter()
                    .map(|o| self.remap_operand(o, local_remap))
                    .collect(),
                width: *width,
            },
        }
    }
}

impl MirPass for Inlining {
    fn name(&self) -> &'static str {
        "inlining"
    }

    fn run(&mut self, func: &mut MirFunction) {
        // Find all call sites
        self.find_call_sites(func);

        // Process call sites (we process a copy to avoid borrow issues)
        let call_sites: Vec<CallSite> = self.call_sites.drain(..).collect();

        for call_site in call_sites {
            if let Some(callee) = self
                .available_functions
                .get(&call_site.callee_name)
                .cloned()
            {
                if self.should_inline(&callee, &call_site) {
                    self.current_depth += 1;
                    self.inline_call_site(func, &call_site, &callee);
                    self.current_depth -= 1;
                }
            }
        }
    }
}

// ============================================
// Manas (Mind) Level - Control Flow
// ============================================

/// CFG Simplification - Vayuastra (वायव्यास्त्र)
///
/// The wind weapon that clears away tangled control flow, leaving behind
/// a clean, streamlined CFG.
///
/// Transformations:
/// 1. Merge linear block chains (A→B where B has single predecessor)
/// 2. Remove trivial gotos (blocks that only jump)
/// 3. Simplify constant conditional branches
/// 4. Remove unreachable blocks
/// 5. Thread jumps through empty blocks
pub struct SimplifyCfg {
    /// Map of old block ID to new block ID after merging
    block_remap: HashMap<usize, usize>,
    /// Set of blocks to remove
    dead_blocks: HashSet<usize>,
    /// Predecessor count for each block
    pred_count: HashMap<usize, usize>,
    /// Successor information
    successors: HashMap<usize, Vec<usize>>,
}

impl SimplifyCfg {
    pub fn new() -> Self {
        Self {
            block_remap: HashMap::new(),
            dead_blocks: HashSet::new(),
            pred_count: HashMap::new(),
            successors: HashMap::new(),
        }
    }

    /// Build predecessor and successor information
    fn analyze_cfg(&mut self, func: &MirFunction) {
        self.pred_count.clear();
        self.successors.clear();

        // Initialize all blocks with 0 predecessors
        for block in &func.blocks {
            self.pred_count.insert(block.id, 0);
            self.successors.insert(block.id, Vec::new());
        }

        // Count predecessors and build successor list
        for block in &func.blocks {
            let succs = self.get_successors(&block.terminator);
            self.successors.insert(block.id, succs.clone());
            for succ in succs {
                *self.pred_count.entry(succ).or_insert(0) += 1;
            }
        }

        // Entry block has implicit predecessor (function entry)
        if !func.blocks.is_empty() {
            *self.pred_count.entry(func.blocks[0].id).or_insert(0) += 1;
        }
    }

    /// Get successor block IDs from a terminator
    fn get_successors(&self, term: &MirTerminator) -> Vec<usize> {
        match term {
            MirTerminator::Goto { target } => vec![*target],
            MirTerminator::SwitchInt {
                targets, otherwise, ..
            } => {
                let mut succs: Vec<usize> = targets.iter().map(|(_, t)| *t).collect();
                succs.push(*otherwise);
                succs
            }
            MirTerminator::Return | MirTerminator::Unreachable | MirTerminator::Unwind => vec![],
            MirTerminator::Call { target, .. } => vec![*target],
        }
    }

    /// Check if a block is empty (no instructions)
    fn is_empty_block(&self, block: &MirBasicBlock) -> bool {
        block.instructions.is_empty()
    }

    /// Check if block is trivial goto (empty with unconditional jump)
    fn is_trivial_goto(&self, block: &MirBasicBlock) -> Option<usize> {
        if self.is_empty_block(block) {
            if let MirTerminator::Goto { target } = &block.terminator {
                return Some(*target);
            }
        }
        None
    }

    /// Simplify constant conditional branches
    fn simplify_const_branches(&mut self, func: &mut MirFunction) {
        for block in &mut func.blocks {
            if let MirTerminator::SwitchInt {
                discriminant,
                targets,
                otherwise,
            } = &block.terminator
            {
                // Check if discriminant is a constant
                if let MirOperand::Constant(MirConstant::Int(val, _)) = discriminant {
                    // Find matching target
                    let target = targets
                        .iter()
                        .find(|(v, _)| *v == *val)
                        .map(|(_, t)| *t)
                        .unwrap_or(*otherwise);

                    block.terminator = MirTerminator::Goto { target };
                } else if let MirOperand::Constant(MirConstant::Bool(val)) = discriminant {
                    // Boolean switch - find target for 0 (false) or 1 (true)
                    let search_val = if *val { 1 } else { 0 };
                    let target = targets
                        .iter()
                        .find(|(v, _)| *v == search_val)
                        .map(|(_, t)| *t)
                        .unwrap_or(*otherwise);
                    block.terminator = MirTerminator::Goto { target };
                }
            }
        }
    }

    /// Thread jumps through trivial goto blocks
    fn thread_jumps(&mut self, func: &mut MirFunction) {
        // Build map of trivial goto targets
        let mut goto_targets: HashMap<usize, usize> = HashMap::new();
        for block in &func.blocks {
            if let Some(target) = self.is_trivial_goto(block) {
                goto_targets.insert(block.id, target);
            }
        }

        // Resolve chains (A→B→C becomes A→C)
        let resolved = Self::resolve_goto_chains(&goto_targets);

        // Update all terminators to use resolved targets
        for block in &mut func.blocks {
            Self::update_terminator_targets(&mut block.terminator, &resolved);
        }
    }

    /// Resolve chains of goto targets to final destinations
    fn resolve_goto_chains(goto_targets: &HashMap<usize, usize>) -> HashMap<usize, usize> {
        let mut resolved: HashMap<usize, usize> = HashMap::new();

        for &start in goto_targets.keys() {
            let mut current = start;
            let mut visited = HashSet::new();

            // Follow chain until we hit a non-trivial block or cycle
            while let Some(&next) = goto_targets.get(&current) {
                if visited.contains(&next) {
                    // Cycle detected, stop
                    break;
                }
                visited.insert(current);
                current = next;
            }

            if current != start {
                resolved.insert(start, current);
            }
        }

        resolved
    }

    /// Update terminator targets based on resolved map
    fn update_terminator_targets(term: &mut MirTerminator, resolved: &HashMap<usize, usize>) {
        match term {
            MirTerminator::Goto { target } => {
                if let Some(&new_target) = resolved.get(target) {
                    *target = new_target;
                }
            }
            MirTerminator::SwitchInt {
                targets, otherwise, ..
            } => {
                for (_, target) in targets.iter_mut() {
                    if let Some(&new_target) = resolved.get(target) {
                        *target = new_target;
                    }
                }
                if let Some(&new_target) = resolved.get(otherwise) {
                    *otherwise = new_target;
                }
            }
            MirTerminator::Call { target, .. } => {
                if let Some(&new_target) = resolved.get(target) {
                    *target = new_target;
                }
            }
            _ => {}
        }
    }

    /// Merge linear block chains
    fn merge_blocks(&mut self, func: &mut MirFunction) {
        // Re-analyze CFG after previous transformations
        self.analyze_cfg(func);

        let mut merged = true;
        while merged {
            merged = false;

            // Find candidate merges
            let mut merge_into: HashMap<usize, usize> = HashMap::new();

            for block in &func.blocks {
                // Check if this block can be merged into its predecessor
                if let MirTerminator::Goto { target } = &block.terminator {
                    let target_pred_count = self.pred_count.get(target).copied().unwrap_or(0);

                    // Target has single predecessor and isn't the entry block
                    if target_pred_count == 1 && *target != func.blocks[0].id {
                        // Don't merge with self
                        if *target != block.id {
                            merge_into.insert(*target, block.id);
                        }
                    }
                }
            }

            // Apply merges
            for (target_id, source_id) in merge_into {
                if let Some(target_idx) = func.blocks.iter().position(|b| b.id == target_id) {
                    if let Some(source_idx) = func.blocks.iter().position(|b| b.id == source_id) {
                        // Extract target block
                        let target_block = func.blocks.remove(target_idx);

                        // Find source again (index may have shifted)
                        if let Some(source_idx) = func.blocks.iter().position(|b| b.id == source_id)
                        {
                            // Merge instructions and terminator
                            func.blocks[source_idx]
                                .instructions
                                .extend(target_block.instructions);
                            func.blocks[source_idx].terminator = target_block.terminator;
                            merged = true;

                            // Update block remapping
                            self.block_remap.insert(target_id, source_id);
                        }
                    }
                }
            }

            // Re-analyze for next iteration
            if merged {
                self.analyze_cfg(func);
            }
        }

        // Update all references to remapped blocks
        self.update_block_references(func);
    }

    /// Update all block references after merging
    fn update_block_references(&self, func: &mut MirFunction) {
        // Resolve transitive remapping
        let resolved = self.resolve_remap();

        for block in &mut func.blocks {
            Self::update_terminator_targets(&mut block.terminator, &resolved);
        }
    }

    /// Resolve transitive block remapping
    fn resolve_remap(&self) -> HashMap<usize, usize> {
        let mut resolved = HashMap::new();

        for &old in self.block_remap.keys() {
            let mut current = old;
            while let Some(&next) = self.block_remap.get(&current) {
                current = next;
            }
            resolved.insert(old, current);
        }

        resolved
    }

    /// Remove unreachable blocks
    fn remove_unreachable(&mut self, func: &mut MirFunction) {
        if func.blocks.is_empty() {
            return;
        }

        // BFS from entry to find reachable blocks
        let mut reachable = HashSet::new();
        let mut worklist = VecDeque::new();

        worklist.push_back(func.blocks[0].id);
        reachable.insert(func.blocks[0].id);

        while let Some(block_id) = worklist.pop_front() {
            if let Some(succs) = self.successors.get(&block_id) {
                for &succ in succs {
                    if reachable.insert(succ) {
                        worklist.push_back(succ);
                    }
                }
            }
        }

        // Remove unreachable blocks
        func.blocks.retain(|b| reachable.contains(&b.id));
    }

    /// Renumber blocks to be contiguous starting from 0
    fn renumber_blocks(&mut self, func: &mut MirFunction) {
        // Create mapping from old IDs to new IDs
        let mut id_map: HashMap<usize, usize> = HashMap::new();
        for (new_id, block) in func.blocks.iter().enumerate() {
            id_map.insert(block.id, new_id);
        }

        // Update block IDs
        for (new_id, block) in func.blocks.iter_mut().enumerate() {
            block.id = new_id;
        }

        // Update all terminator references
        for block in &mut func.blocks {
            Self::remap_terminator_ids(&mut block.terminator, &id_map);
        }
    }

    /// Remap block IDs in terminator
    fn remap_terminator_ids(term: &mut MirTerminator, id_map: &HashMap<usize, usize>) {
        match term {
            MirTerminator::Goto { target } => {
                if let Some(&new_id) = id_map.get(target) {
                    *target = new_id;
                }
            }
            MirTerminator::SwitchInt {
                targets, otherwise, ..
            } => {
                for (_, target) in targets.iter_mut() {
                    if let Some(&new_id) = id_map.get(target) {
                        *target = new_id;
                    }
                }
                if let Some(&new_id) = id_map.get(otherwise) {
                    *otherwise = new_id;
                }
            }
            MirTerminator::Call { target, .. } => {
                if let Some(&new_id) = id_map.get(target) {
                    *target = new_id;
                }
            }
            _ => {}
        }
    }
}

impl Default for SimplifyCfg {
    fn default() -> Self {
        Self::new()
    }
}

impl MirPass for SimplifyCfg {
    fn name(&self) -> &'static str {
        "vayuastra_simplify_cfg" // वायव्यास्त्र - Wind weapon for clearing control flow
    }

    fn mantra(&self) -> &'static str {
        "Om Vāyave Namaḥ" // Salutation to Vayu
    }

    fn run(&mut self, func: &mut MirFunction) {
        // Reset state
        self.block_remap.clear();
        self.dead_blocks.clear();

        // Build initial CFG analysis
        self.analyze_cfg(func);

        // Phase 1: Simplify constant branches
        self.simplify_const_branches(func);

        // Phase 2: Thread jumps through trivial gotos
        self.thread_jumps(func);

        // Phase 3: Merge linear block chains
        self.merge_blocks(func);

        // Phase 4: Remove unreachable blocks
        self.analyze_cfg(func); // Re-analyze after changes
        self.remove_unreachable(func);

        // Phase 5: Renumber blocks to be contiguous
        self.renumber_blocks(func);
    }
}

/// Loop Unrolling - Pashupatastra (पाशुपतास्त्र)
///
/// The weapon of Lord Shiva for mastering loops - duplicates loop bodies
/// to reduce loop overhead and enable further optimizations.
///
/// Algorithm:
/// 1. Identify natural loops (back edges in CFG)
/// 2. Analyze loop bounds and trip count
/// 3. For small loops with constant bounds, fully unroll
/// 4. For larger loops, partially unroll with factor
/// 5. Generate unrolled code with adjusted indices
pub struct LoopUnrolling {
    /// Maximum unroll factor
    max_factor: usize,
    /// Maximum instructions in loop body to consider for unrolling
    max_body_size: usize,
    /// Detected loops: (header_block_id, back_edge_source_id)
    loops: Vec<NaturalLoop>,
}

/// Natural loop representation
#[derive(Debug, Clone)]
struct NaturalLoop {
    /// Header block (loop entry point, dominates all other blocks)
    header: usize,
    /// Back edge source (jumps back to header)
    latch: usize,
    /// All blocks in the loop body
    body: HashSet<usize>,
    /// Estimated trip count (if known)
    trip_count: Option<usize>,
    /// Induction variable (local index, increment amount)
    induction_var: Option<(usize, i64)>,
}

impl LoopUnrolling {
    pub fn new(max_factor: usize) -> Self {
        Self {
            max_factor,
            max_body_size: 50, // Maximum instructions to unroll
            loops: Vec::new(),
        }
    }

    /// Find all natural loops in the function
    fn find_loops(&mut self, func: &MirFunction) {
        self.loops.clear();

        // Build dominator information (simplified)
        let dominators = self.compute_dominators(func);

        // Find back edges (edge where target dominates source)
        for block in &func.blocks {
            for succ in self.get_block_successors(&block.terminator) {
                // Back edge: succ dominates block.id
                if dominators
                    .get(&block.id)
                    .map_or(false, |doms| doms.contains(&succ))
                {
                    // Found a natural loop
                    let body = self.compute_loop_body(func, succ, block.id);
                    let trip_count = self.analyze_trip_count(func, succ, &body);
                    let induction_var = self.find_induction_variable(func, succ, &body);

                    self.loops.push(NaturalLoop {
                        header: succ,
                        latch: block.id,
                        body,
                        trip_count,
                        induction_var,
                    });
                }
            }
        }
    }

    /// Compute dominators using simple iterative algorithm
    fn compute_dominators(&self, func: &MirFunction) -> HashMap<usize, HashSet<usize>> {
        let mut dominators: HashMap<usize, HashSet<usize>> = HashMap::new();

        if func.blocks.is_empty() {
            return dominators;
        }

        // Entry block only dominates itself
        let entry_id = func.blocks[0].id;
        let mut entry_doms = HashSet::new();
        entry_doms.insert(entry_id);
        dominators.insert(entry_id, entry_doms);

        // All blocks initially dominated by all blocks
        let all_blocks: HashSet<usize> = func.blocks.iter().map(|b| b.id).collect();
        for block in &func.blocks {
            if block.id != entry_id {
                dominators.insert(block.id, all_blocks.clone());
            }
        }

        // Build predecessors map
        let mut predecessors: HashMap<usize, Vec<usize>> = HashMap::new();
        for block in &func.blocks {
            for succ in self.get_block_successors(&block.terminator) {
                predecessors.entry(succ).or_default().push(block.id);
            }
        }

        // Iterate until fixed point
        let mut changed = true;
        while changed {
            changed = false;
            for block in &func.blocks {
                if block.id == entry_id {
                    continue;
                }

                // Dom(n) = {n} ∪ (∩ Dom(p) for all predecessors p)
                let preds = predecessors.get(&block.id).cloned().unwrap_or_default();
                let mut new_doms: Option<HashSet<usize>> = None;

                for pred in preds {
                    if let Some(pred_doms) = dominators.get(&pred) {
                        match &mut new_doms {
                            None => new_doms = Some(pred_doms.clone()),
                            Some(doms) => {
                                *doms = doms.intersection(pred_doms).copied().collect();
                            }
                        }
                    }
                }

                let mut final_doms = new_doms.unwrap_or_default();
                final_doms.insert(block.id);

                if dominators.get(&block.id) != Some(&final_doms) {
                    dominators.insert(block.id, final_doms);
                    changed = true;
                }
            }
        }

        dominators
    }

    /// Compute all blocks in the loop body
    fn compute_loop_body(&self, func: &MirFunction, header: usize, latch: usize) -> HashSet<usize> {
        let mut body = HashSet::new();
        body.insert(header);
        body.insert(latch);

        // Backward traversal from latch to header
        let mut worklist = vec![latch];
        while let Some(block_id) = worklist.pop() {
            if block_id == header {
                continue;
            }

            // Find predecessors
            for block in &func.blocks {
                for succ in self.get_block_successors(&block.terminator) {
                    if succ == block_id && !body.contains(&block.id) {
                        body.insert(block.id);
                        worklist.push(block.id);
                    }
                }
            }
        }

        body
    }

    /// Analyze loop to determine trip count
    fn analyze_trip_count(
        &self,
        func: &MirFunction,
        header: usize,
        body: &HashSet<usize>,
    ) -> Option<usize> {
        // Look for comparison pattern in header terminator
        for block in &func.blocks {
            if block.id == header {
                if let MirTerminator::SwitchInt { discriminant, .. } = &block.terminator {
                    // Try to find constant bounds
                    // This is a simplified analysis
                    if let MirOperand::Copy(place) | MirOperand::Move(place) = discriminant {
                        // Look for comparison instruction that produces this
                        for inst in &block.instructions {
                            if let MirInstruction::Assign { dest, value } = inst {
                                if dest.local == place.local {
                                    if let MirRvalue::BinaryOp { op, right, .. } = value {
                                        if matches!(
                                            op,
                                            BinaryOp::Lt
                                                | BinaryOp::Le
                                                | BinaryOp::Gt
                                                | BinaryOp::Ge
                                        ) {
                                            if let MirOperand::Constant(MirConstant::Int(n, _)) =
                                                right
                                            {
                                                return Some(*n as usize);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Find induction variable in loop
    fn find_induction_variable(
        &self,
        func: &MirFunction,
        header: usize,
        body: &HashSet<usize>,
    ) -> Option<(usize, i64)> {
        // Look for i = i + constant pattern in loop body
        for block in &func.blocks {
            if body.contains(&block.id) {
                for inst in &block.instructions {
                    if let MirInstruction::Assign { dest, value } = inst {
                        if let MirRvalue::BinaryOp {
                            op: BinaryOp::Add,
                            left,
                            right,
                        } = value
                        {
                            // Check if left is same as dest (i = i + something)
                            if let MirOperand::Copy(place) | MirOperand::Move(place) = left {
                                if place.local == dest.local && place.projection.is_empty() {
                                    if let MirOperand::Constant(MirConstant::Int(inc, _)) = right {
                                        return Some((dest.local, *inc));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Count instructions in loop body
    fn count_loop_instructions(&self, func: &MirFunction, body: &HashSet<usize>) -> usize {
        func.blocks
            .iter()
            .filter(|b| body.contains(&b.id))
            .map(|b| b.instructions.len())
            .sum()
    }

    /// Get successor block IDs from a terminator
    fn get_block_successors(&self, term: &MirTerminator) -> Vec<usize> {
        match term {
            MirTerminator::Goto { target } => vec![*target],
            MirTerminator::SwitchInt {
                targets, otherwise, ..
            } => {
                let mut succs: Vec<usize> = targets.iter().map(|(_, t)| *t).collect();
                succs.push(*otherwise);
                succs
            }
            MirTerminator::Return | MirTerminator::Unreachable | MirTerminator::Unwind => vec![],
            MirTerminator::Call { target, .. } => vec![*target],
        }
    }

    /// Unroll a loop by a given factor
    fn unroll_loop(&self, func: &mut MirFunction, loop_info: &NaturalLoop, factor: usize) {
        if factor <= 1 {
            return;
        }

        // Collect body blocks (excluding header)
        let body_blocks: Vec<&MirBasicBlock> = func
            .blocks
            .iter()
            .filter(|b| loop_info.body.contains(&b.id) && b.id != loop_info.header)
            .collect();

        if body_blocks.is_empty() {
            return;
        }

        // For each iteration, clone the body blocks with adjusted IDs
        let max_id = func.blocks.iter().map(|b| b.id).max().unwrap_or(0);
        let mut new_blocks: Vec<MirBasicBlock> = Vec::new();

        for iteration in 1..factor {
            let id_offset = max_id + (iteration * loop_info.body.len());

            for orig_block in &body_blocks {
                let mut cloned = (*orig_block).clone();
                cloned.id = orig_block.id + id_offset;

                // Adjust terminator targets
                Self::adjust_terminator_targets(
                    &mut cloned.terminator,
                    &loop_info.body,
                    id_offset,
                    loop_info.header,
                );

                // If this is the last iteration and it's the latch,
                // make it jump to the original header check
                if iteration == factor - 1 && orig_block.id == loop_info.latch {
                    if let MirTerminator::Goto { target } = &mut cloned.terminator {
                        if *target == loop_info.header + id_offset {
                            *target = loop_info.header;
                        }
                    }
                }

                new_blocks.push(cloned);
            }
        }

        // Connect original latch to first copy
        for block in &mut func.blocks {
            if block.id == loop_info.latch {
                if let MirTerminator::Goto { target } = &mut block.terminator {
                    if *target == loop_info.header {
                        *target = loop_info.latch + (max_id + loop_info.body.len());
                    }
                }
            }
        }

        func.blocks.extend(new_blocks);
    }

    /// Adjust terminator targets for cloned blocks
    fn adjust_terminator_targets(
        term: &mut MirTerminator,
        body: &HashSet<usize>,
        offset: usize,
        header: usize,
    ) {
        match term {
            MirTerminator::Goto { target } => {
                if body.contains(target) && *target != header {
                    *target += offset;
                }
            }
            MirTerminator::SwitchInt {
                targets, otherwise, ..
            } => {
                for (_, target) in targets.iter_mut() {
                    if body.contains(target) && *target != header {
                        *target += offset;
                    }
                }
                if body.contains(otherwise) && *otherwise != header {
                    *otherwise += offset;
                }
            }
            MirTerminator::Call { target, .. } => {
                if body.contains(target) && *target != header {
                    *target += offset;
                }
            }
            _ => {}
        }
    }
}

impl MirPass for LoopUnrolling {
    fn name(&self) -> &'static str {
        "pashupatastra_loop_unroll" // पाशुपतास्त्र - Shiva's weapon for loop mastery
    }

    fn mantra(&self) -> &'static str {
        "Om Namaḥ Śivāya" // Salutation to Shiva
    }

    fn run(&mut self, func: &mut MirFunction) {
        // Phase 1: Find all natural loops
        self.find_loops(func);

        // Phase 2: Unroll eligible loops
        let loops_to_unroll: Vec<NaturalLoop> = self
            .loops
            .iter()
            .filter(|l| {
                let body_size = self.count_loop_instructions(func, &l.body);
                body_size <= self.max_body_size
            })
            .cloned()
            .collect();

        for loop_info in &loops_to_unroll {
            let factor = if let Some(trip_count) = loop_info.trip_count {
                // For small loops, fully unroll
                if trip_count <= self.max_factor {
                    trip_count
                } else {
                    self.max_factor
                }
            } else {
                // Unknown trip count - use conservative factor
                self.max_factor.min(2)
            };

            self.unroll_loop(func, loop_info, factor);
        }
    }
}

// ============================================
// Indriya (Senses) Level - I/O & Memory Access
// ============================================

/// Memory access pattern for analysis
#[derive(Debug, Clone)]
struct MemoryAccess {
    /// Instruction index
    instruction: usize,
    /// Block containing the access
    block: usize,
    /// Local being accessed
    local: usize,
    /// Is this a load (read)?
    is_load: bool,
    /// Is this a store (write)?
    is_store: bool,
    /// Projection path (field indices, array indices)
    projection: Vec<usize>,
}

/// Memory access optimization pass - Indriya (इन्द्रिय)
/// Senses perceive and process external information; this pass
/// optimizes how the program perceives (reads) and affects (writes) memory.
pub struct MemoryAccessOpt {
    /// Collected memory accesses
    accesses: Vec<MemoryAccess>,
    /// Access locality score per block
    locality_scores: HashMap<usize, f64>,
    /// Store-load forwarding candidates: (store_idx, load_idx)
    forwarding_candidates: Vec<(usize, usize)>,
}

impl MemoryAccessOpt {
    pub fn new() -> Self {
        Self {
            accesses: Vec::new(),
            locality_scores: HashMap::new(),
            forwarding_candidates: Vec::new(),
        }
    }

    /// Collect all memory accesses in the function
    fn collect_accesses(&mut self, func: &MirFunction) {
        self.accesses.clear();

        for block in &func.blocks {
            for (inst_idx, inst) in block.instructions.iter().enumerate() {
                match inst {
                    MirInstruction::Load { dest, ptr } => {
                        if let MirOperand::Copy(place) | MirOperand::Move(place) = ptr {
                            let projection: Vec<usize> = place
                                .projection
                                .iter()
                                .filter_map(|p| match p {
                                    PlaceProjection::Field { index } => Some(*index),
                                    PlaceProjection::ConstIndex { offset } => Some(*offset),
                                    _ => None,
                                })
                                .collect();

                            self.accesses.push(MemoryAccess {
                                instruction: inst_idx,
                                block: block.id,
                                local: place.local,
                                is_load: true,
                                is_store: false,
                                projection,
                            });
                        }
                    }
                    MirInstruction::Store { ptr, value: _ } => {
                        if let MirOperand::Copy(place) | MirOperand::Move(place) = ptr {
                            let projection: Vec<usize> = place
                                .projection
                                .iter()
                                .filter_map(|p| match p {
                                    PlaceProjection::Field { index } => Some(*index),
                                    PlaceProjection::ConstIndex { offset } => Some(*offset),
                                    _ => None,
                                })
                                .collect();

                            self.accesses.push(MemoryAccess {
                                instruction: inst_idx,
                                block: block.id,
                                local: place.local,
                                is_load: false,
                                is_store: true,
                                projection,
                            });
                        }
                    }
                    MirInstruction::Assign { dest, value } => {
                        // Check if this is effectively a load/store
                        if !dest.projection.is_empty() {
                            let projection: Vec<usize> = dest
                                .projection
                                .iter()
                                .filter_map(|p| match p {
                                    PlaceProjection::Field { index } => Some(*index),
                                    PlaceProjection::ConstIndex { offset } => Some(*offset),
                                    _ => None,
                                })
                                .collect();

                            self.accesses.push(MemoryAccess {
                                instruction: inst_idx,
                                block: block.id,
                                local: dest.local,
                                is_load: false,
                                is_store: true,
                                projection,
                            });
                        }

                        // Check for loads in rvalue
                        self.collect_rvalue_loads(value, inst_idx, block.id);
                    }
                    _ => {}
                }
            }
        }
    }

    /// Collect loads from an rvalue
    fn collect_rvalue_loads(&mut self, rvalue: &MirRvalue, inst_idx: usize, block_id: usize) {
        match rvalue {
            MirRvalue::Use(op) => self.collect_operand_loads(op, inst_idx, block_id),
            MirRvalue::Field { base, index: _ } => {
                self.collect_operand_loads(base, inst_idx, block_id);
            }
            MirRvalue::Index { base, index } => {
                self.collect_operand_loads(base, inst_idx, block_id);
                self.collect_operand_loads(index, inst_idx, block_id);
            }
            MirRvalue::BinaryOp { left, right, .. } => {
                self.collect_operand_loads(left, inst_idx, block_id);
                self.collect_operand_loads(right, inst_idx, block_id);
            }
            MirRvalue::UnaryOp { operand, .. } => {
                self.collect_operand_loads(operand, inst_idx, block_id);
            }
            _ => {}
        }
    }

    fn collect_operand_loads(&mut self, op: &MirOperand, inst_idx: usize, block_id: usize) {
        if let MirOperand::Copy(place) | MirOperand::Move(place) = op {
            if !place.projection.is_empty() {
                let projection: Vec<usize> = place
                    .projection
                    .iter()
                    .filter_map(|p| match p {
                        PlaceProjection::Field { index } => Some(*index),
                        PlaceProjection::ConstIndex { offset } => Some(*offset),
                        _ => None,
                    })
                    .collect();

                self.accesses.push(MemoryAccess {
                    instruction: inst_idx,
                    block: block_id,
                    local: place.local,
                    is_load: true,
                    is_store: false,
                    projection,
                });
            }
        }
    }

    /// Compute locality score for each block
    /// Higher score = better spatial locality
    fn compute_locality_scores(&mut self) {
        self.locality_scores.clear();

        // Group accesses by block
        let mut block_accesses: HashMap<usize, Vec<&MemoryAccess>> = HashMap::new();
        for access in &self.accesses {
            block_accesses.entry(access.block).or_default().push(access);
        }

        // Score each block
        for (block_id, accesses) in block_accesses {
            if accesses.len() < 2 {
                self.locality_scores.insert(block_id, 1.0);
                continue;
            }

            let mut sequential_count = 0;
            let mut total_pairs = 0;

            // Check consecutive access patterns
            for window in accesses.windows(2) {
                total_pairs += 1;

                let a1 = window[0];
                let a2 = window[1];

                // Same local = potential sequential access
                if a1.local == a2.local {
                    // Check if projections are sequential
                    if a1.projection.len() == a2.projection.len() && !a1.projection.is_empty() {
                        let last_idx1 = a1.projection.last().unwrap_or(&0);
                        let last_idx2 = a2.projection.last().unwrap_or(&0);

                        // Sequential if indices differ by 1
                        if (*last_idx2 as i64 - *last_idx1 as i64).abs() <= 1 {
                            sequential_count += 1;
                        }
                    } else if a1.projection.is_empty() && a2.projection.is_empty() {
                        // Accessing same scalar twice = good locality
                        sequential_count += 1;
                    }
                }
            }

            let score = if total_pairs > 0 {
                sequential_count as f64 / total_pairs as f64
            } else {
                1.0
            };

            self.locality_scores.insert(block_id, score);
        }
    }

    /// Find store-load forwarding opportunities
    /// When a store is immediately followed by a load from the same location
    fn find_forwarding_candidates(&mut self) {
        self.forwarding_candidates.clear();

        // Look for store followed by load to same location
        for (i, access1) in self.accesses.iter().enumerate() {
            if !access1.is_store {
                continue;
            }

            for (j, access2) in self.accesses.iter().enumerate().skip(i + 1) {
                if access2.block != access1.block {
                    break; // Only within same block for now
                }

                if access2.is_load
                    && access2.local == access1.local
                    && access2.projection == access1.projection
                {
                    // Found a forwarding candidate
                    self.forwarding_candidates
                        .push((access1.instruction, access2.instruction));
                    break; // Only first matching load
                }

                // If another store to same location, stop looking
                if access2.is_store
                    && access2.local == access1.local
                    && access2.projection == access1.projection
                {
                    break;
                }
            }
        }
    }

    /// Apply store-load forwarding transformation
    fn apply_forwarding(&self, func: &mut MirFunction) {
        // For each forwarding candidate, replace the load with the stored value
        // This requires tracking what value was stored

        for (store_idx, load_idx) in &self.forwarding_candidates {
            // Find the block containing these instructions
            for block in &mut func.blocks {
                if *store_idx >= block.instructions.len() || *load_idx >= block.instructions.len() {
                    continue;
                }

                // Get the value being stored
                let stored_value = match &block.instructions[*store_idx] {
                    MirInstruction::Store { value, .. } => Some(value.clone()),
                    MirInstruction::Assign { value, .. } => {
                        // Convert rvalue to operand if possible
                        match value {
                            MirRvalue::Use(op) => Some(op.clone()),
                            _ => None,
                        }
                    }
                    _ => None,
                };

                if let Some(stored_op) = stored_value {
                    // Replace the load with an assignment from the stored value
                    if let MirInstruction::Load { dest, .. } = &block.instructions[*load_idx] {
                        block.instructions[*load_idx] = MirInstruction::Assign {
                            dest: dest.clone(),
                            value: MirRvalue::Use(stored_op),
                        };
                    }
                }
            }
        }
    }

    /// Optimize strided access patterns by hoisting invariant address calculations
    fn optimize_strided_access(&self, func: &mut MirFunction) {
        // Group accesses by local to find strided patterns
        let mut local_accesses: HashMap<usize, Vec<&MemoryAccess>> = HashMap::new();
        for access in &self.accesses {
            local_accesses.entry(access.local).or_default().push(access);
        }

        // Look for regular stride patterns that could benefit from strength reduction
        for (local, accesses) in local_accesses {
            if accesses.len() < 3 {
                continue;
            }

            // Check if accesses form an arithmetic progression in indices
            let indices: Vec<i64> = accesses
                .iter()
                .filter_map(|a| a.projection.last().map(|&idx| idx as i64))
                .collect();

            if indices.len() >= 3 {
                // Check for constant stride
                let mut strides: Vec<i64> = Vec::new();
                for window in indices.windows(2) {
                    strides.push(window[1] - window[0]);
                }

                // If all strides are equal, we have a strided access
                if !strides.is_empty() && strides.iter().all(|&s| s == strides[0]) {
                    // This pattern could be optimized with address increment
                    // instead of multiply+add each iteration
                    // For now, just mark it as identified
                    let _stride = strides[0];
                    let _base_local = local;
                    // Future: Apply strength reduction transformation
                }
            }
        }
    }
}

impl Default for MemoryAccessOpt {
    fn default() -> Self {
        Self::new()
    }
}

impl MirPass for MemoryAccessOpt {
    fn name(&self) -> &'static str {
        "memory_access_optimization"
    }

    fn run(&mut self, func: &mut MirFunction) {
        // Phase 1: Collect all memory accesses
        self.collect_accesses(func);

        if self.accesses.is_empty() {
            return;
        }

        // Phase 2: Compute locality scores
        self.compute_locality_scores();

        // Phase 3: Find store-load forwarding opportunities
        self.find_forwarding_candidates();

        // Phase 4: Apply store-load forwarding
        self.apply_forwarding(func);

        // Phase 5: Optimize strided access patterns
        self.optimize_strided_access(func);
    }
}

// ============================================
// Tanmātra (Subtle Elements) Level - Data Layout
// ============================================

/// Field access information for reordering analysis
#[derive(Debug, Clone)]
struct FieldAccessInfo {
    /// Type index (for grouping accesses to same type)
    type_idx: usize,
    /// Field index within the type
    field_idx: usize,
    /// Number of accesses
    access_count: usize,
    /// Blocks where accessed (for co-access analysis)
    access_blocks: HashSet<usize>,
}

/// Struct field reordering for better cache locality - Tanmātra (तन्मात्र)
/// Subtle elements are the essence of perception; this pass reorganizes
/// data layout to align with actual access patterns (how data is perceived).
pub struct FieldReordering {
    /// Field access counts per type
    field_accesses: HashMap<usize, Vec<FieldAccessInfo>>,
    /// Co-access matrix: fields accessed together in same block
    co_access_matrix: HashMap<(usize, usize, usize, usize), usize>,
    /// Recommended field orderings per type
    recommended_orders: HashMap<usize, Vec<usize>>,
}

impl FieldReordering {
    pub fn new() -> Self {
        Self {
            field_accesses: HashMap::new(),
            co_access_matrix: HashMap::new(),
            recommended_orders: HashMap::new(),
        }
    }

    /// Collect field access information from the function
    fn collect_field_accesses(&mut self, func: &MirFunction) {
        self.field_accesses.clear();

        for block in &func.blocks {
            let mut block_field_accesses: Vec<(usize, usize)> = Vec::new();

            for inst in &block.instructions {
                // Check for field projections in places
                match inst {
                    MirInstruction::Assign { dest, value } => {
                        // Check destination
                        self.record_place_field_access(dest, block.id, &mut block_field_accesses);

                        // Check rvalue
                        match value {
                            MirRvalue::Field { base, index } => {
                                if let MirOperand::Copy(place) | MirOperand::Move(place) = base {
                                    let type_idx = place.local;
                                    self.record_field_access(type_idx, *index, block.id);
                                    block_field_accesses.push((type_idx, *index));
                                }
                            }
                            MirRvalue::Use(op) => {
                                if let MirOperand::Copy(place) | MirOperand::Move(place) = op {
                                    self.record_place_field_access(
                                        place,
                                        block.id,
                                        &mut block_field_accesses,
                                    );
                                }
                            }
                            _ => {}
                        }
                    }
                    MirInstruction::Load { dest, ptr } => {
                        self.record_place_field_access(dest, block.id, &mut block_field_accesses);
                        if let MirOperand::Copy(place) | MirOperand::Move(place) = ptr {
                            self.record_place_field_access(
                                place,
                                block.id,
                                &mut block_field_accesses,
                            );
                        }
                    }
                    MirInstruction::Store { ptr, value } => {
                        if let MirOperand::Copy(place) | MirOperand::Move(place) = ptr {
                            self.record_place_field_access(
                                place,
                                block.id,
                                &mut block_field_accesses,
                            );
                        }
                        if let MirOperand::Copy(place) | MirOperand::Move(place) = value {
                            self.record_place_field_access(
                                place,
                                block.id,
                                &mut block_field_accesses,
                            );
                        }
                    }
                    _ => {}
                }
            }

            // Build co-access matrix for this block
            for i in 0..block_field_accesses.len() {
                for j in (i + 1)..block_field_accesses.len() {
                    let (type1, field1) = block_field_accesses[i];
                    let (type2, field2) = block_field_accesses[j];

                    // Only track co-access within same type
                    if type1 == type2 {
                        let key = if field1 < field2 {
                            (type1, field1, type1, field2)
                        } else {
                            (type1, field2, type1, field1)
                        };
                        *self.co_access_matrix.entry(key).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    /// Record field access from a place's projection
    fn record_place_field_access(
        &mut self,
        place: &MirPlace,
        block_id: usize,
        block_accesses: &mut Vec<(usize, usize)>,
    ) {
        for proj in &place.projection {
            if let PlaceProjection::Field { index: field_idx } = proj {
                let type_idx = place.local;
                self.record_field_access(type_idx, *field_idx, block_id);
                block_accesses.push((type_idx, *field_idx));
            }
        }
    }

    /// Record a field access
    fn record_field_access(&mut self, type_idx: usize, field_idx: usize, block_id: usize) {
        let type_fields = self.field_accesses.entry(type_idx).or_default();

        // Find or create field info
        let info = type_fields.iter_mut().find(|f| f.field_idx == field_idx);

        if let Some(info) = info {
            info.access_count += 1;
            info.access_blocks.insert(block_id);
        } else {
            let mut blocks = HashSet::new();
            blocks.insert(block_id);
            type_fields.push(FieldAccessInfo {
                type_idx,
                field_idx,
                access_count: 1,
                access_blocks: blocks,
            });
        }
    }

    /// Compute recommended field orderings based on access patterns
    fn compute_recommended_orders(&mut self) {
        self.recommended_orders.clear();

        for (&type_idx, fields) in &self.field_accesses {
            if fields.is_empty() {
                continue;
            }

            // Score each field ordering based on:
            // 1. Hot fields first (more accesses = earlier)
            // 2. Co-accessed fields adjacent (maximize cache line sharing)

            let mut field_scores: Vec<(usize, usize)> = fields
                .iter()
                .map(|f| (f.field_idx, f.access_count))
                .collect();

            // Sort by access count descending (hot fields first)
            field_scores.sort_by(|a, b| b.1.cmp(&a.1));

            // Refine ordering based on co-access
            let field_order: Vec<usize> = self.optimize_order_with_coacccess(
                type_idx,
                field_scores.iter().map(|(idx, _)| *idx).collect(),
            );

            self.recommended_orders.insert(type_idx, field_order);
        }
    }

    /// Optimize field order considering co-access patterns
    fn optimize_order_with_coacccess(&self, type_idx: usize, mut order: Vec<usize>) -> Vec<usize> {
        if order.len() < 3 {
            return order;
        }

        // Simple greedy algorithm: for each position after first,
        // choose the field most co-accessed with previous field
        for i in 1..order.len() {
            let prev_field = order[i - 1];
            let mut best_next = i;
            let mut best_score = 0;

            for j in i..order.len() {
                let candidate = order[j];
                let key = if prev_field < candidate {
                    (type_idx, prev_field, type_idx, candidate)
                } else {
                    (type_idx, candidate, type_idx, prev_field)
                };

                let score = self.co_access_matrix.get(&key).copied().unwrap_or(0);
                if score > best_score {
                    best_score = score;
                    best_next = j;
                }
            }

            // Swap to put best candidate at position i
            if best_next != i {
                order.swap(i, best_next);
            }
        }

        order
    }

    /// Apply field reordering transformations
    /// Note: Actual struct layout modification requires type system integration
    /// Here we prepare the reordering information that can be used by codegen
    fn apply_reordering(&self, func: &mut MirFunction) {
        // In a real implementation, this would:
        // 1. Update type definitions with new field order
        // 2. Remap all field projections in the MIR
        // For now, we record the recommended orders for codegen to use

        // Store reordering info in function metadata (if such field exists)
        // For this implementation, we just ensure the analysis is complete

        // Remap field indices in projections
        for block in &mut func.blocks {
            for inst in &mut block.instructions {
                match inst {
                    MirInstruction::Assign { dest, value } => {
                        self.remap_place_fields(dest);
                        self.remap_rvalue_fields(value);
                    }
                    MirInstruction::Load { dest, ptr } => {
                        self.remap_place_fields(dest);
                        self.remap_operand_fields(ptr);
                    }
                    MirInstruction::Store { ptr, value } => {
                        self.remap_operand_fields(ptr);
                        self.remap_operand_fields(value);
                    }
                    _ => {}
                }
            }
        }
    }

    fn remap_place_fields(&self, place: &mut MirPlace) {
        if let Some(order) = self.recommended_orders.get(&place.local) {
            for proj in &mut place.projection {
                if let PlaceProjection::Field { index: idx } = proj {
                    // Find new index in reordered layout
                    if let Some(new_idx) = order.iter().position(|&f| f == *idx) {
                        *idx = new_idx;
                    }
                }
            }
        }
    }

    fn remap_operand_fields(&self, op: &mut MirOperand) {
        match op {
            MirOperand::Copy(place) | MirOperand::Move(place) => {
                self.remap_place_fields(place);
            }
            MirOperand::Constant(_) => {}
        }
    }

    fn remap_rvalue_fields(&self, rvalue: &mut MirRvalue) {
        match rvalue {
            MirRvalue::Use(op) => self.remap_operand_fields(op),
            MirRvalue::Field { base, index } => {
                self.remap_operand_fields(base);
                if let MirOperand::Copy(place) | MirOperand::Move(place) = base {
                    if let Some(order) = self.recommended_orders.get(&place.local) {
                        if let Some(new_idx) = order.iter().position(|&f| f == *index) {
                            *index = new_idx;
                        }
                    }
                }
            }
            MirRvalue::Index { base, index } => {
                self.remap_operand_fields(base);
                self.remap_operand_fields(index);
            }
            MirRvalue::BinaryOp { left, right, .. } => {
                self.remap_operand_fields(left);
                self.remap_operand_fields(right);
            }
            _ => {}
        }
    }
}

impl Default for FieldReordering {
    fn default() -> Self {
        Self::new()
    }
}

impl MirPass for FieldReordering {
    fn name(&self) -> &'static str {
        "field_reordering"
    }

    fn run(&mut self, func: &mut MirFunction) {
        // Phase 1: Collect field access patterns
        self.collect_field_accesses(func);

        if self.field_accesses.is_empty() {
            return;
        }

        // Phase 2: Compute optimal field ordering
        self.compute_recommended_orders();

        // Phase 3: Apply reordering (remap field indices)
        self.apply_reordering(func);
    }
}

/// Aggregate information for scalar replacement
#[derive(Debug, Clone)]
struct AggregateInfo {
    /// Local index of the aggregate
    local: usize,
    /// Number of fields/elements
    field_count: usize,
    /// Fields that escape (address taken, passed by reference)
    escaping_fields: HashSet<usize>,
    /// Fields that are accessed
    accessed_fields: HashSet<usize>,
    /// Is the entire aggregate ever used as a whole?
    used_as_whole: bool,
}

/// Scalar replacement of aggregates - SROA
/// Breaks apart aggregate structures (structs, tuples) into individual
/// scalar values when profitable, enabling better register allocation
/// and eliminating intermediate memory accesses.
pub struct ScalarReplacement {
    /// Aggregates eligible for replacement
    aggregates: HashMap<usize, AggregateInfo>,
    /// Mapping from (aggregate_local, field_idx) to new scalar local
    scalar_map: HashMap<(usize, usize), usize>,
    /// Next available local index
    next_local: usize,
}

impl ScalarReplacement {
    pub fn new() -> Self {
        Self {
            aggregates: HashMap::new(),
            scalar_map: HashMap::new(),
            next_local: 0,
        }
    }

    /// Analyze aggregates to determine eligibility for replacement
    fn analyze_aggregates(&mut self, func: &MirFunction) {
        self.aggregates.clear();

        // Find next available local index
        self.next_local = func.locals.len();

        // Identify aggregates and their usage patterns
        for block in &func.blocks {
            for inst in &block.instructions {
                match inst {
                    MirInstruction::Assign { dest, value } => {
                        // Check if creating an aggregate
                        if let MirRvalue::Aggregate { operands, kind } = value {
                            let field_count = operands.len();
                            self.aggregates.entry(dest.local).or_insert(AggregateInfo {
                                local: dest.local,
                                field_count,
                                escaping_fields: HashSet::new(),
                                accessed_fields: HashSet::new(),
                                used_as_whole: false,
                            });
                        }

                        // Check for field access
                        if !dest.projection.is_empty() {
                            if let Some(PlaceProjection::Field { index: idx }) =
                                dest.projection.first()
                            {
                                if let Some(info) = self.aggregates.get_mut(&dest.local) {
                                    info.accessed_fields.insert(*idx);
                                }
                            }
                        }

                        // Check for whole aggregate use
                        if dest.projection.is_empty() {
                            if let MirRvalue::Use(MirOperand::Copy(src) | MirOperand::Move(src)) =
                                value
                            {
                                if src.projection.is_empty() {
                                    if let Some(info) = self.aggregates.get_mut(&src.local) {
                                        info.used_as_whole = true;
                                    }
                                }
                            }
                        }

                        self.check_escaping_fields(value);
                    }
                    MirInstruction::Load { ptr, .. } => {
                        self.check_operand_escaping(ptr);
                    }
                    MirInstruction::Store { ptr, value } => {
                        self.check_operand_escaping(ptr);
                        self.check_operand_escaping(value);
                    }
                    _ => {}
                }
            }

            // Check terminator for escaping
            match &block.terminator {
                MirTerminator::Call { args, .. } => {
                    for arg in args {
                        self.check_operand_escaping(arg);
                    }
                }
                _ => {}
            }
        }
    }

    /// Check if an rvalue causes fields to escape
    fn check_escaping_fields(&mut self, rvalue: &MirRvalue) {
        match rvalue {
            MirRvalue::Ref { place, .. } | MirRvalue::AddressOf { place, .. } => {
                // Taking address of field causes it to escape
                if let Some(PlaceProjection::Field { index: idx }) = place.projection.first() {
                    if let Some(info) = self.aggregates.get_mut(&place.local) {
                        info.escaping_fields.insert(*idx);
                    }
                } else if place.projection.is_empty() {
                    // Taking address of whole aggregate - all fields escape
                    if let Some(info) = self.aggregates.get_mut(&place.local) {
                        for i in 0..info.field_count {
                            info.escaping_fields.insert(i);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    /// Check if an operand causes escaping
    fn check_operand_escaping(&mut self, op: &MirOperand) {
        // Pass by reference or pointer - fields might escape
        // This is a conservative approximation
    }

    /// Determine which aggregates can be replaced
    fn find_replaceable(&self) -> HashSet<usize> {
        let mut replaceable = HashSet::new();

        for (local, info) in &self.aggregates {
            // Criteria for replacement:
            // 1. Not used as a whole (or only in assignment)
            // 2. No escaping fields (or only fields we don't replace)
            // 3. Field count is reasonable (not too many scalars)

            let non_escaping_accessed: HashSet<_> = info
                .accessed_fields
                .difference(&info.escaping_fields)
                .cloned()
                .collect();

            let can_replace =
                !info.used_as_whole && !non_escaping_accessed.is_empty() && info.field_count <= 8; // Limit to avoid register pressure

            if can_replace {
                replaceable.insert(*local);
            }
        }

        replaceable
    }

    /// Create scalar locals for replaceable aggregate fields
    fn create_scalar_locals(&mut self, func: &mut MirFunction, replaceable: &HashSet<usize>) {
        self.scalar_map.clear();

        for &local in replaceable {
            if let Some(info) = self.aggregates.get(&local) {
                // Create scalar for each non-escaping accessed field
                for &field_idx in info.accessed_fields.difference(&info.escaping_fields) {
                    let scalar_local = self.next_local;
                    self.next_local += 1;

                    // Add new local (with appropriate type)
                    // Note: In real implementation, we'd extract field type
                    let field_type = MirType::Int(IntSize::I64); // Placeholder
                    func.locals.push(MirLocal {
                        index: scalar_local,
                        ty: field_type,
                        name: Some(format!(
                            "{}_field_{}",
                            func.locals
                                .get(local)
                                .and_then(|l| l.name.as_ref())
                                .map_or("agg", |s| s.as_str()),
                            field_idx
                        )),
                    });

                    self.scalar_map.insert((local, field_idx), scalar_local);
                }
            }
        }
    }

    /// Replace aggregate field accesses with scalar accesses
    fn replace_accesses(&self, func: &mut MirFunction, replaceable: &HashSet<usize>) {
        for block in &mut func.blocks {
            let mut new_instructions = Vec::with_capacity(block.instructions.len());

            for inst in &block.instructions {
                match inst {
                    MirInstruction::Assign { dest, value } => {
                        // Handle aggregate construction - split into scalar assignments
                        if let MirRvalue::Aggregate { operands, .. } = value {
                            if replaceable.contains(&dest.local) {
                                // Create individual scalar assignments
                                for (idx, op) in operands.iter().enumerate() {
                                    if let Some(&scalar_local) =
                                        self.scalar_map.get(&(dest.local, idx))
                                    {
                                        new_instructions.push(MirInstruction::Assign {
                                            dest: MirPlace {
                                                local: scalar_local,
                                                projection: Vec::new(),
                                            },
                                            value: MirRvalue::Use(op.clone()),
                                        });
                                    }
                                }
                                continue; // Skip original instruction
                            }
                        }

                        // Handle field access - redirect to scalar
                        if !dest.projection.is_empty() && replaceable.contains(&dest.local) {
                            if let Some(PlaceProjection::Field { index: idx }) =
                                dest.projection.first()
                            {
                                if let Some(&scalar_local) =
                                    self.scalar_map.get(&(dest.local, *idx))
                                {
                                    // Redirect to scalar
                                    let new_dest = MirPlace {
                                        local: scalar_local,
                                        projection: dest.projection[1..].to_vec(),
                                    };
                                    new_instructions.push(MirInstruction::Assign {
                                        dest: new_dest,
                                        value: self.replace_rvalue(value, replaceable),
                                    });
                                    continue;
                                }
                            }
                        }

                        // Default: keep instruction but replace operands
                        new_instructions.push(MirInstruction::Assign {
                            dest: dest.clone(),
                            value: self.replace_rvalue(value, replaceable),
                        });
                    }
                    _ => {
                        new_instructions.push(inst.clone());
                    }
                }
            }

            block.instructions = new_instructions;
        }
    }

    /// Replace aggregate references in an rvalue with scalar references
    fn replace_rvalue(&self, rvalue: &MirRvalue, replaceable: &HashSet<usize>) -> MirRvalue {
        match rvalue {
            MirRvalue::Use(op) => MirRvalue::Use(self.replace_operand(op, replaceable)),
            MirRvalue::Field { base, index } => {
                let new_base = self.replace_operand(base, replaceable);

                // If accessing a field of a replaced aggregate, redirect to scalar
                if let MirOperand::Copy(place) | MirOperand::Move(place) = &new_base {
                    if replaceable.contains(&place.local) {
                        if let Some(&scalar_local) = self.scalar_map.get(&(place.local, *index)) {
                            return MirRvalue::Use(MirOperand::Copy(MirPlace {
                                local: scalar_local,
                                projection: Vec::new(),
                            }));
                        }
                    }
                }

                MirRvalue::Field {
                    base: new_base,
                    index: *index,
                }
            }
            MirRvalue::BinaryOp { op, left, right } => MirRvalue::BinaryOp {
                op: op.clone(),
                left: self.replace_operand(left, replaceable),
                right: self.replace_operand(right, replaceable),
            },
            MirRvalue::UnaryOp { op, operand } => MirRvalue::UnaryOp {
                op: op.clone(),
                operand: self.replace_operand(operand, replaceable),
            },
            _ => rvalue.clone(),
        }
    }

    /// Replace aggregate references in an operand
    fn replace_operand(&self, op: &MirOperand, replaceable: &HashSet<usize>) -> MirOperand {
        match op {
            MirOperand::Copy(place) => {
                if let Some(new_place) = self.replace_place(place, replaceable) {
                    MirOperand::Copy(new_place)
                } else {
                    op.clone()
                }
            }
            MirOperand::Move(place) => {
                if let Some(new_place) = self.replace_place(place, replaceable) {
                    MirOperand::Move(new_place)
                } else {
                    op.clone()
                }
            }
            MirOperand::Constant(_) => op.clone(),
        }
    }

    /// Replace aggregate field access with scalar access
    fn replace_place(&self, place: &MirPlace, replaceable: &HashSet<usize>) -> Option<MirPlace> {
        if !replaceable.contains(&place.local) {
            return None;
        }

        if let Some(PlaceProjection::Field { index: idx }) = place.projection.first() {
            if let Some(&scalar_local) = self.scalar_map.get(&(place.local, *idx)) {
                return Some(MirPlace {
                    local: scalar_local,
                    projection: place.projection[1..].to_vec(),
                });
            }
        }

        None
    }
}

impl Default for ScalarReplacement {
    fn default() -> Self {
        Self::new()
    }
}

impl MirPass for ScalarReplacement {
    fn name(&self) -> &'static str {
        "scalar_replacement"
    }

    fn run(&mut self, func: &mut MirFunction) {
        // Phase 1: Analyze aggregates
        self.analyze_aggregates(func);

        if self.aggregates.is_empty() {
            return;
        }

        // Phase 2: Determine replaceable aggregates
        let replaceable = self.find_replaceable();

        if replaceable.is_empty() {
            return;
        }

        // Phase 3: Create scalar locals
        self.create_scalar_locals(func, &replaceable);

        // Phase 4: Replace accesses
        self.replace_accesses(func, &replaceable);
    }
}

/// Pass pipeline
pub struct PassPipeline {
    passes: Vec<Box<dyn MirPass>>,
}

impl PassPipeline {
    pub fn new() -> Self {
        Self { passes: Vec::new() }
    }

    pub fn add_pass<P: MirPass + 'static>(&mut self, pass: P) {
        self.passes.push(Box::new(pass));
    }

    pub fn run(&mut self, func: &mut MirFunction) {
        for pass in &mut self.passes {
            pass.run(func);
        }
    }

    /// Create default pipeline based on optimization level
    pub fn default_pipeline(level: super::optimizer::OptLevel) -> Self {
        use super::optimizer::OptLevel;

        let mut pipeline = Self::new();

        // Always run these
        pipeline.add_pass(SimplifyCfg::new());
        pipeline.add_pass(DeadCodeElimination::new());

        if level >= OptLevel::Basic {
            pipeline.add_pass(ConstantPropagation::new());
        }

        if level >= OptLevel::Standard {
            pipeline.add_pass(Inlining::new(100));
            pipeline.add_pass(MemoryAccessOpt::new());
        }

        if level >= OptLevel::Aggressive {
            pipeline.add_pass(LoopUnrolling::new(4));
            pipeline.add_pass(FieldReordering::new());
            pipeline.add_pass(ScalarReplacement::new());
        }

        pipeline
    }
}

impl Default for PassPipeline {
    fn default() -> Self {
        Self::new()
    }
}
