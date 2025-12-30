//! # Kramana - Sorting Algorithms (क्रमण)
//!
//! Various sorting algorithms and utilities.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Check if slice is sorted ascending
pub fn kramit_hai<T: Ord>(slice: &[T]) -> bool {
    slice.windows(2).all(|w| w[0] <= w[1])
}

/// Check if sorted descending
pub fn avakramit_hai<T: Ord>(slice: &[T]) -> bool {
    slice.windows(2).all(|w| w[0] >= w[1])
}

/// Insertion sort (stable, in-place, O(n²))
pub fn avesha_kramana<T: Ord>(slice: &mut [T]) {
    for i in 1..slice.len() {
        let mut j = i;
        while j > 0 && slice[j - 1] > slice[j] {
            slice.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Selection sort (in-place, O(n²))
pub fn chayan_kramana<T: Ord>(slice: &mut [T]) {
    for i in 0..slice.len() {
        let mut min_idx = i;
        for j in (i + 1)..slice.len() {
            if slice[j] < slice[min_idx] {
                min_idx = j;
            }
        }
        if min_idx != i {
            slice.swap(i, min_idx);
        }
    }
}

/// Bubble sort (stable, in-place, O(n²))
pub fn bubuda_kramana<T: Ord>(slice: &mut [T]) {
    let n = slice.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..(n - 1 - i) {
            if slice[j] > slice[j + 1] {
                slice.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

/// Shell sort (in-place, O(n log n) to O(n²))
pub fn khol_kramana<T: Ord>(slice: &mut [T]) {
    let n = slice.len();
    let mut gap = n / 2;

    while gap > 0 {
        for i in gap..n {
            let mut j = i;
            while j >= gap && slice[j - gap] > slice[j] {
                slice.swap(j - gap, j);
                j -= gap;
            }
        }
        gap /= 2;
    }
}

/// Heap sort helper - sift down
fn niche_chhan<T: Ord>(heap: &mut [T], root: usize, end: usize) {
    let mut root = root;

    loop {
        let child = 2 * root + 1;
        if child > end {
            break;
        }

        let mut swap = root;
        if heap[swap] < heap[child] {
            swap = child;
        }
        if child + 1 <= end && heap[swap] < heap[child + 1] {
            swap = child + 1;
        }

        if swap == root {
            break;
        }

        heap.swap(root, swap);
        root = swap;
    }
}

/// Heap sort (in-place, O(n log n))
pub fn dheri_kramana<T: Ord>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }

    // Build heap
    let end = slice.len() - 1;
    let mut start = (end - 1) / 2;

    loop {
        niche_chhan(slice, start, end);
        if start == 0 {
            break;
        }
        start -= 1;
    }

    // Extract elements
    let mut end = slice.len() - 1;
    while end > 0 {
        slice.swap(0, end);
        end -= 1;
        niche_chhan(slice, 0, end);
    }
}

/// Quick sort partition
fn vibhajan<T: Ord>(slice: &mut [T]) -> usize {
    let pivot_idx = slice.len() / 2;
    slice.swap(pivot_idx, slice.len() - 1);

    let mut store = 0;
    let last = slice.len() - 1;

    for i in 0..last {
        if slice[i] < slice[last] {
            slice.swap(i, store);
            store += 1;
        }
    }

    slice.swap(store, last);
    store
}

/// Quick sort (in-place, O(n log n) average)
pub fn shighra_kramana<T: Ord>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }

    let pivot = vibhajan(slice);

    if pivot > 0 {
        shighra_kramana(&mut slice[..pivot]);
    }
    if pivot + 1 < slice.len() {
        shighra_kramana(&mut slice[pivot + 1..]);
    }
}

/// Merge sort (stable, O(n log n), requires allocation)
#[cfg(feature = "alloc")]
pub fn milana_kramana<T: Ord + Clone>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }

    let mid = slice.len() / 2;
    milana_kramana(&mut slice[..mid]);
    milana_kramana(&mut slice[mid..]);

    // Merge
    let left: Vec<T> = slice[..mid].to_vec();
    let right: Vec<T> = slice[mid..].to_vec();

    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            slice[k] = left[i].clone();
            i += 1;
        } else {
            slice[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        slice[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        slice[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

/// Counting sort for u8
#[cfg(feature = "alloc")]
pub fn ginti_kramana_u8(slice: &mut [u8]) {
    if slice.is_empty() {
        return;
    }

    let mut count = [0usize; 256];

    for &x in slice.iter() {
        count[x as usize] += 1;
    }

    let mut idx = 0;
    for (val, &cnt) in count.iter().enumerate() {
        for _ in 0..cnt {
            slice[idx] = val as u8;
            idx += 1;
        }
    }
}

/// Radix sort for u32
#[cfg(feature = "alloc")]
pub fn mula_kramana_u32(slice: &mut [u32]) {
    if slice.is_empty() {
        return;
    }

    let max_val = *slice.iter().max().unwrap();
    let mut exp = 1u32;

    let mut output = vec![0u32; slice.len()];

    while max_val / exp > 0 {
        let mut count = [0usize; 10];

        for &x in slice.iter() {
            let digit = ((x / exp) % 10) as usize;
            count[digit] += 1;
        }

        for i in 1..10 {
            count[i] += count[i - 1];
        }

        for &x in slice.iter().rev() {
            let digit = ((x / exp) % 10) as usize;
            count[digit] -= 1;
            output[count[digit]] = x;
        }

        slice.copy_from_slice(&output);
        exp *= 10;
    }
}

/// Partial sort - get k smallest elements (unsorted)
pub fn aamsik_kramana<T: Ord>(slice: &mut [T], k: usize) {
    if k >= slice.len() {
        return;
    }

    // Use quickselect
    let mut lo = 0;
    let mut hi = slice.len() - 1;

    while lo < hi {
        let pivot = vibhajan(&mut slice[lo..=hi]) + lo;

        if pivot == k {
            break;
        } else if pivot < k {
            lo = pivot + 1;
        } else {
            hi = pivot - 1;
        }
    }
}

/// Find k-th smallest element (0-indexed)
pub fn kramank<T: Ord + Clone>(slice: &mut [T], k: usize) -> Option<T> {
    if k >= slice.len() {
        return None;
    }

    aamsik_kramana(slice, k);
    Some(slice[k].clone())
}

/// Sort by key
pub fn kunji_kramana<T, K: Ord, F>(slice: &mut [T], mut key: F)
where
    F: FnMut(&T) -> K,
{
    for i in 1..slice.len() {
        let mut j = i;
        while j > 0 && key(&slice[j - 1]) > key(&slice[j]) {
            slice.swap(j - 1, j);
            j -= 1;
        }
    }
}

/// Reverse slice in-place
pub fn ulat<T>(slice: &mut [T]) {
    slice.reverse();
}

/// Rotate left by k positions
pub fn ghumao_vaama<T>(slice: &mut [T], k: usize) {
    if slice.is_empty() {
        return;
    }
    let k = k % slice.len();
    slice[..k].reverse();
    slice[k..].reverse();
    slice.reverse();
}

/// Rotate right by k positions
pub fn ghumao_dakshina<T>(slice: &mut [T], k: usize) {
    if slice.is_empty() {
        return;
    }
    let k = k % slice.len();
    ghumao_vaama(slice, slice.len() - k);
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion_sort() {
        let mut arr = [5, 2, 8, 1, 9, 3];
        avesha_kramana(&mut arr);
        assert!(kramit_hai(&arr));
    }

    #[test]
    fn test_heap_sort() {
        let mut arr = [5, 2, 8, 1, 9, 3];
        dheri_kramana(&mut arr);
        assert!(kramit_hai(&arr));
    }

    #[test]
    fn test_quick_sort() {
        let mut arr = [5, 2, 8, 1, 9, 3, 7, 4, 6];
        shighra_kramana(&mut arr);
        assert!(kramit_hai(&arr));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_merge_sort() {
        let mut arr = [5, 2, 8, 1, 9, 3];
        milana_kramana(&mut arr);
        assert!(kramit_hai(&arr));
    }

    #[test]
    fn test_rotation() {
        let mut arr = [1, 2, 3, 4, 5];
        ghumao_vaama(&mut arr, 2);
        assert_eq!(arr, [3, 4, 5, 1, 2]);
    }
}
