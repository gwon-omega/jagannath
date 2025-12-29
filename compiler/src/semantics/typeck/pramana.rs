//! Nyāya Pramāṇa - Valid Means of Knowledge (प्रमाण)
//!
//! Implements the Nyāya philosophical epistemology for type inference.
//! The four pramāṇas represent different sources of knowledge with
//! varying certainty levels, mapped to type inference strategies.
//!
//! ## The Four Pramāṇas:
//!
//! | Pramāṇa | Meaning | Type Inference | Certainty |
//! |---------|---------|----------------|-----------|
//! | Pratyakṣa | Direct perception | Explicit annotation | 100% |
//! | Anumāna | Inference | Logical deduction | 95% |
//! | Śabda | Testimony | Function signatures | 90% |
//! | Upamāna | Comparison | Pattern matching | 85% |
//!
//! This philosophical framework provides a principled way to
//! communicate inference certainty to users, helping them understand
//! when explicit annotations might improve code clarity.

/// Nyāya pramāṇa (प्रमाण - means of valid knowledge)
///
/// The four valid sources of knowledge in Nyāya philosophy,
/// mapped to type inference certainty levels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pramana {
    /// Pratyakṣa (प्रत्यक्ष) - Direct perception (explicit annotation)
    /// Certainty: 100% - The programmer explicitly stated the type
    ///
    /// In Nyāya, pratyakṣa is the foundational pramāṇa - direct sensory
    /// perception of reality. For type checking, this maps to explicit
    /// type annotations where the programmer directly states their intent.
    Pratyaksha,

    /// Anumāna (अनुमान) - Inference (logical deduction)
    /// Certainty: 95% - Type deduced from logical reasoning
    ///
    /// Uses the 5-step Nyāya syllogism (Pañcāvayava):
    /// 1. Pratijñā (thesis): "x has type T"
    /// 2. Hetu (reason): "because x = expr"
    /// 3. Udāharaṇa (example): "wherever we see expr pattern, type T follows"
    /// 4. Upanaya (application): "x is such a case"
    /// 5. Nigamana (conclusion): "therefore x: T"
    Anumana,

    /// Śabda (शब्द) - Testimony (documentation/contract)
    /// Certainty: 90% - Type from authoritative source
    ///
    /// In Nyāya, śabda refers to the testimony of reliable authorities.
    /// For type checking, this maps to function signatures, documentation,
    /// and stdlib type contracts - authoritative sources of type information.
    Shabda,

    /// Upamāna (उपमान) - Comparison (analogy/pattern matching)
    /// Certainty: 85% - Type inferred by similarity to known patterns
    ///
    /// The lowest certainty pramāṇa, upamāna involves knowing something
    /// by its similarity to something already known. For type checking,
    /// this is used for structural pattern matching and generic inference.
    Upamana,
}

impl Pramana {
    /// Get certainty level (0.0 - 1.0)
    ///
    /// The certainty levels follow the Nyāya hierarchy:
    /// - Pratyakṣa (100%): Direct perception is most certain
    /// - Anumāna (95%): Inference is highly reliable
    /// - Śabda (90%): Testimony requires trust in authority
    /// - Upamāna (85%): Comparison is least certain
    pub fn certainty(&self) -> f32 {
        match self {
            Pramana::Pratyaksha => 1.0,
            Pramana::Anumana => 0.95,
            Pramana::Shabda => 0.90,
            Pramana::Upamana => 0.85,
        }
    }

    /// Sanskrit name with Devanagari script
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Pramana::Pratyaksha => "प्रत्यक्ष",
            Pramana::Anumana => "अनुमान",
            Pramana::Shabda => "शब्द",
            Pramana::Upamana => "उपमान",
        }
    }

    /// Romanized Sanskrit name with diacritics
    pub fn romanized_name(&self) -> &'static str {
        match self {
            Pramana::Pratyaksha => "Pratyakṣa",
            Pramana::Anumana => "Anumāna",
            Pramana::Shabda => "Śabda",
            Pramana::Upamana => "Upamāna",
        }
    }

    /// English meaning
    pub fn meaning(&self) -> &'static str {
        match self {
            Pramana::Pratyaksha => "Direct Perception",
            Pramana::Anumana => "Inference",
            Pramana::Shabda => "Testimony",
            Pramana::Upamana => "Comparison",
        }
    }

    /// Type inference application
    pub fn inference_method(&self) -> &'static str {
        match self {
            Pramana::Pratyaksha => "Explicit type annotation",
            Pramana::Anumana => "Logical deduction from context",
            Pramana::Shabda => "Function signature or documentation",
            Pramana::Upamana => "Pattern matching by analogy",
        }
    }

    /// Get a suggestion for increasing certainty
    pub fn suggestion(&self) -> &'static str {
        match self {
            Pramana::Pratyaksha => "Type is explicitly annotated - maximum certainty",
            Pramana::Anumana => "Add explicit annotation for Pratyakṣa (100%) certainty",
            Pramana::Shabda => "Add explicit annotation to override function signature",
            Pramana::Upamana => "Add explicit annotation - pattern inference is least certain",
        }
    }

    /// All pramāṇas in order of certainty (highest first)
    pub fn all() -> &'static [Pramana] {
        &[
            Pramana::Pratyaksha,
            Pramana::Anumana,
            Pramana::Shabda,
            Pramana::Upamana,
        ]
    }
}

impl std::fmt::Display for Pramana {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ({}, {:.0}%)",
            self.romanized_name(),
            self.meaning(),
            self.certainty() * 100.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_certainty_ordering() {
        assert!(Pramana::Pratyaksha.certainty() > Pramana::Anumana.certainty());
        assert!(Pramana::Anumana.certainty() > Pramana::Shabda.certainty());
        assert!(Pramana::Shabda.certainty() > Pramana::Upamana.certainty());
    }

    #[test]
    fn test_sanskrit_names() {
        assert_eq!(Pramana::Pratyaksha.sanskrit_name(), "प्रत्यक्ष");
        assert_eq!(Pramana::Anumana.sanskrit_name(), "अनुमान");
        assert_eq!(Pramana::Shabda.sanskrit_name(), "शब्द");
        assert_eq!(Pramana::Upamana.sanskrit_name(), "उपमान");
    }

    #[test]
    fn test_display() {
        let display = format!("{}", Pramana::Pratyaksha);
        assert!(display.contains("Pratyakṣa"));
        assert!(display.contains("100%"));
    }
}
