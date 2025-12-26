//! Prāṇāyāma - Breath Control/Resource Management (प्राणायाम)
//!
//! The fourth limb of Ashtanga Yoga - Control of vital energy.
//! In software, prāṇa (vital energy) maps to system resources:
//!
//! - Memory (RAM) - The body's oxygen
//! - CPU cycles - The body's metabolism
//! - I/O bandwidth - The body's circulation
//! - Network - The body's nervous system

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Resource types (types of prāṇa)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Prana {
    /// Memory resources
    Memory,
    /// CPU cycles
    Cpu,
    /// Disk I/O
    DiskIO,
    /// Network I/O
    NetworkIO,
    /// File handles
    FileHandles,
    /// Thread count
    Threads,
}

impl Prana {
    /// Sanskrit term for this resource type
    pub fn sanskrit(&self) -> &'static str {
        match self {
            Self::Memory => "स्मृति (Smṛti)",
            Self::Cpu => "चिन्तन (Cintana)",
            Self::DiskIO => "लेखन (Lekhana)",
            Self::NetworkIO => "संचार (Saṃcāra)",
            Self::FileHandles => "द्वार (Dvāra)",
            Self::Threads => "तन्तु (Tantu)",
        }
    }

    /// Yogic breath phase this maps to
    pub fn breath_phase(&self) -> &'static str {
        match self {
            Self::Memory => "Pūraka (inhalation) - acquiring resources",
            Self::Cpu => "Kumbhaka (retention) - processing",
            Self::DiskIO => "Recaka (exhalation) - releasing to storage",
            Self::NetworkIO => "Śūnyaka (pause) - external exchange",
            Self::FileHandles => "Pūraka (inhalation) - opening channels",
            Self::Threads => "Kumbhaka (retention) - parallel processing",
        }
    }
}

/// Resource budget (prāṇa allowance)
#[derive(Debug, Clone)]
pub struct PranaBudget {
    /// Resource type
    pub prana: Prana,
    /// Maximum allowed
    pub limit: u64,
    /// Current usage
    pub used: u64,
    /// Peak usage
    pub peak: u64,
    /// Unit of measurement
    pub unit: &'static str,
}

impl PranaBudget {
    pub fn new(prana: Prana, limit: u64, unit: &'static str) -> Self {
        Self {
            prana,
            limit,
            used: 0,
            peak: 0,
            unit,
        }
    }

    /// Allocate resource (pūraka - inhale)
    pub fn allocate(&mut self, amount: u64) -> Result<(), PranaError> {
        if self.used + amount > self.limit {
            return Err(PranaError::ExceededBudget {
                prana: self.prana,
                requested: amount,
                available: self.limit - self.used,
            });
        }
        self.used += amount;
        self.peak = self.peak.max(self.used);
        Ok(())
    }

    /// Release resource (recaka - exhale)
    pub fn release(&mut self, amount: u64) {
        self.used = self.used.saturating_sub(amount);
    }

    /// Get utilization percentage
    pub fn utilization(&self) -> f64 {
        if self.limit == 0 {
            return 0.0;
        }
        (self.used as f64 / self.limit as f64) * 100.0
    }

    /// Check if over budget
    pub fn is_over_budget(&self) -> bool {
        self.used > self.limit
    }
}

/// Resource management error
#[derive(Debug, Clone)]
pub enum PranaError {
    ExceededBudget {
        prana: Prana,
        requested: u64,
        available: u64,
    },
    ResourceLeak {
        prana: Prana,
        leaked: u64,
    },
    Timeout {
        operation: String,
        elapsed: Duration,
    },
}

/// Prāṇāyāma resource manager
pub struct PranayamaManager {
    /// Resource budgets
    budgets: HashMap<Prana, PranaBudget>,
    /// Active allocations (for leak detection)
    allocations: HashMap<String, (Prana, u64, Instant)>,
    /// Leak threshold (duration after which allocation is suspicious)
    leak_threshold: Duration,
}

impl PranayamaManager {
    pub fn new() -> Self {
        let mut manager = Self {
            budgets: HashMap::new(),
            allocations: HashMap::new(),
            leak_threshold: Duration::from_secs(60),
        };

        // Set default budgets
        manager.set_budget(PranaBudget::new(Prana::Memory, 1024 * 1024 * 1024, "bytes")); // 1GB
        manager.set_budget(PranaBudget::new(Prana::FileHandles, 1024, "handles"));
        manager.set_budget(PranaBudget::new(Prana::Threads, 64, "threads"));

        manager
    }

    /// Set budget for a resource type
    pub fn set_budget(&mut self, budget: PranaBudget) {
        self.budgets.insert(budget.prana, budget);
    }

    /// Pūraka (inhalation) - Acquire resources
    pub fn puraka(&mut self, id: &str, prana: Prana, amount: u64) -> Result<(), PranaError> {
        let budget = self.budgets.get_mut(&prana).ok_or_else(|| {
            PranaError::ExceededBudget {
                prana,
                requested: amount,
                available: 0,
            }
        })?;

        budget.allocate(amount)?;
        self.allocations.insert(id.to_string(), (prana, amount, Instant::now()));
        Ok(())
    }

    /// Recaka (exhalation) - Release resources
    pub fn recaka(&mut self, id: &str) -> Option<(Prana, u64)> {
        if let Some((prana, amount, _)) = self.allocations.remove(id) {
            if let Some(budget) = self.budgets.get_mut(&prana) {
                budget.release(amount);
            }
            Some((prana, amount))
        } else {
            None
        }
    }

    /// Kumbhaka (retention) - Check resource status during hold
    pub fn kumbhaka(&self, prana: Prana) -> Option<&PranaBudget> {
        self.budgets.get(&prana)
    }

    /// Śūnyaka (pause) - Check for leaks
    pub fn sunyaka(&self) -> Vec<PranaError> {
        let now = Instant::now();
        let mut leaks = Vec::new();

        for (id, (prana, amount, allocated_at)) in &self.allocations {
            let elapsed = now.duration_since(*allocated_at);
            if elapsed > self.leak_threshold {
                leaks.push(PranaError::ResourceLeak {
                    prana: *prana,
                    leaked: *amount,
                });
            }
        }

        leaks
    }

    /// Get overall resource health
    pub fn health(&self) -> PranaHealth {
        let mut health = PranaHealth::default();

        for budget in self.budgets.values() {
            let util = budget.utilization();
            health.utilizations.insert(budget.prana, util);

            if util > 90.0 {
                health.critical.push(budget.prana);
            } else if util > 70.0 {
                health.warning.push(budget.prana);
            }
        }

        health.leaks = self.sunyaka().len();
        health
    }

    /// Optimize breathing pattern (resource rebalancing)
    pub fn optimize_breathing(&mut self) -> Vec<String> {
        let mut suggestions = Vec::new();

        for budget in self.budgets.values() {
            let util = budget.utilization();

            if util > 90.0 {
                suggestions.push(format!(
                    "CRITICAL: {} at {:.1}% - immediate release needed",
                    budget.prana.sanskrit(),
                    util
                ));
            } else if util > 70.0 {
                suggestions.push(format!(
                    "WARNING: {} at {:.1}% - consider releasing unused resources",
                    budget.prana.sanskrit(),
                    util
                ));
            } else if util < 10.0 && budget.peak > 0 {
                suggestions.push(format!(
                    "INFO: {} at {:.1}% but peaked at {} - budget may be too large",
                    budget.prana.sanskrit(),
                    util,
                    budget.peak
                ));
            }
        }

        suggestions
    }

    /// Get statistics
    pub fn stats(&self) -> PranaStats {
        PranaStats {
            active_allocations: self.allocations.len(),
            budgets: self.budgets.values().cloned().collect(),
        }
    }
}

/// Resource health status
#[derive(Debug, Default)]
pub struct PranaHealth {
    pub utilizations: HashMap<Prana, f64>,
    pub critical: Vec<Prana>,
    pub warning: Vec<Prana>,
    pub leaks: usize,
}

impl PranaHealth {
    pub fn is_healthy(&self) -> bool {
        self.critical.is_empty() && self.leaks == 0
    }
}

/// Resource statistics
#[derive(Debug)]
pub struct PranaStats {
    pub active_allocations: usize,
    pub budgets: Vec<PranaBudget>,
}

impl Default for PranayamaManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation_and_release() {
        let mut manager = PranayamaManager::new();
        manager.set_budget(PranaBudget::new(Prana::Memory, 1000, "bytes"));

        // Allocate
        assert!(manager.puraka("alloc1", Prana::Memory, 500).is_ok());
        assert_eq!(manager.kumbhaka(Prana::Memory).unwrap().used, 500);

        // Release
        manager.recaka("alloc1");
        assert_eq!(manager.kumbhaka(Prana::Memory).unwrap().used, 0);
    }

    #[test]
    fn test_budget_exceeded() {
        let mut manager = PranayamaManager::new();
        manager.set_budget(PranaBudget::new(Prana::Memory, 100, "bytes"));

        assert!(manager.puraka("alloc1", Prana::Memory, 150).is_err());
    }

    #[test]
    fn test_utilization() {
        let mut budget = PranaBudget::new(Prana::Cpu, 100, "cycles");
        budget.allocate(75).unwrap();
        assert!((budget.utilization() - 75.0).abs() < 0.1);
    }
}
