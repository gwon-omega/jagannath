//! # Penance
//!
//! Actions required to atone for code sins.

/// A penance action to redeem from error
#[derive(Debug, Clone)]
pub struct Penance {
    /// Type of penance
    pub kind: PenanceKind,
    /// Description
    pub description: String,
    /// Required (must do) vs recommended
    pub required: bool,
}

/// Kinds of penance
#[derive(Debug, Clone)]
pub enum PenanceKind {
    /// Add a check
    AddCheck(String),
    /// Add error handling
    AddErrorHandling(String),
    /// Add resource cleanup
    AddCleanup(String),
    /// Add synchronization
    AddSync(String),
    /// Add sanitization
    AddSanitization(String),
    /// Remove unsafe code
    RemoveUnsafe(String),
    /// Refactor code
    Refactor(String),
    /// Add documentation
    AddDocumentation(String),
    /// Add test
    AddTest(String),
}

impl Penance {
    /// Create a new required penance
    pub fn required(kind: PenanceKind, description: &str) -> Self {
        Self {
            kind,
            description: description.to_string(),
            required: true,
        }
    }

    /// Create a recommended penance
    pub fn recommended(kind: PenanceKind, description: &str) -> Self {
        Self {
            kind,
            description: description.to_string(),
            required: false,
        }
    }

    /// Get description
    pub fn describe(&self) -> String {
        let prefix = if self.required { "[Required]" } else { "[Recommended]" };
        format!("{} {}", prefix, self.description)
    }

    /// Get Sanskrit name for penance type
    pub fn sanskrit_name(&self) -> &'static str {
        match &self.kind {
            PenanceKind::AddCheck(_) => "parīkṣā-yoga",           // add examination
            PenanceKind::AddErrorHandling(_) => "doṣa-nirvāraṇa", // error prevention
            PenanceKind::AddCleanup(_) => "śuddhi-karma",         // purification action
            PenanceKind::AddSync(_) => "samaya-bandha",           // time binding
            PenanceKind::AddSanitization(_) => "pāvana-kriyā",    // cleansing action
            PenanceKind::RemoveUnsafe(_) => "bhaya-tyāga",        // fear abandonment
            PenanceKind::Refactor(_) => "punar-racanā",           // re-composition
            PenanceKind::AddDocumentation(_) => "vivṛti-lekha",   // explanation writing
            PenanceKind::AddTest(_) => "parīkṣā-sthāpana",        // test establishment
        }
    }
}

/// Common penances for specific errors
impl Penance {
    /// Penance for use-after-free
    pub fn for_use_after_free() -> Self {
        Self::required(
            PenanceKind::AddCheck("null_check_before_use".to_string()),
            "Add null check before using pointer, or restructure ownership"
        )
    }

    /// Penance for buffer overflow
    pub fn for_buffer_overflow() -> Self {
        Self::required(
            PenanceKind::AddCheck("bounds_check".to_string()),
            "Add bounds checking before array access"
        )
    }

    /// Penance for resource leak
    pub fn for_resource_leak() -> Self {
        Self::required(
            PenanceKind::AddCleanup("resource_free".to_string()),
            "Add resource cleanup (defer/finally/RAII)"
        )
    }

    /// Penance for race condition
    pub fn for_race_condition() -> Self {
        Self::required(
            PenanceKind::AddSync("mutex_lock".to_string()),
            "Add proper synchronization (mutex/lock/atomic)"
        )
    }

    /// Penance for tainted data
    pub fn for_tainted_data() -> Self {
        Self::required(
            PenanceKind::AddSanitization("input_validation".to_string()),
            "Sanitize input before use in sensitive operations"
        )
    }
}
