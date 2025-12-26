//! Tantu - Threads (तन्तु)
//!
//! Threading and concurrency.

use std::thread::{self, JoinHandle};
use std::time::Duration;

/// Thread handle (Tantu-Dhāraka - तन्तुधारक)
pub struct TantuDharaka<T> {
    inner: JoinHandle<T>,
}

impl<T> TantuDharaka<T> {
    /// Wait for completion (प्रतीक्षा - pratīkṣā)
    pub fn pratiksha(self) -> std::thread::Result<T> {
        self.inner.join()
    }

    /// Is finished (समाप्त - samāpta)
    pub fn samapta(&self) -> bool {
        self.inner.is_finished()
    }
}

/// Spawn new thread (उत्पन्न - utpanna)
pub fn utpanna<F, T>(f: F) -> TantuDharaka<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    TantuDharaka {
        inner: thread::spawn(f)
    }
}

/// Sleep (निद्रा - nidrā)
pub fn nidra(duration: Duration) {
    thread::sleep(duration)
}

/// Sleep milliseconds (निद्रा मिलि - nidrā mili)
pub fn nidra_mili(millis: u64) {
    thread::sleep(Duration::from_millis(millis))
}

/// Sleep seconds (निद्रा क्षण - nidrā kṣaṇa)
pub fn nidra_kshana(secs: u64) {
    thread::sleep(Duration::from_secs(secs))
}

/// Yield (त्यज - tyaja)
pub fn tyaja() {
    thread::yield_now()
}

/// Current thread ID (वर्तमान - vartamāna)
pub fn vartamana_id() -> thread::ThreadId {
    thread::current().id()
}

/// Park thread (विराम - virāma)
pub fn virama() {
    thread::park()
}

/// Park with timeout (विराम काल - virāma kāla)
pub fn virama_kala(duration: Duration) {
    thread::park_timeout(duration)
}

/// Thread builder (Tantu-Nirmātṛ - तन्तुनिर्मातृ)
pub struct TantuNirmatr {
    inner: thread::Builder,
}

impl TantuNirmatr {
    /// Create new builder (नव - nava)
    pub fn nava() -> Self {
        Self {
            inner: thread::Builder::new()
        }
    }

    /// Set name (नाम - nāma)
    pub fn nama(mut self, name: &str) -> Self {
        self.inner = self.inner.name(name.to_string());
        self
    }

    /// Set stack size (चिति आकार - citi ākāra)
    pub fn citi_akara(mut self, size: usize) -> Self {
        self.inner = self.inner.stack_size(size);
        self
    }

    /// Spawn (उत्पन्न - utpanna)
    pub fn utpanna<F, T>(self, f: F) -> io::Result<TantuDharaka<T>>
    where
        F: FnOnce() -> T + Send + 'static,
        T: Send + 'static,
    {
        Ok(TantuDharaka {
            inner: self.inner.spawn(f)?
        })
    }
}

use std::io;
