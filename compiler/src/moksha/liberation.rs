//! # Liberation - The Final State of Moksha
//!
//! > **"ब्रह्म वेद ब्रह्मैव भवति"**
//! > *"One who knows Brahman becomes Brahman"*
//! > — Muṇḍaka Upaniṣad 3.2.9
//!
//! In the Moksha framework, **Liberation** represents the final state
//! where the Jīva has been completely transformed into Ātman.
//! This is the goal of compilation - perfect, optimized code.
//!
//! ## Four States of Liberation (Mukti)
//! - **Sālokya** - Same realm (correct syntax)
//! - **Sāmīpya** - Nearness (correct semantics)
//! - **Sārūpya** - Same form (optimized structure)
//! - **Sāyujya** - Union (perfect compilation)
//!
//! ## Signs of Liberation
//! - No Avidyā (bugs/inefficiency) remains
//! - All Karma (technical debt) resolved
//! - Sat-Cit-Ānanda achieved
//! - Binary is pure (Śuddha)

use super::atman::Atman;
use super::MokshaJourney;

/// The four stages of liberation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MuktiStage {
    /// सालोक्य - Same realm: Code compiles, exists in correct domain
    Salokya,

    /// सामीप्य - Nearness: Semantically correct, close to ideal
    Samipya,

    /// सारूप्य - Same form: Structurally optimized
    Sarupya,

    /// सायुज्य - Union: Perfect compilation, complete liberation
    Sayujya,
}

impl MuktiStage {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            MuktiStage::Salokya => "सालोक्य",
            MuktiStage::Samipya => "सामीप्य",
            MuktiStage::Sarupya => "सारूप्य",
            MuktiStage::Sayujya => "सायुज्य",
        }
    }

    /// IAST transliteration
    pub fn iast(&self) -> &'static str {
        match self {
            MuktiStage::Salokya => "Sālokya",
            MuktiStage::Samipya => "Sāmīpya",
            MuktiStage::Sarupya => "Sārūpya",
            MuktiStage::Sayujya => "Sāyujya",
        }
    }

    /// Meaning
    pub fn meaning(&self) -> &'static str {
        match self {
            MuktiStage::Salokya => "Same realm - code compiles correctly",
            MuktiStage::Samipya => "Nearness - semantically correct",
            MuktiStage::Sarupya => "Same form - structurally optimized",
            MuktiStage::Sayujya => "Union - perfect compilation achieved",
        }
    }

    /// Compilation phase
    pub fn compilation_phase(&self) -> &'static str {
        match self {
            MuktiStage::Salokya => "Parsing complete",
            MuktiStage::Samipya => "Type checking passed",
            MuktiStage::Sarupya => "Optimization complete",
            MuktiStage::Sayujya => "Final binary generated",
        }
    }
}

/// Certificate of Liberation
///
/// Issued when code achieves Moksha (perfect compilation).
#[derive(Debug, Clone)]
pub struct LiberationCertificate {
    /// The liberated Ātman
    pub atman: Atman,

    /// Stage of liberation achieved
    pub mukti_stage: MuktiStage,

    /// Time of liberation
    pub liberation_time: std::time::Instant,

    /// Journey statistics
    pub journey_stats: JourneyStatistics,

    /// Verification hash
    pub verification_hash: String,
}

/// Statistics from the liberation journey
#[derive(Debug, Clone, Default)]
pub struct JourneyStatistics {
    /// Total Avidyā discovered
    pub avidya_discovered: usize,

    /// Total Avidyā removed
    pub avidya_removed: usize,

    /// Tapas rounds performed
    pub tapas_rounds: usize,

    /// Saṃskāras formed
    pub samskaras_formed: usize,

    /// Veda phases completed
    pub vedas_completed: Vec<String>,

    /// Total compilation time
    pub total_time: std::time::Duration,
}

/// Verifier for liberation status
pub struct LiberationVerifier;

impl LiberationVerifier {
    /// Verify that liberation has been achieved
    pub fn verify(journey: &MokshaJourney) -> LiberationVerdict {
        // Check for remaining blocking Avidyā
        let blocking_avidya: Vec<_> = journey
            .avidya
            .iter()
            .filter(|a| a.is_blocking())
            .collect();

        if !blocking_avidya.is_empty() {
            return LiberationVerdict::Blocked {
                reason: format!("{} blocking Avidyā remain", blocking_avidya.len()),
                blockers: blocking_avidya.iter().map(|a| a.description.clone()).collect(),
            };
        }

        // Check Jīva state
        if journey.jiva.state < super::jiva::JivaState::Jivanmukta {
            return LiberationVerdict::NotReady {
                current_state: journey.jiva.state.iast().to_string(),
                required_state: "Jīvanmukta".to_string(),
            };
        }

        // Check karma weight
        let karma_weight = journey.jiva.karma_weight();
        if karma_weight > 10 {
            return LiberationVerdict::KarmaRemains {
                weight: karma_weight,
                threshold: 10,
            };
        }

        // All checks passed
        LiberationVerdict::Achieved {
            stage: Self::determine_stage(journey),
        }
    }

    fn determine_stage(journey: &MokshaJourney) -> MuktiStage {
        if journey.jiva.karma_weight() == 0 && journey.avidya.is_empty() {
            MuktiStage::Sayujya
        } else if journey.metrics.avidya_removed > journey.metrics.avidya_discovered / 2 {
            MuktiStage::Sarupya
        } else if journey.jiva.analysis.semantics_analyzed {
            MuktiStage::Samipya
        } else {
            MuktiStage::Salokya
        }
    }
}

/// Result of liberation verification
#[derive(Debug)]
pub enum LiberationVerdict {
    /// Liberation achieved
    Achieved { stage: MuktiStage },

    /// Blocked by remaining Avidyā
    Blocked {
        reason: String,
        blockers: Vec<String>,
    },

    /// Not yet ready
    NotReady {
        current_state: String,
        required_state: String,
    },

    /// Karma still accumulating
    KarmaRemains { weight: usize, threshold: usize },
}

impl LiberationVerdict {
    /// Is liberation achieved?
    pub fn is_achieved(&self) -> bool {
        matches!(self, LiberationVerdict::Achieved { .. })
    }
}

/// Generate liberation certificate
pub fn generate_certificate(
    journey: &MokshaJourney,
    atman: Atman,
) -> Option<LiberationCertificate> {
    let verdict = LiberationVerifier::verify(journey);

    if let LiberationVerdict::Achieved { stage } = verdict {
        Some(LiberationCertificate {
            atman,
            mukti_stage: stage,
            liberation_time: std::time::Instant::now(),
            journey_stats: JourneyStatistics {
                avidya_discovered: journey.metrics.avidya_discovered,
                avidya_removed: journey.metrics.avidya_removed,
                tapas_rounds: journey.metrics.tapas_rounds,
                samskaras_formed: journey.metrics.samskaras_formed,
                vedas_completed: vec![
                    "Rig Veda".into(),
                    "Yajur Veda".into(),
                    "Sāma Veda".into(),
                    "Atharva Veda".into(),
                ],
                total_time: journey
                    .metrics
                    .phase_times
                    .values()
                    .sum(),
            },
            verification_hash: generate_hash(&journey),
        })
    } else {
        None
    }
}

fn generate_hash(_journey: &MokshaJourney) -> String {
    // Would generate actual hash
    format!("moksha-{:x}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mukti_stages() {
        assert_eq!(MuktiStage::Salokya.sanskrit(), "सालोक्य");
        assert_eq!(MuktiStage::Sayujya.meaning(), "Union - perfect compilation achieved");
    }

    #[test]
    fn test_stage_ordering() {
        assert!(MuktiStage::Salokya < MuktiStage::Samipya);
        assert!(MuktiStage::Samipya < MuktiStage::Sarupya);
        assert!(MuktiStage::Sarupya < MuktiStage::Sayujya);
    }

    #[test]
    fn test_liberation_verdict() {
        let verdict = LiberationVerdict::Achieved {
            stage: MuktiStage::Sayujya,
        };
        assert!(verdict.is_achieved());

        let blocked = LiberationVerdict::Blocked {
            reason: "test".into(),
            blockers: vec![],
        };
        assert!(!blocked.is_achieved());
    }
}
