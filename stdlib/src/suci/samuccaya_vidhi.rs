//! Samuccaya Vidhi - HashSet Operations (समुच्चय विधि)
//!
//! Operation traits for hash sets (unique element collections).
//!
//! Based on समुच्चय (samuccaya) - meaning aggregate, collection, or set.

#[cfg(feature = "std")]
use std::collections::HashSet;
#[cfg(feature = "std")]
use std::hash::Hash;

/// HashSet operations trait (Samuccaya-Vidhi - समुच्चयविधि)
#[cfg(feature = "std")]
pub trait SamuccayaVidhi<T> {
    /// Create empty set (नव - nava)
    fn nava() -> Self;

    /// Create with capacity (क्षमता - kṣamatā)
    fn kshamata(capacity: usize) -> Self;

    /// Insert element (योजय - yojaya)
    fn yojaya(&mut self, tatva: T) -> bool;

    /// Check if element exists (अस्ति - asti)
    fn asti(&self, tatva: &T) -> bool;

    /// Remove element (निष्कासय - niṣkāsaya)
    fn nishkasaya(&mut self, tatva: &T) -> bool;

    /// Length (दीर्घता - dīrghatā)
    fn dirghata(&self) -> usize;

    /// Is empty (रिक्त - rikta)
    fn rikta(&self) -> bool;

    /// Clear all (शुद्ध - śuddha)
    fn shuddha(&mut self);
}

#[cfg(feature = "std")]
impl<T: Eq + Hash> SamuccayaVidhi<T> for HashSet<T> {
    fn nava() -> Self {
        HashSet::new()
    }

    fn kshamata(capacity: usize) -> Self {
        HashSet::with_capacity(capacity)
    }

    fn yojaya(&mut self, tatva: T) -> bool {
        self.insert(tatva)
    }

    fn asti(&self, tatva: &T) -> bool {
        self.contains(tatva)
    }

    fn nishkasaya(&mut self, tatva: &T) -> bool {
        self.remove(tatva)
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

/// Set theory operations (Samuccaya Gaṇita - समुच्चय गणित)
///
/// Mathematical set operations named after Sanskrit terms.
#[cfg(feature = "std")]
pub trait SamuccayaGanita<T>: SamuccayaVidhi<T> {
    /// Union (संयोग - saṃyoga)
    ///
    /// Elements in either set A or B (A ∪ B)
    fn samyoga(&self, anya: &Self) -> Self;

    /// Intersection (सन्धि - sandhi)
    ///
    /// Elements in both sets A and B (A ∩ B)
    fn sandhi(&self, anya: &Self) -> Self;

    /// Difference (अन्तर - antara)
    ///
    /// Elements in A but not in B (A - B)
    fn antara(&self, anya: &Self) -> Self;

    /// Symmetric difference (समान्तर - samāntara)
    ///
    /// Elements in A or B but not both (A △ B)
    fn samantara(&self, anya: &Self) -> Self;

    /// Is subset (उपसमुच्चय - upasamuccaya)
    ///
    /// True if all elements of self are in other (A ⊆ B)
    fn upasamuccaya(&self, anya: &Self) -> bool;

    /// Is superset (अधिसमुच्चय - adhisamuccaya)
    ///
    /// True if all elements of other are in self (A ⊇ B)
    fn adhisamuccaya(&self, anya: &Self) -> bool;

    /// Is disjoint (विभक्त - vibhakta)
    ///
    /// True if sets have no common elements
    fn vibhakta(&self, anya: &Self) -> bool;
}

#[cfg(feature = "std")]
impl<T: Eq + Hash + Clone> SamuccayaGanita<T> for HashSet<T> {
    fn samyoga(&self, anya: &Self) -> Self {
        self.union(anya).cloned().collect()
    }

    fn sandhi(&self, anya: &Self) -> Self {
        self.intersection(anya).cloned().collect()
    }

    fn antara(&self, anya: &Self) -> Self {
        self.difference(anya).cloned().collect()
    }

    fn samantara(&self, anya: &Self) -> Self {
        self.symmetric_difference(anya).cloned().collect()
    }

    fn upasamuccaya(&self, anya: &Self) -> bool {
        self.is_subset(anya)
    }

    fn adhisamuccaya(&self, anya: &Self) -> bool {
        self.is_superset(anya)
    }

    fn vibhakta(&self, anya: &Self) -> bool {
        self.is_disjoint(anya)
    }
}

// ============================================================================
// BTreeSet operations (Vṛkṣa-Samuccaya - वृक्षसमुच्चय)
// ============================================================================

#[cfg(feature = "alloc")]
use alloc::collections::BTreeSet;

/// Ordered set operations (Kramita-Samuccaya-Vidhi - क्रमितसमुच्चयविधि)
#[cfg(feature = "alloc")]
pub trait KramitaSamuccayaVidhi<T> {
    /// Get minimum element (न्यूनतम - nyūnatama)
    fn nyunatama(&self) -> Option<&T>;

    /// Get maximum element (अधिकतम - adhikatama)
    fn adhikatama(&self) -> Option<&T>;

    /// Get elements in range (परिसर - parisara)
    fn parisara<'a>(&'a self, from: &T, to: &T) -> impl Iterator<Item = &'a T>
    where
        T: 'a;

    /// Pop minimum (न्यूनतम निष्कासय - nyūnatama niṣkāsaya)
    fn nyunatama_nishkasaya(&mut self) -> Option<T>;

    /// Pop maximum (अधिकतम निष्कासय - adhikatama niṣkāsaya)
    fn adhikatama_nishkasaya(&mut self) -> Option<T>;
}

#[cfg(feature = "alloc")]
impl<T: Ord + Clone> KramitaSamuccayaVidhi<T> for BTreeSet<T> {
    fn nyunatama(&self) -> Option<&T> {
        self.first()
    }

    fn adhikatama(&self) -> Option<&T> {
        self.last()
    }

    fn parisara<'a>(&'a self, from: &T, to: &T) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
    {
        self.range(from..to)
    }

    fn nyunatama_nishkasaya(&mut self) -> Option<T> {
        self.pop_first()
    }

    fn adhikatama_nishkasaya(&mut self) -> Option<T> {
        self.pop_last()
    }
}
