//! # Varna - Colors (वर्ण)
//!
//! Color representations and conversions.
//!
//! > **"वर्णाः जीवनस्य रङ्गाः"**
//! > *"Colors are the hues of life"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::format;
#[cfg(feature = "alloc")]
use alloc::string::String;

use core::fmt;

// ============================================================================
// RGB COLOR
// ============================================================================

/// RGB color (0-255 per channel)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Rgb {
    pub laal: u8,  // Red
    pub hara: u8,  // Green
    pub neela: u8, // Blue
}

impl Rgb {
    /// Create new RGB color
    pub const fn nava(laal: u8, hara: u8, neela: u8) -> Self {
        Self { laal, hara, neela }
    }

    /// Black
    pub const fn kaala() -> Self {
        Self {
            laal: 0,
            hara: 0,
            neela: 0,
        }
    }

    /// White
    pub const fn shveta() -> Self {
        Self {
            laal: 255,
            hara: 255,
            neela: 255,
        }
    }

    /// Red
    pub const fn laal() -> Self {
        Self {
            laal: 255,
            hara: 0,
            neela: 0,
        }
    }

    /// Green
    pub const fn hara() -> Self {
        Self {
            laal: 0,
            hara: 255,
            neela: 0,
        }
    }

    /// Blue
    pub const fn neela() -> Self {
        Self {
            laal: 0,
            hara: 0,
            neela: 255,
        }
    }

    /// Yellow
    pub const fn peela() -> Self {
        Self {
            laal: 255,
            hara: 255,
            neela: 0,
        }
    }

    /// Cyan
    pub const fn firozi() -> Self {
        Self {
            laal: 0,
            hara: 255,
            neela: 255,
        }
    }

    /// Magenta
    pub const fn rani() -> Self {
        Self {
            laal: 255,
            hara: 0,
            neela: 255,
        }
    }

    /// Create from hex value (0xRRGGBB)
    pub const fn hex_se(hex: u32) -> Self {
        Self {
            laal: ((hex >> 16) & 0xFF) as u8,
            hara: ((hex >> 8) & 0xFF) as u8,
            neela: (hex & 0xFF) as u8,
        }
    }

    /// Convert to hex value
    pub const fn hex_mein(&self) -> u32 {
        ((self.laal as u32) << 16) | ((self.hara as u32) << 8) | (self.neela as u32)
    }

    /// Convert to hex string
    #[cfg(feature = "alloc")]
    pub fn hex_sutra(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.laal, self.hara, self.neela)
    }

    /// Parse from hex string
    #[cfg(feature = "alloc")]
    pub fn hex_vishleshan(s: &str) -> Option<Self> {
        let s = s.trim_start_matches('#');
        if s.len() == 6 {
            let hex = u32::from_str_radix(s, 16).ok()?;
            Some(Self::hex_se(hex))
        } else if s.len() == 3 {
            // Shorthand like "F00" -> "FF0000"
            let chars: Vec<char> = s.chars().collect();
            let r = u8::from_str_radix(&format!("{}{}", chars[0], chars[0]), 16).ok()?;
            let g = u8::from_str_radix(&format!("{}{}", chars[1], chars[1]), 16).ok()?;
            let b = u8::from_str_radix(&format!("{}{}", chars[2], chars[2]), 16).ok()?;
            Some(Self::nava(r, g, b))
        } else {
            None
        }
    }

    /// Convert to RGBA
    pub const fn rgba(&self, alpha: u8) -> Rgba {
        Rgba::nava(self.laal, self.hara, self.neela, alpha)
    }

    /// Grayscale value (luminosity method)
    pub fn dhoosar_mana(&self) -> u8 {
        let l = 0.2126 * (self.laal as f64)
            + 0.7152 * (self.hara as f64)
            + 0.0722 * (self.neela as f64);
        l.clamp(0.0, 255.0) as u8
    }

    /// Convert to grayscale
    pub fn dhoosar(&self) -> Self {
        let g = self.dhoosar_mana();
        Self::nava(g, g, g)
    }

    /// Invert color
    pub fn ulat(&self) -> Self {
        Self::nava(255 - self.laal, 255 - self.hara, 255 - self.neela)
    }

    /// Blend with another color (linear)
    pub fn mishran(&self, other: &Self, t: f64) -> Self {
        let t = t.clamp(0.0, 1.0);
        Self::nava(
            ((1.0 - t) * self.laal as f64 + t * other.laal as f64) as u8,
            ((1.0 - t) * self.hara as f64 + t * other.hara as f64) as u8,
            ((1.0 - t) * self.neela as f64 + t * other.neela as f64) as u8,
        )
    }

    /// Convert to HSL
    pub fn hsl(&self) -> Hsl {
        let r = self.laal as f64 / 255.0;
        let g = self.hara as f64 / 255.0;
        let b = self.neela as f64 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let l = (max + min) / 2.0;

        let s = if delta == 0.0 {
            0.0
        } else {
            delta / (1.0 - (2.0 * l - 1.0).abs())
        };

        let h = if delta == 0.0 {
            0.0
        } else if max == r {
            60.0 * (((g - b) / delta) % 6.0)
        } else if max == g {
            60.0 * ((b - r) / delta + 2.0)
        } else {
            60.0 * ((r - g) / delta + 4.0)
        };

        let h = if h < 0.0 { h + 360.0 } else { h };

        Hsl {
            chhaya: h,
            santrapi: s,
            prakash: l,
        }
    }
}

impl fmt::Display for Rgb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rgb({}, {}, {})", self.laal, self.hara, self.neela)
    }
}

// ============================================================================
// RGBA COLOR
// ============================================================================

/// RGBA color with alpha
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Rgba {
    pub laal: u8,
    pub hara: u8,
    pub neela: u8,
    pub paadarshita: u8, // Alpha
}

impl Rgba {
    /// Create new RGBA color
    pub const fn nava(laal: u8, hara: u8, neela: u8, paadarshita: u8) -> Self {
        Self {
            laal,
            hara,
            neela,
            paadarshita,
        }
    }

    /// Create from RGB
    pub const fn rgb_se(rgb: Rgb) -> Self {
        Self::nava(rgb.laal, rgb.hara, rgb.neela, 255)
    }

    /// Transparent
    pub const fn paadarshi() -> Self {
        Self {
            laal: 0,
            hara: 0,
            neela: 0,
            paadarshita: 0,
        }
    }

    /// Get RGB part
    pub const fn rgb(&self) -> Rgb {
        Rgb::nava(self.laal, self.hara, self.neela)
    }

    /// Alpha as float (0.0 to 1.0)
    pub fn alpha(&self) -> f64 {
        self.paadarshita as f64 / 255.0
    }

    /// Check if fully opaque
    pub const fn apaadarshi_hai(&self) -> bool {
        self.paadarshita == 255
    }

    /// Check if fully transparent
    pub const fn paadarshi_hai(&self) -> bool {
        self.paadarshita == 0
    }

    /// Alpha blend over another color
    pub fn alpha_mishran(&self, niche: &Rgba) -> Rgba {
        let a1 = self.paadarshita as f64 / 255.0;
        let a2 = niche.paadarshita as f64 / 255.0;
        let a_out = a1 + a2 * (1.0 - a1);

        if a_out == 0.0 {
            return Rgba::paadarshi();
        }

        let r = (self.laal as f64 * a1 + niche.laal as f64 * a2 * (1.0 - a1)) / a_out;
        let g = (self.hara as f64 * a1 + niche.hara as f64 * a2 * (1.0 - a1)) / a_out;
        let b = (self.neela as f64 * a1 + niche.neela as f64 * a2 * (1.0 - a1)) / a_out;

        Rgba::nava(r as u8, g as u8, b as u8, (a_out * 255.0) as u8)
    }
}

impl fmt::Display for Rgba {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "rgba({}, {}, {}, {:.2})",
            self.laal,
            self.hara,
            self.neela,
            self.alpha()
        )
    }
}

// ============================================================================
// HSL COLOR
// ============================================================================

/// HSL color (Hue, Saturation, Lightness)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Hsl {
    pub chhaya: f64,   // Hue (0-360)
    pub santrapi: f64, // Saturation (0-1)
    pub prakash: f64,  // Lightness (0-1)
}

impl Hsl {
    /// Create new HSL color
    pub fn nava(chhaya: f64, santrapi: f64, prakash: f64) -> Self {
        Self {
            chhaya: chhaya % 360.0,
            santrapi: santrapi.clamp(0.0, 1.0),
            prakash: prakash.clamp(0.0, 1.0),
        }
    }

    /// Convert to RGB
    pub fn rgb(&self) -> Rgb {
        let c = (1.0 - (2.0 * self.prakash - 1.0).abs()) * self.santrapi;
        let h_prime = self.chhaya / 60.0;
        let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
        let m = self.prakash - c / 2.0;

        let (r, g, b) = match h_prime as i32 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        Rgb::nava(
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }

    /// Rotate hue
    pub fn chhaya_ghurnana(&self, digri: f64) -> Self {
        Self::nava(self.chhaya + digri, self.santrapi, self.prakash)
    }

    /// Lighten
    pub fn ujjaval(&self, matra: f64) -> Self {
        Self::nava(self.chhaya, self.santrapi, (self.prakash + matra).min(1.0))
    }

    /// Darken
    pub fn dhundla(&self, matra: f64) -> Self {
        Self::nava(self.chhaya, self.santrapi, (self.prakash - matra).max(0.0))
    }

    /// Saturate
    pub fn santrapi_badha(&self, matra: f64) -> Self {
        Self::nava(self.chhaya, (self.santrapi + matra).min(1.0), self.prakash)
    }

    /// Desaturate
    pub fn santrapi_ghatao(&self, matra: f64) -> Self {
        Self::nava(self.chhaya, (self.santrapi - matra).max(0.0), self.prakash)
    }

    /// Complementary color (opposite hue)
    pub fn poorak(&self) -> Self {
        self.chhaya_ghurnana(180.0)
    }
}

impl fmt::Display for Hsl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "hsl({:.0}, {:.0}%, {:.0}%)",
            self.chhaya,
            self.santrapi * 100.0,
            self.prakash * 100.0
        )
    }
}

// ============================================================================
// HSV COLOR
// ============================================================================

/// HSV color (Hue, Saturation, Value)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Hsv {
    pub chhaya: f64,   // Hue (0-360)
    pub santrapi: f64, // Saturation (0-1)
    pub mana: f64,     // Value (0-1)
}

impl Hsv {
    /// Create new HSV color
    pub fn nava(chhaya: f64, santrapi: f64, mana: f64) -> Self {
        Self {
            chhaya: chhaya % 360.0,
            santrapi: santrapi.clamp(0.0, 1.0),
            mana: mana.clamp(0.0, 1.0),
        }
    }

    /// Convert to RGB
    pub fn rgb(&self) -> Rgb {
        let c = self.mana * self.santrapi;
        let h_prime = self.chhaya / 60.0;
        let x = c * (1.0 - (h_prime % 2.0 - 1.0).abs());
        let m = self.mana - c;

        let (r, g, b) = match h_prime as i32 {
            0 => (c, x, 0.0),
            1 => (x, c, 0.0),
            2 => (0.0, c, x),
            3 => (0.0, x, c),
            4 => (x, 0.0, c),
            _ => (c, 0.0, x),
        };

        Rgb::nava(
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }
}

// ============================================================================
// COLOR PALETTE
// ============================================================================

/// Standard web colors
pub mod rang {
    use super::Rgb;

    pub const ALICE_BLUE: Rgb = Rgb::hex_se(0xF0F8FF);
    pub const CORAL: Rgb = Rgb::hex_se(0xFF7F50);
    pub const CRIMSON: Rgb = Rgb::hex_se(0xDC143C);
    pub const DARK_BLUE: Rgb = Rgb::hex_se(0x00008B);
    pub const FOREST_GREEN: Rgb = Rgb::hex_se(0x228B22);
    pub const GOLD: Rgb = Rgb::hex_se(0xFFD700);
    pub const INDIGO: Rgb = Rgb::hex_se(0x4B0082);
    pub const LAVENDER: Rgb = Rgb::hex_se(0xE6E6FA);
    pub const LIME_GREEN: Rgb = Rgb::hex_se(0x32CD32);
    pub const NAVY: Rgb = Rgb::hex_se(0x000080);
    pub const OLIVE: Rgb = Rgb::hex_se(0x808000);
    pub const ORANGE: Rgb = Rgb::hex_se(0xFFA500);
    pub const ORCHID: Rgb = Rgb::hex_se(0xDA70D6);
    pub const PINK: Rgb = Rgb::hex_se(0xFFC0CB);
    pub const PLUM: Rgb = Rgb::hex_se(0xDDA0DD);
    pub const PURPLE: Rgb = Rgb::hex_se(0x800080);
    pub const SALMON: Rgb = Rgb::hex_se(0xFA8072);
    pub const SEA_GREEN: Rgb = Rgb::hex_se(0x2E8B57);
    pub const SILVER: Rgb = Rgb::hex_se(0xC0C0C0);
    pub const SKY_BLUE: Rgb = Rgb::hex_se(0x87CEEB);
    pub const SLATE_GRAY: Rgb = Rgb::hex_se(0x708090);
    pub const TEAL: Rgb = Rgb::hex_se(0x008080);
    pub const TOMATO: Rgb = Rgb::hex_se(0xFF6347);
    pub const TURQUOISE: Rgb = Rgb::hex_se(0x40E0D0);
    pub const VIOLET: Rgb = Rgb::hex_se(0xEE82EE);
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb_hex() {
        let color = Rgb::hex_se(0xFF5733);
        assert_eq!(color.laal, 255);
        assert_eq!(color.hara, 87);
        assert_eq!(color.neela, 51);

        assert_eq!(color.hex_mein(), 0xFF5733);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_hex_string() {
        let color = Rgb::nava(255, 87, 51);
        assert_eq!(color.hex_sutra(), "#FF5733");

        let parsed = Rgb::hex_vishleshan("#FF5733").unwrap();
        assert_eq!(parsed, color);
    }

    #[test]
    fn test_grayscale() {
        let red = Rgb::laal();
        let gray = red.dhoosar();
        // Red should become relatively dark gray due to luminosity
        assert!(gray.laal < 100);
    }

    #[test]
    fn test_invert() {
        let black = Rgb::kaala();
        let white = black.ulat();
        assert_eq!(white, Rgb::shveta());
    }

    #[test]
    fn test_hsl_conversion() {
        let red = Rgb::laal();
        let hsl = red.hsl();

        assert!((hsl.chhaya - 0.0).abs() < 1.0);
        assert!((hsl.santrapi - 1.0).abs() < 0.01);

        let back = hsl.rgb();
        assert_eq!(back.laal, 255);
    }

    #[test]
    fn test_alpha_blend() {
        let top = Rgba::nava(255, 0, 0, 128); // 50% red
        let bottom = Rgba::nava(0, 0, 255, 255); // blue
        let result = top.alpha_mishran(&bottom);

        // Should be purple-ish
        assert!(result.laal > 100);
        assert!(result.neela > 100);
    }

    #[test]
    fn test_complementary() {
        let red = Hsl::nava(0.0, 1.0, 0.5);
        let cyan = red.poorak();
        assert!((cyan.chhaya - 180.0).abs() < 1.0);
    }
}
