//! # Moksha Path
//!
//! Different paths to liberation from code sins.

use crate::garuda::narakas::Naraka;

/// Path to moksha (liberation from error)
#[derive(Debug, Clone)]
pub enum MokshaPath {
    /// Jñāna Yoga - Knowledge/Understanding
    /// Fix through better understanding of the code
    Jnana(JnanaFix),

    /// Karma Yoga - Action/Deeds
    /// Fix through code changes
    Karma(KarmaFix),

    /// Bhakti Yoga - Devotion/Discipline
    /// Fix through following best practices
    Bhakti(BhaktiFix),

    /// Raja Yoga - Royal/Complete
    /// Major refactoring required
    Raja(RajaFix),
}

impl MokshaPath {
    /// Get name of this path
    pub fn name(&self) -> &str {
        match self {
            MokshaPath::Jnana(_) => "Jñāna Yoga (Knowledge)",
            MokshaPath::Karma(_) => "Karma Yoga (Action)",
            MokshaPath::Bhakti(_) => "Bhakti Yoga (Discipline)",
            MokshaPath::Raja(_) => "Rāja Yoga (Royal)",
        }
    }

    /// Get the recommended path for a naraka
    pub fn for_naraka(naraka: &Naraka) -> Self {
        match naraka {
            // Memory errors → Karma (action/code changes)
            Naraka::Tamisram | Naraka::Andhatamisram | Naraka::Andhakupa |
            Naraka::Krimibhaksha | Naraka::Asipatravana => {
                MokshaPath::Karma(KarmaFix::default())
            }

            // Concurrency errors → Raja (major refactoring)
            Naraka::Sandamsha | Naraka::Pranarodha => {
                MokshaPath::Raja(RajaFix::default())
            }

            // Security errors → Bhakti (follow best practices)
            Naraka::Vaitarani | Naraka::Visasana | Naraka::Lalabhaksha => {
                MokshaPath::Bhakti(BhaktiFix::default())
            }

            // Logic errors → Jnana (understanding)
            _ => MokshaPath::Jnana(JnanaFix::default()),
        }
    }
}

/// Fix through knowledge/understanding
#[derive(Debug, Clone, Default)]
pub struct JnanaFix {
    /// Explanation of the issue
    pub explanation: String,
    /// Documentation reference
    pub doc_reference: Option<String>,
    /// Example of correct code
    pub example: Option<String>,
}

/// Fix through code action
#[derive(Debug, Clone, Default)]
pub struct KarmaFix {
    /// Code to remove
    pub remove: Option<String>,
    /// Code to add
    pub add: Option<String>,
    /// Replacement code
    pub replace: Option<(String, String)>,
}

/// Fix through discipline/best practices
#[derive(Debug, Clone, Default)]
pub struct BhaktiFix {
    /// Best practice to follow
    pub practice: String,
    /// Pattern to adopt
    pub pattern: Option<String>,
    /// Lint rule to enable
    pub lint_rule: Option<String>,
}

/// Major refactoring fix
#[derive(Debug, Clone, Default)]
pub struct RajaFix {
    /// Refactoring description
    pub description: String,
    /// Steps to perform
    pub steps: Vec<String>,
    /// Estimated effort
    pub effort: RefactoringEffort,
}

/// Effort required for refactoring
#[derive(Debug, Clone, Copy, Default)]
pub enum RefactoringEffort {
    #[default]
    Minor,
    Moderate,
    Major,
    Complete,
}
