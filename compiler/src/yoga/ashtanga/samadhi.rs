//! Samādhi - Absorption/Deployment (समाधि)
//!
//! The eighth and final limb of Ashtanga Yoga - Complete absorption.
//! In software, this is the final stage: perfect deployment.
//!
//! - All tests pass
//! - No warnings
//! - Benchmarks met
//! - Production ready
//! - Continuous deployment

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Deployment readiness criteria
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReadinessCriterion {
    /// All tests pass
    TestsPass,
    /// No compiler warnings
    NoWarnings,
    /// No linter errors
    NoLintErrors,
    /// Benchmarks meet targets
    BenchmarksMet,
    /// Security audit passed
    SecurityAudit,
    /// Documentation complete
    DocumentationComplete,
    /// All dependencies updated
    DependenciesUpdated,
    /// No known vulnerabilities
    NoVulnerabilities,
}

impl ReadinessCriterion {
    /// Sanskrit term
    pub fn sanskrit(&self) -> &'static str {
        match self {
            Self::TestsPass => "परीक्षा सिद्ध (Parīkṣā Siddha) - Tests accomplished",
            Self::NoWarnings => "निर्दोष (Nirdoṣa) - Faultless",
            Self::NoLintErrors => "शुद्ध (Śuddha) - Pure",
            Self::BenchmarksMet => "लक्ष्य प्राप्त (Lakṣya Prāpta) - Target achieved",
            Self::SecurityAudit => "सुरक्षित (Surakṣita) - Secured",
            Self::DocumentationComplete => "प्रलेखित (Pralekhita) - Documented",
            Self::DependenciesUpdated => "अद्यतन (Adyatana) - Updated",
            Self::NoVulnerabilities => "अभेद्य (Abhedya) - Impenetrable",
        }
    }
}

/// Deployment stage
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum DeploymentStage {
    /// Building
    Building,
    /// Testing
    Testing,
    /// Staging
    Staging,
    /// Production
    Production,
    /// Complete (Samādhi achieved)
    Samadhi,
}

/// Deployment status
#[derive(Debug, Clone)]
pub struct DeploymentStatus {
    pub criterion: ReadinessCriterion,
    pub passed: bool,
    pub message: Option<String>,
    pub checked_at: Option<Instant>,
}

/// Samādhi deployment manager
pub struct SamadhiDeployment {
    /// Criteria statuses
    criteria: HashMap<ReadinessCriterion, DeploymentStatus>,
    /// Current stage
    stage: DeploymentStage,
    /// Deployment history
    history: Vec<DeploymentEvent>,
    /// Configuration
    config: SamadhiConfig,
}

/// Deployment event
#[derive(Debug, Clone)]
pub struct DeploymentEvent {
    pub timestamp: Instant,
    pub event_type: EventType,
    pub message: String,
}

/// Event types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    StageTransition,
    CriterionPassed,
    CriterionFailed,
    DeploymentStarted,
    DeploymentCompleted,
    DeploymentFailed,
    Rollback,
}

/// Configuration for deployment
#[derive(Debug, Clone)]
pub struct SamadhiConfig {
    /// Required criteria for production
    pub required_criteria: Vec<ReadinessCriterion>,
    /// Allow deployment with warnings?
    pub allow_warnings: bool,
    /// Auto-rollback on failure?
    pub auto_rollback: bool,
    /// Maximum deployment duration
    pub max_duration: Duration,
}

impl Default for SamadhiConfig {
    fn default() -> Self {
        Self {
            required_criteria: vec![
                ReadinessCriterion::TestsPass,
                ReadinessCriterion::NoWarnings,
                ReadinessCriterion::SecurityAudit,
            ],
            allow_warnings: false,
            auto_rollback: true,
            max_duration: Duration::from_secs(600), // 10 minutes
        }
    }
}

impl SamadhiDeployment {
    pub fn new(config: SamadhiConfig) -> Self {
        Self {
            criteria: HashMap::new(),
            stage: DeploymentStage::Building,
            history: Vec::new(),
            config,
        }
    }

    /// Record criterion check result
    pub fn record_criterion(&mut self, criterion: ReadinessCriterion, passed: bool, message: Option<String>) {
        let status = DeploymentStatus {
            criterion,
            passed,
            message: message.clone(),
            checked_at: Some(Instant::now()),
        };

        self.criteria.insert(criterion, status);

        self.history.push(DeploymentEvent {
            timestamp: Instant::now(),
            event_type: if passed { EventType::CriterionPassed } else { EventType::CriterionFailed },
            message: format!("{:?}: {}", criterion, message.unwrap_or_else(|| (if passed { "PASSED" } else { "FAILED" }).to_string())),
        });
    }

    /// Check if all required criteria are met
    pub fn all_criteria_met(&self) -> bool {
        self.config.required_criteria.iter().all(|c| {
            self.criteria.get(c)
                .map(|s| s.passed)
                .unwrap_or(false)
        })
    }

    /// Transition to next stage
    pub fn transition(&mut self) -> Result<DeploymentStage, String> {
        let next_stage = match self.stage {
            DeploymentStage::Building => {
                if self.criteria.get(&ReadinessCriterion::NoWarnings)
                    .map(|s| s.passed || self.config.allow_warnings)
                    .unwrap_or(self.config.allow_warnings)
                {
                    DeploymentStage::Testing
                } else {
                    return Err("Build has warnings".to_string());
                }
            }
            DeploymentStage::Testing => {
                if self.criteria.get(&ReadinessCriterion::TestsPass)
                    .map(|s| s.passed)
                    .unwrap_or(false)
                {
                    DeploymentStage::Staging
                } else {
                    return Err("Tests did not pass".to_string());
                }
            }
            DeploymentStage::Staging => {
                if self.all_criteria_met() {
                    DeploymentStage::Production
                } else {
                    let missing: Vec<_> = self.config.required_criteria.iter()
                        .filter(|c| !self.criteria.get(c).map(|s| s.passed).unwrap_or(false))
                        .map(|c| format!("{:?}", c))
                        .collect();
                    return Err(format!("Missing criteria: {}", missing.join(", ")));
                }
            }
            DeploymentStage::Production => {
                DeploymentStage::Samadhi
            }
            DeploymentStage::Samadhi => {
                return Err("Already at Samādhi (complete)".to_string());
            }
        };

        self.history.push(DeploymentEvent {
            timestamp: Instant::now(),
            event_type: EventType::StageTransition,
            message: format!("{:?} → {:?}", self.stage, next_stage),
        });

        self.stage = next_stage;
        Ok(next_stage)
    }

    /// Deploy to production
    pub fn deploy(&mut self) -> DeploymentResult {
        let start = Instant::now();

        self.history.push(DeploymentEvent {
            timestamp: Instant::now(),
            event_type: EventType::DeploymentStarted,
            message: "Deployment initiated".to_string(),
        });

        // Check all required criteria
        if !self.all_criteria_met() {
            let missing: Vec<_> = self.config.required_criteria.iter()
                .filter(|c| !self.criteria.get(c).map(|s| s.passed).unwrap_or(false))
                .map(|c| format!("{:?}", c))
                .collect();

            self.history.push(DeploymentEvent {
                timestamp: Instant::now(),
                event_type: EventType::DeploymentFailed,
                message: format!("Missing criteria: {}", missing.join(", ")),
            });

            return DeploymentResult {
                success: false,
                stage_reached: self.stage,
                duration: start.elapsed(),
                message: format!("Deployment blocked: missing {}", missing.join(", ")),
            };
        }

        // Advance through stages
        while self.stage != DeploymentStage::Samadhi {
            match self.transition() {
                Ok(_) => continue,
                Err(e) => {
                    self.history.push(DeploymentEvent {
                        timestamp: Instant::now(),
                        event_type: EventType::DeploymentFailed,
                        message: e.clone(),
                    });

                    return DeploymentResult {
                        success: false,
                        stage_reached: self.stage,
                        duration: start.elapsed(),
                        message: e,
                    };
                }
            }
        }

        self.history.push(DeploymentEvent {
            timestamp: Instant::now(),
            event_type: EventType::DeploymentCompleted,
            message: "Samādhi achieved - deployment complete".to_string(),
        });

        DeploymentResult {
            success: true,
            stage_reached: DeploymentStage::Samadhi,
            duration: start.elapsed(),
            message: "🕉️ Samādhi achieved - perfect absorption into production".to_string(),
        }
    }

    /// Rollback deployment
    pub fn rollback(&mut self, reason: &str) {
        self.history.push(DeploymentEvent {
            timestamp: Instant::now(),
            event_type: EventType::Rollback,
            message: format!("Rollback initiated: {}", reason),
        });

        self.stage = DeploymentStage::Building;
    }

    /// Get current stage
    pub fn stage(&self) -> DeploymentStage {
        self.stage
    }

    /// Get readiness report
    pub fn readiness_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== Samādhi Readiness Report ===\n\n");
        report.push_str(&format!("Current Stage: {:?}\n\n", self.stage));

        report.push_str("Criteria Status:\n");
        for criterion in &self.config.required_criteria {
            let status = self.criteria.get(criterion);
            let (symbol, msg) = match status {
                Some(s) if s.passed => ("✓", "PASSED"),
                Some(s) => ("✗", s.message.as_deref().unwrap_or("FAILED")),
                None => ("○", "NOT CHECKED"),
            };
            report.push_str(&format!("  {} {:?} ({}): {}\n", symbol, criterion, criterion.sanskrit(), msg));
        }

        report.push_str(&format!("\nReady for Production: {}\n",
            if self.all_criteria_met() { "YES ✓" } else { "NO ✗" }));

        report
    }

    /// Get deployment history
    pub fn history(&self) -> &[DeploymentEvent] {
        &self.history
    }

    /// Clear for new deployment
    pub fn clear(&mut self) {
        self.criteria.clear();
        self.stage = DeploymentStage::Building;
        self.history.clear();
    }
}

/// Deployment result
#[derive(Debug)]
pub struct DeploymentResult {
    pub success: bool,
    pub stage_reached: DeploymentStage,
    pub duration: Duration,
    pub message: String,
}

impl Default for SamadhiDeployment {
    fn default() -> Self {
        Self::new(SamadhiConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deployment_flow() {
        let mut deployment = SamadhiDeployment::default();

        // Record passing all criteria
        deployment.record_criterion(ReadinessCriterion::TestsPass, true, None);
        deployment.record_criterion(ReadinessCriterion::NoWarnings, true, None);
        deployment.record_criterion(ReadinessCriterion::SecurityAudit, true, None);

        let result = deployment.deploy();
        assert!(result.success);
        assert_eq!(result.stage_reached, DeploymentStage::Samadhi);
    }

    #[test]
    fn test_deployment_blocked() {
        let mut deployment = SamadhiDeployment::default();

        // Only some criteria pass
        deployment.record_criterion(ReadinessCriterion::TestsPass, true, None);
        deployment.record_criterion(ReadinessCriterion::NoWarnings, false, Some("2 warnings".to_string()));

        let result = deployment.deploy();
        assert!(!result.success);
    }

    #[test]
    fn test_stage_transition() {
        let mut deployment = SamadhiDeployment::new(SamadhiConfig {
            allow_warnings: true,
            ..Default::default()
        });

        deployment.record_criterion(ReadinessCriterion::NoWarnings, true, None);

        let result = deployment.transition();
        assert!(result.is_ok());
        assert_eq!(deployment.stage(), DeploymentStage::Testing);
    }
}
