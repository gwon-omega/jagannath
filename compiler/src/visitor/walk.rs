//! Walk Functions - Default Traversal Implementations
//!
//! Provides walk_* functions for common traversal patterns.
//! These are used by visitors that only need to override specific nodes.

use crate::mir::types::*;
use super::VisitResult;
use super::mir_visitor::MirVisitor;
use std::ops::ControlFlow;

// ============================================================================
// MIR Walk Functions
// ============================================================================

/// Walk all functions in a MIR module
pub fn walk_mir_module<V: MirVisitor>(visitor: &mut V, module: &MirModule) -> VisitResult<V::Break> {
    for func in &module.functions {
        visitor.visit_function(func)?;
    }
    for global in &module.globals {
        visitor.visit_global(global)?;
    }
    ControlFlow::Continue(())
}

/// Walk a MIR function
pub fn walk_mir_function<V: MirVisitor>(
    visitor: &mut V,
    func: &MirFunction,
) -> VisitResult<V::Break> {
    for block in &func.blocks {
        visitor.visit_basic_block(block)?;
    }
    ControlFlow::Continue(())
}

/// Walk a basic block
pub fn walk_basic_block<V: MirVisitor>(
    visitor: &mut V,
    block: &MirBasicBlock,
) -> VisitResult<V::Break> {
    for instr in &block.instructions {
        visitor.visit_instruction(instr)?;
    }
    visitor.visit_terminator(&block.terminator)
}

/// Walk an r-value
pub fn walk_rvalue<V: MirVisitor>(visitor: &mut V, rv: &MirRvalue) -> VisitResult<V::Break> {
    match rv {
        MirRvalue::Use(op) => {
            visitor.visit_operand(op)?;
        }
        MirRvalue::Ref { place, .. } => {
            visitor.visit_place(place)?;
        }
        MirRvalue::BinaryOp { left, right, .. } => {
            visitor.visit_operand(left)?;
            visitor.visit_operand(right)?;
        }
        MirRvalue::UnaryOp { operand, .. } => {
            visitor.visit_operand(operand)?;
        }
        MirRvalue::Aggregate { operands, .. } => {
            for op in operands {
                visitor.visit_operand(op)?;
            }
        }
        MirRvalue::Cast { operand, .. } => {
            visitor.visit_operand(operand)?;
        }
        MirRvalue::Discriminant(place) | MirRvalue::Len(place) => {
            visitor.visit_place(place)?;
        }
        MirRvalue::AddressOf { place, .. } => {
            visitor.visit_place(place)?;
        }
        MirRvalue::Field { base, .. } => {
            visitor.visit_operand(base)?;
        }
        MirRvalue::Index { base, index } => {
            visitor.visit_operand(base)?;
            visitor.visit_operand(index)?;
        }
        MirRvalue::FloatOp { left, right, .. } => {
            visitor.visit_operand(left)?;
            visitor.visit_operand(right)?;
        }
        MirRvalue::SimdOp { operands, .. } => {
            for op in operands {
                visitor.visit_operand(op)?;
            }
        }
    }
    ControlFlow::Continue(())
}

// ============================================================================
// Depth-First Traversal Utilities
// ============================================================================

/// Perform post-order DFS on MIR basic blocks
/// Returns blocks in reverse post-order (RPO) - useful for dataflow
pub fn mir_post_order(func: &MirFunction) -> Vec<usize> {
    let mut visited = vec![false; func.blocks.len()];
    let mut post_order = Vec::new();

    fn dfs(func: &MirFunction, block_id: usize, visited: &mut [bool], post_order: &mut Vec<usize>) {
        if block_id >= visited.len() || visited[block_id] {
            return;
        }
        visited[block_id] = true;

        // Visit successors first
        let block = &func.blocks[block_id];
        for succ in successors(&block.terminator) {
            dfs(func, succ, visited, post_order);
        }

        // Then add self (post-order)
        post_order.push(block_id);
    }

    dfs(func, 0, &mut visited, &mut post_order);
    post_order
}

/// Get reverse post-order (RPO) - topological order for forward dataflow
pub fn mir_reverse_post_order(func: &MirFunction) -> Vec<usize> {
    let mut rpo = mir_post_order(func);
    rpo.reverse();
    rpo
}

/// Get successors of a terminator
fn successors(term: &MirTerminator) -> Vec<usize> {
    match term {
        MirTerminator::Goto { target } => vec![*target],
        MirTerminator::SwitchInt { targets, otherwise, .. } => {
            let mut succs: Vec<_> = targets.iter().map(|(_, t)| *t).collect();
            succs.push(*otherwise);
            succs.dedup();
            succs
        }
        MirTerminator::Call { target, .. } => {
            vec![*target]
        }
        MirTerminator::Return | MirTerminator::Unreachable | MirTerminator::Unwind => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn make_cfg() -> MirFunction {
        // Simple CFG: entry -> loop_header -> (loop_body -> loop_header | exit)
        MirFunction {
            name: "test_cfg".to_string(),
            params: vec![],
            return_type: MirType::Unit,
            blocks: vec![
                MirBasicBlock {
                    id: 0, // entry
                    instructions: vec![],
                    terminator: MirTerminator::Goto { target: 1 },
                },
                MirBasicBlock {
                    id: 1, // loop_header
                    instructions: vec![],
                    terminator: MirTerminator::SwitchInt {
                        discriminant: MirOperand::Constant(MirConstant::Bool(true)),
                        targets: vec![(1, 2)], // true->body
                        otherwise: 3,          // false->exit
                    },
                },
                MirBasicBlock {
                    id: 2, // loop_body
                    instructions: vec![],
                    terminator: MirTerminator::Goto { target: 1 },
                },
                MirBasicBlock {
                    id: 3, // exit
                    instructions: vec![],
                    terminator: MirTerminator::Return,
                },
            ],
            locals: vec![],
            karaka_hints: HashMap::new(),
        }
    }

    #[test]
    fn test_post_order() {
        let func = make_cfg();
        let post = mir_post_order(&func);

        // Post-order visits children before parents
        // exit(3) should come before entry(0)
        let exit_pos = post.iter().position(|&x| x == 3).unwrap();
        let entry_pos = post.iter().position(|&x| x == 0).unwrap();
        assert!(
            exit_pos < entry_pos,
            "Exit should be visited before entry in post-order"
        );
    }

    #[test]
    fn test_reverse_post_order() {
        let func = make_cfg();
        let rpo = mir_reverse_post_order(&func);

        // RPO: entry should come first
        assert_eq!(rpo[0], 0, "Entry should be first in RPO");
    }

    #[test]
    fn test_successors() {
        let term = MirTerminator::SwitchInt {
            discriminant: MirOperand::Constant(MirConstant::Bool(true)),
            targets: vec![(0, 1)],
            otherwise: 2,
        };

        let succs = successors(&term);
        assert!(succs.contains(&1));
        assert!(succs.contains(&2));
    }
}
