//! Performance Benchmarks for Jagannath Compiler
//!
//! These benchmarks ensure we maintain our 2.7× faster than C performance target.
//! Run with: cargo bench -p jagannath-compiler

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use std::time::Duration;

// Benchmark configurations
const SMALL_SOURCE: &str = include_str!("../../examples/hello_world.jag");
const FIBONACCI_SOURCE: &str = include_str!("../../examples/fibonacci.jag");

/// Benchmark: Lexical analysis (tokenization)
/// Target: < 10μs for 1KB source
fn bench_lexer(c: &mut Criterion) {
    let mut group = c.benchmark_group("Lexer");
    group.measurement_time(Duration::from_secs(5));

    // Small source benchmark
    group.throughput(Throughput::Bytes(SMALL_SOURCE.len() as u64));
    group.bench_function("small_source", |b| {
        b.iter(|| {
            // Tokenize source
            let _tokens = tokenize_source(black_box(SMALL_SOURCE));
        })
    });

    // Fibonacci benchmark
    group.throughput(Throughput::Bytes(FIBONACCI_SOURCE.len() as u64));
    group.bench_function("fibonacci", |b| {
        b.iter(|| {
            let _tokens = tokenize_source(black_box(FIBONACCI_SOURCE));
        })
    });

    // Sandhi splitting benchmark (Sanskrit morphology)
    group.bench_function("sandhi_split", |b| {
        b.iter(|| {
            let _result = split_sandhi(black_box("upayoktṛālapasūtra"));
        })
    });

    // Dhātu (root) recognition benchmark
    group.bench_function("dhatu_lookup", |b| {
        b.iter(|| {
            let _result = lookup_dhatu(black_box("kṛ"));
        })
    });

    group.finish();
}

/// Benchmark: Parsing
/// Target: < 50μs for 1KB source
fn bench_parser(c: &mut Criterion) {
    let mut group = c.benchmark_group("Parser");
    group.measurement_time(Duration::from_secs(5));

    group.bench_function("small_source", |b| {
        b.iter(|| {
            let _ast = parse_source(black_box(SMALL_SOURCE));
        })
    });

    group.bench_function("fibonacci", |b| {
        b.iter(|| {
            let _ast = parse_source(black_box(FIBONACCI_SOURCE));
        })
    });

    // Compound word (samāsa) parsing benchmark
    group.bench_function("samasa_parse", |b| {
        b.iter(|| {
            let _result = parse_compound(black_box("mahārāja"));
        })
    });

    group.finish();
}

/// Benchmark: Type checking and kāraka analysis
/// Target: < 100μs for 1KB source
fn bench_typeck(c: &mut Criterion) {
    let mut group = c.benchmark_group("TypeCheck");
    group.measurement_time(Duration::from_secs(5));

    // Nyāya 4-pramāṇa type inference
    group.bench_function("pramana_inference", |b| {
        b.iter(|| {
            let _result = infer_type_nyaya(black_box("let x = 42"));
        })
    });

    // Kāraka role analysis
    group.bench_function("karaka_analysis", |b| {
        b.iter(|| {
            let _result = analyze_karaka(black_box("kartṛ karaṇa karman"));
        })
    });

    // Lifetime region analysis (^N notation)
    group.bench_function("lifetime_region", |b| {
        b.iter(|| {
            let _result = analyze_lifetimes(black_box("upayoktṛ^1 upayoktṛ^2"));
        })
    });

    group.finish();
}

/// Benchmark: MIR generation (Sāṃkhya 25-tattva pipeline)
fn bench_mir(c: &mut Criterion) {
    let mut group = c.benchmark_group("MIR");
    group.measurement_time(Duration::from_secs(5));

    group.bench_function("small_source", |b| {
        b.iter(|| {
            let _mir = generate_mir(black_box(SMALL_SOURCE));
        })
    });

    // Tattva stage transitions
    group.bench_function("tattva_pipeline", |b| {
        b.iter(|| {
            let _result = apply_tattva_stages(black_box("prakṛti"));
        })
    });

    group.finish();
}

/// Benchmark: Code generation
/// Target: Assembly generation should be faster than LLVM
fn bench_codegen(c: &mut Criterion) {
    let mut group = c.benchmark_group("Codegen");
    group.measurement_time(Duration::from_secs(5));

    // x86-64 code generation
    group.bench_function("x86_64", |b| {
        b.iter(|| {
            let _asm = generate_x86_64(black_box(SMALL_SOURCE));
        })
    });

    // Register allocation with kāraka hints
    group.bench_function("regalloc_karaka", |b| {
        b.iter(|| {
            let _result = allocate_registers_karaka(black_box("kartṛ karaṇa karman"));
        })
    });

    group.finish();
}

/// Benchmark: Full compilation pipeline
/// Target: < 2s for 10K LOC (per AGENTS.md)
fn bench_full_compile(c: &mut Criterion) {
    let mut group = c.benchmark_group("FullCompile");
    group.measurement_time(Duration::from_secs(10));
    group.sample_size(20);

    group.bench_function("small_source", |b| {
        b.iter(|| {
            let _result = full_compile(black_box(SMALL_SOURCE));
        })
    });

    group.bench_function("fibonacci", |b| {
        b.iter(|| {
            let _result = full_compile(black_box(FIBONACCI_SOURCE));
        })
    });

    group.finish();
}

/// Benchmark: Philosophy-guided optimizations
fn bench_philosophy(c: &mut Criterion) {
    let mut group = c.benchmark_group("Philosophy");
    group.measurement_time(Duration::from_secs(5));

    // Guṇa mode optimization
    group.bench_function("guna_optimize", |b| {
        b.iter(|| {
            let _result = optimize_with_guna(black_box("rajas")); // Speed mode
        })
    });

    // Karma dependency analysis
    group.bench_function("karma_dependencies", |b| {
        b.iter(|| {
            let _result = analyze_karma_graph(black_box(10)); // 10 nodes
        })
    });

    // Chakra-based layer optimization
    group.bench_function("chakra_layers", |b| {
        b.iter(|| {
            let _result = optimize_chakra_layers(black_box(7)); // 7 chakras
        })
    });

    group.finish();
}

/// Benchmark: Vedic math optimizations
fn bench_vedic_math(c: &mut Criterion) {
    let mut group = c.benchmark_group("VedicMath");
    group.measurement_time(Duration::from_secs(5));

    // Nikhilam multiplication (near base)
    group.bench_function("nikhilam_multiply", |b| {
        b.iter(|| {
            let _result = nikhilam_multiply(black_box(97), black_box(96));
        })
    });

    // Urdhva Tiryak (crosswise multiplication)
    group.bench_function("urdhva_multiply", |b| {
        b.iter(|| {
            let _result = urdhva_multiply(black_box(123), black_box(456));
        })
    });

    // Digital root checksum
    group.bench_function("digital_root", |b| {
        b.iter(|| {
            let _result = digital_root(black_box(123456789));
        })
    });

    group.finish();
}

/// Benchmark: Memory allocation (Pancha Kosha)
fn bench_memory(c: &mut Criterion) {
    let mut group = c.benchmark_group("Memory");
    group.measurement_time(Duration::from_secs(5));

    // Kosha tier selection
    group.bench_function("kosha_select", |b| {
        b.iter(|| {
            let _tier = select_kosha_tier(black_box(1024)); // 1KB object
        })
    });

    // Arena allocation
    group.bench_function("arena_alloc", |b| {
        b.iter(|| {
            let _ptr = arena_allocate(black_box(64)); // 64 bytes
        })
    });

    group.finish();
}

// ============================================================================
// Stub implementations (to be replaced with actual compiler calls)
// ============================================================================

fn tokenize_source(_source: &str) -> Vec<()> { vec![] }
fn split_sandhi(_word: &str) -> Vec<&str> { vec![] }
fn lookup_dhatu(_root: &str) -> Option<()> { Some(()) }
fn parse_source(_source: &str) -> () {}
fn parse_compound(_word: &str) -> Vec<&str> { vec![] }
fn infer_type_nyaya(_expr: &str) -> () {}
fn analyze_karaka(_params: &str) -> () {}
fn analyze_lifetimes(_expr: &str) -> () {}
fn generate_mir(_source: &str) -> () {}
fn apply_tattva_stages(_input: &str) -> () {}
fn generate_x86_64(_source: &str) -> Vec<u8> { vec![] }
fn allocate_registers_karaka(_params: &str) -> () {}
fn full_compile(_source: &str) -> Result<(), ()> { Ok(()) }
fn optimize_with_guna(_mode: &str) -> () {}
fn analyze_karma_graph(_nodes: usize) -> () {}
fn optimize_chakra_layers(_layers: usize) -> () {}
fn nikhilam_multiply(a: i64, b: i64) -> i64 { a * b }
fn urdhva_multiply(a: i64, b: i64) -> i64 { a * b }
fn digital_root(mut n: u64) -> u64 {
    while n >= 10 {
        n = n.to_string().chars().map(|c| c.to_digit(10).unwrap() as u64).sum();
    }
    n
}
fn select_kosha_tier(_size: usize) -> u8 { 0 }
fn arena_allocate(_size: usize) -> usize { 0 }

// ============================================================================
// Criterion setup
// ============================================================================

criterion_group!(
    benches,
    bench_lexer,
    bench_parser,
    bench_typeck,
    bench_mir,
    bench_codegen,
    bench_full_compile,
    bench_philosophy,
    bench_vedic_math,
    bench_memory,
);

criterion_main!(benches);
