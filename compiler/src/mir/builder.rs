//! MIR Builder
//!
//! Converts AST to MIR.

use super::types::*;
use crate::parser::ast;

/// MIR Builder
pub struct MirBuilder {
    /// Current function being built
    current_function: Option<MirFunction>,
    /// Current block index
    current_block: usize,
    /// Next local index
    next_local: usize,
    /// Next block index
    next_block: usize,
}

impl MirBuilder {
    pub fn new() -> Self {
        Self {
            current_function: None,
            current_block: 0,
            next_local: 0,
            next_block: 0,
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
                ast::Item::Const(const_def) => {
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
        self.next_local = 0;
        self.next_block = 0;

        let params: Vec<MirParam> = func
            .params
            .iter()
            .enumerate()
            .map(|(i, p)| MirParam {
                index: i,
                ty: self.convert_type(&p.ty),
                karaka: p.karaka,
            })
            .collect();

        let return_type = func
            .return_type
            .as_ref()
            .map(|t| self.convert_type(t))
            .unwrap_or(MirType::Unit);

        // Create entry block
        let entry_block = MirBasicBlock {
            id: self.alloc_block(),
            instructions: Vec::new(),
            terminator: MirTerminator::Return,
        };

        let mut mir_func = MirFunction {
            name: func.name.name.clone(),
            params,
            return_type,
            blocks: vec![entry_block],
            locals: Vec::new(),
            karaka_hints: std::collections::HashMap::new(),
        };

        // TODO: Lower function body to MIR

        Some(mir_func)
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
                        (
                            v.name.name.clone(),
                            v.ty.as_ref().map(|t| self.convert_type(t)),
                        )
                    })
                    .collect();
                MirTypeDefKind::Enum { variants: mir_variants }
            }
            _ => return None,
        };

        Some(MirTypeDef {
            name: typedef.name.name.clone(),
            kind,
        })
    }

    /// Build MIR for constant
    fn build_const(&mut self, const_def: &ast::ConstDef) -> Option<MirGlobal> {
        Some(MirGlobal {
            name: const_def.name.name.clone(),
            ty: self.convert_type(&const_def.ty),
            init: None, // TODO: evaluate constant expression
            mutable: false,
        })
    }

    /// Convert AST type to MIR type
    fn convert_type(&self, ty: &ast::Type) -> MirType {
        match ty {
            ast::Type::Named { path, generics } => {
                let name = &path.segments.last().unwrap().name;
                match name.as_str() {
                    "i8" => MirType::Int(IntSize::I8),
                    "i16" => MirType::Int(IntSize::I16),
                    "i32" => MirType::Int(IntSize::I32),
                    "i64" => MirType::Int(IntSize::I64),
                    "u8" => MirType::Int(IntSize::U8),
                    "u16" => MirType::Int(IntSize::U16),
                    "u32" => MirType::Int(IntSize::U32),
                    "u64" => MirType::Int(IntSize::U64),
                    "f32" => MirType::Float(FloatSize::F32),
                    "f64" => MirType::Float(FloatSize::F64),
                    "bool" => MirType::Bool,
                    "()" => MirType::Unit,
                    _ => MirType::Named(name.clone()),
                }
            }
            ast::Type::Reference { mutable, ty } => MirType::Ref {
                mutable: *mutable,
                ty: Box::new(self.convert_type(ty)),
            },
            ast::Type::Pointer { mutable, ty } => MirType::Ptr(Box::new(self.convert_type(ty))),
            ast::Type::Array { element, size } => MirType::Array {
                element: Box::new(self.convert_type(element)),
                size: *size,
            },
            ast::Type::Slice { element } => MirType::Slice(Box::new(self.convert_type(element))),
            ast::Type::Tuple(elements) => {
                MirType::Tuple(elements.iter().map(|t| self.convert_type(t)).collect())
            }
            ast::Type::Function { params, return_type } => MirType::Function {
                params: params.iter().map(|t| self.convert_type(t)).collect(),
                ret: Box::new(self.convert_type(return_type)),
            },
            ast::Type::Infer => MirType::Unit, // Should be resolved before MIR
        }
    }

    /// Allocate a new local variable
    fn alloc_local(&mut self, ty: MirType, name: Option<String>) -> usize {
        let index = self.next_local;
        self.next_local += 1;
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
