//! x86-64 Code Generation (x86-64 कोड निर्माण)
//!
//! Generates x86-64 assembly with kāraka-guided register allocation.
//!
//! ## Kāraka → Register Mapping
//! - **Kartṛ** (Agent) → Callee-saved (RBX, R12-R15) - long-lived
//! - **Karma** (Object) → RAX (return value)
//! - **Karaṇa** (Instrument) → Scratch registers (RCX, RDX)
//! - **Sampradāna** (Recipient) → RDI (first arg)
//! - **Apādāna** (Source) → RSI (second arg)
//!
//! ## ABI
//! Uses System V AMD64 ABI:
//! - Args: RDI, RSI, RDX, RCX, R8, R9
//! - Return: RAX
//! - Callee-saved: RBX, RBP, R12-R15

use super::{AsmEmitter, Instruction, MemoryRef, Operand, Register, RegisterKind};
use crate::mir::types::{
    BinaryOp, IntSize, MirConstant, MirFunction, MirInstruction, MirOperand, MirPlace, MirRvalue,
    MirTerminator, RegisterClass, UnaryOp,
};

/// x86-64 assembly emitter
pub struct X86_64Emitter {
    /// Generated instructions
    instructions: Vec<String>,
    /// Current stack offset
    stack_offset: i64,
    /// Register allocation
    reg_alloc: X86RegAlloc,
    /// Current function name
    current_func: String,
    /// Label counter for unique labels
    label_counter: usize,
}

/// x86-64 registers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum X86Reg {
    // Caller-saved (scratch)
    RAX = 0,
    RCX = 1,
    RDX = 2,
    RSI = 6,
    RDI = 7,
    R8 = 8,
    R9 = 9,
    R10 = 10,
    R11 = 11,

    // Callee-saved (preserved)
    RBX = 3,
    RBP = 5,
    R12 = 12,
    R13 = 13,
    R14 = 14,
    R15 = 15,

    // Stack pointer (special)
    RSP = 4,
}

impl X86Reg {
    pub fn name(&self) -> &'static str {
        match self {
            Self::RAX => "rax",
            Self::RBX => "rbx",
            Self::RCX => "rcx",
            Self::RDX => "rdx",
            Self::RSI => "rsi",
            Self::RDI => "rdi",
            Self::RSP => "rsp",
            Self::RBP => "rbp",
            Self::R8 => "r8",
            Self::R9 => "r9",
            Self::R10 => "r10",
            Self::R11 => "r11",
            Self::R12 => "r12",
            Self::R13 => "r13",
            Self::R14 => "r14",
            Self::R15 => "r15",
        }
    }

    /// Get 32-bit version of register
    pub fn name32(&self) -> &'static str {
        match self {
            Self::RAX => "eax",
            Self::RBX => "ebx",
            Self::RCX => "ecx",
            Self::RDX => "edx",
            Self::RSI => "esi",
            Self::RDI => "edi",
            Self::RSP => "esp",
            Self::RBP => "ebp",
            Self::R8 => "r8d",
            Self::R9 => "r9d",
            Self::R10 => "r10d",
            Self::R11 => "r11d",
            Self::R12 => "r12d",
            Self::R13 => "r13d",
            Self::R14 => "r14d",
            Self::R15 => "r15d",
        }
    }

    /// Get 8-bit version of register
    pub fn name8(&self) -> &'static str {
        match self {
            Self::RAX => "al",
            Self::RBX => "bl",
            Self::RCX => "cl",
            Self::RDX => "dl",
            Self::RSI => "sil",
            Self::RDI => "dil",
            Self::RSP => "spl",
            Self::RBP => "bpl",
            Self::R8 => "r8b",
            Self::R9 => "r9b",
            Self::R10 => "r10b",
            Self::R11 => "r11b",
            Self::R12 => "r12b",
            Self::R13 => "r13b",
            Self::R14 => "r14b",
            Self::R15 => "r15b",
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
        matches!(
            self,
            Self::RBX | Self::RBP | Self::R12 | Self::R13 | Self::R14 | Self::R15
        )
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
    /// Local variable to stack offset mapping
    local_offsets: std::collections::HashMap<usize, i64>,
}

impl X86RegAlloc {
    fn new() -> Self {
        Self {
            caller_saved: vec![
                X86Reg::RAX,
                X86Reg::RCX,
                X86Reg::RDX,
                X86Reg::RSI,
                X86Reg::RDI,
                X86Reg::R8,
                X86Reg::R9,
                X86Reg::R10,
                X86Reg::R11,
            ],
            callee_saved: vec![
                X86Reg::RBX,
                X86Reg::R12,
                X86Reg::R13,
                X86Reg::R14,
                X86Reg::R15,
            ],
            used_callee_saved: Vec::new(),
            local_offsets: std::collections::HashMap::new(),
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
            RegisterClass::General => self.caller_saved.pop().or_else(|| {
                let reg = self.callee_saved.pop()?;
                self.used_callee_saved.push(reg);
                Some(reg)
            }),
        }
    }

    /// Get stack offset for a local variable
    fn get_local_offset(&self, local: usize) -> Option<i64> {
        self.local_offsets.get(&local).copied()
    }

    /// Set stack offset for a local variable
    fn set_local_offset(&mut self, local: usize, offset: i64) {
        self.local_offsets.insert(local, offset);
    }
}

impl X86_64Emitter {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            stack_offset: 0,
            reg_alloc: X86RegAlloc::new(),
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
        self.instructions.push(format!("    ; {}", comment));
    }

    fn emit_directive(&mut self, directive: &str) {
        self.instructions.push(directive.to_string());
    }

    /// Generate unique label
    fn new_label(&mut self, prefix: &str) -> String {
        let label = format!(".L{}_{}", prefix, self.label_counter);
        self.label_counter += 1;
        label
    }

    /// Emit operand to string
    fn operand_to_str(&self, operand: &MirOperand) -> String {
        match operand {
            MirOperand::Constant(c) => self.const_to_str(c),
            MirOperand::Copy(place) | MirOperand::Move(place) => self.place_to_str(place),
        }
    }

    /// Emit constant to string
    fn const_to_str(&self, constant: &MirConstant) -> String {
        match constant {
            MirConstant::Int(val, _) => format!("{}", val),
            MirConstant::Float(val, _) => format!("{}", val),
            MirConstant::Bool(b) => if *b { "1" } else { "0" }.to_string(),
            MirConstant::Unit => "0".to_string(),
            MirConstant::String(s) => format!("OFFSET .LC_{}", s.len()), // Would need string pool
        }
    }

    /// Emit place to stack reference
    fn place_to_str(&self, place: &MirPlace) -> String {
        if let Some(offset) = self.reg_alloc.get_local_offset(place.local) {
            format!(
                "QWORD PTR [rbp{}]",
                if offset >= 0 {
                    format!("+{}", offset)
                } else {
                    format!("{}", offset)
                }
            )
        } else {
            format!("QWORD PTR [rbp-{}]", (place.local + 1) * 8)
        }
    }

    /// Load operand into register
    fn load_operand(&mut self, operand: &MirOperand, reg: X86Reg) {
        match operand {
            MirOperand::Constant(c) => {
                let val = self.const_to_str(c);
                self.emit(&format!("mov {}, {}", reg.name(), val));
            }
            MirOperand::Copy(place) | MirOperand::Move(place) => {
                let src = self.place_to_str(place);
                self.emit(&format!("mov {}, {}", reg.name(), src));
            }
        }
    }

    /// Store register to place
    fn store_to_place(&mut self, reg: X86Reg, place: &MirPlace) {
        let dest = self.place_to_str(place);
        self.emit(&format!("mov {}, {}", dest, reg.name()));
    }

    /// Emit binary operation
    fn emit_binary_op(&mut self, op: BinaryOp, dest: X86Reg, left: X86Reg, right: X86Reg) {
        match op {
            BinaryOp::Add => {
                if dest != left {
                    self.emit(&format!("mov {}, {}", dest.name(), left.name()));
                }
                self.emit(&format!("add {}, {}", dest.name(), right.name()));
            }
            BinaryOp::Sub => {
                if dest != left {
                    self.emit(&format!("mov {}, {}", dest.name(), left.name()));
                }
                self.emit(&format!("sub {}, {}", dest.name(), right.name()));
            }
            BinaryOp::Mul => {
                self.emit(&format!("mov rax, {}", left.name()));
                self.emit(&format!("imul {}", right.name()));
                if dest != X86Reg::RAX {
                    self.emit(&format!("mov {}, rax", dest.name()));
                }
            }
            BinaryOp::Div => {
                self.emit(&format!("mov rax, {}", left.name()));
                self.emit("cqo"); // Sign extend RAX into RDX:RAX
                self.emit(&format!("idiv {}", right.name()));
                if dest != X86Reg::RAX {
                    self.emit(&format!("mov {}, rax", dest.name()));
                }
            }
            BinaryOp::Rem => {
                self.emit(&format!("mov rax, {}", left.name()));
                self.emit("cqo");
                self.emit(&format!("idiv {}", right.name()));
                self.emit(&format!("mov {}, rdx", dest.name())); // Remainder in RDX
            }
            BinaryOp::BitAnd => {
                if dest != left {
                    self.emit(&format!("mov {}, {}", dest.name(), left.name()));
                }
                self.emit(&format!("and {}, {}", dest.name(), right.name()));
            }
            BinaryOp::BitOr => {
                if dest != left {
                    self.emit(&format!("mov {}, {}", dest.name(), left.name()));
                }
                self.emit(&format!("or {}, {}", dest.name(), right.name()));
            }
            BinaryOp::BitXor => {
                if dest != left {
                    self.emit(&format!("mov {}, {}", dest.name(), left.name()));
                }
                self.emit(&format!("xor {}, {}", dest.name(), right.name()));
            }
            BinaryOp::Shl => {
                if dest != left {
                    self.emit(&format!("mov {}, {}", dest.name(), left.name()));
                }
                self.emit(&format!("mov rcx, {}", right.name()));
                self.emit(&format!("shl {}, cl", dest.name()));
            }
            BinaryOp::Shr => {
                if dest != left {
                    self.emit(&format!("mov {}, {}", dest.name(), left.name()));
                }
                self.emit(&format!("mov rcx, {}", right.name()));
                self.emit(&format!("sar {}, cl", dest.name())); // Arithmetic shift
            }
            BinaryOp::Eq
            | BinaryOp::Ne
            | BinaryOp::Lt
            | BinaryOp::Le
            | BinaryOp::Gt
            | BinaryOp::Ge => {
                self.emit(&format!("cmp {}, {}", left.name(), right.name()));
                let set_instr = match op {
                    BinaryOp::Eq => "sete",
                    BinaryOp::Ne => "setne",
                    BinaryOp::Lt => "setl",
                    BinaryOp::Le => "setle",
                    BinaryOp::Gt => "setg",
                    BinaryOp::Ge => "setge",
                    _ => unreachable!(),
                };
                self.emit(&format!("{} {}", set_instr, dest.name8()));
                self.emit(&format!("movzx {}, {}", dest.name(), dest.name8()));
            }
        }
    }
}

impl AsmEmitter for X86_64Emitter {
    fn emit_prologue(&mut self, func: &MirFunction) {
        self.current_func = func.name.clone();

        // Global and extern declarations
        self.emit_directive(&format!(".global {}", func.name));
        self.emit_directive(&format!(".type {}, @function", func.name));
        self.emit_label(&func.name);

        // Standard prologue
        self.emit("push rbp");
        self.emit("mov rbp, rsp");

        // Save callee-saved registers based on kāraka hints
        let callee_saved_needed: Vec<_> = func
            .karaka_hints
            .iter()
            .filter(|(_, hint)| hint.register_class == RegisterClass::CalleeSaved)
            .collect();

        for (param_idx, hint) in &callee_saved_needed {
            if let Some(reg) = self.reg_alloc.allocate(RegisterClass::CalleeSaved) {
                self.emit_comment(&format!("Save {} (kartṛ - agent)", reg.name()));
                self.emit(&format!("push {}", reg.name()));
            }
        }

        // Calculate stack space for locals (aligned to 16 bytes)
        let locals_space = func.locals.len() * 8;
        let aligned_space = (locals_space + 15) & !15;
        if aligned_space > 0 {
            self.emit(&format!("sub rsp, {}", aligned_space));
            self.stack_offset = aligned_space as i64;
        }

        // Assign stack offsets to locals
        for (i, local) in func.locals.iter().enumerate() {
            let offset = -(((i + 1) * 8) as i64);
            self.reg_alloc.set_local_offset(local.index, offset);
        }

        // Move arguments from registers to stack
        for (i, param) in func.params.iter().enumerate() {
            if let Some(reg) = X86Reg::arg_register(i) {
                let offset = -(((func.locals.len() + i + 1) * 8) as i64);
                self.emit_comment(&format!("Store arg {} from {}", i, reg.name()));
                self.emit(&format!("mov QWORD PTR [rbp{}], {}", offset, reg.name()));
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
        // Emit epilogue label for multiple return points
        self.emit_label(&format!(".L{}_epilogue", self.current_func));

        // Restore stack
        if self.stack_offset > 0 {
            self.emit(&format!("add rsp, {}", self.stack_offset));
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
            self.emit(&format!("pop {}", reg.name()));
        }

        // Standard epilogue
        self.emit("pop rbp");
        self.emit("ret");

        // Function size directive
        self.emit_directive(&format!(".size {}, .-{}", func.name, func.name));
    }

    fn get_asm(&self) -> String {
        let mut output = String::new();

        // Assembly header
        output.push_str("; Jagannath x86-64 Assembly\n");
        output.push_str("; Generated by jagc compiler\n");
        output.push_str(".intel_syntax noprefix\n");
        output.push_str(".text\n\n");

        // Instructions
        output.push_str(&self.instructions.join("\n"));

        output
    }

    fn get_machine_code(&self) -> Vec<u8> {
        // Would use an assembler like NASM or integrate with Cranelift
        Vec::new()
    }
}

impl X86_64Emitter {
    fn emit_mir_instruction(&mut self, instr: &MirInstruction) {
        match instr {
            MirInstruction::Assign { dest, value } => {
                self.emit_comment("Assignment");
                self.emit_rvalue(value, dest);
            }
            MirInstruction::Drop { place } => {
                self.emit_comment(&format!("Drop local {}", place.local));
                // For primitive types, drop is a no-op
                // For complex types, would call destructor
            }
            MirInstruction::Nop => {
                self.emit("nop");
            }
            MirInstruction::Assert { condition, message } => {
                self.emit_comment(&format!("Assert: {}", message));
                self.load_operand(condition, X86Reg::RAX);
                self.emit("test rax, rax");
                let pass_label = self.new_label("assert_pass");
                self.emit(&format!("jnz {}", pass_label));
                // On failure, call panic (would need runtime linkage)
                self.emit("ud2"); // Undefined instruction trap
                self.emit_label(&pass_label);
            }
        }
    }

    fn emit_rvalue(&mut self, rvalue: &MirRvalue, dest: &MirPlace) {
        match rvalue {
            MirRvalue::Use(operand) => {
                self.load_operand(operand, X86Reg::RAX);
                self.store_to_place(X86Reg::RAX, dest);
            }
            MirRvalue::BinaryOp { op, left, right } => {
                self.load_operand(left, X86Reg::RCX);
                self.load_operand(right, X86Reg::RDX);
                self.emit_binary_op(*op, X86Reg::RAX, X86Reg::RCX, X86Reg::RDX);
                self.store_to_place(X86Reg::RAX, dest);
            }
            MirRvalue::UnaryOp { op, operand } => {
                self.load_operand(operand, X86Reg::RAX);
                match op {
                    UnaryOp::Not => self.emit("not rax"),
                    UnaryOp::Neg => self.emit("neg rax"),
                }
                self.store_to_place(X86Reg::RAX, dest);
            }
            MirRvalue::Ref { mutable: _, place } => {
                // Load effective address
                let src = self.place_to_str(place);
                self.emit(&format!("lea rax, {}", src));
                self.store_to_place(X86Reg::RAX, dest);
            }
            MirRvalue::Cast {
                kind: _,
                operand,
                ty: _,
            } => {
                // For now, just copy (proper casting would need type info)
                self.load_operand(operand, X86Reg::RAX);
                self.store_to_place(X86Reg::RAX, dest);
            }
            MirRvalue::Discriminant(place) => {
                // Load discriminant (first field of enum)
                let src = self.place_to_str(place);
                self.emit(&format!("mov rax, {}", src));
                self.store_to_place(X86Reg::RAX, dest);
            }
            MirRvalue::Len(place) => {
                // Load length (for slices, stored after pointer)
                let src = self.place_to_str(place);
                self.emit(&format!("mov rax, {}", src));
                self.emit("mov rax, QWORD PTR [rax+8]"); // Length at offset 8
                self.store_to_place(X86Reg::RAX, dest);
            }
            MirRvalue::Aggregate { kind: _, operands } => {
                // Store each operand at consecutive offsets
                for (i, operand) in operands.iter().enumerate() {
                    self.load_operand(operand, X86Reg::RAX);
                    let offset = -(((dest.local + 1) * 8 + i * 8) as i64);
                    self.emit(&format!("mov QWORD PTR [rbp{}], rax", offset));
                }
            }
        }
    }

    fn emit_terminator(&mut self, term: &MirTerminator) {
        match term {
            MirTerminator::Return => {
                // Jump to epilogue
                self.emit(&format!("jmp .L{}_epilogue", self.current_func));
            }
            MirTerminator::Goto { target } => {
                self.emit(&format!("jmp .L{}", target));
            }
            MirTerminator::SwitchInt {
                discriminant,
                targets,
                otherwise,
            } => {
                self.emit_comment("Switch on discriminant");
                self.load_operand(discriminant, X86Reg::RAX);
                for (value, target) in targets {
                    self.emit(&format!("cmp rax, {}", value));
                    self.emit(&format!("je .L{}", target));
                }
                self.emit(&format!("jmp .L{}", otherwise));
            }
            MirTerminator::Call {
                func,
                args,
                destination,
                target,
            } => {
                self.emit_comment("Function call");
                // Load arguments into registers
                for (i, arg) in args.iter().enumerate() {
                    if let Some(reg) = X86Reg::arg_register(i) {
                        self.load_operand(arg, reg);
                    } else {
                        // Push to stack (in reverse order)
                        self.load_operand(arg, X86Reg::RAX);
                        self.emit("push rax");
                    }
                }
                // Call function
                match func {
                    MirOperand::Constant(MirConstant::String(name)) => {
                        self.emit(&format!("call {}", name));
                    }
                    _ => {
                        self.load_operand(func, X86Reg::RAX);
                        self.emit("call rax");
                    }
                }
                // Store return value
                if let Some(dest) = destination {
                    self.store_to_place(X86Reg::RAX, dest);
                }
                // Continue to target block
                self.emit(&format!("jmp .L{}", target));
            }
            MirTerminator::Unreachable => {
                self.emit_comment("Unreachable code");
                self.emit("ud2");
            }
            MirTerminator::Unwind => {
                self.emit_comment("Unwind/panic cleanup");
                // Would integrate with runtime panic handling
                self.emit(&format!("jmp .L{}_epilogue", self.current_func));
            }
        }
    }
}

impl Default for X86_64Emitter {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Hello World Support
// ============================================================================

impl X86_64Emitter {
    /// Generate a simple hello world program
    pub fn generate_hello_world() -> String {
        let mut asm = String::new();

        asm.push_str("; Jagannath Hello World (जगन्नाथ नमस्ते विश्व)\n");
        asm.push_str("; Generated by jagc compiler\n\n");
        asm.push_str(".intel_syntax noprefix\n\n");

        // Data section
        asm.push_str(".section .data\n");
        asm.push_str("    message: .asciz \"नमस्ते जगन्नाथ! (Namaste Jagannath!)\\n\"\n");
        asm.push_str("    msg_len = . - message\n\n");

        // Text section
        asm.push_str(".section .text\n");
        asm.push_str(".global _start\n\n");

        // _start entry point
        asm.push_str("_start:\n");
        asm.push_str("    ; Write syscall (sys_write = 1)\n");
        asm.push_str("    mov rax, 1          ; syscall number\n");
        asm.push_str("    mov rdi, 1          ; stdout\n");
        asm.push_str("    lea rsi, [message]  ; message address\n");
        asm.push_str("    mov rdx, msg_len    ; message length\n");
        asm.push_str("    syscall\n\n");

        asm.push_str("    ; Exit syscall (sys_exit = 60)\n");
        asm.push_str("    mov rax, 60         ; syscall number\n");
        asm.push_str("    xor rdi, rdi        ; exit code 0\n");
        asm.push_str("    syscall\n");

        asm
    }

    /// Generate Windows hello world (uses different ABI)
    pub fn generate_hello_world_windows() -> String {
        let mut asm = String::new();

        asm.push_str("; Jagannath Hello World for Windows\n");
        asm.push_str("; Generated by jagc compiler\n\n");
        asm.push_str(".intel_syntax noprefix\n\n");

        // External declarations
        asm.push_str("extern GetStdHandle\n");
        asm.push_str("extern WriteConsoleA\n");
        asm.push_str("extern ExitProcess\n\n");

        // Data section
        asm.push_str("section .data\n");
        asm.push_str("    message db 'Namaste Jagannath!', 13, 10, 0\n");
        asm.push_str("    msg_len equ $ - message - 1\n");
        asm.push_str("    written dq 0\n\n");

        // Text section
        asm.push_str("section .text\n");
        asm.push_str("global main\n\n");

        asm.push_str("main:\n");
        asm.push_str("    sub rsp, 40          ; Shadow space + alignment\n\n");

        asm.push_str("    ; GetStdHandle(-11) for stdout\n");
        asm.push_str("    mov rcx, -11\n");
        asm.push_str("    call GetStdHandle\n");
        asm.push_str("    mov rbx, rax          ; Save handle\n\n");

        asm.push_str("    ; WriteConsoleA(handle, message, len, &written, NULL)\n");
        asm.push_str("    mov rcx, rbx\n");
        asm.push_str("    lea rdx, [message]\n");
        asm.push_str("    mov r8, msg_len\n");
        asm.push_str("    lea r9, [written]\n");
        asm.push_str("    mov qword ptr [rsp+32], 0\n");
        asm.push_str("    call WriteConsoleA\n\n");

        asm.push_str("    ; ExitProcess(0)\n");
        asm.push_str("    xor rcx, rcx\n");
        asm.push_str("    call ExitProcess\n");

        asm
    }
}
