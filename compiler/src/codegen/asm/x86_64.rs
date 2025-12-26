//! x86-64 Code Generation
//!
//! Generates x86-64 assembly with kāraka-guided register allocation.

use super::{AsmEmitter, Instruction, Operand, Register, RegisterKind, MemoryRef};
use crate::mir::types::{MirFunction, MirInstruction, MirTerminator, RegisterClass};

/// x86-64 assembly emitter
pub struct X86_64Emitter {
    /// Generated instructions
    instructions: Vec<String>,
    /// Current stack offset
    stack_offset: i64,
    /// Register allocation
    reg_alloc: X86RegAlloc,
}

/// x86-64 registers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum X86Reg {
    // Caller-saved (scratch)
    RAX = 0, RCX = 1, RDX = 2, RSI = 6, RDI = 7,
    R8 = 8, R9 = 9, R10 = 10, R11 = 11,

    // Callee-saved (preserved)
    RBX = 3, RBP = 5, R12 = 12, R13 = 13, R14 = 14, R15 = 15,

    // Stack pointer (special)
    RSP = 4,
}

impl X86Reg {
    pub fn name(&self) -> &'static str {
        match self {
            Self::RAX => "rax", Self::RBX => "rbx", Self::RCX => "rcx", Self::RDX => "rdx",
            Self::RSI => "rsi", Self::RDI => "rdi", Self::RSP => "rsp", Self::RBP => "rbp",
            Self::R8 => "r8", Self::R9 => "r9", Self::R10 => "r10", Self::R11 => "r11",
            Self::R12 => "r12", Self::R13 => "r13", Self::R14 => "r14", Self::R15 => "r15",
        }
    }

    /// Get argument register by index (System V AMD64 ABI)
    pub fn arg_register(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::RDI),
            1 => Some(Self::RSI),
            2 => Some(Self::RDX),
            3 => Some(Self::RCX),
            4 => Some(Self::R8),
            5 => Some(Self::R9),
            _ => None, // Stack argument
        }
    }

    /// Is this a callee-saved register?
    pub fn is_callee_saved(&self) -> bool {
        matches!(self, Self::RBX | Self::RBP | Self::R12 | Self::R13 | Self::R14 | Self::R15)
    }
}

/// Register allocator for x86-64
struct X86RegAlloc {
    /// Available caller-saved registers
    caller_saved: Vec<X86Reg>,
    /// Available callee-saved registers
    callee_saved: Vec<X86Reg>,
    /// Used callee-saved registers (need to save in prologue)
    used_callee_saved: Vec<X86Reg>,
}

impl X86RegAlloc {
    fn new() -> Self {
        Self {
            caller_saved: vec![X86Reg::RAX, X86Reg::RCX, X86Reg::RDX, X86Reg::RSI, X86Reg::RDI,
                              X86Reg::R8, X86Reg::R9, X86Reg::R10, X86Reg::R11],
            callee_saved: vec![X86Reg::RBX, X86Reg::R12, X86Reg::R13, X86Reg::R14, X86Reg::R15],
            used_callee_saved: Vec::new(),
        }
    }

    /// Allocate register based on kāraka hint
    fn allocate(&mut self, class: RegisterClass) -> Option<X86Reg> {
        match class {
            RegisterClass::CalleeSaved => {
                let reg = self.callee_saved.pop()?;
                self.used_callee_saved.push(reg);
                Some(reg)
            }
            RegisterClass::CallerSaved => self.caller_saved.pop(),
            RegisterClass::Output => Some(X86Reg::RAX),
            RegisterClass::General => {
                self.caller_saved.pop().or_else(|| {
                    let reg = self.callee_saved.pop()?;
                    self.used_callee_saved.push(reg);
                    Some(reg)
                })
            }
        }
    }
}

impl X86_64Emitter {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            stack_offset: 0,
            reg_alloc: X86RegAlloc::new(),
        }
    }

    fn emit(&mut self, instr: &str) {
        self.instructions.push(format!("    {}", instr));
    }

    fn emit_label(&mut self, label: &str) {
        self.instructions.push(format!("{}:", label));
    }

    fn emit_comment(&mut self, comment: &str) {
        self.instructions.push(format!("    ; {}", comment));
    }
}

impl AsmEmitter for X86_64Emitter {
    fn emit_prologue(&mut self, func: &MirFunction) {
        self.emit_label(&func.name);

        // Standard prologue
        self.emit("push rbp");
        self.emit("mov rbp, rsp");

        // Save callee-saved registers based on kāraka hints
        for (param_idx, hint) in &func.karaka_hints {
            if hint.register_class == RegisterClass::CalleeSaved {
                // Allocate callee-saved register for this parameter
                if let Some(reg) = self.reg_alloc.allocate(RegisterClass::CalleeSaved) {
                    self.emit_comment(&format!("Save {} (kartṛ)", reg.name()));
                    self.emit(&format!("push {}", reg.name()));
                }
            }
        }

        // Allocate stack space for locals
        let stack_space = func.locals.len() * 8;
        if stack_space > 0 {
            self.emit(&format!("sub rsp, {}", stack_space));
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
        // Restore callee-saved registers (reverse order)
        // Clone the list to avoid borrowing self.reg_alloc while calling emit()
        let callee_saved: Vec<_> = self.reg_alloc.used_callee_saved.iter().rev().copied().collect();
        for reg in callee_saved {
            self.emit(&format!("pop {}", reg.name()));
        }

        // Standard epilogue
        self.emit("mov rsp, rbp");
        self.emit("pop rbp");
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

impl X86_64Emitter {
    fn emit_mir_instruction(&mut self, instr: &MirInstruction) {
        match instr {
            MirInstruction::Assign { dest, value } => {
                // TODO: Emit assignment
            }
            MirInstruction::Drop { place } => {
                // TODO: Emit drop
            }
            MirInstruction::Nop => {
                self.emit("nop");
            }
            MirInstruction::Assert { condition, message } => {
                // TODO: Emit assertion
            }
        }
    }

    fn emit_terminator(&mut self, term: &MirTerminator) {
        match term {
            MirTerminator::Return => {
                // Return value should already be in RAX
            }
            MirTerminator::Goto { target } => {
                self.emit(&format!("jmp .L{}", target));
            }
            MirTerminator::SwitchInt { discriminant, targets, otherwise } => {
                // TODO: Emit switch
            }
            MirTerminator::Call { func, args, destination, target } => {
                // TODO: Emit call
            }
            MirTerminator::Unreachable => {
                self.emit("ud2");
            }
            MirTerminator::Unwind => {
                // TODO: Emit unwind handling
            }
        }
    }
}

impl Default for X86_64Emitter {
    fn default() -> Self {
        Self::new()
    }
}
