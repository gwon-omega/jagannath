//! # Stara - Log Levels (स्तर)
//!
//! Log severity levels.
//!
//! > **"स्तरेण ज्ञायते गुरुता"**
//! > *"Severity is known by the level"*

// ============================================================================
// LOG LEVEL
// ============================================================================

/// Log level (लॉग स्तर)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum LogStara {
    /// Trace level - most detailed (पदचिह्न)
    Padachihna = 0,
    /// Debug level (डीबग)
    Nirikshana = 1,
    /// Info level (सूचना)
    Suchana = 2,
    /// Warning level (चेतावनी)
    Chetavani = 3,
    /// Error level (त्रुटि)
    Truti = 4,
    /// Fatal/Critical level (घातक)
    Ghataka = 5,
}

impl LogStara {
    /// Get level from integer
    pub fn se_sankhya(n: u8) -> Option<Self> {
        match n {
            0 => Some(LogStara::Padachihna),
            1 => Some(LogStara::Nirikshana),
            2 => Some(LogStara::Suchana),
            3 => Some(LogStara::Chetavani),
            4 => Some(LogStara::Truti),
            5 => Some(LogStara::Ghataka),
            _ => None,
        }
    }

    /// Get level name
    pub fn nama(&self) -> &'static str {
        match self {
            LogStara::Padachihna => "TRACE",
            LogStara::Nirikshana => "DEBUG",
            LogStara::Suchana => "INFO",
            LogStara::Chetavani => "WARN",
            LogStara::Truti => "ERROR",
            LogStara::Ghataka => "FATAL",
        }
    }

    /// Get Sanskrit name
    pub fn sanskrit_nama(&self) -> &'static str {
        match self {
            LogStara::Padachihna => "पदचिह्न",
            LogStara::Nirikshana => "निरीक्षण",
            LogStara::Suchana => "सूचना",
            LogStara::Chetavani => "चेतावनी",
            LogStara::Truti => "त्रुटि",
            LogStara::Ghataka => "घातक",
        }
    }

    /// Get short name (single char)
    pub fn laghu_nama(&self) -> char {
        match self {
            LogStara::Padachihna => 'T',
            LogStara::Nirikshana => 'D',
            LogStara::Suchana => 'I',
            LogStara::Chetavani => 'W',
            LogStara::Truti => 'E',
            LogStara::Ghataka => 'F',
        }
    }

    /// Get numeric value
    pub fn mana(&self) -> u8 {
        *self as u8
    }

    /// Check if this level is enabled for given filter
    pub fn samartha(&self, filter: LogStara) -> bool {
        *self >= filter
    }
}

impl Default for LogStara {
    fn default() -> Self {
        LogStara::Suchana
    }
}

impl core::fmt::Display for LogStara {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.nama())
    }
}

// ============================================================================
// LOG FILTER
// ============================================================================

/// Log filter configuration (छनक)
#[derive(Debug, Clone, Copy)]
pub struct LogChanak {
    /// Minimum level to log
    pub nyunatam_stara: LogStara,
    /// Maximum level to log (optional)
    pub adhikatam_stara: Option<LogStara>,
}

impl LogChanak {
    /// Create filter with minimum level
    pub fn nava(min_level: LogStara) -> Self {
        Self {
            nyunatam_stara: min_level,
            adhikatam_stara: None,
        }
    }

    /// Create filter with range
    pub fn sima(min_level: LogStara, max_level: LogStara) -> Self {
        Self {
            nyunatam_stara: min_level,
            adhikatam_stara: Some(max_level),
        }
    }

    /// Allow all logs
    pub fn sabhi() -> Self {
        Self {
            nyunatam_stara: LogStara::Padachihna,
            adhikatam_stara: None,
        }
    }

    /// Allow none
    pub fn koi_nahi() -> Self {
        Self {
            nyunatam_stara: LogStara::Ghataka,
            adhikatam_stara: Some(LogStara::Padachihna),
        }
    }

    /// Check if level passes filter
    pub fn manya(&self, level: LogStara) -> bool {
        if level < self.nyunatam_stara {
            return false;
        }

        if let Some(max) = self.adhikatam_stara {
            if level > max {
                return false;
            }
        }

        true
    }
}

impl Default for LogChanak {
    fn default() -> Self {
        Self::nava(LogStara::Suchana)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_ordering() {
        assert!(LogStara::Padachihna < LogStara::Nirikshana);
        assert!(LogStara::Nirikshana < LogStara::Suchana);
        assert!(LogStara::Suchana < LogStara::Chetavani);
        assert!(LogStara::Chetavani < LogStara::Truti);
        assert!(LogStara::Truti < LogStara::Ghataka);
    }

    #[test]
    fn test_filter() {
        let filter = LogChanak::nava(LogStara::Chetavani);

        assert!(!filter.manya(LogStara::Padachihna));
        assert!(!filter.manya(LogStara::Nirikshana));
        assert!(!filter.manya(LogStara::Suchana));
        assert!(filter.manya(LogStara::Chetavani));
        assert!(filter.manya(LogStara::Truti));
        assert!(filter.manya(LogStara::Ghataka));
    }

    #[test]
    fn test_range_filter() {
        let filter = LogChanak::sima(LogStara::Nirikshana, LogStara::Chetavani);

        assert!(!filter.manya(LogStara::Padachihna));
        assert!(filter.manya(LogStara::Nirikshana));
        assert!(filter.manya(LogStara::Suchana));
        assert!(filter.manya(LogStara::Chetavani));
        assert!(!filter.manya(LogStara::Truti));
    }
}
