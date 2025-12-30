//! # Lekhaka - Loggers (लेखक)
//!
//! Logger implementations - console, formatter, writers.
//!
//! > **"वाचं वद सत्यं च"**
//! > *"Speak speech and truth"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::format;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use super::lekha::LogAbhilekha;
use super::stara::{LogChanak, LogStara};

// ============================================================================
// LOG TRAIT
// ============================================================================

/// Logger trait (लेखक)
#[cfg(feature = "alloc")]
pub trait Lekhaka: Send + Sync {
    /// Log a record
    fn likha(&self, record: &LogAbhilekha);

    /// Flush buffered logs
    fn bahao(&self) {}

    /// Check if level is enabled
    fn samartha(&self, stara: LogStara) -> bool;
}

// ============================================================================
// FORMAT
// ============================================================================

/// Log format (स्वरूप)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogSvarupa {
    /// Simple text
    Saral,
    /// With timestamp
    SamayYukta,
    /// JSON structured
    Sanrachita,
    /// Compact single line
    Sankshepa,
    /// Full with all fields
    Purna,
}

impl Default for LogSvarupa {
    fn default() -> Self {
        LogSvarupa::SamayYukta
    }
}

/// Format a log record
#[cfg(feature = "alloc")]
pub fn rupankarita(record: &LogAbhilekha, svarupa: LogSvarupa) -> String {
    match svarupa {
        LogSvarupa::Saral => format_saral(record),
        LogSvarupa::SamayYukta => format_samay_yukta(record),
        LogSvarupa::Sanrachita => format_sanrachita(record),
        LogSvarupa::Sankshepa => format_sankshepa(record),
        LogSvarupa::Purna => format_purna(record),
    }
}

#[cfg(feature = "alloc")]
fn format_saral(record: &LogAbhilekha) -> String {
    format!("[{}] {}", record.stara.laghu_nama(), record.sandesh)
}

#[cfg(feature = "alloc")]
fn format_samay_yukta(record: &LogAbhilekha) -> String {
    let samay = record.samay.unwrap_or(0);
    let secs = samay / 1000;
    let millis = samay % 1000;
    format!(
        "{}.{:03} [{}] {}",
        secs,
        millis,
        record.stara.laghu_nama(),
        record.sandesh
    )
}

#[cfg(feature = "alloc")]
fn format_sanrachita(record: &LogAbhilekha) -> String {
    let mut parts = Vec::new();

    parts.push(format!("\"stara\":\"{}\"", record.stara.nama()));
    parts.push(format!("\"sandesh\":\"{}\"", escape_json(&record.sandesh)));

    if let Some(ref lakshya) = record.lakshya {
        parts.push(format!("\"lakshya\":\"{}\"", escape_json(lakshya)));
    }

    if let Some(ref phaila) = record.phaila {
        parts.push(format!("\"phaila\":\"{}\"", escape_json(phaila)));
    }

    if let Some(pankti) = record.pankti {
        parts.push(format!("\"pankti\":{}", pankti));
    }

    if let Some(samay) = record.samay {
        parts.push(format!("\"samay\":{}", samay));
    }

    if !record.khetra.is_empty() {
        let fields: Vec<String> = record
            .khetra
            .iter()
            .map(|(k, v)| format!("\"{}\":{}", k, v))
            .collect();
        parts.push(format!("\"khetra\":{{{}}}", fields.join(",")));
    }

    format!("{{{}}}", parts.join(","))
}

#[cfg(feature = "alloc")]
fn format_sankshepa(record: &LogAbhilekha) -> String {
    let lakshya = record.lakshya.as_deref().unwrap_or("");
    format!(
        "{} {} {}",
        record.stara.laghu_nama(),
        lakshya,
        record.sandesh
    )
}

#[cfg(feature = "alloc")]
fn format_purna(record: &LogAbhilekha) -> String {
    let mut result = String::new();

    // Timestamp
    if let Some(samay) = record.samay {
        let secs = samay / 1000;
        let millis = samay % 1000;
        result.push_str(&format!("{}.{:03} ", secs, millis));
    }

    // Level
    result.push_str(&format!("[{}]", record.stara.nama()));

    // Target
    if let Some(ref lakshya) = record.lakshya {
        result.push_str(&format!(" {}", lakshya));
    }

    // Location
    if let Some(ref phaila) = record.phaila {
        if let Some(pankti) = record.pankti {
            result.push_str(&format!(" ({}:{})", phaila, pankti));
        } else {
            result.push_str(&format!(" ({})", phaila));
        }
    }

    // Message
    result.push_str(&format!(": {}", record.sandesh));

    // Fields
    if !record.khetra.is_empty() {
        result.push_str(" {");
        let fields: Vec<String> = record
            .khetra
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect();
        result.push_str(&fields.join(", "));
        result.push('}');
    }

    result
}

#[cfg(feature = "alloc")]
fn escape_json(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c => result.push(c),
        }
    }
    result
}

// ============================================================================
// CONSOLE LOGGER
// ============================================================================

/// Console logger (सांत्वना लेखक)
#[cfg(feature = "std")]
pub struct SantavanaLekhaka {
    chanak: LogChanak,
    svarupa: LogSvarupa,
    rang_samartha: bool,
}

#[cfg(feature = "std")]
impl SantavanaLekhaka {
    /// Create new console logger
    pub fn nava() -> Self {
        Self {
            chanak: LogChanak::nava(LogStara::Suchana),
            svarupa: LogSvarupa::default(),
            rang_samartha: true,
        }
    }

    /// Set minimum level
    pub fn nyunatam_stara(mut self, stara: LogStara) -> Self {
        self.chanak = LogChanak::nava(stara);
        self
    }

    /// Set format
    pub fn svarupa(mut self, svarupa: LogSvarupa) -> Self {
        self.svarupa = svarupa;
        self
    }

    /// Enable/disable colors
    pub fn rang(mut self, samartha: bool) -> Self {
        self.rang_samartha = samartha;
        self
    }

    fn rang_koda(&self, stara: LogStara) -> &'static str {
        if !self.rang_samartha {
            return "";
        }
        match stara {
            LogStara::Padachihna => "\x1b[90m", // Gray
            LogStara::Nirikshana => "\x1b[36m", // Cyan
            LogStara::Suchana => "\x1b[32m",    // Green
            LogStara::Chetavani => "\x1b[33m",  // Yellow
            LogStara::Truti => "\x1b[31m",      // Red
            LogStara::Ghataka => "\x1b[35m",    // Magenta
        }
    }

    fn rang_punasthapita(&self) -> &'static str {
        if self.rang_samartha {
            "\x1b[0m"
        } else {
            ""
        }
    }
}

#[cfg(feature = "std")]
impl Lekhaka for SantavanaLekhaka {
    fn likha(&self, record: &LogAbhilekha) {
        if !self.samartha(record.stara) {
            return;
        }

        let formatted = rupankarita(record, self.svarupa);
        let rang = self.rang_koda(record.stara);
        let reset = self.rang_punasthapita();

        eprintln!("{}{}{}", rang, formatted, reset);
    }

    fn samartha(&self, stara: LogStara) -> bool {
        self.chanak.manya(stara)
    }
}

// ============================================================================
// BUFFER LOGGER
// ============================================================================

/// In-memory buffer logger (बफर लेखक) - single-threaded only
/// Use for testing and debugging purposes
#[cfg(feature = "alloc")]
pub struct BapharLekhaka {
    chanak: LogChanak,
    svarupa: LogSvarupa,
    #[cfg(feature = "std")]
    logs: std::sync::Mutex<Vec<String>>,
    #[cfg(not(feature = "std"))]
    logs: core::cell::RefCell<Vec<String>>,
    adhikatam: usize,
}

#[cfg(feature = "alloc")]
impl BapharLekhaka {
    /// Create new buffer logger
    #[cfg(feature = "std")]
    pub fn nava() -> Self {
        Self {
            chanak: LogChanak::nava(LogStara::Padachihna),
            svarupa: LogSvarupa::Saral,
            logs: std::sync::Mutex::new(Vec::new()),
            adhikatam: 10000,
        }
    }

    #[cfg(not(feature = "std"))]
    pub fn nava() -> Self {
        Self {
            chanak: LogChanak::nava(LogStara::Padachihna),
            svarupa: LogSvarupa::Saral,
            logs: core::cell::RefCell::new(Vec::new()),
            adhikatam: 10000,
        }
    }

    /// Set minimum level
    pub fn nyunatam_stara(mut self, stara: LogStara) -> Self {
        self.chanak = LogChanak::nava(stara);
        self
    }

    /// Set format
    pub fn svarupa(mut self, svarupa: LogSvarupa) -> Self {
        self.svarupa = svarupa;
        self
    }

    /// Set maximum entries
    pub fn adhikatam(mut self, max: usize) -> Self {
        self.adhikatam = max;
        self
    }

    /// Get all logs
    #[cfg(feature = "std")]
    pub fn sabhi_prapta(&self) -> Vec<String> {
        self.logs.lock().unwrap().clone()
    }

    #[cfg(not(feature = "std"))]
    pub fn sabhi_prapta(&self) -> Vec<String> {
        self.logs.borrow().clone()
    }

    /// Clear logs
    #[cfg(feature = "std")]
    pub fn shuddhikarana(&self) {
        self.logs.lock().unwrap().clear();
    }

    #[cfg(not(feature = "std"))]
    pub fn shuddhikarana(&self) {
        self.logs.borrow_mut().clear();
    }

    /// Get log count
    #[cfg(feature = "std")]
    pub fn ganana(&self) -> usize {
        self.logs.lock().unwrap().len()
    }

    #[cfg(not(feature = "std"))]
    pub fn ganana(&self) -> usize {
        self.logs.borrow().len()
    }
}

#[cfg(all(feature = "alloc", feature = "std"))]
impl Lekhaka for BapharLekhaka {
    fn likha(&self, record: &LogAbhilekha) {
        if !self.samartha(record.stara) {
            return;
        }

        let formatted = rupankarita(record, self.svarupa);
        let mut logs = self.logs.lock().unwrap();

        if logs.len() >= self.adhikatam {
            logs.remove(0);
        }

        logs.push(formatted);
    }

    fn samartha(&self, stara: LogStara) -> bool {
        self.chanak.manya(stara)
    }
}

// ============================================================================
// MULTI LOGGER
// ============================================================================

/// Multiple loggers combined (बहु लेखक)
#[cfg(feature = "alloc")]
pub struct BahuLekhaka {
    lekhaka_suci: Vec<Box<dyn Lekhaka>>,
}

#[cfg(feature = "alloc")]
impl BahuLekhaka {
    /// Create new multi logger
    pub fn nava() -> Self {
        Self {
            lekhaka_suci: Vec::new(),
        }
    }

    /// Add a logger
    pub fn jodna(mut self, lekhaka: Box<dyn Lekhaka>) -> Self {
        self.lekhaka_suci.push(lekhaka);
        self
    }
}

#[cfg(feature = "alloc")]
impl Lekhaka for BahuLekhaka {
    fn likha(&self, record: &LogAbhilekha) {
        for lekhaka in &self.lekhaka_suci {
            lekhaka.likha(record);
        }
    }

    fn bahao(&self) {
        for lekhaka in &self.lekhaka_suci {
            lekhaka.bahao();
        }
    }

    fn samartha(&self, stara: LogStara) -> bool {
        self.lekhaka_suci.iter().any(|l| l.samartha(stara))
    }
}

// ============================================================================
// NULL LOGGER
// ============================================================================

/// Null logger - discards everything (शून्य लेखक)
pub struct ShunyaLekhaka;

impl ShunyaLekhaka {
    /// Create null logger
    pub fn nava() -> Self {
        Self
    }
}

#[cfg(feature = "alloc")]
impl Lekhaka for ShunyaLekhaka {
    fn likha(&self, _record: &LogAbhilekha) {}

    fn samartha(&self, _stara: LogStara) -> bool {
        false
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vivriti::lekha::LogNirmata;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_format_saral() {
        let record = LogNirmata::suchana("Hello").nirmana();
        let formatted = rupankarita(&record, LogSvarupa::Saral);
        // laghu_nama returns single char 'I' for Suchana (info)
        assert!(formatted.contains("[I]") || formatted.contains("INFO"));
        assert!(formatted.contains("Hello"));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_format_sanrachita() {
        let record = LogNirmata::truti("Error occurred")
            .lakshya("test")
            .purnanka("code", 500)
            .nirmana();
        let formatted = rupankarita(&record, LogSvarupa::Sanrachita);
        assert!(formatted.starts_with('{'));
        assert!(formatted.ends_with('}'));
        assert!(formatted.contains("ERROR"));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_buffer_logger() {
        let logger = BapharLekhaka::nava();

        let record = LogNirmata::suchana("Test 1").nirmana();
        logger.likha(&record);

        let record = LogNirmata::chetavani("Test 2").nirmana();
        logger.likha(&record);

        assert_eq!(logger.ganana(), 2);

        let logs = logger.sabhi_prapta();
        assert!(logs[0].contains("Test 1"));
        assert!(logs[1].contains("Test 2"));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_buffer_max() {
        let logger = BapharLekhaka::nava().adhikatam(3);

        for i in 0..5 {
            let record = LogNirmata::suchana(format!("Msg {}", i)).nirmana();
            logger.likha(&record);
        }

        assert_eq!(logger.ganana(), 3);
        let logs = logger.sabhi_prapta();
        assert!(logs[0].contains("Msg 2"));
        assert!(logs[2].contains("Msg 4"));
    }
}
