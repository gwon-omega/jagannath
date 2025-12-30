//! # UUID - Universally Unique Identifiers (यूयूआईडी)
//!
//! UUID generation and parsing.
//!
//! > **"विश्वव्यापी अद्वितीय पहचान"**
//! > *"Universally unique identity"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::format;

// ============================================================================
// UUID STRUCT
// ============================================================================

/// UUID (128-bit identifier)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Uuid {
    bytes: [u8; 16],
}

impl Uuid {
    /// Create UUID from bytes
    pub const fn nava_bait(bytes: [u8; 16]) -> Self {
        Self { bytes }
    }
    
    /// Create nil UUID (all zeros)
    pub const fn shunya() -> Self {
        Self { bytes: [0; 16] }
    }
    
    /// Create max UUID (all ones)
    pub const fn adhikatam() -> Self {
        Self { bytes: [0xFF; 16] }
    }
    
    /// Create UUID v4 (random) using simple RNG
    #[cfg(feature = "alloc")]
    pub fn nava_v4(rng: &mut impl FnMut() -> u32) -> Self {
        let mut bytes = [0u8; 16];
        
        // Fill with random bytes
        for i in 0..4 {
            let val = rng();
            bytes[i * 4] = (val >> 24) as u8;
            bytes[i * 4 + 1] = (val >> 16) as u8;
            bytes[i * 4 + 2] = (val >> 8) as u8;
            bytes[i * 4 + 3] = val as u8;
        }
        
        // Set version (4) and variant (RFC 4122)
        bytes[6] = (bytes[6] & 0x0f) | 0x40;  // Version 4
        bytes[8] = (bytes[8] & 0x3f) | 0x80;  // Variant RFC 4122
        
        Self { bytes }
    }
    
    /// Get bytes
    pub fn bait(&self) -> [u8; 16] {
        self.bytes
    }
    
    /// Get version
    pub fn sansakaran(&self) -> u8 {
        (self.bytes[6] >> 4) & 0x0f
    }
    
    /// Get variant
    pub fn prakar(&self) -> UuidPrakar {
        match self.bytes[8] >> 4 {
            0..=7 => UuidPrakar::Ncs,
            8..=11 => UuidPrakar::Rfc4122,
            12..=13 => UuidPrakar::Microsoft,
            _ => UuidPrakar::Bhavishya,
        }
    }
    
    /// Is nil?
    pub fn shunya_hai(&self) -> bool {
        self.bytes.iter().all(|&b| b == 0)
    }
    
    /// Is max?
    pub fn adhikatam_hai(&self) -> bool {
        self.bytes.iter().all(|&b| b == 0xFF)
    }
    
    /// Format as string
    #[cfg(feature = "alloc")]
    pub fn sutra(&self) -> String {
        format!(
            "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3],
            self.bytes[4], self.bytes[5],
            self.bytes[6], self.bytes[7],
            self.bytes[8], self.bytes[9],
            self.bytes[10], self.bytes[11], self.bytes[12], self.bytes[13], self.bytes[14], self.bytes[15]
        )
    }
    
    /// Format as uppercase string
    #[cfg(feature = "alloc")]
    pub fn sutra_brihat(&self) -> String {
        self.sutra().to_uppercase()
    }
    
    /// Format without hyphens
    #[cfg(feature = "alloc")]
    pub fn sutra_sankshepa(&self) -> String {
        format!(
            "{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            self.bytes[0], self.bytes[1], self.bytes[2], self.bytes[3],
            self.bytes[4], self.bytes[5], self.bytes[6], self.bytes[7],
            self.bytes[8], self.bytes[9], self.bytes[10], self.bytes[11],
            self.bytes[12], self.bytes[13], self.bytes[14], self.bytes[15]
        )
    }
    
    /// Format as URN
    #[cfg(feature = "alloc")]
    pub fn urn(&self) -> String {
        format!("urn:uuid:{}", self.sutra())
    }
    
    /// Parse from string
    #[cfg(feature = "alloc")]
    pub fn vishleshan(s: &str) -> Result<Self, UuidDosha> {
        let clean: String = s.chars()
            .filter(|c| c.is_ascii_hexdigit())
            .collect();
        
        if clean.len() != 32 {
            return Err(UuidDosha::AmanyadLambai);
        }
        
        let mut bytes = [0u8; 16];
        for i in 0..16 {
            bytes[i] = u8::from_str_radix(&clean[i * 2..i * 2 + 2], 16)
                .map_err(|_| UuidDosha::AmanyadHex)?;
        }
        
        Ok(Self { bytes })
    }
}

#[cfg(feature = "alloc")]
impl core::fmt::Display for Uuid {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.sutra())
    }
}

/// UUID variant
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UuidPrakar {
    /// NCS backward compatibility
    Ncs,
    /// RFC 4122 / DCE 1.1
    Rfc4122,
    /// Microsoft GUID
    Microsoft,
    /// Reserved for future
    Bhavishya,
}

// ============================================================================
// UUID v5 (SHA-1 based)
// ============================================================================

/// Well-known namespace UUIDs
#[cfg(feature = "alloc")]
pub mod namasthan {
    use super::Uuid;
    
    /// DNS namespace
    pub const DNS: Uuid = Uuid::nava_bait([
        0x6b, 0xa7, 0xb8, 0x10, 0x9d, 0xad, 0x11, 0xd1,
        0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30, 0xc8
    ]);
    
    /// URL namespace
    pub const URL: Uuid = Uuid::nava_bait([
        0x6b, 0xa7, 0xb8, 0x11, 0x9d, 0xad, 0x11, 0xd1,
        0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30, 0xc8
    ]);
    
    /// OID namespace
    pub const OID: Uuid = Uuid::nava_bait([
        0x6b, 0xa7, 0xb8, 0x12, 0x9d, 0xad, 0x11, 0xd1,
        0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30, 0xc8
    ]);
    
    /// X.500 namespace
    pub const X500: Uuid = Uuid::nava_bait([
        0x6b, 0xa7, 0xb8, 0x14, 0x9d, 0xad, 0x11, 0xd1,
        0x80, 0xb4, 0x00, 0xc0, 0x4f, 0xd4, 0x30, 0xc8
    ]);
}

// ============================================================================
// ERROR TYPE
// ============================================================================

/// UUID error
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub enum UuidDosha {
    /// Invalid length
    AmanyadLambai,
    /// Invalid hex character
    AmanyadHex,
    /// Invalid format
    AmanyadSvarupa,
}

#[cfg(feature = "alloc")]
impl core::fmt::Display for UuidDosha {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            UuidDosha::AmanyadLambai => write!(f, "Invalid UUID length"),
            UuidDosha::AmanyadHex => write!(f, "Invalid hex character"),
            UuidDosha::AmanyadSvarupa => write!(f, "Invalid UUID format"),
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
    fn test_nil() {
        let uuid = Uuid::shunya();
        assert!(uuid.shunya_hai());
        assert_eq!(uuid.sutra(), "00000000-0000-0000-0000-000000000000");
    }
    
    #[test]
    fn test_max() {
        let uuid = Uuid::adhikatam();
        assert!(uuid.adhikatam_hai());
        assert_eq!(uuid.sutra(), "ffffffff-ffff-ffff-ffff-ffffffffffff");
    }
    
    #[test]
    #[cfg(feature = "alloc")]
    fn test_parse() {
        let uuid = Uuid::vishleshan("550e8400-e29b-41d4-a716-446655440000").unwrap();
        assert_eq!(uuid.sutra(), "550e8400-e29b-41d4-a716-446655440000");
    }
    
    #[test]
    #[cfg(feature = "alloc")]
    fn test_parse_no_hyphens() {
        let uuid = Uuid::vishleshan("550e8400e29b41d4a716446655440000").unwrap();
        assert_eq!(uuid.sutra(), "550e8400-e29b-41d4-a716-446655440000");
    }
    
    #[test]
    #[cfg(feature = "alloc")]
    fn test_v4() {
        let mut counter = 0u32;
        let mut rng = || {
            counter = counter.wrapping_mul(1103515245).wrapping_add(12345);
            counter
        };
        
        let uuid = Uuid::nava_v4(&mut rng);
        assert_eq!(uuid.sansakaran(), 4);
        assert_eq!(uuid.prakar(), UuidPrakar::Rfc4122);
    }
    
    #[test]
    #[cfg(feature = "alloc")]
    fn test_urn() {
        let uuid = Uuid::vishleshan("550e8400-e29b-41d4-a716-446655440000").unwrap();
        assert_eq!(uuid.urn(), "urn:uuid:550e8400-e29b-41d4-a716-446655440000");
    }
}
