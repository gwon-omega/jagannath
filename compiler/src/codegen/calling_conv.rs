//! Calling Conventions
//!
//! Defines calling conventions for different platforms.

/// Calling convention
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallingConvention {
    /// System V AMD64 ABI (Linux, macOS, BSD)
    SysV,
    /// Microsoft x64 (Windows)
    Win64,
    /// AAPCS64 (ARM64)
    AAPCS64,
    /// RISC-V standard calling convention
    RiscV,
    /// Jagannath custom (kāraka-guided)
    Jagannath,
}

/// Parameter location
#[derive(Debug, Clone)]
pub enum ParamLocation {
    /// Register
    Register(u8),
    /// Stack offset
    Stack(i64),
}

/// Calling convention info
pub struct CallingConvInfo {
    /// Argument registers
    pub arg_regs: Vec<u8>,
    /// Return registers
    pub ret_regs: Vec<u8>,
    /// Callee-saved registers
    pub callee_saved: Vec<u8>,
    /// Caller-saved registers
    pub caller_saved: Vec<u8>,
    /// Stack alignment
    pub stack_alignment: usize,
}

impl CallingConvention {
    /// Get calling convention info
    pub fn info(&self) -> CallingConvInfo {
        match self {
            Self::SysV => CallingConvInfo {
                arg_regs: vec![7, 6, 2, 1, 8, 9],  // RDI, RSI, RDX, RCX, R8, R9
                ret_regs: vec![0, 2],              // RAX, RDX
                callee_saved: vec![3, 5, 12, 13, 14, 15], // RBX, RBP, R12-R15
                caller_saved: vec![0, 1, 2, 6, 7, 8, 9, 10, 11], // RAX, RCX, RDX, RSI, RDI, R8-R11
                stack_alignment: 16,
            },
            Self::Win64 => CallingConvInfo {
                arg_regs: vec![1, 2, 8, 9],        // RCX, RDX, R8, R9
                ret_regs: vec![0],                 // RAX
                callee_saved: vec![3, 5, 6, 7, 12, 13, 14, 15], // RBX, RBP, RSI, RDI, R12-R15
                caller_saved: vec![0, 1, 2, 8, 9, 10, 11], // RAX, RCX, RDX, R8-R11
                stack_alignment: 16,
            },
            Self::AAPCS64 => CallingConvInfo {
                arg_regs: (0..8).collect(),        // X0-X7
                ret_regs: vec![0, 1],              // X0, X1
                callee_saved: (19..29).collect(),  // X19-X28
                caller_saved: (0..19).collect(),   // X0-X18
                stack_alignment: 16,
            },
            Self::RiscV => CallingConvInfo {
                arg_regs: (10..18).collect(),      // a0-a7
                ret_regs: vec![10, 11],            // a0, a1
                callee_saved: vec![8, 9, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27], // s0-s11
                caller_saved: vec![5, 6, 7, 28, 29, 30, 31], // t0-t6
                stack_alignment: 16,
            },
            Self::Jagannath => CallingConvInfo {
                // Kāraka-guided: allocate based on semantic roles
                arg_regs: vec![],   // Dynamic based on kāraka
                ret_regs: vec![0],  // Standard return
                callee_saved: vec![],
                caller_saved: vec![],
                stack_alignment: 16,
            },
        }
    }

    /// Determine parameter locations
    pub fn param_locations(&self, param_count: usize) -> Vec<ParamLocation> {
        let info = self.info();
        let mut locations = Vec::new();
        let mut stack_offset: i64 = 0;

        for i in 0..param_count {
            if i < info.arg_regs.len() {
                locations.push(ParamLocation::Register(info.arg_regs[i]));
            } else {
                locations.push(ParamLocation::Stack(stack_offset));
                stack_offset += 8;
            }
        }

        locations
    }
}
