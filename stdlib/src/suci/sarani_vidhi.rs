//! Sāraṇī Vidhi - HashMap Operations (सारणी विधि)
//!
//! Operation traits for hash maps (key-value stores).
//!
//! Based on सारणी (sāraṇī) - meaning table, list, or index.

#[cfg(feature = "std")]
use std::collections::HashMap;
#[cfg(feature = "std")]
use std::hash::Hash;

/// HashMap operations trait (Sāraṇī-Vidhi - सारणीविधि)
#[cfg(feature = "std")]
pub trait SaraniVidhi<K, V> {
    /// Create empty map (नव - nava)
    fn nava() -> Self;

    /// Create with capacity (क्षमता - kṣamatā)
    fn kshamata(capacity: usize) -> Self;

    /// Insert key-value (स्थापय - sthāpaya)
    fn sthapaya(&mut self, kunji: K, mulya: V) -> Option<V>;

    /// Get value by key (लभस्व - labhasva)
    fn labhasva(&self, kunji: &K) -> Option<&V>;

    /// Get mutable value (परिवर्तनीय लभस्व)
    fn parivartaniya_labhasva(&mut self, kunji: &K) -> Option<&mut V>;

    /// Remove by key (निष्कासय - niṣkāsaya)
    fn nishkasaya(&mut self, kunji: &K) -> Option<V>;

    /// Check if key exists (अस्ति - asti)
    fn asti(&self, kunji: &K) -> bool;

    /// Length (दीर्घता - dīrghatā)
    fn dirghata(&self) -> usize;

    /// Is empty (रिक्त - rikta)
    fn rikta(&self) -> bool;

    /// Clear all (शुद्ध - śuddha)
    fn shuddha(&mut self);
}

#[cfg(feature = "std")]
impl<K: Eq + Hash, V> SaraniVidhi<K, V> for HashMap<K, V> {
    fn nava() -> Self {
        HashMap::new()
    }

    fn kshamata(capacity: usize) -> Self {
        HashMap::with_capacity(capacity)
    }

    fn sthapaya(&mut self, kunji: K, mulya: V) -> Option<V> {
        self.insert(kunji, mulya)
    }

    fn labhasva(&self, kunji: &K) -> Option<&V> {
        self.get(kunji)
    }

    fn parivartaniya_labhasva(&mut self, kunji: &K) -> Option<&mut V> {
        self.get_mut(kunji)
    }

    fn nishkasaya(&mut self, kunji: &K) -> Option<V> {
        self.remove(kunji)
    }

    fn asti(&self, kunji: &K) -> bool {
        self.contains_key(kunji)
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

/// Extended HashMap operations (Vistarita Sāraṇī Vidhi - विस्तारित सारणी विधि)
#[cfg(feature = "std")]
pub trait VistaritaSaraniVidhi<K, V>: SaraniVidhi<K, V> {
    /// Get or insert with default (लभस्व अथवा स्थापय - labhasva athavā sthāpaya)
    fn labhasva_athava_sthapaya(&mut self, kunji: K, pratiyogi: V) -> &mut V;

    /// Get or insert with function (कार्य सहित स्थापय)
    fn karya_sahita_sthapaya<F: FnOnce() -> V>(&mut self, kunji: K, f: F) -> &mut V;

    /// Retain entries matching predicate (धारय - dhāraya)
    fn dharaya<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut V) -> bool;
}

#[cfg(feature = "std")]
impl<K: Eq + Hash, V> VistaritaSaraniVidhi<K, V> for HashMap<K, V> {
    fn labhasva_athava_sthapaya(&mut self, kunji: K, pratiyogi: V) -> &mut V {
        self.entry(kunji).or_insert(pratiyogi)
    }

    fn karya_sahita_sthapaya<F: FnOnce() -> V>(&mut self, kunji: K, f: F) -> &mut V {
        self.entry(kunji).or_insert_with(f)
    }

    fn dharaya<F>(&mut self, f: F)
    where
        F: FnMut(&K, &mut V) -> bool,
    {
        self.retain(f);
    }
}
