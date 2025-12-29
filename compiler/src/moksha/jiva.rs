//! # Jīva - The Individual Soul (Source Code)
//!
//! > **"जीवो ब्रह्मैव नापरः"**
//! > *"The Jīva is Brahman itself, nothing else"*
//!
//! In the Moksha framework, **Jīva** represents source code in its
//! unenlightened state - bound by inefficiencies, carrying the karma
//! of technical debt, trapped in the cycle of Saṃsāra.
//!
//! ## The Soul's Bondage
//! - Source code enters as Jīva with accumulated impurities
//! - Avidyā (ignorance) manifests as bugs, inefficiencies, anti-patterns
//! - Through Tapas (optimization), Jīva is purified
//! - Ultimate goal: merge into Ātman (optimized binary)
//!
//! ## Upādhi (Limiting Adjuncts)
//! The compiler progressively removes Upādhis:
//! - Syntactic sugar → Desugared AST
//! - High-level abstractions → MIR
//! - Platform-agnostic code → Target-specific assembly

use crate::parser::Ast;
use std::collections::HashMap;

/// Jīva - The individual soul embodied in source code
///
/// Represents the unenlightened state of code before optimization.
/// Contains accumulated karma (technical debt) and is bound by
/// Avidyā (inefficiency/ignorance).
#[derive(Debug, Clone)]
pub struct Jiva {
    /// Original source code (the soul's initial embodiment)
    pub source: String,

    /// Parsed AST (first layer of understanding)
    pub ast: Option<Ast>,

    /// Accumulated Karma (technical debt)
    pub karma: Vec<Karma>,

    /// Upādhis (limiting adjuncts) - layers to be stripped
    pub upadhis: Vec<Upadhi>,

    /// Current state in the liberation journey
    pub state: JivaState,

    /// Analysis results
    pub analysis: JivaAnalysis,
}

/// The state of Jīva in its liberation journey
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum JivaState {
    /// बद्ध - Bound: Initial unanalyzed state
    #[default]
    Baddha,

    /// मुमुक्षु - Seeker: Being analyzed
    Mumukshu,

    /// साधक - Practitioner: Undergoing transformation
    Sadhaka,

    /// जीवन्मुक्त - Liberated while living: Optimized but not yet final
    Jivanmukta,

    /// विदेहमुक्त - Bodiless liberation: Transformed to Ātman
    Videhamukta,
}

impl JivaState {
    /// Get Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            JivaState::Baddha => "बद्ध",
            JivaState::Mumukshu => "मुमुक्षु",
            JivaState::Sadhaka => "साधक",
            JivaState::Jivanmukta => "जीवन्मुक्त",
            JivaState::Videhamukta => "विदेहमुक्त",
        }
    }

    /// Get IAST transliteration
    pub fn iast(&self) -> &'static str {
        match self {
            JivaState::Baddha => "Baddha",
            JivaState::Mumukshu => "Mumukṣu",
            JivaState::Sadhaka => "Sādhaka",
            JivaState::Jivanmukta => "Jīvanmukta",
            JivaState::Videhamukta => "Videhamukta",
        }
    }

    /// Get meaning in compilation context
    pub fn meaning(&self) -> &'static str {
        match self {
            JivaState::Baddha => "Unanalyzed source (bound)",
            JivaState::Mumukshu => "Being parsed (seeking)",
            JivaState::Sadhaka => "Being optimized (practicing)",
            JivaState::Jivanmukta => "Optimized IR (liberated-while-embodied)",
            JivaState::Videhamukta => "Final binary (bodiless liberation)",
        }
    }
}

/// Karma - Technical debt and accumulated issues
#[derive(Debug, Clone)]
pub struct Karma {
    /// Type of karmic accumulation
    pub kind: KarmaKind,

    /// Description
    pub description: String,

    /// Location in source
    pub location: Option<(usize, usize)>,

    /// Severity (affects liberation difficulty)
    pub severity: KarmaSeverity,
}

/// Kinds of Karma (technical debt)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KarmaKind {
    /// Sanchita - Accumulated (legacy code)
    Sanchita,

    /// Prarabdha - Currently manifesting (active bugs)
    Prarabdha,

    /// Kriyamana - Being created (new issues)
    Kriyamana,

    /// Agami - Future consequences (potential problems)
    Agami,
}

impl KarmaKind {
    /// Get Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            KarmaKind::Sanchita => "संचित",
            KarmaKind::Prarabdha => "प्रारब्ध",
            KarmaKind::Kriyamana => "क्रियमाण",
            KarmaKind::Agami => "आगामि",
        }
    }

    /// Get IAST transliteration
    pub fn iast(&self) -> &'static str {
        match self {
            KarmaKind::Sanchita => "Sañcita",
            KarmaKind::Prarabdha => "Prārabdha",
            KarmaKind::Kriyamana => "Kriyamāṇa",
            KarmaKind::Agami => "Āgāmi",
        }
    }

    /// Get meaning
    pub fn meaning(&self) -> &'static str {
        match self {
            KarmaKind::Sanchita => "Accumulated (legacy debt)",
            KarmaKind::Prarabdha => "Currently manifesting (active bugs)",
            KarmaKind::Kriyamana => "Being created (new issues)",
            KarmaKind::Agami => "Future consequences (potential problems)",
        }
    }
}

/// Severity of Karma
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum KarmaSeverity {
    /// Light karma - easily resolved
    Light,
    /// Medium karma - requires effort
    Medium,
    /// Heavy karma - significant work
    Heavy,
    /// Critical karma - blocks liberation
    Critical,
}

/// Upādhi - Limiting adjunct (abstraction layer to be stripped)
#[derive(Debug, Clone)]
pub struct Upadhi {
    /// Type of limiting adjunct
    pub kind: UpadhiKind,

    /// Has this been stripped?
    pub stripped: bool,

    /// Transformation that stripped it
    pub stripped_by: Option<String>,
}

/// Types of Upādhis (abstraction layers)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpadhiKind {
    /// SyntacticSugar - Convenience syntax
    SyntacticSugar,

    /// HighLevelAbstraction - Iterator, closures, etc.
    HighLevelAbstraction,

    /// PlatformAgnostic - Generic code
    PlatformAgnostic,

    /// TypeAnnotation - Explicit types
    TypeAnnotation,

    /// LifetimeAnnotation - Explicit lifetimes
    LifetimeAnnotation,
}

impl UpadhiKind {
    /// Get Sanskrit name
    pub fn sanskrit(&self) -> &'static str {
        match self {
            UpadhiKind::SyntacticSugar => "मधुरवाक्",
            UpadhiKind::HighLevelAbstraction => "उच्चस्तर",
            UpadhiKind::PlatformAgnostic => "सर्वव्यापी",
            UpadhiKind::TypeAnnotation => "प्रकारचिह्न",
            UpadhiKind::LifetimeAnnotation => "आयुषचिह्न",
        }
    }
}

/// Analysis results for Jīva
#[derive(Debug, Clone, Default)]
pub struct JivaAnalysis {
    /// Grammar analysis complete
    pub grammar_analyzed: bool,

    /// Types analyzed
    pub types_analyzed: bool,

    /// Semantics analyzed
    pub semantics_analyzed: bool,

    /// Function count
    pub function_count: usize,

    /// Type definition count
    pub type_count: usize,

    /// Complexity metrics
    pub complexity: HashMap<String, usize>,
}

impl Jiva {
    /// Create Jīva from source code
    pub fn from_source(source: &str) -> Self {
        Self {
            source: source.to_string(),
            ast: None,
            karma: Vec::new(),
            upadhis: Self::initial_upadhis(),
            state: JivaState::Baddha,
            analysis: JivaAnalysis::default(),
        }
    }

    /// Create Jīva from AST
    pub fn from_ast(ast: Ast) -> Self {
        Self {
            source: String::new(),
            ast: Some(ast),
            karma: Vec::new(),
            upadhis: Self::initial_upadhis(),
            state: JivaState::Mumukshu,
            analysis: JivaAnalysis::default(),
        }
    }

    /// Initial Upādhis that all code has
    fn initial_upadhis() -> Vec<Upadhi> {
        vec![
            Upadhi {
                kind: UpadhiKind::SyntacticSugar,
                stripped: false,
                stripped_by: None,
            },
            Upadhi {
                kind: UpadhiKind::HighLevelAbstraction,
                stripped: false,
                stripped_by: None,
            },
            Upadhi {
                kind: UpadhiKind::PlatformAgnostic,
                stripped: false,
                stripped_by: None,
            },
            Upadhi {
                kind: UpadhiKind::TypeAnnotation,
                stripped: false,
                stripped_by: None,
            },
            Upadhi {
                kind: UpadhiKind::LifetimeAnnotation,
                stripped: false,
                stripped_by: None,
            },
        ]
    }

    /// Analyze grammar (Rig Veda phase)
    pub fn analyze_grammar(&mut self) {
        self.state = JivaState::Mumukshu;
        // Grammar analysis would happen here
        self.analysis.grammar_analyzed = true;
    }

    /// Analyze types (Rig Veda phase)
    pub fn analyze_types(&mut self) {
        // Type analysis would happen here
        self.analysis.types_analyzed = true;
    }

    /// Analyze semantics (Rig Veda phase)
    pub fn analyze_semantics(&mut self) {
        // Semantic analysis would happen here
        self.analysis.semantics_analyzed = true;
    }

    /// Perform ritual transformation (Yajur Veda phase)
    pub fn perform_ritual_transformation(&mut self) {
        self.state = JivaState::Sadhaka;
        // Transformation rituals happen here
        self.strip_upadhi(UpadhiKind::SyntacticSugar, "desugaring");
        self.strip_upadhi(UpadhiKind::HighLevelAbstraction, "lowering");
    }

    /// Harmonize (Sāma Veda phase)
    pub fn harmonize(&mut self) {
        self.state = JivaState::Jivanmukta;
        // Harmonization/balancing happens here
    }

    /// Apply practical magic (Atharva Veda phase)
    pub fn apply_practical_magic(&mut self) {
        // Runtime integration, stdlib linkage, etc.
        self.strip_upadhi(UpadhiKind::PlatformAgnostic, "target_specialization");
    }

    /// Strip an Upādhi (remove abstraction layer)
    fn strip_upadhi(&mut self, kind: UpadhiKind, by: &str) {
        for upadhi in &mut self.upadhis {
            if upadhi.kind == kind && !upadhi.stripped {
                upadhi.stripped = true;
                upadhi.stripped_by = Some(by.to_string());
                break;
            }
        }
    }

    /// Add Karma (technical debt)
    pub fn add_karma(&mut self, karma: Karma) {
        self.karma.push(karma);
    }

    /// Get total karma weight
    pub fn karma_weight(&self) -> usize {
        self.karma
            .iter()
            .map(|k| match k.severity {
                KarmaSeverity::Light => 1,
                KarmaSeverity::Medium => 3,
                KarmaSeverity::Heavy => 7,
                KarmaSeverity::Critical => 15,
            })
            .sum()
    }

    /// Check if ready for liberation
    pub fn ready_for_liberation(&self) -> bool {
        // No critical karma and most Upādhis stripped
        !self
            .karma
            .iter()
            .any(|k| k.severity == KarmaSeverity::Critical)
            && self.upadhis.iter().filter(|u| u.stripped).count() >= 3
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jiva_creation() {
        let jiva = Jiva::from_source("test code");
        assert_eq!(jiva.state, JivaState::Baddha);
        assert_eq!(jiva.karma.len(), 0);
        assert_eq!(jiva.upadhis.len(), 5);
    }

    #[test]
    fn test_jiva_states() {
        assert_eq!(JivaState::Baddha.sanskrit(), "बद्ध");
        assert_eq!(JivaState::Mumukshu.iast(), "Mumukṣu");
        assert_eq!(
            JivaState::Jivanmukta.meaning(),
            "Optimized IR (liberated-while-embodied)"
        );
    }

    #[test]
    fn test_karma_kinds() {
        assert_eq!(KarmaKind::Sanchita.sanskrit(), "संचित");
        assert_eq!(
            KarmaKind::Prarabdha.meaning(),
            "Currently manifesting (active bugs)"
        );
    }

    #[test]
    fn test_karma_weight() {
        let mut jiva = Jiva::from_source("test");
        jiva.add_karma(Karma {
            kind: KarmaKind::Prarabdha,
            description: "Bug".into(),
            location: None,
            severity: KarmaSeverity::Heavy,
        });
        jiva.add_karma(Karma {
            kind: KarmaKind::Sanchita,
            description: "Legacy".into(),
            location: None,
            severity: KarmaSeverity::Light,
        });
        assert_eq!(jiva.karma_weight(), 8); // 7 + 1
    }
}
