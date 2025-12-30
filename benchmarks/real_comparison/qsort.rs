// QuickSort benchmark - Rust
use std::time::Instant;

const SIZE: usize = 1000000;

fn quicksort(a: &mut [i32], low: i32, high: i32) {
    if low < high {
        let pivot = a[high as usize];
        let mut i = low - 1;

        for j in low..high {
            if a[j as usize] <= pivot {
                i += 1;
                a.swap(i as usize, j as usize);
            }
        }
        a.swap((i + 1) as usize, high as usize);

        let pi = i + 1;
        quicksort(a, low, pi - 1);
        quicksort(a, pi + 1, high);
    }
}

fn main() {
    let mut arr = vec![0i32; SIZE];

    let start = Instant::now();

    for run in 0..5 {
        // Simple LCG random number generator
        let mut seed: u64 = 42 + run;
        for i in 0..SIZE {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            arr[i] = ((seed >> 16) % 1000000) as i32;
        }
        quicksort(&mut arr, 0, (SIZE - 1) as i32);
    }

    let elapsed = start.elapsed();
    println!(
        "Rust: QuickSort {} elements, Time: {:.2} ms (5 runs)",
        SIZE,
        elapsed.as_secs_f64() * 1000.0
    );
}
