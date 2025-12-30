//! # Dvyaṅka - Binary Serialization (द्व्यंक)
//!
//! Binary data encoding and decoding.
//!
//! > **"द्वे अंके एव पर्याप्ते"**
//! > *"Two digits are sufficient"*
//!
//! ## Etymology
//! द्व्यंक (dvyaṅka) = binary (two + digit)

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// BYTE ORDER
// ============================================================================

/// Byte order for multi-byte values (बाइट क्रम)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaitKrama {
    /// Little-endian (लघु अंत)
    LaghuAnta,
    /// Big-endian (वृहत् अंत)
    VrihatAnta,
}

// ============================================================================
// BINARY WRITER
// ============================================================================

/// Binary writer (द्व्यंक लेखक)
#[cfg(feature = "alloc")]
pub struct DvyankaLekhaka {
    buffer: Vec<u8>,
    endian: BaitKrama,
}

#[cfg(feature = "alloc")]
impl DvyankaLekhaka {
    /// Create new writer (नव लेखक)
    pub fn nava() -> Self {
        Self {
            buffer: Vec::new(),
            endian: BaitKrama::LaghuAnta,
        }
    }

    /// Create with capacity
    pub fn nava_kshamata(capacity: usize) -> Self {
        Self {
            buffer: Vec::with_capacity(capacity),
            endian: BaitKrama::LaghuAnta,
        }
    }

    /// Create with endianness
    pub fn nava_krama(endian: BaitKrama) -> Self {
        Self {
            buffer: Vec::new(),
            endian,
        }
    }

    /// Set endianness
    pub fn krama(&mut self, endian: BaitKrama) -> &mut Self {
        self.endian = endian;
        self
    }

    /// Write u8
    pub fn likha_u8(&mut self, value: u8) -> &mut Self {
        self.buffer.push(value);
        self
    }

    /// Write i8
    pub fn likha_i8(&mut self, value: i8) -> &mut Self {
        self.buffer.push(value as u8);
        self
    }

    /// Write u16
    pub fn likha_u16(&mut self, value: u16) -> &mut Self {
        let bytes = match self.endian {
            BaitKrama::LaghuAnta => value.to_le_bytes(),
            BaitKrama::VrihatAnta => value.to_be_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
        self
    }

    /// Write i16
    pub fn likha_i16(&mut self, value: i16) -> &mut Self {
        let bytes = match self.endian {
            BaitKrama::LaghuAnta => value.to_le_bytes(),
            BaitKrama::VrihatAnta => value.to_be_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
        self
    }

    /// Write u32
    pub fn likha_u32(&mut self, value: u32) -> &mut Self {
        let bytes = match self.endian {
            BaitKrama::LaghuAnta => value.to_le_bytes(),
            BaitKrama::VrihatAnta => value.to_be_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
        self
    }

    /// Write i32
    pub fn likha_i32(&mut self, value: i32) -> &mut Self {
        let bytes = match self.endian {
            BaitKrama::LaghuAnta => value.to_le_bytes(),
            BaitKrama::VrihatAnta => value.to_be_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
        self
    }

    /// Write u64
    pub fn likha_u64(&mut self, value: u64) -> &mut Self {
        let bytes = match self.endian {
            BaitKrama::LaghuAnta => value.to_le_bytes(),
            BaitKrama::VrihatAnta => value.to_be_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
        self
    }

    /// Write i64
    pub fn likha_i64(&mut self, value: i64) -> &mut Self {
        let bytes = match self.endian {
            BaitKrama::LaghuAnta => value.to_le_bytes(),
            BaitKrama::VrihatAnta => value.to_be_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
        self
    }

    /// Write f32
    pub fn likha_f32(&mut self, value: f32) -> &mut Self {
        let bytes = match self.endian {
            BaitKrama::LaghuAnta => value.to_le_bytes(),
            BaitKrama::VrihatAnta => value.to_be_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
        self
    }

    /// Write f64
    pub fn likha_f64(&mut self, value: f64) -> &mut Self {
        let bytes = match self.endian {
            BaitKrama::LaghuAnta => value.to_le_bytes(),
            BaitKrama::VrihatAnta => value.to_be_bytes(),
        };
        self.buffer.extend_from_slice(&bytes);
        self
    }

    /// Write bool
    pub fn likha_satya(&mut self, value: bool) -> &mut Self {
        self.buffer.push(if value { 1 } else { 0 });
        self
    }

    /// Write raw bytes
    pub fn likha_baits(&mut self, bytes: &[u8]) -> &mut Self {
        self.buffer.extend_from_slice(bytes);
        self
    }

    /// Write length-prefixed bytes (u32 length)
    pub fn likha_lambai_baits(&mut self, bytes: &[u8]) -> &mut Self {
        self.likha_u32(bytes.len() as u32);
        self.buffer.extend_from_slice(bytes);
        self
    }

    /// Write length-prefixed string (u32 length + UTF-8)
    pub fn likha_sutra(&mut self, s: &str) -> &mut Self {
        self.likha_lambai_baits(s.as_bytes())
    }

    /// Write variable-length integer (LEB128)
    pub fn likha_varint(&mut self, mut value: u64) -> &mut Self {
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            self.buffer.push(byte);
            if value == 0 {
                break;
            }
        }
        self
    }

    /// Get current position
    pub fn sthana(&self) -> usize {
        self.buffer.len()
    }

    /// Get buffer
    pub fn avashesha(&self) -> &[u8] {
        &self.buffer
    }

    /// Take buffer
    pub fn samapti(self) -> Vec<u8> {
        self.buffer
    }

    /// Clear buffer
    pub fn shuddhikaran(&mut self) {
        self.buffer.clear();
    }
}

// ============================================================================
// BINARY READER
// ============================================================================

/// Binary reader (द्व्यंक पाठक)
pub struct DvyankaPathaka<'a> {
    data: &'a [u8],
    position: usize,
    endian: BaitKrama,
}

/// Read error (पठन त्रुटि)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PathanaDosha {
    /// End of data (आंकड़ा समाप्त)
    AnkadaSamapti,
    /// Invalid data (अमान्य आंकड़ा)
    AmanyaAnkada,
    /// Invalid UTF-8 (अमान्य UTF-8)
    AmanyaUtf8,
}

impl<'a> DvyankaPathaka<'a> {
    /// Create new reader
    pub fn nava(data: &'a [u8]) -> Self {
        Self {
            data,
            position: 0,
            endian: BaitKrama::LaghuAnta,
        }
    }

    /// Create with endianness
    pub fn nava_krama(data: &'a [u8], endian: BaitKrama) -> Self {
        Self {
            data,
            position: 0,
            endian,
        }
    }

    /// Set endianness
    pub fn krama(&mut self, endian: BaitKrama) -> &mut Self {
        self.endian = endian;
        self
    }

    /// Get current position
    pub fn sthana(&self) -> usize {
        self.position
    }

    /// Get remaining bytes
    pub fn shesh(&self) -> usize {
        self.data.len().saturating_sub(self.position)
    }

    /// Check if at end
    pub fn samapti(&self) -> bool {
        self.position >= self.data.len()
    }

    /// Seek to position
    pub fn khoj(&mut self, pos: usize) -> Result<(), PathanaDosha> {
        if pos > self.data.len() {
            return Err(PathanaDosha::AnkadaSamapti);
        }
        self.position = pos;
        Ok(())
    }

    /// Skip bytes
    pub fn chhod(&mut self, count: usize) -> Result<(), PathanaDosha> {
        if self.position + count > self.data.len() {
            return Err(PathanaDosha::AnkadaSamapti);
        }
        self.position += count;
        Ok(())
    }

    /// Read u8
    pub fn padha_u8(&mut self) -> Result<u8, PathanaDosha> {
        if self.position >= self.data.len() {
            return Err(PathanaDosha::AnkadaSamapti);
        }
        let value = self.data[self.position];
        self.position += 1;
        Ok(value)
    }

    /// Read i8
    pub fn padha_i8(&mut self) -> Result<i8, PathanaDosha> {
        self.padha_u8().map(|v| v as i8)
    }

    /// Read u16
    pub fn padha_u16(&mut self) -> Result<u16, PathanaDosha> {
        if self.position + 2 > self.data.len() {
            return Err(PathanaDosha::AnkadaSamapti);
        }
        let bytes: [u8; 2] = self.data[self.position..self.position + 2]
            .try_into()
            .unwrap();
        self.position += 2;
        Ok(match self.endian {
            BaitKrama::LaghuAnta => u16::from_le_bytes(bytes),
            BaitKrama::VrihatAnta => u16::from_be_bytes(bytes),
        })
    }

    /// Read i16
    pub fn padha_i16(&mut self) -> Result<i16, PathanaDosha> {
        if self.position + 2 > self.data.len() {
            return Err(PathanaDosha::AnkadaSamapti);
        }
        let bytes: [u8; 2] = self.data[self.position..self.position + 2]
            .try_into()
            .unwrap();
        self.position += 2;
        Ok(match self.endian {
            BaitKrama::LaghuAnta => i16::from_le_bytes(bytes),
            BaitKrama::VrihatAnta => i16::from_be_bytes(bytes),
        })
    }

    /// Read u32
    pub fn padha_u32(&mut self) -> Result<u32, PathanaDosha> {
        if self.position + 4 > self.data.len() {
            return Err(PathanaDosha::AnkadaSamapti);
        }
        let bytes: [u8; 4] = self.data[self.position..self.position + 4]
            .try_into()
            .unwrap();
        self.position += 4;
        Ok(match self.endian {
            BaitKrama::LaghuAnta => u32::from_le_bytes(bytes),
            BaitKrama::VrihatAnta => u32::from_be_bytes(bytes),
        })
    }

    /// Read i32
    pub fn padha_i32(&mut self) -> Result<i32, PathanaDosha> {
        if self.position + 4 > self.data.len() {
            return Err(PathanaDosha::AnkadaSamapti);
        }
        let bytes: [u8; 4] = self.data[self.position..self.position + 4]
            .try_into()
            .unwrap();
        self.position += 4;
        Ok(match self.endian {
            BaitKrama::LaghuAnta => i32::from_le_bytes(bytes),
            BaitKrama::VrihatAnta => i32::from_be_bytes(bytes),
        })
    }

    /// Read u64
    pub fn padha_u64(&mut self) -> Result<u64, PathanaDosha> {
        if self.position + 8 > self.data.len() {
            return Err(PathanaDosha::AnkadaSamapti);
        }
        let bytes: [u8; 8] = self.data[self.position..self.position + 8]
            .try_into()
            .unwrap();
        self.position += 8;
        Ok(match self.endian {
            BaitKrama::LaghuAnta => u64::from_le_bytes(bytes),
            BaitKrama::VrihatAnta => u64::from_be_bytes(bytes),
        })
    }

    /// Read i64
    pub fn padha_i64(&mut self) -> Result<i64, PathanaDosha> {
        if self.position + 8 > self.data.len() {
            return Err(PathanaDosha::AnkadaSamapti);
        }
        let bytes: [u8; 8] = self.data[self.position..self.position + 8]
            .try_into()
            .unwrap();
        self.position += 8;
        Ok(match self.endian {
            BaitKrama::LaghuAnta => i64::from_le_bytes(bytes),
            BaitKrama::VrihatAnta => i64::from_be_bytes(bytes),
        })
    }

    /// Read f32
    pub fn padha_f32(&mut self) -> Result<f32, PathanaDosha> {
        if self.position + 4 > self.data.len() {
            return Err(PathanaDosha::AnkadaSamapti);
        }
        let bytes: [u8; 4] = self.data[self.position..self.position + 4]
            .try_into()
            .unwrap();
        self.position += 4;
        Ok(match self.endian {
            BaitKrama::LaghuAnta => f32::from_le_bytes(bytes),
            BaitKrama::VrihatAnta => f32::from_be_bytes(bytes),
        })
    }

    /// Read f64
    pub fn padha_f64(&mut self) -> Result<f64, PathanaDosha> {
        if self.position + 8 > self.data.len() {
            return Err(PathanaDosha::AnkadaSamapti);
        }
        let bytes: [u8; 8] = self.data[self.position..self.position + 8]
            .try_into()
            .unwrap();
        self.position += 8;
        Ok(match self.endian {
            BaitKrama::LaghuAnta => f64::from_le_bytes(bytes),
            BaitKrama::VrihatAnta => f64::from_be_bytes(bytes),
        })
    }

    /// Read bool
    pub fn padha_satya(&mut self) -> Result<bool, PathanaDosha> {
        self.padha_u8().map(|v| v != 0)
    }

    /// Read raw bytes
    pub fn padha_baits(&mut self, count: usize) -> Result<&'a [u8], PathanaDosha> {
        if self.position + count > self.data.len() {
            return Err(PathanaDosha::AnkadaSamapti);
        }
        let bytes = &self.data[self.position..self.position + count];
        self.position += count;
        Ok(bytes)
    }

    /// Read length-prefixed bytes
    #[cfg(feature = "alloc")]
    pub fn padha_lambai_baits(&mut self) -> Result<Vec<u8>, PathanaDosha> {
        let len = self.padha_u32()? as usize;
        let bytes = self.padha_baits(len)?;
        Ok(bytes.to_vec())
    }

    /// Read length-prefixed string
    #[cfg(feature = "alloc")]
    pub fn padha_sutra(&mut self) -> Result<String, PathanaDosha> {
        let len = self.padha_u32()? as usize;
        let bytes = self.padha_baits(len)?;
        String::from_utf8(bytes.to_vec()).map_err(|_| PathanaDosha::AmanyaUtf8)
    }

    /// Read variable-length integer (LEB128)
    pub fn padha_varint(&mut self) -> Result<u64, PathanaDosha> {
        let mut result = 0u64;
        let mut shift = 0;

        loop {
            let byte = self.padha_u8()?;
            result |= ((byte & 0x7F) as u64) << shift;

            if byte & 0x80 == 0 {
                break;
            }

            shift += 7;
            if shift >= 64 {
                return Err(PathanaDosha::AmanyaAnkada);
            }
        }

        Ok(result)
    }
}

// ============================================================================
// HEX ENCODING
// ============================================================================

/// Convert bytes to hex string (षोडशी रूपांतरण)
#[cfg(feature = "alloc")]
pub fn shodashi_rupantarana(data: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut result = String::with_capacity(data.len() * 2);

    for &byte in data {
        result.push(HEX[(byte >> 4) as usize] as char);
        result.push(HEX[(byte & 0x0F) as usize] as char);
    }

    result
}

/// Convert hex string to bytes (षोडशी विश्लेषण)
#[cfg(feature = "alloc")]
pub fn shodashi_vishleshan(hex: &str) -> Option<Vec<u8>> {
    if hex.len() % 2 != 0 {
        return None;
    }

    let mut result = Vec::with_capacity(hex.len() / 2);
    let chars: Vec<char> = hex.chars().collect();

    for i in (0..chars.len()).step_by(2) {
        let hi = hex_digit(chars[i])?;
        let lo = hex_digit(chars[i + 1])?;
        result.push((hi << 4) | lo);
    }

    Some(result)
}

fn hex_digit(c: char) -> Option<u8> {
    match c {
        '0'..='9' => Some(c as u8 - b'0'),
        'a'..='f' => Some(c as u8 - b'a' + 10),
        'A'..='F' => Some(c as u8 - b'A' + 10),
        _ => None,
    }
}

// ============================================================================
// ZIGZAG ENCODING (for signed integers)
// ============================================================================

/// Zigzag encode signed to unsigned
pub fn zigzag_kodita(n: i64) -> u64 {
    ((n << 1) ^ (n >> 63)) as u64
}

/// Zigzag decode unsigned to signed
pub fn zigzag_vikodita(n: u64) -> i64 {
    ((n >> 1) as i64) ^ (-((n & 1) as i64))
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_writer_reader_integers() {
        let mut writer = DvyankaLekhaka::nava();
        writer
            .likha_u8(0x12)
            .likha_u16(0x3456)
            .likha_u32(0x789ABCDE)
            .likha_u64(0x0123456789ABCDEF);

        let data = writer.samapti();
        let mut reader = DvyankaPathaka::nava(&data);

        assert_eq!(reader.padha_u8().unwrap(), 0x12);
        assert_eq!(reader.padha_u16().unwrap(), 0x3456);
        assert_eq!(reader.padha_u32().unwrap(), 0x789ABCDE);
        assert_eq!(reader.padha_u64().unwrap(), 0x0123456789ABCDEF);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_big_endian() {
        let mut writer = DvyankaLekhaka::nava_krama(BaitKrama::VrihatAnta);
        writer.likha_u32(0x01020304);

        let data = writer.samapti();
        assert_eq!(data, [0x01, 0x02, 0x03, 0x04]);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_little_endian() {
        let mut writer = DvyankaLekhaka::nava_krama(BaitKrama::LaghuAnta);
        writer.likha_u32(0x01020304);

        let data = writer.samapti();
        assert_eq!(data, [0x04, 0x03, 0x02, 0x01]);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_string() {
        let mut writer = DvyankaLekhaka::nava();
        writer.likha_sutra("Hello, 世界!");

        let data = writer.samapti();
        let mut reader = DvyankaPathaka::nava(&data);

        assert_eq!(reader.padha_sutra().unwrap(), "Hello, 世界!");
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_varint() {
        let mut writer = DvyankaLekhaka::nava();
        writer
            .likha_varint(0)
            .likha_varint(127)
            .likha_varint(128)
            .likha_varint(16383)
            .likha_varint(16384);

        let data = writer.samapti();
        let mut reader = DvyankaPathaka::nava(&data);

        assert_eq!(reader.padha_varint().unwrap(), 0);
        assert_eq!(reader.padha_varint().unwrap(), 127);
        assert_eq!(reader.padha_varint().unwrap(), 128);
        assert_eq!(reader.padha_varint().unwrap(), 16383);
        assert_eq!(reader.padha_varint().unwrap(), 16384);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_hex() {
        let data = [0xDE, 0xAD, 0xBE, 0xEF];
        let hex = shodashi_rupantarana(&data);
        assert_eq!(hex, "deadbeef");

        let back = shodashi_vishleshan(&hex).unwrap();
        assert_eq!(back, data);
    }

    #[test]
    fn test_zigzag() {
        assert_eq!(zigzag_kodita(0), 0);
        assert_eq!(zigzag_kodita(-1), 1);
        assert_eq!(zigzag_kodita(1), 2);
        assert_eq!(zigzag_kodita(-2), 3);
        assert_eq!(zigzag_kodita(2), 4);

        assert_eq!(zigzag_vikodita(0), 0);
        assert_eq!(zigzag_vikodita(1), -1);
        assert_eq!(zigzag_vikodita(2), 1);
        assert_eq!(zigzag_vikodita(3), -2);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_floats() {
        let mut writer = DvyankaLekhaka::nava();
        writer.likha_f32(3.14159).likha_f64(2.718281828459045);

        let data = writer.samapti();
        let mut reader = DvyankaPathaka::nava(&data);

        let f32_val = reader.padha_f32().unwrap();
        let f64_val = reader.padha_f64().unwrap();

        assert!((f32_val - 3.14159).abs() < 0.00001);
        assert!((f64_val - 2.718281828459045).abs() < 0.0000000001);
    }
}
