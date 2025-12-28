//! Prādhānyatā - Priority Queue / Heap (प्राधान्यता)
//!
//! A priority queue implemented as a binary heap.
//! Named after the Sanskrit concept of primacy/priority.

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use core::cmp::Ord;

/// Binary heap / Priority queue (Prādhānyatā-Śreṇī - प्राधान्यताश्रेणी)
///
/// A max-heap where the highest priority element is at the top.
#[cfg(feature = "alloc")]
pub struct PradhanyataShreni<T> {
    /// Data storage (तथ्य - tathya)
    tathya: Vec<T>,
}

#[cfg(feature = "alloc")]
impl<T: Ord> PradhanyataShreni<T> {
    /// Create empty heap (नव - nava)
    pub fn nava() -> Self {
        Self {
            tathya: Vec::new(),
        }
    }

    /// Create with capacity (क्षमता - kṣamatā)
    pub fn kshamata(capacity: usize) -> Self {
        Self {
            tathya: Vec::with_capacity(capacity),
        }
    }

    /// Is empty (रिक्त - rikta)
    pub fn rikta(&self) -> bool {
        self.tathya.is_empty()
    }

    /// Length (दीर्घता - dīrghatā)
    pub fn dirghata(&self) -> usize {
        self.tathya.len()
    }

    /// Push item (योजय - yojaya)
    pub fn yojaya(&mut self, item: T) {
        self.tathya.push(item);
        self.sift_up(self.tathya.len() - 1);
    }

    /// Pop highest priority item (निष्कासय - niṣkāsaya)
    pub fn nishkasaya(&mut self) -> Option<T> {
        if self.tathya.is_empty() {
            return None;
        }

        let last_idx = self.tathya.len() - 1;
        self.tathya.swap(0, last_idx);
        let result = self.tathya.pop();

        if !self.tathya.is_empty() {
            self.sift_down(0);
        }

        result
    }

    /// Peek at highest priority item (दृश् - dṛś)
    pub fn drsh(&self) -> Option<&T> {
        self.tathya.first()
    }

    /// Clear (शुद्ध - śuddha)
    pub fn shuddha(&mut self) {
        self.tathya.clear();
    }

    // Internal: sift up to maintain heap property
    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.tathya[idx] <= self.tathya[parent] {
                break;
            }
            self.tathya.swap(idx, parent);
            idx = parent;
        }
    }

    // Internal: sift down to maintain heap property
    fn sift_down(&mut self, mut idx: usize) {
        let len = self.tathya.len();

        loop {
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;
            let mut largest = idx;

            if left < len && self.tathya[left] > self.tathya[largest] {
                largest = left;
            }
            if right < len && self.tathya[right] > self.tathya[largest] {
                largest = right;
            }

            if largest == idx {
                break;
            }

            self.tathya.swap(idx, largest);
            idx = largest;
        }
    }
}

#[cfg(feature = "alloc")]
impl<T: Ord> Default for PradhanyataShreni<T> {
    fn default() -> Self {
        Self::nava()
    }
}

#[cfg(feature = "alloc")]
impl<T: Ord> FromIterator<T> for PradhanyataShreni<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut heap = Self::nava();
        for item in iter {
            heap.yojaya(item);
        }
        heap
    }
}

/// Min-heap wrapper (Nyūna-Prādhānyatā - न्यूनप्राधान्यता)
///
/// A min-heap where the lowest priority element is at the top.
#[cfg(feature = "alloc")]
pub struct NyunaPradhanyata<T> {
    inner: PradhanyataShreni<core::cmp::Reverse<T>>,
}

#[cfg(feature = "alloc")]
impl<T: Ord> NyunaPradhanyata<T> {
    /// Create empty min-heap (नव - nava)
    pub fn nava() -> Self {
        Self {
            inner: PradhanyataShreni::nava(),
        }
    }

    /// Is empty (रिक्त - rikta)
    pub fn rikta(&self) -> bool {
        self.inner.rikta()
    }

    /// Length (दीर्घता - dīrghatā)
    pub fn dirghata(&self) -> usize {
        self.inner.dirghata()
    }

    /// Push item (योजय - yojaya)
    pub fn yojaya(&mut self, item: T) {
        self.inner.yojaya(core::cmp::Reverse(item));
    }

    /// Pop lowest priority item (निष्कासय - niṣkāsaya)
    pub fn nishkasaya(&mut self) -> Option<T> {
        self.inner.nishkasaya().map(|r| r.0)
    }

    /// Peek at lowest priority item (दृश् - dṛś)
    pub fn drsh(&self) -> Option<&T> {
        self.inner.drsh().map(|r| &r.0)
    }

    /// Clear (शुद्ध - śuddha)
    pub fn shuddha(&mut self) {
        self.inner.shuddha();
    }
}

#[cfg(feature = "alloc")]
impl<T: Ord> Default for NyunaPradhanyata<T> {
    fn default() -> Self {
        Self::nava()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
#[cfg(feature = "alloc")]
mod tests {
    use super::*;

    #[test]
    fn test_max_heap_basic() {
        let mut heap = PradhanyataShreni::nava();
        heap.yojaya(3);
        heap.yojaya(1);
        heap.yojaya(4);
        heap.yojaya(1);
        heap.yojaya(5);

        assert_eq!(heap.dirghata(), 5);
        assert_eq!(heap.nishkasaya(), Some(5));
        assert_eq!(heap.nishkasaya(), Some(4));
        assert_eq!(heap.nishkasaya(), Some(3));
        assert_eq!(heap.nishkasaya(), Some(1));
        assert_eq!(heap.nishkasaya(), Some(1));
        assert!(heap.rikta());
    }

    #[test]
    fn test_max_heap_peek() {
        let mut heap = PradhanyataShreni::nava();
        assert!(heap.drsh().is_none());

        heap.yojaya(10);
        assert_eq!(heap.drsh(), Some(&10));

        heap.yojaya(20);
        assert_eq!(heap.drsh(), Some(&20));
    }

    #[test]
    fn test_min_heap_basic() {
        let mut heap = NyunaPradhanyata::nava();
        heap.yojaya(3);
        heap.yojaya(1);
        heap.yojaya(4);
        heap.yojaya(1);
        heap.yojaya(5);

        assert_eq!(heap.nishkasaya(), Some(1));
        assert_eq!(heap.nishkasaya(), Some(1));
        assert_eq!(heap.nishkasaya(), Some(3));
        assert_eq!(heap.nishkasaya(), Some(4));
        assert_eq!(heap.nishkasaya(), Some(5));
    }

    #[test]
    fn test_from_iterator() {
        let heap: PradhanyataShreni<i32> = vec![3, 1, 4, 1, 5].into_iter().collect();
        assert_eq!(heap.dirghata(), 5);
        assert_eq!(heap.drsh(), Some(&5));
    }

    #[test]
    fn test_clear() {
        let mut heap = PradhanyataShreni::nava();
        heap.yojaya(1);
        heap.yojaya(2);
        heap.shuddha();
        assert!(heap.rikta());
    }
}
