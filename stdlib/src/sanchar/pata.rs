//! # Pata - URLs (पता)
//!
//! URL parsing and manipulation.
//!
//! > **"पता गन्तव्यस्य मार्गदर्शकः"**
//! > *"Address is the guide to destination"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::format;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use core::fmt;

// ============================================================================
// URL ERROR
// ============================================================================

/// URL parsing error
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PataDosha {
    /// Empty URL
    RiktaPata,
    /// Invalid scheme
    AmanyaYojana,
    /// Invalid host
    AmanyaMejban,
    /// Invalid port
    AmanyaBandar,
    /// Invalid path
    AmanyaPatha,
    /// Invalid query
    AmanyaPrashna,
    /// Invalid encoding
    AmanyaSanketan,
}

impl fmt::Display for PataDosha {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::RiktaPata => write!(f, "Empty URL"),
            Self::AmanyaYojana => write!(f, "Invalid scheme"),
            Self::AmanyaMejban => write!(f, "Invalid host"),
            Self::AmanyaBandar => write!(f, "Invalid port"),
            Self::AmanyaPatha => write!(f, "Invalid path"),
            Self::AmanyaPrashna => write!(f, "Invalid query"),
            Self::AmanyaSanketan => write!(f, "Invalid encoding"),
        }
    }
}

// ============================================================================
// URL STRUCT
// ============================================================================

/// URL components
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(feature = "alloc")]
pub struct Pata {
    /// Scheme (http, https, ftp, etc.)
    pub yojana: String,
    /// Username
    pub upayokta: Option<String>,
    /// Password
    pub gupta_shabda: Option<String>,
    /// Host
    pub mejban: String,
    /// Port
    pub bandar: Option<u16>,
    /// Path
    pub patha: String,
    /// Query string
    pub prashna: Option<String>,
    /// Fragment
    pub khand: Option<String>,
}

#[cfg(feature = "alloc")]
impl Pata {
    /// Parse URL from string
    pub fn vishleshan(url: &str) -> Result<Self, PataDosha> {
        if url.is_empty() {
            return Err(PataDosha::RiktaPata);
        }

        let mut remaining = url;

        // Extract scheme
        let yojana = if let Some(pos) = remaining.find("://") {
            let scheme = remaining[..pos].to_lowercase();
            remaining = &remaining[pos + 3..];
            scheme
        } else {
            return Err(PataDosha::AmanyaYojana);
        };

        // Extract fragment
        let khand = if let Some(pos) = remaining.find('#') {
            let fragment = remaining[pos + 1..].to_string();
            remaining = &remaining[..pos];
            Some(fragment)
        } else {
            None
        };

        // Extract query
        let prashna = if let Some(pos) = remaining.find('?') {
            let query = remaining[pos + 1..].to_string();
            remaining = &remaining[..pos];
            Some(query)
        } else {
            None
        };

        // Extract path
        let (authority, patha) = if let Some(pos) = remaining.find('/') {
            (&remaining[..pos], remaining[pos..].to_string())
        } else {
            (remaining, String::from("/"))
        };

        // Parse authority (user:pass@host:port)
        let (userinfo, hostport) = if let Some(pos) = authority.find('@') {
            (&authority[..pos], &authority[pos + 1..])
        } else {
            ("", authority)
        };

        // Parse userinfo
        let (upayokta, gupta_shabda) = if !userinfo.is_empty() {
            if let Some(pos) = userinfo.find(':') {
                (
                    Some(userinfo[..pos].to_string()),
                    Some(userinfo[pos + 1..].to_string()),
                )
            } else {
                (Some(userinfo.to_string()), None)
            }
        } else {
            (None, None)
        };

        // Parse host:port
        let (mejban, bandar) = if let Some(pos) = hostport.rfind(':') {
            let port_str = &hostport[pos + 1..];
            if let Ok(port) = port_str.parse::<u16>() {
                (hostport[..pos].to_string(), Some(port))
            } else {
                (hostport.to_string(), None)
            }
        } else {
            (hostport.to_string(), None)
        };

        if mejban.is_empty() {
            return Err(PataDosha::AmanyaMejban);
        }

        Ok(Self {
            yojana,
            upayokta,
            gupta_shabda,
            mejban,
            bandar,
            patha,
            prashna,
            khand,
        })
    }

    /// Convert to string
    pub fn sutra(&self) -> String {
        let mut result = format!("{}://", self.yojana);

        if let Some(ref user) = self.upayokta {
            result.push_str(user);
            if let Some(ref pass) = self.gupta_shabda {
                result.push(':');
                result.push_str(pass);
            }
            result.push('@');
        }

        result.push_str(&self.mejban);

        if let Some(port) = self.bandar {
            result.push(':');
            result.push_str(&port.to_string());
        }

        result.push_str(&self.patha);

        if let Some(ref query) = self.prashna {
            result.push('?');
            result.push_str(query);
        }

        if let Some(ref fragment) = self.khand {
            result.push('#');
            result.push_str(fragment);
        }

        result
    }

    /// Get origin (scheme://host:port)
    pub fn mulasthan(&self) -> String {
        let mut result = format!("{}://{}", self.yojana, self.mejban);
        if let Some(port) = self.bandar {
            result.push(':');
            result.push_str(&port.to_string());
        }
        result
    }

    /// Check if HTTPS
    pub fn surakshit_hai(&self) -> bool {
        self.yojana == "https"
    }

    /// Get default port for scheme
    pub fn manaka_bandar(&self) -> Option<u16> {
        match self.yojana.as_str() {
            "http" => Some(80),
            "https" => Some(443),
            "ftp" => Some(21),
            "ssh" => Some(22),
            "ws" => Some(80),
            "wss" => Some(443),
            _ => None,
        }
    }

    /// Get effective port
    pub fn prabhavi_bandar(&self) -> Option<u16> {
        self.bandar.or_else(|| self.manaka_bandar())
    }
}

#[cfg(feature = "alloc")]
impl fmt::Display for Pata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.sutra())
    }
}

// ============================================================================
// QUERY PARAMETERS
// ============================================================================

/// Query parameter pair
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg(feature = "alloc")]
pub struct PrashnaYugma {
    pub kunji: String,
    pub mana: String,
}

/// Parse query string into parameters
#[cfg(feature = "alloc")]
pub fn prashna_vishleshan(query: &str) -> Vec<PrashnaYugma> {
    if query.is_empty() {
        return Vec::new();
    }

    query
        .split('&')
        .filter(|s| !s.is_empty())
        .filter_map(|part| {
            let mut parts = part.splitn(2, '=');
            let kunji = parts.next()?;
            let mana = parts.next().unwrap_or("");
            Some(PrashnaYugma {
                kunji: prakshalan_anvayan(kunji),
                mana: prakshalan_anvayan(mana),
            })
        })
        .collect()
}

/// Build query string from parameters
#[cfg(feature = "alloc")]
pub fn prashna_nirmana(params: &[PrashnaYugma]) -> String {
    params
        .iter()
        .map(|p| format!("{}={}", url_sanketan(&p.kunji), url_sanketan(&p.mana)))
        .collect::<Vec<_>>()
        .join("&")
}

// ============================================================================
// URL ENCODING
// ============================================================================

/// URL encode string
#[cfg(feature = "alloc")]
pub fn url_sanketan(s: &str) -> String {
    let mut result = String::with_capacity(s.len());

    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            _ => {
                result.push('%');
                result.push_str(&format!("{:02X}", byte));
            }
        }
    }

    result
}

/// URL decode string
#[cfg(feature = "alloc")]
pub fn prakshalan_anvayan(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let bytes = s.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(byte) = u8::from_str_radix(&String::from_utf8_lossy(&bytes[i + 1..i + 3]), 16)
            {
                result.push(byte as char);
                i += 3;
                continue;
            }
        } else if bytes[i] == b'+' {
            result.push(' ');
            i += 1;
            continue;
        }

        result.push(bytes[i] as char);
        i += 1;
    }

    result
}

// ============================================================================
// URL BUILDER
// ============================================================================

/// URL builder
#[cfg(feature = "alloc")]
pub struct PataNirmata {
    yojana: String,
    mejban: String,
    bandar: Option<u16>,
    patha_khand: Vec<String>,
    prashna_yugma: Vec<PrashnaYugma>,
    khand: Option<String>,
}

#[cfg(feature = "alloc")]
impl PataNirmata {
    /// Create new builder
    pub fn nava(mejban: &str) -> Self {
        Self {
            yojana: String::from("https"),
            mejban: mejban.to_string(),
            bandar: None,
            patha_khand: Vec::new(),
            prashna_yugma: Vec::new(),
            khand: None,
        }
    }

    /// Set scheme
    pub fn yojana(mut self, yojana: &str) -> Self {
        self.yojana = yojana.to_string();
        self
    }

    /// Set port
    pub fn bandar(mut self, bandar: u16) -> Self {
        self.bandar = Some(bandar);
        self
    }

    /// Add path segment
    pub fn patha(mut self, khand: &str) -> Self {
        self.patha_khand.push(khand.to_string());
        self
    }

    /// Add query parameter
    pub fn prashna(mut self, kunji: &str, mana: &str) -> Self {
        self.prashna_yugma.push(PrashnaYugma {
            kunji: kunji.to_string(),
            mana: mana.to_string(),
        });
        self
    }

    /// Set fragment
    pub fn khand(mut self, khand: &str) -> Self {
        self.khand = Some(khand.to_string());
        self
    }

    /// Build URL
    pub fn nirmana(&self) -> Pata {
        let patha = if self.patha_khand.is_empty() {
            String::from("/")
        } else {
            format!("/{}", self.patha_khand.join("/"))
        };

        let prashna = if self.prashna_yugma.is_empty() {
            None
        } else {
            Some(prashna_nirmana(&self.prashna_yugma))
        };

        Pata {
            yojana: self.yojana.clone(),
            upayokta: None,
            gupta_shabda: None,
            mejban: self.mejban.clone(),
            bandar: self.bandar,
            patha,
            prashna,
            khand: self.khand.clone(),
        }
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
    fn test_url_parse() {
        let url = Pata::vishleshan("https://example.com:8080/path?key=value#section").unwrap();

        assert_eq!(url.yojana, "https");
        assert_eq!(url.mejban, "example.com");
        assert_eq!(url.bandar, Some(8080));
        assert_eq!(url.patha, "/path");
        assert_eq!(url.prashna, Some("key=value".to_string()));
        assert_eq!(url.khand, Some("section".to_string()));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_url_with_auth() {
        let url = Pata::vishleshan("https://user:pass@example.com/path").unwrap();

        assert_eq!(url.upayokta, Some("user".to_string()));
        assert_eq!(url.gupta_shabda, Some("pass".to_string()));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_url_encode() {
        assert_eq!(url_sanketan("hello world"), "hello%20world");
        assert_eq!(url_sanketan("foo=bar"), "foo%3Dbar");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_url_decode() {
        assert_eq!(prakshalan_anvayan("hello%20world"), "hello world");
        assert_eq!(prakshalan_anvayan("hello+world"), "hello world");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_query_parse() {
        let params = prashna_vishleshan("foo=bar&baz=qux");
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].kunji, "foo");
        assert_eq!(params[0].mana, "bar");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_url_builder() {
        let url = PataNirmata::nava("api.example.com")
            .yojana("https")
            .patha("v1")
            .patha("users")
            .prashna("page", "1")
            .prashna("limit", "10")
            .khand("top")
            .nirmana();

        assert_eq!(url.mejban, "api.example.com");
        assert_eq!(url.patha, "/v1/users");
        assert!(url.prashna.unwrap().contains("page=1"));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_default_port() {
        let url = Pata::vishleshan("https://example.com/").unwrap();
        assert_eq!(url.manaka_bandar(), Some(443));
        assert_eq!(url.prabhavi_bandar(), Some(443));
    }
}
