//! # Sthiti - HTTP Status (स्थिति)
//!
//! HTTP status codes and categories.
//!
//! > **"स्थिति उत्तरस्य संकेतः"**
//! > *"Status is the signal of response"*

use core::fmt;

// ============================================================================
// STATUS CODE
// ============================================================================

/// HTTP status code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HttpSthiti(pub u16);

impl HttpSthiti {
    // 1xx Informational
    pub const JAARI: Self = Self(100); // Continue
    pub const VISHAYA_BADALNA: Self = Self(101); // Switching Protocols
    pub const PRAKRIYA: Self = Self(102); // Processing

    // 2xx Success
    pub const SAFALA: Self = Self(200); // OK
    pub const NIRMIT: Self = Self(201); // Created
    pub const SVIKRIYA: Self = Self(202); // Accepted
    pub const AMAANIK: Self = Self(203); // Non-Authoritative
    pub const RIKTA_VISHAI: Self = Self(204); // No Content
    pub const PUNASTHAPIT: Self = Self(205); // Reset Content
    pub const ANSHIK: Self = Self(206); // Partial Content

    // 3xx Redirection
    pub const BAHUL_VIKALPA: Self = Self(300); // Multiple Choices
    pub const STHAYEE_STHANAN: Self = Self(301); // Moved Permanently
    pub const PRAPT: Self = Self(302); // Found
    pub const ANYA_DEKHO: Self = Self(303); // See Other
    pub const APARIVARTIT: Self = Self(304); // Not Modified
    pub const AASTHAYEE_STHANAN: Self = Self(307); // Temporary Redirect
    pub const STHAYEE_PUNARNIRDESH: Self = Self(308); // Permanent Redirect

    // 4xx Client Errors
    pub const KHOTA_PRARTHANA: Self = Self(400); // Bad Request
    pub const ANADHIKRIT: Self = Self(401); // Unauthorized
    pub const BHUGTAN_APEKSHIT: Self = Self(402); // Payment Required
    pub const NISHEDH: Self = Self(403); // Forbidden
    pub const NAHI_MILA: Self = Self(404); // Not Found
    pub const VIDHI_AMANYA: Self = Self(405); // Method Not Allowed
    pub const ASVIKARYA: Self = Self(406); // Not Acceptable
    pub const SAMAY_SAMAPAT: Self = Self(408); // Request Timeout
    pub const SANGHARSH: Self = Self(409); // Conflict
    pub const CHALA_GAYA: Self = Self(410); // Gone
    pub const LAMBAI_APEKSHIT: Self = Self(411); // Length Required
    pub const PURVASHARAT_ASAFAL: Self = Self(412); // Precondition Failed
    pub const VISHAI_BADA: Self = Self(413); // Payload Too Large
    pub const URI_LAMBA: Self = Self(414); // URI Too Long
    pub const AMANYA_MADHYAM: Self = Self(415); // Unsupported Media Type
    pub const BAHUT_PRARTHANA: Self = Self(429); // Too Many Requests

    // 5xx Server Errors
    pub const AANTARIK_DOSHA: Self = Self(500); // Internal Server Error
    pub const AKARYANVIT: Self = Self(501); // Not Implemented
    pub const KHOTA_GATEWAY: Self = Self(502); // Bad Gateway
    pub const SEVA_ANUPALABDHA: Self = Self(503); // Service Unavailable
    pub const GATEWAY_SAMAY: Self = Self(504); // Gateway Timeout
    pub const HTTP_AMANYA: Self = Self(505); // HTTP Version Not Supported

    /// Create from code
    pub const fn nava(code: u16) -> Self {
        Self(code)
    }

    /// Get status code value
    pub const fn mana(&self) -> u16 {
        self.0
    }

    /// Check if informational (1xx)
    pub const fn suchana_hai(&self) -> bool {
        self.0 >= 100 && self.0 < 200
    }

    /// Check if success (2xx)
    pub const fn safala_hai(&self) -> bool {
        self.0 >= 200 && self.0 < 300
    }

    /// Check if redirection (3xx)
    pub const fn punarnirdesh_hai(&self) -> bool {
        self.0 >= 300 && self.0 < 400
    }

    /// Check if client error (4xx)
    pub const fn grahak_dosha_hai(&self) -> bool {
        self.0 >= 400 && self.0 < 500
    }

    /// Check if server error (5xx)
    pub const fn sevak_dosha_hai(&self) -> bool {
        self.0 >= 500 && self.0 < 600
    }

    /// Check if error (4xx or 5xx)
    pub const fn dosha_hai(&self) -> bool {
        self.0 >= 400
    }

    /// Get reason phrase
    pub const fn kaaran(&self) -> &'static str {
        match self.0 {
            100 => "Continue",
            101 => "Switching Protocols",
            102 => "Processing",

            200 => "OK",
            201 => "Created",
            202 => "Accepted",
            203 => "Non-Authoritative Information",
            204 => "No Content",
            205 => "Reset Content",
            206 => "Partial Content",

            300 => "Multiple Choices",
            301 => "Moved Permanently",
            302 => "Found",
            303 => "See Other",
            304 => "Not Modified",
            307 => "Temporary Redirect",
            308 => "Permanent Redirect",

            400 => "Bad Request",
            401 => "Unauthorized",
            402 => "Payment Required",
            403 => "Forbidden",
            404 => "Not Found",
            405 => "Method Not Allowed",
            406 => "Not Acceptable",
            408 => "Request Timeout",
            409 => "Conflict",
            410 => "Gone",
            411 => "Length Required",
            412 => "Precondition Failed",
            413 => "Payload Too Large",
            414 => "URI Too Long",
            415 => "Unsupported Media Type",
            429 => "Too Many Requests",

            500 => "Internal Server Error",
            501 => "Not Implemented",
            502 => "Bad Gateway",
            503 => "Service Unavailable",
            504 => "Gateway Timeout",
            505 => "HTTP Version Not Supported",

            _ => "Unknown Status",
        }
    }
}

impl fmt::Display for HttpSthiti {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.0, self.kaaran())
    }
}

impl From<u16> for HttpSthiti {
    fn from(code: u16) -> Self {
        Self(code)
    }
}

// ============================================================================
// HTTP METHOD
// ============================================================================

/// HTTP request method
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpVidhi {
    /// GET
    Prapta,
    /// POST
    Pratishthit,
    /// PUT
    Sthapita,
    /// DELETE
    Nashta,
    /// PATCH
    Samshodhit,
    /// HEAD
    Shirsha,
    /// OPTIONS
    Vikalpa,
    /// CONNECT
    Samyojana,
    /// TRACE
    Anusaran,
}

impl HttpVidhi {
    /// Convert to string
    pub const fn sutra(&self) -> &'static str {
        match self {
            Self::Prapta => "GET",
            Self::Pratishthit => "POST",
            Self::Sthapita => "PUT",
            Self::Nashta => "DELETE",
            Self::Samshodhit => "PATCH",
            Self::Shirsha => "HEAD",
            Self::Vikalpa => "OPTIONS",
            Self::Samyojana => "CONNECT",
            Self::Anusaran => "TRACE",
        }
    }

    /// Parse from string
    pub fn vishleshan(s: &str) -> Option<Self> {
        match s.to_uppercase().as_str() {
            "GET" => Some(Self::Prapta),
            "POST" => Some(Self::Pratishthit),
            "PUT" => Some(Self::Sthapita),
            "DELETE" => Some(Self::Nashta),
            "PATCH" => Some(Self::Samshodhit),
            "HEAD" => Some(Self::Shirsha),
            "OPTIONS" => Some(Self::Vikalpa),
            "CONNECT" => Some(Self::Samyojana),
            "TRACE" => Some(Self::Anusaran),
            _ => None,
        }
    }

    /// Check if safe (doesn't modify)
    pub const fn surakshit_hai(&self) -> bool {
        matches!(
            self,
            Self::Prapta | Self::Shirsha | Self::Vikalpa | Self::Anusaran
        )
    }

    /// Check if idempotent
    pub const fn avyavartaniya_hai(&self) -> bool {
        matches!(
            self,
            Self::Prapta
                | Self::Sthapita
                | Self::Nashta
                | Self::Shirsha
                | Self::Vikalpa
                | Self::Anusaran
        )
    }

    /// Check if can have body
    pub const fn sharira_yogya_hai(&self) -> bool {
        matches!(self, Self::Pratishthit | Self::Sthapita | Self::Samshodhit)
    }
}

impl fmt::Display for HttpVidhi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.sutra())
    }
}

// ============================================================================
// HTTP VERSION
// ============================================================================

/// HTTP protocol version
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpSanskarana {
    /// HTTP/1.0
    Ek,
    /// HTTP/1.1
    EkEk,
    /// HTTP/2
    Dvi,
    /// HTTP/3
    Tri,
}

impl HttpSanskarana {
    /// Convert to string
    pub const fn sutra(&self) -> &'static str {
        match self {
            Self::Ek => "HTTP/1.0",
            Self::EkEk => "HTTP/1.1",
            Self::Dvi => "HTTP/2",
            Self::Tri => "HTTP/3",
        }
    }

    /// Check if supports keep-alive by default
    pub const fn sthir_samyojana(&self) -> bool {
        !matches!(self, Self::Ek)
    }

    /// Check if supports multiplexing
    pub const fn bahugunitakaran(&self) -> bool {
        matches!(self, Self::Dvi | Self::Tri)
    }
}

impl fmt::Display for HttpSanskarana {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.sutra())
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_categories() {
        assert!(HttpSthiti::JAARI.suchana_hai());
        assert!(HttpSthiti::SAFALA.safala_hai());
        assert!(HttpSthiti::STHAYEE_STHANAN.punarnirdesh_hai());
        assert!(HttpSthiti::NAHI_MILA.grahak_dosha_hai());
        assert!(HttpSthiti::AANTARIK_DOSHA.sevak_dosha_hai());
    }

    #[test]
    fn test_status_display() {
        assert_eq!(HttpSthiti::SAFALA.to_string(), "200 OK");
        assert_eq!(HttpSthiti::NAHI_MILA.to_string(), "404 Not Found");
    }

    #[test]
    fn test_method_parse() {
        assert_eq!(HttpVidhi::vishleshan("GET"), Some(HttpVidhi::Prapta));
        assert_eq!(HttpVidhi::vishleshan("post"), Some(HttpVidhi::Pratishthit));
    }

    #[test]
    fn test_method_properties() {
        assert!(HttpVidhi::Prapta.surakshit_hai());
        assert!(HttpVidhi::Prapta.avyavartaniya_hai());
        assert!(!HttpVidhi::Pratishthit.avyavartaniya_hai());
        assert!(HttpVidhi::Pratishthit.sharira_yogya_hai());
    }

    #[test]
    fn test_version() {
        assert!(!HttpSanskarana::Ek.sthir_samyojana());
        assert!(HttpSanskarana::EkEk.sthir_samyojana());
        assert!(HttpSanskarana::Dvi.bahugunitakaran());
    }
}
