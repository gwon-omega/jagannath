//! AArch64 (ARM64) Code Generation
//!
//! Generates AArch64 assembly with kāraka-guided register allocation.

use super::{AsmEmitter, Instruction, Operand, Register, RegisterKind};
use crate::mir::types::{MirFunction, MirInstruction, MirTerminator, RegisterClass};

/// AArch64 assembly emitter
pub struct AArch64Emitter {
    /// Generated instructions
    instructions: Vec<String>,
    /// Current stack offset
    stack_offset: i64,
}

/// AArch64 registers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AArch64Reg {
    // General purpose registers
    X0, X1, X2, X3, X4, X5, X6, X7,   // Arguments/results
    X8,                                // Indirect result
    X9, X10, X11, X12, X13, X14, X15, // Caller-saved (temp)
    X16, X17,                          // IP0, IP1 (intra-procedure call)
    X18,                               // Platform register
    X19, X20, X21, X22, X23, X24, X25, X26, X27, X28, // Callee-saved
    X29,                               // Frame pointer (FP)
    X30,                               // Link register (LR)
    SP,                                // Stack pointer
    XZR,                               // Zero register
}

impl AArch64Reg {
    pub fn name(&self) -> &'static str {
        match self {
            Self::X0 => "x0", Self::X1 => "x1", Self::X2 => "x2", Self::X3 => "x3",
            Self::X4 => "x4", Self::X5 => "x5", Self::X6 => "x6", Self::X7 => "x7",
            Self::X8 => "x8", Self::X9 => "x9", Self::X10 => "x10", Self::X11 => "x11",
            Self::X12 => "x12", Self::X13 => "x13", Self::X14 => "x14", Self::X15 => "x15",
            Self::X16 => "x16", Self::X17 => "x17", Self::X18 => "x18", Self::X19 => "x19",
            Self::X20 => "x20", Self::X21 => "x21", Self::X22 => "x22", Self::X23 => "x23",
            Self::X24 => "x24", Self::X25 => "x25", Self::X26 => "x26", Self::X27 => "x27",
            Self::X28 => "x28", Self::X29 => "x29", Self::X30 => "x30",
            Self::SP => "sp", Self::XZR => "xzr",
        }
    }

    /// Get argument register by index (AAPCS64)
    pub fn arg_register(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::X0),
            1 => Some(Self::X1),
            2 => Some(Self::X2),
            3 => Some(Self::X3),
            4 => Some(Self::X4),
            5 => Some(Self::X5),
            6 => Some(Self::X6),
            7 => Some(Self::X7),
            _ => None, // Stack argument
        }
    }

    /// Is this a callee-saved register?
    pub fn is_callee_saved(&self) -> bool {
        matches!(
            self,
            Self::X19 | Self::X20 | Self::X21 | Self::X22 | Self::X23 |
            Self::X24 | Self::X25 | Self::X26 | Self::X27 | Self::X28
        )
    }
}

impl AArch64Emitter {
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
        self.instructions.push(format!("    // {}", comment));
    }
}

impl AsmEmitter for AArch64Emitter {
    fn emit_prologue(&mut self, func: &MirFunction) {
        self.emit_label(&func.name);

        // Save frame pointer and link register
        self.emit("stp x29, x30, [sp, #-16]!");
        self.emit("mov x29, sp");

        // Save callee-saved registers based on kāraka hints
        for (param_idx, hint) in &func.karaka_hints {
            if hint.register_class == RegisterClass::CalleeSaved {
                self.emit_comment(&format!("kartṛ parameter - preserve"));
            }
        }

        // Allocate stack space for locals
        let stack_space = (func.locals.len() * 8 + 15) & !15; // 16-byte aligned
        if stack_space > 0 {
            self.emit(&format!("sub sp, sp, #{}", stack_space));
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
        // Deallocate stack and restore
        self.emit("mov sp, x29");
        self.emit("ldp x29, x30, [sp], #16");
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

impl AArch64Emitter {
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
                // Return value should already be in X0
            }
            MirTerminator::Goto { target } => {
                self.emit(&format!("b .L{}", target));
            }
            MirTerminator::Unreachable => {
                self.emit("brk #1");
            }
            _ => {
                // TODO: Implement other terminators
            }
        }
    }
}

impl Default for AArch64Emitter {
    fn default() -> Self {
        Self::new()
    }
}
