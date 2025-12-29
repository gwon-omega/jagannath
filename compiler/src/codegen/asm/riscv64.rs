//! RISC-V 64 Code Generation
//!
//! Generates RISC-V 64-bit assembly with kāraka-guided register allocation.
//!
//! ## Kāraka → Register Mapping
//! - **Kartṛ** (Agent) → Callee-saved (s0-s11) - long-lived
//! - **Karma** (Object) → a0 (return value)
//! - **Karaṇa** (Instrument) → Temporaries (t0-t6)
//! - **Sampradāna** (Recipient) → a0 (first arg)
//! - **Apādāna** (Source) → a1 (second arg)
//!
//! ## ABI
//! Uses RISC-V LP64 ABI:
//! - Integer args: a0-a7 (x10-x17)
//! - Float args: fa0-fa7 (f10-f17)
//! - Return: a0 (int), fa0 (float)
//! - Callee-saved: s0-s11 (x8-x9, x18-x27), ra (x1)

use super::AsmEmitter;
use crate::mir::types::{
    BinaryOp, FloatBinaryOp, FloatCmp, MirConstant, MirFunction, MirInstruction, MirOperand,
    MirPlace, MirRvalue, MirTerminator, RegisterClass, UnaryOp,
};
use std::collections::HashMap;

/// RISC-V 64 assembly emitter
pub struct RiscV64Emitter {
    /// Generated instructions
    instructions: Vec<String>,
    /// Current stack offset
    stack_offset: i64,
    /// Register allocator
    reg_alloc: RiscVRegAlloc,
    /// Current function name
    current_func: String,
    /// Label counter
    label_counter: usize,
}

/// RISC-V registers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiscVReg {
    // Zero register
    Zero, // x0 - hardwired zero

    // Return address
    Ra, // x1

    // Stack pointer
    Sp, // x2

    // Global pointer
    Gp, // x3

    // Thread pointer
    Tp, // x4

    // Temporaries (caller-saved)
    T0,
    T1,
    T2, // x5-x7

    // Saved registers (callee-saved)
    S0,
    S1, // x8-x9 (s0 = fp)

    // Arguments/return values
    A0,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7, // x10-x17

    // Saved registers (callee-saved)
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11, // x18-x27

    // Temporaries (caller-saved)
    T3,
    T4,
    T5,
    T6, // x28-x31
}

/// RISC-V floating-point registers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FReg {
    // Temporaries
    Ft0,
    Ft1,
    Ft2,
    Ft3,
    Ft4,
    Ft5,
    Ft6,
    Ft7, // f0-f7
    // Callee-saved
    Fs0,
    Fs1, // f8-f9
    // Arguments/return
    Fa0,
    Fa1,
    Fa2,
    Fa3,
    Fa4,
    Fa5,
    Fa6,
    Fa7, // f10-f17
    // Callee-saved
    Fs2,
    Fs3,
    Fs4,
    Fs5,
    Fs6,
    Fs7,
    Fs8,
    Fs9,
    Fs10,
    Fs11, // f18-f27
    // Temporaries
    Ft8,
    Ft9,
    Ft10,
    Ft11, // f28-f31
}

impl FReg {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Ft0 => "ft0",
            Self::Ft1 => "ft1",
            Self::Ft2 => "ft2",
            Self::Ft3 => "ft3",
            Self::Ft4 => "ft4",
            Self::Ft5 => "ft5",
            Self::Ft6 => "ft6",
            Self::Ft7 => "ft7",
            Self::Fs0 => "fs0",
            Self::Fs1 => "fs1",
            Self::Fa0 => "fa0",
            Self::Fa1 => "fa1",
            Self::Fa2 => "fa2",
            Self::Fa3 => "fa3",
            Self::Fa4 => "fa4",
            Self::Fa5 => "fa5",
            Self::Fa6 => "fa6",
            Self::Fa7 => "fa7",
            Self::Fs2 => "fs2",
            Self::Fs3 => "fs3",
            Self::Fs4 => "fs4",
            Self::Fs5 => "fs5",
            Self::Fs6 => "fs6",
            Self::Fs7 => "fs7",
            Self::Fs8 => "fs8",
            Self::Fs9 => "fs9",
            Self::Fs10 => "fs10",
            Self::Fs11 => "fs11",
            Self::Ft8 => "ft8",
            Self::Ft9 => "ft9",
            Self::Ft10 => "ft10",
            Self::Ft11 => "ft11",
        }
    }
}

/// Register allocator for RISC-V
struct RiscVRegAlloc {
    /// Caller-saved temporaries
    caller_saved: Vec<RiscVReg>,
    /// Callee-saved registers
    callee_saved: Vec<RiscVReg>,
    /// Used callee-saved registers
    used_callee_saved: Vec<RiscVReg>,
    /// Local variable to stack offset mapping
    local_offsets: HashMap<usize, i64>,
}

impl RiscVRegAlloc {
    fn new() -> Self {
        Self {
            caller_saved: vec![
                RiscVReg::T0,
                RiscVReg::T1,
                RiscVReg::T2,
                RiscVReg::T3,
                RiscVReg::T4,
                RiscVReg::T5,
                RiscVReg::T6,
            ],
            callee_saved: vec![
                RiscVReg::S2,
                RiscVReg::S3,
                RiscVReg::S4,
                RiscVReg::S5,
                RiscVReg::S6,
                RiscVReg::S7,
                RiscVReg::S8,
                RiscVReg::S9,
                RiscVReg::S10,
                RiscVReg::S11,
            ],
            used_callee_saved: Vec::new(),
            local_offsets: HashMap::new(),
        }
    }

    fn allocate(&mut self, class: RegisterClass) -> Option<RiscVReg> {
        match class {
            RegisterClass::CalleeSaved => {
                let reg = self.callee_saved.pop()?;
                self.used_callee_saved.push(reg);
                Some(reg)
            }
            RegisterClass::CallerSaved => self.caller_saved.pop(),
            RegisterClass::Output => Some(RiscVReg::A0),
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

impl RiscVReg {
    pub fn name(&self) -> &'static str {
        match self {
            Self::Zero => "zero",
            Self::Ra => "ra",
            Self::Sp => "sp",
            Self::Gp => "gp",
            Self::Tp => "tp",
            Self::T0 => "t0",
            Self::T1 => "t1",
            Self::T2 => "t2",
            Self::S0 => "s0",
            Self::S1 => "s1",
            Self::A0 => "a0",
            Self::A1 => "a1",
            Self::A2 => "a2",
            Self::A3 => "a3",
            Self::A4 => "a4",
            Self::A5 => "a5",
            Self::A6 => "a6",
            Self::A7 => "a7",
            Self::S2 => "s2",
            Self::S3 => "s3",
            Self::S4 => "s4",
            Self::S5 => "s5",
            Self::S6 => "s6",
            Self::S7 => "s7",
            Self::S8 => "s8",
            Self::S9 => "s9",
            Self::S10 => "s10",
            Self::S11 => "s11",
            Self::T3 => "t3",
            Self::T4 => "t4",
            Self::T5 => "t5",
            Self::T6 => "t6",
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
            Self::S0
                | Self::S1
                | Self::S2
                | Self::S3
                | Self::S4
                | Self::S5
                | Self::S6
                | Self::S7
                | Self::S8
                | Self::S9
                | Self::S10
                | Self::S11
        )
    }
}

impl RiscV64Emitter {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            stack_offset: 0,
            reg_alloc: RiscVRegAlloc::new(),
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
        self.instructions.push(format!("    # {}", comment));
    }

    fn emit_directive(&mut self, directive: &str) {
        self.instructions.push(directive.to_string());
    }

    fn new_label(&mut self, prefix: &str) -> String {
        let label = format!(".L{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    /// Emit place to memory reference
    fn place_to_str(&self, place: &MirPlace) -> String {
        if let Some(offset) = self.reg_alloc.get_local_offset(place.local) {
            format!("{}(s0)", offset)
        } else {
            format!("-{}(s0)", (place.local + 1) * 8)
        }
    }

    /// Load operand into register
    fn load_operand(&mut self, operand: &MirOperand, reg: RiscVReg) {
        match operand {
            MirOperand::Constant(c) => {
                match c {
                    MirConstant::Int(val, _) => {
                        let v = *val;
                        if v >= -2048 && v <= 2047 {
                            self.emit(&format!("li {}, {}", reg.name(), v));
                        } else {
                            // Use lui/addi for larger immediates
                            self.emit(&format!("lui {}, %hi({})", reg.name(), v));
                            self.emit(&format!("addi {}, {}, %lo({})", reg.name(), reg.name(), v));
                        }
                    }
                    MirConstant::Bool(b) => {
                        self.emit(&format!("li {}, {}", reg.name(), if *b { 1 } else { 0 }));
                    }
                    MirConstant::Unit => {
                        self.emit(&format!("li {}, 0", reg.name()));
                    }
                    _ => {
                        self.emit(&format!("li {}, 0", reg.name()));
                    }
                }
            }
            MirOperand::Copy(place) | MirOperand::Move(place) => {
                let src = self.place_to_str(place);
                self.emit(&format!("ld {}, {}", reg.name(), src));
            }
        }
    }

    /// Store register to place
    fn store_to_place(&mut self, reg: RiscVReg, place: &MirPlace) {
        let dest = self.place_to_str(place);
        self.emit(&format!("sd {}, {}", reg.name(), dest));
    }

    /// Emit binary operation
    fn emit_binary_op(&mut self, op: BinaryOp, dest: RiscVReg, left: RiscVReg, right: RiscVReg) {
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
                    "div {}, {}, {}",
                    dest.name(),
                    left.name(),
                    right.name()
                ));
            }
            BinaryOp::Rem => {
                self.emit(&format!(
                    "rem {}, {}, {}",
                    dest.name(),
                    left.name(),
                    right.name()
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
                    "or {}, {}, {}",
                    dest.name(),
                    left.name(),
                    right.name()
                ));
            }
            BinaryOp::BitXor => {
                self.emit(&format!(
                    "xor {}, {}, {}",
                    dest.name(),
                    left.name(),
                    right.name()
                ));
            }
            BinaryOp::Shl => {
                self.emit(&format!(
                    "sll {}, {}, {}",
                    dest.name(),
                    left.name(),
                    right.name()
                ));
            }
            BinaryOp::Shr => {
                self.emit(&format!(
                    "sra {}, {}, {}",
                    dest.name(),
                    left.name(),
                    right.name()
                ));
            }
            BinaryOp::Eq => {
                self.emit(&format!(
                    "sub {}, {}, {}",
                    dest.name(),
                    left.name(),
                    right.name()
                ));
                self.emit(&format!("seqz {}, {}", dest.name(), dest.name()));
            }
            BinaryOp::Ne => {
                self.emit(&format!(
                    "sub {}, {}, {}",
                    dest.name(),
                    left.name(),
                    right.name()
                ));
                self.emit(&format!("snez {}, {}", dest.name(), dest.name()));
            }
            BinaryOp::Lt => {
                self.emit(&format!(
                    "slt {}, {}, {}",
                    dest.name(),
                    left.name(),
                    right.name()
                ));
            }
            BinaryOp::Le => {
                // a <= b  is  !(a > b)  is  !(b < a)
                self.emit(&format!(
                    "slt {}, {}, {}",
                    dest.name(),
                    right.name(),
                    left.name()
                ));
                self.emit(&format!("xori {}, {}, 1", dest.name(), dest.name()));
            }
            BinaryOp::Gt => {
                self.emit(&format!(
                    "slt {}, {}, {}",
                    dest.name(),
                    right.name(),
                    left.name()
                ));
            }
            BinaryOp::Ge => {
                // a >= b  is  !(a < b)
                self.emit(&format!(
                    "slt {}, {}, {}",
                    dest.name(),
                    left.name(),
                    right.name()
                ));
                self.emit(&format!("xori {}, {}, 1", dest.name(), dest.name()));
            }
        }
    }

    /// Load float operand into F register
    fn load_float_operand(&mut self, operand: &MirOperand, reg: FReg, is_double: bool) {
        match operand {
            MirOperand::Constant(MirConstant::Float(val, _)) => {
                // Load float via integer register
                let bits = if is_double {
                    val.to_bits()
                } else {
                    (*val as f32).to_bits() as u64
                };
                self.emit(&format!("li t0, {}", bits));
                if is_double {
                    self.emit(&format!("fmv.d.x {}, t0", reg.name()));
                } else {
                    self.emit(&format!("fmv.w.x {}, t0", reg.name()));
                }
            }
            MirOperand::Copy(place) | MirOperand::Move(place) => {
                let src = self.place_to_str(place);
                if is_double {
                    self.emit(&format!("fld {}, {}", reg.name(), src));
                } else {
                    self.emit(&format!("flw {}, {}", reg.name(), src));
                }
            }
            _ => {}
        }
    }

    /// Emit float binary operation
    fn emit_float_binary_op(
        &mut self,
        op: FloatBinaryOp,
        dest: FReg,
        left: FReg,
        right: FReg,
        is_double: bool,
    ) {
        let suffix = if is_double { ".d" } else { ".s" };

        match op {
            FloatBinaryOp::Add => {
                self.emit(&format!(
                    "fadd{} {}, {}, {}",
                    suffix,
                    dest.name(),
                    left.name(),
                    right.name()
                ));
            }
            FloatBinaryOp::Sub => {
                self.emit(&format!(
                    "fsub{} {}, {}, {}",
                    suffix,
                    dest.name(),
                    left.name(),
                    right.name()
                ));
            }
            FloatBinaryOp::Mul => {
                self.emit(&format!(
                    "fmul{} {}, {}, {}",
                    suffix,
                    dest.name(),
                    left.name(),
                    right.name()
                ));
            }
            FloatBinaryOp::Div => {
                self.emit(&format!(
                    "fdiv{} {}, {}, {}",
                    suffix,
                    dest.name(),
                    left.name(),
                    right.name()
                ));
            }
            FloatBinaryOp::Min => {
                self.emit(&format!(
                    "fmin{} {}, {}, {}",
                    suffix,
                    dest.name(),
                    left.name(),
                    right.name()
                ));
            }
            FloatBinaryOp::Max => {
                self.emit(&format!(
                    "fmax{} {}, {}, {}",
                    suffix,
                    dest.name(),
                    left.name(),
                    right.name()
                ));
            }
            FloatBinaryOp::Cmp(cmp) => {
                match cmp {
                    FloatCmp::Eq => self.emit(&format!(
                        "feq{} a0, {}, {}",
                        suffix,
                        left.name(),
                        right.name()
                    )),
                    FloatCmp::Ne => {
                        self.emit(&format!(
                            "feq{} a0, {}, {}",
                            suffix,
                            left.name(),
                            right.name()
                        ));
                        self.emit("xori a0, a0, 1");
                    }
                    FloatCmp::Lt => self.emit(&format!(
                        "flt{} a0, {}, {}",
                        suffix,
                        left.name(),
                        right.name()
                    )),
                    FloatCmp::Le => self.emit(&format!(
                        "fle{} a0, {}, {}",
                        suffix,
                        left.name(),
                        right.name()
                    )),
                    FloatCmp::Gt => self.emit(&format!(
                        "flt{} a0, {}, {}",
                        suffix,
                        right.name(),
                        left.name()
                    )),
                    FloatCmp::Ge => self.emit(&format!(
                        "fle{} a0, {}, {}",
                        suffix,
                        right.name(),
                        left.name()
                    )),
                    FloatCmp::Ord | FloatCmp::Unord => {
                        // Check for NaN
                        self.emit(&format!(
                            "feq{} t0, {}, {}",
                            suffix,
                            left.name(),
                            left.name()
                        ));
                        self.emit(&format!(
                            "feq{} t1, {}, {}",
                            suffix,
                            right.name(),
                            right.name()
                        ));
                        self.emit("and a0, t0, t1");
                        if matches!(cmp, FloatCmp::Unord) {
                            self.emit("xori a0, a0, 1");
                        }
                    }
                }
            }
        }
    }
}

impl AsmEmitter for RiscV64Emitter {
    fn emit_prologue(&mut self, func: &MirFunction) {
        self.current_func = func.name.clone();

        self.emit_directive(&format!(".global {}", func.name));
        self.emit_directive(&format!(".type {}, @function", func.name));
        self.emit_label(&func.name);

        // Calculate stack frame size
        let frame_size = 16 + func.locals.len() * 8; // ra + s0 + locals
        let aligned_size = (frame_size + 15) & !15;

        // Allocate stack frame
        self.emit(&format!("addi sp, sp, -{}", aligned_size));
        self.stack_offset = aligned_size as i64;

        // Save return address and frame pointer
        self.emit(&format!("sd ra, {}(sp)", aligned_size - 8));
        self.emit(&format!("sd s0, {}(sp)", aligned_size - 16));

        // Set up frame pointer
        self.emit(&format!("addi s0, sp, {}", aligned_size));

        // Save callee-saved registers based on kāraka hints
        let callee_saved_needed: Vec<_> = func
            .karaka_hints
            .iter()
            .filter(|(_, hint)| hint.register_class == RegisterClass::CalleeSaved)
            .collect();

        for (_param_idx, _hint) in &callee_saved_needed {
            if let Some(reg) = self.reg_alloc.allocate(RegisterClass::CalleeSaved) {
                self.emit_comment(&format!("kartṛ - preserve {} across calls", reg.name()));
            }
        }

        // Assign stack offsets to locals
        for (i, local) in func.locals.iter().enumerate() {
            let offset = -(((i + 1) * 8) as i64);
            self.reg_alloc.set_local_offset(local.index, offset);
        }

        // Move arguments from registers to stack
        for (i, _param) in func.params.iter().enumerate() {
            if let Some(reg) = RiscVReg::arg_register(i) {
                let offset = -(((func.locals.len() + i + 1) * 8) as i64);
                self.emit_comment(&format!("Store arg {} from {}", i, reg.name()));
                self.emit(&format!("sd {}, {}(s0)", reg.name(), offset));
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
        self.emit_label(&format!(".L{}_epilogue", self.current_func));

        let frame_size = 16 + func.locals.len() * 8;
        let aligned_size = (frame_size + 15) & !15;

        // Restore return address and frame pointer
        self.emit(&format!("ld ra, {}(sp)", aligned_size - 8));
        self.emit(&format!("ld s0, {}(sp)", aligned_size - 16));

        // Deallocate stack frame
        self.emit(&format!("addi sp, sp, {}", aligned_size));

        self.emit("ret");

        self.emit_directive(&format!(".size {}, .-{}", func.name, func.name));
    }

    fn get_asm(&self) -> String {
        let mut output = String::new();

        output.push_str("# Jagannath RISC-V 64 Assembly\n");
        output.push_str("# Generated by jagc compiler\n");
        output.push_str(".text\n\n");

        output.push_str(&self.instructions.join("\n"));
        output
    }

    fn get_machine_code(&self) -> Vec<u8> {
        Vec::new()
    }
}

impl RiscV64Emitter {
    fn emit_mir_instruction(&mut self, instr: &MirInstruction) {
        match instr {
            MirInstruction::Assign { dest, value } => {
                self.emit_comment("Assignment");
                self.emit_rvalue(value, dest);
            }
            MirInstruction::Store { ptr, value } => {
                self.emit_comment("Store through pointer");
                self.load_operand(value, RiscVReg::T0);
                self.load_operand(ptr, RiscVReg::T1);
                self.emit("sd t0, 0(t1)");
            }
            MirInstruction::Load { dest, ptr } => {
                self.emit_comment("Load through pointer");
                self.load_operand(ptr, RiscVReg::T1);
                self.emit("ld t0, 0(t1)");
                self.store_to_place(RiscVReg::T0, dest);
            }
            MirInstruction::SetDiscriminant { place, variant } => {
                self.emit_comment(&format!("Set discriminant to {}", variant));
                self.emit(&format!("li t0, {}", variant));
                self.store_to_place(RiscVReg::T0, place);
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
                self.load_operand(index, RiscVReg::T0);
                self.load_operand(len, RiscVReg::T1);
                let pass_label = self.new_label("bounds_ok");
                self.emit(&format!("bltu t0, t1, {}", pass_label));
                self.emit_comment("Bounds check failed - Asipatravana!");
                self.emit("ebreak");
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
                self.load_operand(condition, RiscVReg::T0);
                let pass_label = self.new_label("assert_pass");
                self.emit(&format!("bnez t0, {}", pass_label));
                self.emit("ebreak");
                self.emit_label(&pass_label);
            }
        }
    }

    fn emit_rvalue(&mut self, rvalue: &MirRvalue, dest: &MirPlace) {
        match rvalue {
            MirRvalue::Use(operand) => {
                self.load_operand(operand, RiscVReg::T0);
                self.store_to_place(RiscVReg::T0, dest);
            }
            MirRvalue::BinaryOp { op, left, right } => {
                self.load_operand(left, RiscVReg::T1);
                self.load_operand(right, RiscVReg::T2);
                self.emit_binary_op(*op, RiscVReg::T0, RiscVReg::T1, RiscVReg::T2);
                self.store_to_place(RiscVReg::T0, dest);
            }
            MirRvalue::UnaryOp { op, operand } => {
                self.load_operand(operand, RiscVReg::T0);
                match op {
                    UnaryOp::Not => self.emit("not t0, t0"),
                    UnaryOp::Neg => self.emit("neg t0, t0"),
                }
                self.store_to_place(RiscVReg::T0, dest);
            }
            MirRvalue::Ref { mutable: _, place } => {
                if let Some(offset) = self.reg_alloc.get_local_offset(place.local) {
                    self.emit(&format!("addi t0, s0, {}", offset));
                } else {
                    self.emit(&format!("addi t0, s0, -{}", (place.local + 1) * 8));
                }
                self.store_to_place(RiscVReg::T0, dest);
            }
            MirRvalue::AddressOf { mutable: _, place } => {
                self.emit_comment("AddressOf - raw pointer creation");
                if let Some(offset) = self.reg_alloc.get_local_offset(place.local) {
                    self.emit(&format!("addi t0, s0, {}", offset));
                } else {
                    self.emit(&format!("addi t0, s0, -{}", (place.local + 1) * 8));
                }
                self.store_to_place(RiscVReg::T0, dest);
            }
            MirRvalue::Field { base, index } => {
                self.emit_comment(&format!("Field access at index {}", index));
                match base {
                    MirOperand::Copy(place) | MirOperand::Move(place) => {
                        let base_offset = self.reg_alloc.get_local_offset(place.local).unwrap_or(0);
                        let field_offset = (index * 8) as i64;
                        let total_offset = base_offset + field_offset;
                        self.emit(&format!("ld t0, {}(s0)", total_offset));
                        self.store_to_place(RiscVReg::T0, dest);
                    }
                    _ => {}
                }
            }
            MirRvalue::Index { base, index } => {
                self.emit_comment("Array index access");
                let elem_size = 8;
                match base {
                    MirOperand::Copy(place) | MirOperand::Move(place) => {
                        let base_offset = self.reg_alloc.get_local_offset(place.local).unwrap_or(0);
                        self.emit(&format!("addi t1, s0, {}", base_offset));
                    }
                    _ => {
                        self.load_operand(base, RiscVReg::T1);
                    }
                }
                self.load_operand(index, RiscVReg::T2);
                self.emit(&format!("li t3, {}", elem_size));
                self.emit("mul t2, t2, t3");
                self.emit("add t1, t1, t2");
                self.emit("ld t0, 0(t1)");
                self.store_to_place(RiscVReg::T0, dest);
            }
            MirRvalue::FloatOp { op, left, right } => {
                self.emit_comment("FloatOp - RVF/RVD operation");
                self.load_float_operand(left, FReg::Ft0, true);
                self.load_float_operand(right, FReg::Ft1, true);
                self.emit_float_binary_op(*op, FReg::Ft0, FReg::Ft0, FReg::Ft1, true);
                let dest_str = self.place_to_str(dest);
                self.emit(&format!("fsd ft0, {}", dest_str));
            }
            MirRvalue::SimdOp {
                op: _op,
                operands,
                width: _,
            } => {
                self.emit_comment("SimdOp - RISC-V Vector extension (if available)");
                // Basic scalar fallback
                if !operands.is_empty() {
                    self.load_operand(&operands[0], RiscVReg::T0);
                    self.store_to_place(RiscVReg::T0, dest);
                }
            }
            MirRvalue::Cast {
                kind: _,
                operand,
                ty: _,
            } => {
                self.load_operand(operand, RiscVReg::T0);
                self.store_to_place(RiscVReg::T0, dest);
            }
            MirRvalue::Discriminant(place) => {
                let src = self.place_to_str(place);
                self.emit(&format!("ld t0, {}", src));
                self.store_to_place(RiscVReg::T0, dest);
            }
            MirRvalue::Len(place) => {
                let src = self.place_to_str(place);
                self.emit(&format!("ld t0, {}", src));
                self.emit("ld t0, 8(t0)");
                self.store_to_place(RiscVReg::T0, dest);
            }
            MirRvalue::Aggregate { kind: _, operands } => {
                for (i, operand) in operands.iter().enumerate() {
                    self.load_operand(operand, RiscVReg::T0);
                    let offset = -(((dest.local + 1) * 8 + i * 8) as i64);
                    self.emit(&format!("sd t0, {}(s0)", offset));
                }
            }
        }
    }

    fn emit_terminator(&mut self, term: &MirTerminator) {
        match term {
            MirTerminator::Return => {
                self.emit(&format!("j .L{}_epilogue", self.current_func));
            }
            MirTerminator::Goto { target } => {
                self.emit(&format!("j .L{}", target));
            }
            MirTerminator::SwitchInt {
                discriminant,
                targets,
                otherwise,
            } => {
                self.emit_comment("Switch on discriminant");
                self.load_operand(discriminant, RiscVReg::T0);
                for (value, target) in targets {
                    self.emit(&format!("li t1, {}", value));
                    self.emit(&format!("beq t0, t1, .L{}", target));
                }
                self.emit(&format!("j .L{}", otherwise));
            }
            MirTerminator::Call {
                func,
                args,
                destination,
                target,
            } => {
                self.emit_comment("Function call");
                for (i, arg) in args.iter().enumerate().take(8) {
                    if let Some(reg) = RiscVReg::arg_register(i) {
                        self.load_operand(arg, reg);
                    }
                }
                // Call function
                match func {
                    MirOperand::Constant(MirConstant::String(name)) => {
                        self.emit(&format!("call {}", name));
                    }
                    _ => {
                        self.load_operand(func, RiscVReg::T0);
                        self.emit("jalr ra, t0, 0");
                    }
                }
                // Store return value
                if let Some(dest) = destination {
                    self.store_to_place(RiscVReg::A0, dest);
                }
                // Continue to target block
                self.emit(&format!("j .L{}", target));
            }
            MirTerminator::Unreachable => {
                self.emit_comment("Unreachable code - trap");
                self.emit("ebreak");
            }
            MirTerminator::Unwind => {
                self.emit_comment("Unwind/panic cleanup");
                self.emit(&format!("j .L{}_epilogue", self.current_func));
            }
        }
    }
}

impl Default for RiscV64Emitter {
    fn default() -> Self {
        Self::new()
    }
}
