//! Prakāra - Core Types (प्रकार)
//!
//! Fundamental type definitions.

/// Optional value (Vikalpa - विकल्प)
/// Alternative/choice - may or may not have value
pub enum Vikalpa<T> {
    /// Has value (Kiñcit - किञ्चित्)
    Kincit(T),
    /// No value (Śūnya - शून्य)
    Shunya,
}

impl<T> Vikalpa<T> {
    /// Check if has value
    pub fn asti(&self) -> bool {
        matches!(self, Self::Kincit(_))
    }

    /// Check if empty
    pub fn shunya(&self) -> bool {
        matches!(self, Self::Shunya)
    }

    /// Unwrap or panic
    pub fn uddhrta(self) -> T {
        match self {
            Self::Kincit(val) => val,
            Self::Shunya => panic!("Called uddhrta on Shunya"),
        }
    }

    /// Unwrap or default
    pub fn uddhrta_va(self, default: T) -> T {
        match self {
            Self::Kincit(val) => val,
            Self::Shunya => default,
        }
    }

    /// Map function
    pub fn citra<U, F: FnOnce(T) -> U>(self, f: F) -> Vikalpa<U> {
        match self {
            Self::Kincit(val) => Vikalpa::Kincit(f(val)),
            Self::Shunya => Vikalpa::Shunya,
        }
    }
}

/// Result type (Pariṇāma - परिणाम)
/// Outcome - success or failure
pub enum Parinama<T, E> {
    /// Success (Siddhi - सिद्धि)
    Siddhi(T),
    /// Failure (Doṣa - दोष)
    Dosha(E),
}

impl<T, E> Parinama<T, E> {
    /// Check if success
    pub fn siddha(&self) -> bool {
        matches!(self, Self::Siddhi(_))
    }

    /// Check if failure
    pub fn doshavat(&self) -> bool {
        matches!(self, Self::Dosha(_))
    }

    /// Unwrap success or panic
    pub fn uddhrta(self) -> T {
        match self {
            Self::Siddhi(val) => val,
            Self::Dosha(_) => panic!("Called uddhrta on Dosha"),
        }
    }

    /// Map success
    pub fn citra<U, F: FnOnce(T) -> U>(self, f: F) -> Parinama<U, E> {
        match self {
            Self::Siddhi(val) => Parinama::Siddhi(f(val)),
            Self::Dosha(err) => Parinama::Dosha(err),
        }
    }

    /// Map error
    pub fn dosha_citra<F2, E2, FE: FnOnce(E) -> E2>(self, f: FE) -> Parinama<T, E2> {
        match self {
            Self::Siddhi(val) => Parinama::Siddhi(val),
            Self::Dosha(err) => Parinama::Dosha(f(err)),
        }
    }
}

/// Unit type (Śūnya - शून्य)
pub type Shunya = ();

/// Boolean (Satya-Asatya - सत्य/असत्य)
pub type Tarka = bool;

/// True constant
pub const SATYA: Tarka = true;
/// False constant
pub const ASATYA: Tarka = false;
