//! Compiler Session
//!
//! Saṃkalana Satra (Compilation Session) - orchestrates the complete
//! compilation pipeline from source to executable.

use super::{CompileError, CompileResult, CompileTiming, CompilerOptions};
use crate::codegen::asm::AsmEmitter;
use crate::codegen::linker::BuildPipeline;
use crate::philosophy::kala::Kala;
use crate::philosophy::samkhya::SamkhyaPipeline;
use std::path::PathBuf;
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

        // Stage 3.5: Security Analysis via Nava Durga (9 Goddess Layers)
        // "Sarva Maṅgala Māṅgalye Śive Sarvārtha Sādhike"
        // - May the auspicious one protect all our code
        if self.options.security_check {
            let security_timer = self.kala.begin_phase("security");
            self.security_check(source)?;
            self.kala.end_phase(security_timer);
        }

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

    /// Type checking via Nyāya 4-pramāṇa inference (Prakāra Parīkṣā)
    ///
    /// Like Yama examining the soul's karma before judgment,
    /// this phase examines each expression's type before compilation proceeds.
    /// Uses the 4 pramāṇas (valid means of knowledge) for type inference:
    /// - Pratyakṣa (100%): Explicit type annotation
    /// - Anumāna (95%): Logical deduction
    /// - Śabda (90%): Testimony from function signatures
    /// - Upamāna (85%): Pattern matching by analogy
    fn type_check(&mut self, ast: &crate::parser::ast::Ast) -> Result<(), CompileError> {
        let start = Instant::now();

        let mut typeck = crate::semantics::TypeChecker::new();

        // Perform Nyāya-based type checking
        typeck.check(ast).map_err(|errors| {
            let mut msg = String::from("Type errors (Prakāra Doṣa):");
            for error in &errors {
                msg.push_str(&format!("\n  ॥ {} ॥", error));
            }
            if errors.len() > 1 {
                msg.push_str(&format!(
                    "\n\n  Total: {} errors detected by Yama's judgment",
                    errors.len()
                ));
            }
            CompileError {
                message: msg,
                location: errors.first().and_then(|e| e.span()).map(|span| {
                    crate::driver::SourceLocation {
                        file: String::new(),
                        line: span.start,
                        column: 0,
                    }
                }),
                notes: vec![
                    "Type inference follows Nyāya epistemology".to_string(),
                    "Add explicit type annotations for Pratyakṣa (100% certainty)".to_string(),
                ],
            }
        })?;

        self.timing.type_checking_us = start.elapsed().as_micros() as u64;
        Ok(())
    }

    /// Security analysis via Nava Durga (9 Goddess Protection Layers)
    ///
    /// "सर्वमङ्गलमाङ्गल्ये शिवे सर्वार्थसाधिके ।
    ///  शरण्ये त्र्यम्बके गौरि नारायणि नमोऽस्तु ते ॥"
    /// - May the auspicious Goddess protect this code
    ///
    /// The 9 Durgas progressively harden the code:
    /// 1. Śailaputrī - Hardware security foundation
    /// 2. Brahmacāriṇī - Authentication checks
    /// 3. Candraghaṇṭā - Encryption verification
    /// 4. Kūṣmāṇḍā - Access control analysis
    /// 5. Skandamātā - Process isolation checks
    /// 6. Kātyāyanī - Input validation
    /// 7. Kālarātrī - Intrusion detection
    /// 8. Mahāgaurī - Audit logging
    /// 9. Siddhidātrī - Formal verification (perfection)
    fn security_check(&mut self, source: &str) -> Result<(), CompileError> {
        use crate::nava_durga::{NavaDurgaDefense, SecurityContext, SecurityResult};

        // Create security context with source
        let mut context = SecurityContext::new(source);

        // Trust level based on deterministic build flag
        // Deterministic builds are more trusted as they're reproducible
        let trust_level = if self.options.deterministic { 0.7 } else { 0.3 };
        context = context.with_trust(trust_level);

        // Deploy the 9 Durga defense layers
        let defense = NavaDurgaDefense::new();
        let result = defense.protect(&mut context);

        match result {
            SecurityResult::Perfect {
                layers_passed,
                trust_level,
            } => {
                if self.options.verbose {
                    eprintln!(
                        "🛡️  Nava Durga Protection Complete: {} layers passed (trust: {:.0}%)",
                        layers_passed,
                        trust_level * 100.0
                    );
                    eprintln!("   ॐ सिद्धिदात्र्यै नमः - Siddhidatri grants perfection");
                }
                Ok(())
            }
            SecurityResult::Blocked {
                layer,
                goddess,
                reason,
            } => Err(CompileError {
                message: format!(
                    "Security blocked at layer {} ({}):\n  {}",
                    layer, goddess, reason
                ),
                location: None,
                notes: vec![
                    format!("Goddess {} protects against this vulnerability", goddess),
                    "Review security annotations and trust boundaries".to_string(),
                    "Use --no-security to bypass (not recommended)".to_string(),
                ],
            }),
        }
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

    /// Optimization via Divine Astras (Divya Astra Anukūlana)
    ///
    /// Like Arjuna deploying divine weapons on the battlefield of Kurukshetra,
    /// this phase deploys optimization astras against inefficient code.
    ///
    /// Weapon deployment order follows the Mahābhārata hierarchy:
    /// 1. Analysis weapons (Nāgāstra, Varuṇāstra, Vāyavāstra)
    /// 2. Transformation weapons (Agneyāstra, Garuḍāstra)
    /// 3. Iterative refinement (Sudarśana Cakra)
    /// 4. Final cleanup (Brahmāstra)
    /// 5. Preservation (Nārāyaṇāstra)
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

        // Standard MIR optimization
        let mut optimizer = crate::mir::MirOptimizer::new(opt_level, guna_mode);
        optimizer.optimize(&mut mir);

        // Deploy Divine Astras for aggressive optimization (level 3+)
        // "Om Brahmāstrāya Phaṭ" - May the divine weapons destroy inefficiency
        if self.options.opt_level >= 3 {
            use crate::astras::{AstraArsenal, AstraResult};

            let arsenal = AstraArsenal::new();
            let results = arsenal.deploy_all(&mut mir);

            if self.options.verbose {
                eprintln!("⚔️  Divine Astra deployment results:");
                for (i, result) in results.iter().enumerate() {
                    match result {
                        AstraResult::Deployed {
                            power_level,
                            transformations,
                            mantra: _mantra,
                        } => {
                            eprintln!(
                                "    {}. 🏹 {} transformations (power {})",
                                i + 1,
                                transformations,
                                power_level
                            );
                        }
                        AstraResult::NoTargets => {
                            eprintln!("    {}. ○ No targets found", i + 1);
                        }
                        AstraResult::Failed { reason } => {
                            eprintln!("    {}. ✗ Failed: {}", i + 1, reason);
                        }
                    }
                }
            }
        }

        self.timing.optimization_us = start.elapsed().as_micros() as u64;
        Ok(mir)
    }

    fn generate_code(
        &mut self,
        mir: &crate::mir::types::MirModule,
    ) -> Result<Vec<u8>, CompileError> {
        let start = Instant::now();

        // Select emitter based on target architecture
        let asm = match self.options.target {
            crate::codegen::asm::Target::X86_64 => {
                let mut emitter = crate::codegen::asm::x86_64::X86_64Emitter::new();
                for func in &mir.functions {
                    emitter.emit_prologue(func);
                    emitter.emit_body(func);
                    emitter.emit_epilogue(func);
                }
                emitter.get_asm()
            }
            crate::codegen::asm::Target::AArch64 => {
                let mut emitter = crate::codegen::asm::aarch64::AArch64Emitter::new();
                for func in &mir.functions {
                    emitter.emit_prologue(func);
                    emitter.emit_body(func);
                    emitter.emit_epilogue(func);
                }
                emitter.get_asm()
            }
            crate::codegen::asm::Target::RiscV64 => {
                let mut emitter = crate::codegen::asm::riscv64::RiscV64Emitter::new();
                for func in &mir.functions {
                    emitter.emit_prologue(func);
                    emitter.emit_body(func);
                    emitter.emit_epilogue(func);
                }
                emitter.get_asm()
            }
        };

        let output = asm.into_bytes();

        self.timing.codegen_us = start.elapsed().as_micros() as u64;
        Ok(output)
    }

    /// Assemble and link to produce executable
    ///
    /// Kriyā (Action) - The final manifestation stage where assembly
    /// becomes executable through the BuildPipeline.
    fn assemble_and_link(&mut self, asm_output: &[u8]) -> Result<Vec<u8>, CompileError> {
        let _start = Instant::now();

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
