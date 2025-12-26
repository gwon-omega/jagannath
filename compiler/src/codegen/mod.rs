//! Codegen Module - Code Generation
//!
//! Converts MIR to machine code for target architectures:
//! - x86-64 (Intel/AMD)
//! - AArch64 (ARM64)
//! - RISC-V 64
//!
//! Uses kāraka hints for optimal register allocation.

pub mod asm;
pub mod calling_conv;
pub mod linker;
pub mod regalloc;

// Re-exports
pub use asm::{AsmEmitter, Instruction};
pub use linker::{Assembler, BuildError, BuildPipeline, LinkOutput, Linker};
pub use regalloc::RegisterAllocator;
