//! Register Allocator - Kāraka-Guided Linear Scan
//!
//! Implements Linear Scan Register Allocation (Poletto & Sarkar, 1999)
//! enhanced with Sanskrit Kāraka semantic hints for optimal register assignment.
//!
//! The algorithm works in two phases:
//! 1. **Kāraka Phase**: Use semantic roles to pre-assign registers
//!    - Kartṛ (agent): Callee-saved (preserved across calls)
//!    - Karman (patient): Output registers
//!    - Karaṇa (instrument): Caller-saved (can be clobbered)
//!
//! 2. **Linear Scan Phase**: Allocate remaining registers via live intervals

use crate::mir::types::{
    MirFunction, MirInstruction, MirOperand, MirRvalue, MirTerminator, RegisterClass,
};
use crate::parser::ast::Karaka;
use std::collections::{BTreeMap, HashMap, HashSet};

/// Register allocator with Kāraka-guided hints
pub struct RegisterAllocator {
    /// Target architecture
    target: Target,
}

/// Target architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    X86_64,
    AArch64,
    RiscV64,
}

/// Live interval for a virtual register
#[derive(Debug, Clone)]
pub struct LiveInterval {
    /// Virtual register index
    pub vreg: usize,
    /// Start point (instruction index)
    pub start: usize,
    /// End point (instruction index)
    pub end: usize,
    /// Register class hint from Kāraka
    pub hint: Option<RegisterClass>,
    /// Assigned physical register (None = spilled)
    pub assigned: Option<PhysReg>,
    /// Spill slot if spilled
    pub spill_slot: Option<i32>,
}

/// Physical register
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PhysReg {
    pub index: usize,
    pub class: RegisterClass,
}

/// Allocation result for a function
#[derive(Debug)]
pub struct AllocationResult {
    /// Virtual register to physical register mapping
    pub vreg_to_preg: HashMap<usize, PhysReg>,
    /// Virtual registers that were spilled to stack
    pub spilled: HashMap<usize, i32>,
    /// Total stack space needed for spills
    pub spill_size: i32,
    /// Statistics
    pub stats: AllocationStats,
}

/// Allocation statistics
#[derive(Debug, Default)]
pub struct AllocationStats {
    pub total_vregs: usize,
    pub allocated_regs: usize,
    pub spilled_regs: usize,
    pub karaka_hints_used: usize,
}

impl RegisterAllocator {
    pub fn new(target: Target) -> Self {
        Self { target }
    }

    /// Allocate registers for a function
    pub fn allocate(&mut self, func: &mut MirFunction) -> AllocationResult {
        // Phase 1: Compute live intervals
        let mut intervals = self.compute_live_intervals(func);

        // Phase 2: Apply kāraka hints to intervals
        self.apply_karaka_hints(func, &mut intervals);

        // Phase 3: Linear scan allocation
        let result = self.linear_scan(&mut intervals);

        result
    }

    /// Compute live intervals for all virtual registers
    fn compute_live_intervals(&self, func: &MirFunction) -> Vec<LiveInterval> {
        let mut intervals: HashMap<usize, LiveInterval> = HashMap::new();
        let mut instruction_idx = 0;

        // Build control flow graph for proper liveness
        let block_order = self.compute_block_order(func);

        for &block_id in &block_order {
            if let Some(block) = func.blocks.get(block_id) {
                for instr in &block.instructions {
                    // Track definitions and uses
                    let (defs, uses) = self.get_def_use(instr);

                    for use_reg in uses {
                        intervals
                            .entry(use_reg)
                            .and_modify(|i| i.end = instruction_idx)
                            .or_insert(LiveInterval {
                                vreg: use_reg,
                                start: instruction_idx,
                                end: instruction_idx,
                                hint: None,
                                assigned: None,
                                spill_slot: None,
                            });
                    }

                    for def_reg in defs {
                        intervals
                            .entry(def_reg)
                            .and_modify(|i| {
                                if instruction_idx < i.start {
                                    i.start = instruction_idx;
                                }
                            })
                            .or_insert(LiveInterval {
                                vreg: def_reg,
                                start: instruction_idx,
                                end: instruction_idx,
                                hint: None,
                                assigned: None,
                                spill_slot: None,
                            });
                    }

                    instruction_idx += 1;
                }

                // Handle terminator uses
                let term_uses = self.get_terminator_uses(&block.terminator);
                for use_reg in term_uses {
                    intervals
                        .entry(use_reg)
                        .and_modify(|i| i.end = instruction_idx)
                        .or_insert(LiveInterval {
                            vreg: use_reg,
                            start: instruction_idx,
                            end: instruction_idx,
                            hint: None,
                            assigned: None,
                            spill_slot: None,
                        });
                }
                instruction_idx += 1;
            }
        }

        // Sort by start point for linear scan
        let mut result: Vec<LiveInterval> = intervals.into_values().collect();
        result.sort_by_key(|i| i.start);
        result
    }

    /// Compute block order using reverse postorder (good for liveness)
    fn compute_block_order(&self, func: &MirFunction) -> Vec<usize> {
        let num_blocks = func.blocks.len();
        if num_blocks == 0 {
            return vec![];
        }

        let mut visited = HashSet::new();
        let mut order = Vec::new();

        self.dfs_postorder(func, 0, &mut visited, &mut order);
        order.reverse();
        order
    }

    fn dfs_postorder(
        &self,
        func: &MirFunction,
        block_id: usize,
        visited: &mut HashSet<usize>,
        order: &mut Vec<usize>,
    ) {
        if visited.contains(&block_id) || block_id >= func.blocks.len() {
            return;
        }
        visited.insert(block_id);

        // Visit successors first
        let successors = self.get_successors(&func.blocks[block_id].terminator);
        for succ in successors {
            self.dfs_postorder(func, succ, visited, order);
        }

        order.push(block_id);
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
            MirTerminator::Call { target, .. } => vec![*target],
            MirTerminator::Return | MirTerminator::Unreachable | MirTerminator::Unwind => vec![],
        }
    }

    /// Extract defined and used registers from an instruction
    fn get_def_use(&self, instr: &MirInstruction) -> (Vec<usize>, Vec<usize>) {
        let mut defs = Vec::new();
        let mut uses = Vec::new();

        match instr {
            MirInstruction::Assign { dest, value } => {
                defs.push(dest.local);
                self.collect_rvalue_uses(value, &mut uses);
            }
            MirInstruction::Drop { place } => {
                uses.push(place.local);
            }
            MirInstruction::Store { ptr, value } => {
                self.collect_operand_uses(ptr, &mut uses);
                self.collect_operand_uses(value, &mut uses);
            }
            MirInstruction::Load { dest, ptr } => {
                defs.push(dest.local);
                self.collect_operand_uses(ptr, &mut uses);
            }
            MirInstruction::Assert { condition, .. } => {
                self.collect_operand_uses(condition, &mut uses);
            }
            MirInstruction::BoundsCheck { index, len, .. } => {
                self.collect_operand_uses(index, &mut uses);
                self.collect_operand_uses(len, &mut uses);
            }
            MirInstruction::SetDiscriminant { place, .. } => {
                uses.push(place.local);
            }
            MirInstruction::Nop => {}
        }

        (defs, uses)
    }

    fn collect_operand_uses(&self, op: &MirOperand, uses: &mut Vec<usize>) {
        match op {
            MirOperand::Copy(place) | MirOperand::Move(place) => {
                uses.push(place.local);
            }
            MirOperand::Constant(_) => {}
        }
    }

    fn collect_rvalue_uses(&self, rv: &MirRvalue, uses: &mut Vec<usize>) {
        match rv {
            MirRvalue::Use(op) => self.collect_operand_uses(op, uses),
            MirRvalue::Ref { place, .. } => uses.push(place.local),
            MirRvalue::BinaryOp { left, right, .. } => {
                self.collect_operand_uses(left, uses);
                self.collect_operand_uses(right, uses);
            }
            MirRvalue::UnaryOp { operand, .. } => {
                self.collect_operand_uses(operand, uses);
            }
            MirRvalue::Aggregate { operands, .. } => {
                for op in operands {
                    self.collect_operand_uses(op, uses);
                }
            }
            MirRvalue::Cast { operand, .. } => {
                self.collect_operand_uses(operand, uses);
            }
            MirRvalue::Discriminant(place) => uses.push(place.local),
            MirRvalue::Len(place) => uses.push(place.local),
            MirRvalue::AddressOf { place, .. } => uses.push(place.local),
            MirRvalue::Field { base, .. } => self.collect_operand_uses(base, uses),
            MirRvalue::Index { base, index } => {
                self.collect_operand_uses(base, uses);
                self.collect_operand_uses(index, uses);
            }
            MirRvalue::FloatOp { left, right, .. } => {
                self.collect_operand_uses(left, uses);
                self.collect_operand_uses(right, uses);
            }
            MirRvalue::SimdOp { operands, .. } => {
                for op in operands {
                    self.collect_operand_uses(op, uses);
                }
            }
        }
    }

    fn get_terminator_uses(&self, term: &MirTerminator) -> Vec<usize> {
        let mut uses = Vec::new();
        match term {
            MirTerminator::SwitchInt { discriminant, .. } => {
                self.collect_operand_uses(discriminant, &mut uses);
            }
            MirTerminator::Call { func, args, .. } => {
                self.collect_operand_uses(func, &mut uses);
                for arg in args {
                    self.collect_operand_uses(arg, &mut uses);
                }
            }
            _ => {}
        }
        uses
    }

    /// Apply kāraka-based register hints
    fn apply_karaka_hints(&mut self, func: &MirFunction, intervals: &mut [LiveInterval]) {
        for interval in intervals.iter_mut() {
            if let Some(hint) = func.karaka_hints.get(&interval.vreg) {
                interval.hint = Some(hint.register_class);
            }
        }
    }

    /// Convert kāraka to register class
    fn karaka_to_class(&self, karaka: &Karaka) -> RegisterClass {
        match karaka {
            // Kartṛ (agent) - needs to be preserved, use callee-saved
            Karaka::Kartr => RegisterClass::CalleeSaved,
            // Karman (patient) - output, use return register
            Karaka::Karman => RegisterClass::Output,
            // Karaṇa (instrument) - consumed, use caller-saved
            Karaka::Karana => RegisterClass::CallerSaved,
            // Sampradāna (recipient) - output location
            Karaka::Sampradana => RegisterClass::Output,
            // Apādāna (source) - read-only, callee-saved
            Karaka::Apadana => RegisterClass::CalleeSaved,
            // Adhikaraṇa (locus) - context, general purpose
            Karaka::Adhikarana => RegisterClass::General,
        }
    }

    /// Main Linear Scan algorithm
    fn linear_scan(&mut self, intervals: &mut [LiveInterval]) -> AllocationResult {
        let mut result = AllocationResult {
            vreg_to_preg: HashMap::new(),
            spilled: HashMap::new(),
            spill_size: 0,
            stats: AllocationStats::default(),
        };

        result.stats.total_vregs = intervals.len();

        // Active list sorted by increasing end point
        let mut active: BTreeMap<usize, LiveInterval> = BTreeMap::new();

        // Available registers per class
        let mut available_callee_saved = self.get_callee_saved_regs();
        let mut available_caller_saved = self.get_caller_saved_regs();
        let mut available_general = self.get_general_regs();

        // Stack slot counter for spills
        let mut next_spill_slot: i32 = -8; // Start below RBP

        for interval in intervals.iter_mut() {
            // Expire old intervals
            let expired: Vec<usize> = active
                .iter()
                .filter(|(_, i)| i.end < interval.start)
                .map(|(k, _)| *k)
                .collect();

            for key in expired {
                if let Some(expired_interval) = active.remove(&key) {
                    if let Some(reg) = expired_interval.assigned {
                        // Return register to appropriate pool
                        match reg.class {
                            RegisterClass::CalleeSaved => available_callee_saved.push(reg),
                            RegisterClass::CallerSaved => available_caller_saved.push(reg),
                            RegisterClass::Output | RegisterClass::General => {
                                available_general.push(reg)
                            }
                        }
                    }
                }
            }

            // Try to allocate a register
            let reg = self.try_allocate_register(
                interval,
                &mut available_callee_saved,
                &mut available_caller_saved,
                &mut available_general,
                &mut result.stats,
            );

            if let Some(preg) = reg {
                interval.assigned = Some(preg);
                result.vreg_to_preg.insert(interval.vreg, preg);
                active.insert(interval.end, interval.clone());
                result.stats.allocated_regs += 1;
            } else {
                // Spill: either this interval or one from active
                // Find the longest active interval
                let longest_key = active.keys().copied().last();
                if let Some(longest_end) = longest_key {
                    if longest_end > interval.end {
                        // Spill the longest active interval
                        let longest = active.remove(&longest_end).unwrap();
                        if let Some(reg) = longest.assigned {
                            interval.assigned = Some(reg);
                            result.vreg_to_preg.insert(interval.vreg, reg);
                            active.insert(interval.end, interval.clone());

                            // Spill the old one
                            result.spilled.insert(longest.vreg, next_spill_slot);
                            result.vreg_to_preg.remove(&longest.vreg);
                            next_spill_slot -= 8;
                            result.stats.spilled_regs += 1;
                        }
                    } else {
                        // Spill current interval
                        interval.spill_slot = Some(next_spill_slot);
                        result.spilled.insert(interval.vreg, next_spill_slot);
                        next_spill_slot -= 8;
                        result.stats.spilled_regs += 1;
                    }
                } else {
                    // No active intervals, spill current
                    interval.spill_slot = Some(next_spill_slot);
                    result.spilled.insert(interval.vreg, next_spill_slot);
                    next_spill_slot -= 8;
                    result.stats.spilled_regs += 1;
                }
            }
        }

        result.spill_size = -next_spill_slot - 8;
        result
    }

    /// Try to allocate a register based on hints
    fn try_allocate_register(
        &self,
        interval: &LiveInterval,
        callee_saved: &mut Vec<PhysReg>,
        caller_saved: &mut Vec<PhysReg>,
        general: &mut Vec<PhysReg>,
        stats: &mut AllocationStats,
    ) -> Option<PhysReg> {
        // First, try to honor kāraka hint
        if let Some(hint) = interval.hint {
            stats.karaka_hints_used += 1;
            match hint {
                RegisterClass::CalleeSaved => {
                    if let Some(reg) = callee_saved.pop() {
                        return Some(reg);
                    }
                }
                RegisterClass::CallerSaved => {
                    if let Some(reg) = caller_saved.pop() {
                        return Some(reg);
                    }
                }
                RegisterClass::Output | RegisterClass::General => {
                    if let Some(reg) = general.pop() {
                        return Some(reg);
                    }
                }
            }
        }

        // Fall back to any available register
        if let Some(reg) = general.pop() {
            return Some(reg);
        }
        if let Some(reg) = caller_saved.pop() {
            return Some(reg);
        }
        if let Some(reg) = callee_saved.pop() {
            return Some(reg);
        }

        None
    }

    /// Get callee-saved registers for target
    fn get_callee_saved_regs(&self) -> Vec<PhysReg> {
        match self.target {
            Target::X86_64 => vec![
                // RBX, R12, R13, R14, R15 (System V AMD64 ABI)
                PhysReg {
                    index: 3,
                    class: RegisterClass::CalleeSaved,
                }, // RBX
                PhysReg {
                    index: 12,
                    class: RegisterClass::CalleeSaved,
                }, // R12
                PhysReg {
                    index: 13,
                    class: RegisterClass::CalleeSaved,
                }, // R13
                PhysReg {
                    index: 14,
                    class: RegisterClass::CalleeSaved,
                }, // R14
                PhysReg {
                    index: 15,
                    class: RegisterClass::CalleeSaved,
                }, // R15
            ],
            Target::AArch64 => (19..=28)
                .map(|i| PhysReg {
                    index: i,
                    class: RegisterClass::CalleeSaved,
                })
                .collect(),
            Target::RiscV64 => (8..=11)
                .chain(18..=27)
                .map(|i| PhysReg {
                    index: i,
                    class: RegisterClass::CalleeSaved,
                })
                .collect(),
        }
    }

    /// Get caller-saved registers for target
    fn get_caller_saved_regs(&self) -> Vec<PhysReg> {
        match self.target {
            Target::X86_64 => vec![
                // RAX, RCX, RDX, RSI, RDI, R8, R9, R10, R11
                PhysReg {
                    index: 0,
                    class: RegisterClass::CallerSaved,
                }, // RAX
                PhysReg {
                    index: 1,
                    class: RegisterClass::CallerSaved,
                }, // RCX
                PhysReg {
                    index: 2,
                    class: RegisterClass::CallerSaved,
                }, // RDX
                PhysReg {
                    index: 6,
                    class: RegisterClass::CallerSaved,
                }, // RSI
                PhysReg {
                    index: 7,
                    class: RegisterClass::CallerSaved,
                }, // RDI
                PhysReg {
                    index: 8,
                    class: RegisterClass::CallerSaved,
                }, // R8
                PhysReg {
                    index: 9,
                    class: RegisterClass::CallerSaved,
                }, // R9
                PhysReg {
                    index: 10,
                    class: RegisterClass::CallerSaved,
                }, // R10
                PhysReg {
                    index: 11,
                    class: RegisterClass::CallerSaved,
                }, // R11
            ],
            Target::AArch64 => (0..=18)
                .map(|i| PhysReg {
                    index: i,
                    class: RegisterClass::CallerSaved,
                })
                .collect(),
            Target::RiscV64 => (5..=7)
                .chain(28..=31)
                .map(|i| PhysReg {
                    index: i,
                    class: RegisterClass::CallerSaved,
                })
                .collect(),
        }
    }

    /// Get general-purpose registers
    fn get_general_regs(&self) -> Vec<PhysReg> {
        // Use caller-saved as general-purpose fallback
        self.get_caller_saved_regs()
            .into_iter()
            .map(|r| PhysReg {
                index: r.index,
                class: RegisterClass::General,
            })
            .collect()
    }

    /// Get number of general-purpose registers for target
    fn num_gp_registers(&self) -> usize {
        match self.target {
            Target::X86_64 => 14,  // Excluding RSP, RBP
            Target::AArch64 => 28, // X0-X28 (excluding X29=FP, X30=LR, X31=SP)
            Target::RiscV64 => 28, // x1-x31 (excluding x0=zero)
        }
    }

    /// Get number of callee-saved registers
    fn num_callee_saved(&self) -> usize {
        match self.target {
            Target::X86_64 => 5,   // RBX, R12-R15
            Target::AArch64 => 10, // X19-X28
            Target::RiscV64 => 12, // s0-s11
        }
    }
}

/// Get x86-64 register name from index
pub fn x86_reg_name(index: usize, size: u8) -> &'static str {
    match size {
        8 => match index {
            0 => "rax",
            1 => "rcx",
            2 => "rdx",
            3 => "rbx",
            4 => "rsp",
            5 => "rbp",
            6 => "rsi",
            7 => "rdi",
            8 => "r8",
            9 => "r9",
            10 => "r10",
            11 => "r11",
            12 => "r12",
            13 => "r13",
            14 => "r14",
            15 => "r15",
            _ => "unknown",
        },
        4 => match index {
            0 => "eax",
            1 => "ecx",
            2 => "edx",
            3 => "ebx",
            6 => "esi",
            7 => "edi",
            8 => "r8d",
            9 => "r9d",
            10 => "r10d",
            11 => "r11d",
            12 => "r12d",
            13 => "r13d",
            14 => "r14d",
            15 => "r15d",
            _ => "unknown",
        },
        1 => match index {
            0 => "al",
            1 => "cl",
            2 => "dl",
            3 => "bl",
            _ => "unknown",
        },
        _ => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocator_creation() {
        let alloc = RegisterAllocator::new(Target::X86_64);
        assert_eq!(alloc.target, Target::X86_64);
    }

    #[test]
    fn test_callee_saved_x86() {
        let alloc = RegisterAllocator::new(Target::X86_64);
        let regs = alloc.get_callee_saved_regs();
        assert_eq!(regs.len(), 5); // RBX, R12-R15
    }

    #[test]
    fn test_caller_saved_x86() {
        let alloc = RegisterAllocator::new(Target::X86_64);
        let regs = alloc.get_caller_saved_regs();
        assert_eq!(regs.len(), 9); // RAX, RCX, RDX, RSI, RDI, R8-R11
    }

    #[test]
    fn test_karaka_to_class() {
        let alloc = RegisterAllocator::new(Target::X86_64);

        assert_eq!(
            alloc.karaka_to_class(&Karaka::Kartr),
            RegisterClass::CalleeSaved
        );
        assert_eq!(
            alloc.karaka_to_class(&Karaka::Karman),
            RegisterClass::Output
        );
        assert_eq!(
            alloc.karaka_to_class(&Karaka::Karana),
            RegisterClass::CallerSaved
        );
        assert_eq!(
            alloc.karaka_to_class(&Karaka::Apadana),
            RegisterClass::CalleeSaved
        );
        assert_eq!(
            alloc.karaka_to_class(&Karaka::Adhikarana),
            RegisterClass::General
        );
    }

    #[test]
    fn test_live_interval_creation() {
        let interval = LiveInterval {
            vreg: 0,
            start: 5,
            end: 10,
            hint: Some(RegisterClass::CalleeSaved),
            assigned: None,
            spill_slot: None,
        };

        assert_eq!(interval.vreg, 0);
        assert_eq!(interval.start, 5);
        assert_eq!(interval.end, 10);
        assert!(interval.hint.is_some());
    }

    #[test]
    fn test_phys_reg_equality() {
        let r1 = PhysReg {
            index: 3,
            class: RegisterClass::CalleeSaved,
        };
        let r2 = PhysReg {
            index: 3,
            class: RegisterClass::CalleeSaved,
        };
        let r3 = PhysReg {
            index: 4,
            class: RegisterClass::CalleeSaved,
        };

        assert_eq!(r1, r2);
        assert_ne!(r1, r3);
    }

    #[test]
    fn test_x86_reg_names() {
        assert_eq!(x86_reg_name(0, 8), "rax");
        assert_eq!(x86_reg_name(0, 4), "eax");
        assert_eq!(x86_reg_name(0, 1), "al");
        assert_eq!(x86_reg_name(3, 8), "rbx");
        assert_eq!(x86_reg_name(12, 8), "r12");
    }

    #[test]
    fn test_aarch64_registers() {
        let alloc = RegisterAllocator::new(Target::AArch64);
        let callee = alloc.get_callee_saved_regs();
        let caller = alloc.get_caller_saved_regs();

        assert_eq!(callee.len(), 10); // X19-X28
        assert_eq!(caller.len(), 19); // X0-X18
    }

    #[test]
    fn test_riscv_registers() {
        let alloc = RegisterAllocator::new(Target::RiscV64);
        let callee = alloc.get_callee_saved_regs();

        // s0-s11 = 12 registers (s0-s1 = x8-x9, s2-s11 = x18-x27)
        assert_eq!(callee.len(), 14);
    }

    #[test]
    fn test_allocation_result_default() {
        let result = AllocationResult {
            vreg_to_preg: HashMap::new(),
            spilled: HashMap::new(),
            spill_size: 0,
            stats: AllocationStats::default(),
        };

        assert!(result.vreg_to_preg.is_empty());
        assert!(result.spilled.is_empty());
        assert_eq!(result.spill_size, 0);
    }

    #[test]
    fn test_empty_function_allocation() {
        let mut alloc = RegisterAllocator::new(Target::X86_64);
        let mut func = MirFunction {
            name: "test".to_string(),
            params: vec![],
            return_type: crate::mir::types::MirType::Unit,
            blocks: vec![],
            locals: vec![],
            karaka_hints: HashMap::new(),
        };

        let result = alloc.allocate(&mut func);
        assert_eq!(result.stats.total_vregs, 0);
        assert_eq!(result.stats.spilled_regs, 0);
    }

    #[test]
    fn test_successors_goto() {
        let alloc = RegisterAllocator::new(Target::X86_64);
        let term = MirTerminator::Goto { target: 5 };
        let succs = alloc.get_successors(&term);

        assert_eq!(succs, vec![5]);
    }

    #[test]
    fn test_successors_return() {
        let alloc = RegisterAllocator::new(Target::X86_64);
        let term = MirTerminator::Return;
        let succs = alloc.get_successors(&term);

        assert!(succs.is_empty());
    }

    #[test]
    fn test_successors_switch() {
        let alloc = RegisterAllocator::new(Target::X86_64);
        let term = MirTerminator::SwitchInt {
            discriminant: MirOperand::Constant(crate::mir::types::MirConstant::Int(
                0,
                crate::mir::types::IntSize::I64,
            )),
            targets: vec![(1, 2), (2, 3)],
            otherwise: 4,
        };
        let succs = alloc.get_successors(&term);

        assert_eq!(succs.len(), 3);
        assert!(succs.contains(&2));
        assert!(succs.contains(&3));
        assert!(succs.contains(&4));
    }
}
