//! # Kramaṇa - Sorting Algorithms (क्रमण)
//!
//! Comprehensive sorting algorithms with Sanskrit naming.
//!
//! > **"क्रमो हि सर्वत्र प्रशस्तः"**
//! > *"Order is praised everywhere"*
//!
//! ## Algorithms
//!
//! - [`tvarit_krama`] - Quicksort (त्वरित क्रम) - O(n log n) avg
//! - [`mishrit_krama`] - Mergesort (मिश्रित क्रम) - O(n log n) stable
//! - [`stambha_krama`] - Heapsort (स्तम्भ क्रम) - O(n log n) in-place
//! - [`pravisht_krama`] - Insertion sort (प्रविष्ट क्रम) - O(n²)
//! - [`chayan_krama`] - Selection sort (चयन क्रम) - O(n²)
//! - [`budbudaka_krama`] - Bubble sort (बुद्बुदक क्रम) - O(n²)
//! - [`ginti_krama`] - Counting sort (गिनती क्रम) - O(n+k)
//! - [`mulanka_krama`] - Radix sort (मूलांक क्रम) - O(nk)

use core::cmp::Ordering;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// QUICKSORT (त्वरित क्रम)
// ============================================================================

/// Quicksort - Fast average-case sorting (त्वरित क्रम)
///
/// A divide-and-conquer algorithm with O(n log n) average time.
///
/// # Etymology
/// त्वरित (tvarita) = swift, quick
/// क्रम (krama) = order, sequence
///
/// # Algorithm
/// 1. Choose pivot element
/// 2. Partition: elements < pivot go left, > pivot go right
/// 3. Recursively sort partitions
///
/// # Complexity
/// - Average: O(n log n)
/// - Worst: O(n²) - when pivot selection is poor
/// - Space: O(log n) for recursion
pub fn tvarit_krama<T: Ord>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    tvarit_krama_impl(arr, 0, arr.len() - 1);
}

fn tvarit_krama_impl<T: Ord>(arr: &mut [T], low: usize, high: usize) {
    if low < high {
        let pivot = vibhajana(arr, low, high);
        if pivot > 0 {
            tvarit_krama_impl(arr, low, pivot - 1);
        }
        tvarit_krama_impl(arr, pivot + 1, high);
    }
}

/// Partition function for quicksort (विभाजन)
///
/// Uses Lomuto partition scheme with last element as pivot.
fn vibhajana<T: Ord>(arr: &mut [T], low: usize, high: usize) -> usize {
    let pivot_idx = high;
    let mut i = low;

    for j in low..high {
        if arr[j] <= arr[pivot_idx] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}

/// Quicksort with custom comparator (त्वरित क्रम सतुलना)
pub fn tvarit_krama_by<T, F>(arr: &mut [T], compare: F)
where
    F: Fn(&T, &T) -> Ordering + Copy,
{
    if arr.len() <= 1 {
        return;
    }
    tvarit_krama_by_impl(arr, 0, arr.len() - 1, compare);
}

fn tvarit_krama_by_impl<T, F>(arr: &mut [T], low: usize, high: usize, compare: F)
where
    F: Fn(&T, &T) -> Ordering + Copy,
{
    if low < high {
        let pivot = vibhajana_by(arr, low, high, compare);
        if pivot > 0 {
            tvarit_krama_by_impl(arr, low, pivot - 1, compare);
        }
        tvarit_krama_by_impl(arr, pivot + 1, high, compare);
    }
}

fn vibhajana_by<T, F>(arr: &mut [T], low: usize, high: usize, compare: F) -> usize
where
    F: Fn(&T, &T) -> Ordering,
{
    let pivot_idx = high;
    let mut i = low;

    for j in low..high {
        if compare(&arr[j], &arr[pivot_idx]) != Ordering::Greater {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i
}

// ============================================================================
// MERGESORT (मिश्रित क्रम)
// ============================================================================

/// Mergesort - Stable O(n log n) sorting (मिश्रित क्रम)
///
/// A stable divide-and-conquer algorithm.
///
/// # Etymology
/// मिश्रित (miśrita) = mixed, merged
///
/// # Algorithm
/// 1. Divide array into two halves
/// 2. Recursively sort each half
/// 3. Merge sorted halves
///
/// # Complexity
/// - Time: O(n log n) all cases
/// - Space: O(n) auxiliary
#[cfg(feature = "alloc")]
pub fn mishrit_krama<T: Ord + Clone>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }
    let mid = arr.len() / 2;
    let mut left: Vec<T> = arr[..mid].to_vec();
    let mut right: Vec<T> = arr[mid..].to_vec();

    mishrit_krama(&mut left);
    mishrit_krama(&mut right);

    mishrana(&left, &right, arr);
}

/// Merge two sorted arrays (मिश्रण)
#[cfg(feature = "alloc")]
fn mishrana<T: Ord + Clone>(left: &[T], right: &[T], result: &mut [T]) {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result[k] = left[i].clone();
            i += 1;
        } else {
            result[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        result[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        result[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

// ============================================================================
// HEAPSORT (स्तम्भ क्रम)
// ============================================================================

/// Heapsort - In-place O(n log n) sorting (स्तम्भ क्रम)
///
/// Uses max-heap property for sorting.
///
/// # Etymology
/// स्तम्भ (stambha) = pillar, heap structure
///
/// # Algorithm
/// 1. Build max-heap from array
/// 2. Extract max element, rebuild heap
/// 3. Repeat until sorted
///
/// # Complexity
/// - Time: O(n log n) all cases
/// - Space: O(1) in-place
pub fn stambha_krama<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    if n <= 1 {
        return;
    }

    // Build max heap
    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    // Extract elements from heap
    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

/// Heapify subtree rooted at index (हीपीकृत)
fn heapify<T: Ord>(arr: &mut [T], n: usize, root: usize) {
    let mut largest = root;
    let left = 2 * root + 1;
    let right = 2 * root + 2;

    if left < n && arr[left] > arr[largest] {
        largest = left;
    }

    if right < n && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != root {
        arr.swap(root, largest);
        heapify(arr, n, largest);
    }
}

// ============================================================================
// INSERTION SORT (प्रविष्ट क्रम)
// ============================================================================

/// Insertion sort - Simple O(n²) sorting (प्रविष्ट क्रम)
///
/// Efficient for small arrays and nearly sorted data.
///
/// # Etymology
/// प्रविष्ट (praviṣṭa) = entered, inserted
///
/// # Complexity
/// - Best: O(n) - already sorted
/// - Average/Worst: O(n²)
/// - Space: O(1)
pub fn pravisht_krama<T: Ord>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}

// ============================================================================
// SELECTION SORT (चयन क्रम)
// ============================================================================

/// Selection sort - Simple O(n²) sorting (चयन क्रम)
///
/// Minimizes swaps, good when writes are expensive.
///
/// # Etymology
/// चयन (cayana) = selection, choosing
///
/// # Complexity
/// - Time: O(n²) all cases
/// - Space: O(1)
/// - Swaps: O(n) maximum
pub fn chayan_krama<T: Ord>(arr: &mut [T]) {
    for i in 0..arr.len() {
        let mut min_idx = i;
        for j in (i + 1)..arr.len() {
            if arr[j] < arr[min_idx] {
                min_idx = j;
            }
        }
        if min_idx != i {
            arr.swap(i, min_idx);
        }
    }
}

// ============================================================================
// BUBBLE SORT (बुद्बुदक क्रम)
// ============================================================================

/// Bubble sort - Simple O(n²) sorting (बुद्बुदक क्रम)
///
/// Educational algorithm, not for production use.
///
/// # Etymology
/// बुद्बुदक (budbudaka) = bubble, small bubble
///
/// # Complexity
/// - Best: O(n) - already sorted (with optimization)
/// - Average/Worst: O(n²)
pub fn budbudaka_krama<T: Ord>(arr: &mut [T]) {
    let n = arr.len();
    for i in 0..n {
        let mut swapped = false;
        for j in 0..(n - i - 1) {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break; // Already sorted
        }
    }
}

// ============================================================================
// COUNTING SORT (गिनती क्रम)
// ============================================================================

/// Counting sort - O(n+k) integer sorting (गिनती क्रम)
///
/// Efficient for integers in a small range.
///
/// # Etymology
/// गिनती (gintī) = counting
///
/// # Parameters
/// - `arr`: Input array of non-negative integers
/// - `max_val`: Maximum value in array
///
/// # Complexity
/// - Time: O(n + k) where k = max_val
/// - Space: O(k)
#[cfg(feature = "alloc")]
pub fn ginti_krama(arr: &mut [usize], max_val: usize) {
    if arr.is_empty() {
        return;
    }

    let mut count = vec![0usize; max_val + 1];

    // Count occurrences
    for &val in arr.iter() {
        count[val] += 1;
    }

    // Reconstruct sorted array
    let mut idx = 0;
    for (val, &cnt) in count.iter().enumerate() {
        for _ in 0..cnt {
            arr[idx] = val;
            idx += 1;
        }
    }
}

// ============================================================================
// RADIX SORT (मूलांक क्रम)
// ============================================================================

/// Radix sort - O(nk) digit-based sorting (मूलांक क्रम)
///
/// Sorts by processing digits from least to most significant.
///
/// # Etymology
/// मूलांक (mūlāṅka) = radix, base digit
///
/// # Complexity
/// - Time: O(n × k) where k = number of digits
/// - Space: O(n + b) where b = base
#[cfg(feature = "alloc")]
pub fn mulanka_krama(arr: &mut [u64]) {
    if arr.is_empty() {
        return;
    }

    let max_val = *arr.iter().max().unwrap();
    let mut exp = 1u64;

    while max_val / exp > 0 {
        ginti_krama_for_radix(arr, exp);
        exp *= 10;
    }
}

#[cfg(feature = "alloc")]
fn ginti_krama_for_radix(arr: &mut [u64], exp: u64) {
    let n = arr.len();
    let mut output = vec![0u64; n];
    let mut count = [0usize; 10];

    // Count occurrences of digits
    for &val in arr.iter() {
        let digit = ((val / exp) % 10) as usize;
        count[digit] += 1;
    }

    // Change count[i] to position of digit in output
    for i in 1..10 {
        count[i] += count[i - 1];
    }

    // Build output array (stable, right to left)
    for i in (0..n).rev() {
        let digit = ((arr[i] / exp) % 10) as usize;
        count[digit] -= 1;
        output[count[digit]] = arr[i];
    }

    arr.copy_from_slice(&output);
}

// ============================================================================
// INTROSORT (आत्मनिरीक्षण क्रम)
// ============================================================================

/// Introsort - Hybrid quicksort (आत्मनिरीक्षण क्रम)
///
/// Combines quicksort, heapsort, and insertion sort.
/// Used by many standard library implementations.
///
/// # Etymology
/// आत्मनिरीक्षण (ātma-nirīkṣaṇa) = introspection
///
/// # Algorithm
/// - Start with quicksort
/// - Switch to heapsort if recursion too deep
/// - Use insertion sort for small subarrays
pub fn atma_nirikshan_krama<T: Ord>(arr: &mut [T]) {
    let max_depth = (arr.len() as f64).log2() as usize * 2;
    introsort_impl(arr, max_depth);
}

fn introsort_impl<T: Ord>(arr: &mut [T], depth_limit: usize) {
    let n = arr.len();

    // Use insertion sort for small arrays
    if n <= 16 {
        pravisht_krama(arr);
        return;
    }

    // Switch to heapsort if recursion too deep
    if depth_limit == 0 {
        stambha_krama(arr);
        return;
    }

    // Otherwise, quicksort
    let pivot = vibhajana(arr, 0, n - 1);

    if pivot > 0 {
        introsort_impl(&mut arr[..pivot], depth_limit - 1);
    }
    introsort_impl(&mut arr[pivot + 1..], depth_limit - 1);
}

// ============================================================================
// UTILITY FUNCTIONS
// ============================================================================

/// Check if array is sorted (क्रमबद्ध जाँच)
pub fn is_kramabaddha<T: Ord>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] <= w[1])
}

/// Check if array is sorted in reverse (विपरीत क्रमबद्ध)
pub fn is_viparit_kramabaddha<T: Ord>(arr: &[T]) -> bool {
    arr.windows(2).all(|w| w[0] >= w[1])
}

/// Reverse array in place (उलटना)
pub fn ulatan<T>(arr: &mut [T]) {
    arr.reverse();
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tvarit_krama() {
        let mut arr = vec![64, 34, 25, 12, 22, 11, 90];
        tvarit_krama(&mut arr);
        assert!(is_kramabaddha(&arr));
        assert_eq!(arr, vec![11, 12, 22, 25, 34, 64, 90]);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_mishrit_krama() {
        let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
        mishrit_krama(&mut arr);
        assert!(is_kramabaddha(&arr));
        assert_eq!(arr, vec![3, 9, 10, 27, 38, 43, 82]);
    }

    #[test]
    fn test_stambha_krama() {
        let mut arr = vec![12, 11, 13, 5, 6, 7];
        stambha_krama(&mut arr);
        assert!(is_kramabaddha(&arr));
        assert_eq!(arr, vec![5, 6, 7, 11, 12, 13]);
    }

    #[test]
    fn test_pravisht_krama() {
        let mut arr = vec![5, 2, 4, 6, 1, 3];
        pravisht_krama(&mut arr);
        assert!(is_kramabaddha(&arr));
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn test_chayan_krama() {
        let mut arr = vec![64, 25, 12, 22, 11];
        chayan_krama(&mut arr);
        assert!(is_kramabaddha(&arr));
        assert_eq!(arr, vec![11, 12, 22, 25, 64]);
    }

    #[test]
    fn test_budbudaka_krama() {
        let mut arr = vec![5, 1, 4, 2, 8];
        budbudaka_krama(&mut arr);
        assert!(is_kramabaddha(&arr));
        assert_eq!(arr, vec![1, 2, 4, 5, 8]);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_ginti_krama() {
        let mut arr = vec![4, 2, 2, 8, 3, 3, 1];
        ginti_krama(&mut arr, 8);
        assert!(is_kramabaddha(&arr));
        assert_eq!(arr, vec![1, 2, 2, 3, 3, 4, 8]);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_mulanka_krama() {
        let mut arr: Vec<u64> = vec![170, 45, 75, 90, 802, 24, 2, 66];
        mulanka_krama(&mut arr);
        assert!(is_kramabaddha(&arr));
        assert_eq!(arr, vec![2, 24, 45, 66, 75, 90, 170, 802]);
    }

    #[test]
    fn test_atma_nirikshan_krama() {
        let mut arr = vec![9, 7, 5, 11, 12, 2, 14, 3, 10, 6];
        atma_nirikshan_krama(&mut arr);
        assert!(is_kramabaddha(&arr));
    }

    #[test]
    fn test_empty_arrays() {
        let mut empty: Vec<i32> = vec![];
        tvarit_krama(&mut empty);
        assert!(empty.is_empty());

        stambha_krama(&mut empty);
        assert!(empty.is_empty());
    }

    #[test]
    fn test_single_element() {
        let mut single = vec![42];
        tvarit_krama(&mut single);
        assert_eq!(single, vec![42]);
    }

    #[test]
    fn test_already_sorted() {
        let mut sorted = vec![1, 2, 3, 4, 5];
        budbudaka_krama(&mut sorted);
        assert_eq!(sorted, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_reverse_sorted() {
        let mut reverse = vec![5, 4, 3, 2, 1];
        tvarit_krama(&mut reverse);
        assert_eq!(reverse, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_custom_comparator() {
        let mut arr = vec![5, 2, 8, 1, 9];
        tvarit_krama_by(&mut arr, |a, b| b.cmp(a)); // Descending
        assert!(is_viparit_kramabaddha(&arr));
        assert_eq!(arr, vec![9, 8, 5, 2, 1]);
    }
}
