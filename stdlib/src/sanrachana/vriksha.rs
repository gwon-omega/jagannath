//! # Vriksha - Trees (वृक्ष)
//!
//! Tree data structures including BST, AVL, Trie.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
#[cfg(feature = "alloc")]
use alloc::string::String;

// ============================================================================
// BINARY SEARCH TREE (द्विखोज वृक्ष)
// ============================================================================

/// BST Node
#[cfg(feature = "alloc")]
pub struct DvikhojGanth<K, V> {
    /// Key
    pub kunci: K,
    /// Value
    pub mulya: V,
    /// Left child
    pub vaam: Option<Box<DvikhojGanth<K, V>>>,
    /// Right child
    pub dakshin: Option<Box<DvikhojGanth<K, V>>>,
}

#[cfg(feature = "alloc")]
impl<K: Ord, V> DvikhojGanth<K, V> {
    /// Create new node
    pub fn naya(kunci: K, mulya: V) -> Self {
        Self {
            kunci,
            mulya,
            vaam: None,
            dakshin: None,
        }
    }

    /// Insert into tree
    pub fn daalo(&mut self, kunci: K, mulya: V) {
        if kunci < self.kunci {
            match &mut self.vaam {
                Some(node) => node.daalo(kunci, mulya),
                None => self.vaam = Some(Box::new(DvikhojGanth::naya(kunci, mulya))),
            }
        } else if kunci > self.kunci {
            match &mut self.dakshin {
                Some(node) => node.daalo(kunci, mulya),
                None => self.dakshin = Some(Box::new(DvikhojGanth::naya(kunci, mulya))),
            }
        } else {
            self.mulya = mulya;
        }
    }

    /// Search for key
    pub fn khojo(&self, kunci: &K) -> Option<&V> {
        if kunci == &self.kunci {
            Some(&self.mulya)
        } else if kunci < &self.kunci {
            self.vaam.as_ref()?.khojo(kunci)
        } else {
            self.dakshin.as_ref()?.khojo(kunci)
        }
    }

    /// Contains key
    pub fn shamil(&self, kunci: &K) -> bool {
        self.khojo(kunci).is_some()
    }

    /// Minimum key
    pub fn nyunatam(&self) -> &K {
        match &self.vaam {
            Some(node) => node.nyunatam(),
            None => &self.kunci,
        }
    }

    /// Maximum key
    pub fn adhikatam(&self) -> &K {
        match &self.dakshin {
            Some(node) => node.adhikatam(),
            None => &self.kunci,
        }
    }

    /// Height
    pub fn unchai(&self) -> usize {
        let left_h = self.vaam.as_ref().map(|n| n.unchai()).unwrap_or(0);
        let right_h = self.dakshin.as_ref().map(|n| n.unchai()).unwrap_or(0);
        1 + left_h.max(right_h)
    }

    /// Count nodes
    pub fn ginti(&self) -> usize {
        let left_c = self.vaam.as_ref().map(|n| n.ginti()).unwrap_or(0);
        let right_c = self.dakshin.as_ref().map(|n| n.ginti()).unwrap_or(0);
        1 + left_c + right_c
    }

    /// In-order traversal
    pub fn madhya_bhraman<'a>(&'a self, result: &mut Vec<(&'a K, &'a V)>) {
        if let Some(ref left) = self.vaam {
            left.madhya_bhraman(result);
        }
        result.push((&self.kunci, &self.mulya));
        if let Some(ref right) = self.dakshin {
            right.madhya_bhraman(result);
        }
    }

    /// Pre-order traversal
    pub fn purva_bhraman<'a>(&'a self, result: &mut Vec<(&'a K, &'a V)>) {
        result.push((&self.kunci, &self.mulya));
        if let Some(ref left) = self.vaam {
            left.purva_bhraman(result);
        }
        if let Some(ref right) = self.dakshin {
            right.purva_bhraman(result);
        }
    }

    /// Post-order traversal
    pub fn pasch_bhraman<'a>(&'a self, result: &mut Vec<(&'a K, &'a V)>) {
        if let Some(ref left) = self.vaam {
            left.pasch_bhraman(result);
        }
        if let Some(ref right) = self.dakshin {
            right.pasch_bhraman(result);
        }
        result.push((&self.kunci, &self.mulya));
    }
}

// ============================================================================
// TRIE (त्रि-वृक्ष)
// ============================================================================

/// Trie node (for ASCII strings)
#[cfg(feature = "alloc")]
pub struct TriGanth {
    /// Children (256 for ASCII)
    santan: [Option<Box<TriGanth>>; 256],
    /// Is end of word
    shabd_ant: bool,
    /// Value at this node
    mulya: Option<usize>,
}

#[cfg(feature = "alloc")]
impl Default for TriGanth {
    fn default() -> Self {
        Self::naya()
    }
}

#[cfg(feature = "alloc")]
impl TriGanth {
    /// Create new trie node
    pub fn naya() -> Self {
        Self {
            santan: core::array::from_fn(|_| None),
            shabd_ant: false,
            mulya: None,
        }
    }

    /// Insert word
    pub fn daalo(&mut self, shabd: &str, mulya: usize) {
        let mut current = self;

        for byte in shabd.bytes() {
            let idx = byte as usize;
            if current.santan[idx].is_none() {
                current.santan[idx] = Some(Box::new(TriGanth::naya()));
            }
            current = current.santan[idx].as_mut().unwrap();
        }

        current.shabd_ant = true;
        current.mulya = Some(mulya);
    }

    /// Search for word
    pub fn khojo(&self, shabd: &str) -> Option<usize> {
        let mut current = self;

        for byte in shabd.bytes() {
            let idx = byte as usize;
            match &current.santan[idx] {
                Some(node) => current = node,
                None => return None,
            }
        }

        if current.shabd_ant {
            current.mulya
        } else {
            None
        }
    }

    /// Check if word exists
    pub fn shamil(&self, shabd: &str) -> bool {
        self.khojo(shabd).is_some()
    }

    /// Check if prefix exists
    pub fn upasarga_hai(&self, prefix: &str) -> bool {
        let mut current = self;

        for byte in prefix.bytes() {
            let idx = byte as usize;
            match &current.santan[idx] {
                Some(node) => current = node,
                None => return false,
            }
        }

        true
    }

    /// Get all words with prefix
    pub fn upasarga_shabd(&self, prefix: &str) -> Vec<String> {
        let mut results = Vec::new();

        let mut current = self;
        for byte in prefix.bytes() {
            let idx = byte as usize;
            match &current.santan[idx] {
                Some(node) => current = node,
                None => return results,
            }
        }

        // DFS to collect all words
        let mut stack: Vec<(String, &TriGanth)> = vec![(prefix.to_string(), current)];

        while let Some((path, node)) = stack.pop() {
            if node.shabd_ant {
                results.push(path.clone());
            }

            for (i, child) in node.santan.iter().enumerate() {
                if let Some(child_node) = child {
                    let mut new_path = path.clone();
                    new_path.push(i as u8 as char);
                    stack.push((new_path, child_node));
                }
            }
        }

        results
    }
}

// ============================================================================
// SEGMENT TREE (खंड वृक्ष)
// ============================================================================

/// Segment tree for range queries
#[cfg(feature = "alloc")]
pub struct KhandVriksha {
    /// Tree array
    vriksha: Vec<i64>,
    /// Original size
    aakaar: usize,
    /// Operation (0=sum, 1=min, 2=max)
    kriya: u8,
}

#[cfg(feature = "alloc")]
impl KhandVriksha {
    /// Create with sum operation
    pub fn yoga_se(arr: &[i64]) -> Self {
        let n = arr.len();
        let mut tree = vec![0i64; 4 * n];
        let mut st = Self { vriksha: tree, aakaar: n, kriya: 0 };
        st.banao(arr, 0, 0, n - 1);
        st
    }

    /// Create with min operation
    pub fn nyunatam_se(arr: &[i64]) -> Self {
        let n = arr.len();
        let tree = vec![i64::MAX; 4 * n];
        let mut st = Self { vriksha: tree, aakaar: n, kriya: 1 };
        st.banao(arr, 0, 0, n - 1);
        st
    }

    /// Create with max operation
    pub fn adhikatam_se(arr: &[i64]) -> Self {
        let n = arr.len();
        let tree = vec![i64::MIN; 4 * n];
        let mut st = Self { vriksha: tree, aakaar: n, kriya: 2 };
        st.banao(arr, 0, 0, n - 1);
        st
    }

    fn combine(&self, a: i64, b: i64) -> i64 {
        match self.kriya {
            0 => a + b,
            1 => a.min(b),
            2 => a.max(b),
            _ => a + b,
        }
    }

    fn identity(&self) -> i64 {
        match self.kriya {
            0 => 0,
            1 => i64::MAX,
            2 => i64::MIN,
            _ => 0,
        }
    }

    fn banao(&mut self, arr: &[i64], node: usize, start: usize, end: usize) {
        if start == end {
            self.vriksha[node] = arr[start];
        } else {
            let mid = (start + end) / 2;
            self.banao(arr, 2 * node + 1, start, mid);
            self.banao(arr, 2 * node + 2, mid + 1, end);
            self.vriksha[node] = self.combine(
                self.vriksha[2 * node + 1],
                self.vriksha[2 * node + 2]
            );
        }
    }

    /// Range query [l, r]
    pub fn prashna(&self, l: usize, r: usize) -> i64 {
        self.prashna_inner(0, 0, self.aakaar - 1, l, r)
    }

    fn prashna_inner(&self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i64 {
        if r < start || l > end {
            return self.identity();
        }

        if l <= start && end <= r {
            return self.vriksha[node];
        }

        let mid = (start + end) / 2;
        let left = self.prashna_inner(2 * node + 1, start, mid, l, r);
        let right = self.prashna_inner(2 * node + 2, mid + 1, end, l, r);

        self.combine(left, right)
    }

    /// Point update
    pub fn badlo(&mut self, idx: usize, mulya: i64) {
        self.badlo_inner(0, 0, self.aakaar - 1, idx, mulya);
    }

    fn badlo_inner(&mut self, node: usize, start: usize, end: usize, idx: usize, mulya: i64) {
        if start == end {
            self.vriksha[node] = mulya;
        } else {
            let mid = (start + end) / 2;
            if idx <= mid {
                self.badlo_inner(2 * node + 1, start, mid, idx, mulya);
            } else {
                self.badlo_inner(2 * node + 2, mid + 1, end, idx, mulya);
            }
            self.vriksha[node] = self.combine(
                self.vriksha[2 * node + 1],
                self.vriksha[2 * node + 2]
            );
        }
    }
}

// ============================================================================
// FENWICK TREE / BIT (फेनविक वृक्ष)
// ============================================================================

/// Binary Indexed Tree
#[cfg(feature = "alloc")]
pub struct FenwickVriksha {
    vriksha: Vec<i64>,
}

#[cfg(feature = "alloc")]
impl FenwickVriksha {
    /// Create from array
    pub fn naya(arr: &[i64]) -> Self {
        let n = arr.len();
        let mut tree = vec![0i64; n + 1];

        for (i, &val) in arr.iter().enumerate() {
            let mut j = i + 1;
            while j <= n {
                tree[j] += val;
                j += j & (!j + 1); // j += j & -j
            }
        }

        Self { vriksha: tree }
    }

    /// Prefix sum [0, idx]
    pub fn yoga(&self, idx: usize) -> i64 {
        let mut sum = 0i64;
        let mut i = idx + 1;
        while i > 0 {
            sum += self.vriksha[i];
            i -= i & (!i + 1);
        }
        sum
    }

    /// Range sum [l, r]
    pub fn paridhi_yoga(&self, l: usize, r: usize) -> i64 {
        if l == 0 {
            self.yoga(r)
        } else {
            self.yoga(r) - self.yoga(l - 1)
        }
    }

    /// Point update: add delta to idx
    pub fn jodo(&mut self, idx: usize, delta: i64) {
        let n = self.vriksha.len() - 1;
        let mut i = idx + 1;
        while i <= n {
            self.vriksha[i] += delta;
            i += i & (!i + 1);
        }
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
    fn test_bst() {
        let mut root = DvikhojGanth::naya(5, "five");
        root.daalo(3, "three");
        root.daalo(7, "seven");
        root.daalo(1, "one");

        assert_eq!(root.khojo(&3), Some(&"three"));
        assert_eq!(root.khojo(&10), None);
        assert_eq!(*root.nyunatam(), 1);
        assert_eq!(*root.adhikatam(), 7);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_trie() {
        let mut trie = TriGanth::naya();
        trie.daalo("hello", 1);
        trie.daalo("help", 2);
        trie.daalo("world", 3);

        assert!(trie.shamil("hello"));
        assert!(trie.shamil("help"));
        assert!(!trie.shamil("hel"));
        assert!(trie.upasarga_hai("hel"));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_segment_tree() {
        let arr = [1i64, 3, 5, 7, 9, 11];
        let st = KhandVriksha::yoga_se(&arr);

        assert_eq!(st.prashna(1, 3), 15); // 3 + 5 + 7
        assert_eq!(st.prashna(0, 5), 36); // sum all
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_fenwick() {
        let arr = [1i64, 3, 5, 7, 9, 11];
        let bit = FenwickVriksha::naya(&arr);

        assert_eq!(bit.yoga(2), 9); // 1 + 3 + 5
        assert_eq!(bit.paridhi_yoga(1, 3), 15); // 3 + 5 + 7
    }
}
