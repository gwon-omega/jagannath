//! Chitta Vṛtti - Mind-State Management
//!
//! Concurrency model based on Yoga Sūtras:
//! "Yogaś citta-vṛtti-nirodhaḥ" - Yoga is the cessation of mind fluctuations
//!
//! Maps mental states to thread/process states.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};

/// Chitta Vṛttis (mental modifications/fluctuations)
/// Maps to thread states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ChittaVritti {
    /// Pramāṇa (valid cognition) - Active execution
    Pramana,

    /// Viparyaya (misconception) - Error state
    Viparyaya,

    /// Vikalpa (imagination) - Speculative execution
    Vikalpa,

    /// Nidrā (sleep) - Suspended/blocked
    Nidra,

    /// Smṛti (memory) - Waiting on I/O/cache
    Smriti,
}

impl ChittaVritti {
    /// Get Sanskrit name
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::Pramana => "प्रमाण",
            Self::Viparyaya => "विपर्यय",
            Self::Vikalpa => "विकल्प",
            Self::Nidra => "निद्रा",
            Self::Smriti => "स्मृति",
        }
    }

    /// Get thread state mapping
    pub fn thread_state(&self) -> &'static str {
        match self {
            Self::Pramana => "Running",
            Self::Viparyaya => "Error",
            Self::Vikalpa => "Speculative",
            Self::Nidra => "Blocked",
            Self::Smriti => "Waiting",
        }
    }
}

/// Vṛtti Nirodha (fluctuation cessation) - Synchronization primitive
pub struct VrittiNirodha {
    /// State counter
    state: AtomicU64,
    /// Waiters
    waiters: std::sync::Mutex<Vec<std::sync::mpsc::Sender<()>>>,
}

impl VrittiNirodha {
    pub fn new() -> Self {
        Self {
            state: AtomicU64::new(0),
            waiters: std::sync::Mutex::new(Vec::new()),
        }
    }

    /// Enter nirodha (quiescent state)
    pub fn enter_nirodha(&self) {
        self.state.fetch_add(1, Ordering::SeqCst);
    }

    /// Exit nirodha
    pub fn exit_nirodha(&self) {
        let prev = self.state.fetch_sub(1, Ordering::SeqCst);
        if prev == 1 {
            // Wake all waiters
            if let Ok(waiters) = self.waiters.lock() {
                for waiter in waiters.iter() {
                    let _ = waiter.send(());
                }
            }
        }
    }

    /// Wait for nirodha (all fluctuations to cease)
    pub fn await_nirodha(&self) {
        if self.state.load(Ordering::SeqCst) == 0 {
            return;
        }

        let (tx, rx) = std::sync::mpsc::channel();
        if let Ok(mut waiters) = self.waiters.lock() {
            waiters.push(tx);
        }

        let _ = rx.recv();
    }

    /// Is in nirodha state?
    pub fn is_nirodha(&self) -> bool {
        self.state.load(Ordering::SeqCst) == 0
    }
}

/// Thread state tracker
pub struct ChittaTracker {
    /// Thread states
    states: HashMap<u64, ChittaVritti>,
    /// State history
    history: Vec<(u64, ChittaVritti, std::time::Instant)>,
}

impl ChittaTracker {
    pub fn new() -> Self {
        Self {
            states: HashMap::new(),
            history: Vec::new(),
        }
    }

    /// Register a thread
    pub fn register(&mut self, thread_id: u64) {
        self.states.insert(thread_id, ChittaVritti::Nidra);
    }

    /// Update thread state
    pub fn update(&mut self, thread_id: u64, state: ChittaVritti) {
        self.states.insert(thread_id, state);
        self.history.push((thread_id, state, std::time::Instant::now()));
    }

    /// Get thread state
    pub fn state(&self, thread_id: u64) -> Option<ChittaVritti> {
        self.states.get(&thread_id).copied()
    }

    /// Count threads in each state
    pub fn state_counts(&self) -> HashMap<ChittaVritti, usize> {
        let mut counts = HashMap::new();
        for state in self.states.values() {
            *counts.entry(*state).or_insert(0) += 1;
        }
        counts
    }

    /// Get threads in error state
    pub fn error_threads(&self) -> Vec<u64> {
        self.states
            .iter()
            .filter(|(_, s)| **s == ChittaVritti::Viparyaya)
            .map(|(id, _)| *id)
            .collect()
    }
}

impl Default for VrittiNirodha {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ChittaTracker {
    fn default() -> Self {
        Self::new()
    }
}
