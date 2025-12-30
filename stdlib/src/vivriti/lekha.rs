//! # Lekha - Log Records (लेख)
//!
//! Log record structures.
//!
//! > **"लेखनं स्मृतिः शाश्वती"**
//! > *"Writing is eternal memory"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::collections::BTreeMap;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use super::stara::LogStara;

// ============================================================================
// LOG RECORD
// ============================================================================

/// Log record (लॉग अभिलेख)
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct LogAbhilekha {
    /// Log level
    pub stara: LogStara,
    /// Message
    pub sandesh: String,
    /// Target/module
    pub lakshya: Option<String>,
    /// File location
    pub phaila: Option<String>,
    /// Line number
    pub pankti: Option<u32>,
    /// Timestamp (Unix millis)
    pub samay: Option<u64>,
    /// Structured fields
    pub khetra: BTreeMap<String, LogMulya>,
}

#[cfg(feature = "alloc")]
impl LogAbhilekha {
    /// Create new log record
    pub fn nava(stara: LogStara, sandesh: String) -> Self {
        Self {
            stara,
            sandesh,
            lakshya: None,
            phaila: None,
            pankti: None,
            samay: None,
            khetra: BTreeMap::new(),
        }
    }

    /// Set target
    pub fn lakshya_sthapita(mut self, target: String) -> Self {
        self.lakshya = Some(target);
        self
    }

    /// Set location
    pub fn sthana_sthapita(mut self, file: String, line: u32) -> Self {
        self.phaila = Some(file);
        self.pankti = Some(line);
        self
    }

    /// Set timestamp
    pub fn samay_sthapita(mut self, timestamp: u64) -> Self {
        self.samay = Some(timestamp);
        self
    }

    /// Add string field
    pub fn sutra_khetra(mut self, key: String, value: String) -> Self {
        self.khetra.insert(key, LogMulya::Sutra(value));
        self
    }

    /// Add integer field
    pub fn purnanka_khetra(mut self, key: String, value: i64) -> Self {
        self.khetra.insert(key, LogMulya::Purnanka(value));
        self
    }

    /// Add float field
    pub fn dashamalav_khetra(mut self, key: String, value: f64) -> Self {
        self.khetra.insert(key, LogMulya::Dashamalav(value));
        self
    }

    /// Add boolean field
    pub fn satya_khetra(mut self, key: String, value: bool) -> Self {
        self.khetra.insert(key, LogMulya::Satya(value));
        self
    }
}

/// Log field value (लॉग मूल्य)
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub enum LogMulya {
    /// String value
    Sutra(String),
    /// Integer value
    Purnanka(i64),
    /// Float value
    Dashamalav(f64),
    /// Boolean value
    Satya(bool),
}

#[cfg(feature = "alloc")]
impl core::fmt::Display for LogMulya {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LogMulya::Sutra(s) => write!(f, "\"{}\"", s),
            LogMulya::Purnanka(n) => write!(f, "{}", n),
            LogMulya::Dashamalav(n) => write!(f, "{}", n),
            LogMulya::Satya(b) => write!(f, "{}", b),
        }
    }
}

// ============================================================================
// LOG BUILDER
// ============================================================================

/// Log record builder (लॉग निर्माता)
#[cfg(feature = "alloc")]
pub struct LogNirmata {
    record: LogAbhilekha,
}

#[cfg(feature = "alloc")]
impl LogNirmata {
    /// Create trace log
    pub fn padachihna(sandesh: impl Into<String>) -> Self {
        Self {
            record: LogAbhilekha::nava(LogStara::Padachihna, sandesh.into()),
        }
    }

    /// Create debug log
    pub fn nirikshana(sandesh: impl Into<String>) -> Self {
        Self {
            record: LogAbhilekha::nava(LogStara::Nirikshana, sandesh.into()),
        }
    }

    /// Create info log
    pub fn suchana(sandesh: impl Into<String>) -> Self {
        Self {
            record: LogAbhilekha::nava(LogStara::Suchana, sandesh.into()),
        }
    }

    /// Create warning log
    pub fn chetavani(sandesh: impl Into<String>) -> Self {
        Self {
            record: LogAbhilekha::nava(LogStara::Chetavani, sandesh.into()),
        }
    }

    /// Create error log
    pub fn truti(sandesh: impl Into<String>) -> Self {
        Self {
            record: LogAbhilekha::nava(LogStara::Truti, sandesh.into()),
        }
    }

    /// Create fatal log
    pub fn ghataka(sandesh: impl Into<String>) -> Self {
        Self {
            record: LogAbhilekha::nava(LogStara::Ghataka, sandesh.into()),
        }
    }

    /// Set target
    pub fn lakshya(mut self, target: impl Into<String>) -> Self {
        self.record.lakshya = Some(target.into());
        self
    }

    /// Set location
    pub fn sthana(mut self, file: impl Into<String>, line: u32) -> Self {
        self.record.phaila = Some(file.into());
        self.record.pankti = Some(line);
        self
    }

    /// Set timestamp
    pub fn samay(mut self, timestamp: u64) -> Self {
        self.record.samay = Some(timestamp);
        self
    }

    /// Add string field
    pub fn sutra(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.record
            .khetra
            .insert(key.into(), LogMulya::Sutra(value.into()));
        self
    }

    /// Add integer field
    pub fn purnanka(mut self, key: impl Into<String>, value: i64) -> Self {
        self.record
            .khetra
            .insert(key.into(), LogMulya::Purnanka(value));
        self
    }

    /// Add float field
    pub fn dashamalav(mut self, key: impl Into<String>, value: f64) -> Self {
        self.record
            .khetra
            .insert(key.into(), LogMulya::Dashamalav(value));
        self
    }

    /// Add boolean field
    pub fn satya(mut self, key: impl Into<String>, value: bool) -> Self {
        self.record
            .khetra
            .insert(key.into(), LogMulya::Satya(value));
        self
    }

    /// Build the record
    pub fn nirmana(self) -> LogAbhilekha {
        self.record
    }
}

// ============================================================================
// SPAN
// ============================================================================

/// Log span for tracing (अवधि)
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct LogAvadhi {
    /// Span name
    pub nama: String,
    /// Parent span ID (if any)
    pub janaka_id: Option<u64>,
    /// This span's ID
    pub id: u64,
    /// Start time
    pub arambha_samay: Option<u64>,
    /// Fields
    pub khetra: BTreeMap<String, LogMulya>,
}

#[cfg(feature = "alloc")]
impl LogAvadhi {
    /// Create new span
    pub fn nava(nama: impl Into<String>, id: u64) -> Self {
        Self {
            nama: nama.into(),
            janaka_id: None,
            id,
            arambha_samay: None,
            khetra: BTreeMap::new(),
        }
    }

    /// Set parent
    pub fn janaka(mut self, parent_id: u64) -> Self {
        self.janaka_id = Some(parent_id);
        self
    }

    /// Set start time
    pub fn arambha(mut self, timestamp: u64) -> Self {
        self.arambha_samay = Some(timestamp);
        self
    }

    /// Add field
    pub fn khetra(mut self, key: impl Into<String>, value: LogMulya) -> Self {
        self.khetra.insert(key.into(), value);
        self
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_log_builder() {
        let record = LogNirmata::suchana("Test message")
            .lakshya("test::module")
            .sutra("user", "john")
            .purnanka("count", 42)
            .nirmana();

        assert_eq!(record.stara, LogStara::Suchana);
        assert_eq!(record.sandesh, "Test message");
        assert_eq!(record.lakshya, Some("test::module".into()));
        assert_eq!(record.khetra.len(), 2);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_span() {
        let span = LogAvadhi::nava("my_operation", 1)
            .arambha(12345)
            .khetra("param", LogMulya::Sutra("value".into()));

        assert_eq!(span.nama, "my_operation");
        assert_eq!(span.id, 1);
        assert!(span.khetra.contains_key("param"));
    }
}
