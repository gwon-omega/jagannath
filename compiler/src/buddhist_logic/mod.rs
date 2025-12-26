//! Buddhist Logic - Catuṣkoṭi (Tetralemma)
//!
//! Four-valued logic for handling uncertainty:
//! 1. True (is)
//! 2. False (is not)
//! 3. Both (is and is not)
//! 4. Neither (neither is nor is not)

/// Catuṣkoṭi - Four-valued logic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Catuskoti {
    /// Asti (is) - Definitely true
    Asti,

    /// Nāsti (is not) - Definitely false
    Nasti,

    /// Ubhaya (both) - True and false (contradiction/superposition)
    Ubhaya,

    /// Anubhaya (neither) - Neither true nor false (unknown/undefined)
    Anubhaya,
}

impl Catuskoti {
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::Asti => "अस्ति",
            Self::Nasti => "नास्ति",
            Self::Ubhaya => "उभय",
            Self::Anubhaya => "अनुभय",
        }
    }

    /// Convert from boolean
    pub fn from_bool(b: bool) -> Self {
        if b { Self::Asti } else { Self::Nasti }
    }

    /// Try to convert to boolean
    pub fn to_bool(&self) -> Option<bool> {
        match self {
            Self::Asti => Some(true),
            Self::Nasti => Some(false),
            Self::Ubhaya | Self::Anubhaya => None,
        }
    }

    /// Is definitely known?
    pub fn is_definite(&self) -> bool {
        matches!(self, Self::Asti | Self::Nasti)
    }

    /// Logical NOT
    pub fn not(self) -> Self {
        match self {
            Self::Asti => Self::Nasti,
            Self::Nasti => Self::Asti,
            Self::Ubhaya => Self::Ubhaya,    // Both true and false → still both
            Self::Anubhaya => Self::Anubhaya, // Neither → still neither
        }
    }

    /// Logical AND
    pub fn and(self, other: Self) -> Self {
        match (self, other) {
            (Self::Asti, Self::Asti) => Self::Asti,
            (Self::Nasti, _) | (_, Self::Nasti) => Self::Nasti,
            (Self::Anubhaya, _) | (_, Self::Anubhaya) => Self::Anubhaya,
            (Self::Ubhaya, _) | (_, Self::Ubhaya) => Self::Ubhaya,
        }
    }

    /// Logical OR
    pub fn or(self, other: Self) -> Self {
        match (self, other) {
            (Self::Nasti, Self::Nasti) => Self::Nasti,
            (Self::Asti, _) | (_, Self::Asti) => Self::Asti,
            (Self::Anubhaya, _) | (_, Self::Anubhaya) => Self::Anubhaya,
            (Self::Ubhaya, _) | (_, Self::Ubhaya) => Self::Ubhaya,
        }
    }

    /// Join (for lattice operations in analysis)
    pub fn join(self, other: Self) -> Self {
        match (self, other) {
            (a, b) if a == b => a,
            (Self::Anubhaya, other) | (other, Self::Anubhaya) => other,
            _ => Self::Ubhaya, // Conflicting info → both
        }
    }

    /// Meet (for lattice operations)
    pub fn meet(self, other: Self) -> Self {
        match (self, other) {
            (a, b) if a == b => a,
            (Self::Ubhaya, other) | (other, Self::Ubhaya) => other,
            _ => Self::Anubhaya, // No common ground → neither
        }
    }
}

/// Four-valued type for nullable analysis
pub type Nullable<T> = Option<Option<T>>;

/// Extension trait for four-valued operations
pub trait CatuskotiExt<T> {
    fn catuskoti_state(&self) -> Catuskoti;
}

impl<T> CatuskotiExt<T> for Nullable<T> {
    fn catuskoti_state(&self) -> Catuskoti {
        match self {
            Some(Some(_)) => Catuskoti::Asti,  // Has value
            Some(None) => Catuskoti::Nasti,     // Explicitly null
            None => Catuskoti::Anubhaya,        // Unknown
        }
    }
}

/// Four-valued abstract interpreter
pub struct CatuskotiInterpreter {
    /// Variable states
    states: std::collections::HashMap<String, Catuskoti>,
}

impl CatuskotiInterpreter {
    pub fn new() -> Self {
        Self {
            states: std::collections::HashMap::new(),
        }
    }

    /// Set variable state
    pub fn set(&mut self, name: &str, state: Catuskoti) {
        self.states.insert(name.to_string(), state);
    }

    /// Get variable state
    pub fn get(&self, name: &str) -> Catuskoti {
        self.states.get(name).copied().unwrap_or(Catuskoti::Anubhaya)
    }

    /// Evaluate condition
    pub fn evaluate_condition(&self, condition: &str) -> Catuskoti {
        // Simple evaluation - TODO: proper expression parsing
        if let Some(state) = self.states.get(condition) {
            *state
        } else {
            Catuskoti::Anubhaya
        }
    }

    /// Merge states from two branches
    pub fn merge(&mut self, other: &Self) {
        for (name, &other_state) in &other.states {
            let self_state = self.get(name);
            let merged = self_state.join(other_state);
            self.set(name, merged);
        }
    }
}

impl Default for CatuskotiInterpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_catuskoti_logic() {
        assert_eq!(Catuskoti::Asti.not(), Catuskoti::Nasti);
        assert_eq!(Catuskoti::Asti.and(Catuskoti::Asti), Catuskoti::Asti);
        assert_eq!(Catuskoti::Asti.and(Catuskoti::Nasti), Catuskoti::Nasti);
        assert_eq!(Catuskoti::Ubhaya.not(), Catuskoti::Ubhaya);
    }
}
