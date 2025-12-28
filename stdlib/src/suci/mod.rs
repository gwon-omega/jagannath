//! Sūcī - Collections (सूची)
//!
//! Vector, HashMap, and other collection types.

#[cfg(feature = "alloc")]
use alloc::vec::Vec as AllocVec;

// Sub-modules
#[cfg(feature = "alloc")]
pub mod srinkhala;
#[cfg(feature = "alloc")]
pub mod pradhanyata;

#[cfg(feature = "alloc")]
pub use srinkhala::*;
#[cfg(feature = "alloc")]
pub use pradhanyata::*;

/// Vector/List (Sūcī - सूची)
#[cfg(feature = "alloc")]
pub type Suci<T> = AllocVec<T>;

/// Vector operations trait
#[cfg(feature = "alloc")]
pub trait SuciVidhi<T> {
    /// Create empty (नव)
    fn nava() -> Self;

    /// Create with capacity (क्षमता)
    fn kshamata(capacity: usize) -> Self;

    /// Push item (योजय)
    fn yojaya(&mut self, item: T);

    /// Pop item (निष्कासय)
    fn nishkasaya(&mut self) -> Option<T>;

    /// Length (दीर्घता)
    fn dirghata(&self) -> usize;

    /// Is empty (रिक्त)
    fn rikta(&self) -> bool;

    /// Clear (शुद्ध)
    fn shuddha(&mut self);
}

#[cfg(feature = "alloc")]
impl<T> SuciVidhi<T> for Suci<T> {
    fn nava() -> Self {
        AllocVec::new()
    }

    fn kshamata(capacity: usize) -> Self {
        AllocVec::with_capacity(capacity)
    }

    fn yojaya(&mut self, item: T) {
        self.push(item);
    }

    fn nishkasaya(&mut self) -> Option<T> {
        self.pop()
    }

    fn dirghata(&self) -> usize {
        self.len()
    }

    fn rikta(&self) -> bool {
        self.is_empty()
    }

    fn shuddha(&mut self) {
        self.clear();
    }
}

/// HashMap (Sāraṇī - सारणी)
#[cfg(feature = "std")]
pub type Sarani<K, V> = std::collections::HashMap<K, V>;

/// HashSet (Samuccaya - समुच्चय)
#[cfg(feature = "std")]
pub type Samuccaya<T> = std::collections::HashSet<T>;

/// BTreeMap (Vṛkṣa-Sāraṇī - वृक्षसारणी)
#[cfg(feature = "alloc")]
pub type VrkshaSarani<K, V> = alloc::collections::BTreeMap<K, V>;

/// BTreeSet (Vṛkṣa-Samuccaya - वृक्षसमुच्चय)
#[cfg(feature = "alloc")]
pub type VrkshaSamuccaya<T> = alloc::collections::BTreeSet<T>;

/// Deque (Dvimukha - द्विमुख)
#[cfg(feature = "alloc")]
pub type Dvimukha<T> = alloc::collections::VecDeque<T>;
