//! # Sānketika - Hash-based Structures (सांकेतिक)
//!
//! Hash maps, hash sets, and related structures.
//!
//! > **"कूटेन कोषम्"**
//! > *"Treasury by code"*
//!
//! ## Structures
//!
//! - `SanketikaPatra` - HashMap
//! - `SanketikaSamudaya` - HashSet
//! - `KramitPatra` - LinkedHashMap
//! - `BahuPatra` - MultiMap

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::vec;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use core::hash::{Hash, Hasher};

// ============================================================================
// SIMPLE HASH FUNCTIONS (सरल संकेत कार्य)
// ============================================================================

/// FNV-1a hash (fast, good distribution)
pub fn fnv1a_sanketa(data: &[u8]) -> u64 {
    const FNV_OFFSET: u64 = 14695981039346656037;
    const FNV_PRIME: u64 = 1099511628211;

    let mut hash = FNV_OFFSET;
    for byte in data {
        hash ^= *byte as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }
    hash
}

/// DJB2 hash (simple, fast)
pub fn djb2_sanketa(data: &[u8]) -> u64 {
    let mut hash: u64 = 5381;
    for byte in data {
        hash = ((hash << 5).wrapping_add(hash)).wrapping_add(*byte as u64);
    }
    hash
}

/// Polynomial rolling hash
pub fn bahupada_sanketa(data: &[u8], base: u64, modulo: u64) -> u64 {
    let mut hash: u64 = 0;
    let mut pow: u64 = 1;

    for byte in data {
        hash = (hash + (*byte as u64) * pow) % modulo;
        pow = (pow * base) % modulo;
    }
    hash
}

// ============================================================================
// HASH MAP (सांकेतिक पत्र)
// ============================================================================

const DEFAULT_CAPACITY: usize = 16;
const LOAD_FACTOR: f32 = 0.75;

/// HashMap entry (प्रविष्टि)
#[cfg(feature = "alloc")]
#[derive(Clone)]
struct Pravishti<K, V> {
    kunji: K,     // key
    mulya: V,     // value
    sanketa: u64, // cached hash
}

/// HashMap bucket (कलश)
#[cfg(feature = "alloc")]
type Kalasha<K, V> = Vec<Pravishti<K, V>>;

/// HashMap (सांकेतिक पत्र)
///
/// Open addressing with chaining.
///
/// # Etymology
/// सांकेतिक (sānketika) = symbolic, encoded
/// पत्र (patra) = document, map
#[cfg(feature = "alloc")]
pub struct SanketikaPatra<K, V> {
    kalasha: Vec<Kalasha<K, V>>,
    sankhya: usize, // count
}

#[cfg(feature = "alloc")]
impl<K: Hash + Eq, V> SanketikaPatra<K, V> {
    /// Create empty map (शून्य पत्र)
    pub fn nava() -> Self {
        Self::nava_kshamata(DEFAULT_CAPACITY)
    }

    /// Create with capacity
    pub fn nava_kshamata(kshamata: usize) -> Self {
        let kshamata = kshamata.max(1);
        Self {
            kalasha: (0..kshamata).map(|_| Vec::new()).collect(),
            sankhya: 0,
        }
    }

    /// Hash a key
    fn hash_kunji(kunji: &K) -> u64 {
        use core::hash::BuildHasher;

        // Simple hasher implementation
        struct SimpleHasher(u64);

        impl Hasher for SimpleHasher {
            fn finish(&self) -> u64 {
                self.0
            }
            fn write(&mut self, bytes: &[u8]) {
                for byte in bytes {
                    self.0 = self.0.wrapping_mul(31).wrapping_add(*byte as u64);
                }
            }
        }

        let mut hasher = SimpleHasher(0);
        kunji.hash(&mut hasher);
        hasher.finish()
    }

    /// Get bucket index
    fn kalasha_anukram(&self, sanketa: u64) -> usize {
        (sanketa as usize) % self.kalasha.len()
    }

    /// Check if empty
    pub fn rikta(&self) -> bool {
        self.sankhya == 0
    }

    /// Get count
    pub fn akara(&self) -> usize {
        self.sankhya
    }

    /// Insert key-value (प्रविष्ट)
    pub fn pravisht(&mut self, kunji: K, mulya: V) -> Option<V> {
        // Check if resize needed
        if (self.sankhya + 1) as f32 > self.kalasha.len() as f32 * LOAD_FACTOR {
            self.punargathan();
        }

        let sanketa = Self::hash_kunji(&kunji);
        let idx = self.kalasha_anukram(sanketa);

        // Check for existing key
        for pravishti in &mut self.kalasha[idx] {
            if pravishti.kunji == kunji {
                let old = core::mem::replace(&mut pravishti.mulya, mulya);
                return Some(old);
            }
        }

        // Insert new
        self.kalasha[idx].push(Pravishti {
            kunji,
            mulya,
            sanketa,
        });
        self.sankhya += 1;
        None
    }

    /// Get value by key (प्राप्त)
    pub fn prapta(&self, kunji: &K) -> Option<&V> {
        let sanketa = Self::hash_kunji(kunji);
        let idx = self.kalasha_anukram(sanketa);

        for pravishti in &self.kalasha[idx] {
            if &pravishti.kunji == kunji {
                return Some(&pravishti.mulya);
            }
        }
        None
    }

    /// Get mutable value (परिवर्तनीय प्राप्त)
    pub fn prapta_mut(&mut self, kunji: &K) -> Option<&mut V> {
        let sanketa = Self::hash_kunji(kunji);
        let idx = self.kalasha_anukram(sanketa);

        for pravishti in &mut self.kalasha[idx] {
            if &pravishti.kunji == kunji {
                return Some(&mut pravishti.mulya);
            }
        }
        None
    }

    /// Check if contains key (विद्यमान)
    pub fn vidyamana(&self, kunji: &K) -> bool {
        self.prapta(kunji).is_some()
    }

    /// Remove key (निष्कासित)
    pub fn nishkasit(&mut self, kunji: &K) -> Option<V> {
        let sanketa = Self::hash_kunji(kunji);
        let idx = self.kalasha_anukram(sanketa);

        let kalasha = &mut self.kalasha[idx];
        for i in 0..kalasha.len() {
            if &kalasha[i].kunji == kunji {
                self.sankhya -= 1;
                return Some(kalasha.swap_remove(i).mulya);
            }
        }
        None
    }

    /// Resize and rehash (पुनर्गठन)
    fn punargathan(&mut self) {
        let new_size = self.kalasha.len() * 2;
        let mut new_kalasha: Vec<Kalasha<K, V>> = (0..new_size).map(|_| Vec::new()).collect();

        for kalasha in self.kalasha.drain(..) {
            for pravishti in kalasha {
                let idx = (pravishti.sanketa as usize) % new_size;
                new_kalasha[idx].push(pravishti);
            }
        }

        self.kalasha = new_kalasha;
    }

    /// Get or insert (प्राप्त अथवा प्रविष्ट)
    pub fn prapta_athva_pravisht(&mut self, kunji: K, mulya: V) -> &mut V
    where
        K: Clone,
    {
        if !self.vidyamana(&kunji) {
            self.pravisht(kunji.clone(), mulya);
        }
        self.prapta_mut(&kunji).unwrap()
    }

    /// Iterate over entries (प्रत्येक)
    pub fn pratyeka(&self) -> impl Iterator<Item = (&K, &V)> {
        self.kalasha
            .iter()
            .flat_map(|k| k.iter())
            .map(|p| (&p.kunji, &p.mulya))
    }

    /// Get all keys (कुञ्जी सूची)
    pub fn kunji_suci(&self) -> Vec<&K> {
        self.pratyeka().map(|(k, _)| k).collect()
    }

    /// Get all values (मूल्य सूची)
    pub fn mulya_suci(&self) -> Vec<&V> {
        self.pratyeka().map(|(_, v)| v).collect()
    }

    /// Clear all entries (शुद्ध)
    pub fn shuddha(&mut self) {
        for kalasha in &mut self.kalasha {
            kalasha.clear();
        }
        self.sankhya = 0;
    }
}

// ============================================================================
// HASH SET (सांकेतिक समुदाय)
// ============================================================================

/// HashSet (सांकेतिक समुदाय)
///
/// # Etymology
/// समुदाय (samudāya) = collection, set
#[cfg(feature = "alloc")]
pub struct SanketikaSamudaya<T> {
    patra: SanketikaPatra<T, ()>,
}

#[cfg(feature = "alloc")]
impl<T: Hash + Eq> SanketikaSamudaya<T> {
    /// Create empty set
    pub fn nava() -> Self {
        Self {
            patra: SanketikaPatra::nava(),
        }
    }

    pub fn rikta(&self) -> bool {
        self.patra.rikta()
    }

    pub fn akara(&self) -> usize {
        self.patra.akara()
    }

    /// Insert element (प्रविष्ट)
    pub fn pravisht(&mut self, tattva: T) -> bool {
        self.patra.pravisht(tattva, ()).is_none()
    }

    /// Check if contains (विद्यमान)
    pub fn vidyamana(&self, tattva: &T) -> bool {
        self.patra.vidyamana(tattva)
    }

    /// Remove element (निष्कासित)
    pub fn nishkasit(&mut self, tattva: &T) -> bool {
        self.patra.nishkasit(tattva).is_some()
    }

    /// Iterate over elements
    pub fn pratyeka(&self) -> impl Iterator<Item = &T> {
        self.patra.pratyeka().map(|(k, _)| k)
    }

    /// Union with another set (संघ)
    pub fn sangha(&self, anya: &Self) -> Self
    where
        T: Clone,
    {
        let mut result = Self::nava();
        for tattva in self.pratyeka() {
            result.pravisht(tattva.clone());
        }
        for tattva in anya.pratyeka() {
            result.pravisht(tattva.clone());
        }
        result
    }

    /// Intersection (प्रतिच्छेद)
    pub fn praticcheda(&self, anya: &Self) -> Self
    where
        T: Clone,
    {
        let mut result = Self::nava();
        for tattva in self.pratyeka() {
            if anya.vidyamana(tattva) {
                result.pravisht(tattva.clone());
            }
        }
        result
    }

    /// Difference (अन्तर)
    pub fn antara(&self, anya: &Self) -> Self
    where
        T: Clone,
    {
        let mut result = Self::nava();
        for tattva in self.pratyeka() {
            if !anya.vidyamana(tattva) {
                result.pravisht(tattva.clone());
            }
        }
        result
    }
}

// ============================================================================
// BLOOM FILTER (प्रस्फुटन छन्नक)
// ============================================================================

/// Bloom Filter for probabilistic membership testing
///
/// # Etymology
/// प्रस्फुटन (prasphuṭana) = bloom, expansion
/// छन्नक (channaka) = filter
#[cfg(feature = "alloc")]
pub struct PrasphutanaChannaka {
    bits: Vec<bool>,
    hash_count: usize,
}

#[cfg(feature = "alloc")]
impl PrasphutanaChannaka {
    /// Create with specified size and hash count
    pub fn nava(akara: usize, hash_sankhya: usize) -> Self {
        Self {
            bits: vec![false; akara],
            hash_count: hash_sankhya,
        }
    }

    /// Create with optimal parameters for n elements with false positive rate p
    pub fn nava_optimal(n: usize, p: f64) -> Self {
        let m = (-(n as f64) * p.ln() / (2.0_f64.ln().powi(2))).ceil() as usize;
        let k = ((m as f64 / n as f64) * 2.0_f64.ln()).ceil() as usize;
        Self::nava(m.max(1), k.max(1))
    }

    /// Get hash positions
    fn hash_sthiti(&self, data: &[u8]) -> impl Iterator<Item = usize> + '_ {
        let h1 = fnv1a_sanketa(data);
        let h2 = djb2_sanketa(data);
        let m = self.bits.len();

        (0..self.hash_count).map(move |i| {
            let combined = h1.wrapping_add((i as u64).wrapping_mul(h2));
            (combined as usize) % m
        })
    }

    /// Add element (प्रविष्ट)
    pub fn pravisht(&mut self, data: &[u8]) {
        let positions: alloc::vec::Vec<_> = self.hash_sthiti(data).collect();
        for pos in positions {
            self.bits[pos] = true;
        }
    }

    /// Check membership (possibly with false positives) (सम्भव विद्यमान)
    pub fn sambhava_vidyamana(&self, data: &[u8]) -> bool {
        for pos in self.hash_sthiti(data) {
            if !self.bits[pos] {
                return false;
            }
        }
        true
    }

    /// Clear filter (शुद्ध)
    pub fn shuddha(&mut self) {
        for bit in &mut self.bits {
            *bit = false;
        }
    }
}

// ============================================================================
// COUNT-MIN SKETCH (गणना-न्यूनतम रेखाचित्र)
// ============================================================================

/// Count-Min Sketch for frequency estimation
///
/// # Etymology
/// गणना (gaṇanā) = counting
/// न्यूनतम (nyūnatama) = minimum
/// रेखाचित्र (rekhācitra) = sketch
#[cfg(feature = "alloc")]
pub struct GananaNyunatamaRekha {
    counts: Vec<Vec<u64>>,
    width: usize,
    depth: usize,
}

#[cfg(feature = "alloc")]
impl GananaNyunatamaRekha {
    /// Create sketch with specified width and depth
    pub fn nava(chaurai: usize, gahrai: usize) -> Self {
        Self {
            counts: vec![vec![0; chaurai]; gahrai],
            width: chaurai,
            depth: gahrai,
        }
    }

    /// Get hash positions for each row
    fn hash_sthiti(&self, data: &[u8]) -> Vec<usize> {
        let h1 = fnv1a_sanketa(data);
        let h2 = djb2_sanketa(data);

        (0..self.depth)
            .map(|i| {
                let combined = h1.wrapping_add((i as u64).wrapping_mul(h2));
                (combined as usize) % self.width
            })
            .collect()
    }

    /// Add element (बढ़ाना)
    pub fn badhana(&mut self, data: &[u8]) {
        let positions = self.hash_sthiti(data);
        for (row, &col) in positions.iter().enumerate() {
            self.counts[row][col] = self.counts[row][col].saturating_add(1);
        }
    }

    /// Estimate count (अनुमानित गणना)
    pub fn anumanit_ganana(&self, data: &[u8]) -> u64 {
        let positions = self.hash_sthiti(data);
        positions
            .iter()
            .enumerate()
            .map(|(row, &col)| self.counts[row][col])
            .min()
            .unwrap_or(0)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_functions() {
        let data = b"hello world";

        let h1 = fnv1a_sanketa(data);
        let h2 = djb2_sanketa(data);

        assert_ne!(h1, 0);
        assert_ne!(h2, 0);

        // Same input should give same hash
        assert_eq!(fnv1a_sanketa(data), h1);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_hashmap_basic() {
        let mut map: SanketikaPatra<&str, i32> = SanketikaPatra::nava();

        assert!(map.rikta());

        map.pravisht("one", 1);
        map.pravisht("two", 2);
        map.pravisht("three", 3);

        assert_eq!(map.akara(), 3);
        assert_eq!(map.prapta(&"two"), Some(&2));
        assert_eq!(map.prapta(&"four"), None);
        assert!(map.vidyamana(&"one"));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_hashmap_update() {
        let mut map: SanketikaPatra<i32, i32> = SanketikaPatra::nava();

        map.pravisht(1, 10);
        assert_eq!(map.pravisht(1, 20), Some(10));
        assert_eq!(map.prapta(&1), Some(&20));
        assert_eq!(map.akara(), 1);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_hashmap_remove() {
        let mut map: SanketikaPatra<i32, i32> = SanketikaPatra::nava();

        map.pravisht(1, 10);
        map.pravisht(2, 20);

        assert_eq!(map.nishkasit(&1), Some(10));
        assert_eq!(map.akara(), 1);
        assert!(!map.vidyamana(&1));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_hashset() {
        let mut set: SanketikaSamudaya<i32> = SanketikaSamudaya::nava();

        assert!(set.pravisht(1));
        assert!(set.pravisht(2));
        assert!(!set.pravisht(1)); // Duplicate

        assert_eq!(set.akara(), 2);
        assert!(set.vidyamana(&1));
        assert!(!set.vidyamana(&3));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_hashset_operations() {
        let mut set1: SanketikaSamudaya<i32> = SanketikaSamudaya::nava();
        let mut set2: SanketikaSamudaya<i32> = SanketikaSamudaya::nava();

        set1.pravisht(1);
        set1.pravisht(2);
        set1.pravisht(3);

        set2.pravisht(2);
        set2.pravisht(3);
        set2.pravisht(4);

        let union = set1.sangha(&set2);
        assert_eq!(union.akara(), 4);

        let inter = set1.praticcheda(&set2);
        assert_eq!(inter.akara(), 2);
        assert!(inter.vidyamana(&2));
        assert!(inter.vidyamana(&3));

        let diff = set1.antara(&set2);
        assert_eq!(diff.akara(), 1);
        assert!(diff.vidyamana(&1));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_bloom_filter() {
        let mut bloom = PrasphutanaChannaka::nava(1000, 3);

        bloom.pravisht(b"hello");
        bloom.pravisht(b"world");

        assert!(bloom.sambhava_vidyamana(b"hello"));
        assert!(bloom.sambhava_vidyamana(b"world"));
        // May have false positives, but not false negatives
        // This test just checks basic functionality
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_count_min_sketch() {
        let mut sketch = GananaNyunatamaRekha::nava(1000, 5);

        for _ in 0..10 {
            sketch.badhana(b"apple");
        }
        for _ in 0..5 {
            sketch.badhana(b"banana");
        }

        // Estimates should be at least as large as actual counts
        assert!(sketch.anumanit_ganana(b"apple") >= 10);
        assert!(sketch.anumanit_ganana(b"banana") >= 5);
    }
}
