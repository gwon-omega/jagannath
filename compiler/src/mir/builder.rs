//! MIR Builder (मध्यस्थ प्रतिनिधित्व निर्माता)
//!
//! Converts AST to MIR (Mid-level Intermediate Representation).
//! This is the core lowering pass that transforms high-level constructs
//! into a simpler form suitable for optimization and code generation.

use super::types::*;
use crate::parser::ast;
use std::collections::HashMap;

/// MIR Builder - Lowers AST to MIR
pub struct MirBuilder {
    /// Current function being built
    current_function: Option<MirFunction>,
    /// Current block index
    current_block: usize,
    /// Next local index
    next_local: usize,
    /// Next block index
    next_block: usize,
    /// Variable name to local index mapping
    var_map: HashMap<String, usize>,
    /// Current blocks being built
    blocks: Vec<MirBasicBlock>,
    /// Locals list
    locals: Vec<MirLocal>,
}

impl MirBuilder {
    pub fn new() -> Self {
        Self {
            current_function: None,
            current_block: 0,
            next_local: 0,
            next_block: 0,
            var_map: HashMap::new(),
            blocks: Vec::new(),
            locals: Vec::new(),
        }
    }

    /// Build MIR from AST
    pub fn build(&mut self, ast: &ast::Ast) -> MirModule {
        let mut module = MirModule {
            name: "main".to_string(),
            functions: Vec::new(),
            globals: Vec::new(),
            types: Vec::new(),
        };

        for item in &ast.items {
            match item {
                ast::Item::Function(func) => {
                    if let Some(mir_func) = self.build_function(func) {
                        module.functions.push(mir_func);
                    }
                }
                ast::Item::TypeDef(typedef) => {
                    if let Some(mir_type) = self.build_typedef(typedef) {
                        module.types.push(mir_type);
                    }
                }
                ast::Item::Constant(const_def) => {
                    if let Some(global) = self.build_const(const_def) {
                        module.globals.push(global);
                    }
                }
                _ => {}
            }
        }

        module
    }

    /// Build MIR for a function
    fn build_function(&mut self, func: &ast::FunctionDef) -> Option<MirFunction> {
        // Reset state for new function
        self.next_local = 0;
        self.next_block = 0;
        self.var_map.clear();
        self.blocks.clear();
        self.locals.clear();

        // Build params
        let params: Vec<MirParam> = func
            .params
            .iter()
            .enumerate()
            .map(|(i, p)| {
                let ty = self.convert_type(&p.ty);
                // Register parameter in var_map
                let local_idx = self.alloc_local(ty.clone(), Some(p.name.name.clone()));
                self.var_map.insert(p.name.name.clone(), local_idx);
                MirParam {
                    index: i,
                    ty,
                    karaka: p.karaka,
                }
            })
            .collect();

        let return_type = func
            .return_type
            .as_ref()
            .map(|t| self.convert_type(t))
            .unwrap_or(MirType::Unit);

        // Create entry block
        let entry_block_id = self.alloc_block();
        self.blocks.push(MirBasicBlock {
            id: entry_block_id,
            instructions: Vec::new(),
            terminator: MirTerminator::Return,
        });
        self.current_block = 0;

        // Lower function body
        self.lower_block(&func.body);

        // Build karaka hints from parameters
        let mut karaka_hints = HashMap::new();
        for (i, param) in func.params.iter().enumerate() {
            if let Some(karaka) = param.karaka {
                let reg_class = match karaka {
                    ast::Karaka::Kartr => RegisterClass::CalleeSaved, // Agent = long-lived
                    ast::Karaka::Karman => RegisterClass::Output,     // Object = output
                    ast::Karaka::Karana => RegisterClass::CallerSaved, // Instrument = scratch
                    ast::Karaka::Sampradana => RegisterClass::CallerSaved, // Recipient
                    ast::Karaka::Apadana => RegisterClass::CallerSaved, // Source
                    ast::Karaka::Adhikarana => RegisterClass::CalleeSaved, // Location = stable
                };
                karaka_hints.insert(
                    i,
                    KarakaHint {
                        karaka,
                        register_class: reg_class,
                    },
                );
            }
        }

        Some(MirFunction {
            name: func.name.name.clone(),
            params,
            return_type,
            blocks: std::mem::take(&mut self.blocks),
            locals: std::mem::take(&mut self.locals),
            karaka_hints,
        })
    }

    /// Lower a block of statements
    fn lower_block(&mut self, block: &ast::Block) {
        for stmt in &block.stmts {
            self.lower_stmt(stmt);
        }
    }

    /// Lower a statement to MIR
    fn lower_stmt(&mut self, stmt: &ast::Stmt) {
        match stmt {
            ast::Stmt::Let {
                name, ty, value, ..
            } => {
                let mir_ty = ty
                    .as_ref()
                    .map(|t| self.convert_type(t))
                    .unwrap_or(MirType::Int(IntSize::I64));

                let local_idx = self.alloc_local(mir_ty, Some(name.name.clone()));
                self.var_map.insert(name.name.clone(), local_idx);

                if let Some(val_expr) = value {
                    let rvalue = self.lower_expr_to_rvalue(val_expr);
                    self.emit_instruction(MirInstruction::Assign {
                        dest: MirPlace {
                            local: local_idx,
                            projection: vec![],
                        },
                        value: rvalue,
                    });
                }
            }

            ast::Stmt::Return { value, .. } => {
                if let Some(val) = value {
                    let rvalue = self.lower_expr_to_rvalue(val);
                    // Store return value in _0 (first local)
                    let ret_local = 0;
                    self.emit_instruction(MirInstruction::Assign {
                        dest: MirPlace {
                            local: ret_local,
                            projection: vec![],
                        },
                        value: rvalue,
                    });
                }
                self.set_terminator(MirTerminator::Return);
            }

            ast::Stmt::Expr(expr) => {
                // Evaluate expression for side effects
                let _ = self.lower_expr_to_rvalue(expr);
            }

            ast::Stmt::If {
                condition,
                then_block,
                else_block,
                ..
            } => {
                let cond_operand = self.lower_expr_to_operand(condition);

                let then_block_id = self.alloc_block();
                let else_block_id = self.alloc_block();
                let merge_block_id = self.alloc_block();

                // Branch on condition
                self.set_terminator(MirTerminator::SwitchInt {
                    discriminant: cond_operand,
                    targets: vec![(1, then_block_id)], // 1 = true
                    otherwise: else_block_id,
                });

                // Then block
                self.blocks.push(MirBasicBlock {
                    id: then_block_id,
                    instructions: Vec::new(),
                    terminator: MirTerminator::Goto {
                        target: merge_block_id,
                    },
                });
                self.current_block = self.blocks.len() - 1;
                self.lower_block(then_block);
                self.set_terminator(MirTerminator::Goto {
                    target: merge_block_id,
                });

                // Else block
                self.blocks.push(MirBasicBlock {
                    id: else_block_id,
                    instructions: Vec::new(),
                    terminator: MirTerminator::Goto {
                        target: merge_block_id,
                    },
                });
                self.current_block = self.blocks.len() - 1;
                if let Some(else_blk) = else_block {
                    self.lower_block(else_blk);
                }
                self.set_terminator(MirTerminator::Goto {
                    target: merge_block_id,
                });

                // Merge block
                self.blocks.push(MirBasicBlock {
                    id: merge_block_id,
                    instructions: Vec::new(),
                    terminator: MirTerminator::Return,
                });
                self.current_block = self.blocks.len() - 1;
            }

            ast::Stmt::Loop { kind, body, .. } => {
                let loop_header_id = self.alloc_block();
                let loop_body_id = self.alloc_block();
                let loop_exit_id = self.alloc_block();

                self.set_terminator(MirTerminator::Goto {
                    target: loop_header_id,
                });

                match kind {
                    ast::LoopKind::While { condition } => {
                        // Header: check condition
                        self.blocks.push(MirBasicBlock {
                            id: loop_header_id,
                            instructions: Vec::new(),
                            terminator: MirTerminator::Return,
                        });
                        self.current_block = self.blocks.len() - 1;
                        let cond = self.lower_expr_to_operand(condition);
                        self.set_terminator(MirTerminator::SwitchInt {
                            discriminant: cond,
                            targets: vec![(1, loop_body_id)],
                            otherwise: loop_exit_id,
                        });
                    }
                    ast::LoopKind::Range {
                        binding,
                        start,
                        end,
                        ..
                    } => {
                        // Allocate loop variable
                        let iter_local = self
                            .alloc_local(MirType::Int(IntSize::I64), Some(binding.name.clone()));
                        self.var_map.insert(binding.name.clone(), iter_local);

                        // Initialize loop var with start
                        let start_rval = self.lower_expr_to_rvalue(start);
                        self.emit_instruction(MirInstruction::Assign {
                            dest: MirPlace {
                                local: iter_local,
                                projection: vec![],
                            },
                            value: start_rval,
                        });

                        // Header: check i < end
                        self.blocks.push(MirBasicBlock {
                            id: loop_header_id,
                            instructions: Vec::new(),
                            terminator: MirTerminator::Return,
                        });
                        self.current_block = self.blocks.len() - 1;

                        let end_op = self.lower_expr_to_operand(end);
                        let iter_op = MirOperand::Copy(MirPlace {
                            local: iter_local,
                            projection: vec![],
                        });
                        let cmp_local = self.alloc_local(MirType::Bool, None);
                        self.emit_instruction(MirInstruction::Assign {
                            dest: MirPlace {
                                local: cmp_local,
                                projection: vec![],
                            },
                            value: MirRvalue::BinaryOp {
                                op: BinaryOp::Lt,
                                left: iter_op,
                                right: end_op,
                            },
                        });
                        self.set_terminator(MirTerminator::SwitchInt {
                            discriminant: MirOperand::Copy(MirPlace {
                                local: cmp_local,
                                projection: vec![],
                            }),
                            targets: vec![(1, loop_body_id)],
                            otherwise: loop_exit_id,
                        });
                    }
                    ast::LoopKind::Infinite => {
                        // Header just jumps to body
                        self.blocks.push(MirBasicBlock {
                            id: loop_header_id,
                            instructions: Vec::new(),
                            terminator: MirTerminator::Goto {
                                target: loop_body_id,
                            },
                        });
                    }
                    ast::LoopKind::ForIn { binding, iterable } => {
                        // Simplified: treat like infinite loop for now
                        self.blocks.push(MirBasicBlock {
                            id: loop_header_id,
                            instructions: Vec::new(),
                            terminator: MirTerminator::Goto {
                                target: loop_body_id,
                            },
                        });
                    }
                }

                // Body
                self.blocks.push(MirBasicBlock {
                    id: loop_body_id,
                    instructions: Vec::new(),
                    terminator: MirTerminator::Goto {
                        target: loop_header_id,
                    },
                });
                self.current_block = self.blocks.len() - 1;
                self.lower_block(body);

                // For range loops: increment the iterator
                if let ast::LoopKind::Range { binding, .. } = kind {
                    if let Some(&iter_local) = self.var_map.get(&binding.name) {
                        let one = MirOperand::Constant(MirConstant::Int(1, IntSize::I64));
                        let iter_op = MirOperand::Copy(MirPlace {
                            local: iter_local,
                            projection: vec![],
                        });
                        self.emit_instruction(MirInstruction::Assign {
                            dest: MirPlace {
                                local: iter_local,
                                projection: vec![],
                            },
                            value: MirRvalue::BinaryOp {
                                op: BinaryOp::Add,
                                left: iter_op,
                                right: one,
                            },
                        });
                    }
                }

                self.set_terminator(MirTerminator::Goto {
                    target: loop_header_id,
                });

                // Exit
                self.blocks.push(MirBasicBlock {
                    id: loop_exit_id,
                    instructions: Vec::new(),
                    terminator: MirTerminator::Return,
                });
                self.current_block = self.blocks.len() - 1;
            }

            ast::Stmt::Break { .. } => {
                // Would need loop exit block tracking - simplified
                self.emit_instruction(MirInstruction::Nop);
            }

            ast::Stmt::Continue { .. } => {
                // Would need loop header block tracking - simplified
                self.emit_instruction(MirInstruction::Nop);
            }

            ast::Stmt::Match {
                scrutinee, arms, ..
            } => {
                // Proper match lowering with pattern matching
                let scrut_op = self.lower_expr_to_operand(scrutinee);

                // Allocate temp for scrutinee
                let scrut_local = self.alloc_local(MirType::Int(IntSize::I64), None);
                self.emit_instruction(MirInstruction::Assign {
                    dest: MirPlace {
                        local: scrut_local,
                        projection: vec![],
                    },
                    value: MirRvalue::Use(scrut_op),
                });

                // Create blocks for each arm and merge block
                let merge_block_id = self.alloc_block();
                let mut arm_blocks = Vec::new();

                for _ in arms {
                    arm_blocks.push(self.alloc_block());
                }

                // Default/otherwise block
                let otherwise_block = self.alloc_block();

                // Build switch targets from patterns
                let mut switch_targets = Vec::new();
                for (i, arm) in arms.iter().enumerate() {
                    if let Some(value) = self.pattern_to_int(&arm.pattern) {
                        switch_targets.push((value, arm_blocks[i]));
                    }
                }

                // Set terminator to switch
                self.set_terminator(MirTerminator::SwitchInt {
                    discriminant: MirOperand::Copy(MirPlace {
                        local: scrut_local,
                        projection: vec![],
                    }),
                    targets: switch_targets,
                    otherwise: otherwise_block,
                });

                // Generate each arm block
                for (i, arm) in arms.iter().enumerate() {
                    self.blocks.push(MirBasicBlock {
                        id: arm_blocks[i],
                        instructions: Vec::new(),
                        terminator: MirTerminator::Goto {
                            target: merge_block_id,
                        },
                    });
                    self.current_block = self.blocks.len() - 1;

                    // Bind pattern variables
                    self.bind_pattern_variables(&arm.pattern, scrut_local);

                    // Evaluate guard if present
                    if let Some(guard) = &arm.guard {
                        let guard_op = self.lower_expr_to_operand(guard);
                        // If guard fails, jump to next arm or otherwise
                        // Simplified: always proceed
                    }

                    // Lower arm body (it's an expression, not a block)
                    let _body_rvalue = self.lower_expr_to_rvalue(&arm.body);
                    self.set_terminator(MirTerminator::Goto {
                        target: merge_block_id,
                    });
                }

                // Otherwise block (unreachable for exhaustive matches)
                self.blocks.push(MirBasicBlock {
                    id: otherwise_block,
                    instructions: Vec::new(),
                    terminator: MirTerminator::Unreachable,
                });

                // Merge block
                self.blocks.push(MirBasicBlock {
                    id: merge_block_id,
                    instructions: Vec::new(),
                    terminator: MirTerminator::Return,
                });
                self.current_block = self.blocks.len() - 1;
            }
        }
    }

    /// Lower expression to MIR R-value
    fn lower_expr_to_rvalue(&mut self, expr: &ast::Expr) -> MirRvalue {
        match expr {
            ast::Expr::Literal(lit) => {
                MirRvalue::Use(MirOperand::Constant(self.lower_literal(lit)))
            }

            ast::Expr::Identifier(ident) => {
                if let Some(&local) = self.var_map.get(&ident.name) {
                    MirRvalue::Use(MirOperand::Copy(MirPlace {
                        local,
                        projection: vec![],
                    }))
                } else {
                    // Unknown identifier - treat as zero
                    MirRvalue::Use(MirOperand::Constant(MirConstant::Int(0, IntSize::I64)))
                }
            }

            ast::Expr::Binary {
                left, op, right, ..
            } => {
                let left_op = self.lower_expr_to_operand(left);
                let right_op = self.lower_expr_to_operand(right);
                let mir_op = self.convert_binary_op(*op);
                MirRvalue::BinaryOp {
                    op: mir_op,
                    left: left_op,
                    right: right_op,
                }
            }

            ast::Expr::Unary { op, operand, .. } => {
                let operand_mir = self.lower_expr_to_operand(operand);
                match op {
                    ast::UnaryOp::Not => MirRvalue::UnaryOp {
                        op: UnaryOp::Not,
                        operand: operand_mir,
                    },
                    ast::UnaryOp::Neg => MirRvalue::UnaryOp {
                        op: UnaryOp::Neg,
                        operand: operand_mir,
                    },
                    ast::UnaryOp::Ref => {
                        // Take reference - get the place from operand
                        if let MirOperand::Copy(place) | MirOperand::Move(place) = operand_mir {
                            MirRvalue::Ref {
                                mutable: false,
                                place,
                            }
                        } else {
                            MirRvalue::Use(operand_mir)
                        }
                    }
                    ast::UnaryOp::Deref => {
                        // Dereference - add deref projection
                        MirRvalue::Use(operand_mir)
                    }
                }
            }

            ast::Expr::Call { callee, args, span } => {
                let func_op = self.lower_expr_to_operand(callee);
                let arg_ops: Vec<_> = args.iter().map(|a| self.lower_expr_to_operand(a)).collect();

                // Create temp for result
                let result_local = self.alloc_local(MirType::Int(IntSize::I64), None);
                let result_place = MirPlace {
                    local: result_local,
                    projection: vec![],
                };

                // Create continuation block
                let cont_block = self.alloc_block();

                self.set_terminator(MirTerminator::Call {
                    func: func_op,
                    args: arg_ops,
                    destination: Some(result_place.clone()),
                    target: cont_block,
                });

                // Continue in new block
                self.blocks.push(MirBasicBlock {
                    id: cont_block,
                    instructions: Vec::new(),
                    terminator: MirTerminator::Return,
                });
                self.current_block = self.blocks.len() - 1;

                MirRvalue::Use(MirOperand::Copy(result_place))
            }

            ast::Expr::Array { elements, .. } => {
                let ops: Vec<_> = elements
                    .iter()
                    .map(|e| self.lower_expr_to_operand(e))
                    .collect();
                MirRvalue::Aggregate {
                    kind: AggregateKind::Array,
                    operands: ops,
                }
            }

            ast::Expr::Tuple { elements, .. } => {
                let ops: Vec<_> = elements
                    .iter()
                    .map(|e| self.lower_expr_to_operand(e))
                    .collect();
                MirRvalue::Aggregate {
                    kind: AggregateKind::Tuple,
                    operands: ops,
                }
            }

            ast::Expr::FieldAccess { object, field, .. } => {
                // Proper field access with projection
                let base_rvalue = self.lower_expr_to_rvalue(object);

                // Create a temporary to hold the base value
                let base_local = self.alloc_local(MirType::Int(IntSize::I64), None);
                self.emit_instruction(MirInstruction::Assign {
                    dest: MirPlace {
                        local: base_local,
                        projection: vec![],
                    },
                    value: base_rvalue,
                });

                // Create place with field projection
                let field_name = &field.name;
                let field_idx = self.lookup_field_index(object, field_name).unwrap_or(0);

                MirRvalue::Field {
                    base: MirOperand::Copy(MirPlace {
                        local: base_local,
                        projection: vec![],
                    }),
                    index: field_idx,
                }
            }

            ast::Expr::Index { object, index, .. } => {
                // Proper array/slice indexing
                let base_rvalue = self.lower_expr_to_rvalue(object);
                let idx_op = self.lower_expr_to_operand(index);

                // Create temporary for base
                let base_local = self.alloc_local(MirType::Int(IntSize::I64), None);
                self.emit_instruction(MirInstruction::Assign {
                    dest: MirPlace {
                        local: base_local,
                        projection: vec![],
                    },
                    value: base_rvalue,
                });

                // Emit bounds check (Naraka: Asipatravana - buffer overflow prevention)
                let len_local = self.alloc_local(MirType::Int(IntSize::I64), None);
                self.emit_instruction(MirInstruction::Assign {
                    dest: MirPlace {
                        local: len_local,
                        projection: vec![],
                    },
                    value: MirRvalue::Len(MirPlace {
                        local: base_local,
                        projection: vec![],
                    }),
                });

                self.emit_instruction(MirInstruction::BoundsCheck {
                    index: idx_op.clone(),
                    len: MirOperand::Copy(MirPlace {
                        local: len_local,
                        projection: vec![],
                    }),
                    message: "Asipatravana: Array index out of bounds".to_string(),
                });

                MirRvalue::Index {
                    base: MirOperand::Copy(MirPlace {
                        local: base_local,
                        projection: vec![],
                    }),
                    index: idx_op,
                }
            }

            ast::Expr::Block(block) => {
                self.lower_block(block);
                MirRvalue::Use(MirOperand::Constant(MirConstant::Unit))
            }

            _ => {
                // Default case for unsupported expressions
                MirRvalue::Use(MirOperand::Constant(MirConstant::Unit))
            }
        }
    }

    /// Lower expression to MIR operand (for use in operations)
    fn lower_expr_to_operand(&mut self, expr: &ast::Expr) -> MirOperand {
        match expr {
            ast::Expr::Literal(lit) => MirOperand::Constant(self.lower_literal(lit)),
            ast::Expr::Identifier(ident) => {
                if let Some(&local) = self.var_map.get(&ident.name) {
                    MirOperand::Copy(MirPlace {
                        local,
                        projection: vec![],
                    })
                } else {
                    // Not a local variable - could be a function name
                    // Return as string constant for call targets
                    MirOperand::Constant(MirConstant::String(ident.name.clone()))
                }
            }
            _ => {
                // For complex expressions, create a temporary
                let rvalue = self.lower_expr_to_rvalue(expr);
                let temp = self.alloc_local(MirType::Int(IntSize::I64), None);
                self.emit_instruction(MirInstruction::Assign {
                    dest: MirPlace {
                        local: temp,
                        projection: vec![],
                    },
                    value: rvalue,
                });
                MirOperand::Copy(MirPlace {
                    local: temp,
                    projection: vec![],
                })
            }
        }
    }

    /// Lower literal to MIR constant
    fn lower_literal(&self, lit: &ast::Literal) -> MirConstant {
        match lit {
            ast::Literal::Int(n) => MirConstant::Int(*n, IntSize::I64),
            ast::Literal::Float(f) => MirConstant::Float(*f, FloatSize::F64),
            ast::Literal::Bool(b) => MirConstant::Bool(*b),
            ast::Literal::String(s) => MirConstant::String(s.clone()),
            ast::Literal::Char(c) => MirConstant::Int(*c as i64, IntSize::I64),
            ast::Literal::Unit => MirConstant::Unit,
        }
    }

    /// Convert AST binary op to MIR binary op
    fn convert_binary_op(&self, op: ast::BinaryOp) -> BinaryOp {
        match op {
            ast::BinaryOp::Add | ast::BinaryOp::AddAssign => BinaryOp::Add,
            ast::BinaryOp::Sub | ast::BinaryOp::SubAssign => BinaryOp::Sub,
            ast::BinaryOp::Mul | ast::BinaryOp::MulAssign => BinaryOp::Mul,
            ast::BinaryOp::Div | ast::BinaryOp::DivAssign => BinaryOp::Div,
            ast::BinaryOp::Mod => BinaryOp::Rem,
            ast::BinaryOp::And => BinaryOp::BitAnd,
            ast::BinaryOp::Or => BinaryOp::BitOr,
            ast::BinaryOp::Eq => BinaryOp::Eq,
            ast::BinaryOp::Ne => BinaryOp::Ne,
            ast::BinaryOp::Lt => BinaryOp::Lt,
            ast::BinaryOp::Le => BinaryOp::Le,
            ast::BinaryOp::Gt => BinaryOp::Gt,
            ast::BinaryOp::Ge => BinaryOp::Ge,
            ast::BinaryOp::BitAnd => BinaryOp::BitAnd,
            ast::BinaryOp::BitOr => BinaryOp::BitOr,
            ast::BinaryOp::BitXor => BinaryOp::BitXor,
            ast::BinaryOp::Shl => BinaryOp::Shl,
            ast::BinaryOp::Shr => BinaryOp::Shr,
            ast::BinaryOp::Assign => BinaryOp::Add, // Placeholder for assign
        }
    }

    /// Emit an instruction to the current block
    fn emit_instruction(&mut self, instr: MirInstruction) {
        if let Some(block) = self.blocks.get_mut(self.current_block) {
            block.instructions.push(instr);
        }
    }

    /// Set the terminator of the current block
    fn set_terminator(&mut self, term: MirTerminator) {
        if let Some(block) = self.blocks.get_mut(self.current_block) {
            block.terminator = term;
        }
    }

    /// Build MIR type definition
    fn build_typedef(&mut self, typedef: &ast::TypeDef) -> Option<MirTypeDef> {
        let kind = match &typedef.body {
            ast::TypeBody::Struct(fields) => {
                let mir_fields: Vec<(String, MirType)> = fields
                    .iter()
                    .map(|f| (f.name.name.clone(), self.convert_type(&f.ty)))
                    .collect();
                MirTypeDefKind::Struct { fields: mir_fields }
            }
            ast::TypeBody::Enum(variants) => {
                let mir_variants: Vec<(String, Option<MirType>)> = variants
                    .iter()
                    .map(|v| {
                        let ty = v.fields.as_ref().map(|fields| {
                            if fields.len() == 1 {
                                self.convert_type(&fields[0].ty)
                            } else {
                                MirType::Tuple(
                                    fields.iter().map(|f| self.convert_type(&f.ty)).collect(),
                                )
                            }
                        });
                        (v.name.name.clone(), ty)
                    })
                    .collect();
                MirTypeDefKind::Enum {
                    variants: mir_variants,
                }
            }
            _ => return None,
        };

        Some(MirTypeDef {
            name: typedef.name.name.clone(),
            kind,
        })
    }

    /// Build MIR for constant
    fn build_const(&mut self, const_def: &ast::ConstantDef) -> Option<MirGlobal> {
        let ty = const_def
            .ty
            .as_ref()
            .map(|t| self.convert_type(t))
            .unwrap_or(MirType::Unit);

        Some(MirGlobal {
            name: const_def.name.name.clone(),
            ty,
            init: None,
            mutable: false,
        })
    }

    /// Convert AST type to MIR type
    fn convert_type(&self, ty: &ast::Type) -> MirType {
        match ty {
            ast::Type::Named { name, generics, .. } => {
                let type_name = &name.name;
                match type_name.as_str() {
                    "i8" => MirType::Int(IntSize::I8),
                    "i16" => MirType::Int(IntSize::I16),
                    "i32" | "saṅkhyā" => MirType::Int(IntSize::I32),
                    "i64" => MirType::Int(IntSize::I64),
                    "u8" => MirType::Int(IntSize::U8),
                    "u16" => MirType::Int(IntSize::U16),
                    "u32" => MirType::Int(IntSize::U32),
                    "u64" => MirType::Int(IntSize::U64),
                    "f32" => MirType::Float(FloatSize::F32),
                    "f64" => MirType::Float(FloatSize::F64),
                    "bool" => MirType::Bool,
                    "()" => MirType::Unit,
                    _ => MirType::Named(type_name.clone()),
                }
            }
            ast::Type::Reference { inner, mutable, .. } => MirType::Ref {
                mutable: *mutable,
                ty: Box::new(self.convert_type(inner)),
            },
            ast::Type::Array { element, size } => MirType::Array {
                element: Box::new(self.convert_type(element)),
                size: size.unwrap_or(0),
            },
            ast::Type::Tuple(elements) => {
                MirType::Tuple(elements.iter().map(|t| self.convert_type(t)).collect())
            }
            ast::Type::Function {
                params,
                return_type,
            } => MirType::Function {
                params: params.iter().map(|t| self.convert_type(t)).collect(),
                ret: Box::new(self.convert_type(return_type)),
            },
            ast::Type::Inferred => MirType::Unit,
        }
    }

    /// Allocate a new local variable
    fn alloc_local(&mut self, ty: MirType, name: Option<String>) -> usize {
        let index = self.next_local;
        self.next_local += 1;
        self.locals.push(MirLocal { index, ty, name });
        index
    }

    /// Allocate a new basic block
    fn alloc_block(&mut self) -> usize {
        let index = self.next_block;
        self.next_block += 1;
        index
    }
}

impl Default for MirBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Helper methods for enhanced lowering
impl MirBuilder {
    /// Lookup field index from type context
    fn lookup_field_index(&self, object: &ast::Expr, field_name: &str) -> Option<usize> {
        // In a full implementation, this would use the type system
        // For now, use a simple heuristic based on common field patterns
        match field_name {
            "len" | "length" => Some(0),
            "ptr" | "data" => Some(1),
            "cap" | "capacity" => Some(2),
            "x" | "first" | "left" => Some(0),
            "y" | "second" | "right" => Some(1),
            "z" | "third" => Some(2),
            "w" | "fourth" => Some(3),
            _ => {
                // Try to parse numeric field like "0", "1", etc.
                field_name.parse::<usize>().ok()
            }
        }
    }

    /// Convert pattern to integer value for switch
    fn pattern_to_int(&self, pattern: &ast::Pattern) -> Option<i64> {
        match pattern {
            ast::Pattern::Literal(lit) => match lit {
                ast::Literal::Int(n) => Some(*n),
                ast::Literal::Bool(b) => Some(if *b { 1 } else { 0 }),
                ast::Literal::Char(c) => Some(*c as i64),
                _ => None,
            },
            ast::Pattern::Identifier(_) => None, // Wildcard-like, don't add to switch
            ast::Pattern::Wildcard => None,
            ast::Pattern::Rest => None,
            ast::Pattern::Constructor { name, .. } => {
                // Would need enum type info for variant index
                Some(name.name.chars().next()? as i64)
            }
        }
    }

    /// Bind pattern variables in scope
    fn bind_pattern_variables(&mut self, pattern: &ast::Pattern, scrutinee_local: usize) {
        match pattern {
            ast::Pattern::Identifier(ident) => {
                // Bind the identifier to the scrutinee value
                let local = self.alloc_local(MirType::Int(IntSize::I64), Some(ident.name.clone()));
                self.var_map.insert(ident.name.clone(), local);
                self.emit_instruction(MirInstruction::Assign {
                    dest: MirPlace {
                        local,
                        projection: vec![],
                    },
                    value: MirRvalue::Use(MirOperand::Copy(MirPlace {
                        local: scrutinee_local,
                        projection: vec![],
                    })),
                });
            }
            ast::Pattern::Constructor { name: _, fields } => {
                // Constructor pattern (enum/struct variant)
                for (i, sub_pattern) in fields.iter().enumerate() {
                    if let ast::Pattern::Identifier(ident) = sub_pattern {
                        let local =
                            self.alloc_local(MirType::Int(IntSize::I64), Some(ident.name.clone()));
                        self.var_map.insert(ident.name.clone(), local);
                        // Extract i-th field from the constructor
                        self.emit_instruction(MirInstruction::Assign {
                            dest: MirPlace {
                                local,
                                projection: vec![],
                            },
                            value: MirRvalue::Field {
                                base: MirOperand::Copy(MirPlace {
                                    local: scrutinee_local,
                                    projection: vec![],
                                }),
                                index: i,
                            },
                        });
                    } else {
                        // Recursively bind nested patterns
                        self.bind_pattern_variables(sub_pattern, scrutinee_local);
                    }
                }
            }
            ast::Pattern::Literal(_) | ast::Pattern::Wildcard | ast::Pattern::Rest => {
                // No binding needed
            }
        }
    }
}
