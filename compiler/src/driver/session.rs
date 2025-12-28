//! Compiler Session
//!
//! Saṃkalana Satra (Compilation Session) - orchestrates the complete
//! compilation pipeline from source to executable.

use super::{CompileError, CompileResult, CompileTiming, CompilerOptions};
use crate::codegen::asm::AsmEmitter;
use crate::codegen::linker::{BuildPipeline, LinkOutput};
use crate::philosophy::kala::Kala;
use crate::philosophy::samkhya::{SamkhyaPipeline, Tattva};
use std::path::{Path, PathBuf};
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
    /// Input file path (for deriving output path)
    input_path: Option<PathBuf>,
}

impl CompilerSession {
    pub fn new(options: CompilerOptions) -> Self {
        let time_budget = options
            .time_budget_ms
            .map(Duration::from_millis)
            .unwrap_or(Duration::from_secs(60));

        // Extract input path if available
        let input_path = options.inputs.first().map(|s| PathBuf::from(s));

        Self {
            options,
            pipeline: SamkhyaPipeline::new(),
            kala: Kala::new(time_budget),
            timing: CompileTiming::default(),
            input_path,
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
        let asm_output = self.generate_code(&optimized_mir)?;
        self.kala.end_phase(codegen_timer);

        // Stage 7: Assembly & Linking (Kriyā - action)
        // If emit_asm is set, just write the assembly file
        let output = if self.options.emit_asm {
            self.emit_assembly_only(&asm_output)?
        } else {
            let linking_timer = self.kala.begin_phase("linking");
            let result = self.assemble_and_link(&asm_output)?;
            self.kala.end_phase(linking_timer);
            result
        };

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

    /// Assemble and link to produce executable
    ///
    /// Kriyā (Action) - The final manifestation stage where assembly
    /// becomes executable through the BuildPipeline.
    fn assemble_and_link(&mut self, asm_output: &[u8]) -> Result<Vec<u8>, CompileError> {
        let start = Instant::now();

        // Create build directory
        let build_dir = std::env::temp_dir().join("jagannath_build");
        std::fs::create_dir_all(&build_dir).map_err(|e| CompileError {
            message: format!("Failed to create build directory: {}", e),
            location: None,
            notes: Vec::new(),
        })?;

        // Determine output path
        let exe_name = if let Some(ref out) = self.options.output {
            PathBuf::from(out)
        } else if let Some(ref input) = self.input_path {
            // Derive from input: foo.jag -> foo (or foo.exe on Windows)
            let stem = input.file_stem().unwrap_or_default();
            let mut exe_path = PathBuf::from(stem);
            if cfg!(windows) {
                exe_path.set_extension("exe");
            }
            exe_path
        } else {
            // Default output name
            let mut exe_path = PathBuf::from("a.out");
            if cfg!(windows) {
                exe_path.set_extension("exe");
            }
            exe_path
        };

        // Write assembly to temp file
        let asm_path = build_dir.join("output.s");
        std::fs::write(&asm_path, asm_output).map_err(|e| CompileError {
            message: format!("Failed to write assembly: {}", e),
            location: None,
            notes: Vec::new(),
        })?;

        if self.options.verbose {
            eprintln!("🔧 Assembly written to: {}", asm_path.display());
        }

        // Use BuildPipeline to assemble and link
        let pipeline = BuildPipeline::new();
        pipeline
            .build_executable(&asm_path, &exe_name)
            .map_err(|e| CompileError {
                message: format!("Build failed: {}", e),
                location: None,
                notes: vec![
                    "Ensure GCC or Clang is installed and in PATH".to_string(),
                    "On Windows, install MinGW-w64 or WSL".to_string(),
                ],
            })?;

        if self.options.verbose {
            eprintln!("✨ Executable created: {}", exe_name.display());
        }

        // Read the executable back as bytes for CompileResult
        let exe_bytes = std::fs::read(&exe_name).map_err(|e| CompileError {
            message: format!("Failed to read executable: {}", e),
            location: None,
            notes: Vec::new(),
        })?;

        // Cleanup temp assembly file
        let _ = std::fs::remove_file(&asm_path);

        Ok(exe_bytes)
    }

    /// Emit assembly file only (no linking)
    ///
    /// Vāk (Speech) - The assembly is the linguistic expression of the program,
    /// written to file for inspection or external assembly.
    fn emit_assembly_only(&self, asm_output: &[u8]) -> Result<Vec<u8>, CompileError> {
        // Determine output path
        let asm_name = if let Some(ref out) = self.options.output {
            PathBuf::from(out)
        } else if let Some(ref input) = self.input_path {
            // Derive from input: foo.jag -> foo.s
            let mut asm_path = input.clone();
            asm_path.set_extension("s");
            asm_path
        } else {
            PathBuf::from("a.s")
        };

        // Write assembly to file
        std::fs::write(&asm_name, asm_output).map_err(|e| CompileError {
            message: format!("Failed to write assembly: {}", e),
            location: None,
            notes: Vec::new(),
        })?;

        if self.options.verbose {
            eprintln!("📝 Assembly written to: {}", asm_name.display());
        }

        // Return the assembly as bytes
        Ok(asm_output.to_vec())
    }
}
