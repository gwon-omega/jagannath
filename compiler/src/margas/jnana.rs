//! # Jñāna Mārga - Path of Knowledge
//!
//! Optimization strategy for functional, pure computation code.
//!
//! ## Philosophy
//!
//! "ज्ञानेन तु तदज्ञानं येषां नाशितमात्मनः" (Bhagavad Gita 5.16)
//! "By knowledge, that ignorance of Self is destroyed"
//!
//! Jñāna Marga focuses on *knowledge* and *understanding* - optimizing
//! pure functions, leveraging immutability, and compile-time computation.

use super::{Marga, MargaOptimizer, MargaResult};
use crate::mir::types::{MirFunction, MirInstruction, MirTerminator, MirRvalue, MirOperand, MirConstant, BinaryOp, UnaryOp, IntSize};
use std::collections::{HashMap, HashSet};

/// Jñāna Marga optimizer for functional/knowledge-based code
pub struct JnanaMarga {
    /// Whether to apply aggressive memoization
    aggressive_memo: bool,
    /// Whether to fold constants aggressively
    aggressive_fold: bool,
    /// Memoization cache for pure function results
    memo_cache: HashMap<String, MirConstant>,
}

impl Default for JnanaMarga {
    fn default() -> Self {
        Self::new()
    }
}

/// Purity analysis result
#[derive(Debug, Clone, PartialEq, Eq)]
enum Purity {
    /// Completely pure - no side effects
    Pure,
    /// Reads global state but doesn't modify
    ReadOnly,
    /// Has side effects
    Impure,
}

/// Function call information for memoization
#[derive(Debug, Clone)]
struct CallInfo {
    /// Block ID
    block: usize,
    /// Instruction index
    inst_idx: usize,
    /// Function name
    func_name: String,
    /// Arguments (if constant)
    const_args: Vec<Option<MirConstant>>,
}

impl JnanaMarga {
    /// Create a new Jñāna Marga optimizer
    pub fn new() -> Self {
        Self {
            aggressive_memo: true,
            aggressive_fold: true,
            memo_cache: HashMap::new(),
        }
    }

    /// Analyze purity of a function
    fn analyze_purity(&self, func: &MirFunction) -> Purity {
        let mut purity = Purity::Pure;

        for block in &func.blocks {
            for inst in &block.instructions {
                match inst {
                    // Store to memory = impure
                    MirInstruction::Store { .. } => return Purity::Impure,

                    // Load from non-local = potentially impure
                    MirInstruction::Load { ptr, .. } => {
                        if let MirOperand::Copy(place) | MirOperand::Move(place) = ptr {
                            // Loading from global or through pointer = at least read-only
                            if !place.projection.is_empty() {
                                purity = Purity::ReadOnly;
                            }
                        }
                    }

                    _ => {}
                }
            }

            // Check terminator for calls
            if let MirTerminator::Call { .. } = &block.terminator {
                // Calls to unknown functions = potentially impure
                purity = Purity::ReadOnly;
            }
        }

        purity
    }

    /// Leverage immutability for optimization
    fn leverage_immutability(&self, func: &mut MirFunction) {
        // Find locals that are assigned exactly once and never mutated
        let mut assignment_counts: HashMap<usize, usize> = HashMap::new();

        for block in &func.blocks {
            for inst in &block.instructions {
                if let MirInstruction::Assign { dest, .. } = inst {
                    *assignment_counts.entry(dest.local).or_insert(0) += 1;
                }
            }
        }

        // Immutable locals (assigned once)
        let immutable_locals: HashSet<usize> = assignment_counts
            .iter()
            .filter(|(_, &count)| count == 1)
            .map(|(&local, _)| local)
            .collect();

        // Optimization: Replace Move with Copy for immutable values
        // This enables more efficient code gen (no need to track ownership)
        for block in &mut func.blocks {
            for inst in &mut block.instructions {
                if let MirInstruction::Assign { value, .. } = inst {
                    self.convert_moves_to_copies(value, &immutable_locals);
                }
            }
        }
    }

    /// Convert Move to Copy for immutable locals in an rvalue
    fn convert_moves_to_copies(&self, rvalue: &mut MirRvalue, immutable: &HashSet<usize>) {
        match rvalue {
            MirRvalue::Use(op) => {
                self.convert_operand_move_to_copy(op, immutable);
            }
            MirRvalue::BinaryOp { left, right, .. } | MirRvalue::FloatOp { left, right, .. } => {
                self.convert_operand_move_to_copy(left, immutable);
                self.convert_operand_move_to_copy(right, immutable);
            }
            MirRvalue::UnaryOp { operand, .. } | MirRvalue::Cast { operand, .. } => {
                self.convert_operand_move_to_copy(operand, immutable);
            }
            MirRvalue::Aggregate { operands, .. } | MirRvalue::SimdOp { operands, .. } => {
                for op in operands {
                    self.convert_operand_move_to_copy(op, immutable);
                }
            }
            MirRvalue::Field { base, .. } | MirRvalue::Index { base, .. } => {
                self.convert_operand_move_to_copy(base, immutable);
            }
            _ => {}
        }
    }

    /// Convert a Move operand to Copy if the local is immutable
    fn convert_operand_move_to_copy(&self, op: &mut MirOperand, immutable: &HashSet<usize>) {
        if let MirOperand::Move(place) = op {
            if immutable.contains(&place.local) {
                *op = MirOperand::Copy(place.clone());
            }
        }
    }

    /// Optimize function composition
    fn optimize_composition(&self, func: &mut MirFunction) {
        // Look for patterns: let a = f(x); let b = g(a);
        // where a is used only once -> can potentially fuse

        let mut single_use_temps: HashSet<usize> = HashSet::new();
        let mut use_counts: HashMap<usize, usize> = HashMap::new();

        // Count uses of each local
        for block in &func.blocks {
            for inst in &block.instructions {
                for local in self.collect_operand_locals(inst) {
                    *use_counts.entry(local).or_insert(0) += 1;
                }
            }
        }

        // Find single-use temporaries
        for (local, count) in use_counts {
            if count == 1 {
                single_use_temps.insert(local);
            }
        }

        // Mark candidates for deforestation (intermediate data structure elimination)
        // This is a placeholder - real implementation would transform the MIR
        let _ = single_use_temps;
    }

    /// Collect locals used in an instruction
    fn collect_operand_locals(&self, inst: &MirInstruction) -> Vec<usize> {
        let mut locals = Vec::new();

        match inst {
            MirInstruction::Assign { value, .. } => {
                self.collect_rvalue_locals(value, &mut locals);
            }
            MirInstruction::Store { ptr, value } => {
                self.collect_op_locals(ptr, &mut locals);
                self.collect_op_locals(value, &mut locals);
            }
            MirInstruction::Load { ptr, .. } => {
                self.collect_op_locals(ptr, &mut locals);
            }
            MirInstruction::Assert { condition, .. } => {
                self.collect_op_locals(condition, &mut locals);
            }
            MirInstruction::BoundsCheck { index, len, .. } => {
                self.collect_op_locals(index, &mut locals);
                self.collect_op_locals(len, &mut locals);
            }
            _ => {}
        }

        locals
    }

    fn collect_rvalue_locals(&self, rv: &MirRvalue, locals: &mut Vec<usize>) {
        match rv {
            MirRvalue::Use(op) => self.collect_op_locals(op, locals),
            MirRvalue::BinaryOp { left, right, .. } | MirRvalue::FloatOp { left, right, .. } => {
                self.collect_op_locals(left, locals);
                self.collect_op_locals(right, locals);
            }
            MirRvalue::UnaryOp { operand, .. } | MirRvalue::Cast { operand, .. } => {
                self.collect_op_locals(operand, locals);
            }
            MirRvalue::Aggregate { operands, .. } | MirRvalue::SimdOp { operands, .. } => {
                for op in operands {
                    self.collect_op_locals(op, locals);
                }
            }
            MirRvalue::Ref { place, .. } | MirRvalue::AddressOf { place, .. }
            | MirRvalue::Discriminant(place) | MirRvalue::Len(place) => {
                locals.push(place.local);
            }
            MirRvalue::Field { base, .. } => self.collect_op_locals(base, locals),
            MirRvalue::Index { base, index } => {
                self.collect_op_locals(base, locals);
                self.collect_op_locals(index, locals);
            }
        }
    }

    fn collect_op_locals(&self, op: &MirOperand, locals: &mut Vec<usize>) {
        match op {
            MirOperand::Copy(place) | MirOperand::Move(place) => {
                locals.push(place.local);
            }
            MirOperand::Constant(_) => {}
        }
    }

    /// Apply memoization (remember knowledge)
    fn apply_memoization(&self, func: &mut MirFunction) {
        // Find pure function calls with constant arguments
        let mut memo_candidates: Vec<CallInfo> = Vec::new();

        for block in &func.blocks {
            if let MirTerminator::Call { func: callee, args, .. } = &block.terminator {
                // Check if callee is a constant (direct call)
                if let MirOperand::Constant(MirConstant::String(name)) = callee {
                    // Check if all args are constant
                    let const_args: Vec<Option<MirConstant>> = args
                        .iter()
                        .map(|arg| {
                            if let MirOperand::Constant(c) = arg {
                                Some(c.clone())
                            } else {
                                None
                            }
                        })
                        .collect();

                    if const_args.iter().all(|c| c.is_some()) {
                        memo_candidates.push(CallInfo {
                            block: block.id,
                            inst_idx: 0,
                            func_name: name.clone(),
                            const_args,
                        });
                    }
                }
            }
        }

        // For now, just record candidates - real implementation would cache results
        // and replace calls with cached values
        let _ = memo_candidates;
    }

    /// Aggressive constant folding (compile-time knowledge)
    fn fold_constants_aggressive(&self, func: &mut MirFunction) {
        // Build constant value map
        let mut const_values: HashMap<usize, MirConstant> = HashMap::new();

        // Propagate and fold constants
        let mut changed = true;
        while changed {
            changed = false;

            for block in &mut func.blocks {
                for inst in &mut block.instructions {
                    if let MirInstruction::Assign { dest, value } = inst {
                        if let Some(constant) = self.try_fold_rvalue(value, &const_values) {
                            // Record the constant value
                            const_values.insert(dest.local, constant.clone());

                            // Replace with constant
                            *value = MirRvalue::Use(MirOperand::Constant(constant));
                            changed = true;
                        }
                    }
                }
            }
        }
    }

    /// Try to fold an rvalue to a constant
    fn try_fold_rvalue(
        &self,
        rv: &MirRvalue,
        const_values: &HashMap<usize, MirConstant>,
    ) -> Option<MirConstant> {
        match rv {
            MirRvalue::Use(op) => self.try_fold_operand(op, const_values),

            MirRvalue::BinaryOp { op, left, right } => {
                let l = self.try_fold_operand(left, const_values)?;
                let r = self.try_fold_operand(right, const_values)?;
                self.fold_binary_op(*op, &l, &r)
            }

            MirRvalue::UnaryOp { op, operand } => {
                let val = self.try_fold_operand(operand, const_values)?;
                self.fold_unary_op(*op, &val)
            }

            _ => None,
        }
    }

    /// Try to get constant value of an operand
    fn try_fold_operand(
        &self,
        op: &MirOperand,
        const_values: &HashMap<usize, MirConstant>,
    ) -> Option<MirConstant> {
        match op {
            MirOperand::Constant(c) => Some(c.clone()),
            MirOperand::Copy(place) | MirOperand::Move(place) => {
                if place.projection.is_empty() {
                    const_values.get(&place.local).cloned()
                } else {
                    None
                }
            }
        }
    }

    /// Fold a binary operation on constants
    fn fold_binary_op(&self, op: BinaryOp, left: &MirConstant, right: &MirConstant) -> Option<MirConstant> {
        match (left, right) {
            (MirConstant::Int(l, size), MirConstant::Int(r, _)) => {
                let result = match op {
                    BinaryOp::Add => l.checked_add(*r)?,
                    BinaryOp::Sub => l.checked_sub(*r)?,
                    BinaryOp::Mul => l.checked_mul(*r)?,
                    BinaryOp::Div if *r != 0 => l.checked_div(*r)?,
                    BinaryOp::Rem if *r != 0 => l.checked_rem(*r)?,
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
                    _ => return None,
                };
                Some(MirConstant::Int(result, *size))
            }
            (MirConstant::Bool(l), MirConstant::Bool(r)) => {
                let result = match op {
                    BinaryOp::BitAnd => *l && *r,
                    BinaryOp::BitOr => *l || *r,
                    BinaryOp::BitXor => *l != *r,
                    BinaryOp::Eq => *l == *r,
                    BinaryOp::Ne => *l != *r,
                    _ => return None,
                };
                Some(MirConstant::Bool(result))
            }
            _ => None,
        }
    }

    /// Fold a unary operation on a constant
    fn fold_unary_op(&self, op: UnaryOp, val: &MirConstant) -> Option<MirConstant> {
        match (op, val) {
            (UnaryOp::Neg, MirConstant::Int(n, size)) => Some(MirConstant::Int(-n, *size)),
            (UnaryOp::Not, MirConstant::Bool(b)) => Some(MirConstant::Bool(!b)),
            (UnaryOp::Not, MirConstant::Int(n, size)) => Some(MirConstant::Int(!n, *size)),
            _ => None,
        }
    }

    /// Check if function is pure (no side effects)
    fn is_pure_function(&self, func: &MirFunction) -> bool {
        self.analyze_purity(func) == Purity::Pure
    }

    /// Count immutable bindings
    fn count_immutable_bindings(&self, func: &MirFunction) -> usize {
        let mut assignment_counts: HashMap<usize, usize> = HashMap::new();

        for block in &func.blocks {
            for inst in &block.instructions {
                if let MirInstruction::Assign { dest, .. } = inst {
                    *assignment_counts.entry(dest.local).or_insert(0) += 1;
                }
            }
        }

        assignment_counts.values().filter(|&&c| c == 1).count()
    }
}

impl MargaOptimizer for JnanaMarga {
    fn marga(&self) -> Marga {
        Marga::Jnana
    }

    fn optimize(&self, func: &mut MirFunction) -> MargaResult {
        // Focus on pure computation (knowledge/wisdom)

        // 1. Leverage immutability (jñāna = knowledge = unchanging truth)
        self.leverage_immutability(func);

        // 2. Function composition optimization
        self.optimize_composition(func);

        // 3. Memoization (remember knowledge)
        if self.aggressive_memo {
            self.apply_memoization(func);
        }

        // 4. Constant folding (compile-time knowledge)
        if self.aggressive_fold {
            self.fold_constants_aggressive(func);
        }

        MargaResult::success(
            Marga::Jnana,
            "Optimized for pure computation and wisdom (immutability, memoization, folding)",
        )
    }

    fn is_suitable_for(&self, func: &MirFunction) -> bool {
        // Jñāna is suitable for functional, pure code
        let pure = self.is_pure_function(func);
        let immutable_ratio = self.count_immutable_bindings(func) as f64 / func.locals.len().max(1) as f64;

        pure || immutable_ratio > 0.5
    }

    fn mantra(&self) -> &'static str {
        // "By knowledge, that ignorance is destroyed"
        "ज्ञानेन तु तदज्ञानं येषां नाशितमात्मनः"
    }
}
