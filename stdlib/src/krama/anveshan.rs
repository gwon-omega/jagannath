//! # Anveṣaṇa - Searching Algorithms (अन्वेषण)
//!
//! Efficient search algorithms for various data structures.
//!
//! > **"अन्वेषणं ज्ञानस्य द्वारम्"**
//! > *"Search is the gateway to knowledge"*
//!
//! ## Algorithms
//!
//! - [`rekha_anveshan`] - Linear search (रेखा अन्वेषण)
//! - [`dvidha_anveshan`] - Binary search (द्विधा अन्वेषण)
//! - [`pratishambha_anveshan`] - Interpolation search (प्रतिसम्भा)
//! - [`ucchalan_anveshan`] - Jump search (उच्चलन अन्वेषण)
//! - [`charghatan_anveshan`] - Exponential search (चरघातन)

use core::cmp::Ordering;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// LINEAR SEARCH (रेखा अन्वेषण)
// ============================================================================

/// Linear search - O(n) sequential search (रेखा अन्वेषण)
///
/// Searches element by element from start to end.
/// Works on any array, sorted or unsorted.
///
/// # Etymology
/// रेखा (rekhā) = line (sequential)
/// अन्वेषण (anveṣaṇa) = search, investigation
///
/// # Returns
/// `Some(index)` if found, `None` otherwise
///
/// # Complexity
/// - Time: O(n)
/// - Space: O(1)
pub fn rekha_anveshan<T: PartialEq>(arr: &[T], lakshya: &T) -> Option<usize> {
    for (i, item) in arr.iter().enumerate() {
        if item == lakshya {
            return Some(i);
        }
    }
    None
}

/// Linear search returning all indices (सर्व रेखा अन्वेषण)
#[cfg(feature = "alloc")]
pub fn sarva_rekha_anveshan<T: PartialEq>(arr: &[T], lakshya: &T) -> Vec<usize> {
    arr.iter()
        .enumerate()
        .filter(|(_, item)| *item == lakshya)
        .map(|(i, _)| i)
        .collect()
}

/// Linear search with predicate (विधेय अन्वेषण)
pub fn vidheyai_anveshan<T, F>(arr: &[T], predicate: F) -> Option<usize>
where
    F: Fn(&T) -> bool,
{
    for (i, item) in arr.iter().enumerate() {
        if predicate(item) {
            return Some(i);
        }
    }
    None
}

// ============================================================================
// BINARY SEARCH (द्विधा अन्वेषण)
// ============================================================================

/// Binary search - O(log n) on sorted arrays (द्विधा अन्वेषण)
///
/// Divides search space in half each iteration.
/// Requires sorted array.
///
/// # Etymology
/// द्विधा (dvidhā) = in two ways, binary
///
/// # Returns
/// `Ok(index)` if found exactly
/// `Err(index)` where element would be inserted
///
/// # Complexity
/// - Time: O(log n)
/// - Space: O(1)
pub fn dvidha_anveshan<T: Ord>(arr: &[T], lakshya: &T) -> Result<usize, usize> {
    if arr.is_empty() {
        return Err(0);
    }

    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;
        match arr[mid].cmp(lakshya) {
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid,
            Ordering::Equal => return Ok(mid),
        }
    }

    Err(low)
}

/// Binary search with custom comparator
pub fn dvidha_anveshan_by<T, F>(arr: &[T], lakshya: &T, compare: F) -> Result<usize, usize>
where
    F: Fn(&T, &T) -> Ordering,
{
    if arr.is_empty() {
        return Err(0);
    }

    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;
        match compare(&arr[mid], lakshya) {
            Ordering::Less => low = mid + 1,
            Ordering::Greater => high = mid,
            Ordering::Equal => return Ok(mid),
        }
    }

    Err(low)
}

/// Find lower bound - first element >= target (निम्न सीमा)
pub fn nimna_sima<T: Ord>(arr: &[T], lakshya: &T) -> usize {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;
        if arr[mid] < *lakshya {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

/// Find upper bound - first element > target (उच्च सीमा)
pub fn ucca_sima<T: Ord>(arr: &[T], lakshya: &T) -> usize {
    let mut low = 0;
    let mut high = arr.len();

    while low < high {
        let mid = low + (high - low) / 2;
        if arr[mid] <= *lakshya {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    low
}

/// Count occurrences using binary search (गणना)
pub fn ganana<T: Ord>(arr: &[T], lakshya: &T) -> usize {
    ucca_sima(arr, lakshya) - nimna_sima(arr, lakshya)
}

// ============================================================================
// INTERPOLATION SEARCH (प्रतिसम्भा अन्वेषण)
// ============================================================================

/// Interpolation search - O(log log n) for uniform distribution (प्रतिसम्भा)
///
/// Estimates position based on value distribution.
/// Best for uniformly distributed data.
///
/// # Etymology
/// प्रतिसम्भा (pratisambhā) = interpolation, estimation
///
/// # Complexity
/// - Best/Average: O(log log n) for uniform data
/// - Worst: O(n) for skewed distribution
pub fn pratisambha_anveshan(arr: &[i64], lakshya: i64) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high && lakshya >= arr[low] && lakshya <= arr[high] {
        if low == high {
            if arr[low] == lakshya {
                return Some(low);
            }
            return None;
        }

        // Interpolate position
        let range = arr[high] - arr[low];
        if range == 0 {
            if arr[low] == lakshya {
                return Some(low);
            }
            return None;
        }

        let pos = low + (((lakshya - arr[low]) as usize * (high - low)) / range as usize);
        let pos = pos.min(high).max(low);

        if arr[pos] == lakshya {
            return Some(pos);
        } else if arr[pos] < lakshya {
            low = pos + 1;
        } else {
            if pos == 0 {
                return None;
            }
            high = pos - 1;
        }
    }

    None
}

// ============================================================================
// JUMP SEARCH (उच्चलन अन्वेषण)
// ============================================================================

/// Jump search - O(√n) block-based search (उच्चलन अन्वेषण)
///
/// Jumps ahead by fixed steps, then linear search.
/// Balance between linear and binary search.
///
/// # Etymology
/// उच्चलन (uccalana) = jumping, leaping
///
/// # Complexity
/// - Time: O(√n)
/// - Space: O(1)
pub fn ucchalan_anveshan<T: Ord>(arr: &[T], lakshya: &T) -> Option<usize> {
    let n = arr.len();
    if n == 0 {
        return None;
    }

    let step = (n as f64).sqrt() as usize;
    let step = step.max(1);

    // Jump ahead
    let mut prev = 0;
    let mut curr = step;

    while curr < n && arr[curr] < *lakshya {
        prev = curr;
        curr += step;
        if curr >= n {
            curr = n;
        }
    }

    // Linear search in block
    for i in prev..curr.min(n) {
        if arr[i] == *lakshya {
            return Some(i);
        }
    }

    None
}

// ============================================================================
// EXPONENTIAL SEARCH (चरघातन अन्वेषण)
// ============================================================================

/// Exponential search - O(log n) with unknown size (चरघातन अन्वेषण)
///
/// Finds range exponentially, then binary search.
/// Good for unbounded/infinite arrays.
///
/// # Etymology
/// चरघातन (caraghātana) = exponential
///
/// # Complexity
/// - Time: O(log n)
/// - Space: O(1)
pub fn charghatan_anveshan<T: Ord>(arr: &[T], lakshya: &T) -> Option<usize> {
    let n = arr.len();
    if n == 0 {
        return None;
    }

    if arr[0] == *lakshya {
        return Some(0);
    }

    // Find range by exponential jumps
    let mut i = 1;
    while i < n && arr[i] <= *lakshya {
        i *= 2;
    }

    // Binary search in found range
    let low = i / 2;
    let high = i.min(n);

    match dvidha_anveshan(&arr[low..high], lakshya) {
        Ok(idx) => Some(low + idx),
        Err(_) => None,
    }
}

// ============================================================================
// TERNARY SEARCH (त्रिधा अन्वेषण)
// ============================================================================

/// Ternary search for unimodal function maximum (त्रिधा अन्वेषण)
///
/// Finds maximum of unimodal function by dividing in thirds.
///
/// # Etymology
/// त्रिधा (tridhā) = in three ways, ternary
///
/// # Returns
/// x value where f(x) is maximum
pub fn tridha_anveshan<F>(mut low: f64, mut high: f64, epsilon: f64, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    while (high - low) > epsilon {
        let mid1 = low + (high - low) / 3.0;
        let mid2 = high - (high - low) / 3.0;

        if f(mid1) < f(mid2) {
            low = mid1;
        } else {
            high = mid2;
        }
    }

    (low + high) / 2.0
}

/// Ternary search for minimum
pub fn tridha_anveshan_nyunatam<F>(low: f64, high: f64, epsilon: f64, f: F) -> f64
where
    F: Fn(f64) -> f64,
{
    tridha_anveshan(low, high, epsilon, |x| -f(x))
}

// ============================================================================
// PEAK FINDING (शिखर अन्वेषण)
// ============================================================================

/// Find a peak element in array (शिखर अन्वेषण)
///
/// A peak is an element greater than its neighbors.
///
/// # Etymology
/// शिखर (śikhara) = peak, summit
///
/// # Complexity
/// - Time: O(log n)
pub fn shikhara_anveshan<T: Ord>(arr: &[T]) -> Option<usize> {
    let n = arr.len();
    if n == 0 {
        return None;
    }
    if n == 1 {
        return Some(0);
    }

    let mut low = 0;
    let mut high = n - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        let is_left_smaller = mid == 0 || arr[mid - 1] <= arr[mid];
        let is_right_smaller = mid == n - 1 || arr[mid + 1] <= arr[mid];

        if is_left_smaller && is_right_smaller {
            return Some(mid);
        } else if mid > 0 && arr[mid - 1] > arr[mid] {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }

    None
}

// ============================================================================
// KTH ELEMENT (क-तम तत्त्व)
// ============================================================================

/// Find kth smallest element - O(n) average (क-तम तत्त्व)
///
/// Uses quickselect algorithm.
///
/// # Etymology
/// क-तम (ka-tama) = which-th, kth
/// तत्त्व (tattva) = element
pub fn ka_tama_tattva<T: Ord + Clone>(arr: &mut [T], k: usize) -> Option<T> {
    if k >= arr.len() {
        return None;
    }
    Some(quickselect(arr, k))
}

fn quickselect<T: Ord + Clone>(arr: &mut [T], k: usize) -> T {
    if arr.len() == 1 {
        return arr[0].clone();
    }

    let pivot_idx = partition_quickselect(arr);

    if k == pivot_idx {
        arr[k].clone()
    } else if k < pivot_idx {
        quickselect(&mut arr[..pivot_idx], k)
    } else {
        quickselect(&mut arr[pivot_idx + 1..], k - pivot_idx - 1)
    }
}

fn partition_quickselect<T: Ord>(arr: &mut [T]) -> usize {
    let pivot_idx = arr.len() - 1;
    let mut i = 0;

    for j in 0..pivot_idx {
        if arr[j] <= arr[pivot_idx] {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, pivot_idx);
    i
}

/// Find median (मध्यम)
pub fn madhyama<T: Ord + Clone>(arr: &mut [T]) -> Option<T> {
    if arr.is_empty() {
        return None;
    }
    ka_tama_tattva(arr, arr.len() / 2)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rekha_anveshan() {
        let arr = vec![10, 20, 30, 40, 50];
        assert_eq!(rekha_anveshan(&arr, &30), Some(2));
        assert_eq!(rekha_anveshan(&arr, &35), None);
        assert_eq!(rekha_anveshan(&arr, &10), Some(0));
        assert_eq!(rekha_anveshan(&arr, &50), Some(4));
    }

    #[test]
    fn test_dvidha_anveshan() {
        let arr = vec![1, 3, 5, 7, 9, 11, 13];
        assert_eq!(dvidha_anveshan(&arr, &7), Ok(3));
        assert_eq!(dvidha_anveshan(&arr, &1), Ok(0));
        assert_eq!(dvidha_anveshan(&arr, &13), Ok(6));
        assert_eq!(dvidha_anveshan(&arr, &6), Err(3)); // Would insert at index 3
    }

    #[test]
    fn test_nimna_ucca_sima() {
        let arr = vec![1, 2, 2, 2, 3, 3, 5];

        assert_eq!(nimna_sima(&arr, &2), 1); // First 2
        assert_eq!(ucca_sima(&arr, &2), 4); // First element > 2
        assert_eq!(ganana(&arr, &2), 3); // Three 2s
        assert_eq!(ganana(&arr, &3), 2); // Two 3s
        assert_eq!(ganana(&arr, &4), 0); // No 4s
    }

    #[test]
    fn test_pratisambha_anveshan() {
        // Uniformly distributed
        let arr: Vec<i64> = vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100];
        assert_eq!(pratisambha_anveshan(&arr, 70), Some(6));
        assert_eq!(pratisambha_anveshan(&arr, 10), Some(0));
        assert_eq!(pratisambha_anveshan(&arr, 100), Some(9));
        assert_eq!(pratisambha_anveshan(&arr, 55), None);
    }

    #[test]
    fn test_ucchalan_anveshan() {
        let arr = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89];
        assert_eq!(ucchalan_anveshan(&arr, &55), Some(10));
        assert_eq!(ucchalan_anveshan(&arr, &0), Some(0));
        assert_eq!(ucchalan_anveshan(&arr, &89), Some(11));
        assert_eq!(ucchalan_anveshan(&arr, &50), None);
    }

    #[test]
    fn test_charghatan_anveshan() {
        let arr = vec![2, 3, 4, 10, 40];
        assert_eq!(charghatan_anveshan(&arr, &10), Some(3));
        assert_eq!(charghatan_anveshan(&arr, &2), Some(0));
        assert_eq!(charghatan_anveshan(&arr, &40), Some(4));
        assert_eq!(charghatan_anveshan(&arr, &5), None);
    }

    #[test]
    fn test_tridha_anveshan() {
        // Find maximum of -(x-3)² + 10, max at x=3
        let max_x = tridha_anveshan(0.0, 6.0, 0.001, |x| -(x - 3.0).powi(2) + 10.0);
        assert!((max_x - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_shikhara_anveshan() {
        let arr = vec![1, 3, 20, 4, 1, 0];
        let peak = shikhara_anveshan(&arr);
        assert!(peak.is_some());
        let idx = peak.unwrap();
        // Peak should be 20 at index 2
        assert!(idx == 0 || arr[idx] >= arr[idx - 1]);
        assert!(idx == arr.len() - 1 || arr[idx] >= arr[idx + 1]);
    }

    #[test]
    fn test_ka_tama_tattva() {
        let mut arr = vec![7, 10, 4, 3, 20, 15];

        // 0th smallest (minimum)
        let k0 = ka_tama_tattva(&mut arr.clone(), 0);
        assert_eq!(k0, Some(3));

        // 2nd smallest (third element when sorted)
        let k2 = ka_tama_tattva(&mut arr.clone(), 2);
        assert_eq!(k2, Some(7));
    }

    #[test]
    fn test_madhyama() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        // Sorted: [1, 1, 2, 3, 4, 5, 6, 9], median at index 4 is 4
        let median = madhyama(&mut arr);
        assert!(median.is_some());
    }

    #[test]
    fn test_empty_array() {
        let empty: Vec<i32> = vec![];
        assert_eq!(rekha_anveshan(&empty, &5), None);
        assert_eq!(dvidha_anveshan(&empty, &5), Err(0));
        assert_eq!(ucchalan_anveshan(&empty, &5), None);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_sarva_rekha_anveshan() {
        let arr = vec![1, 2, 3, 2, 4, 2, 5];
        let indices = sarva_rekha_anveshan(&arr, &2);
        assert_eq!(indices, vec![1, 3, 5]);
    }
}
