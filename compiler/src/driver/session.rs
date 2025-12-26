//! Compiler Session

use super::{CompileError, CompileResult, CompileTiming, CompilerOptions};
use crate::codegen::asm::AsmEmitter;
use crate::philosophy::kala::Kala;
use crate::philosophy::samkhya::{SamkhyaPipeline, Tattva};
use std::time::{Duration, Instant};

/// Compiler session state
pub struct CompilerSession {
    /// Options
    options: CompilerOptions,
    /// Sāṃkhya pipeline
    pipeline: SamkhyaPipeline,
    /// Kāla time tracker
    kala: Kala,
    /// Timing information
    timing: CompileTiming,
}

impl CompilerSession {
    pub fn new(options: CompilerOptions) -> Self {
        let time_budget = options
            .time_budget_ms
            .map(Duration::from_millis)
            .unwrap_or(Duration::from_secs(60));

        Self {
            options,
            pipeline: SamkhyaPipeline::new(),
            kala: Kala::new(time_budget),
            timing: CompileTiming::default(),
        }
    }

    /// Compile source code
    pub fn compile(&mut self, source: &str) -> Result<CompileResult, CompileError> {
        self.kala.start();
        let start = Instant::now();

        // Stage 1: Lexing (Śrotra - hearing)
        let lexing_timer = self.kala.begin_phase("lexing");
        let tokens = self.lex(source)?;
        self.kala.end_phase(lexing_timer);

        // Stage 2: Parsing (Tvak - touch)
        let parsing_timer = self.kala.begin_phase("parsing");
        let ast = self.parse(&tokens)?;
        self.kala.end_phase(parsing_timer);

        // Stage 3: Type Checking (Rasana - taste)
        let typeck_timer = self.kala.begin_phase("type_checking");
        self.type_check(&ast)?;
        self.kala.end_phase(typeck_timer);

        // Stage 4: MIR Building
        let mir_timer = self.kala.begin_phase("mir_building");
        let mir = self.build_mir(&ast)?;
        self.kala.end_phase(mir_timer);

        // Stage 5: Optimization
        let opt_timer = self.kala.begin_phase("optimization");
        let optimized_mir = self.optimize(mir)?;
        self.kala.end_phase(opt_timer);

        // Stage 6: Code Generation
        let codegen_timer = self.kala.begin_phase("codegen");
        let output = self.generate_code(&optimized_mir)?;
        self.kala.end_phase(codegen_timer);

        self.timing.total_us = start.elapsed().as_micros() as u64;

        if self.options.verbose {
            eprintln!("{}", self.kala.timing_report());
        }

        Ok(CompileResult {
            output,
            warnings: Vec::new(),
            timing: std::mem::take(&mut self.timing),
        })
    }

    fn lex(&mut self, source: &str) -> Result<Vec<crate::lexer::Token>, CompileError> {
        let start = Instant::now();

        let mut lexer = crate::lexer::Lexer::new(source);
        let tokens = lexer.tokenize();

        self.timing.lexing_us = start.elapsed().as_micros() as u64;
        Ok(tokens)
    }

    fn parse(
        &mut self,
        tokens: &[crate::lexer::Token],
    ) -> Result<crate::parser::ast::Ast, CompileError> {
        let start = Instant::now();

        let mut parser = crate::parser::Parser::new(tokens.to_vec());
        let ast = parser.parse().map_err(|errors| {
            let mut msg = String::from("Parse errors:");
            for e in errors {
                msg.push_str(&format!("\n  - {} at {:?}", e.message, e.span));
            }
            CompileError {
                message: msg,
                location: None,
                notes: Vec::new(),
            }
        })?;

        self.timing.parsing_us = start.elapsed().as_micros() as u64;
        Ok(ast)
    }

    fn type_check(&mut self, ast: &crate::parser::ast::Ast) -> Result<(), CompileError> {
        let start = Instant::now();

        let mut typeck = crate::semantics::TypeChecker::new();
        // TODO: Implement type checking

        self.timing.type_checking_us = start.elapsed().as_micros() as u64;
        Ok(())
    }

    fn build_mir(
        &mut self,
        ast: &crate::parser::ast::Ast,
    ) -> Result<crate::mir::types::MirModule, CompileError> {
        let start = Instant::now();

        let mut builder = crate::mir::MirBuilder::new();
        let mir = builder.build(ast);

        self.timing.mir_building_us = start.elapsed().as_micros() as u64;
        Ok(mir)
    }

    fn optimize(
        &mut self,
        mut mir: crate::mir::types::MirModule,
    ) -> Result<crate::mir::types::MirModule, CompileError> {
        let start = Instant::now();

        let opt_level = match self.options.opt_level {
            0 => crate::mir::optimizer::OptLevel::None,
            1 => crate::mir::optimizer::OptLevel::Basic,
            2 => crate::mir::optimizer::OptLevel::Standard,
            _ => crate::mir::optimizer::OptLevel::Aggressive,
        };

        let guna_mode = match self.options.guna {
            crate::philosophy::guna::Guna::Sattva => crate::mir::optimizer::GunaMode::Sattva,
            crate::philosophy::guna::Guna::Rajas => crate::mir::optimizer::GunaMode::Rajas,
            crate::philosophy::guna::Guna::Tamas => crate::mir::optimizer::GunaMode::Tamas,
        };

        let mut optimizer = crate::mir::MirOptimizer::new(opt_level, guna_mode);
        optimizer.optimize(&mut mir);

        self.timing.optimization_us = start.elapsed().as_micros() as u64;
        Ok(mir)
    }

    fn generate_code(
        &mut self,
        mir: &crate::mir::types::MirModule,
    ) -> Result<Vec<u8>, CompileError> {
        let start = Instant::now();

        // Use x86-64 emitter to generate assembly
        let mut emitter = crate::codegen::asm::x86_64::X86_64Emitter::new();

        for func in &mir.functions {
            emitter.emit_prologue(func);
            emitter.emit_body(func);
            emitter.emit_epilogue(func);
        }

        // Get the generated assembly
        let asm = emitter.get_asm();
        let output = asm.into_bytes();

        self.timing.codegen_us = start.elapsed().as_micros() as u64;
        Ok(output)
    }
}
