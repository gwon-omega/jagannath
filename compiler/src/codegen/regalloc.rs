//! Register Allocator
//!
//! Kāraka-guided register allocation.
//! Uses semantic roles to determine optimal register assignment.

use crate::mir::types::{MirFunction, RegisterClass, KarakaHint};
use crate::parser::ast::Karaka;

/// Register allocator
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

impl RegisterAllocator {
    pub fn new(target: Target) -> Self {
        Self { target }
    }

    /// Allocate registers for a function
    pub fn allocate(&mut self, func: &mut MirFunction) {
        // First pass: Apply kāraka hints
        self.apply_karaka_hints(func);

        // Second pass: Linear scan allocation for remaining
        self.linear_scan(func);
    }

    /// Apply kāraka-based register hints
    fn apply_karaka_hints(&mut self, func: &MirFunction) {
        for (param_idx, hint) in &func.karaka_hints {
            let class = self.karaka_to_class(&hint.karaka);
            // TODO: Reserve appropriate registers
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

    /// Linear scan register allocation
    fn linear_scan(&mut self, func: &mut MirFunction) {
        // TODO: Implement linear scan algorithm
    }

    /// Get number of general-purpose registers for target
    fn num_gp_registers(&self) -> usize {
        match self.target {
            Target::X86_64 => 14,   // Excluding RSP, RBP
            Target::AArch64 => 28,  // X0-X28 (excluding X29=FP, X30=LR, X31=SP)
            Target::RiscV64 => 28,  // x1-x31 (excluding x0=zero)
        }
    }

    /// Get number of callee-saved registers
    fn num_callee_saved(&self) -> usize {
        match self.target {
            Target::X86_64 => 5,    // RBX, R12-R15
            Target::AArch64 => 10,  // X19-X28
            Target::RiscV64 => 12,  // s0-s11
        }
    }
}
