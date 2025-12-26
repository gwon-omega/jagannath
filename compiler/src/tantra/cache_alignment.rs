//! Cache Alignment - Kuṇḍalinī Flow (कुण्डलिनी)
//!
//! Data alignment for optimal cache utilization.
//! Kuṇḍalinī (coiled energy) rises through the chakras -
//! data flows efficiently through cache hierarchy.
//!
//! Features:
//! - Cache line alignment
//! - False sharing prevention
//! - Prefetch scheduling
//! - Hot/cold data separation

use std::collections::HashMap;
use std::alloc::Layout;

/// Cache hierarchy configuration
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// L1 data cache size (bytes)
    pub l1d_size: usize,
    /// L1 instruction cache size (bytes)
    pub l1i_size: usize,
    /// L2 cache size (bytes)
    pub l2_size: usize,
    /// L3 cache size (bytes)
    pub l3_size: usize,
    /// Cache line size (bytes)
    pub cache_line: usize,
    /// L1 associativity
    pub l1_assoc: usize,
    /// L2 associativity
    pub l2_assoc: usize,
}

impl Default for CacheConfig {
    fn default() -> Self {
        // Common Intel/AMD desktop configuration
        Self {
            l1d_size: 32 * 1024,       // 32KB
            l1i_size: 32 * 1024,       // 32KB
            l2_size: 256 * 1024,       // 256KB
            l3_size: 8 * 1024 * 1024,  // 8MB
            cache_line: 64,
            l1_assoc: 8,
            l2_assoc: 8,
        }
    }
}

/// Kuṇḍalinī data flow optimizer
pub struct KundaliniFlow {
    /// Cache configuration
    config: CacheConfig,
    /// Data structures being tracked
    tracked: HashMap<String, DataStructure>,
    /// Alignment recommendations
    alignments: Vec<AlignmentRecommendation>,
}

/// Tracked data structure
#[derive(Debug, Clone)]
pub struct DataStructure {
    pub name: String,
    pub size: usize,
    pub access_pattern: AccessPattern,
    pub temperature: DataTemperature,
    pub current_alignment: usize,
}

/// Data access patterns
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessPattern {
    /// Sequential access (stride 1)
    Sequential,
    /// Strided access (fixed stride)
    Strided(usize),
    /// Random access
    Random,
    /// Temporal locality (accessed repeatedly)
    Temporal,
    /// Write-mostly
    WriteMostly,
    /// Read-mostly
    ReadMostly,
}

/// Data temperature (frequency of access)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DataTemperature {
    /// Critical - innermost loops
    Critical,
    /// Hot - frequently accessed
    Hot,
    /// Warm - moderately accessed
    Warm,
    /// Cold - rarely accessed
    Cold,
    /// Frozen - almost never accessed
    Frozen,
}

/// Alignment recommendation
#[derive(Debug, Clone)]
pub struct AlignmentRecommendation {
    pub data_name: String,
    pub recommended_alignment: usize,
    pub reason: String,
    pub estimated_improvement: f64,
}

/// Prefetch strategy
#[derive(Debug, Clone)]
pub struct PrefetchStrategy {
    pub distance: usize,
    pub hint: PrefetchHint,
    pub locations: Vec<String>,
}

/// Prefetch hint level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrefetchHint {
    /// T0 - All cache levels
    T0,
    /// T1 - L2 and above
    T1,
    /// T2 - L3 and above
    T2,
    /// NTA - Non-temporal (bypass cache)
    NonTemporal,
}

impl KundaliniFlow {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            config,
            tracked: HashMap::new(),
            alignments: Vec::new(),
        }
    }

    /// Track a data structure
    pub fn track(&mut self, data: DataStructure) {
        self.tracked.insert(data.name.clone(), data);
    }

    /// Analyze and recommend alignments
    pub fn analyze(&mut self) {
        self.alignments.clear();

        for data in self.tracked.values() {
            let rec = self.recommend_alignment(data);
            if let Some(r) = rec {
                self.alignments.push(r);
            }
        }
    }

    /// Get alignment recommendation for a data structure
    fn recommend_alignment(&self, data: &DataStructure) -> Option<AlignmentRecommendation> {
        // Already well-aligned?
        if data.current_alignment >= self.config.cache_line {
            return None;
        }

        // Critical/hot data should be cache-line aligned
        let recommended = match data.temperature {
            DataTemperature::Critical | DataTemperature::Hot => {
                self.config.cache_line
            }
            DataTemperature::Warm => {
                // At least 16-byte alignment for SIMD
                16
            }
            _ => {
                // Cold data doesn't need special alignment
                return None;
            }
        };

        // Check if improvement is worthwhile
        if recommended <= data.current_alignment {
            return None;
        }

        let improvement = match data.access_pattern {
            AccessPattern::Sequential => 0.15, // 15% improvement
            AccessPattern::Strided(_) => 0.10,
            AccessPattern::Temporal => 0.20,
            _ => 0.05,
        };

        Some(AlignmentRecommendation {
            data_name: data.name.clone(),
            recommended_alignment: recommended,
            reason: format!(
                "{:?} data with {:?} access pattern",
                data.temperature, data.access_pattern
            ),
            estimated_improvement: improvement,
        })
    }

    /// Generate prefetch strategy for sequential access
    pub fn prefetch_strategy(&self, data: &DataStructure) -> Option<PrefetchStrategy> {
        match data.access_pattern {
            AccessPattern::Sequential | AccessPattern::Strided(_) => {
                // Prefetch distance based on temperature
                let distance = match data.temperature {
                    DataTemperature::Critical => 8,  // 8 cache lines ahead
                    DataTemperature::Hot => 4,
                    DataTemperature::Warm => 2,
                    _ => return None,
                };

                let hint = match data.temperature {
                    DataTemperature::Critical => PrefetchHint::T0,
                    DataTemperature::Hot => PrefetchHint::T1,
                    _ => PrefetchHint::T2,
                };

                Some(PrefetchStrategy {
                    distance: distance * self.config.cache_line,
                    hint,
                    locations: vec![data.name.clone()],
                })
            }
            _ => None,
        }
    }

    /// Check for potential false sharing
    pub fn check_false_sharing(&self) -> Vec<FalseSharingWarning> {
        let mut warnings = Vec::new();

        // Group data by potential cache line conflicts
        let mut line_groups: HashMap<usize, Vec<&DataStructure>> = HashMap::new();

        for data in self.tracked.values() {
            if data.size < self.config.cache_line {
                // Small structures might share cache lines
                let key = data.size; // Simplified - would use actual address
                line_groups.entry(key).or_default().push(data);
            }
        }

        // Check for write conflicts
        for (_, group) in line_groups {
            if group.len() > 1 {
                let writers: Vec<_> = group.iter()
                    .filter(|d| matches!(d.access_pattern, AccessPattern::WriteMostly))
                    .collect();

                if writers.len() > 1 {
                    warnings.push(FalseSharingWarning {
                        structures: writers.iter().map(|d| d.name.clone()).collect(),
                        recommendation: "Pad structures to cache line boundaries".to_string(),
                    });
                }
            }
        }

        warnings
    }

    /// Generate aligned layout for a type
    pub fn aligned_layout(&self, size: usize, temp: DataTemperature) -> Layout {
        let align = match temp {
            DataTemperature::Critical | DataTemperature::Hot => self.config.cache_line,
            DataTemperature::Warm => 16,
            _ => 8,
        };

        Layout::from_size_align(size, align)
            .expect("Invalid layout")
    }

    /// Get recommendations
    pub fn recommendations(&self) -> &[AlignmentRecommendation] {
        &self.alignments
    }

    /// Generate report
    pub fn report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Kuṇḍalinī Cache Flow Report ===\n\n");
        report.push_str(&format!("Cache Config: L1={}KB, L2={}KB, L3={}MB, Line={}B\n\n",
            self.config.l1d_size / 1024,
            self.config.l2_size / 1024,
            self.config.l3_size / (1024 * 1024),
            self.config.cache_line
        ));

        report.push_str("Tracked Data:\n");
        for data in self.tracked.values() {
            report.push_str(&format!("  {} - {}B, {:?}, {:?}\n",
                data.name, data.size, data.temperature, data.access_pattern
            ));
        }

        if !self.alignments.is_empty() {
            report.push_str("\nAlignment Recommendations:\n");
            for rec in &self.alignments {
                report.push_str(&format!("  {} → align({}) - {:.0}% improvement\n",
                    rec.data_name, rec.recommended_alignment, rec.estimated_improvement * 100.0
                ));
            }
        }

        report
    }
}

/// False sharing warning
#[derive(Debug, Clone)]
pub struct FalseSharingWarning {
    pub structures: Vec<String>,
    pub recommendation: String,
}

impl Default for KundaliniFlow {
    fn default() -> Self {
        Self::new(CacheConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alignment_recommendation() {
        let mut flow = KundaliniFlow::default();

        flow.track(DataStructure {
            name: "hot_array".to_string(),
            size: 1024,
            access_pattern: AccessPattern::Sequential,
            temperature: DataTemperature::Hot,
            current_alignment: 8,
        });

        flow.analyze();

        assert!(!flow.recommendations().is_empty());
        assert_eq!(flow.recommendations()[0].recommended_alignment, 64);
    }

    #[test]
    fn test_prefetch_strategy() {
        let flow = KundaliniFlow::default();

        let data = DataStructure {
            name: "critical_buffer".to_string(),
            size: 4096,
            access_pattern: AccessPattern::Sequential,
            temperature: DataTemperature::Critical,
            current_alignment: 64,
        };

        let strategy = flow.prefetch_strategy(&data);
        assert!(strategy.is_some());
        assert_eq!(strategy.unwrap().hint, PrefetchHint::T0);
    }
}
