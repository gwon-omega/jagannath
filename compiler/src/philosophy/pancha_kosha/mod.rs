//! Pancha Kosha Memory Hierarchy
//!
//! 5-tier memory hierarchy based on the 5 sheaths (koshas):
//! 1. Annamaya (food) - Registers/L1 cache (hottest)
//! 2. Prāṇamaya (breath) - L2/L3 cache
//! 3. Manomaya (mind) - RAM
//! 4. Vijñānamaya (wisdom) - Disk/SSD
//! 5. Ānandamaya (bliss) - Network/Cloud

use std::collections::HashMap;

use crate::traits::{PhilosophicalEnum, SanskritDescribed, SanskritNamed};

/// The 5 Koshas (memory tiers)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Kosha {
    /// Annamaya (physical) - Registers/L1 cache
    /// Fastest, smallest, most precious
    Annamaya = 1,

    /// Prāṇamaya (vital) - L2/L3 cache
    /// Fast, limited
    Pranamaya = 2,

    /// Manomaya (mental) - RAM
    /// Moderate speed, larger
    Manomaya = 3,

    /// Vijñānamaya (wisdom) - Disk/SSD
    /// Slow, persistent
    Vijnanamaya = 4,

    /// Ānandamaya (bliss) - Network/Cloud
    /// Slowest, infinite capacity
    Anandamaya = 5,
}

impl Kosha {
    /// Get all Koshas in order (inner to outer)
    pub fn all() -> [Kosha; 5] {
        [
            Kosha::Annamaya,
            Kosha::Pranamaya,
            Kosha::Manomaya,
            Kosha::Vijnanamaya,
            Kosha::Anandamaya,
        ]
    }

    /// Get IAST transliteration
    pub fn iast(&self) -> &'static str {
        match self {
            Self::Annamaya => "Annamaya",
            Self::Pranamaya => "Prāṇamaya",
            Self::Manomaya => "Manomaya",
            Self::Vijnanamaya => "Vijñānamaya",
            Self::Anandamaya => "Ānandamaya",
        }
    }

    /// Get English meaning
    pub fn english(&self) -> &'static str {
        match self {
            Self::Annamaya => "Food Sheath",
            Self::Pranamaya => "Vital Sheath",
            Self::Manomaya => "Mental Sheath",
            Self::Vijnanamaya => "Wisdom Sheath",
            Self::Anandamaya => "Bliss Sheath",
        }
    }

    /// Get Sanskrit name
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::Annamaya => "अन्नमय",
            Self::Pranamaya => "प्राणमय",
            Self::Manomaya => "मनोमय",
            Self::Vijnanamaya => "विज्ञानमय",
            Self::Anandamaya => "आनन्दमय",
        }
    }

    /// Get physical mapping
    pub fn physical_mapping(&self) -> &'static str {
        match self {
            Self::Annamaya => "Registers/L1 Cache",
            Self::Pranamaya => "L2/L3 Cache",
            Self::Manomaya => "RAM",
            Self::Vijnanamaya => "SSD/Disk",
            Self::Anandamaya => "Network/Cloud",
        }
    }

    /// Approximate latency in nanoseconds
    pub fn latency_ns(&self) -> u64 {
        match self {
            Self::Annamaya => 1,           // ~1ns for registers
            Self::Pranamaya => 10,         // ~10ns for L2/L3
            Self::Manomaya => 100,         // ~100ns for RAM
            Self::Vijnanamaya => 100_000,  // ~100μs for SSD
            Self::Anandamaya => 1_000_000, // ~1ms for network
        }
    }

    /// Approximate capacity (symbolic)
    pub fn capacity_order(&self) -> u64 {
        match self {
            Self::Annamaya => 1,            // ~16 registers
            Self::Pranamaya => 100,         // ~MB
            Self::Manomaya => 10_000,       // ~GB
            Self::Vijnanamaya => 1_000_000, // ~TB
            Self::Anandamaya => u64::MAX,   // ~unlimited
        }
    }
}

// ============================================================================
// v10.0 Trait Implementations
// ============================================================================

impl SanskritNamed for Kosha {
    fn sanskrit(&self) -> &'static str {
        self.sanskrit_name()
    }

    fn iast(&self) -> &'static str {
        self.iast()
    }

    fn english(&self) -> &'static str {
        self.english()
    }
}

impl SanskritDescribed for Kosha {
    fn meaning(&self) -> &'static str {
        match self {
            Self::Annamaya => "The physical sheath made of food, the outermost layer",
            Self::Pranamaya => "The vital energy sheath, breath and life force",
            Self::Manomaya => "The mental sheath, thoughts and emotions",
            Self::Vijnanamaya => "The wisdom sheath, intellect and discernment",
            Self::Anandamaya => "The bliss sheath, innermost layer of pure joy",
        }
    }

    fn explanation(&self) -> &'static str {
        match self {
            Self::Annamaya => "Maps to CPU registers and L1 cache - fastest, most precious",
            Self::Pranamaya => "Maps to L2/L3 cache - vital flow of data between CPU and memory",
            Self::Manomaya => "Maps to RAM - where thoughts (data) are processed",
            Self::Vijnanamaya => "Maps to SSD/disk - persistent wisdom (storage)",
            Self::Anandamaya => "Maps to network/cloud - infinite bliss of distributed data",
        }
    }

    fn mantra(&self) -> Option<&'static str> {
        Some(match self {
            Self::Annamaya => "अन्नाद्भवन्ति भूतानि (From food beings are born)",
            Self::Pranamaya => "प्राणस्य प्राणमुत (Of life, the life)",
            Self::Manomaya => "मनोमयः प्राणशरीरनेता (Mind-made, leader of body and breath)",
            Self::Vijnanamaya => "विज्ञानं ब्रह्मेति व्यजानात् (Know wisdom as Brahman)",
            Self::Anandamaya => "आनन्दो ब्रह्मेति व्यजानात् (Know bliss as Brahman)",
        })
    }

    fn category(&self) -> &'static str {
        "Pancha Kosha (पञ्चकोश)"
    }
}

impl PhilosophicalEnum for Kosha {
    fn all() -> &'static [Self] {
        &[
            Kosha::Annamaya,
            Kosha::Pranamaya,
            Kosha::Manomaya,
            Kosha::Vijnanamaya,
            Kosha::Anandamaya,
        ]
    }

    fn count() -> usize {
        5
    }

    fn index(&self) -> usize {
        *self as usize - 1
    }

    fn ordinal(&self) -> usize {
        *self as usize
    }

    fn next(&self) -> Self {
        match self {
            Self::Annamaya => Self::Pranamaya,
            Self::Pranamaya => Self::Manomaya,
            Self::Manomaya => Self::Vijnanamaya,
            Self::Vijnanamaya => Self::Anandamaya,
            Self::Anandamaya => Self::Annamaya, // Cycle back
        }
    }

    fn prev(&self) -> Self {
        match self {
            Self::Annamaya => Self::Anandamaya, // Cycle back
            Self::Pranamaya => Self::Annamaya,
            Self::Manomaya => Self::Pranamaya,
            Self::Vijnanamaya => Self::Manomaya,
            Self::Anandamaya => Self::Vijnanamaya,
        }
    }

    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Annamaya),
            1 => Some(Self::Pranamaya),
            2 => Some(Self::Manomaya),
            3 => Some(Self::Vijnanamaya),
            4 => Some(Self::Anandamaya),
            _ => None,
        }
    }
}

/// Pancha Kosha allocator
pub struct PanchaKoshaAllocator {
    /// Allocation hints per symbol
    hints: HashMap<String, KoshaHint>,
    /// Access frequency tracking
    access_counts: HashMap<String, u64>,
    /// Current tier assignments
    assignments: HashMap<String, Kosha>,
}

/// Kosha allocation hint
#[derive(Debug, Clone)]
pub struct KoshaHint {
    /// Preferred kosha
    pub preferred: Kosha,
    /// Access frequency estimate
    pub access_frequency: AccessFrequency,
    /// Size in bytes
    pub size: usize,
}

/// Access frequency
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessFrequency {
    /// Constant - accessed every instruction
    Constant,
    /// High - accessed in hot loops
    High,
    /// Medium - regular access
    Medium,
    /// Low - occasional access
    Low,
    /// Rare - initialization/cleanup only
    Rare,
}

impl AccessFrequency {
    pub fn to_kosha(&self) -> Kosha {
        match self {
            Self::Constant => Kosha::Annamaya,
            Self::High => Kosha::Pranamaya,
            Self::Medium => Kosha::Manomaya,
            Self::Low => Kosha::Vijnanamaya,
            Self::Rare => Kosha::Anandamaya,
        }
    }
}

impl PanchaKoshaAllocator {
    pub fn new() -> Self {
        Self {
            hints: HashMap::new(),
            access_counts: HashMap::new(),
            assignments: HashMap::new(),
        }
    }

    /// Add allocation hint
    pub fn add_hint(&mut self, name: String, hint: KoshaHint) {
        self.hints.insert(name, hint);
    }

    /// Record access
    pub fn record_access(&mut self, name: &str) {
        *self.access_counts.entry(name.to_string()).or_insert(0) += 1;
    }

    /// Compute optimal tier for a symbol
    pub fn compute_tier(&self, name: &str) -> Kosha {
        // Use hint if available
        if let Some(hint) = self.hints.get(name) {
            return hint.preferred;
        }

        // Use access frequency
        let count = self.access_counts.get(name).copied().unwrap_or(0);
        match count {
            0..=10 => Kosha::Vijnanamaya,
            11..=100 => Kosha::Manomaya,
            101..=1000 => Kosha::Pranamaya,
            _ => Kosha::Annamaya,
        }
    }

    /// Assign symbol to kosha
    pub fn assign(&mut self, name: String, kosha: Kosha) {
        self.assignments.insert(name, kosha);
    }

    /// Get assignment
    pub fn get_assignment(&self, name: &str) -> Option<Kosha> {
        self.assignments.get(name).copied()
    }

    /// Rebalance assignments based on access patterns
    pub fn rebalance(&mut self) {
        let names: Vec<String> = self.assignments.keys().cloned().collect();
        for name in names {
            let optimal = self.compute_tier(&name);
            self.assignments.insert(name, optimal);
        }
    }
}

impl Default for PanchaKoshaAllocator {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{PhilosophicalEnum, SanskritDescribed, SanskritNamed};

    #[test]
    fn test_kosha_sanskrit_named_trait() {
        let kosha = Kosha::Annamaya;
        assert_eq!(kosha.sanskrit(), "अन्नमय");
        assert_eq!(kosha.iast(), "Annamaya");
        assert_eq!(kosha.english(), "Food Sheath");
    }

    #[test]
    fn test_kosha_sanskrit_described_trait() {
        let kosha = Kosha::Vijnanamaya;
        assert!(kosha.meaning().contains("wisdom"));
        assert!(kosha.explanation().contains("SSD"));
        assert!(kosha.mantra().is_some());
        assert_eq!(kosha.category(), "Pancha Kosha (पञ्चकोश)");
    }

    #[test]
    fn test_kosha_philosophical_enum_trait() {
        assert_eq!(Kosha::count(), 5);
        assert_eq!(Kosha::all().len(), 5);
        assert_eq!(Kosha::Annamaya.index(), 0);
        assert_eq!(Kosha::Anandamaya.ordinal(), 5);
    }

    #[test]
    fn test_kosha_navigation_cycle() {
        // Forward cycle
        assert_eq!(Kosha::Annamaya.next(), Kosha::Pranamaya);
        assert_eq!(Kosha::Anandamaya.next(), Kosha::Annamaya); // Wrap

        // Backward cycle
        assert_eq!(Kosha::Pranamaya.prev(), Kosha::Annamaya);
        assert_eq!(Kosha::Annamaya.prev(), Kosha::Anandamaya); // Wrap
    }

    #[test]
    fn test_kosha_from_index() {
        assert_eq!(Kosha::from_index(0), Some(Kosha::Annamaya));
        assert_eq!(Kosha::from_index(4), Some(Kosha::Anandamaya));
        assert_eq!(Kosha::from_index(5), None);
    }

    #[test]
    fn test_kosha_all_have_mantras() {
        for kosha in Kosha::all() {
            assert!(kosha.mantra().is_some(), "{:?} should have mantra", kosha);
        }
    }

    #[test]
    fn test_kosha_latency_ordering() {
        // Latency should increase as we go from inner to outer
        assert!(Kosha::Annamaya.latency_ns() < Kosha::Pranamaya.latency_ns());
        assert!(Kosha::Pranamaya.latency_ns() < Kosha::Manomaya.latency_ns());
        assert!(Kosha::Manomaya.latency_ns() < Kosha::Vijnanamaya.latency_ns());
        assert!(Kosha::Vijnanamaya.latency_ns() < Kosha::Anandamaya.latency_ns());
    }

    #[test]
    fn test_kosha_capacity_ordering() {
        // Capacity should increase as we go from inner to outer
        assert!(Kosha::Annamaya.capacity_order() < Kosha::Pranamaya.capacity_order());
        assert!(Kosha::Pranamaya.capacity_order() < Kosha::Manomaya.capacity_order());
        assert!(Kosha::Manomaya.capacity_order() < Kosha::Vijnanamaya.capacity_order());
        assert!(Kosha::Vijnanamaya.capacity_order() < Kosha::Anandamaya.capacity_order());
    }
}
