//! Assembly Module
//!
//! Low-level assembly emission for target architectures.

pub mod x86_64;
pub mod aarch64;
pub mod riscv64;

use crate::mir::types::MirFunction;

/// Assembly emitter trait
pub trait AsmEmitter {
    /// Emit function prologue
    fn emit_prologue(&mut self, func: &MirFunction);

    /// Emit function body
    fn emit_body(&mut self, func: &MirFunction);

    /// Emit function epilogue
    fn emit_epilogue(&mut self, func: &MirFunction);

    /// Get generated assembly as string
    fn get_asm(&self) -> String;

    /// Get generated machine code as bytes
    fn get_machine_code(&self) -> Vec<u8>;
}

/// Generic instruction representation
#[derive(Debug, Clone)]
pub struct Instruction {
    pub opcode: String,
    pub operands: Vec<Operand>,
    pub comment: Option<String>,
}

/// Operand for instructions
#[derive(Debug, Clone)]
pub enum Operand {
    /// Register operand
    Register(Register),
    /// Immediate value
    Immediate(i64),
    /// Memory reference
    Memory(MemoryRef),
    /// Label reference
    Label(String),
}

/// Generic register representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Register {
    pub id: u8,
    pub kind: RegisterKind,
}

/// Register kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterKind {
    GeneralPurpose,
    FloatingPoint,
    Vector,
}

/// Memory reference
#[derive(Debug, Clone)]
pub struct MemoryRef {
    pub base: Option<Register>,
    pub index: Option<Register>,
    pub scale: u8,
    pub displacement: i64,
}

/// Target architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Target {
    X86_64,
    AArch64,
    RiscV64,
}

impl Target {
    /// Get pointer size in bytes
    pub fn pointer_size(&self) -> usize {
        8 // All supported targets are 64-bit
    }

    /// Get word size in bytes
    pub fn word_size(&self) -> usize {
        8
    }
}
