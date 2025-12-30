//! # Shirsha - Headers (शीर्ष)
//!
//! HTTP header utilities.
//!
//! > **"शीर्षं सन्देशस्य मुकुटम्"**
//! > *"Header is the crown of a message"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use core::fmt;

// ============================================================================
// COMMON HEADERS
// ============================================================================

/// Common HTTP header names
pub mod naam {
    // Request headers
    pub const SVIKRITI: &str = "Accept";
    pub const SVIKRITI_BHASHA: &str = "Accept-Language";
    pub const SVIKRITI_SANKETAN: &str = "Accept-Encoding";
    pub const PRAMANANA: &str = "Authorization";
    pub const SAMSTHAPAD: &str = "Cache-Control";
    pub const SAMYOJANA: &str = "Connection";
    pub const COOKIE: &str = "Cookie";
    pub const VISHAI_LAMBAI: &str = "Content-Length";
    pub const VISHAI_PRAKARA: &str = "Content-Type";
    pub const MEJBAN: &str = "Host";
    pub const MULASTHAN: &str = "Origin";
    pub const PUNARNIRDESHAK: &str = "Referer";
    pub const UPAYOKTA_ABHIKARTA: &str = "User-Agent";

    // Response headers
    pub const AGE: &str = "Age";
    pub const STHANA: &str = "Location";
    pub const COOKIE_STHAPITA: &str = "Set-Cookie";
    pub const SEVAK: &str = "Server";
    pub const ETAG: &str = "ETag";
    pub const SAMAPAT: &str = "Expires";
    pub const ANTIM_PARIVARTAN: &str = "Last-Modified";

    // CORS headers
    pub const CORS_MULASTHAN_ANUMATI: &str = "Access-Control-Allow-Origin";
    pub const CORS_VIDHI_ANUMATI: &str = "Access-Control-Allow-Methods";
    pub const CORS_SHIRSHA_ANUMATI: &str = "Access-Control-Allow-Headers";
    pub const CORS_PRAMANIK_ANUMATI: &str = "Access-Control-Allow-Credentials";

    // Security headers
    pub const X_FRAME_VIKALPA: &str = "X-Frame-Options";
    pub const X_VISHAI_PRAKARA: &str = "X-Content-Type-Options";
    pub const X_XSS_RAKSHA: &str = "X-XSS-Protection";
    pub const VISHAI_SURAKSHA_NITI: &str = "Content-Security-Policy";
    pub const KATHOR_PARIVAHAN_SURAKSHA: &str = "Strict-Transport-Security";
}

// ============================================================================
// HEADER VALUE
// ============================================================================

/// Single header entry
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(feature = "alloc")]
pub struct ShirshaYugma {
    /// Header name
    pub naam: String,
    /// Header value
    pub mana: String,
}

#[cfg(feature = "alloc")]
impl ShirshaYugma {
    /// Create new header
    pub fn nava(naam: &str, mana: &str) -> Self {
        Self {
            naam: naam.to_string(),
            mana: mana.to_string(),
        }
    }

    /// Check if name matches (case-insensitive)
    pub fn naam_milata(&self, naam: &str) -> bool {
        self.naam.eq_ignore_ascii_case(naam)
    }
}

// ============================================================================
// HEADER MAP
// ============================================================================

/// Collection of headers
#[derive(Debug, Clone, Default)]
#[cfg(feature = "alloc")]
pub struct ShirshaManachitra {
    headers: Vec<ShirshaYugma>,
}

#[cfg(feature = "alloc")]
impl ShirshaManachitra {
    /// Create empty header map
    pub fn nava() -> Self {
        Self {
            headers: Vec::new(),
        }
    }

    /// Add or update header
    pub fn jod(&mut self, naam: &str, mana: &str) {
        // Remove existing header with same name
        self.headers.retain(|h| !h.naam_milata(naam));
        self.headers.push(ShirshaYugma::nava(naam, mana));
    }

    /// Append header (allows multiple)
    pub fn jod_bahul(&mut self, naam: &str, mana: &str) {
        self.headers.push(ShirshaYugma::nava(naam, mana));
    }

    /// Get header value
    pub fn prapta(&self, naam: &str) -> Option<&str> {
        self.headers
            .iter()
            .find(|h| h.naam_milata(naam))
            .map(|h| h.mana.as_str())
    }

    /// Get all values for a header
    pub fn sabhi_prapta(&self, naam: &str) -> Vec<&str> {
        self.headers
            .iter()
            .filter(|h| h.naam_milata(naam))
            .map(|h| h.mana.as_str())
            .collect()
    }

    /// Remove header
    pub fn hatao(&mut self, naam: &str) -> bool {
        let len = self.headers.len();
        self.headers.retain(|h| !h.naam_milata(naam));
        len != self.headers.len()
    }

    /// Check if header exists
    pub fn dhaarana(&self, naam: &str) -> bool {
        self.headers.iter().any(|h| h.naam_milata(naam))
    }

    /// Get number of headers
    pub fn lambai(&self) -> usize {
        self.headers.len()
    }

    /// Check if empty
    pub fn rikta_hai(&self) -> bool {
        self.headers.is_empty()
    }

    /// Clear all headers
    pub fn shuddhikarana(&mut self) {
        self.headers.clear();
    }

    /// Iterate over headers
    pub fn iter(&self) -> impl Iterator<Item = &ShirshaYugma> {
        self.headers.iter()
    }

    /// Get content type
    pub fn vishai_prakara(&self) -> Option<&str> {
        self.prapta(naam::VISHAI_PRAKARA)
    }

    /// Get content length
    pub fn vishai_lambai(&self) -> Option<usize> {
        self.prapta(naam::VISHAI_LAMBAI)
            .and_then(|s| s.parse().ok())
    }
}

// ============================================================================
// CONTENT TYPE
// ============================================================================

/// Common content types
pub mod vishai_prakara {
    pub const JSON: &str = "application/json";
    pub const XML: &str = "application/xml";
    pub const HTML: &str = "text/html";
    pub const TEXT: &str = "text/plain";
    pub const CSS: &str = "text/css";
    pub const JS: &str = "application/javascript";
    pub const FORM: &str = "application/x-www-form-urlencoded";
    pub const MULTIPART: &str = "multipart/form-data";
    pub const OCTET: &str = "application/octet-stream";
    pub const PDF: &str = "application/pdf";
    pub const PNG: &str = "image/png";
    pub const JPEG: &str = "image/jpeg";
    pub const GIF: &str = "image/gif";
    pub const SVG: &str = "image/svg+xml";
    pub const WEBP: &str = "image/webp";
}

/// Parsed content type
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(feature = "alloc")]
pub struct VishaiPrakaraMana {
    /// Media type (e.g., "application/json")
    pub madhyam_prakara: String,
    /// Charset parameter
    pub varnasancha: Option<String>,
    /// Boundary parameter (for multipart)
    pub seema: Option<String>,
}

#[cfg(feature = "alloc")]
impl VishaiPrakaraMana {
    /// Parse content type header
    pub fn vishleshan(mana: &str) -> Self {
        let parts: Vec<&str> = mana.split(';').map(|s| s.trim()).collect();

        let madhyam_prakara = parts.first().map(|s| s.to_lowercase()).unwrap_or_default();

        let mut varnasancha = None;
        let mut seema = None;

        for part in parts.iter().skip(1) {
            if let Some(charset) = part.strip_prefix("charset=") {
                varnasancha = Some(charset.trim_matches('"').to_string());
            } else if let Some(boundary) = part.strip_prefix("boundary=") {
                seema = Some(boundary.trim_matches('"').to_string());
            }
        }

        Self {
            madhyam_prakara,
            varnasancha,
            seema,
        }
    }

    /// Check if JSON
    pub fn json_hai(&self) -> bool {
        self.madhyam_prakara == "application/json" || self.madhyam_prakara.ends_with("+json")
    }

    /// Check if XML
    pub fn xml_hai(&self) -> bool {
        self.madhyam_prakara == "application/xml"
            || self.madhyam_prakara == "text/xml"
            || self.madhyam_prakara.ends_with("+xml")
    }

    /// Check if text
    pub fn text_hai(&self) -> bool {
        self.madhyam_prakara.starts_with("text/")
    }

    /// Check if image
    pub fn chitra_hai(&self) -> bool {
        self.madhyam_prakara.starts_with("image/")
    }
}

// ============================================================================
// CACHE CONTROL
// ============================================================================

/// Cache control directives
#[derive(Debug, Clone, Default)]
#[cfg(feature = "alloc")]
pub struct SamsthapadNirdesh {
    /// Max age in seconds
    pub adhiktam_aayu: Option<u64>,
    /// No cache
    pub samsthapad_nahi: bool,
    /// No store
    pub bhandan_nahi: bool,
    /// Private
    pub niji: bool,
    /// Public
    pub sarvajanik: bool,
    /// Must revalidate
    pub punamanyata_anivarya: bool,
}

#[cfg(feature = "alloc")]
impl SamsthapadNirdesh {
    /// Parse cache control header
    pub fn vishleshan(mana: &str) -> Self {
        let mut result = Self::default();

        for part in mana.split(',').map(|s| s.trim()) {
            if part.eq_ignore_ascii_case("no-cache") {
                result.samsthapad_nahi = true;
            } else if part.eq_ignore_ascii_case("no-store") {
                result.bhandan_nahi = true;
            } else if part.eq_ignore_ascii_case("private") {
                result.niji = true;
            } else if part.eq_ignore_ascii_case("public") {
                result.sarvajanik = true;
            } else if part.eq_ignore_ascii_case("must-revalidate") {
                result.punamanyata_anivarya = true;
            } else if let Some(age) = part.strip_prefix("max-age=") {
                result.adhiktam_aayu = age.parse().ok();
            }
        }

        result
    }

    /// Build cache control header value
    pub fn sutra(&self) -> String {
        let mut parts = Vec::new();

        if self.samsthapad_nahi {
            parts.push("no-cache".to_string());
        }
        if self.bhandan_nahi {
            parts.push("no-store".to_string());
        }
        if self.niji {
            parts.push("private".to_string());
        }
        if self.sarvajanik {
            parts.push("public".to_string());
        }
        if self.punamanyata_anivarya {
            parts.push("must-revalidate".to_string());
        }
        if let Some(age) = self.adhiktam_aayu {
            parts.push(alloc::format!("max-age={}", age));
        }

        parts.join(", ")
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
    fn test_header_map() {
        let mut headers = ShirshaManachitra::nava();
        headers.jod("Content-Type", "application/json");
        headers.jod("Accept", "text/html");

        assert_eq!(headers.prapta("content-type"), Some("application/json"));
        assert!(headers.dhaarana("accept"));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_header_overwrite() {
        let mut headers = ShirshaManachitra::nava();
        headers.jod("X-Custom", "value1");
        headers.jod("X-Custom", "value2");

        assert_eq!(headers.prapta("X-Custom"), Some("value2"));
        assert_eq!(headers.lambai(), 1);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_content_type_parse() {
        let ct = VishaiPrakaraMana::vishleshan("application/json; charset=utf-8");

        assert_eq!(ct.madhyam_prakara, "application/json");
        assert_eq!(ct.varnasancha, Some("utf-8".to_string()));
        assert!(ct.json_hai());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_cache_control() {
        let cc = SamsthapadNirdesh::vishleshan("max-age=3600, public");

        assert_eq!(cc.adhiktam_aayu, Some(3600));
        assert!(cc.sarvajanik);
        assert!(!cc.niji);
    }
}
