// Fibonacci benchmark - Rust
use std::time::Instant;

fn fib(n: i32) -> i64 {
    if n <= 1 {
        return n as i64;
    }
    fib(n - 1) + fib(n - 2)
}

fn main() {
    let start = Instant::now();

    let mut result: i64 = 0;
    for _ in 0..5 {
        result = fib(40);
    }

    let elapsed = start.elapsed();
    println!(
        "Rust: fib(40) = {}, Time: {:.2} ms (5 runs)",
        result,
        elapsed.as_secs_f64() * 1000.0
    );
}
