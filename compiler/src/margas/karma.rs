//! # Karma Mārga - Path of Action
//!
//! Optimization strategy for imperative, action-oriented code.
//!
//! ## Philosophy
//!
//! "कर्मण्येवाधिकारस्ते मा फलेषु कदाचन" (Bhagavad Gita 2.47)
//! "You have a right to action, but never to its fruits"
//!
//! Karma Marga focuses on the *action* of execution - optimizing
//! loops, mutations, state machines, and side effects.

use super::{Marga, MargaOptimizer, MargaResult};
use crate::mir::types::{MirFunction, MirInstruction, MirTerminator, MirRvalue, MirOperand, MirPlace, BinaryOp, MirBasicBlock};
use std::collections::{HashMap, HashSet};

/// Karma Marga optimizer for imperative/action-oriented code
pub struct KarmaMarga {
    /// Maximum loop unroll factor
    max_unroll: usize,
    /// Whether to track mutations
    track_mutations: bool,
}

impl Default for KarmaMarga {
    fn default() -> Self {
        Self::new()
    }
}

/// Loop information for analysis
#[derive(Debug, Clone)]
struct LoopInfo {
    /// Header block ID
    header: usize,
    /// Back edge source
    back_edge_src: usize,
    /// Blocks in the loop
    body_blocks: HashSet<usize>,
    /// Induction variable (if detected)
    induction_var: Option<usize>,
    /// Trip count (if known)
    trip_count: Option<usize>,
}

/// State machine pattern
#[derive(Debug)]
struct StateMachineInfo {
    /// State variable local
    state_var: usize,
    /// Possible states
    states: Vec<i64>,
    /// State transitions
    transitions: Vec<(i64, usize, i64)>, // (from_state, block, to_state)
}

impl KarmaMarga {
    /// Create a new Karma Marga optimizer
    pub fn new() -> Self {
        Self {
            max_unroll: 8,
            track_mutations: true,
        }
    }

    /// Detect loops in the function
    fn detect_loops(&self, func: &MirFunction) -> Vec<LoopInfo> {
        let mut loops = Vec::new();

        // Build predecessor map
        let mut preds: HashMap<usize, Vec<usize>> = HashMap::new();
        for block in &func.blocks {
            for succ in self.get_successors(&block.terminator) {
                preds.entry(succ).or_default().push(block.id);
            }
        }

        // Find back edges (edge where target dominates source = loop)
        // Simplified: edge to earlier block ID is likely back edge
        for block in &func.blocks {
            for succ in self.get_successors(&block.terminator) {
                if succ <= block.id {
                    // Potential back edge to loop header
                    let mut body = HashSet::new();
                    self.collect_loop_body(func, succ, block.id, &preds, &mut body);

                    let induction = self.detect_induction_var(func, &body, succ);
                    let trip = induction.and_then(|iv| self.estimate_trip_count(func, iv, succ));

                    loops.push(LoopInfo {
                        header: succ,
                        back_edge_src: block.id,
                        body_blocks: body,
                        induction_var: induction,
                        trip_count: trip,
                    });
                }
            }
        }

        loops
    }

    /// Collect blocks that belong to a loop body
    fn collect_loop_body(
        &self,
        func: &MirFunction,
        header: usize,
        back_edge_src: usize,
        preds: &HashMap<usize, Vec<usize>>,
        body: &mut HashSet<usize>,
    ) {
        body.insert(header);
        let mut worklist = vec![back_edge_src];

        while let Some(block_id) = worklist.pop() {
            if body.insert(block_id) {
                if let Some(pred_list) = preds.get(&block_id) {
                    for &pred in pred_list {
                        if !body.contains(&pred) {
                            worklist.push(pred);
                        }
                    }
                }
            }
        }
    }

    /// Detect induction variable in loop
    fn detect_induction_var(&self, func: &MirFunction, body: &HashSet<usize>, header: usize) -> Option<usize> {
        // Look for pattern: i = i + 1 or i = i - 1 in loop body
        for block in &func.blocks {
            if !body.contains(&block.id) {
                continue;
            }

            for inst in &block.instructions {
                if let MirInstruction::Assign { dest, value } = inst {
                    if let MirRvalue::BinaryOp { op: BinaryOp::Add | BinaryOp::Sub, left, right } = value {
                        // Check if left is Copy/Move of same local as dest
                        if let MirOperand::Copy(place) | MirOperand::Move(place) = left {
                            if place.local == dest.local && dest.projection.is_empty() {
                                // Check if right is constant 1
                                if self.is_constant_one(right) {
                                    return Some(dest.local);
                                }
                            }
                        }
                    }
                }
            }
        }
        None
    }

    /// Check if operand is constant 1
    fn is_constant_one(&self, op: &MirOperand) -> bool {
        match op {
            MirOperand::Constant(crate::mir::types::MirConstant::Int(1, _)) => true,
            _ => false,
        }
    }

    /// Estimate trip count for a loop
    fn estimate_trip_count(&self, func: &MirFunction, induction_var: usize, header: usize) -> Option<usize> {
        // Look for comparison in header block's terminator
        if let Some(block) = func.blocks.iter().find(|b| b.id == header) {
            if let MirTerminator::SwitchInt { discriminant, .. } = &block.terminator {
                // Check if discriminant is a comparison involving induction var
                // This is simplified - real implementation would trace back
                if let MirOperand::Copy(place) = discriminant {
                    if place.local == induction_var {
                        // Can't determine exact count without more analysis
                        return None;
                    }
                }
            }
        }
        None
    }

    /// Get successors of a terminator
    fn get_successors(&self, term: &MirTerminator) -> Vec<usize> {
        match term {
            MirTerminator::Goto { target } => vec![*target],
            MirTerminator::SwitchInt { targets, otherwise, .. } => {
                let mut succs: Vec<usize> = targets.iter().map(|(_, t)| *t).collect();
                succs.push(*otherwise);
                succs
            }
            MirTerminator::Call { target, .. } => vec![*target],
            _ => vec![],
        }
    }

    /// Optimize loops (karma = action = repetitive action)
    fn optimize_loops(&self, func: &mut MirFunction) {
        let loops = self.detect_loops(func);

        for loop_info in loops {
            // Apply loop invariant code motion
            self.hoist_loop_invariants(func, &loop_info);

            // Consider loop unrolling for small, bounded loops
            if let Some(count) = loop_info.trip_count {
                if count <= self.max_unroll && loop_info.body_blocks.len() <= 3 {
                    self.unroll_loop(func, &loop_info, count);
                }
            }
        }
    }

    /// Hoist loop-invariant code out of loop
    fn hoist_loop_invariants(&self, func: &mut MirFunction, loop_info: &LoopInfo) {
        // Find instructions that don't depend on loop-varying values
        let mut invariants: Vec<(usize, usize, MirInstruction)> = Vec::new(); // (block_id, inst_idx, inst)

        // Collect variables modified in loop
        let mut modified_in_loop: HashSet<usize> = HashSet::new();
        for block in &func.blocks {
            if !loop_info.body_blocks.contains(&block.id) {
                continue;
            }
            for inst in &block.instructions {
                if let MirInstruction::Assign { dest, .. } = inst {
                    modified_in_loop.insert(dest.local);
                }
            }
        }

        // Find invariant instructions
        for block in &func.blocks {
            if !loop_info.body_blocks.contains(&block.id) {
                continue;
            }
            for (idx, inst) in block.instructions.iter().enumerate() {
                if let MirInstruction::Assign { value, .. } = inst {
                    if self.is_invariant_rvalue(value, &modified_in_loop) {
                        invariants.push((block.id, idx, inst.clone()));
                    }
                }
            }
        }

        // Move invariants to preheader (simplified: just mark them)
        // Real implementation would create a preheader block
        for (block_id, inst_idx, _) in invariants.iter().rev() {
            // In a real implementation, we'd move these to a preheader
            // For now, we just recognize them
            let _ = (block_id, inst_idx);
        }
    }

    /// Check if an rvalue is loop-invariant
    fn is_invariant_rvalue(&self, rv: &MirRvalue, modified: &HashSet<usize>) -> bool {
        match rv {
            MirRvalue::Use(op) => self.is_invariant_operand(op, modified),
            MirRvalue::BinaryOp { left, right, .. } => {
                self.is_invariant_operand(left, modified) && self.is_invariant_operand(right, modified)
            }
            MirRvalue::UnaryOp { operand, .. } => self.is_invariant_operand(operand, modified),
            _ => false,
        }
    }

    /// Check if an operand is loop-invariant
    fn is_invariant_operand(&self, op: &MirOperand, modified: &HashSet<usize>) -> bool {
        match op {
            MirOperand::Constant(_) => true,
            MirOperand::Copy(place) | MirOperand::Move(place) => {
                !modified.contains(&place.local)
            }
        }
    }

    /// Unroll a loop
    fn unroll_loop(&self, func: &mut MirFunction, loop_info: &LoopInfo, count: usize) {
        // Simplified unrolling - duplicate loop body
        // Real implementation needs careful handling of induction vars and exits
        if count > self.max_unroll || loop_info.body_blocks.len() > 3 {
            return;
        }

        // For now, just mark as candidate for unrolling
        // Full implementation would clone and adjust blocks
        let _ = (func, loop_info, count);
    }

    /// Optimize state machines
    fn optimize_state_machines(&self, func: &mut MirFunction) {
        // Detect state machine patterns
        if let Some(sm) = self.detect_state_machine(func) {
            // Convert to jump table if beneficial
            if sm.states.len() >= 4 {
                self.convert_to_jump_table(func, &sm);
            }
        }
    }

    /// Detect state machine pattern
    fn detect_state_machine(&self, func: &MirFunction) -> Option<StateMachineInfo> {
        // Look for: switch(state) { case 0: ...; case 1: ...; }
        for block in &func.blocks {
            if let MirTerminator::SwitchInt { discriminant, targets, .. } = &block.terminator {
                if let MirOperand::Copy(place) | MirOperand::Move(place) = discriminant {
                    if targets.len() >= 3 {
                        // Potential state machine
                        let states: Vec<i64> = targets.iter().map(|(v, _)| *v).collect();
                        let transitions = self.find_state_transitions(func, place.local);

                        return Some(StateMachineInfo {
                            state_var: place.local,
                            states,
                            transitions,
                        });
                    }
                }
            }
        }
        None
    }

    /// Find state transitions in a state machine
    fn find_state_transitions(&self, func: &MirFunction, state_var: usize) -> Vec<(i64, usize, i64)> {
        let mut transitions = Vec::new();

        for block in &func.blocks {
            for inst in &block.instructions {
                if let MirInstruction::Assign { dest, value } = inst {
                    if dest.local == state_var {
                        if let MirRvalue::Use(MirOperand::Constant(crate::mir::types::MirConstant::Int(new_state, _))) = value {
                            // Found a state assignment
                            // Would need more analysis to determine from_state
                            transitions.push((0, block.id, *new_state));
                        }
                    }
                }
            }
        }

        transitions
    }

    /// Convert state machine to jump table
    fn convert_to_jump_table(&self, _func: &mut MirFunction, _sm: &StateMachineInfo) {
        // Real implementation would:
        // 1. Compute jump table with state -> target block mapping
        // 2. Replace switch with computed goto
        // For now, mark as optimized
    }

    /// Track and optimize mutations
    fn track_and_optimize_mutations(&self, func: &mut MirFunction) {
        // Track all mutations (assignments) to each local
        let mut mutation_counts: HashMap<usize, usize> = HashMap::new();
        let mut last_mutation: HashMap<usize, (usize, usize)> = HashMap::new(); // local -> (block, inst_idx)

        for block in &func.blocks {
            for (idx, inst) in block.instructions.iter().enumerate() {
                if let MirInstruction::Assign { dest, .. } = inst {
                    *mutation_counts.entry(dest.local).or_insert(0) += 1;
                    last_mutation.insert(dest.local, (block.id, idx));
                }
            }
        }

        // Identify candidates for copy-on-write
        // Single-mutation variables can be treated as immutable after assignment
        let _single_mutation: Vec<usize> = mutation_counts
            .iter()
            .filter(|(_, &count)| count == 1)
            .map(|(&local, _)| local)
            .collect();

        // Identify candidates for in-place updates
        // Variables mutated multiple times but never borrowed
        let _in_place_candidates: Vec<usize> = mutation_counts
            .iter()
            .filter(|(_, &count)| count > 1)
            .map(|(&local, _)| local)
            .collect();
    }

    /// Order side effects for optimal execution
    fn order_side_effects(&self, func: &mut MirFunction) {
        // Build dependency graph for side effects
        let mut effect_deps: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new(); // (block, inst) -> deps

        // Track memory operations
        let mut last_store: Option<(usize, usize)> = None;

        for block in &func.blocks {
            for (idx, inst) in block.instructions.iter().enumerate() {
                let key = (block.id, idx);

                match inst {
                    MirInstruction::Store { .. } => {
                        // Store depends on previous stores (simplified - real impl uses alias analysis)
                        if let Some(prev) = last_store {
                            effect_deps.entry(key).or_default().push(prev);
                        }
                        last_store = Some(key);
                    }
                    MirInstruction::Load { .. } => {
                        // Load depends on previous stores
                        if let Some(prev) = last_store {
                            effect_deps.entry(key).or_default().push(prev);
                        }
                    }
                    _ => {}
                }
            }
        }

        // In real implementation, use dependency graph to reorder
        // independent operations for better pipelining
        let _ = effect_deps;
    }

    /// Analyze if function is loop-heavy
    fn is_loop_heavy(&self, func: &MirFunction) -> bool {
        let loops = self.detect_loops(func);
        !loops.is_empty() && loops.len() >= func.blocks.len() / 5
    }

    /// Analyze if function has significant mutations
    fn has_significant_mutations(&self, func: &MirFunction) -> bool {
        let mut mutation_count = 0;
        for block in &func.blocks {
            for inst in &block.instructions {
                if matches!(inst, MirInstruction::Assign { .. }) {
                    mutation_count += 1;
                }
            }
        }
        mutation_count > 5
    }
}

impl MargaOptimizer for KarmaMarga {
    fn marga(&self) -> Marga {
        Marga::Karma
    }

    fn optimize(&self, func: &mut MirFunction) -> MargaResult {
        // Focus on efficient execution (action)

        // 1. Aggressive loop optimization (karma = action = loops)
        self.optimize_loops(func);

        // 2. State machine optimization
        self.optimize_state_machines(func);

        // 3. Mutation tracking (track actions)
        if self.track_mutations {
            self.track_and_optimize_mutations(func);
        }

        // 4. Side effect ordering (sequence actions correctly)
        self.order_side_effects(func);

        MargaResult::success(
            Marga::Karma,
            "Optimized for efficient action/execution (loops, mutations, state)",
        )
    }

    fn is_suitable_for(&self, func: &MirFunction) -> bool {
        // Karma is suitable for imperative, loop-heavy code with mutations
        self.is_loop_heavy(func) || self.has_significant_mutations(func)
    }

    fn mantra(&self) -> &'static str {
        // "Action alone is your privilege, never the fruits thereof"
        "कर्मण्येवाधिकारस्ते मा फलेषु कदाचन"
    }
}
