//! MIR Visitor - Traversal for Mid-level IR
//!
//! Provides traits for traversing the Mid-level Intermediate Representation.
//! Essential for optimization passes (Astras) and analysis.

use crate::mir::types::*;
use super::VisitResult;
use std::ops::ControlFlow;

/// Immutable MIR visitor - for analysis passes
///
/// # Astra Mapping
/// Each optimization pass (Astra) implements MirVisitor:
/// - Brahmastra (DCE): Visits to mark used code
/// - Varunastra (flow): Visits to track data flow
/// - Sudarshana (refinement): Multiple visit passes
pub trait MirVisitor {
    /// Associated break type for early termination
    type Break;

    /// Continue helper for correct typing
    fn continue_(&self) -> VisitResult<Self::Break> {
        ControlFlow::Continue(())
    }

    /// Visit an entire MIR module
    fn visit_module(&mut self, module: &MirModule) -> VisitResult<Self::Break> {
        for func in &module.functions {
            self.visit_function(func)?;
        }
        for global in &module.globals {
            self.visit_global(global)?;
        }
        self.continue_()
    }

    /// Visit a function
    fn visit_function(&mut self, func: &MirFunction) -> VisitResult<Self::Break> {
        for block in &func.blocks {
            self.visit_basic_block(block)?;
        }
        self.continue_()
    }

    /// Visit a global variable
    fn visit_global(&mut self, _global: &MirGlobal) -> VisitResult<Self::Break> {
        self.continue_()
    }

    /// Visit a basic block
    fn visit_basic_block(&mut self, block: &MirBasicBlock) -> VisitResult<Self::Break> {
        for instr in &block.instructions {
            self.visit_instruction(instr)?;
        }
        self.visit_terminator(&block.terminator)
    }

    /// Visit an instruction
    fn visit_instruction(&mut self, instr: &MirInstruction) -> VisitResult<Self::Break> {
        match instr {
            MirInstruction::Assign { dest, value } => {
                self.visit_place(dest)?;
                self.visit_rvalue(value)?;
            }
            MirInstruction::Drop { place } => {
                self.visit_place(place)?;
            }
            MirInstruction::Store { ptr, value } => {
                self.visit_operand(ptr)?;
                self.visit_operand(value)?;
            }
            MirInstruction::Load { dest, ptr } => {
                self.visit_place(dest)?;
                self.visit_operand(ptr)?;
            }
            MirInstruction::Assert { condition, .. } => {
                self.visit_operand(condition)?;
            }
            MirInstruction::BoundsCheck { index, len, .. } => {
                self.visit_operand(index)?;
                self.visit_operand(len)?;
            }
            MirInstruction::SetDiscriminant { place, .. } => {
                self.visit_place(place)?;
            }
            MirInstruction::Nop => {}
        }
        self.continue_()
    }

    /// Visit a terminator
    fn visit_terminator(&mut self, term: &MirTerminator) -> VisitResult<Self::Break> {
        match term {
            MirTerminator::Goto { .. } => {}
            MirTerminator::SwitchInt { discriminant, .. } => {
                self.visit_operand(discriminant)?;
            }
            MirTerminator::Return => {}
            MirTerminator::Call { func, args, destination, .. } => {
                self.visit_operand(func)?;
                for arg in args {
                    self.visit_operand(arg)?;
                }
                if let Some(dest) = destination {
                    self.visit_place(dest)?;
                }
            }
            MirTerminator::Unreachable | MirTerminator::Unwind => {}
        }
        self.continue_()
    }

    /// Visit a place (l-value)
    fn visit_place(&mut self, _place: &MirPlace) -> VisitResult<Self::Break> {
        self.continue_()
    }

    /// Visit an operand
    fn visit_operand(&mut self, op: &MirOperand) -> VisitResult<Self::Break> {
        match op {
            MirOperand::Copy(place) | MirOperand::Move(place) => {
                self.visit_place(place)?;
            }
            MirOperand::Constant(c) => {
                self.visit_constant(c)?;
            }
        }
        self.continue_()
    }

    /// Visit an r-value
    fn visit_rvalue(&mut self, rv: &MirRvalue) -> VisitResult<Self::Break> {
        match rv {
            MirRvalue::Use(op) => {
                self.visit_operand(op)?;
            }
            MirRvalue::Ref { place, .. } => {
                self.visit_place(place)?;
            }
            MirRvalue::BinaryOp { left, right, .. } => {
                self.visit_operand(left)?;
                self.visit_operand(right)?;
            }
            MirRvalue::UnaryOp { operand, .. } => {
                self.visit_operand(operand)?;
            }
            MirRvalue::Aggregate { operands, .. } => {
                for op in operands {
                    self.visit_operand(op)?;
                }
            }
            MirRvalue::Cast { operand, .. } => {
                self.visit_operand(operand)?;
            }
            MirRvalue::Discriminant(place) | MirRvalue::Len(place) => {
                self.visit_place(place)?;
            }
            MirRvalue::AddressOf { place, .. } => {
                self.visit_place(place)?;
            }
            MirRvalue::Field { base, .. } => {
                self.visit_operand(base)?;
            }
            MirRvalue::Index { base, index } => {
                self.visit_operand(base)?;
                self.visit_operand(index)?;
            }
            MirRvalue::FloatOp { left, right, .. } => {
                self.visit_operand(left)?;
                self.visit_operand(right)?;
            }
            MirRvalue::SimdOp { operands, .. } => {
                for op in operands {
                    self.visit_operand(op)?;
                }
            }
        }
        self.continue_()
    }

    /// Visit a constant
    fn visit_constant(&mut self, _constant: &MirConstant) -> VisitResult<Self::Break> {
        self.continue_()
    }
}

/// Mutable MIR visitor - for transformation passes
///
/// Used by Astras that modify MIR:
/// - Brahmastra: Remove dead instructions
/// - Pashupatastra: Inline functions
/// - Vayuastra: Simplify control flow
pub trait MirVisitorMut {
    /// Associated break type
    type Break;

    /// Continue helper
    fn continue_(&self) -> VisitResult<Self::Break> {
        ControlFlow::Continue(())
    }

    /// Visit and potentially modify a function
    fn visit_function_mut(&mut self, func: &mut MirFunction) -> VisitResult<Self::Break> {
        for block in &mut func.blocks {
            self.visit_basic_block_mut(block)?;
        }
        self.continue_()
    }

    /// Visit and potentially modify a basic block
    fn visit_basic_block_mut(&mut self, block: &mut MirBasicBlock) -> VisitResult<Self::Break> {
        for instr in &mut block.instructions {
            self.visit_instruction_mut(instr)?;
        }
        self.visit_terminator_mut(&mut block.terminator)
    }

    /// Visit and potentially modify an instruction
    fn visit_instruction_mut(&mut self, _instr: &mut MirInstruction) -> VisitResult<Self::Break> {
        self.continue_()
    }

    /// Visit and potentially modify a terminator
    fn visit_terminator_mut(&mut self, _term: &mut MirTerminator) -> VisitResult<Self::Break> {
        self.continue_()
    }

    /// Visit and potentially modify an operand
    fn visit_operand_mut(&mut self, _op: &mut MirOperand) -> VisitResult<Self::Break> {
        self.continue_()
    }

    /// Visit and potentially modify an r-value
    fn visit_rvalue_mut(&mut self, _rv: &mut MirRvalue) -> VisitResult<Self::Break> {
        self.continue_()
    }
}

/// Use collector - finds all uses of a variable
pub struct UseCollector {
    /// Variable we're looking for
    pub target: usize,
    /// Locations where it's used
    pub uses: Vec<UseLocation>,
}

/// Location of a use
#[derive(Debug, Clone)]
pub struct UseLocation {
    pub block: usize,
    pub instruction: usize,
    pub kind: UseKind,
}

/// Kind of use
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UseKind {
    Read,
    Write,
    ReadWrite,
}

impl UseCollector {
    pub fn new(target: usize) -> Self {
        Self {
            target,
            uses: Vec::new(),
        }
    }

    pub fn collect(&mut self, func: &MirFunction) {
        for (block_idx, block) in func.blocks.iter().enumerate() {
            for (instr_idx, instr) in block.instructions.iter().enumerate() {
                if let Some(kind) = self.check_instruction(instr) {
                    self.uses.push(UseLocation {
                        block: block_idx,
                        instruction: instr_idx,
                        kind,
                    });
                }
            }
        }
    }

    fn check_instruction(&self, instr: &MirInstruction) -> Option<UseKind> {
        match instr {
            MirInstruction::Assign { dest, value } => {
                let is_def = dest.local == self.target;
                let is_use = self.rvalue_uses(value);

                match (is_def, is_use) {
                    (true, true) => Some(UseKind::ReadWrite),
                    (true, false) => Some(UseKind::Write),
                    (false, true) => Some(UseKind::Read),
                    (false, false) => None,
                }
            }
            MirInstruction::Drop { place } => {
                if place.local == self.target {
                    Some(UseKind::Read)
                } else {
                    None
                }
            }
            MirInstruction::Store { ptr, value } => {
                if self.operand_uses(ptr) || self.operand_uses(value) {
                    Some(UseKind::Read)
                } else {
                    None
                }
            }
            MirInstruction::Load { dest, ptr } => {
                let is_def = dest.local == self.target;
                let is_use = self.operand_uses(ptr);

                match (is_def, is_use) {
                    (true, true) => Some(UseKind::ReadWrite),
                    (true, false) => Some(UseKind::Write),
                    (false, true) => Some(UseKind::Read),
                    (false, false) => None,
                }
            }
            _ => None,
        }
    }

    fn operand_uses(&self, op: &MirOperand) -> bool {
        match op {
            MirOperand::Copy(place) | MirOperand::Move(place) => place.local == self.target,
            MirOperand::Constant(_) => false,
        }
    }

    fn rvalue_uses(&self, rv: &MirRvalue) -> bool {
        match rv {
            MirRvalue::Use(op) => self.operand_uses(op),
            MirRvalue::Ref { place, .. } => place.local == self.target,
            MirRvalue::BinaryOp { left, right, .. } => {
                self.operand_uses(left) || self.operand_uses(right)
            }
            MirRvalue::UnaryOp { operand, .. } => self.operand_uses(operand),
            MirRvalue::Aggregate { operands, .. } => {
                operands.iter().any(|op| self.operand_uses(op))
            }
            MirRvalue::Cast { operand, .. } => self.operand_uses(operand),
            MirRvalue::Discriminant(place) | MirRvalue::Len(place) => place.local == self.target,
            MirRvalue::AddressOf { place, .. } => place.local == self.target,
            MirRvalue::Field { base, .. } => self.operand_uses(base),
            MirRvalue::Index { base, index } => {
                self.operand_uses(base) || self.operand_uses(index)
            }
            MirRvalue::FloatOp { left, right, .. } => {
                self.operand_uses(left) || self.operand_uses(right)
            }
            MirRvalue::SimdOp { operands, .. } => operands.iter().any(|op| self.operand_uses(op)),
        }
    }
}

/// Def collector - finds all definitions of a variable
pub struct DefCollector {
    /// Variable we're looking for
    pub target: usize,
    /// Locations where it's defined
    pub defs: Vec<DefLocation>,
}

/// Location of a definition
#[derive(Debug, Clone)]
pub struct DefLocation {
    pub block: usize,
    pub instruction: usize,
}

impl DefCollector {
    pub fn new(target: usize) -> Self {
        Self {
            target,
            defs: Vec::new(),
        }
    }

    pub fn collect(&mut self, func: &MirFunction) {
        for (block_idx, block) in func.blocks.iter().enumerate() {
            for (instr_idx, instr) in block.instructions.iter().enumerate() {
                if self.is_def(instr) {
                    self.defs.push(DefLocation {
                        block: block_idx,
                        instruction: instr_idx,
                    });
                }
            }
        }
    }

    fn is_def(&self, instr: &MirInstruction) -> bool {
        match instr {
            MirInstruction::Assign { dest, .. } => dest.local == self.target,
            MirInstruction::Load { dest, .. } => dest.local == self.target,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_func() -> MirFunction {
        MirFunction {
            name: "test".to_string(),
            params: vec![],
            return_type: MirType::Unit,
            blocks: vec![MirBasicBlock {
                id: 0,
                instructions: vec![
                    MirInstruction::Assign {
                        dest: MirPlace {
                            local: 0,
                            projection: vec![],
                        },
                        value: MirRvalue::Use(MirOperand::Constant(MirConstant::Int(
                            42,
                            IntSize::I64,
                        ))),
                    },
                    MirInstruction::Assign {
                        dest: MirPlace {
                            local: 1,
                            projection: vec![],
                        },
                        value: MirRvalue::Use(MirOperand::Copy(MirPlace {
                            local: 0,
                            projection: vec![],
                        })),
                    },
                ],
                terminator: MirTerminator::Return,
            }],
            locals: vec![],
            karaka_hints: HashMap::new(),
        }
    }

    #[test]
    fn test_use_collector() {
        let func = make_func();
        let mut collector = UseCollector::new(0);
        collector.collect(&func);

        assert_eq!(collector.uses.len(), 2);
        // First is Write (def), second is Read (use)
        assert_eq!(collector.uses[0].kind, UseKind::Write);
        assert_eq!(collector.uses[1].kind, UseKind::Read);
    }

    #[test]
    fn test_def_collector() {
        let func = make_func();
        let mut collector = DefCollector::new(0);
        collector.collect(&func);

        assert_eq!(collector.defs.len(), 1);
        assert_eq!(collector.defs[0].block, 0);
        assert_eq!(collector.defs[0].instruction, 0);
    }

    /// Simple visitor that counts instructions
    struct InstructionCounter {
        count: usize,
    }

    impl MirVisitor for InstructionCounter {
        type Break = ();

        fn visit_instruction(&mut self, _instr: &MirInstruction) -> VisitResult<Self::Break> {
            self.count += 1;
            self.continue_()
        }
    }

    #[test]
    fn test_mir_visitor() {
        let func = make_func();
        let module = MirModule {
            name: "test".to_string(),
            functions: vec![func],
            globals: vec![],
            types: vec![],
        };

        let mut counter = InstructionCounter { count: 0 };
        let _ = counter.visit_module(&module);

        assert_eq!(counter.count, 2);
    }

    #[test]
    fn test_use_location() {
        let loc = UseLocation {
            block: 0,
            instruction: 1,
            kind: UseKind::Read,
        };

        assert_eq!(loc.block, 0);
        assert_eq!(loc.instruction, 1);
        assert_eq!(loc.kind, UseKind::Read);
    }
}
