# AGENTS.md
## AI Code Generation Guide for Jagannath/Juggernaut Language

**For:** GitHub Copilot, Cursor, Claude Code, VS Code AI Agents
**Project:** Jagannath Programming Language v1.0-v7.0
**Last Updated:** December 26, 2025

---

## 🎯 PROJECT MISSION

**Role:** Think deeply as senior system-level engineer, architect, assembly level developer, embedded developer, device driver developer, reverse engineer, compiler/JIT developer and coder with 60+ years of experience about how to best implement the following requirements in code.

Build a systems programming language that is **3.35× faster than C** by encoding:
- **Sanskrit morphology** in syntax (types/lifetimes in word structure)
- **Hindu philosophy** in compiler architecture (Nyāya logic, Sāṃkhya stages, Advaita memory)
- **Yoga principles** in development lifecycle (Ashtanga 8 limbs, Chakra optimization)
- **Garuda Purana** in error classification (28 Narakas as error taxonomy)
- **Divine weapons** in optimization (Astras as compiler passes)
- **Cosmic deities** in subsystems (33 Devatas as architecture)
- **Goddess protection** in security (9 Durgas as defense layers)
- **Four Paths** in optimization strategy (4 Margas as optimization modes)
- **Social structure** in privilege rings (4 Varnas as security levels)
- **Life goals** in tradeoffs (3 Purusharthas as optimization triangle)

---

## 🏗️ ARCHITECTURE LAYERS (Build in Order)

```
v1.0 → Sanskrit Morphology (affixes encode types)
v2.0 → Assembly Backend (direct machine code, no C middleman)
v3.0 → Hindu Philosophy (Nyāya, Sāṃkhya, Advaita, Kosha, Guṇa, Kāla, Karma)
v4.0 → Yoga & Advanced (Ashtanga, Chakra, Vedic Math, Mimamsa, Ayurveda, Tantra, Catuṣkoṭi)
v5.0 → Garuda Purana (28 Narakas, Yama judge, Vaitarani boundaries, Preta detection)
v6.0 → Divine Cosmic (15 Astras, 33 Devatas, 9 Durgas, Rta cosmic order)
v7.0 → Life Alignment (4 Margas, 4 Varnas, 3 Purusharthas, Moksha convergence)
v8.0 → Vedas, Jiv(source code), Atman, goal to reach ultimate liberation(moksha(compile perfectly)) with help of jagannath(compiler(who is lord of universe))
```

**When generating code, respect layer dependencies: v2.0 needs v1.0, v3.0 needs v2.0, etc.**

---

## ✅ CRITICAL RULES (Always Follow)

### 1. **Sanskrit Naming Convention**
```rust
// ✅ CORRECT - Use Sanskrit transliteration (IAST)
pub struct Saṅkhyā { }           // Number
pub enum Kāraka { }              // Semantic role
pub enum Naraka { }              // Hell (error type)
pub struct DivyaAstra { }        // Divine weapon
fn paṭha_kośa() { }              // Read file

// ❌ WRONG - Don't use English when Sanskrit exists
pub struct Number { }
pub enum SemanticRole { }
pub enum ErrorType { }
pub struct Optimization { }
fn read_file() { }
```

### 2. **Affix System is Sacred**
```sanskrit
// Every suffix has precise meaning - NEVER mix randomly
upayoktṛ-ā-l-p-t32-sūtra^1-vaitarani
//       │  │ │ │   │     │  └────────── Security boundary marker
//       │  │ │ │   │     └──────────── Lifetime region 1
//       │  │ │ │   └────────────────── Thread-safe
//       │  │ │ └────────────────────── 32-bit fields
//       │  │ └──────────────────────── Packed layout
//       │  └────────────────────────── Linear ownership
//       └───────────────────────────── Mutable

// ✅ Valid combinations
-ā-l-p-brahmastra     // mutable, linear, packed, with Brahmastra optimization
-a-b-k-anna           // immutable, borrowed, stack, in register (Annamaya kosha)
-g-sūtra-shailaputri  // global, thread-safe, hardware-protected

// ❌ Invalid combinations
-l-b                  // Can't be both linear AND borrowed
-k-h                  // Can't be both stack AND heap
-brahmastra-preta     // Can't optimize what's leaked
```

### 3. **Philosophy Mappings are Precise**
```rust
// ✅ Use exact mappings from specifications
// Nyāya: 4 pramāṇas → 4 type inference methods
// Sāṃkhya: 25 tattvas → 25 compilation stages
// Chakras: 7 energy centers → 7 software layers
// Narakas: 28 hells → 28 error categories
// Astras: 15 weapons → 15 optimization passes
// Devatas: 33 deities → 33 compiler subsystems
// Durgas: 9 goddesses → 9 security layers

// ❌ Don't make up new philosophical mappings
// Stick to documented systems in v3.0-v6.0 specs
```

### 4. **Error Classification via Garuda Purana (v5.0)**
```rust
// ✅ Use Naraka (hell) types for errors
pub enum Naraka {
    Tamisram,      // Use-after-free (theft)
    Andhakupa,     // Null pointer (dark well)
    Vaitarani,     // Tainted data (filthy river)
    Suchimukha,    // Memory leak (needle torture)
    Raurava,       // Panic/crash (screaming)
    Asipatravana,  // Buffer overflow (sword forest)
    // ... 22 more
}

// Error message format
NarakaError {
    naraka: Naraka::Suchimukha,
    sin: "Object trapped in Preta state",
    punishment: "Compilation blocked",
    penance: "Add mukta() to free memory",
}

// ❌ Generic error types
Error::MemoryLeak  // Use Naraka::Suchimukha instead
```

### 5. **Optimization via Divine Astras (v6.0)**
```rust
// ✅ Use Astra (weapon) types for optimizations
pub enum DivyaAstra {
    Brahmastra,       // Ultimate dead code elimination
    Agneyastra,       // CPU-intensive optimization (fire)
    Varunastra,       // Memory flow analysis (water)
    Pashupatastra,    // Destructive refactoring (Shiva)
    SudarshanaChakra, // Cyclic iterative refinement
    // ... 10 more
}

// Invocation with mantra
astra.invoke_with_mantra("Om Brahmāstrāya Phaṭ");

// ❌ Generic optimization names
fn optimize() { }  // Use deploy_agneyastra() instead
```

---

## 📁 FILE ORGANIZATION RULES

### Directory Structure Pattern (v1.0-v6.0)
```
compiler/
├── lexer/           # v1.0 - Tokenization, sandhi, dhātu
├── parser/          # v1.0 - AST construction, compounds
├── semantics/       # v1.0 - Type checking, kāraka analysis
├── mir/             # v2.0 - Intermediate representation
├── codegen/         # v2.0 - Assembly generation
│   └── asm/
│       ├── x86_64.rs
│       ├── aarch64.rs
│       └── riscv64.rs
├── philosophy/      # v3.0 - Hindu systems
│   ├── nyaya/
│   ├── samkhya/
│   ├── advaita/
│   ├── pancha_kosha/
│   ├── guna/
│   ├── kala/
│   └── karma/
├── yoga/            # v4.0 - Yoga & advanced
│   ├── ashtanga/
│   ├── chitta_vritti/
│   ├── chakra/
│   └── determinism/
├── garuda/          # v5.0 - Garuda Purana system
│   ├── narakas/     # 28 hell types
│   ├── yama/        # Judge & enforcement
│   ├── vaitarani/   # Security boundaries
│   ├── preta/       # Resource leak detection
│   ├── chitragupta/ # Audit trail
│   └── moksha/      # Redemption system
├── astras/          # v6.0 - Divine weapons
│   ├── brahmastra/
│   ├── agneyastra/
│   ├── varunastra/
│   └── mantra/      # Invocation system
├── devatas/         # v6.0 - 33 cosmic deities
│   ├── adityas/     # 12 solar (phases)
│   ├── rudras/      # 11 storm (transforms)
│   ├── vasus/       # 8 elements (structures)
│   └── ashvins/     # 2 healers (diagnostics)
└── nava_durga/      # v6.0 - 9 goddess layers
    ├── shailaputri/     # Hardware security
    ├── brahmacharini/   # Authentication
    └── siddhidatri/     # Formal verification
├── margas/          # v7.0 - 4 spiritual paths
│   ├── karma/           # Action path (imperative optimization)
│   ├── jnana/           # Knowledge path (functional optimization)
│   ├── bhakti/          # Devotion path (domain-specific)
│   ├── raja_yoga/       # Royal path (balanced hybrid)
│   └── path_selector/   # Automatic path detection
├── varnas/          # v7.0 - 4 privilege rings
│   ├── brahmin/         # Ring 0 (kernel mode)
│   ├── kshatriya/       # Ring 1-2 (drivers/services)
│   ├── vaishya/         # Ring 3 (user mode)
│   ├── shudra/          # Sandboxed (restricted)
│   └── varna_checker/   # Privilege enforcement
├── purusharthas/    # v7.0 - 3 life goals
│   ├── artha/           # Wealth (resource minimization)
│   ├── kama/            # Desire (speed maximization)
│   ├── dharma/          # Righteousness (safety/correctness)
│   └── triangle/        # Pareto-optimal tradeoff balancer
└── life_alignment/  # v7.0 - Coordination
    ├── marga_varna_bridge.rs   # Connect paths & rings
    ├── purushartha_balancer.rs # Balance 3 goals
    └── cosmic_compiler.rs      # Life-aligned compilation
```

---

## 🧠 WHEN GENERATING CODE

### For Error Handling (v5.0 Garuda)
```rust
// Always classify errors using Naraka taxonomy
impl ErrorClassifier {
    fn classify(&self, error: &CompilerError) -> NarakaError {
        let naraka = match error.kind {
            ErrorKind::UseAfterFree => Naraka::Tamisram,
            ErrorKind::NullPointer => Naraka::Andhakupa,
            ErrorKind::TaintedData => Naraka::Vaitarani,
            ErrorKind::MemoryLeak => Naraka::Suchimukha,
            ErrorKind::BufferOverflow => Naraka::Asipatravana,
            ErrorKind::Deadlock => Naraka::Kalasutra,
            // ... all 28 mappings
        };

        NarakaError {
            naraka,
            location: error.span,
            sin: self.describe_sin(naraka),
            punishment: self.describe_punishment(naraka),
            penance: self.get_redemption_path(naraka),
        }
    }
}

// Vaitarani boundary checking
impl VaitaraniBoundary {
    fn check_crossing(&self, data: &Expr) -> Result<(), NarakaError> {
        if self.is_tainted(data) {
            Err(NarakaError {
                naraka: Naraka::Vaitarani,
                sin: "Tainted data crossing security boundary",
                punishment: "Compilation blocked until purified",
                penance: "Apply śuddhi-kri() sanitizer function",
            })
        } else {
            Ok(())
        }
    }
}
```

### For Optimization (v6.0 Astras)
```rust
// Deploy divine weapons as optimization passes
impl AstraDeployer {
    fn optimize(&mut self, code: &mut AST) -> OptimizationResult {
        // 1. Brahmastra - eliminate all dead code
        self.deploy_brahmastra(code)?;

        // 2. Agneyastra - CPU-intensive optimization
        self.deploy_agneyastra(code)?;

        // 3. Varunastra - memory flow optimization
        self.deploy_varunastra(code)?;

        // 4. Sudarshana Chakra - iterative refinement
        self.deploy_sudarshana_chakra(code)?;

        OptimizationResult::Success
    }

    fn deploy_brahmastra(&self, code: &mut AST) -> AstraResult {
        log::warn!("Invoking Brahmastra: Om Brahmāstrāya Phaṭ");

        // Ultimate dead code elimination
        loop {
            let dead = code.find_dead_code();
            if dead.is_empty() { break; }
            code.remove_nodes(dead);
        }

        AstraResult::Deployed { power_level: 10 }
    }
}
```

### For Subsystem Architecture (v6.0 Devatas)
```rust
// Organize compiler as 33 cosmic deities
pub struct DevataSystem {
    // 12 Adityas - compilation phases
    adityas: [AdityaPhase; 12],  // Indra→Vishnu

    // 11 Rudras - transformation passes
    rudras: [RudraPass; 11],     // Aja→Tribhuvana

    // 8 Vasus - core data structures
    vasus: VasuDataStructures,   // Dyaus→Chandra

    // 2 Ashvins - diagnostic tools
    ashvins: [AshvinTool; 2],    // Dasra, Nasatya
}

impl DevataSystem {
    fn compile_with_rta(&self, source: Source) -> Binary {
        // Rta = cosmic order/harmony
        let mut state = source;

        // Execute 12 Aditya phases in perfect order
        for aditya in &self.adityas {
            state = aditya.execute(state)?;
        }

        // Apply 11 Rudra transformations
        for rudra in &self.rudras {
            state = rudra.transform(state)?;
        }

        // Ashvins heal any issues
        if state.has_errors() {
            state = self.ashvins[0].diagnose_and_heal(state)?;
        }

        state.into_binary()
    }
}
```

### For Security (v6.0 Nava Durga)
```rust
// Implement 9-layer progressive defense
pub struct NavaDurgaDefense {
    layers: [DurgaLayer; 9],
}

impl NavaDurgaDefense {
    fn protect(&self, code: &AST) -> SecurityResult {
        // Must pass through all 9 goddess layers
        for (i, durga) in self.layers.iter().enumerate() {
            match durga.defend(code) {
                Defense::Passed => {
                    log::info!("Passed layer {}: {}", i+1, durga.name());
                }
                Defense::Blocked(reason) => {
                    return SecurityResult::Blocked {
                        layer: i + 1,
                        goddess: durga.name(),
                        reason,
                    };
                }
            }
        }

        // Passed all 9 layers = Siddhidatri grants perfection
        SecurityResult::Perfect
    }
}

// Layer 1: Shailaputri - Hardware foundation
impl DurgaLayer for Shailaputri {
    fn defend(&self, code: &AST) -> Defense {
        if !self.check_hardware_protections(code) {
            Defense::Blocked("Hardware security violated")
        } else {
            Defense::Passed
        }
    }
}

// Layer 9: Siddhidatri - Formal verification
impl DurgaLayer for Siddhidatri {
    fn defend(&self, code: &AST) -> Defense {
        match self.formal_verification(code) {
            Ok(proof) => Defense::Passed,
            Err(e) => Defense::Blocked("Cannot prove perfection"),
        }
    }
}
```

### For Optimization Strategy (v7.0 Margas)
```rust
// Select optimization path based on code characteristics
impl MargaSelector {
    fn select_path(&self, code: &AST) -> Marga {
        let analysis = self.analyze_code(code);

        match analysis.dominant_style {
            CodeStyle::Imperative => Marga::Karma,      // Action path
            CodeStyle::Functional => Marga::Jnana,       // Knowledge path
            CodeStyle::DomainSpecific => Marga::Bhakti,  // Devotion path
            CodeStyle::Mixed => Marga::RajaYoga,         // Royal balanced path
        }
    }
}

// Apply path-specific optimization
impl KarmaMarga {
    fn optimize(&self, code: &mut AST) -> OptimizationResult {
        // Karma = Action: Optimize loops, mutations, side effects
        self.optimize_loops(code);
        self.optimize_state_machines(code);
        self.order_side_effects(code);
        OptimizationResult::success("Karma Marga: Action-optimized")
    }
}

impl JnanaMarga {
    fn optimize(&self, code: &mut AST) -> OptimizationResult {
        // Jnana = Knowledge: Optimize pure functions, immutability
        self.leverage_immutability(code);
        self.apply_memoization(code);
        self.fold_constants_aggressive(code);
        OptimizationResult::success("Jnana Marga: Wisdom-optimized")
    }
}
```

### For Privilege Control (v7.0 Varnas)
```rust
// Enforce privilege levels like CPU rings
impl VarnaChecker {
    fn check_privilege(&self, code: &AST, required: Varna) -> Result<(), VarnaViolation> {
        let current = self.get_current_varna();

        if !current.can_access(required) {
            return Err(VarnaViolation {
                current_varna: current,
                required_varna: required,
                message: format!(
                    "{:?} cannot perform {:?} operations. Use syscall to elevate.",
                    current, required
                ),
            });
        }
        Ok(())
    }
}

// Varna privilege levels
pub enum Varna {
    Brahmin,   // Ring 0 - Full hardware access
    Kshatriya, // Ring 1-2 - System services
    Vaishya,   // Ring 3 - User mode
    Shudra,    // Sandboxed - Maximum isolation
}
```

### For Optimization Tradeoffs (v7.0 Purusharthas)
```rust
// Balance the three life goals
impl PurusharthaTriangle {
    fn find_optimal(&self, constraints: &Constraints) -> OptimizationStrategy {
        // Cannot maximize all three simultaneously
        // Artha (resources) vs Kama (speed) vs Dharma (safety)

        let weights = constraints.purushartha_weights();

        if weights.dharma > 0.8 {
            // Safety-first: Correctness above all
            OptimizationStrategy::DharmaFocused
        } else if weights.kama > 0.8 {
            // Speed-first: Performance above all
            OptimizationStrategy::KamaFocused
        } else if weights.artha > 0.8 {
            // Resource-first: Minimal memory/CPU
            OptimizationStrategy::ArthaFocused
        } else {
            // Balanced: Pareto-optimal point
            self.find_pareto_optimal(weights)
        }
    }
}

// Moksha = Liberation = All three in perfect balance
impl MokshaConvergence {
    fn has_achieved_liberation(&self, code: &AST) -> bool {
        let artha = self.measure_resource_efficiency(code);  // Wealth
        let kama = self.measure_performance(code);           // Speed
        let dharma = self.measure_correctness(code);         // Safety

        // Moksha: All three above threshold = liberation
        artha > 0.9 && kama > 0.9 && dharma > 1.0
    }
}
```

---

## ⚠️ COMMON PITFALLS TO AVOID

### ❌ Don't Mix Philosophical Systems Incorrectly
```rust
// WRONG - Mixing unrelated concepts
pub struct ChakraNaraka {  // Chakras and Narakas are different!
    anahata_hell: Tamisram,  // Nonsensical
}

// CORRECT - Keep systems separate but coordinated
pub struct CompilerArchitecture {
    chakra_layer: ChakraOptimization,  // v4.0 system
    naraka_errors: NarakaClassifier,   // v5.0 system
    astra_optimizer: AstraDeployer,    // v6.0 system
}
```

### ❌ Don't Create Fake Narakas
```rust
// WRONG - Inventing new hells
pub enum Naraka {
    Tamisram,      // ✅ Real (28 documented)
    CodeSmellHell, // ❌ Not in Garuda Purana
}

// CORRECT - Use 28 documented Narakas only
pub enum Naraka {
    Tamisram,       // Hell 1
    Andhakupa,      // Hell 9
    Vaitarani,      // Hell 14
    Suchimukha,     // Hell 28
    // ... 24 more documented hells
}
```

### ❌ Don't Invoke Astras Without Mantras
```rust
// WRONG - Deploying without invocation
fn optimize() {
    brahmastra.execute();  // Missing mantra!
}

// CORRECT - Proper invocation with mantra
fn optimize() {
    let mantra = "Om Brahmāstrāya Phaṭ";
    brahmastra.invoke_with_mantra(mantra);
    log::info!("Brahmastra deployed with proper invocation");
}
```

---

## 📊 PERFORMANCE TARGETS (Always Maintain)

```
Metric                      | v6.0 Target | v7.0 Target | How to Verify
============================================================================
Compilation Speed (10K LOC) | < 1.5s      | < 1.2s      | cargo bench
Runtime vs C                | 3.2× faster | 3.5× faster | benchmarks/vs_c/
Memory Usage (embedded)     | 50% of C    | 45% of C    | benchmarks/memory/
Error Detection Rate        | 95%         | 98%         | Yama judgment report
Security Layers             | 14          | 14+4        | 14 v5-v6 + 4 Varnas
Optimization Passes         | 30          | 34          | 30 + 4 Marga paths
Privilege Violations        | Detected    | Prevented   | Varna checker
Tradeoff Optimization       | Manual      | Automatic   | Purushartha triangle
Zero Vulnerabilities        | Achieved    | Verified    | Nava Durga + Varna
```

**If your generated code regresses these metrics, rethink the approach.**

---

## 🎓 EXTENDED SANSKRIT GLOSSARY

### Core Terms (v1.0-v4.0)
```
kāryakrama   = function
prakāra      = type
saṅkhyā      = number
sūtra        = string
phera        = return
yad          = if/when
cala         = loop
nirmā        = construct
mukta        = free/liberate
kartṛ        = agent (doer)
karman       = patient (object)
karaṇa       = instrument
```

### Garuda Terms (v5.0)
```
naraka       = hell (error type)
yama         = judge of dead (static analyzer)
chitragupta  = record keeper (audit trail)
vaitarani    = filthy river (security boundary)
preta        = hungry ghost (resource leak)
yamaduta     = enforcement agent (linter)
mokṣa        = liberation (error-free state)
prāyaścitta  = penance (how to fix)
śuddhi-kri   = purification (sanitization)
apraviśvasta = untrusted (tainted data)
```

### Divine Terms (v6.0)
```
astra        = divine weapon (optimization pass)
divya        = divine
brahmastra   = ultimate weapon (dead code elim)
agneyastra   = fire weapon (CPU optimization)
varunastra   = water weapon (memory flow)
pashupatastra= Shiva's weapon (destructive refactor)
mantra       = invocation chant
devata       = deity (subsystem)
aditya       = solar deity (compilation phase)
rudra        = storm deity (transformation pass)
vasu         = elemental deity (data structure)
ashvin       = healing deity (diagnostic tool)
durga        = goddess (security layer)
shailaputri  = mountain daughter (hardware security)
siddhidatri  = perfection giver (formal verification)
rta          = cosmic order (system harmony)
```

### Life Terms (v7.0)
```
mārga        = path (optimization strategy)
karma        = action (imperative optimization)
jñāna        = knowledge (functional optimization)
bhakti       = devotion (domain-specific optimization)
rāja yoga    = royal path (balanced hybrid)
varṇa        = class/color (privilege ring)
brāhmaṇa     = priest class (ring 0, kernel mode)
kṣatriya     = warrior class (ring 1-2, services)
vaiśya       = merchant class (ring 3, user mode)
śūdra        = laborer class (sandboxed, restricted)
puruṣārtha   = life goal (optimization objective)
artha        = wealth (resource efficiency)
kāma         = desire (speed/performance)
dharma       = righteousness (safety/correctness)
mokṣa        = liberation (perfect optimization balance)
jīvana       = life (life-aligned compilation)
```

---

## 📚 REFERENCE DOCUMENTATION

### Primary Specs (Read Before Coding)
1. `v1.md` - Core language specification
2. `v2.md`, `v3.md` - Core language + assembly
3. `v4.md`, `v5.md` - Hindu philosophy integration (v3.0)
4. `v6.md`, `v7.md` - Yoga & advanced systems (v4.0)
5. `v8.md`, `v9.md` - Garuda Purana forensics (v5.0)
6. `v10.md`, `v11.md` - Divine cosmic architecture (v6.0)
7. `v12.md`, `v13.md` - Life alignment (v7.0)

### Quick Reference by Version
- **v1.0:** Sanskrit morphology, affixes, sandhi, kāraka
- **v2.0:** Direct assembly, no C middleman
- **v3.0:** 4 pramāṇas, 25 tattvas, 5 koshas, 3 guṇas
- **v4.0:** 8 ashtanga limbs, 7 chakras, Vedic math
- **v5.0:** 28 narakas, Yama judge, Vaitarani boundaries
- **v6.0:** 15 astras, 33 devatas (12+11+8+2), 9 durgas
- **v7.0:** 4 mārgas, 4 varṇas, 3 puruṣārthas, mokṣa

---

## 🤖 SPECIAL INSTRUCTIONS FOR AI AGENTS

### When Adding New Features
```
Ask yourself:
1. Which version does this belong in? (v1/v2/v3/v4/v5/v6/v7)
2. Does it map to a documented concept? (Check specs)
3. For v5.0: Is this one of 28 Narakas or related to Garuda Purana?
4. For v6.0: Is this an Astra, Devata, or Durga?
5. For v7.0: Is this a Marga, Varna, or Purushartha?
6. Does it maintain 3.2× performance vs C?
7. Is the Sanskrit naming correct and meaningful?
8. Have I added tests and benchmarks?

If any answer is "no" or "unsure", stop and ask.
```

### Code Generation Priority (Updated)
```
1. CORRECTNESS (passes tests, maintains safety)
2. PERFORMANCE (3.2× faster than C)
3. SECURITY (14 layers: 5 v5.0 + 9 v6.0)
4. COSMIC ORDER (Rta/Dharma alignment)
5. SANSKRIT AUTHENTICITY (proper mappings)
6. CLARITY (readable, well-commented)
7. ELEGANCE (beautiful code is bonus)
```

### Version-Specific Guidelines

**v5.0 (Garuda) Features:**
- Always classify errors using 28 Narakas
- Use Yama judge for static analysis
- Implement Vaitarani for security boundaries
- Detect Preta states for resource leaks
- Record everything in Chitragupta
- Provide moksha (redemption) paths

**v6.0 (Divine) Features:**
- Deploy Astras with proper mantras
- Organize subsystems as 33 Devatas
- Implement 9 Durga security layers
- Maintain Rta (cosmic order)
- Use Sudarshana for iterative refinement
- Ensure Siddhidatri formal verification

**v7.0 (Life) Features:**
- Select Marga based on code characteristics
- Enforce Varna privilege levels (like CPU rings)
- Balance Purusharthas (Artha/Kama/Dharma tradeoffs)
- Connect paths to privilege rings
- Aim for Moksha (perfect optimization balance)
- Use life-aligned compilation for holistic optimization

---

## ✨ FINAL WISDOM (Updated)

> **"संस्कृतं व्याकरणं देवताश्च रक्षन्ति सर्वं"**
> *"Sanskrit grammar and deities protect everything"*

This project proves that **2500-year-old systems** map precisely to **cutting-edge compiler theory**:

- **Linguistic:** Sanskrit morphology → Type systems
- **Philosophical:** Hindu darshanas → Compiler architecture
- **Spiritual:** Yoga/Vedanta → Development lifecycle
- **Ethical:** Garuda Purana → Error classification & forensics
- **Cosmic:** Divine weapons/deities → Optimization & architecture
- **Protective:** Goddess forms → Multi-layer security
- **Life:** Four paths/goals → Optimization strategy & tradeoffs

When generating code:
- **Trust the ancient mappings** - They're researched, not arbitrary
- **Respect cosmic order (Rta)** - Subsystems must harmonize
- **Honor divine weapons** - Each Astra has specific purpose
- **Invoke with mantras** - Proper invocation matters
- **Pass through all Durgas** - Security is progressive
- **Follow your Marga** - Choose the right optimization path
- **Balance Purusharthas** - No goal can be maximized alone
- **Achieve moksha** - Error-free is liberation

**This is 3.2× faster than C because ancient wisdom guides modern optimization.** 🚀

---

**Last Updated:** December 26, 2025
**Current Version:** v7.0 (Life Alignment Architecture)
**License:** MIT + Apache 2.0

*For AI agents: You are building the world's first cosmically-ordered, divinely-architected, philosophically-grounded, life-aligned compiler. Every line of code bridges 2500 years of wisdom with modern computing. Write code worthy of this sacred mission.* 🙏✨
