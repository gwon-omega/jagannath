# Sāṃkhya Compilation Pipeline
## The 25 Tattvas as Compilation Stages

> *"पुरुषः प्रकृतिश्चैव सर्गप्रलयकारणम्"*
> — Sāṃkhyakārikā 21

---

## Overview

The Sāṃkhya school describes reality through 25 *tattvas* (principles) emanating from pure consciousness to gross matter. Jagannath maps this hierarchy to compilation stages:

```
CONSCIOUSNESS (Abstract)
    │
    ▼
┌───────────────────────────────────────┐
│ 25. Puruṣa    → Source Intent          │ ← You are here (writing code)
│ 24. Prakṛti   → AST Potential          │
├───────────────────────────────────────┤
│ 23. Buddhi    → Semantic Analysis      │ ← Understanding
│ 22. Ahaṃkāra  → Symbol Table           │ ← Identity
│ 21. Manas     → IR Generation          │ ← Processing
├───────────────────────────────────────┤
│ 20-16. Jñānendriyas → Type Checking    │ ← Input Faculties
│ 15-11. Karmendriyas → Code Transforms  │ ← Output Faculties
├───────────────────────────────────────┤
│ 10-6. Tanmātras → Optimization Passes  │ ← Subtle Elements
├───────────────────────────────────────┤
│ 5-1. Mahābhūtas → Machine Code         │ ← Gross Elements
└───────────────────────────────────────┘
    │
    ▼
MATTER (Concrete - Executable Binary)
```

---

## Stage Details

### Level 25: Puruṣa (पुरुष) — Source Intent

The pure consciousness behind the code — your intent as a programmer.

```sanskrit
# Your intent: "Process a list efficiently"
# This intent precedes all code
```

**Compiler Role**: Not directly compiled, but guides optimization choices.

---

### Level 24: Prakṛti (प्रकृति) — AST Potential

Undifferentiated source text, pure potential awaiting manifestation.

```rust
// compiler/samkhya/prakriti.rs
pub struct Prakriti {
    pub source: String,
    pub potential: Vec<Token>,  // Unstructured tokens
}

impl Prakriti {
    /// Manifest into Buddhi (AST)
    pub fn manifest(&self) -> Buddhi {
        let lexer = Lexer::new(&self.source);
        let parser = Parser::new(lexer);
        Buddhi {
            ast: parser.parse(),
            source_map: self.create_source_map(),
        }
    }
}
```

---

### Level 23: Buddhi (बुद्धि) — Semantic Analysis

Intellect/discrimination — the AST with semantic understanding.

```rust
// compiler/samkhya/buddhi.rs
pub struct Buddhi {
    pub ast: Ast,
    pub types: TypeTable,
    pub resolutions: NameResolution,
}

impl Buddhi {
    /// Discriminate: Resolve names, infer types
    pub fn discriminate(&mut self, ctx: &Context) -> Result<(), SemanticError> {
        // Nyāya inference happens here
        self.resolve_names(ctx)?;
        self.infer_types(ctx)?;
        self.check_borrowing(ctx)?;
        Ok(())
    }
}
```

---

### Level 22: Ahaṃkāra (अहंकार) — Symbol Table

Ego/identity — where symbols gain unique identity.

```rust
// compiler/samkhya/ahamkara.rs
pub struct Ahamkara {
    /// Symbol → Identity mapping
    pub symbols: HashMap<SymbolId, SymbolInfo>,
    /// Scope hierarchy
    pub scopes: Vec<Scope>,
}

impl Ahamkara {
    /// Give identity to a symbol
    pub fn identify(&mut self, name: &str, kind: SymbolKind) -> SymbolId {
        let id = SymbolId::new();
        self.symbols.insert(id, SymbolInfo {
            name: name.to_string(),
            kind,
            scope: self.current_scope(),
            mangled_name: self.mangle(name),
        });
        id
    }
}
```

---

### Level 21: Manas (मनस्) — IR Generation

Mind/processing — transforms AST to intermediate representation.

```rust
// compiler/samkhya/manas.rs
pub struct Manas {
    pub mir: MirProgram,
    pub basic_blocks: Vec<BasicBlock>,
}

impl Manas {
    /// Process AST into MIR
    pub fn process(&mut self, buddhi: &Buddhi) -> MirProgram {
        for item in &buddhi.ast.items {
            match item {
                Item::Function(f) => self.lower_function(f),
                Item::Struct(s) => self.lower_struct(s),
                // ...
            }
        }
        self.mir.clone()
    }
}
```

---

### Levels 20-16: Jñānendriyas (ज्ञानेन्द्रिय) — Input Faculties

Five sense organs — type checking and validation.

| Level | Tattva | Sanskrit | Compiler Function |
|-------|--------|----------|-------------------|
| 20 | Śrotra | श्रोत्र (Ear) | Lexical validation |
| 19 | Tvak | त्वक् (Skin) | Syntax validation |
| 18 | Cakṣus | चक्षुस् (Eye) | Type checking |
| 17 | Rasanā | रसना (Tongue) | Semantic validation |
| 16 | Ghrāṇa | घ्राण (Nose) | Borrow checking |

```rust
// compiler/samkhya/indriyas.rs
pub struct JnanaIndriyas {
    pub shrotra: LexicalValidator,   // Hearing: token validity
    pub tvak: SyntaxValidator,       // Touch: structural integrity
    pub cakshus: TypeChecker,        // Sight: type correctness
    pub rasana: SemanticValidator,   // Taste: meaning correctness
    pub ghrana: BorrowChecker,       // Smell: ownership validity
}
```

---

### Levels 15-11: Karmendriyas (कर्मेन्द्रिय) — Output Faculties

Five action organs — code transformations.

| Level | Tattva | Sanskrit | Compiler Function |
|-------|--------|----------|-------------------|
| 15 | Vāk | वाक् (Speech) | Error reporting |
| 14 | Pāṇi | पाणि (Hands) | Code manipulation |
| 13 | Pāda | पाद (Feet) | Control flow transforms |
| 12 | Pāyu | पायु (Excretion) | Dead code elimination |
| 11 | Upastha | उपस्थ (Generation) | Code generation |

```rust
// compiler/samkhya/indriyas.rs
pub struct KarmaIndriyas {
    pub vak: ErrorReporter,          // Speech: emit diagnostics
    pub pani: CodeManipulator,       // Hands: rewrite patterns
    pub pada: ControlFlowTransform,  // Feet: CFG optimization
    pub payu: DeadCodeEliminator,    // Excretion: remove unused
    pub upastha: CodeGenerator,      // Generation: emit code
}
```

---

### Levels 10-6: Tanmātras (तन्मात्र) — Subtle Elements

Five subtle essences — optimization passes.

| Level | Tattva | Sanskrit | Compiler Function |
|-------|--------|----------|-------------------|
| 10 | Śabda | शब्द (Sound) | Constant propagation |
| 9 | Sparśa | स्पर्श (Touch) | Common subexpression |
| 8 | Rūpa | रूप (Form) | Loop optimization |
| 7 | Rasa | रस (Taste) | Inlining decisions |
| 6 | Gandha | गन्ध (Smell) | Memory optimization |

```rust
// compiler/samkhya/tanmatras.rs
pub struct Tanmatras {
    pub shabda: ConstantPropagator,  // Sound: propagate constants
    pub sparsha: CSEPass,            // Touch: eliminate common subexpr
    pub rupa: LoopOptimizer,         // Form: unroll, vectorize
    pub rasa: Inliner,               // Taste: inline functions
    pub gandha: MemoryOptimizer,     // Smell: reduce allocations
}
```

---

### Levels 5-1: Mahābhūtas (महाभूत) — Gross Elements

Five great elements — machine code generation.

| Level | Tattva | Sanskrit | Compiler Function |
|-------|--------|----------|-------------------|
| 5 | Ākāśa | आकाश (Space/Ether) | Register allocation |
| 4 | Vāyu | वायु (Air) | Instruction scheduling |
| 3 | Tejas | तेजस् (Fire) | Peephole optimization |
| 2 | Āpas | आपस् (Water) | Data layout |
| 1 | Pṛthivī | पृथिवी (Earth) | Binary emission |

```rust
// compiler/samkhya/mahabhutas.rs
pub struct Mahabhutas {
    pub akasha: RegisterAllocator,   // Space: allocate registers
    pub vayu: InstructionScheduler,  // Air: order instructions
    pub tejas: PeepholeOptimizer,    // Fire: local optimizations
    pub apas: DataLayouter,          // Water: arrange data
    pub prithivi: BinaryEmitter,     // Earth: emit executable
}
```

---

## The Complete Pipeline

```rust
/// Run the full Sāṃkhya compilation pipeline
pub fn compile_samkhya(source: &str, options: &Options) -> Result<Binary> {
    // Level 24: Prakṛti → Manifest source
    let prakriti = Prakriti::new(source);

    // Level 23: Buddhi → Semantic analysis
    let buddhi = prakriti.manifest();
    buddhi.discriminate(&options.context)?;

    // Level 22: Ahaṃkāra → Symbol resolution
    let ahamkara = Ahamkara::from(&buddhi);

    // Level 21: Manas → IR generation
    let mut manas = Manas::new();
    let mir = manas.process(&buddhi);

    // Levels 20-16: Jñānendriyas → Type checking
    let jnana = JnanaIndriyas::new();
    jnana.validate_all(&mir)?;

    // Levels 15-11: Karmendriyas → Transformations
    let karma = KarmaIndriyas::new();
    let transformed = karma.transform(&mir);

    // Levels 10-6: Tanmātras → Optimization
    let tanmatras = Tanmatras::with_guna(options.guna);
    let optimized = tanmatras.optimize(&transformed);

    // Levels 5-1: Mahābhūtas → Code generation
    let mahabhutas = Mahabhutas::for_target(options.target);
    let binary = mahabhutas.emit(&optimized)?;

    Ok(binary)
}
```

---

## Guṇa Influence on Pipeline

The three guṇas affect how deeply each stage optimizes:

| Stage | Sattva (Correctness) | Rajas (Speed) | Tamas (Memory) |
|-------|---------------------|---------------|----------------|
| Buddhi | Extra validation | Fast parsing | Minimal AST |
| Tanmātras | Safe opts only | Aggressive | Size-focused |
| Mahābhūtas | Debug symbols | Max opt | Min binary |

---

## Debugging the Pipeline

```sanskrit
# Enable tattva tracing
jagc build --trace-tattvas main.jag

# Output:
[24.Prakṛti] Tokenizing 1,234 bytes...
[23.Buddhi] Building AST: 47 nodes
[22.Ahaṃkāra] Registering 23 symbols
[21.Manas] Generating 156 MIR instructions
[18.Cakṣus] Type checking: 47 expressions
[16.Ghrāṇa] Borrow checking: 12 references
[8.Rūpa] Loop optimization: 3 loops unrolled
[5.Ākāśa] Register allocation: 89% utilized
[1.Pṛthivī] Emitting 4,567 bytes to a.out
```

---

## See Also

- [Nyāya Type Inference](nyaya_guide.md) — How Buddhi uses pramāṇas
- [Pancha Kosha Memory](pancha_kosha.md) — Memory hierarchy (related to Tanmātras)
- [Guṇa Optimization](../yoga/chakra_architecture.md) — How guṇas affect passes
