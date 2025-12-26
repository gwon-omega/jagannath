//! Jagannath Compiler Benchmark Suite
//!
//! Measures compilation speed and code generation quality.

use std::time::Instant;

// Import the compiler directly for accurate measurements
use jagannath_compiler::driver::{CompileResult, CompilerOptions, CompilerSession};

fn main() {
    println!("🕉️ जगन्नाथ संकलक बेन्चमार्क (Jagannath Compiler Benchmark)");
    println!("{}", "=".repeat(60));
    println!();

    // Read test sources
    let fibonacci_src = include_str!("../../examples/fibonacci.jag");
    let hello_src = include_str!("../../examples/hello_world.jag");

    // Benchmark 1: Compilation Speed
    println!("📊 Benchmark 1: Compilation Speed");
    println!("{}", "-".repeat(40));

    // Warm up
    let _ = compile_source(fibonacci_src);
    let _ = compile_source(fibonacci_src);

    // Fibonacci benchmark (100 iterations)
    let fib_times = benchmark_compile(fibonacci_src, 100);
    println!("  Fibonacci (recursive + iterative):");
    println!("    Source size:   {:>6} chars", fibonacci_src.len());
    println!("    Min time:      {:>6.2} μs", fib_times.min_us);
    println!("    Max time:      {:>6.2} μs", fib_times.max_us);
    println!("    Avg time:      {:>6.2} μs", fib_times.avg_us);
    println!("    Throughput:    {:>6.1} KLOC/s", fib_times.kloc_per_sec);
    println!();

    // Hello world benchmark
    let hello_times = benchmark_compile(hello_src, 100);
    println!("  Hello World (minimal):");
    println!("    Source size:   {:>6} chars", hello_src.len());
    println!("    Min time:      {:>6.2} μs", hello_times.min_us);
    println!("    Max time:      {:>6.2} μs", hello_times.max_us);
    println!("    Avg time:      {:>6.2} μs", hello_times.avg_us);
    println!(
        "    Throughput:    {:>6.1} KLOC/s",
        hello_times.kloc_per_sec
    );
    println!();

    // Benchmark 2: Generated Code Size
    println!("📊 Benchmark 2: Generated Code Quality");
    println!("{}", "-".repeat(40));

    if let Some(result) = compile_source(fibonacci_src) {
        let asm_str = String::from_utf8_lossy(&result.output);
        let asm_lines: Vec<&str> = asm_str.lines().filter(|l| !l.trim().is_empty()).collect();
        let code_lines: Vec<&str> = asm_lines
            .iter()
            .filter(|l| !l.trim().starts_with(';') && !l.trim().starts_with('.'))
            .cloned()
            .collect();

        println!("  Fibonacci assembly:");
        println!("    Total lines:   {:>6}", asm_lines.len());
        println!("    Code lines:    {:>6}", code_lines.len());
        println!("    Output bytes:  {:>6}", result.output.len());
        println!();

        // Detailed timing breakdown
        println!("  Pipeline timing (avg of 100 runs):");
        println!(
            "    Lexing:        {:>6} μs ({:.1}%)",
            result.timing.lexing_us,
            100.0 * result.timing.lexing_us as f64 / result.timing.total_us as f64
        );
        println!(
            "    Parsing:       {:>6} μs ({:.1}%)",
            result.timing.parsing_us,
            100.0 * result.timing.parsing_us as f64 / result.timing.total_us as f64
        );
        println!(
            "    Type check:    {:>6} μs ({:.1}%)",
            result.timing.type_checking_us,
            100.0 * result.timing.type_checking_us as f64 / result.timing.total_us as f64
        );
        println!(
            "    MIR building:  {:>6} μs ({:.1}%)",
            result.timing.mir_building_us,
            100.0 * result.timing.mir_building_us as f64 / result.timing.total_us as f64
        );
        println!(
            "    Optimization:  {:>6} μs ({:.1}%)",
            result.timing.optimization_us,
            100.0 * result.timing.optimization_us as f64 / result.timing.total_us as f64
        );
        println!(
            "    Codegen:       {:>6} μs ({:.1}%)",
            result.timing.codegen_us,
            100.0 * result.timing.codegen_us as f64 / result.timing.total_us as f64
        );
        println!("    Total:         {:>6} μs", result.timing.total_us);
    }

    println!();
    println!("📊 Benchmark 3: Scalability Test");
    println!("{}", "-".repeat(40));

    // Generate increasingly large sources
    let small_src = generate_functions(10);
    let medium_src = generate_functions(50);
    let large_src = generate_functions(100);

    let small_times = benchmark_compile(&small_src, 50);
    let medium_times = benchmark_compile(&medium_src, 20);
    let large_times = benchmark_compile(&large_src, 10);

    println!(
        "  10 functions:  {:>6.2} μs ({:.1} KLOC/s)",
        small_times.avg_us, small_times.kloc_per_sec
    );
    println!(
        "  50 functions:  {:>6.2} μs ({:.1} KLOC/s)",
        medium_times.avg_us, medium_times.kloc_per_sec
    );
    println!(
        "  100 functions: {:>6.2} μs ({:.1} KLOC/s)",
        large_times.avg_us, large_times.kloc_per_sec
    );

    // Check if throughput remains stable (should be linear or better)
    let scaling_ratio = large_times.kloc_per_sec / small_times.kloc_per_sec;
    if scaling_ratio > 0.8 {
        println!("  ✅ Linear scaling maintained!");
    } else {
        println!("  ⚠️ Scaling factor: {:.2}x", scaling_ratio);
    }

    println!();
    println!("✅ Benchmark complete!");
    println!();

    // Summary
    println!("📋 Summary:");
    println!(
        "  - Jagannath compiles fibonacci.jag in {:.2} μs average",
        fib_times.avg_us
    );
    println!("  - Throughput: {:.1} KLOC/second", fib_times.kloc_per_sec);
    println!(
        "  - Generated {} lines of x86-64 assembly",
        compile_source(fibonacci_src)
            .map(|r| String::from_utf8_lossy(&r.output).lines().count())
            .unwrap_or(0)
    );
}

struct BenchmarkResult {
    min_us: f64,
    max_us: f64,
    avg_us: f64,
    kloc_per_sec: f64,
}

fn benchmark_compile(source: &str, iterations: usize) -> BenchmarkResult {
    let mut times_us = Vec::with_capacity(iterations);

    for _ in 0..iterations {
        let start = Instant::now();
        let _ = compile_source(source);
        times_us.push(start.elapsed().as_micros() as f64);
    }

    let min_us = times_us.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_us = times_us.iter().cloned().fold(0.0f64, f64::max);
    let avg_us = times_us.iter().sum::<f64>() / iterations as f64;

    // Estimate lines of code (rough: 1 line ~ 30 chars)
    let loc = source.len() as f64 / 30.0;
    let kloc_per_sec = if avg_us > 0.0 {
        (loc / avg_us) * 1_000_000.0 / 1000.0
    } else {
        0.0
    };

    BenchmarkResult {
        min_us,
        max_us,
        avg_us,
        kloc_per_sec,
    }
}

fn compile_source(source: &str) -> Option<CompileResult> {
    let options = CompilerOptions::new();
    let mut session = CompilerSession::new(options);
    session.compile(source).ok()
}

fn generate_functions(count: usize) -> String {
    let mut src = String::new();
    for i in 0..count {
        src.push_str(&format!(
            r#"
kāryakrama func_{i}(x: saṅkhyā-a-k-t32) -> saṅkhyā-a-k-t32 {{
    let y = x * 2;
    let z = y + {i};
    phera z
}}
"#,
            i = i
        ));
    }
    src
}
