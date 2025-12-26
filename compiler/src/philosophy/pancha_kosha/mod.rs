//! Pancha Kosha Memory Hierarchy
//!
//! 5-tier memory hierarchy based on the 5 sheaths (koshas):
//! 1. Annamaya (food) - Registers/L1 cache (hottest)
//! 2. Prāṇamaya (breath) - L2/L3 cache
//! 3. Manomaya (mind) - RAM
//! 4. Vijñānamaya (wisdom) - Disk/SSD
//! 5. Ānandamaya (bliss) - Network/Cloud

use std::collections::HashMap;

/// The 5 Koshas (memory tiers)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
            Self::Annamaya => 1,        // ~16 registers
            Self::Pranamaya => 100,     // ~MB
            Self::Manomaya => 10_000,   // ~GB
            Self::Vijnanamaya => 1_000_000, // ~TB
            Self::Anandamaya => u64::MAX, // ~unlimited
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
