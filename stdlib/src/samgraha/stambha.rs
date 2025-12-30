//! # Stambha - Heap and Priority Queue (स्तम्भ)
//!
//! Binary heap and priority queue implementations.
//!
//! > **"उच्चतमं प्रथमम्"**
//! > *"The highest first"*
//!
//! ## Structures
//!
//! - `DvidhaStambha` - Binary Heap
//! - `PrathamyaStambha` - Priority Queue
//! - `NyunatamStambha` - Min-Heap
//! - `AdhikatamStambha` - Max-Heap

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use core::cmp::Ordering;

// ============================================================================
// BINARY HEAP (द्विध स्तम्भ)
// ============================================================================

/// Binary Heap (द्विध स्तम्भ)
///
/// # Etymology
/// द्विध (dvidha) = binary
/// स्तम्भ (stambha) = pillar, heap
#[cfg(feature = "alloc")]
pub struct DvidhaStambha<T> {
    data: Vec<T>,
    tulana: fn(&T, &T) -> Ordering, // comparison function
}

#[cfg(feature = "alloc")]
impl<T> DvidhaStambha<T> {
    /// Create empty heap with custom comparator
    pub fn nava(tulana: fn(&T, &T) -> Ordering) -> Self {
        Self {
            data: Vec::new(),
            tulana,
        }
    }

    /// Create max heap (अधिकतम स्तम्भ)
    pub fn adhikatam() -> Self
    where
        T: Ord,
    {
        Self {
            data: Vec::new(),
            tulana: |a, b| a.cmp(b),
        }
    }

    /// Create min heap (न्यूनतम स्तम्भ)
    pub fn nyunatam() -> Self
    where
        T: Ord,
    {
        Self {
            data: Vec::new(),
            tulana: |a, b| b.cmp(a),
        }
    }

    /// Check if empty (रिक्त)
    pub fn rikta(&self) -> bool {
        self.data.is_empty()
    }

    /// Get size (आकार)
    pub fn akara(&self) -> usize {
        self.data.len()
    }

    /// Push element (प्रविष्ट)
    pub fn pravisht(&mut self, tattva: T) {
        self.data.push(tattva);
        self.upari_sthapan(self.data.len() - 1);
    }

    /// Pop top element (निष्कासित)
    pub fn nishkasit(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        let len = self.data.len();
        self.data.swap(0, len - 1);
        let result = self.data.pop();

        if !self.data.is_empty() {
            self.adho_sthapan(0);
        }

        result
    }

    /// Peek top element (द्रष्टा)
    pub fn drashta(&self) -> Option<&T> {
        self.data.first()
    }

    /// Heapify up (ऊपरी स्थापन)
    fn upari_sthapan(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if (self.tulana)(&self.data[idx], &self.data[parent]) == Ordering::Greater {
                self.data.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    /// Heapify down (नीचे स्थापन)
    fn adho_sthapan(&mut self, mut idx: usize) {
        let len = self.data.len();

        loop {
            let vama = 2 * idx + 1; // left child
            let dakshina = 2 * idx + 2; // right child
            let mut shreshtha = idx; // largest/smallest

            if vama < len
                && (self.tulana)(&self.data[vama], &self.data[shreshtha]) == Ordering::Greater
            {
                shreshtha = vama;
            }
            if dakshina < len
                && (self.tulana)(&self.data[dakshina], &self.data[shreshtha]) == Ordering::Greater
            {
                shreshtha = dakshina;
            }

            if shreshtha != idx {
                self.data.swap(idx, shreshtha);
                idx = shreshtha;
            } else {
                break;
            }
        }
    }

    /// Build from array (निर्माण)
    pub fn nirman(mut data: Vec<T>, tulana: fn(&T, &T) -> Ordering) -> Self {
        let mut heap = Self { data, tulana };

        // Heapify from bottom-up
        let len = heap.data.len();
        for i in (0..len / 2).rev() {
            heap.adho_sthapan(i);
        }

        heap
    }

    /// Convert to sorted vec (क्रमित परिवर्तन)
    pub fn kramit_parivartan(mut self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.data.len());
        while let Some(item) = self.nishkasit() {
            result.push(item);
        }
        result
    }
}

// ============================================================================
// PRIORITY QUEUE (प्राथम्य पंक्ति)
// ============================================================================

/// Priority Queue entry (प्राथम्य प्रविष्टि)
#[cfg(feature = "alloc")]
#[derive(Clone)]
pub struct PrathamyaPravishti<T, P> {
    pub tattva: T,    // element (तत्त्व)
    pub prathamya: P, // priority (प्राथम्य)
}

/// Priority Queue (प्राथम्य पंक्ति)
///
/// # Etymology
/// प्राथम्य (prāthamya) = priority
/// पंक्ति (paṅkti) = queue, row
#[cfg(feature = "alloc")]
pub struct PrathamyaPankti<T, P: Ord> {
    stambha: Vec<PrathamyaPravishti<T, P>>,
    adhikatam: bool, // true = max priority, false = min priority
}

#[cfg(feature = "alloc")]
impl<T, P: Ord> PrathamyaPankti<T, P> {
    /// Create max priority queue (अधिकतम प्राथम्य)
    pub fn adhikatam() -> Self {
        Self {
            stambha: Vec::new(),
            adhikatam: true,
        }
    }

    /// Create min priority queue (न्यूनतम प्राथम्य)
    pub fn nyunatam() -> Self {
        Self {
            stambha: Vec::new(),
            adhikatam: false,
        }
    }

    /// Check if empty
    pub fn rikta(&self) -> bool {
        self.stambha.is_empty()
    }

    /// Get size
    pub fn akara(&self) -> usize {
        self.stambha.len()
    }

    /// Compare priorities
    fn tulana(&self, a: &P, b: &P) -> bool {
        if self.adhikatam {
            a > b
        } else {
            a < b
        }
    }

    /// Enqueue with priority (प्रविष्ट)
    pub fn pravisht(&mut self, tattva: T, prathamya: P) {
        self.stambha.push(PrathamyaPravishti { tattva, prathamya });
        self.upari_sthapan(self.stambha.len() - 1);
    }

    /// Dequeue (निष्कासित)
    pub fn nishkasit(&mut self) -> Option<PrathamyaPravishti<T, P>> {
        if self.stambha.is_empty() {
            return None;
        }

        let len = self.stambha.len();
        self.stambha.swap(0, len - 1);
        let result = self.stambha.pop();

        if !self.stambha.is_empty() {
            self.adho_sthapan(0);
        }

        result
    }

    /// Peek top (द्रष्टा)
    pub fn drashta(&self) -> Option<&PrathamyaPravishti<T, P>> {
        self.stambha.first()
    }

    fn upari_sthapan(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = (idx - 1) / 2;
            if self.tulana(
                &self.stambha[idx].prathamya,
                &self.stambha[parent].prathamya,
            ) {
                self.stambha.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    fn adho_sthapan(&mut self, mut idx: usize) {
        let len = self.stambha.len();

        loop {
            let vama = 2 * idx + 1;
            let dakshina = 2 * idx + 2;
            let mut shreshtha = idx;

            if vama < len
                && self.tulana(
                    &self.stambha[vama].prathamya,
                    &self.stambha[shreshtha].prathamya,
                )
            {
                shreshtha = vama;
            }
            if dakshina < len
                && self.tulana(
                    &self.stambha[dakshina].prathamya,
                    &self.stambha[shreshtha].prathamya,
                )
            {
                shreshtha = dakshina;
            }

            if shreshtha != idx {
                self.stambha.swap(idx, shreshtha);
                idx = shreshtha;
            } else {
                break;
            }
        }
    }
}

// ============================================================================
// D-ARY HEAP (बहुध स्तम्भ)
// ============================================================================

/// D-ary Heap - heap with D children per node (बहुध स्तम्भ)
///
/// D=2 is binary heap, D=4 often better for cache
#[cfg(feature = "alloc")]
pub struct BahudhaStambha<T, const D: usize = 4> {
    data: Vec<T>,
}

#[cfg(feature = "alloc")]
impl<T: Ord, const D: usize> BahudhaStambha<T, D> {
    pub fn nava() -> Self {
        Self { data: Vec::new() }
    }

    pub fn rikta(&self) -> bool {
        self.data.is_empty()
    }

    pub fn akara(&self) -> usize {
        self.data.len()
    }

    /// Push element (min-heap behavior)
    pub fn pravisht(&mut self, tattva: T) {
        self.data.push(tattva);
        self.upari_sthapan(self.data.len() - 1);
    }

    /// Pop minimum
    pub fn nishkasit(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }

        let len = self.data.len();
        self.data.swap(0, len - 1);
        let result = self.data.pop();

        if !self.data.is_empty() {
            self.adho_sthapan(0);
        }

        result
    }

    pub fn drashta(&self) -> Option<&T> {
        self.data.first()
    }

    fn parent(idx: usize) -> usize {
        if idx == 0 {
            0
        } else {
            (idx - 1) / D
        }
    }

    fn first_child(idx: usize) -> usize {
        D * idx + 1
    }

    fn upari_sthapan(&mut self, mut idx: usize) {
        while idx > 0 {
            let parent = Self::parent(idx);
            if self.data[idx] < self.data[parent] {
                self.data.swap(idx, parent);
                idx = parent;
            } else {
                break;
            }
        }
    }

    fn adho_sthapan(&mut self, mut idx: usize) {
        let len = self.data.len();

        loop {
            let first_child = Self::first_child(idx);
            if first_child >= len {
                break;
            }

            // Find minimum among D children
            let mut min_child = first_child;
            for i in 1..D {
                let child = first_child + i;
                if child < len && self.data[child] < self.data[min_child] {
                    min_child = child;
                }
            }

            if self.data[min_child] < self.data[idx] {
                self.data.swap(idx, min_child);
                idx = min_child;
            } else {
                break;
            }
        }
    }
}

// ============================================================================
// INDEXED PRIORITY QUEUE (अनुक्रमित प्राथम्य पंक्ति)
// ============================================================================

/// Indexed Priority Queue (अनुक्रमित प्राथम्य पंक्ति)
///
/// Allows O(log n) decrease-key operation.
/// Used in Dijkstra's algorithm.
#[cfg(feature = "alloc")]
pub struct AnukramitPrathamyaPankti<P: Ord> {
    n: usize,                   // Number of elements
    stambha: Vec<usize>,        // Heap of indices
    sthiti: Vec<Option<usize>>, // Position of index in heap
    prathamya: Vec<Option<P>>,  // Priority of each index
}

#[cfg(feature = "alloc")]
impl<P: Ord + Clone> AnukramitPrathamyaPankti<P> {
    /// Create with capacity
    pub fn nava(kshamata: usize) -> Self {
        Self {
            n: 0,
            stambha: Vec::with_capacity(kshamata),
            sthiti: vec![None; kshamata],
            prathamya: vec![None; kshamata],
        }
    }

    pub fn rikta(&self) -> bool {
        self.n == 0
    }

    pub fn akara(&self) -> usize {
        self.n
    }

    /// Check if index is in queue (विद्यमान)
    pub fn vidyamana(&self, idx: usize) -> bool {
        idx < self.sthiti.len() && self.sthiti[idx].is_some()
    }

    /// Insert or update (प्रविष्ट अथवा अद्यतन)
    pub fn pravisht_athva_adyatan(&mut self, idx: usize, prathamya: P) {
        if idx >= self.sthiti.len() {
            return;
        }

        if self.vidyamana(idx) {
            // Update existing
            let old = self.prathamya[idx].as_ref().unwrap();
            if prathamya < *old {
                self.prathamya[idx] = Some(prathamya);
                self.upari_sthapan(self.sthiti[idx].unwrap());
            } else if prathamya > *old {
                self.prathamya[idx] = Some(prathamya);
                self.adho_sthapan(self.sthiti[idx].unwrap());
            }
        } else {
            // Insert new
            self.prathamya[idx] = Some(prathamya);
            self.sthiti[idx] = Some(self.n);
            self.stambha.push(idx);
            self.n += 1;
            self.upari_sthapan(self.n - 1);
        }
    }

    /// Pop minimum index (न्यूनतम निष्कासित)
    pub fn nyunatam_nishkasit(&mut self) -> Option<(usize, P)> {
        if self.n == 0 {
            return None;
        }

        let min_idx = self.stambha[0];
        let prathamya = self.prathamya[min_idx].take().unwrap();

        self.stambha.swap(0, self.n - 1);
        self.sthiti[self.stambha[0]] = Some(0);
        self.stambha.pop();
        self.sthiti[min_idx] = None;
        self.n -= 1;

        if self.n > 0 {
            self.adho_sthapan(0);
        }

        Some((min_idx, prathamya))
    }

    fn upari_sthapan(&mut self, mut pos: usize) {
        while pos > 0 {
            let parent = (pos - 1) / 2;
            let pos_idx = self.stambha[pos];
            let par_idx = self.stambha[parent];

            if self.prathamya[pos_idx] < self.prathamya[par_idx] {
                self.stambha.swap(pos, parent);
                self.sthiti[pos_idx] = Some(parent);
                self.sthiti[par_idx] = Some(pos);
                pos = parent;
            } else {
                break;
            }
        }
    }

    fn adho_sthapan(&mut self, mut pos: usize) {
        loop {
            let vama = 2 * pos + 1;
            let dakshina = 2 * pos + 2;
            let mut smallest = pos;

            if vama < self.n {
                let s_idx = self.stambha[smallest];
                let v_idx = self.stambha[vama];
                if self.prathamya[v_idx] < self.prathamya[s_idx] {
                    smallest = vama;
                }
            }
            if dakshina < self.n {
                let s_idx = self.stambha[smallest];
                let d_idx = self.stambha[dakshina];
                if self.prathamya[d_idx] < self.prathamya[s_idx] {
                    smallest = dakshina;
                }
            }

            if smallest != pos {
                let pos_idx = self.stambha[pos];
                let small_idx = self.stambha[smallest];
                self.stambha.swap(pos, smallest);
                self.sthiti[pos_idx] = Some(smallest);
                self.sthiti[small_idx] = Some(pos);
                pos = smallest;
            } else {
                break;
            }
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_max_heap() {
        let mut heap: DvidhaStambha<i32> = DvidhaStambha::adhikatam();

        heap.pravisht(3);
        heap.pravisht(1);
        heap.pravisht(4);
        heap.pravisht(1);
        heap.pravisht(5);
        heap.pravisht(9);

        assert_eq!(heap.akara(), 6);
        assert_eq!(heap.drashta(), Some(&9));
        assert_eq!(heap.nishkasit(), Some(9));
        assert_eq!(heap.nishkasit(), Some(5));
        assert_eq!(heap.nishkasit(), Some(4));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_min_heap() {
        let mut heap: DvidhaStambha<i32> = DvidhaStambha::nyunatam();

        heap.pravisht(3);
        heap.pravisht(1);
        heap.pravisht(4);

        assert_eq!(heap.nishkasit(), Some(1));
        assert_eq!(heap.nishkasit(), Some(3));
        assert_eq!(heap.nishkasit(), Some(4));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_heap_build() {
        let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let heap = DvidhaStambha::nirman(data, |a, b| a.cmp(b));

        assert_eq!(heap.akara(), 8);
        assert_eq!(heap.drashta(), Some(&9));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_heapsort() {
        let data = vec![3, 1, 4, 1, 5, 9, 2, 6];
        let heap = DvidhaStambha::nirman(data, |a, b| a.cmp(b));
        let sorted = heap.kramit_parivartan();

        assert_eq!(sorted, vec![9, 6, 5, 4, 3, 2, 1, 1]);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_priority_queue() {
        let mut pq: PrathamyaPankti<&str, i32> = PrathamyaPankti::adhikatam();

        pq.pravisht("low", 1);
        pq.pravisht("high", 10);
        pq.pravisht("medium", 5);

        let top = pq.nishkasit().unwrap();
        assert_eq!(top.tattva, "high");
        assert_eq!(top.prathamya, 10);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_min_priority_queue() {
        let mut pq: PrathamyaPankti<&str, i32> = PrathamyaPankti::nyunatam();

        pq.pravisht("low", 1);
        pq.pravisht("high", 10);
        pq.pravisht("medium", 5);

        let top = pq.nishkasit().unwrap();
        assert_eq!(top.tattva, "low");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_dary_heap() {
        let mut heap: BahudhaStambha<i32, 4> = BahudhaStambha::nava();

        for i in [5, 3, 8, 1, 9, 2] {
            heap.pravisht(i);
        }

        assert_eq!(heap.nishkasit(), Some(1));
        assert_eq!(heap.nishkasit(), Some(2));
        assert_eq!(heap.nishkasit(), Some(3));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_indexed_pq() {
        let mut ipq: AnukramitPrathamyaPankti<i32> = AnukramitPrathamyaPankti::nava(10);

        ipq.pravisht_athva_adyatan(3, 10);
        ipq.pravisht_athva_adyatan(7, 5);
        ipq.pravisht_athva_adyatan(1, 15);

        assert!(ipq.vidyamana(3));
        assert!(!ipq.vidyamana(5));

        let (idx, pri) = ipq.nyunatam_nishkasit().unwrap();
        assert_eq!(idx, 7);
        assert_eq!(pri, 5);

        // Update priority
        ipq.pravisht_athva_adyatan(1, 2);
        let (idx, pri) = ipq.nyunatam_nishkasit().unwrap();
        assert_eq!(idx, 1);
        assert_eq!(pri, 2);
    }
}
