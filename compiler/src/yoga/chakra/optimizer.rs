//! Chakra Optimizer - 7-Stage Optimization Pipeline (चक्र अनुकूलक)
//!
//! Each chakra represents an optimization stage, from foundation to transcendence:
//!
//! 1. Mūlādhāra - Platform-specific optimizations
//! 2. Svādhiṣṭhāna - Memory optimizations
//! 3. Maṇipūra - CPU/compute optimizations
//! 4. Anāhata - Core algorithm optimizations
//! 5. Viśuddha - API/interface optimizations
//! 6. Ājñā - Analysis/profiling optimizations
//! 7. Sahasrāra - Whole-program optimizations

use std::collections::HashMap;
use std::time::{Duration, Instant};

use super::Chakra;

/// Chakra-based optimization pipeline
pub struct ChakraOptimizer {
    /// Optimization passes per chakra
    passes: HashMap<Chakra, Vec<OptimizationPass>>,
    /// Stats per chakra
    stats: HashMap<Chakra, ChakraStats>,
    /// Configuration
    config: OptimizerConfig,
}

/// Single optimization pass
#[derive(Debug, Clone)]
pub struct OptimizationPass {
    pub name: String,
    pub chakra: Chakra,
    pub description: String,
    pub enabled: bool,
    pub cost: PassCost,
}

/// Pass computational cost
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PassCost {
    /// O(n)
    Linear,
    /// O(n log n)
    LogLinear,
    /// O(n²)
    Quadratic,
    /// O(n³)
    Cubic,
    /// Very expensive
    Expensive,
}

/// Statistics for a chakra
#[derive(Debug, Default, Clone)]
pub struct ChakraStats {
    pub passes_run: usize,
    pub total_time: Duration,
    pub improvements: Vec<Improvement>,
}

/// Recorded improvement
#[derive(Debug, Clone)]
pub struct Improvement {
    pub pass_name: String,
    pub metric: String,
    pub before: f64,
    pub after: f64,
}

/// Optimizer configuration
#[derive(Debug, Clone)]
pub struct OptimizerConfig {
    /// Optimization level (0-3)
    pub opt_level: u8,
    /// Time budget per chakra
    pub time_budget: Duration,
    /// Enable expensive passes?
    pub enable_expensive: bool,
    /// Debug output?
    pub debug: bool,
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        Self {
            opt_level: 2,
            time_budget: Duration::from_secs(10),
            enable_expensive: false,
            debug: false,
        }
    }
}

impl ChakraOptimizer {
    pub fn new(config: OptimizerConfig) -> Self {
        let mut optimizer = Self {
            passes: HashMap::new(),
            stats: HashMap::new(),
            config,
        };

        optimizer.register_default_passes();
        optimizer
    }

    fn register_default_passes(&mut self) {
        // Mūlādhāra - Platform/hardware
        self.register_pass(OptimizationPass {
            name: "instruction_selection".to_string(),
            chakra: Chakra::Muladhara,
            description: "Select optimal instructions for target".to_string(),
            enabled: true,
            cost: PassCost::Linear,
        });
        self.register_pass(OptimizationPass {
            name: "simd_lowering".to_string(),
            chakra: Chakra::Muladhara,
            description: "Lower to SIMD instructions".to_string(),
            enabled: true,
            cost: PassCost::Linear,
        });

        // Svādhiṣṭhāna - Memory
        self.register_pass(OptimizationPass {
            name: "stack_slot_coloring".to_string(),
            chakra: Chakra::Svadhisthana,
            description: "Reuse stack slots".to_string(),
            enabled: true,
            cost: PassCost::LogLinear,
        });
        self.register_pass(OptimizationPass {
            name: "escape_analysis".to_string(),
            chakra: Chakra::Svadhisthana,
            description: "Move heap allocations to stack".to_string(),
            enabled: true,
            cost: PassCost::Quadratic,
        });
        self.register_pass(OptimizationPass {
            name: "alias_analysis".to_string(),
            chakra: Chakra::Svadhisthana,
            description: "Track pointer aliasing".to_string(),
            enabled: true,
            cost: PassCost::Quadratic,
        });

        // Maṇipūra - CPU/compute
        self.register_pass(OptimizationPass {
            name: "register_allocation".to_string(),
            chakra: Chakra::Manipura,
            description: "Allocate variables to registers".to_string(),
            enabled: true,
            cost: PassCost::LogLinear,
        });
        self.register_pass(OptimizationPass {
            name: "instruction_scheduling".to_string(),
            chakra: Chakra::Manipura,
            description: "Reorder for pipeline".to_string(),
            enabled: true,
            cost: PassCost::LogLinear,
        });
        self.register_pass(OptimizationPass {
            name: "strength_reduction".to_string(),
            chakra: Chakra::Manipura,
            description: "Replace expensive ops with cheaper ones".to_string(),
            enabled: true,
            cost: PassCost::Linear,
        });

        // Anāhata - Core algorithms
        self.register_pass(OptimizationPass {
            name: "loop_invariant_motion".to_string(),
            chakra: Chakra::Anahata,
            description: "Move invariants out of loops".to_string(),
            enabled: true,
            cost: PassCost::Linear,
        });
        self.register_pass(OptimizationPass {
            name: "loop_unrolling".to_string(),
            chakra: Chakra::Anahata,
            description: "Unroll loops for performance".to_string(),
            enabled: true,
            cost: PassCost::Linear,
        });
        self.register_pass(OptimizationPass {
            name: "dead_code_elimination".to_string(),
            chakra: Chakra::Anahata,
            description: "Remove unreachable code".to_string(),
            enabled: true,
            cost: PassCost::Linear,
        });
        self.register_pass(OptimizationPass {
            name: "common_subexpr_elim".to_string(),
            chakra: Chakra::Anahata,
            description: "Eliminate redundant computations".to_string(),
            enabled: true,
            cost: PassCost::Quadratic,
        });

        // Viśuddha - API/interface
        self.register_pass(OptimizationPass {
            name: "function_inlining".to_string(),
            chakra: Chakra::Vishuddha,
            description: "Inline small functions".to_string(),
            enabled: true,
            cost: PassCost::LogLinear,
        });
        self.register_pass(OptimizationPass {
            name: "devirtualization".to_string(),
            chakra: Chakra::Vishuddha,
            description: "Convert virtual calls to direct".to_string(),
            enabled: true,
            cost: PassCost::Quadratic,
        });

        // Ājñā - Analysis
        self.register_pass(OptimizationPass {
            name: "profile_guided".to_string(),
            chakra: Chakra::Ajna,
            description: "Use profiling data for optimization".to_string(),
            enabled: false, // Requires profiling
            cost: PassCost::Expensive,
        });
        self.register_pass(OptimizationPass {
            name: "hot_cold_splitting".to_string(),
            chakra: Chakra::Ajna,
            description: "Separate hot and cold code paths".to_string(),
            enabled: true,
            cost: PassCost::Linear,
        });

        // Sahasrāra - Whole-program
        self.register_pass(OptimizationPass {
            name: "link_time_optimization".to_string(),
            chakra: Chakra::Sahasrara,
            description: "Cross-module optimization".to_string(),
            enabled: false, // Expensive
            cost: PassCost::Expensive,
        });
        self.register_pass(OptimizationPass {
            name: "interprocedural_analysis".to_string(),
            chakra: Chakra::Sahasrara,
            description: "Analyze across function boundaries".to_string(),
            enabled: false,
            cost: PassCost::Cubic,
        });
    }

    /// Register optimization pass
    pub fn register_pass(&mut self, pass: OptimizationPass) {
        self.passes
            .entry(pass.chakra)
            .or_default()
            .push(pass);
    }

    /// Enable/disable pass
    pub fn set_pass_enabled(&mut self, name: &str, enabled: bool) {
        for passes in self.passes.values_mut() {
            for pass in passes {
                if pass.name == name {
                    pass.enabled = enabled;
                }
            }
        }
    }

    /// Get passes for opt level
    pub fn passes_for_level(&self, level: u8) -> Vec<&OptimizationPass> {
        let mut result = Vec::new();

        for chakra_passes in self.passes.values() {
            for pass in chakra_passes {
                let include = match level {
                    0 => false, // O0 = no optimization
                    1 => pass.cost == PassCost::Linear,
                    2 => pass.cost != PassCost::Expensive && pass.cost != PassCost::Cubic,
                    3 => true, // O3 = all passes
                    _ => pass.cost != PassCost::Expensive,
                };

                if include && pass.enabled {
                    result.push(pass);
                }
            }
        }

        result
    }

    /// Run optimization pipeline
    pub fn run_pipeline<T>(&mut self, mut ir: T) -> T
    where
        T: Optimizable,
    {
        // Run chakras from root (1) to crown (7)
        let chakras = [
            Chakra::Muladhara,
            Chakra::Svadhisthana,
            Chakra::Manipura,
            Chakra::Anahata,
            Chakra::Vishuddha,
            Chakra::Ajna,
            Chakra::Sahasrara,
        ];

        for chakra in chakras {
            ir = self.run_chakra(chakra, ir);
        }

        ir
    }

    /// Run passes for a single chakra
    pub fn run_chakra<T>(&mut self, chakra: Chakra, mut ir: T) -> T
    where
        T: Optimizable,
    {
        let start = Instant::now();
        let passes = self.passes.get(&chakra).cloned().unwrap_or_default();

        let enabled_passes: Vec<_> = passes.iter()
            .filter(|p| self.should_run_pass(p))
            .collect();

        for pass in enabled_passes {
            if self.config.debug {
                eprintln!("[{}] Running: {}", chakra.sanskrit_name(), pass.name);
            }

            // Run the pass
            ir = ir.apply_pass(&pass.name);

            self.stats
                .entry(chakra)
                .or_default()
                .passes_run += 1;
        }

        let elapsed = start.elapsed();
        self.stats
            .entry(chakra)
            .or_default()
            .total_time += elapsed;

        ir
    }

    /// Should this pass run?
    fn should_run_pass(&self, pass: &OptimizationPass) -> bool {
        if !pass.enabled {
            return false;
        }

        if pass.cost == PassCost::Expensive && !self.config.enable_expensive {
            return false;
        }

        // Check opt level
        match self.config.opt_level {
            0 => false,
            1 => pass.cost == PassCost::Linear,
            2 => pass.cost != PassCost::Expensive && pass.cost != PassCost::Cubic,
            _ => true,
        }
    }

    /// Get optimization report
    pub fn report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Chakra Optimizer Report ===\n\n");
        report.push_str(&format!("Optimization Level: O{}\n\n", self.config.opt_level));

        let chakras = [
            Chakra::Muladhara,
            Chakra::Svadhisthana,
            Chakra::Manipura,
            Chakra::Anahata,
            Chakra::Vishuddha,
            Chakra::Ajna,
            Chakra::Sahasrara,
        ];

        for chakra in chakras {
            let stats = self.stats.get(&chakra);
            let passes_count = self.passes.get(&chakra).map(|p| p.len()).unwrap_or(0);

            report.push_str(&format!(
                "{} ({}):\n",
                chakra.sanskrit_name(),
                chakra.software_layer()
            ));
            report.push_str(&format!("  Passes: {}\n", passes_count));

            if let Some(s) = stats {
                report.push_str(&format!("  Run: {}, Time: {:?}\n", s.passes_run, s.total_time));
            }
        }

        report
    }
}

/// Trait for types that can be optimized
pub trait Optimizable {
    fn apply_pass(self, pass_name: &str) -> Self;
}

// Default implementation for testing
impl Optimizable for () {
    fn apply_pass(self, _pass_name: &str) -> Self {
        self
    }
}

impl Default for ChakraOptimizer {
    fn default() -> Self {
        Self::new(OptimizerConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pass_registration() {
        let optimizer = ChakraOptimizer::default();

        // Should have passes for each chakra
        assert!(!optimizer.passes.is_empty());
        assert!(optimizer.passes.contains_key(&Chakra::Muladhara));
    }

    #[test]
    fn test_opt_level_filtering() {
        let optimizer = ChakraOptimizer::new(OptimizerConfig {
            opt_level: 1,
            ..Default::default()
        });

        let passes = optimizer.passes_for_level(1);

        // Only linear passes at O1
        for pass in passes {
            assert_eq!(pass.cost, PassCost::Linear);
        }
    }

    #[test]
    fn test_pipeline() {
        let mut optimizer = ChakraOptimizer::default();

        // Run empty pipeline
        let result = optimizer.run_pipeline(());
        assert_eq!(result, ());
    }
}
