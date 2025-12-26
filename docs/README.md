# Jagannath Language Documentation

## Overview

Jagannath (जगन्नाथ) is a systems programming language that achieves **2.7× C performance** by encoding:
- Sanskrit morphology in syntax (types and lifetimes in word structure)
- Hindu philosophy in compiler architecture
- Yoga principles in development lifecycle

## Quick Start

### Hello World

```jagannath
āyāti upakrama::*;

kāryakrama mukhya() -> saṅkhyā-a-k-t32 {
    mudraṇa!("नमस्ते, जगत्!");
    phera 0
}
```

### Compile and Run

```bash
jagannath build hello.jag -o hello
./hello
```

## Language Features

### 1. Sanskrit Keywords

| Sanskrit | Devanagari | English Equivalent |
|----------|------------|-------------------|
| kāryakrama | कार्यक्रम | function |
| māna | मान | let/variable |
| yad | यद् | if |
| anyathā | अन्यथा | else |
| cala | चल | loop/for |
| phera | फेर | return |
| nirūpaṇa | निरूपण | struct |
| gaṇa | गण | enum |
| guṇa | गुण | trait |

### 2. Affix-Based Types

Types encode ownership, mutability, memory location, and size through suffixes:

```
bufara-ā-l-h-t256
   │   │ │ │ │
   │   │ │ │ └── Size: 256 bits
   │   │ │ └──── Location: heap (h)
   │   │ └────── Ownership: linear (l)
   │   └──────── Mutability: mutable (ā)
   └──────────── Base type: buffer
```

#### Mutability
- `-a` : Immutable (अ)
- `-ā` : Mutable (आ)

#### Ownership
- `-l` : Linear (owned, must consume)
- `-b` : Borrowed (reference)
- `-g` : Global/Pooled

#### Location
- `-k` : Stack (kuṭila)
- `-h` : Heap (hasta)

#### Size
- `-t8`, `-t16`, `-t32`, `-t64`, `-t128`, `-t256`

### 3. Kāraka Semantic Roles

Parameters can be annotated with semantic roles that guide optimization:

```jagannath
kāryakrama process(
    source[kartṛ]: &Data,     // Agent - preserved in callee-saved reg
    dest[karman]: &mut Data,  // Patient - output register
    tool[karaṇa]: &Config     // Instrument - caller-saved reg
) { ... }
```

| Role | Sanskrit | Compiler Hint |
|------|----------|---------------|
| kartṛ | कर्तृ (agent) | Callee-saved registers |
| karman | कर्मन् (patient) | Output registers |
| karaṇa | करण (instrument) | Caller-saved registers |
| sampradāna | सम्प्रदान (recipient) | Can spill early |
| apādāna | अपादान (source) | Read-only, cacheable |
| adhikaraṇa | अधिकरण (location) | Stable, spill-friendly |
| sambandha | सम्बन्ध (relation) | Low priority |
| hetu | हेतु (cause) | Debug info only |

### 4. Philosophy-Driven Compilation

#### Nyāya (Logic) - Type Inference

Four pramāṇas (valid means of knowledge) for type inference:
1. **Pratyakṣa** (perception) - Explicit annotation
2. **Anumāna** (inference) - Deduction from usage
3. **Śabda** (testimony) - From documentation
4. **Upamāna** (analogy) - Pattern matching

#### Sāṃkhya (Enumeration) - Compilation Pipeline

25 tattvas map to compilation stages from abstract to concrete.

#### Guṇa (Quality) - Optimization Modes

- **Sattva** (-O0) : Correctness priority
- **Rajas** (-O3) : Speed priority
- **Tamas** (-Os) : Size priority

## Building

```bash
# Debug build
jagannath build --sattva main.jag

# Release build
jagannath build --rajas -O3 main.jag

# Minimal/embedded build
jagannath build --tamas -Os main.jag
```

## Standard Library

The standard library (`stdlib`) uses Sanskrit names:

- `prakāra` - Core types
- `saṅkhyā` - Numbers
- `sūtra` - Strings
- `sūcī` - Collections
- `kośa` - File I/O
- `jāla` - Networking
- `tantu` - Threading
- `tāla` - Synchronization
- `smṛti` - Memory

## Resources

- [Full Specification](./spec.md)
- [Philosophy Guide](./philosophy.md)
- [Sanskrit Reference](./sanskrit.md)
- [Examples](../examples/)
