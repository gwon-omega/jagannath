//! Tāla - Synchronization (ताल)
//!
//! Synchronization primitives.

use std::sync::{Arc, Mutex, RwLock, Condvar, Barrier};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// Mutex (Rakṣaka - रक्षक)
pub type Rakshaka<T> = Mutex<T>;

/// RwLock (Paṭhana-Lekhana - पठनलेखन)
pub type PathanaLekhana<T> = RwLock<T>;

/// Atomic boolean (Paramāṇu-Tarka - परमाणुतर्क)
pub type ParamanuTarka = AtomicBool;

/// Atomic usize (Paramāṇu-Saṅkhyā - परमाणुसंख्या)
pub type ParamanuSankhya = AtomicUsize;

/// Memory ordering (Krama - क्रम)
pub mod krama {
    use std::sync::atomic::Ordering;

    /// Relaxed (Śithila - शिथिल)
    pub const SHITHILA: Ordering = Ordering::Relaxed;
    /// Acquire (Grāhaka - ग्राहक)
    pub const GRAHAKA: Ordering = Ordering::Acquire;
    /// Release (Mukta - मुक्त)
    pub const MUKTA: Ordering = Ordering::Release;
    /// AcqRel (Grāhaka-Mukta - ग्राहकमुक्त)
    pub const GRAHAKA_MUKTA: Ordering = Ordering::AcqRel;
    /// SeqCst (Ānukramika - आनुक्रमिक)
    pub const ANUKRAMIKA: Ordering = Ordering::SeqCst;
}

/// Condition variable (Pratyaya - प्रत्यय)
pub struct Pratyaya {
    inner: Condvar,
}

impl Pratyaya {
    /// Create new (नव - nava)
    pub fn nava() -> Self {
        Self {
            inner: Condvar::new()
        }
    }

    /// Wait (प्रतीक्षा - pratīkṣā)
    pub fn pratiksha<'a, T>(&self, guard: std::sync::MutexGuard<'a, T>)
        -> std::sync::LockResult<std::sync::MutexGuard<'a, T>>
    {
        self.inner.wait(guard)
    }

    /// Notify one (सूचय एकम् - sūcaya ekam)
    pub fn suchaya_ekam(&self) {
        self.inner.notify_one()
    }

    /// Notify all (सूचय सर्वान् - sūcaya sarvān)
    pub fn suchaya_sarvan(&self) {
        self.inner.notify_all()
    }
}

/// Barrier (Pratibandha - प्रतिबन्ध)
pub struct Pratibandha {
    inner: Barrier,
}

impl Pratibandha {
    /// Create new (नव - nava)
    pub fn nava(n: usize) -> Self {
        Self {
            inner: Barrier::new(n)
        }
    }

    /// Wait at barrier (प्रतीक्षा - pratīkṣā)
    pub fn pratiksha(&self) -> std::sync::BarrierWaitResult {
        self.inner.wait()
    }
}

/// Once (Sakṛt - सकृत्)
pub struct Sakrt {
    inner: std::sync::Once,
}

impl Sakrt {
    /// Create new
    pub const fn nava() -> Self {
        Self {
            inner: std::sync::Once::new()
        }
    }

    /// Call once (सकृत् आह्वय - sakṛt āhvaya)
    pub fn sakrt_ahvaya<F: FnOnce()>(&self, f: F) {
        self.inner.call_once(f)
    }

    /// Is completed (समाप्त - samāpta)
    pub fn samapta(&self) -> bool {
        self.inner.is_completed()
    }
}

// Channel module
pub mod nalika;
pub use nalika::*;

