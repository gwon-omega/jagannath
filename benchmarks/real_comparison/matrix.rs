// Matrix multiplication benchmark - Rust
use std::time::Instant;

const N: usize = 512;

fn matrix_mult(a: &[[f64; N]; N], b: &[[f64; N]; N], c: &mut [[f64; N]; N]) {
    for i in 0..N {
        for j in 0..N {
            let mut sum = 0.0;
            for k in 0..N {
                sum += a[i][k] * b[k][j];
            }
            c[i][j] = sum;
        }
    }
}

fn main() {
    let mut a = [[0.0f64; N]; N];
    let mut b = [[0.0f64; N]; N];
    let mut c = [[0.0f64; N]; N];

    // Initialize
    for i in 0..N {
        for j in 0..N {
            a[i][j] = (i + j) as f64 / N as f64;
            b[i][j] = (i as i64 - j as i64) as f64 / N as f64;
        }
    }

    let start = Instant::now();

    for _ in 0..3 {
        matrix_mult(&a, &b, &mut c);
    }

    let elapsed = start.elapsed();
    println!(
        "Rust: {}x{} matrix mult, Time: {:.2} ms (3 runs), C[0][0]={:.4}",
        N,
        N,
        elapsed.as_secs_f64() * 1000.0,
        c[0][0]
    );
}
