//! Nyāya Module (न्याय) — Logic and Epistemology
//!
//! Provides the four pramāṇas (means of valid knowledge) for runtime type introspection.

use core::fmt;

/// The four Pramāṇas (means of valid knowledge)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pramana {
    /// Pratyakṣa (प्रत्यक्ष) - Direct perception
    Pratyaksha,
    /// Anumāna (अनुमान) - Inference
    Anumana,
    /// Upamāna (उपमान) - Comparison/Analogy
    Upamana,
    /// Śabda (शब्द) - Verbal testimony
    Shabda,
}

impl Pramana {
    /// Get the certainty level of this pramāṇa (0.0 - 1.0)
    pub fn certainty(&self) -> f32 {
        match self {
            Pramana::Pratyaksha => 1.0,
            Pramana::Anumana => 0.95,
            Pramana::Upamana => 0.85,
            Pramana::Shabda => 0.90,
        }
    }

    /// Get the Sanskrit name
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Pramana::Pratyaksha => "प्रत्यक्ष",
            Pramana::Anumana => "अनुमान",
            Pramana::Upamana => "उपमान",
            Pramana::Shabda => "शब्द",
        }
    }
}

impl fmt::Display for Pramana {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})",
            match self {
                Pramana::Pratyaksha => "Pratyaksha",
                Pramana::Anumana => "Anumana",
                Pramana::Upamana => "Upamana",
                Pramana::Shabda => "Shabda",
            },
            self.sanskrit_name()
        )
    }
}

/// Evidence for inference (hetu)
#[derive(Debug, Clone)]
pub struct Hetu<T> {
    /// The evidence itself
    pub evidence: T,
    /// Source pramāṇa
    pub source: Pramana,
    /// Confidence level
    pub confidence: f32,
}

impl<T> Hetu<T> {
    /// Create new evidence from direct observation
    pub fn pratyaksha(evidence: T) -> Self {
        Self {
            evidence,
            source: Pramana::Pratyaksha,
            confidence: 1.0,
        }
    }

    /// Create new evidence from inference
    pub fn anumana(evidence: T, confidence: f32) -> Self {
        Self {
            evidence,
            source: Pramana::Anumana,
            confidence: confidence.min(0.95),
        }
    }

    /// Map the evidence while preserving metadata
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Hetu<U> {
        Hetu {
            evidence: f(self.evidence),
            source: self.source,
            confidence: self.confidence,
        }
    }
}

/// The five-membered syllogism (Pañcāvayava)
#[derive(Debug, Clone)]
pub struct Syllogism<T> {
    /// Pratijñā (Proposition): The thesis to be proved
    pub pratijnā: T,
    /// Hetu (Reason): The reason or evidence
    pub hetu: T,
    /// Udāharaṇa (Example): The universal rule with example
    pub udaharana: T,
    /// Upanaya (Application): Application of the rule
    pub upanaya: T,
    /// Nigamana (Conclusion): The conclusion
    pub nigamana: T,
}

impl<T: Clone> Syllogism<T> {
    /// Create a new syllogism
    pub fn new(
        pratijnā: T,
        hetu: T,
        udaharana: T,
        upanaya: T,
        nigamana: T,
    ) -> Self {
        Self { pratijnā, hetu, udaharana, upanaya, nigamana }
    }

    /// Validate the syllogism (check logical consistency)
    pub fn validate<F>(&self, validator: F) -> bool
    where
        F: Fn(&T, &T) -> bool
    {
        // Hetu must support pratijñā
        validator(&self.hetu, &self.pratijnā) &&
        // Udāharaṇa must be consistent with hetu
        validator(&self.udaharana, &self.hetu) &&
        // Conclusion must follow
        validator(&self.upanaya, &self.nigamana)
    }
}

/// Hetvābhāsa - Fallacies in reasoning
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Hetvabhasa {
    /// Asiddha - Unestablished reason
    Asiddha,
    /// Viruddha - Contradictory reason
    Viruddha,
    /// Anaikāntika - Inconclusive reason
    Anaikantika,
    /// Satpratipakṣa - Counter-balanced reason
    Satpratipaksha,
    /// Bādhita - Contradicted reason
    Badhita,
}

impl Hetvabhasa {
    /// Describe the fallacy
    pub fn description(&self) -> &'static str {
        match self {
            Self::Asiddha => "The reason is not established",
            Self::Viruddha => "The reason contradicts the thesis",
            Self::Anaikantika => "The reason is inconclusive",
            Self::Satpratipaksha => "An equally strong counter-argument exists",
            Self::Badhita => "The reason is contradicted by other evidence",
        }
    }
}

/// Result of logical inference
pub type NyayaResult<T> = Result<Hetu<T>, Hetvabhasa>;

/// Perform inference with the given evidence
pub fn anumiti<T, F>(evidence: &[Hetu<T>], inference: F) -> NyayaResult<T>
where
    F: FnOnce(&[&T]) -> Option<T>,
    T: Clone,
{
    if evidence.is_empty() {
        return Err(Hetvabhasa::Asiddha);
    }

    let refs: Vec<&T> = evidence.iter().map(|h| &h.evidence).collect();

    match inference(&refs) {
        Some(result) => {
            let avg_confidence: f32 = evidence.iter()
                .map(|h| h.confidence)
                .sum::<f32>() / evidence.len() as f32;

            Ok(Hetu::anumana(result, avg_confidence))
        }
        None => Err(Hetvabhasa::Anaikantika),
    }
}
