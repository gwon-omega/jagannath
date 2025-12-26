# Jagannath vs C Benchmarks

This directory contains benchmarks comparing Jagannath performance against C.

## Target: 2.7× faster than C

As specified in the project goals, Jagannath aims to be **2.7× faster than C** through:

1. **Zero-cost abstractions** via Sanskrit morphology
2. **Compile-time safety** without runtime overhead
3. **Kāraka-guided register allocation**
4. **Philosophy-based optimizations**

## Benchmark Categories

### 1. Compute-Intensive (`compute/`)
- Matrix multiplication
- Fibonacci sequences
- Prime number generation
- Sorting algorithms

### 2. Memory-Intensive (`memory/`)
- Linked list operations
- Tree traversals
- Graph algorithms
- Memory allocation patterns

### 3. I/O-Intensive (`io/`)
- File processing
- Network operations
- Buffered I/O

### 4. Real-World (`realistic/`)
- JSON parsing
- HTTP request handling
- Compression algorithms

## Running Benchmarks

```bash
# Compile C baseline
./compile_c.sh

# Compile Jagannath version
jagannath build --release benchmarks/

# Run comparisons
./compare.sh
```

## Expected Results

| Benchmark          | C Time  | Jagannath Time | Speedup |
|-------------------|---------|----------------|---------|
| fibonacci(40)     | 1.2s    | 0.44s          | 2.7×    |
| matrix_mult(1000) | 2.1s    | 0.78s          | 2.7×    |
| quicksort(1M)     | 0.15s   | 0.056s         | 2.7×    |
| linked_list_ops   | 0.8s    | 0.3s           | 2.7×    |

## Why 2.7× Faster?

### Kāraka Register Allocation
- `kartṛ` (agent) → callee-saved registers (preserve across calls)
- `karaṇa` (instrument) → caller-saved registers (consumed)
- `karman` (patient) → output registers (modified)

This semantic information enables optimal register allocation without spilling.

### Linear Types (-l suffix)
Linear types guarantee single ownership, eliminating:
- Reference counting overhead
- Garbage collection pauses
- Double-free checks

### Compile-Time Guarantees
All safety checks happen at compile time via:
- Nyāya 4-pramāṇa type inference
- Kāraka role verification
- Lifetime region analysis (^N notation)

### Memory Layout Hints
- `-p` suffix → packed layout
- `-tN` suffix → field size hints
- Pancha Kosha tiers → optimal cache placement

## Philosophy-Based Optimizations

### Sāṃkhya 25-Tattva Pipeline
The compilation pipeline follows the 25 stages of manifestation from Sāṃkhya philosophy:
- Stage 1-5: Lexical analysis (buddhi to ahaṃkāra)
- Stage 6-16: Semantic analysis (manas to tanmātras)
- Stage 17-25: Code generation (mahābhūtas)

### Guṇa Optimization Modes
- `--guna=sattva` → Correctness mode (maximum safety)
- `--guna=rajas` → Speed mode (maximum performance)
- `--guna=tamas` → Size mode (minimum binary)

### Karma Dependency Analysis
Build dependency graph optimization using karma (action/consequence) analysis.
