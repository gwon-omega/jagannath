//! AArch64 (ARM64) Code Generation
//!
//! Generates AArch64 assembly with kāraka-guided register allocation.
//!
//! ## Kāraka → Register Mapping
//! - **Kartṛ** (Agent) → Callee-saved (X19-X28) - long-lived
//! - **Karma** (Object) → X0 (return value)
//! - **Karaṇa** (Instrument) → Scratch registers (X9-X15)
//! - **Sampradāna** (Recipient) → X0 (first arg)
//! - **Apādāna** (Source) → X1 (second arg)
//!
//! ## ABI
//! Uses AAPCS64 (ARM 64-bit Procedure Call Standard):
//! - Integer args: X0-X7
//! - Float args: V0-V7 (SIMD/FP registers)
//! - Return: X0 (int), V0 (float)
//! - Callee-saved: X19-X28, X29 (FP), X30 (LR)

use super::AsmEmitter;
use crate::mir::types::{
    BinaryOp, FloatBinaryOp, FloatCmp, MirConstant, MirFunction, MirInstruction, MirOperand,
    MirPlace, MirRvalue, MirTerminator, RegisterClass, SimdOp, SimdWidth, UnaryOp,
};
use std::collections::HashMap;

/// AArch64 assembly emitter
pub struct AArch64Emitter {
    /// Generated instructions
    instructions: Vec<String>,
    /// Current stack offset
    stack_offset: i64,
    /// Register allocator
    reg_alloc: AArch64RegAlloc,
    /// Current function name
    current_func: String,
    /// Label counter
    label_counter: usize,
}

/// AArch64 registers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AArch64Reg {
    // General purpose registers
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7, // Arguments/results
    X8, // Indirect result
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15, // Caller-saved (temp)
    X16,
    X17, // IP0, IP1 (intra-procedure call)
    X18, // Platform register
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28, // Callee-saved
    X29, // Frame pointer (FP)
    X30, // Link register (LR)
    SP,  // Stack pointer
    XZR, // Zero register
}

/// SIMD/FP registers (NEON)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VReg {
    V0,
    V1,
    V2,
    V3,
    V4,
    V5,
    V6,
    V7, // Arguments/return (caller-saved)
    V8,
    V9,
    V10,
    V11,
    V12,
    V13,
    V14,
    V15, // Callee-saved (lower 64 bits)
    V16,
    V17,
    V18,
    V19,
    V20,
    V21,
    V22,
    V23, // Caller-saved
    V24,
    V25,
    V26,
    V27,
    V28,
    V29,
    V30,
    V31, // Caller-saved
}

impl VReg {
    pub fn name(&self) -> &'static str {
        match self {
            Self::V0 => "v0",
            Self::V1 => "v1",
            Self::V2 => "v2",
            Self::V3 => "v3",
            Self::V4 => "v4",
            Self::V5 => "v5",
            Self::V6 => "v6",
            Self::V7 => "v7",
            Self::V8 => "v8",
            Self::V9 => "v9",
            Self::V10 => "v10",
            Self::V11 => "v11",
            Self::V12 => "v12",
            Self::V13 => "v13",
            Self::V14 => "v14",
            Self::V15 => "v15",
            Self::V16 => "v16",
            Self::V17 => "v17",
            Self::V18 => "v18",
            Self::V19 => "v19",
            Self::V20 => "v20",
            Self::V21 => "v21",
            Self::V22 => "v22",
            Self::V23 => "v23",
            Self::V24 => "v24",
            Self::V25 => "v25",
            Self::V26 => "v26",
            Self::V27 => "v27",
            Self::V28 => "v28",
            Self::V29 => "v29",
            Self::V30 => "v30",
            Self::V31 => "v31",
        }
    }

    /// Get double-precision name (d0-d31)
    pub fn name_d(&self) -> &'static str {
        match self {
            Self::V0 => "d0",
            Self::V1 => "d1",
            Self::V2 => "d2",
            Self::V3 => "d3",
            Self::V4 => "d4",
            Self::V5 => "d5",
            Self::V6 => "d6",
            Self::V7 => "d7",
            Self::V8 => "d8",
            Self::V9 => "d9",
            Self::V10 => "d10",
            Self::V11 => "d11",
            Self::V12 => "d12",
            Self::V13 => "d13",
            Self::V14 => "d14",
            Self::V15 => "d15",
            Self::V16 => "d16",
            Self::V17 => "d17",
            Self::V18 => "d18",
            Self::V19 => "d19",
            Self::V20 => "d20",
            Self::V21 => "d21",
            Self::V22 => "d22",
            Self::V23 => "d23",
            Self::V24 => "d24",
            Self::V25 => "d25",
            Self::V26 => "d26",
            Self::V27 => "d27",
            Self::V28 => "d28",
            Self::V29 => "d29",
            Self::V30 => "d30",
            Self::V31 => "d31",
        }
    }

    /// Get single-precision name (s0-s31)
    pub fn name_s(&self) -> &'static str {
        match self {
            Self::V0 => "s0",
            Self::V1 => "s1",
            Self::V2 => "s2",
            Self::V3 => "s3",
            Self::V4 => "s4",
            Self::V5 => "s5",
            Self::V6 => "s6",
            Self::V7 => "s7",
            Self::V8 => "s8",
            Self::V9 => "s9",
            Self::V10 => "s10",
            Self::V11 => "s11",
            Self::V12 => "s12",
            Self::V13 => "s13",
            Self::V14 => "s14",
            Self::V15 => "s15",
            Self::V16 => "s16",
            Self::V17 => "s17",
            Self::V18 => "s18",
            Self::V19 => "s19",
            Self::V20 => "s20",
            Self::V21 => "s21",
            Self::V22 => "s22",
            Self::V23 => "s23",
            Self::V24 => "s24",
            Self::V25 => "s25",
            Self::V26 => "s26",
            Self::V27 => "s27",
            Self::V28 => "s28",
            Self::V29 => "s29",
            Self::V30 => "s30",
            Self::V31 => "s31",
        }
    }
}

/// Register allocator for AArch64
struct AArch64RegAlloc {
    /// Caller-saved registers (X9-X15)
    caller_saved: Vec<AArch64Reg>,
    /// Callee-saved registers (X19-X28)
    callee_saved: Vec<AArch64Reg>,
    /// Used callee-saved registers
    used_callee_saved: Vec<AArch64Reg>,
    /// Local variable to stack offset mapping
    local_offsets: HashMap<usize, i64>,
    /// Available V registers for floats
    v_available: Vec<VReg>,
}

impl AArch64RegAlloc {
    fn new() -> Self {
        Self {
            caller_saved: vec![
                AArch64Reg::X9,
                AArch64Reg::X10,
                AArch64Reg::X11,
                AArch64Reg::X12,
                AArch64Reg::X13,
                AArch64Reg::X14,
                AArch64Reg::X15,
            ],
            callee_saved: vec![
                AArch64Reg::X19,
                AArch64Reg::X20,
                AArch64Reg::X21,
                AArch64Reg::X22,
                AArch64Reg::X23,
                AArch64Reg::X24,
                AArch64Reg::X25,
                AArch64Reg::X26,
                AArch64Reg::X27,
                AArch64Reg::X28,
            ],
            used_callee_saved: Vec::new(),
            local_offsets: HashMap::new(),
            v_available: vec![
                VReg::V0,
                VReg::V1,
                VReg::V2,
                VReg::V3,
                VReg::V4,
                VReg::V5,
                VReg::V6,
                VReg::V7,
                VReg::V16,
                VReg::V17,
                VReg::V18,
                VReg::V19,
                VReg::V20,
                VReg::V21,
                VReg::V22,
                VReg::V23,
            ],
        }
    }

    fn allocate(&mut self, class: RegisterClass) -> Option<AArch64Reg> {
        match class {
            RegisterClass::CalleeSaved => {
                let reg = self.callee_saved.pop()?;
                self.used_callee_saved.push(reg);
                Some(reg)
            }
            RegisterClass::CallerSaved => self.caller_saved.pop(),
            RegisterClass::Output => Some(AArch64Reg::X0),
            RegisterClass::General => self.caller_saved.pop().or_else(|| {
                let reg = self.callee_saved.pop()?;
                self.used_callee_saved.push(reg);
                Some(reg)
            }),
        }
    }

    fn get_local_offset(&self, local: usize) -> Option<i64> {
        self.local_offsets.get(&local).copied()
    }

    fn set_local_offset(&mut self, local: usize, offset: i64) {
        self.local_offsets.insert(local, offset);
    }
}

impl AArch64Reg {
    pub fn name(&self) -> &'static str {
        match self {
            Self::X0 => "x0",
            Self::X1 => "x1",
            Self::X2 => "x2",
            Self::X3 => "x3",
            Self::X4 => "x4",
            Self::X5 => "x5",
            Self::X6 => "x6",
            Self::X7 => "x7",
            Self::X8 => "x8",
            Self::X9 => "x9",
            Self::X10 => "x10",
            Self::X11 => "x11",
            Self::X12 => "x12",
            Self::X13 => "x13",
            Self::X14 => "x14",
            Self::X15 => "x15",
            Self::X16 => "x16",
            Self::X17 => "x17",
            Self::X18 => "x18",
            Self::X19 => "x19",
            Self::X20 => "x20",
            Self::X21 => "x21",
            Self::X22 => "x22",
            Self::X23 => "x23",
            Self::X24 => "x24",
            Self::X25 => "x25",
            Self::X26 => "x26",
            Self::X27 => "x27",
            Self::X28 => "x28",
            Self::X29 => "x29",
            Self::X30 => "x30",
            Self::SP => "sp",
            Self::XZR => "xzr",
        }
    }

    /// Get 32-bit register name (w0-w30)
    pub fn name32(&self) -> &'static str {
        match self {
            Self::X0 => "w0",
            Self::X1 => "w1",
            Self::X2 => "w2",
            Self::X3 => "w3",
            Self::X4 => "w4",
            Self::X5 => "w5",
            Self::X6 => "w6",
            Self::X7 => "w7",
            Self::X8 => "w8",
            Self::X9 => "w9",
            Self::X10 => "w10",
            Self::X11 => "w11",
            Self::X12 => "w12",
            Self::X13 => "w13",
            Self::X14 => "w14",
            Self::X15 => "w15",
            Self::X16 => "w16",
            Self::X17 => "w17",
            Self::X18 => "w18",
            Self::X19 => "w19",
            Self::X20 => "w20",
            Self::X21 => "w21",
            Self::X22 => "w22",
            Self::X23 => "w23",
            Self::X24 => "w24",
            Self::X25 => "w25",
            Self::X26 => "w26",
            Self::X27 => "w27",
            Self::X28 => "w28",
            Self::X29 => "w29",
            Self::X30 => "w30",
            Self::SP => "wsp",
            Self::XZR => "wzr",
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
            Self::X19
                | Self::X20
                | Self::X21
                | Self::X22
                | Self::X23
                | Self::X24
                | Self::X25
                | Self::X26
                | Self::X27
                | Self::X28
        )
    }
}

impl AArch64Emitter {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            stack_offset: 0,
            reg_alloc: AArch64RegAlloc::new(),
            current_func: String::new(),
            label_counter: 0,
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

    fn emit_directive(&mut self, directive: &str) {
        self.instructions.push(directive.to_string());
    }

    fn new_label(&mut self, prefix: &str) -> String {
        let label = format!(".L{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    /// Emit constant to string
    fn const_to_str(&self, constant: &MirConstant) -> String {
        match constant {
            MirConstant::Int(val, _) => format!("#{}", val),
            MirConstant::Float(val, _) => format!("{}", val),
            MirConstant::Bool(b) => if *b { "#1" } else { "#0" }.to_string(),
            MirConstant::Unit => "#0".to_string(),
            MirConstant::String(_) => "=.LC_str".to_string(),
        }
    }

    /// Emit place to memory reference
    fn place_to_str(&self, place: &MirPlace) -> String {
        if let Some(offset) = self.reg_alloc.get_local_offset(place.local) {
            if offset >= 0 {
                format!("[x29, #{}]", offset)
            } else {
                format!("[x29, #{}]", offset)
            }
        } else {
            format!("[x29, #-{}]", (place.local + 1) * 8)
        }
    }

    /// Load operand into register
    fn load_operand(&mut self, operand: &MirOperand, reg: AArch64Reg) {
        match operand {
            MirOperand::Constant(c) => {
                match c {
                    MirConstant::Int(val, _) => {
                        let v = *val;
                        if v >= 0 && v <= 65535 {
                            self.emit(&format!("mov {}, #{}", reg.name(), v));
                        } else {
                            // Use movz/movk for larger immediates
                            self.emit(&format!("mov {}, #{}", reg.name(), v as u64 & 0xFFFF));
                            if v > 0xFFFF {
                                self.emit(&format!(
                                    "movk {}, #{}, lsl #16",
                                    reg.name(),
                                    (v as u64 >> 16) & 0xFFFF
                                ));
                            }
                            if v as u64 > 0xFFFF_FFFF {
                                self.emit(&format!(
                                    "movk {}, #{}, lsl #32",
                                    reg.name(),
                                    (v as u64 >> 32) & 0xFFFF
                                ));
                                self.emit(&format!(
                                    "movk {}, #{}, lsl #48",
                                    reg.name(),
                                    (v as u64 >> 48) & 0xFFFF
                                ));
                            }
                        }
                    }
                    MirConstant::Bool(b) => {
                        self.emit(&format!("mov {}, #{}", reg.name(), if *b { 1 } else { 0 }));
                    }
                    MirConstant::Unit => {
                        self.emit(&format!("mov {}, #0", reg.name()));
                    }
                    _ => {
                        self.emit(&format!("mov {}, #0", reg.name()));
                    }
                }
            }
            MirOperand::Copy(place) | MirOperand::Move(place) => {
                let src = self.place_to_str(place);
                self.emit(&format!("ldr {}, {}", reg.name(), src));
            }
        }
    }

    /// Store register to place
    fn store_to_place(&mut self, reg: AArch64Reg, place: &MirPlace) {
        let dest = self.place_to_str(place);
        self.emit(&format!("str {}, {}", reg.name(), dest));
    }

    /// Emit binary operation
    fn emit_binary_op(
        &mut self,
        op: BinaryOp,
        dest: AArch64Reg,
        left: AArch64Reg,
        right: AArch64Reg,
    ) {
        match op {
            BinaryOp::Add => {
                self.emit(&format!(
                    "add {}, {}, {}",
                    dest.name(),
                    left.name(),
                    right.name()
                ));
            }
            BinaryOp::Sub => {
                self.emit(&format!(
                    "sub {}, {}, {}",
                    dest.name(),
                    left.name(),
                    right.name()
                ));
            }
            BinaryOp::Mul => {
                self.emit(&format!(
                    "mul {}, {}, {}",
                    dest.name(),
                    left.name(),
                    right.name()
                ));
            }
            BinaryOp::Div => {
                self.emit(&format!(
                    "sdiv {}, {}, {}",
                    dest.name(),
                    left.name(),
                    right.name()
                ));
            }
            BinaryOp::Rem => {
                // ARM64: remainder = dividend - (quotient * divisor)
                self.emit(&format!("sdiv x16, {}, {}", left.name(), right.name()));
                self.emit(&format!(
                    "msub {}, x16, {}, {}",
                    dest.name(),
                    right.name(),
                    left.name()
                ));
            }
            BinaryOp::BitAnd => {
                self.emit(&format!(
                    "and {}, {}, {}",
                    dest.name(),
                    left.name(),
                    right.name()
                ));
            }
            BinaryOp::BitOr => {
                self.emit(&format!(
                    "orr {}, {}, {}",
                    dest.name(),
                    left.name(),
                    right.name()
                ));
            }
            BinaryOp::BitXor => {
                self.emit(&format!(
                    "eor {}, {}, {}",
                    dest.name(),
                    left.name(),
                    right.name()
                ));
            }
            BinaryOp::Shl => {
                self.emit(&format!(
                    "lsl {}, {}, {}",
                    dest.name(),
                    left.name(),
                    right.name()
                ));
            }
            BinaryOp::Shr => {
                self.emit(&format!(
                    "asr {}, {}, {}",
                    dest.name(),
                    left.name(),
                    right.name()
                ));
            }
            BinaryOp::Eq
            | BinaryOp::Ne
            | BinaryOp::Lt
            | BinaryOp::Le
            | BinaryOp::Gt
            | BinaryOp::Ge => {
                self.emit(&format!("cmp {}, {}", left.name(), right.name()));
                let cond = match op {
                    BinaryOp::Eq => "eq",
                    BinaryOp::Ne => "ne",
                    BinaryOp::Lt => "lt",
                    BinaryOp::Le => "le",
                    BinaryOp::Gt => "gt",
                    BinaryOp::Ge => "ge",
                    _ => unreachable!(),
                };
                self.emit(&format!("cset {}, {}", dest.name(), cond));
            }
        }
    }

    /// Load float operand into V register
    fn load_float_operand(&mut self, operand: &MirOperand, reg: VReg, is_double: bool) {
        let reg_name = if is_double {
            reg.name_d()
        } else {
            reg.name_s()
        };
        match operand {
            MirOperand::Constant(MirConstant::Float(val, _)) => {
                // Load float constant via integer register
                let bits = if is_double {
                    val.to_bits()
                } else {
                    (*val as f32).to_bits() as u64
                };
                self.emit(&format!("mov x16, #{}", bits));
                self.emit(&format!("fmov {}, x16", reg_name));
            }
            MirOperand::Copy(place) | MirOperand::Move(place) => {
                let src = self.place_to_str(place);
                self.emit(&format!("ldr {}, {}", reg_name, src));
            }
            _ => {}
        }
    }

    /// Emit float binary operation (NEON)
    fn emit_float_binary_op(
        &mut self,
        op: FloatBinaryOp,
        dest: VReg,
        left: VReg,
        right: VReg,
        is_double: bool,
    ) {
        let d = if is_double {
            dest.name_d()
        } else {
            dest.name_s()
        };
        let l = if is_double {
            left.name_d()
        } else {
            left.name_s()
        };
        let r = if is_double {
            right.name_d()
        } else {
            right.name_s()
        };

        match op {
            FloatBinaryOp::Add => {
                self.emit(&format!("fadd {}, {}, {}", d, l, r));
            }
            FloatBinaryOp::Sub => {
                self.emit(&format!("fsub {}, {}, {}", d, l, r));
            }
            FloatBinaryOp::Mul => {
                self.emit(&format!("fmul {}, {}, {}", d, l, r));
            }
            FloatBinaryOp::Div => {
                self.emit(&format!("fdiv {}, {}, {}", d, l, r));
            }
            FloatBinaryOp::Min => {
                self.emit(&format!("fmin {}, {}, {}", d, l, r));
            }
            FloatBinaryOp::Max => {
                self.emit(&format!("fmax {}, {}, {}", d, l, r));
            }
            FloatBinaryOp::Cmp(cmp) => {
                self.emit(&format!("fcmp {}, {}", l, r));
                let cond = match cmp {
                    FloatCmp::Eq => "eq",
                    FloatCmp::Ne => "ne",
                    FloatCmp::Lt => "mi", // Minus (less than)
                    FloatCmp::Le => "ls", // Less than or same
                    FloatCmp::Gt => "gt",
                    FloatCmp::Ge => "ge",
                    FloatCmp::Ord => "vc",   // No overflow (ordered)
                    FloatCmp::Unord => "vs", // Overflow (unordered)
                };
                self.emit(&format!("cset x0, {}", cond));
            }
        }
    }

    /// Emit SIMD operation (Tantra yantra - NEON)
    fn emit_simd_op(&mut self, op: SimdOp, operands: &[MirOperand], _width: SimdWidth, dest: VReg) {
        self.emit_comment("Tantra SIMD operation (NEON)");

        match op {
            SimdOp::Add => {
                if operands.len() >= 2 {
                    self.load_float_operand(&operands[0], VReg::V0, false);
                    self.load_float_operand(&operands[1], VReg::V1, false);
                    self.emit(&format!("fadd {}.4s, v0.4s, v1.4s", dest.name()));
                }
            }
            SimdOp::Mul => {
                if operands.len() >= 2 {
                    self.load_float_operand(&operands[0], VReg::V0, false);
                    self.load_float_operand(&operands[1], VReg::V1, false);
                    self.emit(&format!("fmul {}.4s, v0.4s, v1.4s", dest.name()));
                }
            }
            SimdOp::Broadcast => {
                if !operands.is_empty() {
                    self.load_float_operand(&operands[0], dest, false);
                    self.emit(&format!("dup {}.4s, {}[0]", dest.name(), dest.name()));
                }
            }
            _ => {
                self.emit_comment(&format!("SIMD {:?} not yet implemented", op));
            }
        }
    }
}

impl AsmEmitter for AArch64Emitter {
    fn emit_prologue(&mut self, func: &MirFunction) {
        self.current_func = func.name.clone();

        // Global and type declarations
        self.emit_directive(&format!(".global {}", func.name));
        self.emit_directive(&format!(".type {}, %function", func.name));
        self.emit_label(&func.name);

        // Save frame pointer and link register
        self.emit("stp x29, x30, [sp, #-16]!");
        self.emit("mov x29, sp");

        // Save callee-saved registers based on kāraka hints
        let callee_saved_needed: Vec<_> = func
            .karaka_hints
            .iter()
            .filter(|(_, hint)| hint.register_class == RegisterClass::CalleeSaved)
            .collect();

        for (_param_idx, _hint) in &callee_saved_needed {
            if let Some(reg) = self.reg_alloc.allocate(RegisterClass::CalleeSaved) {
                self.emit_comment(&format!("Save {} (kartṛ - agent)", reg.name()));
                self.emit(&format!("str {}, [sp, #-8]!", reg.name()));
            }
        }

        // Allocate stack space for locals (aligned to 16 bytes)
        let locals_space = func.locals.len() * 8;
        let aligned_space = (locals_space + 15) & !15;
        if aligned_space > 0 {
            self.emit(&format!("sub sp, sp, #{}", aligned_space));
            self.stack_offset = aligned_space as i64;
        }

        // Assign stack offsets to locals
        for (i, local) in func.locals.iter().enumerate() {
            let offset = -(((i + 1) * 8) as i64);
            self.reg_alloc.set_local_offset(local.index, offset);
        }

        // Move arguments from registers to stack
        for (i, _param) in func.params.iter().enumerate() {
            if let Some(reg) = AArch64Reg::arg_register(i) {
                let offset = -(((func.locals.len() + i + 1) * 8) as i64);
                self.emit_comment(&format!("Store arg {} from {}", i, reg.name()));
                self.emit(&format!("str {}, [x29, #{}]", reg.name(), offset));
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
        // Emit epilogue label
        self.emit_label(&format!(".L{}_epilogue", self.current_func));

        // Restore stack
        if self.stack_offset > 0 {
            self.emit(&format!("add sp, sp, #{}", self.stack_offset));
        }

        // Restore callee-saved registers (reverse order)
        let callee_saved: Vec<_> = self
            .reg_alloc
            .used_callee_saved
            .iter()
            .rev()
            .copied()
            .collect();
        for reg in callee_saved {
            self.emit(&format!("ldr {}, [sp], #8", reg.name()));
        }

        // Restore frame pointer and link register
        self.emit("ldp x29, x30, [sp], #16");
        self.emit("ret");

        // Function size directive
        self.emit_directive(&format!(".size {}, .-{}", func.name, func.name));
    }

    fn get_asm(&self) -> String {
        let mut output = String::new();

        // Assembly header
        output.push_str("// Jagannath AArch64 Assembly\n");
        output.push_str("// Generated by jagc compiler\n");
        output.push_str(".text\n\n");

        output.push_str(&self.instructions.join("\n"));
        output
    }

    fn get_machine_code(&self) -> Vec<u8> {
        // Would use an assembler to convert to machine code
        Vec::new()
    }
}

impl AArch64Emitter {
    fn emit_mir_instruction(&mut self, instr: &MirInstruction) {
        match instr {
            MirInstruction::Assign { dest, value } => {
                self.emit_comment("Assignment");
                self.emit_rvalue(value, dest);
            }
            MirInstruction::Store { ptr, value } => {
                self.emit_comment("Store through pointer");
                self.load_operand(value, AArch64Reg::X0);
                self.load_operand(ptr, AArch64Reg::X1);
                self.emit("str x0, [x1]");
            }
            MirInstruction::Load { dest, ptr } => {
                self.emit_comment("Load through pointer");
                self.load_operand(ptr, AArch64Reg::X1);
                self.emit("ldr x0, [x1]");
                self.store_to_place(AArch64Reg::X0, dest);
            }
            MirInstruction::SetDiscriminant { place, variant } => {
                self.emit_comment(&format!("Set discriminant to {}", variant));
                self.emit(&format!("mov x0, #{}", variant));
                self.store_to_place(AArch64Reg::X0, place);
            }
            MirInstruction::BoundsCheck {
                index,
                len,
                message,
            } => {
                self.emit_comment(&format!(
                    "BoundsCheck - Asipatravana prevention: {}",
                    message
                ));
                self.load_operand(index, AArch64Reg::X0);
                self.load_operand(len, AArch64Reg::X1);
                self.emit("cmp x0, x1");
                let pass_label = self.new_label("bounds_ok");
                self.emit(&format!("b.lo {}", pass_label)); // Branch if lower (unsigned)
                self.emit_comment("Bounds check failed - Asipatravana!");
                self.emit("brk #1"); // Breakpoint trap
                self.emit_label(&pass_label);
            }
            MirInstruction::Drop { place } => {
                self.emit_comment(&format!("Drop local {}", place.local));
            }
            MirInstruction::Nop => {
                self.emit("nop");
            }
            MirInstruction::Assert { condition, message } => {
                self.emit_comment(&format!("Assert: {}", message));
                self.load_operand(condition, AArch64Reg::X0);
                self.emit("cbnz x0, .+8"); // Skip trap if non-zero
                self.emit("brk #1");
            }
        }
    }

    fn emit_rvalue(&mut self, rvalue: &MirRvalue, dest: &MirPlace) {
        match rvalue {
            MirRvalue::Use(operand) => {
                self.load_operand(operand, AArch64Reg::X0);
                self.store_to_place(AArch64Reg::X0, dest);
            }
            MirRvalue::BinaryOp { op, left, right } => {
                self.load_operand(left, AArch64Reg::X1);
                self.load_operand(right, AArch64Reg::X2);
                self.emit_binary_op(*op, AArch64Reg::X0, AArch64Reg::X1, AArch64Reg::X2);
                self.store_to_place(AArch64Reg::X0, dest);
            }
            MirRvalue::UnaryOp { op, operand } => {
                self.load_operand(operand, AArch64Reg::X0);
                match op {
                    UnaryOp::Not => self.emit("mvn x0, x0"),
                    UnaryOp::Neg => self.emit("neg x0, x0"),
                }
                self.store_to_place(AArch64Reg::X0, dest);
            }
            MirRvalue::Ref { mutable: _, place } => {
                let _src = self.place_to_str(place);
                // Calculate address
                if let Some(offset) = self.reg_alloc.get_local_offset(place.local) {
                    self.emit(&format!("add x0, x29, #{}", offset));
                } else {
                    self.emit(&format!("add x0, x29, #-{}", (place.local + 1) * 8));
                }
                self.store_to_place(AArch64Reg::X0, dest);
            }
            MirRvalue::AddressOf { mutable: _, place } => {
                self.emit_comment("AddressOf - raw pointer creation");
                if let Some(offset) = self.reg_alloc.get_local_offset(place.local) {
                    self.emit(&format!("add x0, x29, #{}", offset));
                } else {
                    self.emit(&format!("add x0, x29, #-{}", (place.local + 1) * 8));
                }
                self.store_to_place(AArch64Reg::X0, dest);
            }
            MirRvalue::Field { base, index } => {
                self.emit_comment(&format!("Field access at index {}", index));
                match base {
                    MirOperand::Copy(place) | MirOperand::Move(place) => {
                        let base_offset = self.reg_alloc.get_local_offset(place.local).unwrap_or(0);
                        let field_offset = (index * 8) as i64;
                        let total_offset = base_offset + field_offset;
                        self.emit(&format!("ldr x0, [x29, #{}]", total_offset));
                        self.store_to_place(AArch64Reg::X0, dest);
                    }
                    _ => {}
                }
            }
            MirRvalue::Index { base, index } => {
                self.emit_comment("Array index access");
                let elem_size = 8;
                match base {
                    MirOperand::Copy(place) | MirOperand::Move(place) => {
                        let _src = self.place_to_str(place);
                        self.emit(&format!(
                            "add x1, x29, #{}",
                            self.reg_alloc.get_local_offset(place.local).unwrap_or(0)
                        ));
                    }
                    _ => {
                        self.load_operand(base, AArch64Reg::X1);
                    }
                }
                self.load_operand(index, AArch64Reg::X2);
                self.emit(&format!("mov x3, #{}", elem_size));
                self.emit("mul x2, x2, x3");
                self.emit("add x1, x1, x2");
                self.emit("ldr x0, [x1]");
                self.store_to_place(AArch64Reg::X0, dest);
            }
            MirRvalue::FloatOp { op, left, right } => {
                self.emit_comment("FloatOp - NEON operation");
                self.load_float_operand(left, VReg::V0, true);
                self.load_float_operand(right, VReg::V1, true);
                self.emit_float_binary_op(*op, VReg::V0, VReg::V0, VReg::V1, true);
                // Store float result
                let dest_str = self.place_to_str(dest);
                self.emit(&format!("str d0, {}", dest_str));
            }
            MirRvalue::SimdOp {
                op,
                operands,
                width,
            } => {
                self.emit_comment("SimdOp - Tantra NEON operation");
                self.emit_simd_op(*op, operands, *width, VReg::V0);
                let dest_str = self.place_to_str(dest);
                self.emit(&format!("str q0, {}", dest_str));
            }
            MirRvalue::Cast {
                kind: _,
                operand,
                ty: _,
            } => {
                self.load_operand(operand, AArch64Reg::X0);
                self.store_to_place(AArch64Reg::X0, dest);
            }
            MirRvalue::Discriminant(place) => {
                let src = self.place_to_str(place);
                self.emit(&format!("ldr x0, {}", src));
                self.store_to_place(AArch64Reg::X0, dest);
            }
            MirRvalue::Len(place) => {
                let src = self.place_to_str(place);
                self.emit(&format!("ldr x0, {}", src));
                self.emit("ldr x0, [x0, #8]"); // Length at offset 8
                self.store_to_place(AArch64Reg::X0, dest);
            }
            MirRvalue::Aggregate { kind: _, operands } => {
                for (i, operand) in operands.iter().enumerate() {
                    self.load_operand(operand, AArch64Reg::X0);
                    let offset = -(((dest.local + 1) * 8 + i * 8) as i64);
                    self.emit(&format!("str x0, [x29, #{}]", offset));
                }
            }
        }
    }

    fn emit_terminator(&mut self, term: &MirTerminator) {
        match term {
            MirTerminator::Return => {
                self.emit(&format!("b .L{}_epilogue", self.current_func));
            }
            MirTerminator::Goto { target } => {
                self.emit(&format!("b .L{}", target));
            }
            MirTerminator::SwitchInt {
                discriminant,
                targets,
                otherwise,
            } => {
                self.emit_comment("Switch on discriminant");
                self.load_operand(discriminant, AArch64Reg::X0);
                for (value, target) in targets {
                    self.emit(&format!("cmp x0, #{}", value));
                    self.emit(&format!("b.eq .L{}", target));
                }
                self.emit(&format!("b .L{}", otherwise));
            }
            MirTerminator::Call {
                func,
                args,
                destination,
                target,
            } => {
                self.emit_comment("Function call");
                // Pass arguments in registers
                for (i, arg) in args.iter().enumerate().take(8) {
                    if let Some(reg) = AArch64Reg::arg_register(i) {
                        self.load_operand(arg, reg);
                    }
                }
                // Branch and link
                match func {
                    MirOperand::Constant(MirConstant::String(name)) => {
                        self.emit(&format!("bl {}", name));
                    }
                    _ => {
                        self.load_operand(func, AArch64Reg::X9);
                        self.emit("blr x9");
                    }
                }
                // Store return value
                if let Some(dest) = destination {
                    self.store_to_place(AArch64Reg::X0, dest);
                }
                // Continue to target block
                self.emit(&format!("b .L{}", target));
            }
            MirTerminator::Unreachable => {
                self.emit_comment("Unreachable code - trap");
                self.emit("brk #1");
            }
            MirTerminator::Unwind => {
                self.emit_comment("Unwind/panic cleanup");
                self.emit(&format!("b .L{}_epilogue", self.current_func));
            }
        }
    }
}

impl Default for AArch64Emitter {
    fn default() -> Self {
        Self::new()
    }
}
