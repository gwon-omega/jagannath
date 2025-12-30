//! # Sandesh - Messages (सन्देश)
//!
//! HTTP request and response types.
//!
//! > **"सन्देशः संवादस्य वाहकः"**
//! > *"Message is the carrier of communication"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use super::shirsha::ShirshaManachitra;
use super::sthiti::{HttpSanskarana, HttpSthiti, HttpVidhi};

// ============================================================================
// HTTP REQUEST
// ============================================================================

/// HTTP request
#[derive(Debug, Clone)]
#[cfg(feature = "alloc")]
pub struct HttpPrarthana {
    /// HTTP method
    pub vidhi: HttpVidhi,
    /// Request URI
    pub pata: String,
    /// HTTP version
    pub sanskarana: HttpSanskarana,
    /// Headers
    pub shirsha: ShirshaManachitra,
    /// Body
    pub sharira: Vec<u8>,
}

#[cfg(feature = "alloc")]
impl HttpPrarthana {
    /// Create new request
    pub fn nava(vidhi: HttpVidhi, pata: &str) -> Self {
        Self {
            vidhi,
            pata: pata.to_string(),
            sanskarana: HttpSanskarana::EkEk,
            shirsha: ShirshaManachitra::nava(),
            sharira: Vec::new(),
        }
    }

    /// Create GET request
    pub fn prapta(pata: &str) -> Self {
        Self::nava(HttpVidhi::Prapta, pata)
    }

    /// Create POST request
    pub fn pratishthit(pata: &str) -> Self {
        Self::nava(HttpVidhi::Pratishthit, pata)
    }

    /// Create PUT request
    pub fn sthapita(pata: &str) -> Self {
        Self::nava(HttpVidhi::Sthapita, pata)
    }

    /// Create DELETE request
    pub fn nashta(pata: &str) -> Self {
        Self::nava(HttpVidhi::Nashta, pata)
    }

    /// Set HTTP version
    pub fn sanskarana_sthapita(mut self, sanskarana: HttpSanskarana) -> Self {
        self.sanskarana = sanskarana;
        self
    }

    /// Add header
    pub fn shirsha_jod(mut self, naam: &str, mana: &str) -> Self {
        self.shirsha.jod(naam, mana);
        self
    }

    /// Set body bytes
    pub fn sharira_sthapita(mut self, data: Vec<u8>) -> Self {
        self.sharira = data;
        self
    }

    /// Set body string
    pub fn sharira_sutra(mut self, data: &str) -> Self {
        self.sharira = data.as_bytes().to_vec();
        self
    }

    /// Set JSON body
    pub fn json_sharira(mut self, json: &str) -> Self {
        self.shirsha.jod("Content-Type", "application/json");
        self.sharira = json.as_bytes().to_vec();
        self
    }

    /// Get body as string
    pub fn sharira_prapta(&self) -> Option<String> {
        String::from_utf8(self.sharira.clone()).ok()
    }

    /// Check if has body
    pub fn sharira_hai(&self) -> bool {
        !self.sharira.is_empty()
    }
}

// ============================================================================
// HTTP RESPONSE
// ============================================================================

/// HTTP response
#[derive(Debug, Clone)]
#[cfg(feature = "alloc")]
pub struct HttpUttar {
    /// HTTP version
    pub sanskarana: HttpSanskarana,
    /// Status code
    pub sthiti: HttpSthiti,
    /// Headers
    pub shirsha: ShirshaManachitra,
    /// Body
    pub sharira: Vec<u8>,
}

#[cfg(feature = "alloc")]
impl HttpUttar {
    /// Create new response
    pub fn nava(sthiti: HttpSthiti) -> Self {
        Self {
            sanskarana: HttpSanskarana::EkEk,
            sthiti,
            shirsha: ShirshaManachitra::nava(),
            sharira: Vec::new(),
        }
    }

    /// Create OK response
    pub fn safala() -> Self {
        Self::nava(HttpSthiti::SAFALA)
    }

    /// Create Not Found response
    pub fn nahi_mila() -> Self {
        Self::nava(HttpSthiti::NAHI_MILA)
    }

    /// Create Internal Server Error response
    pub fn aantarik_dosha() -> Self {
        Self::nava(HttpSthiti::AANTARIK_DOSHA)
    }

    /// Add header
    pub fn shirsha_jod(mut self, naam: &str, mana: &str) -> Self {
        self.shirsha.jod(naam, mana);
        self
    }

    /// Set body bytes
    pub fn sharira_sthapita(mut self, data: Vec<u8>) -> Self {
        self.sharira = data;
        self
    }

    /// Set body string
    pub fn sharira_sutra(mut self, data: &str) -> Self {
        self.sharira = data.as_bytes().to_vec();
        self
    }

    /// Set JSON response
    pub fn json(mut self, json: &str) -> Self {
        self.shirsha.jod("Content-Type", "application/json");
        self.sharira = json.as_bytes().to_vec();
        self
    }

    /// Set HTML response
    pub fn html(mut self, html: &str) -> Self {
        self.shirsha.jod("Content-Type", "text/html; charset=utf-8");
        self.sharira = html.as_bytes().to_vec();
        self
    }

    /// Set plain text response
    pub fn text(mut self, text: &str) -> Self {
        self.shirsha
            .jod("Content-Type", "text/plain; charset=utf-8");
        self.sharira = text.as_bytes().to_vec();
        self
    }

    /// Get body as string
    pub fn sharira_prapta(&self) -> Option<String> {
        String::from_utf8(self.sharira.clone()).ok()
    }

    /// Check if successful
    pub fn safala_hai(&self) -> bool {
        self.sthiti.safala_hai()
    }

    /// Check if error
    pub fn dosha_hai(&self) -> bool {
        self.sthiti.dosha_hai()
    }

    /// Get content type
    pub fn vishai_prakara(&self) -> Option<&str> {
        self.shirsha.prapta("Content-Type")
    }

    /// Check if JSON response
    pub fn json_hai(&self) -> bool {
        self.vishai_prakara()
            .map(|ct| ct.contains("application/json"))
            .unwrap_or(false)
    }
}

// ============================================================================
// REQUEST BUILDER
// ============================================================================

/// Request builder
#[cfg(feature = "alloc")]
pub struct PrarthanaNirmata {
    vidhi: HttpVidhi,
    pata: String,
    shirsha: ShirshaManachitra,
    sharira: Vec<u8>,
    samay_sima: Option<u64>,
}

#[cfg(feature = "alloc")]
impl PrarthanaNirmata {
    /// Create builder for URL
    pub fn nava(pata: &str) -> Self {
        Self {
            vidhi: HttpVidhi::Prapta,
            pata: pata.to_string(),
            shirsha: ShirshaManachitra::nava(),
            sharira: Vec::new(),
            samay_sima: None,
        }
    }

    /// Set GET method
    pub fn prapta(mut self) -> Self {
        self.vidhi = HttpVidhi::Prapta;
        self
    }

    /// Set POST method
    pub fn pratishthit(mut self) -> Self {
        self.vidhi = HttpVidhi::Pratishthit;
        self
    }

    /// Set PUT method
    pub fn sthapita(mut self) -> Self {
        self.vidhi = HttpVidhi::Sthapita;
        self
    }

    /// Set DELETE method
    pub fn nashta(mut self) -> Self {
        self.vidhi = HttpVidhi::Nashta;
        self
    }

    /// Set PATCH method
    pub fn samshodhit(mut self) -> Self {
        self.vidhi = HttpVidhi::Samshodhit;
        self
    }

    /// Add header
    pub fn shirsha(mut self, naam: &str, mana: &str) -> Self {
        self.shirsha.jod(naam, mana);
        self
    }

    /// Set bearer token
    pub fn vaahak_prakrit(mut self, token: &str) -> Self {
        use alloc::format;
        self.shirsha
            .jod("Authorization", &format!("Bearer {}", token));
        self
    }

    /// Set basic auth
    pub fn mool_pramanana(mut self, upayokta: &str, gupta_shabda: &str) -> Self {
        use alloc::format;
        let credentials = format!("{}:{}", upayokta, gupta_shabda);
        let encoded = base64_sanketan(credentials.as_bytes());
        self.shirsha
            .jod("Authorization", &format!("Basic {}", encoded));
        self
    }

    /// Set body
    pub fn sharira(mut self, data: Vec<u8>) -> Self {
        self.sharira = data;
        self
    }

    /// Set JSON body
    pub fn json(mut self, json: &str) -> Self {
        self.shirsha.jod("Content-Type", "application/json");
        self.sharira = json.as_bytes().to_vec();
        self
    }

    /// Set form body
    pub fn form(mut self, data: &str) -> Self {
        self.shirsha
            .jod("Content-Type", "application/x-www-form-urlencoded");
        self.sharira = data.as_bytes().to_vec();
        self
    }

    /// Set timeout in milliseconds
    pub fn samay_sima(mut self, ms: u64) -> Self {
        self.samay_sima = Some(ms);
        self
    }

    /// Build request
    pub fn nirmana(self) -> HttpPrarthana {
        HttpPrarthana {
            vidhi: self.vidhi,
            pata: self.pata,
            sanskarana: HttpSanskarana::EkEk,
            shirsha: self.shirsha,
            sharira: self.sharira,
        }
    }
}

// ============================================================================
// HELPERS
// ============================================================================

/// Simple base64 encoding
#[cfg(feature = "alloc")]
fn base64_sanketan(data: &[u8]) -> String {
    const TABLE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut result = String::new();
    let mut i = 0;

    while i < data.len() {
        let b0 = data[i] as usize;
        let b1 = data.get(i + 1).copied().unwrap_or(0) as usize;
        let b2 = data.get(i + 2).copied().unwrap_or(0) as usize;

        result.push(TABLE[b0 >> 2] as char);
        result.push(TABLE[((b0 & 3) << 4) | (b1 >> 4)] as char);

        if i + 1 < data.len() {
            result.push(TABLE[((b1 & 15) << 2) | (b2 >> 6)] as char);
        } else {
            result.push('=');
        }

        if i + 2 < data.len() {
            result.push(TABLE[b2 & 63] as char);
        } else {
            result.push('=');
        }

        i += 3;
    }

    result
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_request_creation() {
        let req = HttpPrarthana::prapta("/api/users").shirsha_jod("Accept", "application/json");

        assert_eq!(req.vidhi, HttpVidhi::Prapta);
        assert_eq!(req.pata, "/api/users");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_response_creation() {
        let resp = HttpUttar::safala().json(r#"{"status": "ok"}"#);

        assert!(resp.safala_hai());
        assert!(resp.json_hai());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_request_builder() {
        let req = PrarthanaNirmata::nava("https://api.example.com/users")
            .pratishthit()
            .shirsha("Accept", "application/json")
            .json(r#"{"name": "test"}"#)
            .nirmana();

        assert_eq!(req.vidhi, HttpVidhi::Pratishthit);
        assert!(req.sharira_hai());
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_base64() {
        assert_eq!(base64_sanketan(b"Hello"), "SGVsbG8=");
        assert_eq!(base64_sanketan(b"Hi"), "SGk=");
    }
}
