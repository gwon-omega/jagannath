//! # Avidyā - Ignorance (Inefficiency/Bugs)
//!
//! > **"अविद्यायाम् अन्तरे वर्तमानाः स्वयं धीराः पण्डितंमन्यमानाः"**
//! > *"Living in ignorance, considering themselves wise and learned"*
//! > — Kaṭha Upaniṣad 1.2.5
//!
//! In the Moksha framework, **Avidyā** represents all forms of ignorance
//! that bind the Jīva (source code) and prevent liberation:
//!
//! ## Types of Avidyā
//! - **Bugs** - Logical errors (misconception of reality)
//! - **Inefficiencies** - Wasted resources (attachment to unnecessary)
//! - **Technical Debt** - Deferred cleanup (unresolved karma)
//! - **Anti-patterns** - Wrong approaches (confused knowledge)
//! - **Security Flaws** - Vulnerabilities (false sense of safety)
//!
//! ## The Five Kleshas (Afflictions)
//! Classical Yoga identifies five Kleshas, which map to compiler issues:
//! - **Avidyā** - Ignorance (root cause of bugs)
//! - **Asmitā** - Ego (over-engineered code)
//! - **Rāga** - Attachment (reluctance to refactor)
//! - **Dveṣa** - Aversion (avoiding necessary complexity)
//! - **Abhiniveśa** - Fear of death (backward compatibility concerns)

use super::jiva::Jiva;

/// Avidyā - A unit of ignorance/inefficiency in the code
#[derive(Debug, Clone)]
pub struct Avidya {
    /// Type of ignorance
    pub kind: AvidyaKind,

    /// Specific category within kind
    pub category: AvidyaCategory,

    /// Location in source
    pub location: AvidyaLocation,

    /// Description of the issue
    pub description: String,

    /// Severity level
    pub severity: AvidyaSeverity,

    /// Root Klesha (affliction) causing this
    pub klesha: Klesha,

    /// Can be resolved through Tapas?
    pub removable: bool,

    /// Suggested remedy
    pub remedy: Option<String>,
}

/// Kinds of Avidyā (ignorance)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvidyaKind {
    /// Bug - Logical error
    Bug,

    /// Inefficiency - Performance waste
    Inefficiency,

    /// TechnicalDebt - Deferred cleanup
    TechnicalDebt,

    /// AntiPattern - Wrong approach
    AntiPattern,

    /// SecurityFlaw - Vulnerability
    SecurityFlaw,

    /// UndefinedBehavior - Dangerous code
    UndefinedBehavior,
}

impl AvidyaKind {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            AvidyaKind::Bug => "दोष",
            AvidyaKind::Inefficiency => "अनर्थ",
            AvidyaKind::TechnicalDebt => "ऋण",
            AvidyaKind::AntiPattern => "विपरीतज्ञान",
            AvidyaKind::SecurityFlaw => "छिद्र",
            AvidyaKind::UndefinedBehavior => "अनिश्चित",
        }
    }

    /// IAST transliteration
    pub fn iast(&self) -> &'static str {
        match self {
            AvidyaKind::Bug => "Doṣa",
            AvidyaKind::Inefficiency => "Anartha",
            AvidyaKind::TechnicalDebt => "Ṛṇa",
            AvidyaKind::AntiPattern => "Viparītajñāna",
            AvidyaKind::SecurityFlaw => "Chidra",
            AvidyaKind::UndefinedBehavior => "Aniścita",
        }
    }
}

/// Specific categories of Avidyā
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AvidyaCategory {
    // Bug categories
    LogicError,
    NullDereference,
    OutOfBounds,
    TypeMismatch,
    UseAfterFree,

    // Inefficiency categories
    DeadCode,
    RedundantComputation,
    MemoryLeak,
    UnusedAllocation,
    IneffientLoop,

    // Technical debt categories
    DuplicateCode,
    MagicNumbers,
    MissingDocumentation,
    ComplexFunction,
    LongMethod,

    // Anti-pattern categories
    GodClass,
    FeatureEnvy,
    DataClump,
    PrimitiveObsession,
    SpeculativeGenerality,

    // Security categories
    BufferOverflow,
    SqlInjection,
    PathTraversal,
    HardcodedCredentials,
    InsecureRandom,
}

impl AvidyaCategory {
    /// Get associated kind
    pub fn kind(&self) -> AvidyaKind {
        match self {
            // Bug categories
            AvidyaCategory::LogicError
            | AvidyaCategory::NullDereference
            | AvidyaCategory::OutOfBounds
            | AvidyaCategory::TypeMismatch
            | AvidyaCategory::UseAfterFree => AvidyaKind::Bug,

            // Inefficiency categories
            AvidyaCategory::DeadCode
            | AvidyaCategory::RedundantComputation
            | AvidyaCategory::MemoryLeak
            | AvidyaCategory::UnusedAllocation
            | AvidyaCategory::IneffientLoop => AvidyaKind::Inefficiency,

            // Technical debt categories
            AvidyaCategory::DuplicateCode
            | AvidyaCategory::MagicNumbers
            | AvidyaCategory::MissingDocumentation
            | AvidyaCategory::ComplexFunction
            | AvidyaCategory::LongMethod => AvidyaKind::TechnicalDebt,

            // Anti-pattern categories
            AvidyaCategory::GodClass
            | AvidyaCategory::FeatureEnvy
            | AvidyaCategory::DataClump
            | AvidyaCategory::PrimitiveObsession
            | AvidyaCategory::SpeculativeGenerality => AvidyaKind::AntiPattern,

            // Security categories
            AvidyaCategory::BufferOverflow
            | AvidyaCategory::SqlInjection
            | AvidyaCategory::PathTraversal
            | AvidyaCategory::HardcodedCredentials
            | AvidyaCategory::InsecureRandom => AvidyaKind::SecurityFlaw,
        }
    }
}

/// Location of Avidyā in source
#[derive(Debug, Clone)]
pub struct AvidyaLocation {
    /// File path
    pub file: Option<String>,

    /// Line number
    pub line: Option<usize>,

    /// Column number
    pub column: Option<usize>,

    /// Span (start, end)
    pub span: Option<(usize, usize)>,

    /// Function/method name
    pub function: Option<String>,
}

impl Default for AvidyaLocation {
    fn default() -> Self {
        Self {
            file: None,
            line: None,
            column: None,
            span: None,
            function: None,
        }
    }
}

/// Severity of Avidyā
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AvidyaSeverity {
    /// Hint - Informational only
    Hint,

    /// Style - Code style issue
    Style,

    /// Warning - Potential problem
    Warning,

    /// Error - Definite problem
    Error,

    /// Critical - Blocks liberation
    Critical,
}

impl AvidyaSeverity {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            AvidyaSeverity::Hint => "सूचना",
            AvidyaSeverity::Style => "शैली",
            AvidyaSeverity::Warning => "चेतावनी",
            AvidyaSeverity::Error => "त्रुटि",
            AvidyaSeverity::Critical => "गम्भीर",
        }
    }
}

/// The Five Kleshas (Afflictions) from Yoga Sutras
///
/// > **"अविद्यास्मितारागद्वेषाभिनिवेशाः क्लेशाः"**
/// > *"The afflictions are ignorance, egoism, attachment, aversion, and clinging to life"*
/// > — Yoga Sutras 2.3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Klesha {
    /// अविद्या - Root ignorance (all bugs stem from not understanding)
    #[default]
    Avidya,

    /// अस्मिता - Egoism (over-engineered, "clever" code)
    Asmita,

    /// राग - Attachment (reluctance to refactor working code)
    Raga,

    /// द्वेष - Aversion (avoiding necessary complexity/refactoring)
    Dvesha,

    /// अभिनिवेश - Fear of death (excessive backward compatibility)
    Abhinivesha,
}

impl Klesha {
    /// Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            Klesha::Avidya => "अविद्या",
            Klesha::Asmita => "अस्मिता",
            Klesha::Raga => "राग",
            Klesha::Dvesha => "द्वेष",
            Klesha::Abhinivesha => "अभिनिवेश",
        }
    }

    /// IAST transliteration
    pub fn iast(&self) -> &'static str {
        match self {
            Klesha::Avidya => "Avidyā",
            Klesha::Asmita => "Asmitā",
            Klesha::Raga => "Rāga",
            Klesha::Dvesha => "Dveṣa",
            Klesha::Abhinivesha => "Abhiniveśa",
        }
    }

    /// Meaning in compilation context
    pub fn meaning(&self) -> &'static str {
        match self {
            Klesha::Avidya => "Root ignorance - bugs from not understanding",
            Klesha::Asmita => "Egoism - over-engineered 'clever' code",
            Klesha::Raga => "Attachment - reluctance to refactor",
            Klesha::Dvesha => "Aversion - avoiding necessary complexity",
            Klesha::Abhinivesha => "Fear - excessive backward compatibility",
        }
    }

    /// Remedy for this Klesha
    pub fn remedy(&self) -> &'static str {
        match self {
            Klesha::Avidya => "Study requirements, understand domain",
            Klesha::Asmita => "Keep it simple, follow YAGNI",
            Klesha::Raga => "Embrace refactoring, let go of attachment",
            Klesha::Dvesha => "Accept necessary complexity when justified",
            Klesha::Abhinivesha => "Define clear deprecation policies",
        }
    }
}

impl Avidya {
    /// Create new Avidyā
    pub fn new(category: AvidyaCategory, description: &str) -> Self {
        Self {
            kind: category.kind(),
            category,
            location: AvidyaLocation::default(),
            description: description.to_string(),
            severity: AvidyaSeverity::Warning,
            klesha: Klesha::Avidya,
            removable: true,
            remedy: None,
        }
    }

    /// Set location
    pub fn with_location(mut self, location: AvidyaLocation) -> Self {
        self.location = location;
        self
    }

    /// Set severity
    pub fn with_severity(mut self, severity: AvidyaSeverity) -> Self {
        self.severity = severity;
        self
    }

    /// Set klesha (root affliction)
    pub fn with_klesha(mut self, klesha: Klesha) -> Self {
        self.klesha = klesha;
        self
    }

    /// Set remedy
    pub fn with_remedy(mut self, remedy: &str) -> Self {
        self.remedy = Some(remedy.to_string());
        self
    }

    /// Mark as non-removable (blocking liberation)
    pub fn blocking(mut self) -> Self {
        self.removable = false;
        self
    }

    /// Check if this Avidyā blocks liberation
    pub fn is_blocking(&self) -> bool {
        !self.removable || self.severity == AvidyaSeverity::Critical
    }
}

/// Detector for Avidyā in Jīva
pub struct AvidyaDetector;

impl AvidyaDetector {
    /// Detect all Avidyā in Jīva
    pub fn detect(jiva: &Jiva) -> Vec<Avidya> {
        let mut avidyas = Vec::new();

        // Detect dead code
        avidyas.extend(Self::detect_dead_code(jiva));

        // Detect inefficiencies
        avidyas.extend(Self::detect_inefficiencies(jiva));

        // Detect technical debt
        avidyas.extend(Self::detect_technical_debt(jiva));

        // Detect anti-patterns
        avidyas.extend(Self::detect_anti_patterns(jiva));

        // Detect security flaws
        avidyas.extend(Self::detect_security_flaws(jiva));

        avidyas
    }

    fn detect_dead_code(_jiva: &Jiva) -> Vec<Avidya> {
        // Dead code detection would happen here
        Vec::new()
    }

    fn detect_inefficiencies(_jiva: &Jiva) -> Vec<Avidya> {
        // Inefficiency detection would happen here
        Vec::new()
    }

    fn detect_technical_debt(_jiva: &Jiva) -> Vec<Avidya> {
        // Technical debt detection would happen here
        Vec::new()
    }

    fn detect_anti_patterns(_jiva: &Jiva) -> Vec<Avidya> {
        // Anti-pattern detection would happen here
        Vec::new()
    }

    fn detect_security_flaws(_jiva: &Jiva) -> Vec<Avidya> {
        // Security flaw detection would happen here
        Vec::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_avidya_kinds() {
        assert_eq!(AvidyaKind::Bug.sanskrit(), "दोष");
        assert_eq!(AvidyaKind::TechnicalDebt.iast(), "Ṛṇa");
    }

    #[test]
    fn test_klesha() {
        assert_eq!(Klesha::Avidya.sanskrit(), "अविद्या");
        assert_eq!(Klesha::Asmita.meaning(), "Egoism - over-engineered 'clever' code");
    }

    #[test]
    fn test_avidya_creation() {
        let avidya = Avidya::new(AvidyaCategory::DeadCode, "Unused function")
            .with_severity(AvidyaSeverity::Warning)
            .with_klesha(Klesha::Raga);

        assert_eq!(avidya.kind, AvidyaKind::Inefficiency);
        assert_eq!(avidya.klesha, Klesha::Raga);
        assert!(avidya.removable);
    }

    #[test]
    fn test_blocking_avidya() {
        let avidya = Avidya::new(AvidyaCategory::UseAfterFree, "Memory error").blocking();
        assert!(avidya.is_blocking());
    }

    #[test]
    fn test_category_to_kind() {
        assert_eq!(AvidyaCategory::LogicError.kind(), AvidyaKind::Bug);
        assert_eq!(AvidyaCategory::DeadCode.kind(), AvidyaKind::Inefficiency);
        assert_eq!(AvidyaCategory::GodClass.kind(), AvidyaKind::AntiPattern);
    }
}
