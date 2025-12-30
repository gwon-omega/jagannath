//! # Vṛkṣa - Tree Data Structures (वृक्ष)
//!
//! Binary trees, BST, AVL, Red-Black, and B-Trees.
//!
//! > **"एको वृक्षो बहुशाखः"**
//! > *"One tree, many branches"*
//!
//! ## Trees
//!
//! - `DvidhaVrksha` - Binary Search Tree
//! - `AvlVrksha` - AVL Tree (self-balancing)
//! - `LohitKrishnaVrksha` - Red-Black Tree
//! - `TripatriVrksha` - Trie (prefix tree)

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use core::cmp::Ordering;

// ============================================================================
// BINARY SEARCH TREE (द्विध वृक्ष)
// ============================================================================

/// Binary Search Tree Node (द्विध ग्रन्थि)
#[cfg(feature = "alloc")]
pub struct DvidhaGranthi<K, V> {
    kunji: K,                                   // key (कुञ्जी)
    mulya: V,                                   // value (मूल्य)
    vama: Option<Box<DvidhaGranthi<K, V>>>,     // left (वाम)
    dakshina: Option<Box<DvidhaGranthi<K, V>>>, // right (दक्षिण)
}

/// Binary Search Tree (द्विध खोज वृक्ष)
///
/// # Etymology
/// द्विध (dvidha) = twofold, binary
/// खोज (khoj) = search
/// वृक्ष (vṛkṣa) = tree
#[cfg(feature = "alloc")]
pub struct DvidhaVrksha<K, V> {
    mula: Option<Box<DvidhaGranthi<K, V>>>, // root (मूल)
    sankhya: usize,                         // count (संख्या)
}

#[cfg(feature = "alloc")]
impl<K: Ord, V> DvidhaVrksha<K, V> {
    /// Create empty tree (शून्य वृक्ष)
    pub fn nava() -> Self {
        Self {
            mula: None,
            sankhya: 0,
        }
    }

    /// Check if empty (रिक्त)
    pub fn rikta(&self) -> bool {
        self.mula.is_none()
    }

    /// Get size (आकार)
    pub fn akara(&self) -> usize {
        self.sankhya
    }

    /// Insert key-value (प्रविष्ट)
    pub fn pravisht(&mut self, kunji: K, mulya: V) {
        fn insert_rec<K: Ord, V>(
            node: &mut Option<Box<DvidhaGranthi<K, V>>>,
            kunji: K,
            mulya: V,
        ) -> bool {
            match node {
                None => {
                    *node = Some(Box::new(DvidhaGranthi {
                        kunji,
                        mulya,
                        vama: None,
                        dakshina: None,
                    }));
                    true
                }
                Some(n) => match kunji.cmp(&n.kunji) {
                    Ordering::Less => insert_rec(&mut n.vama, kunji, mulya),
                    Ordering::Greater => insert_rec(&mut n.dakshina, kunji, mulya),
                    Ordering::Equal => {
                        n.mulya = mulya;
                        false
                    }
                },
            }
        }

        if insert_rec(&mut self.mula, kunji, mulya) {
            self.sankhya += 1;
        }
    }

    /// Search for key (खोज)
    pub fn khoj(&self, kunji: &K) -> Option<&V> {
        fn search_rec<'a, K: Ord, V>(
            node: &'a Option<Box<DvidhaGranthi<K, V>>>,
            kunji: &K,
        ) -> Option<&'a V> {
            match node {
                None => None,
                Some(n) => match kunji.cmp(&n.kunji) {
                    Ordering::Less => search_rec(&n.vama, kunji),
                    Ordering::Greater => search_rec(&n.dakshina, kunji),
                    Ordering::Equal => Some(&n.mulya),
                },
            }
        }
        search_rec(&self.mula, kunji)
    }

    /// Check if contains key (vidyamana)
    pub fn vidyamana(&self, kunji: &K) -> bool {
        self.khoj(kunji).is_some()
    }

    /// Get minimum key (न्यूनतम)
    pub fn nyunatam(&self) -> Option<&K> {
        fn min_rec<K, V>(node: &Option<Box<DvidhaGranthi<K, V>>>) -> Option<&K> {
            match node {
                None => None,
                Some(n) => {
                    if n.vama.is_none() {
                        Some(&n.kunji)
                    } else {
                        min_rec(&n.vama)
                    }
                }
            }
        }
        min_rec(&self.mula)
    }

    /// Get maximum key (अधिकतम)
    pub fn adhikatam(&self) -> Option<&K> {
        fn max_rec<K, V>(node: &Option<Box<DvidhaGranthi<K, V>>>) -> Option<&K> {
            match node {
                None => None,
                Some(n) => {
                    if n.dakshina.is_none() {
                        Some(&n.kunji)
                    } else {
                        max_rec(&n.dakshina)
                    }
                }
            }
        }
        max_rec(&self.mula)
    }

    /// In-order traversal (क्रमिक भ्रमण)
    pub fn kramik_bhraman(&self) -> Vec<(&K, &V)> {
        let mut result = Vec::new();
        fn inorder_rec<'a, K, V>(
            node: &'a Option<Box<DvidhaGranthi<K, V>>>,
            result: &mut Vec<(&'a K, &'a V)>,
        ) {
            if let Some(n) = node {
                inorder_rec(&n.vama, result);
                result.push((&n.kunji, &n.mulya));
                inorder_rec(&n.dakshina, result);
            }
        }
        inorder_rec(&self.mula, &mut result);
        result
    }

    /// Tree height (ऊँचाई)
    pub fn unchai(&self) -> usize {
        fn height_rec<K, V>(node: &Option<Box<DvidhaGranthi<K, V>>>) -> usize {
            match node {
                None => 0,
                Some(n) => 1 + height_rec(&n.vama).max(height_rec(&n.dakshina)),
            }
        }
        height_rec(&self.mula)
    }
}

// ============================================================================
// AVL TREE (संतुलित वृक्ष)
// ============================================================================

/// AVL Tree Node (संतुलित ग्रन्थि)
#[cfg(feature = "alloc")]
pub struct AvlGranthi<K, V> {
    kunji: K,
    mulya: V,
    vama: Option<Box<AvlGranthi<K, V>>>,
    dakshina: Option<Box<AvlGranthi<K, V>>>,
    unchai: i32, // Height for balance
}

/// AVL Tree - Self-balancing BST (स्वसंतुलित वृक्ष)
///
/// # Etymology
/// स्वसंतुलित (svasantulita) = self-balancing
#[cfg(feature = "alloc")]
pub struct AvlVrksha<K, V> {
    mula: Option<Box<AvlGranthi<K, V>>>,
    sankhya: usize,
}

#[cfg(feature = "alloc")]
impl<K, V> AvlGranthi<K, V> {
    fn nava(kunji: K, mulya: V) -> Box<Self> {
        Box::new(Self {
            kunji,
            mulya,
            vama: None,
            dakshina: None,
            unchai: 1,
        })
    }

    fn get_unchai(node: &Option<Box<AvlGranthi<K, V>>>) -> i32 {
        match node {
            None => 0,
            Some(n) => n.unchai,
        }
    }

    fn update_unchai(&mut self) {
        self.unchai = 1 + Self::get_unchai(&self.vama).max(Self::get_unchai(&self.dakshina));
    }

    fn santulan_karak(&self) -> i32 {
        Self::get_unchai(&self.vama) - Self::get_unchai(&self.dakshina)
    }
}

#[cfg(feature = "alloc")]
impl<K: Ord, V> AvlVrksha<K, V> {
    /// Create empty AVL tree
    pub fn nava() -> Self {
        Self {
            mula: None,
            sankhya: 0,
        }
    }

    /// Right rotation (दक्षिणावर्त)
    fn dakshinavartan(mut y: Box<AvlGranthi<K, V>>) -> Box<AvlGranthi<K, V>> {
        let mut x = y.vama.take().expect("Left child must exist");
        y.vama = x.dakshina.take();
        y.update_unchai();
        x.dakshina = Some(y);
        x.update_unchai();
        x
    }

    /// Left rotation (वामावर्त)
    fn vamavartan(mut x: Box<AvlGranthi<K, V>>) -> Box<AvlGranthi<K, V>> {
        let mut y = x.dakshina.take().expect("Right child must exist");
        x.dakshina = y.vama.take();
        x.update_unchai();
        y.vama = Some(x);
        y.update_unchai();
        y
    }

    /// Insert with balancing (प्रविष्ट संतुलन)
    pub fn pravisht(&mut self, kunji: K, mulya: V) {
        fn insert_rec<K: Ord, V>(
            node: Option<Box<AvlGranthi<K, V>>>,
            kunji: K,
            mulya: V,
        ) -> (Option<Box<AvlGranthi<K, V>>>, bool) {
            match node {
                None => (Some(AvlGranthi::nava(kunji, mulya)), true),
                Some(mut n) => {
                    let added = match kunji.cmp(&n.kunji) {
                        Ordering::Less => {
                            let (new_left, added) = insert_rec(n.vama.take(), kunji, mulya);
                            n.vama = new_left;
                            added
                        }
                        Ordering::Greater => {
                            let (new_right, added) = insert_rec(n.dakshina.take(), kunji, mulya);
                            n.dakshina = new_right;
                            added
                        }
                        Ordering::Equal => {
                            n.mulya = mulya;
                            return (Some(n), false);
                        }
                    };

                    n.update_unchai();
                    let balanced = AvlVrksha::balance(n);
                    (Some(balanced), added)
                }
            }
        }

        let (new_root, added) = insert_rec(self.mula.take(), kunji, mulya);
        self.mula = new_root;
        if added {
            self.sankhya += 1;
        }
    }

    /// Balance node after insertion
    fn balance(mut node: Box<AvlGranthi<K, V>>) -> Box<AvlGranthi<K, V>> {
        let balance = node.santulan_karak();

        // Left heavy
        if balance > 1 {
            if let Some(ref vama) = node.vama {
                if vama.santulan_karak() < 0 {
                    // Left-Right case
                    node.vama = Some(Self::vamavartan(node.vama.take().unwrap()));
                }
            }
            return Self::dakshinavartan(node);
        }

        // Right heavy
        if balance < -1 {
            if let Some(ref dakshina) = node.dakshina {
                if dakshina.santulan_karak() > 0 {
                    // Right-Left case
                    node.dakshina = Some(Self::dakshinavartan(node.dakshina.take().unwrap()));
                }
            }
            return Self::vamavartan(node);
        }

        node
    }

    /// Search (खोज)
    pub fn khoj(&self, kunji: &K) -> Option<&V> {
        fn search_rec<'a, K: Ord, V>(
            node: &'a Option<Box<AvlGranthi<K, V>>>,
            kunji: &K,
        ) -> Option<&'a V> {
            match node {
                None => None,
                Some(n) => match kunji.cmp(&n.kunji) {
                    Ordering::Less => search_rec(&n.vama, kunji),
                    Ordering::Greater => search_rec(&n.dakshina, kunji),
                    Ordering::Equal => Some(&n.mulya),
                },
            }
        }
        search_rec(&self.mula, kunji)
    }

    pub fn rikta(&self) -> bool {
        self.mula.is_none()
    }

    pub fn akara(&self) -> usize {
        self.sankhya
    }
}

// ============================================================================
// TRIE / PREFIX TREE (त्रिपत्री वृक्ष)
// ============================================================================

/// Trie Node for prefix matching (त्रिपत्री ग्रन्थि)
#[cfg(feature = "alloc")]
pub struct TripatriGranthi {
    santan: [Option<Box<TripatriGranthi>>; 26], // children (for a-z)
    shabda_anta: bool,                          // is end of word (शब्द अन्त)
}

/// Trie - Prefix Tree (त्रिपत्री वृक्ष)
///
/// Efficient for string prefix operations.
///
/// # Etymology
/// त्रिपत्री from "retrieval" + vṛkṣa
#[cfg(feature = "alloc")]
pub struct TripatriVrksha {
    mula: Box<TripatriGranthi>,
    shabda_sankhya: usize, // word count
}

#[cfg(feature = "alloc")]
impl TripatriGranthi {
    fn nava() -> Box<Self> {
        Box::new(Self {
            santan: Default::default(),
            shabda_anta: false,
        })
    }
}

#[cfg(feature = "alloc")]
impl Default for TripatriGranthi {
    fn default() -> Self {
        Self {
            santan: Default::default(),
            shabda_anta: false,
        }
    }
}

#[cfg(feature = "alloc")]
impl TripatriVrksha {
    /// Create empty trie
    pub fn nava() -> Self {
        Self {
            mula: TripatriGranthi::nava(),
            shabda_sankhya: 0,
        }
    }

    /// Insert word (शब्द प्रविष्ट)
    pub fn shabda_pravisht(&mut self, shabda: &str) {
        let mut node = &mut self.mula;

        for ch in shabda.chars() {
            if !ch.is_ascii_lowercase() {
                continue;
            }
            let idx = (ch as usize) - ('a' as usize);

            if node.santan[idx].is_none() {
                node.santan[idx] = Some(TripatriGranthi::nava());
            }
            node = node.santan[idx].as_mut().unwrap();
        }

        if !node.shabda_anta {
            node.shabda_anta = true;
            self.shabda_sankhya += 1;
        }
    }

    /// Search for word (शब्द खोज)
    pub fn shabda_khoj(&self, shabda: &str) -> bool {
        let mut node = &self.mula;

        for ch in shabda.chars() {
            if !ch.is_ascii_lowercase() {
                return false;
            }
            let idx = (ch as usize) - ('a' as usize);

            match &node.santan[idx] {
                None => return false,
                Some(next) => node = next,
            }
        }

        node.shabda_anta
    }

    /// Check prefix exists (उपसर्ग विद्यमान)
    pub fn upasarga_vidyamana(&self, upasarga: &str) -> bool {
        let mut node = &self.mula;

        for ch in upasarga.chars() {
            if !ch.is_ascii_lowercase() {
                return false;
            }
            let idx = (ch as usize) - ('a' as usize);

            match &node.santan[idx] {
                None => return false,
                Some(next) => node = next,
            }
        }

        true
    }

    /// Find all words with prefix (उपसर्ग शब्द)
    pub fn upasarga_shabda(&self, upasarga: &str) -> Vec<String> {
        let mut result = Vec::new();

        // Navigate to prefix node
        let mut node = &self.mula;
        for ch in upasarga.chars() {
            if !ch.is_ascii_lowercase() {
                return result;
            }
            let idx = (ch as usize) - ('a' as usize);
            match &node.santan[idx] {
                None => return result,
                Some(next) => node = next,
            }
        }

        // Collect all words from this node
        fn collect(node: &TripatriGranthi, prefix: &mut String, result: &mut Vec<String>) {
            if node.shabda_anta {
                result.push(prefix.clone());
            }

            for (i, child) in node.santan.iter().enumerate() {
                if let Some(c) = child {
                    let ch = (b'a' + i as u8) as char;
                    prefix.push(ch);
                    collect(c, prefix, result);
                    prefix.pop();
                }
            }
        }

        let mut prefix = String::from(upasarga);
        collect(node, &mut prefix, &mut result);
        result
    }

    pub fn akara(&self) -> usize {
        self.shabda_sankhya
    }
}

// ============================================================================
// SEGMENT TREE (खंड वृक्ष)
// ============================================================================

/// Segment Tree for range queries (खंड वृक्ष)
///
/// Efficient range queries and point updates.
///
/// # Etymology
/// खंड (khaṇḍa) = segment
#[cfg(feature = "alloc")]
pub struct KhandaVrksha<T> {
    vrksha: Vec<T>,
    akara: usize,
    sammelana: fn(&T, &T) -> T, // combine function
}

#[cfg(feature = "alloc")]
impl<T: Clone + Default> KhandaVrksha<T> {
    /// Build segment tree (निर्माण)
    pub fn nirman(data: &[T], sammelana: fn(&T, &T) -> T) -> Self {
        let n = data.len();
        let mut vrksha = vec![T::default(); 4 * n];

        fn build<T: Clone>(
            arr: &[T],
            vrksha: &mut Vec<T>,
            v: usize,
            tl: usize,
            tr: usize,
            combine: fn(&T, &T) -> T,
        ) {
            if tl == tr {
                if tl < arr.len() {
                    vrksha[v] = arr[tl].clone();
                }
            } else {
                let tm = (tl + tr) / 2;
                build(arr, vrksha, 2 * v, tl, tm, combine);
                build(arr, vrksha, 2 * v + 1, tm + 1, tr, combine);
                vrksha[v] = combine(&vrksha[2 * v], &vrksha[2 * v + 1]);
            }
        }

        if n > 0 {
            build(data, &mut vrksha, 1, 0, n - 1, sammelana);
        }

        Self {
            vrksha,
            akara: n,
            sammelana,
        }
    }

    /// Range query (परास प्रश्न)
    pub fn paras_prashna(&self, vama: usize, dakshina: usize) -> T {
        fn query<T: Clone + Default>(
            vrksha: &Vec<T>,
            v: usize,
            tl: usize,
            tr: usize,
            l: usize,
            r: usize,
            combine: fn(&T, &T) -> T,
        ) -> T {
            if l > r {
                return T::default();
            }
            if l == tl && r == tr {
                return vrksha[v].clone();
            }
            let tm = (tl + tr) / 2;
            let left = query(vrksha, 2 * v, tl, tm, l, tm.min(r), combine);
            let right = query(vrksha, 2 * v + 1, tm + 1, tr, (tm + 1).max(l), r, combine);
            combine(&left, &right)
        }

        if self.akara == 0 || dakshina >= self.akara {
            return T::default();
        }

        query(
            &self.vrksha,
            1,
            0,
            self.akara - 1,
            vama,
            dakshina,
            self.sammelana,
        )
    }

    /// Point update (bindu parivartan)
    pub fn bindu_parivartan(&mut self, sthana: usize, mulya: T) {
        fn update<T: Clone>(
            vrksha: &mut Vec<T>,
            v: usize,
            tl: usize,
            tr: usize,
            pos: usize,
            val: T,
            combine: fn(&T, &T) -> T,
        ) {
            if tl == tr {
                vrksha[v] = val;
            } else {
                let tm = (tl + tr) / 2;
                if pos <= tm {
                    update(vrksha, 2 * v, tl, tm, pos, val, combine);
                } else {
                    update(vrksha, 2 * v + 1, tm + 1, tr, pos, val, combine);
                }
                vrksha[v] = combine(&vrksha[2 * v], &vrksha[2 * v + 1]);
            }
        }

        if sthana < self.akara {
            update(
                &mut self.vrksha,
                1,
                0,
                self.akara - 1,
                sthana,
                mulya,
                self.sammelana,
            );
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
    fn test_bst_basic() {
        let mut vrksha: DvidhaVrksha<i32, &str> = DvidhaVrksha::nava();
        assert!(vrksha.rikta());

        vrksha.pravisht(5, "five");
        vrksha.pravisht(3, "three");
        vrksha.pravisht(7, "seven");
        vrksha.pravisht(1, "one");
        vrksha.pravisht(9, "nine");

        assert_eq!(vrksha.akara(), 5);
        assert_eq!(vrksha.khoj(&5), Some(&"five"));
        assert_eq!(vrksha.khoj(&100), None);
        assert_eq!(vrksha.nyunatam(), Some(&1));
        assert_eq!(vrksha.adhikatam(), Some(&9));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_bst_traversal() {
        let mut vrksha: DvidhaVrksha<i32, i32> = DvidhaVrksha::nava();
        vrksha.pravisht(5, 50);
        vrksha.pravisht(3, 30);
        vrksha.pravisht(7, 70);

        let traversal = vrksha.kramik_bhraman();
        assert_eq!(traversal.len(), 3);
        assert_eq!(traversal[0], (&3, &30));
        assert_eq!(traversal[1], (&5, &50));
        assert_eq!(traversal[2], (&7, &70));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_avl_balance() {
        let mut vrksha: AvlVrksha<i32, i32> = AvlVrksha::nava();

        // Insert in order that would create imbalance in BST
        for i in 1..=7 {
            vrksha.pravisht(i, i * 10);
        }

        assert_eq!(vrksha.akara(), 7);
        assert_eq!(vrksha.khoj(&4), Some(&40));
        assert_eq!(vrksha.khoj(&10), None);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_trie() {
        let mut trie = TripatriVrksha::nava();

        trie.shabda_pravisht("apple");
        trie.shabda_pravisht("app");
        trie.shabda_pravisht("application");
        trie.shabda_pravisht("banana");

        assert!(trie.shabda_khoj("apple"));
        assert!(trie.shabda_khoj("app"));
        assert!(!trie.shabda_khoj("ap"));
        assert!(trie.upasarga_vidyamana("ap"));

        let words = trie.upasarga_shabda("app");
        assert_eq!(words.len(), 3);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_segment_tree() {
        let data = vec![1i64, 3, 5, 7, 9, 11];
        let st = KhandaVrksha::nirman(&data, |a, b| a + b);

        // Sum of range [1, 3] = 3 + 5 + 7 = 15
        assert_eq!(st.paras_prashna(1, 3), 15);

        // Sum of entire array
        assert_eq!(st.paras_prashna(0, 5), 36);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_segment_tree_update() {
        let data = vec![1i64, 3, 5, 7, 9, 11];
        let mut st = KhandaVrksha::nirman(&data, |a, b| a + b);

        assert_eq!(st.paras_prashna(0, 5), 36);

        // Update index 2 from 5 to 10
        st.bindu_parivartan(2, 10);

        // New sum = 36 - 5 + 10 = 41
        assert_eq!(st.paras_prashna(0, 5), 41);
    }
}
