//! Āyurveda - System Health Monitoring
//!
//! System health model based on the 3 doshas:
//! - Vāta: CPU/processing health
//! - Pitta: Memory/thermal health
//! - Kapha: Storage/persistence health

use std::time::Instant;

/// The 3 Doshas (system health indicators)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dosha {
    /// Vāta (air/movement) - CPU, processing, responsiveness
    Vata,

    /// Pitta (fire/transformation) - Memory, thermal, computation
    Pitta,

    /// Kapha (earth/structure) - Storage, persistence, stability
    Kapha,
}

impl Dosha {
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::Vata => "वात",
            Self::Pitta => "पित्त",
            Self::Kapha => "कफ",
        }
    }

    pub fn system_aspect(&self) -> &'static str {
        match self {
            Self::Vata => "CPU/Processing",
            Self::Pitta => "Memory/Thermal",
            Self::Kapha => "Storage/Stability",
        }
    }
}

/// Dosha balance state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DoshaState {
    /// Balanced (healthy)
    Sama,
    /// Increased (excess)
    Vriddhi,
    /// Decreased (deficient)
    Kshaya,
}

/// System health monitor
pub struct AyurvedaMonitor {
    /// Vāta metrics
    vata: DoshaMetrics,
    /// Pitta metrics
    pitta: DoshaMetrics,
    /// Kapha metrics
    kapha: DoshaMetrics,
    /// Start time
    start_time: Instant,
}

/// Metrics for a dosha
#[derive(Debug, Clone)]
pub struct DoshaMetrics {
    pub dosha: Dosha,
    pub current_value: f64,
    pub ideal_value: f64,
    pub threshold_high: f64,
    pub threshold_low: f64,
    pub history: Vec<(Instant, f64)>,
}

impl DoshaMetrics {
    pub fn new(dosha: Dosha, ideal: f64) -> Self {
        Self {
            dosha,
            current_value: ideal,
            ideal_value: ideal,
            threshold_high: ideal * 1.5,
            threshold_low: ideal * 0.5,
            history: Vec::new(),
        }
    }

    pub fn update(&mut self, value: f64) {
        self.current_value = value;
        self.history.push((Instant::now(), value));

        // Keep only last 1000 samples
        if self.history.len() > 1000 {
            self.history.remove(0);
        }
    }

    pub fn state(&self) -> DoshaState {
        if self.current_value > self.threshold_high {
            DoshaState::Vriddhi
        } else if self.current_value < self.threshold_low {
            DoshaState::Kshaya
        } else {
            DoshaState::Sama
        }
    }

    pub fn deviation(&self) -> f64 {
        (self.current_value - self.ideal_value).abs() / self.ideal_value
    }
}

impl AyurvedaMonitor {
    pub fn new() -> Self {
        Self {
            vata: DoshaMetrics::new(Dosha::Vata, 50.0),   // CPU usage %
            pitta: DoshaMetrics::new(Dosha::Pitta, 50.0), // Memory usage %
            kapha: DoshaMetrics::new(Dosha::Kapha, 50.0), // Disk usage %
            start_time: Instant::now(),
        }
    }

    /// Update Vāta (CPU metrics)
    pub fn update_vata(&mut self, cpu_usage: f64) {
        self.vata.update(cpu_usage);
    }

    /// Update Pitta (Memory metrics)
    pub fn update_pitta(&mut self, memory_usage: f64) {
        self.pitta.update(memory_usage);
    }

    /// Update Kapha (Storage metrics)
    pub fn update_kapha(&mut self, disk_usage: f64) {
        self.kapha.update(disk_usage);
    }

    /// Get overall prakṛti (constitution/health)
    pub fn prakriti(&self) -> SystemPrakriti {
        let dominant = [
            (Dosha::Vata, self.vata.current_value),
            (Dosha::Pitta, self.pitta.current_value),
            (Dosha::Kapha, self.kapha.current_value),
        ]
        .iter()
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
        .map(|(d, _)| *d)
        .unwrap();

        SystemPrakriti {
            dominant_dosha: dominant,
            vata_state: self.vata.state(),
            pitta_state: self.pitta.state(),
            kapha_state: self.kapha.state(),
        }
    }

    /// Get health recommendations
    pub fn recommendations(&self) -> Vec<String> {
        let mut recs = Vec::new();

        // Vāta recommendations
        match self.vata.state() {
            DoshaState::Vriddhi => {
                recs.push("High CPU: Consider parallelization or reducing computation".to_string());
            }
            DoshaState::Kshaya => {
                recs.push("Low CPU: System is underutilized, consider batch processing".to_string());
            }
            DoshaState::Sama => {}
        }

        // Pitta recommendations
        match self.pitta.state() {
            DoshaState::Vriddhi => {
                recs.push("High Memory: Consider memory optimization or garbage collection".to_string());
            }
            DoshaState::Kshaya => {
                recs.push("Low Memory: System has excess capacity".to_string());
            }
            DoshaState::Sama => {}
        }

        // Kapha recommendations
        match self.kapha.state() {
            DoshaState::Vriddhi => {
                recs.push("High Disk: Consider cleanup or compression".to_string());
            }
            DoshaState::Kshaya => {
                recs.push("Low Disk: Storage is underutilized".to_string());
            }
            DoshaState::Sama => {}
        }

        recs
    }

    /// Generate health report
    pub fn report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Āyurveda System Health Report ===\n\n");

        let prakriti = self.prakriti();
        report.push_str(&format!("Dominant Dosha: {:?}\n\n", prakriti.dominant_dosha));

        for (name, metrics) in [
            ("Vāta (CPU)", &self.vata),
            ("Pitta (Memory)", &self.pitta),
            ("Kapha (Storage)", &self.kapha),
        ] {
            let state = metrics.state();
            let symbol = match state {
                DoshaState::Sama => "●",
                DoshaState::Vriddhi => "▲",
                DoshaState::Kshaya => "▼",
            };
            report.push_str(&format!(
                "{} {}: {:.1}% ({:?})\n",
                symbol, name, metrics.current_value, state
            ));
        }

        let recs = self.recommendations();
        if !recs.is_empty() {
            report.push_str("\nRecommendations:\n");
            for rec in recs {
                report.push_str(&format!("  - {}\n", rec));
            }
        }

        report
    }
}

/// System constitution
#[derive(Debug, Clone)]
pub struct SystemPrakriti {
    pub dominant_dosha: Dosha,
    pub vata_state: DoshaState,
    pub pitta_state: DoshaState,
    pub kapha_state: DoshaState,
}

impl Default for AyurvedaMonitor {
    fn default() -> Self {
        Self::new()
    }
}
