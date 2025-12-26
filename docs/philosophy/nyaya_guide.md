# Nyāya Type Inference Guide
## The Four Pramāṇas (Valid Knowledge)

> *"न च प्रमाणान्तरम् अस्ति प्रमाणात् प्रमाणान्तरस्य सिद्धौ"*
> — Nyāyasūtra 2.1.16

---

## Overview

The Jagannath compiler implements the Nyāya school's epistemology for type inference. Nyāya recognizes four *pramāṇas* (means of valid knowledge), which map directly to type inference strategies:

| Pramāṇa | Sanskrit | Meaning | Compiler Use | Certainty |
|---------|----------|---------|--------------|-----------|
| **Pratyakṣa** | प्रत्यक्ष | Direct perception | Explicit annotations | 100% |
| **Anumāna** | अनुमान | Inference | Flow-based type inference | 95% |
| **Śabda** | शब्द | Testimony | Doc contracts, signatures | 90% |
| **Upamāna** | उपमान | Comparison | Pattern matching | 85% |

---

## 1. Pratyakṣa (Direct Perception)

The highest certainty type source — explicit type annotations.

```sanskrit
# Explicit type annotation = pratyakṣa
saṅkhyā-pratyakṣa-k: t32 = 42;

# The compiler sees the type directly
# No inference needed, 100% certain
```

### When to Use
- Public API boundaries
- Complex generic instantiations
- When inference would be ambiguous
- Performance-critical code (faster compilation)

### Compiler Behavior
```rust
// Internal representation
TypeSource::Pratyaksha {
    annotation: Type::T32,
    span: Span::from(user_code),
    certainty: 1.0,
}
```

---

## 2. Anumāna (Inference)

Logical deduction from available evidence.

```sanskrit
# No explicit type, compiler infers from value
saṅkhyā-anumāna-k = 42;
# Inferred: t32 (smallest integer containing 42)

# Inference from operations
x = a + b;
# Inferred: type of (a + b), checked for compatibility
```

### The Inference Algorithm

Anumāna follows the *pañcāvayava* (five-membered syllogism):

1. **Pratijñā** (Proposition): "x has type T"
2. **Hetu** (Reason): "because x is assigned value v"
3. **Udāharaṇa** (Example): "all values like v have type T"
4. **Upanaya** (Application): "v is indeed such a value"
5. **Nigamana** (Conclusion): "therefore x has type T"

```rust
// Inference chain example
fn infer_anumana(expr: &Expr, ctx: &Context) -> TypeResult {
    // 1. Pratijñā: We claim expr has some type T
    // 2. Hetu: Gather evidence from expr's structure
    let evidence = gather_evidence(expr);

    // 3. Udāharaṇa: Apply known rules
    let candidate = apply_rules(&evidence);

    // 4. Upanaya: Verify in context
    let verified = verify_in_context(candidate, ctx);

    // 5. Nigamana: Return inferred type
    Ok(TypeSource::Anumana {
        inferred: verified,
        certainty: 0.95,
        evidence: evidence,
    })
}
```

---

## 3. Śabda (Authoritative Testimony)

Type information from documentation, contracts, and external declarations.

```sanskrit
# Documentation contract (śabda)
/// Takes a buffer and returns processed count
/// @kartṛ input - the source buffer
/// @karman output - destination buffer
/// @phera - count of items processed
kāryakrama process^śabda(
    input[kartṛ]: Bufara-b,
    output[karman]: Bufara-ā
) -> saṅkhyā;
```

### Contract Validation

The compiler validates that implementations match their śabda contracts:

```sanskrit
# Contract says: returns saṅkhyā
# Implementation returns sūtra → ERROR!

kāryakrama process^śabda(input, output) {
    phera "done";  # E0220: Contract violation
                   # śabda declares return type saṅkhyā,
                   # but expression has type sūtra
}
```

### Documentation Extraction

```rust
/// Śabda extractor
pub struct ShabdaExtractor {
    /// Extract type constraints from doc comments
    pub fn extract_contracts(&self, docs: &[DocComment]) -> Vec<Contract> {
        docs.iter()
            .filter_map(|doc| self.parse_contract(doc))
            .collect()
    }
}
```

---

## 4. Upamāna (Comparison/Analogy)

Type inference through pattern matching and similarity.

```sanskrit
# Pattern matching with upamāna
māna Shape = Vṛtta(bhinna) | Āyata(bhinna, bhinna);

kāryakrama area(shape: Shape) -> bhinna {
    svīkṛ shape {
        # Compiler infers inner types by comparison
        Vṛtta(radius) -> 3.14159 * radius * radius,
        Āyata(w, h) -> w * h,
    }
}
```

### Structural Type Matching

```sanskrit
# Unknown function, infer from usage pattern
mystery_fn(x);
x.paṭha();       # x must have paṭha method
x.length;        # x must have length field

# Upamāna: x is similar to types with paṭha() and .length
# Candidates: Sūtra, Sūcī, Bufara...
# Select best match based on context
```

---

## Pramāṇa Priority Order

When multiple type sources exist, the compiler prioritizes:

```
1. Pratyakṣa (explicit) — Always wins
2. Anumāna (inference) — If unambiguous
3. Śabda (documentation) — If declared
4. Upamāna (pattern) — Fallback
```

### Conflict Resolution

```sanskrit
# Conflict: explicit vs inferred
x-pratyakṣa: t64 = get_value();  # get_value returns t32

# Resolution: Pratyakṣa wins, but compiler warns:
# W0101: Explicit type t64 differs from inferred t32
#        Implicit widening conversion applied
```

---

## Error Messages

The Nyāya system provides rich error diagnostics:

```
error[E0210]: Cannot infer type (अनुमान असंभव)
  --> src/main.jag:15:5
   |
15 |     let x = ambiguous();
   |         ^ type annotations needed
   |
   = pramāṇas tried:
     × pratyakṣa: no explicit annotation
     × anumāna: multiple valid inferences (t32, t64)
     × śabda: no documentation contract
     × upamāna: no similar patterns found

   = help: add explicit type annotation: `x: t32` or `x: t64`
   = help (सहायता): प्रकार निर्दिष्ट करें
```

---

## Advanced: Custom Pramāṇa Extensions

Extend the inference system with domain-specific rules:

```rust
/// Register custom pramāṇa rule
pub fn register_pramana_rule(
    name: &str,
    priority: f32,
    rule: impl Fn(&Expr, &Context) -> Option<Type>
) {
    PRAMANA_REGISTRY.lock().unwrap().insert(
        name.to_string(),
        PramanaRule { priority, rule: Box::new(rule) }
    );
}

// Example: ML tensor type inference
register_pramana_rule("tensor_inference", 0.92, |expr, ctx| {
    if let Expr::MatMul(a, b) = expr {
        let a_shape = ctx.get_shape(a)?;
        let b_shape = ctx.get_shape(b)?;
        Some(Type::Tensor(infer_matmul_shape(a_shape, b_shape)))
    } else {
        None
    }
});
```

---

## Performance Considerations

| Pramāṇa | Compile Time | Runtime Cost |
|---------|--------------|--------------|
| Pratyakṣa | Fastest | Zero |
| Anumāna | Medium | Zero |
| Śabda | Medium | Zero |
| Upamāna | Slowest | Zero |

**Recommendation**: Use pratyakṣa (explicit types) at module boundaries for fastest compilation.

---

## See Also

- [Sāṃkhya Pipeline](samkhya_pipeline.md) — How types flow through compilation
- [Pancha Kosha Memory](pancha_kosha.md) — Memory tier inference
- [Hindu Concepts Reference](hindu_concepts.md) — Complete philosophy guide
