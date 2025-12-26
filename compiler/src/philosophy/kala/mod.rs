//! Kāla Time Budget Compilation
//!
//! Time-aware compilation based on the concept of Kāla (time).
//! Allows setting time budgets for compilation phases.

use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Kāla - Time budget manager
pub struct Kala {
    /// Total time budget
    total_budget: Duration,
    /// Time spent
    time_spent: Duration,
    /// Phase budgets
    phase_budgets: HashMap<String, Duration>,
    /// Phase timing
    phase_timing: HashMap<String, Duration>,
    /// Start time
    start_time: Option<Instant>,
}

/// Kāla scheduler
pub struct KalaScheduler {
    /// Time budgets per phase
    budgets: Vec<PhaseBudget>,
    /// Current phase index
    current_phase: usize,
    /// Is under time pressure
    time_pressure: bool,
}

/// Phase time budget
#[derive(Debug, Clone)]
pub struct PhaseBudget {
    pub name: String,
    pub budget: Duration,
    pub priority: Priority,
    pub can_skip: bool,
}

/// Phase priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Priority {
    /// Critical - cannot skip
    Critical,
    /// High - skip only if desperate
    High,
    /// Normal - can skip if needed
    Normal,
    /// Low - skip early
    Low,
}

impl Kala {
    pub fn new(total_budget: Duration) -> Self {
        Self {
            total_budget,
            time_spent: Duration::ZERO,
            phase_budgets: HashMap::new(),
            phase_timing: HashMap::new(),
            start_time: None,
        }
    }

    /// Start timing
    pub fn start(&mut self) {
        self.start_time = Some(Instant::now());
    }

    /// Set budget for a phase
    pub fn set_phase_budget(&mut self, phase: &str, budget: Duration) {
        self.phase_budgets.insert(phase.to_string(), budget);
    }

    /// Begin a phase
    pub fn begin_phase(&mut self, phase: &str) -> PhaseTimer {
        PhaseTimer {
            phase: phase.to_string(),
            start: Instant::now(),
            budget: self.phase_budgets.get(phase).copied(),
        }
    }

    /// End a phase
    pub fn end_phase(&mut self, timer: PhaseTimer) {
        let elapsed = timer.start.elapsed();
        self.phase_timing.insert(timer.phase, elapsed);
        self.time_spent += elapsed;
    }

    /// Get remaining time
    pub fn remaining(&self) -> Duration {
        self.total_budget.saturating_sub(self.time_spent)
    }

    /// Is over budget?
    pub fn is_over_budget(&self) -> bool {
        self.time_spent > self.total_budget
    }

    /// Get budget percentage used
    pub fn budget_used_percent(&self) -> f64 {
        (self.time_spent.as_secs_f64() / self.total_budget.as_secs_f64()) * 100.0
    }

    /// Get phase timing report
    pub fn timing_report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Kāla Timing Report ===\n");
        report.push_str(&format!("Total budget: {:?}\n", self.total_budget));
        report.push_str(&format!("Time spent: {:?}\n", self.time_spent));
        report.push_str(&format!("Remaining: {:?}\n", self.remaining()));
        report.push_str("\nPhase breakdown:\n");

        for (phase, time) in &self.phase_timing {
            let budget = self.phase_budgets.get(phase);
            let status = match budget {
                Some(b) if time > b => "⚠️ OVER",
                Some(_) => "✓",
                None => "?",
            };
            report.push_str(&format!("  {}: {:?} {}\n", phase, time, status));
        }

        report
    }
}

/// Timer for a compilation phase
pub struct PhaseTimer {
    phase: String,
    start: Instant,
    budget: Option<Duration>,
}

impl PhaseTimer {
    /// Is over budget?
    pub fn is_over_budget(&self) -> bool {
        match self.budget {
            Some(budget) => self.start.elapsed() > budget,
            None => false,
        }
    }

    /// Get elapsed time
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

impl KalaScheduler {
    pub fn new() -> Self {
        Self {
            budgets: Vec::new(),
            current_phase: 0,
            time_pressure: false,
        }
    }

    /// Add a phase budget
    pub fn add_phase(&mut self, budget: PhaseBudget) {
        self.budgets.push(budget);
    }

    /// Get next phase to run
    pub fn next_phase(&mut self) -> Option<&PhaseBudget> {
        while self.current_phase < self.budgets.len() {
            let phase = &self.budgets[self.current_phase];

            // Skip low-priority phases if under time pressure
            if self.time_pressure && phase.can_skip {
                self.current_phase += 1;
                continue;
            }

            self.current_phase += 1;
            return Some(phase);
        }
        None
    }

    /// Signal time pressure
    pub fn set_time_pressure(&mut self, pressure: bool) {
        self.time_pressure = pressure;
    }

    /// Create default compilation scheduler
    pub fn default_compiler_scheduler(total_budget: Duration) -> Self {
        let mut scheduler = Self::new();

        // Distribute budget across phases
        let phase_count = 10;
        let phase_budget = total_budget / phase_count;

        scheduler.add_phase(PhaseBudget {
            name: "lexing".to_string(),
            budget: phase_budget,
            priority: Priority::Critical,
            can_skip: false,
        });

        scheduler.add_phase(PhaseBudget {
            name: "parsing".to_string(),
            budget: phase_budget,
            priority: Priority::Critical,
            can_skip: false,
        });

        scheduler.add_phase(PhaseBudget {
            name: "type_checking".to_string(),
            budget: phase_budget * 2,
            priority: Priority::Critical,
            can_skip: false,
        });

        scheduler.add_phase(PhaseBudget {
            name: "optimization".to_string(),
            budget: phase_budget * 3,
            priority: Priority::Normal,
            can_skip: true,
        });

        scheduler.add_phase(PhaseBudget {
            name: "codegen".to_string(),
            budget: phase_budget * 2,
            priority: Priority::Critical,
            can_skip: false,
        });

        scheduler.add_phase(PhaseBudget {
            name: "linking".to_string(),
            budget: phase_budget,
            priority: Priority::High,
            can_skip: false,
        });

        scheduler
    }
}

impl Default for Kala {
    fn default() -> Self {
        Self::new(Duration::from_secs(60)) // 1 minute default
    }
}

impl Default for KalaScheduler {
    fn default() -> Self {
        Self::new()
    }
}
