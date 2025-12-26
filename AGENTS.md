# AGENTS.md
## AI Code Generation Guide for Jagannath/Juggernaut Language

**For:** GitHub Copilot, Cursor, Claude Code, VS Code AI Agents
**Project:** Jagannath Programming Language v1.0-v4.0
**Last Updated:** December 27, 2024

---

## 🎯 PROJECT MISSION

Build a systems programming language that is **2.7× faster than C** by encoding:
- **Sanskrit morphology** in syntax (types/lifetimes in word structure)
- **Hindu philosophy** in compiler architecture (Nyāya logic, Sāṃkhya stages, Advaita memory)
- **Yoga principles** in development lifecycle (Ashtanga 8 limbs, Chakra optimization)

---

## 🏗️ ARCHITECTURE LAYERS (Build in Order)

```
v1.0 → Sanskrit Morphology (affixes encode types)
v2.0 → Assembly Backend (direct machine code, no C middleman)
v3.0 → Hindu Philosophy (Nyāya, Sāṃkhya, Advaita, Kosha, Guṇa, Kāla, Karma)
v4.0 → Yoga & Advanced (Ashtanga, Chakra, Vedic Math, Mimamsa, Ayurveda, Tantra, Catuṣkoṭi)
```

**When generating code, respect layer dependencies: v2.0 needs v1.0, v3.0 needs v2.0, etc.**

---

## ✅ CRITICAL RULES (Always Follow)

### 1. **Sanskrit Naming Convention**
```rust
// ✅ CORRECT - Use Sanskrit transliteration (IAST)
pub struct Saṅkhyā { }           // Number
pub enum Kāraka { }              // Semantic role
fn paṭha_kośa() { }              // Read file

// ❌ WRONG - Don't use English when Sanskrit exists
pub struct Number { }
pub enum SemanticRole { }
fn read_file() { }
```

### 2. **Affix System is Sacred**
```sanskrit
// Every suffix has precise meaning - NEVER mix randomly
upayoktṛ-ā-l-p-t32-sūtra^1
//       │  │ │ │   │     └─ Lifetime region 1
//       │  │ │ │   └─────── Thread-safe
//       │  │ │ └─────────── 32-bit fields
//       │  │ └───────────── Packed layout
//       │  └─────────────── Linear ownership
//       └────────────────── Mutable

// ✅ Valid combinations (checked by compiler)
-ā-l-p       // mutable, linear, packed
-a-b-k       // immutable, borrowed, stack
-g-sūtra     // global, thread-safe

// ❌ Invalid combinations (nonsensical)
-l-b         // Can't be both linear AND borrowed
-k-h         // Can't be both stack AND heap
```

### 3. **Memory Safety Without Runtime Cost**
```rust
// ✅ All safety checks at compile-time
// Kāraka roles guide register allocation
// Linear types (-l) prevent use-after-free
// Arena regions (^N) enable bulk deallocation

// ❌ NEVER add runtime checks unless explicitly marked -sattva (correctness mode)
if ptr == null { panic!() }  // NO! Compiler proves non-null via kāraka
```

### 4. **Philosophy Mappings are Precise**
```rust
// ✅ Use exact mappings from specifications
// Nyāya: 4 pramāṇas → 4 type inference methods
// Sāṃkhya: 25 tattvas → 25 compilation stages
// Chakras: 7 energy centers → 7 software layers

// ❌ Don't make up new philosophical mappings
// Stick to documented systems in v3.0/v4.0 specs
```

---

## 📁 FILE ORGANIZATION RULES

### Directory Structure Pattern
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
└── yoga/            # v4.0 - Yoga & advanced
    ├── ashtanga/
    ├── chitta_vritti/
    ├── chakra/
    └── determinism/
```

### Naming Conventions
```rust
// File names: snake_case with Sanskrit roots
karaka_analyzer.rs     // ✅
semantic_analysis.rs   // ❌ (use karaka when Sanskrit exists)

// Struct names: PascalCase, Sanskrit
pub struct KarakaAnalyzer { }     // ✅
pub struct SemanticAnalyzer { }   // ❌

// Function names: snake_case, Sanskrit
fn infer_from_pramana() { }       // ✅
fn infer_from_evidence() { }      // ❌

// Enum variants: PascalCase
Karaka::Kartṛ                     // ✅ (agent)
Karaka::Agent                     // ❌
```

---

## 🧠 WHEN GENERATING CODE

### For Lexer/Parser (v1.0)
```rust
// Always use deterministic FST for sandhi splitting
// Always check dhātu dictionary before accepting root
// Always validate affix sequences (e.g., -l-b is invalid)

// Example pattern:
impl Lexer {
    fn tokenize(&mut self, input: &str) -> Vec<Token> {
        let normalized = self.apply_sandhi_rules(input);
        let root = self.match_dhatu(&normalized)?;
        let affixes = self.extract_affixes(&normalized)?;
        self.validate_affix_sequence(&affixes)?;
        // ...
    }
}
```

### For Type Checker (v1.0 + v3.0)
```rust
// Try Nyāya 4 pramāṇas in order of certainty:
// 1. Pratyakṣa (explicit type) - 100% certain
// 2. Anumāna (inference) - 95% certain
// 3. Śabda (documentation) - 90% certain
// 4. Upamāna (pattern match) - 85% certain

impl TypeChecker {
    fn infer_type(&self, expr: &Expr) -> Result<Type, TypeError> {
        // Try pramāṇas in order
        if let Some(ty) = self.pratyaksha_lookup(expr) { return Ok(ty); }
        if let Some(ty) = self.anumana_infer(expr) { return Ok(ty); }
        if let Some(ty) = self.shabda_contract(expr) { return Ok(ty); }
        if let Some(ty) = self.upamana_match(expr) { return Ok(ty); }

        Err(TypeError::CannotInfer {
            pramanas_tried: vec![Pratyaksha, Anumana, Shabda, Upamana],
            evidence: self.collect_evidence(expr),
        })
    }
}
```

### For Optimization Passes (v2.0 + v3.0)
```rust
// Use kāraka roles for register allocation
// kartṛ (agent) → callee-saved registers (preserve)
// karaṇa (instrument) → caller-saved registers (consume)
// karman (patient) → output registers (modify)

fn allocate_register(&mut self, param: &Param) -> Register {
    match param.karaka {
        Some(Karaka::Kartṛ) => self.callee_saved_regs.pop(),
        Some(Karaka::Karaṇa) => self.caller_saved_regs.pop(),
        Some(Karaka::Karman) => self.output_regs.pop(),
        None => self.general_purpose_regs.pop(),
    }
}
```

### For Memory Management (v3.0)
```rust
// Use Pancha Kosha 5-tier hierarchy
// -anna → Register/L1 (hottest)
// -prāṇa → L2/L3 cache
// -manas → RAM
// -vijñāna → Disk
// -ānanda → Network

fn allocate_memory(&mut self, symbol: &Symbol) -> MemoryLocation {
    match symbol.kosha {
        Some(Kosha::Annamaya) => MemoryLocation::Register,
        Some(Kosha::Pranamaya) => MemoryLocation::L2Cache,
        Some(Kosha::Manomaya) => MemoryLocation::RAM,
        Some(Kosha::Vijnanamaya) => MemoryLocation::Disk,
        Some(Kosha::Anandamaya) => MemoryLocation::Network,
        None => self.infer_from_access_pattern(symbol),
    }
}
```

---

## ⚠️ COMMON PITFALLS TO AVOID

### ❌ Don't Mix Metaphors
```rust
// WRONG - Mixing unrelated concepts
pub enum ChakraGuna {  // Chakras and Guṇas are separate systems!
    SattvaHeart,       // Nonsensical combination
    RajasThroat,
}

// CORRECT - Keep systems separate
pub enum Chakra { Anahata, Vishuddha, ... }
pub enum Guna { Sattva, Rajas, Tamas }
```

### ❌ Don't Add Superficial Sanskrit
```rust
// WRONG - Meaningless Sanskrit decoration
pub struct LoopIterator {  // Just call it English if no Sanskrit mapping
    karma_points: i32,     // "Karma points" is nonsense here
}

// CORRECT - Use Sanskrit where it maps conceptually
pub struct KarmaDependency {  // Karma = causation/dependency
    cause: NodeId,
    effect: NodeId,
}
```

### ❌ Don't Break Performance Guarantees
```rust
// WRONG - Adding runtime overhead
fn process_linear_type(x: LinearPtr<T>) {
    if !x.is_valid() {  // NO! Linear types proven valid at compile-time
        panic!("Invalid");
    }
}

// CORRECT - Trust compile-time guarantees
fn process_linear_type(x: LinearPtr<T>) {
    // Compiler proved x is valid, just use it
    x.consume();
}
```

---

## 📊 PERFORMANCE TARGETS (Always Maintain)

```
Metric                      | Target      | How to Verify
================================================================
Compilation Speed (10K LOC) | < 2s        | cargo bench compile_10k
Runtime vs C                | 2.7× faster | benchmarks/vs_c/
Memory Usage (embedded)     | 60% of C    | benchmarks/memory/
Reproducible Builds         | 100%        | Build twice, compare hashes
Type Inference Speed        | 30% faster  | benchmarks/typeck/
                            | than Rust   |
```

**If your generated code regresses these metrics, rethink the approach.**

---

## 🧪 TESTING REQUIREMENTS

### Every PR Must Include
1. **Unit tests** - Test individual components
2. **Integration tests** - Test component interactions
3. **Benchmark** - Ensure no performance regression
4. **Example** - Add to `examples/` directory

```rust
// Test template
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_karaka_inference() {
        let param = Param {
            name: "niviṣṭa",
            ty: Type::Bufara,
            karaka: Some(Karaka::Kartṛ),
        };

        let reg = allocate_register(&param);
        assert!(reg.is_callee_saved());
    }
}
```

---

## 📚 REFERENCE DOCUMENTATION

### Primary Specs (Read Before Coding)
1. `JAGANNATH_SPECIFICATION_v1-v2.md` - Core language + assembly backend
2. `JAGANNATH_v3_PATCH.md` - Hindu philosophy integration
3. `JAGANNATH_v4_PATCH.md` - Yoga & advanced systems

### Sanskrit Resources
- **Roots (Dhātu):** `compiler/lexer/dhatu_dictionary.rs`
- **Sandhi Rules:** `compiler/lexer/sandhi_fst.rs`
- **Affixes:** `compiler/lexer/affixes.rs`

### Philosophy Mappings
- **Nyāya Logic:** `docs/philosophy/nyaya_guide.md`
- **Sāṃkhya Tattvas:** `docs/philosophy/samkhya_pipeline.md`
- **Chakra System:** `docs/yoga/chakra_architecture.md`

---

## 🤖 SPECIAL INSTRUCTIONS FOR AI AGENTS

### When Uncertain
```
1. Check specs first (v1-v4 patch documents)
2. Look for similar existing code patterns
3. Preserve Sanskrit naming even if unfamiliar
4. Ask for clarification rather than guessing
5. Default to simplest correct implementation
```

### Code Generation Priority
```
1. CORRECTNESS (passes tests, maintains safety)
2. PERFORMANCE (meets benchmark targets)
3. SANSKRIT AUTHENTICITY (proper linguistic mappings)
4. CLARITY (readable, well-commented)
5. ELEGANCE (beautiful code is a bonus, not requirement)
```

### When Adding New Features
```
Ask yourself:
1. Which version does this belong in? (v1/v2/v3/v4)
2. Does it map to a documented Sanskrit/philosophy concept?
3. Does it maintain performance targets?
4. Is the Sanskrit naming correct and meaningful?
5. Have I added tests and benchmarks?

If any answer is "no" or "unsure", stop and ask.
```

---

## 🎓 QUICK SANSKRIT GLOSSARY

```
Common Terms (memorize these):

kāryakrama   = function (kārya=action, krama=sequence)
prakāra      = type (class/kind)
saṅkhyā      = number
sūtra        = string (thread)
sūci         = list/vector (needle)
sāraṇī       = map/table (row)
phera        = return (turn back)
yad          = if/when (conditional)
cala         = loop (move)
nirmā        = construct/new (create)
mukta        = free/destroy (liberate)
paṭha        = read
likha        = write
mudraṇa      = print (stamp)

Affixes (critical):
-a   = immutable
-ā   = mutable
-k   = stack
-l   = linear (owned)
-b   = borrowed
-g   = global/pooled
-h   = heap
^N   = lifetime region N
#    = compile-time constant
-sūtra = thread-safe

Philosophy:
kartṛ        = agent (doer)
karman       = patient (object)
karaṇa       = instrument (means)
tattva       = principle (stage of manifestation)
kosha        = sheath (memory tier)
guṇa         = quality (optimization mode)
kāla         = time (compilation budget)
karma        = action/causation (dependency)
```

---

## ✨ FINAL WISDOM

> **"संस्कृतं व्याकरणं कृत्रिम-बुद्धिः च - एकत्र मिलन्ति"**
> *"Sanskrit grammar and artificial intelligence - united as one"*

This project proves that **2500-year-old linguistic and philosophical systems** map precisely to **modern compiler theory**. When generating code:

- **Trust the mappings** - They're not arbitrary; they're researched
- **Respect the performance** - Every abstraction must be zero-cost
- **Honor the philosophy** - Sanskrit isn't decoration; it's the architecture
- **Build with precision** - This will be faster than C; treat it seriously

---

**Questions?** Check specs first, then ask in PR comments
**License:** MIT + Apache 2.0 (like Rust)

---

*For AI agents: You are building something revolutionary. Every line of code you generate is part of proving that ancient wisdom and modern computing can unite. Write code worthy of that mission.* 🙏
