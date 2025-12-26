//! RISC-V 64 Code Generation
//!
//! Generates RISC-V 64-bit assembly with kāraka-guided register allocation.

use super::{AsmEmitter, Instruction, Operand, Register, RegisterKind};
use crate::mir::types::{MirFunction, MirInstruction, MirTerminator, RegisterClass};

/// RISC-V 64 assembly emitter
pub struct RiscV64Emitter {
    /// Generated instructions
    instructions: Vec<String>,
    /// Current stack offset
    stack_offset: i64,
}

/// RISC-V registers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiscVReg {
    // Zero register
    Zero,  // x0 - hardwired zero

    // Return address
    Ra,    // x1

    // Stack pointer
    Sp,    // x2

    // Global pointer
    Gp,    // x3

    // Thread pointer
    Tp,    // x4

    // Temporaries (caller-saved)
    T0, T1, T2,    // x5-x7

    // Saved registers (callee-saved)
    S0, S1,        // x8-x9 (s0 = fp)

    // Arguments/return values
    A0, A1, A2, A3, A4, A5, A6, A7,  // x10-x17

    // Saved registers (callee-saved)
    S2, S3, S4, S5, S6, S7, S8, S9, S10, S11,  // x18-x27

    // Temporaries (caller-saved)
    T3, T4, T5, T6,  // x28-x31
}

impl RiscVReg {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Zero => "zero", Self::Ra => "ra", Self::Sp => "sp",
            Self::Gp => "gp", Self::Tp => "tp",
            Self::T0 => "t0", Self::T1 => "t1", Self::T2 => "t2",
            Self::S0 => "s0", Self::S1 => "s1",
            Self::A0 => "a0", Self::A1 => "a1", Self::A2 => "a2", Self::A3 => "a3",
            Self::A4 => "a4", Self::A5 => "a5", Self::A6 => "a6", Self::A7 => "a7",
            Self::S2 => "s2", Self::S3 => "s3", Self::S4 => "s4", Self::S5 => "s5",
            Self::S6 => "s6", Self::S7 => "s7", Self::S8 => "s8", Self::S9 => "s9",
            Self::S10 => "s10", Self::S11 => "s11",
            Self::T3 => "t3", Self::T4 => "t4", Self::T5 => "t5", Self::T6 => "t6",
        }
    }

    /// Get argument register by index
    pub fn arg_register(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::A0),
            1 => Some(Self::A1),
            2 => Some(Self::A2),
            3 => Some(Self::A3),
            4 => Some(Self::A4),
            5 => Some(Self::A5),
            6 => Some(Self::A6),
            7 => Some(Self::A7),
            _ => None,
        }
    }

    /// Is this a callee-saved register?
    pub fn is_callee_saved(&self) -> bool {
        matches!(
            self,
            Self::S0 | Self::S1 | Self::S2 | Self::S3 | Self::S4 | Self::S5 |
            Self::S6 | Self::S7 | Self::S8 | Self::S9 | Self::S10 | Self::S11
        )
    }
}

impl RiscV64Emitter {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            stack_offset: 0,
        }
    }

    fn emit(&mut self, instr: &str) {
        self.instructions.push(format!("    {}", instr));
    }

    fn emit_label(&mut self, label: &str) {
        self.instructions.push(format!("{}:", label));
    }

    fn emit_comment(&mut self, comment: &str) {
        self.instructions.push(format!("    # {}", comment));
    }
}

impl AsmEmitter for RiscV64Emitter {
    fn emit_prologue(&mut self, func: &MirFunction) {
        self.emit_label(&func.name);

        // Calculate stack frame size
        let frame_size = 16 + func.locals.len() * 8; // ra + s0 + locals
        let aligned_size = (frame_size + 15) & !15;

        // Allocate stack frame
        self.emit(&format!("addi sp, sp, -{}", aligned_size));

        // Save return address and frame pointer
        self.emit(&format!("sd ra, {}(sp)", aligned_size - 8));
        self.emit(&format!("sd s0, {}(sp)", aligned_size - 16));

        // Set up frame pointer
        self.emit(&format!("addi s0, sp, {}", aligned_size));

        // Handle kāraka hints
        for (param_idx, hint) in &func.karaka_hints {
            if hint.register_class == RegisterClass::CalleeSaved {
                self.emit_comment(&format!("kartṛ - preserve across calls"));
            }
        }
    }

    fn emit_body(&mut self, func: &MirFunction) {
        for block in &func.blocks {
            self.emit_label(&format!(".L{}", block.id));

            for instr in &block.instructions {
                self.emit_mir_instruction(instr);
            }

            self.emit_terminator(&block.terminator);
        }
    }

    fn emit_epilogue(&mut self, func: &MirFunction) {
        let frame_size = 16 + func.locals.len() * 8;
        let aligned_size = (frame_size + 15) & !15;

        // Restore return address and frame pointer
        self.emit(&format!("ld ra, {}(sp)", aligned_size - 8));
        self.emit(&format!("ld s0, {}(sp)", aligned_size - 16));

        // Deallocate stack frame
        self.emit(&format!("addi sp, sp, {}", aligned_size));

        self.emit("ret");
    }

    fn get_asm(&self) -> String {
        self.instructions.join("\n")
    }

    fn get_machine_code(&self) -> Vec<u8> {
        // TODO: Assemble to machine code
        Vec::new()
    }
}

impl RiscV64Emitter {
    fn emit_mir_instruction(&mut self, instr: &MirInstruction) {
        match instr {
            MirInstruction::Nop => {
                self.emit("nop");
            }
            _ => {
                // TODO: Implement other instructions
            }
        }
    }

    fn emit_terminator(&mut self, term: &MirTerminator) {
        match term {
            MirTerminator::Return => {
                // Return value should already be in a0
            }
            MirTerminator::Goto { target } => {
                self.emit(&format!("j .L{}", target));
            }
            MirTerminator::Unreachable => {
                self.emit("ebreak");
            }
            _ => {
                // TODO: Implement other terminators
            }
        }
    }
}

impl Default for RiscV64Emitter {
    fn default() -> Self {
        Self::new()
    }
}
