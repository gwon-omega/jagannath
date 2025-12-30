//! # Taranga - Waveforms (तरङ्ग)
//!
//! Audio waveform generation and manipulation.

use core::f64::consts::PI;

/// Waveform type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TarangaPrakara {
    /// Sine wave
    Jya,
    /// Square wave
    Varga,
    /// Triangle wave
    Tribhuja,
    /// Sawtooth wave
    Karaari,
    /// Noise (white)
    Shwet,
}

/// Sample rate
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NamunaDar(pub u32);

impl NamunaDar {
    /// CD quality (44100 Hz)
    pub const CD: Self = Self(44100);
    /// DVD quality (48000 Hz)
    pub const DVD: Self = Self(48000);
    /// High quality (96000 Hz)
    pub const UCHHA: Self = Self(96000);
    /// Low quality (22050 Hz)
    pub const NIMNA: Self = Self(22050);
    /// Telephony (8000 Hz)
    pub const DOORDHWANI: Self = Self(8000);

    /// Get Nyquist frequency
    pub const fn nyquist(&self) -> f64 {
        self.0 as f64 / 2.0
    }
}

/// Audio sample (mono)
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Namuna(pub f64);

impl Namuna {
    /// Create sample
    pub const fn nava(mana: f64) -> Self {
        Self(mana)
    }

    /// Clamp to valid range [-1, 1]
    pub fn seema(&self) -> Self {
        Self(if self.0 < -1.0 {
            -1.0
        } else if self.0 > 1.0 {
            1.0
        } else {
            self.0
        })
    }

    /// Convert to 16-bit integer
    pub fn int16(&self) -> i16 {
        let clamped = self.seema().0;
        (clamped * 32767.0) as i16
    }

    /// Convert from 16-bit integer
    pub fn int16_se(mana: i16) -> Self {
        Self(mana as f64 / 32767.0)
    }
}

/// Oscillator for generating waveforms
#[derive(Debug, Clone)]
pub struct Dolak {
    pub prakara: TarangaPrakara,
    pub aavriti: f64, // Frequency (Hz)
    pub aayaam: f64,  // Amplitude (0-1)
    pub kala: f64,    // Phase (0-2π)
    pub namuna_dar: NamunaDar,
    sthiti: f64, // Current position
}

impl Dolak {
    /// Create new oscillator
    pub fn nava(prakara: TarangaPrakara, aavriti: f64) -> Self {
        Self {
            prakara,
            aavriti,
            aayaam: 1.0,
            kala: 0.0,
            namuna_dar: NamunaDar::CD,
            sthiti: 0.0,
        }
    }

    /// Set amplitude
    pub fn aayaam_sthapita(mut self, aayaam: f64) -> Self {
        self.aayaam = aayaam;
        self
    }

    /// Set phase
    pub fn kala_sthapita(mut self, kala: f64) -> Self {
        self.kala = kala;
        self
    }

    /// Set sample rate
    pub fn dar_sthapita(mut self, dar: NamunaDar) -> Self {
        self.namuna_dar = dar;
        self
    }

    /// Reset oscillator
    pub fn punasthapana(&mut self) {
        self.sthiti = 0.0;
    }

    /// Generate next sample
    pub fn agla(&mut self) -> Namuna {
        let t = self.sthiti + self.kala;

        let mana = match self.prakara {
            TarangaPrakara::Jya => libm::sin(t * 2.0 * PI),
            TarangaPrakara::Varga => {
                if libm::sin(t * 2.0 * PI) >= 0.0 {
                    1.0
                } else {
                    -1.0
                }
            }
            TarangaPrakara::Tribhuja => {
                let phase = t % 1.0;
                if phase < 0.25 {
                    phase * 4.0
                } else if phase < 0.75 {
                    2.0 - phase * 4.0
                } else {
                    phase * 4.0 - 4.0
                }
            }
            TarangaPrakara::Karaari => (t % 1.0) * 2.0 - 1.0,
            TarangaPrakara::Shwet => {
                // Simple LCG for pseudo-random
                let seed = (self.sthiti * 1e9) as u64;
                let rng = seed.wrapping_mul(1103515245).wrapping_add(12345);
                ((rng >> 16) as f64 / 32768.0) - 1.0
            }
        };

        self.sthiti += self.aavriti / self.namuna_dar.0 as f64;
        if self.sthiti >= 1.0 {
            self.sthiti -= 1.0;
        }

        Namuna(mana * self.aayaam)
    }
}

/// Simple sine wave generator function
pub fn jya_taranga(aavriti: f64, samay: f64) -> f64 {
    libm::sin(2.0 * PI * aavriti * samay)
}

/// Generate sine wave samples
pub fn jya_namune(aavriti: f64, avadhi: f64, dar: NamunaDar) -> impl Iterator<Item = Namuna> {
    let kul = (avadhi * dar.0 as f64) as usize;
    (0..kul).map(move |i| {
        let t = i as f64 / dar.0 as f64;
        Namuna(libm::sin(2.0 * PI * aavriti * t))
    })
}

/// ADSR envelope
#[derive(Debug, Clone, Copy)]
pub struct Aavarana {
    pub aakraman: f64, // Attack time (seconds)
    pub kshaya: f64,   // Decay time
    pub sthiti: f64,   // Sustain level (0-1)
    pub mukti: f64,    // Release time
}

impl Aavarana {
    /// Create ADSR envelope
    pub const fn nava(aakraman: f64, kshaya: f64, sthiti: f64, mukti: f64) -> Self {
        Self {
            aakraman,
            kshaya,
            sthiti,
            mukti,
        }
    }

    /// Default envelope
    pub const fn manaka() -> Self {
        Self {
            aakraman: 0.01,
            kshaya: 0.1,
            sthiti: 0.7,
            mukti: 0.3,
        }
    }

    /// Get envelope value at time (note_on = true while key pressed)
    pub fn mana(&self, samay: f64, mukti_samay: Option<f64>) -> f64 {
        match mukti_samay {
            None => {
                // Note is still on
                if samay < self.aakraman {
                    // Attack phase
                    samay / self.aakraman
                } else if samay < self.aakraman + self.kshaya {
                    // Decay phase
                    let t = (samay - self.aakraman) / self.kshaya;
                    1.0 - t * (1.0 - self.sthiti)
                } else {
                    // Sustain phase
                    self.sthiti
                }
            }
            Some(rel) => {
                // Note released at `rel` time
                let sustain_level = self.mana(rel, None);
                let release_elapsed = samay - rel;

                if release_elapsed >= self.mukti {
                    0.0
                } else {
                    sustain_level * (1.0 - release_elapsed / self.mukti)
                }
            }
        }
    }
}

/// Mix multiple audio signals
pub fn mishran(signals: &[Namuna]) -> Namuna {
    if signals.is_empty() {
        return Namuna(0.0);
    }
    let sum: f64 = signals.iter().map(|s| s.0).sum();
    Namuna(sum / signals.len() as f64)
}

/// Linear interpolation between samples
pub fn lerp(a: Namuna, b: Namuna, t: f64) -> Namuna {
    Namuna(a.0 + (b.0 - a.0) * t)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sine() {
        let mut osc = Dolak::nava(TarangaPrakara::Jya, 440.0);
        let sample = osc.agla();
        assert!(sample.0 >= -1.0 && sample.0 <= 1.0);
    }

    #[test]
    fn test_sample_conversion() {
        let s = Namuna(0.5);
        let i = s.int16();
        let back = Namuna::int16_se(i);
        assert!((back.0 - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_envelope() {
        let env = Aavarana::manaka();

        // At start
        assert!((env.mana(0.0, None)).abs() < 1e-10);

        // At end of attack
        assert!((env.mana(0.01, None) - 1.0).abs() < 1e-10);

        // During sustain
        assert!((env.mana(1.0, None) - 0.7).abs() < 1e-10);
    }
}
