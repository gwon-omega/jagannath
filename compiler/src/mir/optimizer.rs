//! MIR Optimizer
//!
//! Applies optimization passes based on Sāṃkhya tattvas (stages).

use super::passes::{ConstantPropagation, DeadCodeElimination, Inlining, MirPass, SimplifyCfg};
use super::types::*;

/// MIR Optimizer
pub struct MirOptimizer {
    /// Optimization level
    level: OptLevel,
    /// Guṇa mode for optimization
    guna: GunaMode,
    /// Dead code elimination pass (Brahmastra)
    dce: DeadCodeElimination,
    /// Inlining pass (Sūkṣmāstra)
    inlining: Inlining,
    /// CFG simplification pass (Vayuastra)
    simplify_cfg: SimplifyCfg,
    /// Constant propagation pass (Agneyastra)
    const_prop: ConstantPropagation,
}

/// Optimization level
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum OptLevel {
    /// No optimization
    None,
    /// Basic optimizations
    Basic,
    /// Standard optimizations
    Standard,
    /// Aggressive optimizations
    Aggressive,
}

/// Guṇa optimization mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GunaMode {
    /// Sattva - Prioritize correctness
    Sattva,
    /// Rajas - Prioritize speed
    Rajas,
    /// Tamas - Prioritize size
    Tamas,
}

impl MirOptimizer {
    pub fn new(level: OptLevel, guna: GunaMode) -> Self {
        Self {
            level,
            guna,
            dce: DeadCodeElimination::new(),
            inlining: Inlining::new(50), // Inline functions up to 50 instructions
            simplify_cfg: SimplifyCfg::new(),
            const_prop: ConstantPropagation::new(),
        }
    }

    /// Optimize a MIR module
    pub fn optimize(&mut self, module: &mut MirModule) {
        if self.level == OptLevel::None {
            return;
        }

        // Register all functions for inlining (cross-function optimization)
        for func in &module.functions {
            self.inlining.register_function(func.clone());
        }

        for func in &mut module.functions {
            self.optimize_function(func);
        }
    }

    /// Optimize a single function
    fn optimize_function(&mut self, func: &mut MirFunction) {
        // Apply passes based on Sāṃkhya tattva ordering
        // (moving from subtle to gross, or inversely for optimization)

        // Buddhi (intellect) - High-level analysis
        self.pass_dead_code_elimination(func);

        // Ahaṃkāra (ego) - Isolation/scoping
        self.pass_inline_small_functions(func);

        // Manas (mind) - Control flow
        self.pass_simplify_cfg(func);

        // Indriyas (senses) - I/O optimization
        self.pass_karaka_register_hints(func);

        // Tanmātras (subtle elements) - Data representation
        self.pass_memory_layout(func);

        // Final cleanup pass
        if self.level >= OptLevel::Standard {
            self.pass_dead_code_elimination(func);
            self.pass_simplify_cfg(func);
        }
    }

    /// Pass: Dead code elimination (Brahmastra - ब्रह्मास्त्र)
    /// The ultimate weapon that removes all unreachable code
    fn pass_dead_code_elimination(&mut self, func: &mut MirFunction) {
        // Invoke Brahmastra for dead code elimination
        self.dce.run(func);
    }

    /// Pass: Inline small functions (Sūkṣmāstra - सूक्ष्मास्त्र)
    /// The subtle weapon that eliminates function call overhead
    fn pass_inline_small_functions(&mut self, func: &mut MirFunction) {
        if self.level < OptLevel::Standard {
            return;
        }
        // Invoke Sūkṣmāstra for inlining
        self.inlining.run(func);
    }

    /// Pass: Simplify control flow graph (Vayuastra - वायव्यास्त्र)
    /// The wind weapon that streamlines control flow
    fn pass_simplify_cfg(&mut self, func: &mut MirFunction) {
        // Invoke Vayuastra for CFG simplification
        self.simplify_cfg.run(func);
    }

    /// Pass: Add kāraka-based register allocation hints
    fn pass_karaka_register_hints(&mut self, func: &mut MirFunction) {
        use crate::parser::ast::Karaka;

        for param in &func.params {
            if let Some(karaka) = param.karaka {
                let register_class = match karaka {
                    Karaka::Kartr => RegisterClass::CalleeSaved,
                    Karaka::Karman => RegisterClass::Output,
                    Karaka::Karana => RegisterClass::CallerSaved,
                    Karaka::Sampradana => RegisterClass::Output,
                    Karaka::Apadana => RegisterClass::CalleeSaved,
                    Karaka::Adhikarana => RegisterClass::General,
                };

                func.karaka_hints.insert(
                    param.index,
                    KarakaHint {
                        karaka,
                        register_class,
                    },
                );
            }
        }
    }

    /// Pass: Memory layout optimization (Kosha-based)
    /// Apply Pancha Kosha memory tier hints for optimal placement
    fn pass_memory_layout(&mut self, func: &mut MirFunction) {
        // Analyze locals and assign memory tiers based on Pancha Kosha
        //
        // Annamaya (physical) - Register tier: Frequently accessed scalars
        // Pranamaya (vital) - L1 cache tier: Hot loop variables
        // Manomaya (mental) - L2 cache tier: Working set data
        // Vijnanamaya (wisdom) - L3/RAM tier: Large data structures
        // Anandamaya (bliss) - Disk/network tier: Persistent storage

        for local in &func.locals {
            // Analyze usage patterns to determine tier
            let usage_count = self.count_local_uses(func, local.index);
            let _tier = self.determine_memory_tier(usage_count, &local.ty);

            // In future: Add hints to MIR for register allocator
            // For now, this analysis informs codegen decisions
        }
    }

    /// Count how many times a local is used
    fn count_local_uses(&self, func: &MirFunction, local_idx: usize) -> usize {
        let mut count = 0;

        for block in &func.blocks {
            for inst in &block.instructions {
                if self.instruction_uses_local(inst, local_idx) {
                    count += 1;
                }
            }
            if self.terminator_uses_local(&block.terminator, local_idx) {
                count += 1;
            }
        }

        count
    }

    /// Check if an instruction uses a specific local
    fn instruction_uses_local(&self, inst: &MirInstruction, local_idx: usize) -> bool {
        match inst {
            MirInstruction::Assign { dest, value } => {
                self.place_uses_local(dest, local_idx) || self.rvalue_uses_local(value, local_idx)
            }
            MirInstruction::Drop { place } => self.place_uses_local(place, local_idx),
            MirInstruction::Store { ptr, value } => {
                self.operand_uses_local(ptr, local_idx) || self.operand_uses_local(value, local_idx)
            }
            MirInstruction::Load { dest, ptr } => {
                self.place_uses_local(dest, local_idx) || self.operand_uses_local(ptr, local_idx)
            }
            MirInstruction::Assert { condition, .. } => {
                self.operand_uses_local(condition, local_idx)
            }
            MirInstruction::SetDiscriminant { place, .. } => {
                self.place_uses_local(place, local_idx)
            }
            MirInstruction::BoundsCheck { index, len, .. } => {
                self.operand_uses_local(index, local_idx) || self.operand_uses_local(len, local_idx)
            }
            MirInstruction::Nop => false,
        }
    }

    /// Check if a terminator uses a specific local
    fn terminator_uses_local(&self, term: &MirTerminator, local_idx: usize) -> bool {
        match term {
            MirTerminator::SwitchInt { discriminant, .. } => {
                self.operand_uses_local(discriminant, local_idx)
            }
            MirTerminator::Call {
                func,
                args,
                destination,
                ..
            } => {
                self.operand_uses_local(func, local_idx)
                    || args.iter().any(|a| self.operand_uses_local(a, local_idx))
                    || destination
                        .as_ref()
                        .map_or(false, |d| self.place_uses_local(d, local_idx))
            }
            _ => false,
        }
    }

    /// Check if a place uses a specific local
    fn place_uses_local(&self, place: &MirPlace, local_idx: usize) -> bool {
        place.local == local_idx
    }

    /// Check if an operand uses a specific local
    fn operand_uses_local(&self, op: &MirOperand, local_idx: usize) -> bool {
        match op {
            MirOperand::Copy(p) | MirOperand::Move(p) => self.place_uses_local(p, local_idx),
            MirOperand::Constant(_) => false,
        }
    }

    /// Check if an rvalue uses a specific local
    fn rvalue_uses_local(&self, rv: &MirRvalue, local_idx: usize) -> bool {
        match rv {
            MirRvalue::Use(op) => self.operand_uses_local(op, local_idx),
            MirRvalue::Ref { place, .. } | MirRvalue::AddressOf { place, .. } => {
                self.place_uses_local(place, local_idx)
            }
            MirRvalue::BinaryOp { left, right, .. } | MirRvalue::FloatOp { left, right, .. } => {
                self.operand_uses_local(left, local_idx)
                    || self.operand_uses_local(right, local_idx)
            }
            MirRvalue::UnaryOp { operand, .. } | MirRvalue::Cast { operand, .. } => {
                self.operand_uses_local(operand, local_idx)
            }
            MirRvalue::Aggregate { operands, .. } | MirRvalue::SimdOp { operands, .. } => operands
                .iter()
                .any(|o| self.operand_uses_local(o, local_idx)),
            MirRvalue::Discriminant(p) | MirRvalue::Len(p) => self.place_uses_local(p, local_idx),
            MirRvalue::Field { base, .. } => self.operand_uses_local(base, local_idx),
            MirRvalue::Index { base, index } => {
                self.operand_uses_local(base, local_idx)
                    || self.operand_uses_local(index, local_idx)
            }
        }
    }

    /// Determine memory tier based on usage and type (Pancha Kosha mapping)
    fn determine_memory_tier(&self, usage_count: usize, ty: &MirType) -> MemoryTier {
        // High usage scalars -> Annamaya (register)
        if usage_count > 10 && self.is_scalar_type(ty) {
            return MemoryTier::Annamaya;
        }

        // Medium-high usage -> Pranamaya (L1)
        if usage_count > 5 {
            return MemoryTier::Pranamaya;
        }

        // Medium usage -> Manomaya (L2)
        if usage_count > 2 {
            return MemoryTier::Manomaya;
        }

        // Low usage or large types -> Vijnanamaya (RAM)
        MemoryTier::Vijnanamaya
    }

    /// Check if a type is a scalar (fits in register)
    fn is_scalar_type(&self, ty: &MirType) -> bool {
        matches!(
            ty,
            MirType::Int(_) | MirType::Float(_) | MirType::Bool | MirType::Ptr(_)
        )
    }
}

/// Memory tier based on Pancha Kosha
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryTier {
    /// Annamaya - Physical/Register tier (fastest, smallest)
    Annamaya,
    /// Pranamaya - Vital/L1 cache tier
    Pranamaya,
    /// Manomaya - Mental/L2 cache tier
    Manomaya,
    /// Vijnanamaya - Wisdom/L3-RAM tier
    Vijnanamaya,
    /// Anandamaya - Bliss/Persistent storage tier (slowest, largest)
    Anandamaya,
}

impl Default for MirOptimizer {
    fn default() -> Self {
        Self::new(OptLevel::Standard, GunaMode::Rajas)
    }
}
