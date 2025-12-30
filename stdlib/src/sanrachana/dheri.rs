//! # Dheri - Heaps (ढेरी)
//!
//! Heap data structures including binary heap and priority queue.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Heap type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DheriPrakara {
    /// Max heap (root is maximum)
    Adhikatam,
    /// Min heap (root is minimum)
    Nyunatam,
}

/// Binary Heap
#[cfg(feature = "alloc")]
pub struct DviDheri<T> {
    /// Elements
    tatva: Vec<T>,
    /// Heap type
    prakara: DheriPrakara,
}

#[cfg(feature = "alloc")]
impl<T: Ord> DviDheri<T> {
    /// Create max heap
    pub fn adhikatam() -> Self {
        Self {
            tatva: Vec::new(),
            prakara: DheriPrakara::Adhikatam,
        }
    }

    /// Create min heap
    pub fn nyunatam() -> Self {
        Self {
            tatva: Vec::new(),
            prakara: DheriPrakara::Nyunatam,
        }
    }

    /// Create from array
    pub fn se(arr: Vec<T>, prakara: DheriPrakara) -> Self {
        let mut heap = Self {
            tatva: arr,
            prakara,
        };
        heap.heapify();
        heap
    }

    /// Build heap from elements
    fn heapify(&mut self) {
        let n = self.tatva.len();
        for i in (0..n / 2).rev() {
            self.sift_down(i);
        }
    }

    fn compare(&self, a: &T, b: &T) -> bool {
        match self.prakara {
            DheriPrakara::Adhikatam => a > b,
            DheriPrakara::Nyunatam => a < b,
        }
    }

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.compare(&self.tatva[idx], &self.tatva[parent]) {
                self.tatva.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut idx: usize) {
        let n = self.tatva.len();
        loop {
            let left = 2 * idx + 1;
            let right = 2 * idx + 2;
            let mut best = idx;

            if left < n && self.compare(&self.tatva[left], &self.tatva[best]) {
                best = left;
            }
            if right < n && self.compare(&self.tatva[right], &self.tatva[best]) {
                best = right;
            }

            if best != idx {
                self.tatva.swap(idx, best);
                idx = best;
            } else {
                break;
            }
        }
    }

    /// Insert element
    pub fn daalo(&mut self, mulya: T) {
        self.tatva.push(mulya);
        self.sift_up(self.tatva.len() - 1);
    }

    /// Remove and return root
    pub fn nikalo(&mut self) -> Option<T> {
        if self.tatva.is_empty() {
            return None;
        }

        let n = self.tatva.len();
        self.tatva.swap(0, n - 1);
        let result = self.tatva.pop();

        if !self.tatva.is_empty() {
            self.sift_down(0);
        }

        result
    }

    /// Peek at root
    pub fn shikhar(&self) -> Option<&T> {
        self.tatva.first()
    }

    /// Is empty
    pub fn khali(&self) -> bool {
        self.tatva.is_empty()
    }

    /// Size
    pub fn aakaar(&self) -> usize {
        self.tatva.len()
    }

    /// Clear all elements
    pub fn saaf(&mut self) {
        self.tatva.clear();
    }

    /// Into sorted vec (heap sort)
    pub fn kramit(mut self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.tatva.len());
        while let Some(val) = self.nikalo() {
            result.push(val);
        }
        result
    }
}

// ============================================================================
// PRIORITY QUEUE WITH KEY (प्राथमिकता पंक्ति)
// ============================================================================

/// Priority queue entry
#[cfg(feature = "alloc")]
#[derive(Debug)]
pub struct PrathamikTatva<K, V> {
    /// Priority key
    pub prathamikta: K,
    /// Value
    pub mulya: V,
}

#[cfg(feature = "alloc")]
impl<K, V> PrathamikTatva<K, V> {
    pub fn naya(prathamikta: K, mulya: V) -> Self {
        Self { prathamikta, mulya }
    }
}

#[cfg(feature = "alloc")]
impl<K: Ord, V> PartialEq for PrathamikTatva<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.prathamikta == other.prathamikta
    }
}

#[cfg(feature = "alloc")]
impl<K: Ord, V> Eq for PrathamikTatva<K, V> {}

#[cfg(feature = "alloc")]
impl<K: Ord, V> PartialOrd for PrathamikTatva<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "alloc")]
impl<K: Ord, V> Ord for PrathamikTatva<K, V> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.prathamikta.cmp(&other.prathamikta)
    }
}

/// Priority Queue
#[cfg(feature = "alloc")]
pub struct PrathamikPankti<K: Ord, V> {
    dheri: DviDheri<PrathamikTatva<K, V>>,
}

#[cfg(feature = "alloc")]
impl<K: Ord, V> PrathamikPankti<K, V> {
    /// Create max-priority queue
    pub fn adhikatam() -> Self {
        Self {
            dheri: DviDheri::adhikatam(),
        }
    }

    /// Create min-priority queue
    pub fn nyunatam() -> Self {
        Self {
            dheri: DviDheri::nyunatam(),
        }
    }

    /// Enqueue with priority
    pub fn daalo(&mut self, prathamikta: K, mulya: V) {
        self.dheri.daalo(PrathamikTatva::naya(prathamikta, mulya));
    }

    /// Dequeue highest/lowest priority
    pub fn nikalo(&mut self) -> Option<(K, V)> {
        self.dheri.nikalo().map(|e| (e.prathamikta, e.mulya))
    }

    /// Peek
    pub fn dekho(&self) -> Option<(&K, &V)> {
        self.dheri.shikhar().map(|e| (&e.prathamikta, &e.mulya))
    }

    /// Is empty
    pub fn khali(&self) -> bool {
        self.dheri.khali()
    }

    /// Size
    pub fn aakaar(&self) -> usize {
        self.dheri.aakaar()
    }
}

// ============================================================================
// D-ARY HEAP (डी-अरी ढेरी)
// ============================================================================

/// D-ary heap (generalized heap with d children per node)
#[cfg(feature = "alloc")]
pub struct DAriDheri<T> {
    tatva: Vec<T>,
    d: usize,
    prakara: DheriPrakara,
}

#[cfg(feature = "alloc")]
impl<T: Ord> DAriDheri<T> {
    /// Create d-ary max heap
    pub fn adhikatam(d: usize) -> Self {
        Self {
            tatva: Vec::new(),
            d: d.max(2),
            prakara: DheriPrakara::Adhikatam,
        }
    }

    /// Create d-ary min heap
    pub fn nyunatam(d: usize) -> Self {
        Self {
            tatva: Vec::new(),
            d: d.max(2),
            prakara: DheriPrakara::Nyunatam,
        }
    }

    fn parent(&self, i: usize) -> usize {
        (i - 1) / self.d
    }

    fn child(&self, i: usize, k: usize) -> usize {
        self.d * i + k + 1
    }

    fn compare(&self, a: &T, b: &T) -> bool {
        match self.prakara {
            DheriPrakara::Adhikatam => a > b,
            DheriPrakara::Nyunatam => a < b,
        }
    }

    fn sift_up(&mut self, mut idx: usize) {
        while idx > 0 {
            let p = self.parent(idx);
            if self.compare(&self.tatva[idx], &self.tatva[p]) {
                self.tatva.swap(idx, p);
                idx = p;
            } else {
                break;
            }
        }
    }

    fn sift_down(&mut self, mut idx: usize) {
        let n = self.tatva.len();
        loop {
            let mut best = idx;

            for k in 0..self.d {
                let c = self.child(idx, k);
                if c < n && self.compare(&self.tatva[c], &self.tatva[best]) {
                    best = c;
                }
            }

            if best != idx {
                self.tatva.swap(idx, best);
                idx = best;
            } else {
                break;
            }
        }
    }

    /// Insert
    pub fn daalo(&mut self, mulya: T) {
        self.tatva.push(mulya);
        self.sift_up(self.tatva.len() - 1);
    }

    /// Remove root
    pub fn nikalo(&mut self) -> Option<T> {
        if self.tatva.is_empty() {
            return None;
        }

        let n = self.tatva.len();
        self.tatva.swap(0, n - 1);
        let result = self.tatva.pop();

        if !self.tatva.is_empty() {
            self.sift_down(0);
        }

        result
    }

    /// Peek
    pub fn shikhar(&self) -> Option<&T> {
        self.tatva.first()
    }

    /// Size
    pub fn aakaar(&self) -> usize {
        self.tatva.len()
    }

    /// Is empty
    pub fn khali(&self) -> bool {
        self.tatva.is_empty()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_max_heap() {
        let mut heap: DviDheri<i32> = DviDheri::adhikatam();
        heap.daalo(5);
        heap.daalo(3);
        heap.daalo(8);
        heap.daalo(1);

        assert_eq!(heap.nikalo(), Some(8));
        assert_eq!(heap.nikalo(), Some(5));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_min_heap() {
        let mut heap: DviDheri<i32> = DviDheri::nyunatam();
        heap.daalo(5);
        heap.daalo(3);
        heap.daalo(8);
        heap.daalo(1);

        assert_eq!(heap.nikalo(), Some(1));
        assert_eq!(heap.nikalo(), Some(3));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_heap_sort() {
        let arr = vec![5, 3, 8, 1, 9, 2];
        let heap = DviDheri::se(arr, DheriPrakara::Adhikatam);
        let sorted = heap.kramit();

        assert_eq!(sorted, vec![9, 8, 5, 3, 2, 1]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_priority_queue() {
        let mut pq: PrathamikPankti<i32, &str> = PrathamikPankti::adhikatam();
        pq.daalo(3, "medium");
        pq.daalo(1, "low");
        pq.daalo(5, "high");

        assert_eq!(pq.nikalo(), Some((5, "high")));
        assert_eq!(pq.nikalo(), Some((3, "medium")));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_d_ary_heap() {
        let mut heap: DAriDheri<i32> = DAriDheri::adhikatam(4);
        heap.daalo(5);
        heap.daalo(3);
        heap.daalo(8);
        heap.daalo(1);

        assert_eq!(heap.nikalo(), Some(8));
    }
}
