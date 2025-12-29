//! # Revision Tracking - Saṃskāra (Impressions)
//!
//! Tracks changes to detect when cached results are stale.
//!
//! # Sanskrit Foundation
//!
//! **Saṃskāra** (संस्कार) - impressions/modifications:
//! Mental impressions left by past actions that influence future behavior.
//! In the compiler, revisions are the impressions left by changes.

use std::sync::atomic::{AtomicU64, Ordering};

/// A revision number representing a point in time
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Revision {
    /// The revision number
    number: u64,
}

impl Revision {
    /// Create a new revision (starts at 1)
    pub fn new() -> Self {
        Self { number: 1 }
    }

    /// Create from a specific number
    pub fn from_number(number: u64) -> Self {
        Self { number }
    }

    /// Get the revision number
    pub fn number(&self) -> u64 {
        self.number
    }

    /// Get the next revision
    pub fn next(&self) -> Self {
        Self {
            number: self.number + 1,
        }
    }

    /// Increment in place
    pub fn increment(&mut self) {
        self.number += 1;
    }

    /// Check if this revision is newer than another
    pub fn is_newer_than(&self, other: &Revision) -> bool {
        self.number > other.number
    }
}

impl Default for Revision {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Revision {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "r{}", self.number)
    }
}

/// Atomic revision counter for thread-safe updates
pub struct AtomicRevision {
    /// The atomic counter
    counter: AtomicU64,
}

impl AtomicRevision {
    /// Create a new atomic revision
    pub fn new() -> Self {
        Self {
            counter: AtomicU64::new(1),
        }
    }

    /// Get the current revision
    pub fn get(&self) -> Revision {
        Revision {
            number: self.counter.load(Ordering::Acquire),
        }
    }

    /// Increment and return the new revision
    pub fn increment(&self) -> Revision {
        let new = self.counter.fetch_add(1, Ordering::AcqRel) + 1;
        Revision { number: new }
    }

    /// Set to a specific value
    pub fn set(&self, revision: Revision) {
        self.counter.store(revision.number, Ordering::Release);
    }
}

impl Default for AtomicRevision {
    fn default() -> Self {
        Self::new()
    }
}

/// RAII guard for tracking revision during computation
pub struct RevisionGuard {
    /// Starting revision
    start: Revision,
    /// Whether completed successfully
    completed: bool,
}

impl RevisionGuard {
    /// Create a new guard
    pub fn new(revision: Revision) -> Self {
        Self {
            start: revision,
            completed: false,
        }
    }

    /// Mark as completed
    pub fn complete(&mut self) {
        self.completed = true;
    }

    /// Get the starting revision
    pub fn revision(&self) -> Revision {
        self.start
    }

    /// Check if computation is still valid (revision hasn't changed)
    pub fn is_valid(&self, current: &Revision) -> bool {
        self.start == *current
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_revision_creation() {
        let rev = Revision::new();
        assert_eq!(rev.number(), 1);
    }

    #[test]
    fn test_revision_increment() {
        let mut rev = Revision::new();
        rev.increment();
        assert_eq!(rev.number(), 2);
    }

    #[test]
    fn test_revision_next() {
        let rev = Revision::new();
        let next = rev.next();
        assert_eq!(next.number(), 2);
        assert_eq!(rev.number(), 1); // Original unchanged
    }

    #[test]
    fn test_revision_comparison() {
        let rev1 = Revision::from_number(5);
        let rev2 = Revision::from_number(10);

        assert!(rev2.is_newer_than(&rev1));
        assert!(!rev1.is_newer_than(&rev2));
    }

    #[test]
    fn test_revision_display() {
        let rev = Revision::from_number(42);
        assert_eq!(rev.to_string(), "r42");
    }

    #[test]
    fn test_atomic_revision() {
        let atomic = AtomicRevision::new();

        let rev1 = atomic.get();
        assert_eq!(rev1.number(), 1);

        let rev2 = atomic.increment();
        assert_eq!(rev2.number(), 2);

        let rev3 = atomic.get();
        assert_eq!(rev3.number(), 2);
    }

    #[test]
    fn test_revision_guard() {
        let rev = Revision::new();
        let mut guard = RevisionGuard::new(rev);

        assert!(guard.is_valid(&rev));
        assert!(!guard.completed);

        guard.complete();
        assert!(guard.completed);

        let new_rev = rev.next();
        assert!(!guard.is_valid(&new_rev));
    }
}
