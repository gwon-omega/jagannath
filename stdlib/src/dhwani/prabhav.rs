//! # Prabhav - Audio Effects (प्रभाव)
//!
//! Common audio effects and filters.

use super::taranga::Namuna;

/// Simple low-pass filter state
#[derive(Debug, Clone)]
pub struct NimnaParivahan {
    cutoff: f64, // Cutoff frequency ratio (0-1)
    pichla: f64, // Previous output
}

impl NimnaParivahan {
    /// Create low-pass filter
    pub fn nava(cutoff: f64) -> Self {
        Self {
            cutoff: cutoff.clamp(0.0, 1.0),
            pichla: 0.0,
        }
    }

    /// Process sample
    pub fn prakriya(&mut self, namuna: Namuna) -> Namuna {
        let alpha = self.cutoff;
        let output = alpha * namuna.0 + (1.0 - alpha) * self.pichla;
        self.pichla = output;
        Namuna(output)
    }

    /// Reset filter state
    pub fn punasthapana(&mut self) {
        self.pichla = 0.0;
    }
}

/// Simple high-pass filter state
#[derive(Debug, Clone)]
pub struct UchhaParivahan {
    cutoff: f64,
    pichla_input: f64,
    pichla_output: f64,
}

impl UchhaParivahan {
    /// Create high-pass filter
    pub fn nava(cutoff: f64) -> Self {
        Self {
            cutoff: cutoff.clamp(0.0, 1.0),
            pichla_input: 0.0,
            pichla_output: 0.0,
        }
    }

    /// Process sample
    pub fn prakriya(&mut self, namuna: Namuna) -> Namuna {
        let alpha = 1.0 - self.cutoff;
        let output = alpha * (self.pichla_output + namuna.0 - self.pichla_input);
        self.pichla_input = namuna.0;
        self.pichla_output = output;
        Namuna(output)
    }
}

/// Delay/Echo effect
#[derive(Debug)]
pub struct Vilamba {
    buffer: [f64; 44100], // 1 second at 44.1kHz max
    write_pos: usize,
    delay_samples: usize,
    mishran: f64,     // Wet/dry mix (0-1)
    pratipushti: f64, // Feedback (0-1)
}

impl Vilamba {
    /// Create delay effect
    pub fn nava(delay_ms: f64, mishran: f64) -> Self {
        let delay_samples = ((delay_ms / 1000.0) * 44100.0) as usize;
        Self {
            buffer: [0.0; 44100],
            write_pos: 0,
            delay_samples: delay_samples.min(44099),
            mishran: mishran.clamp(0.0, 1.0),
            pratipushti: 0.5,
        }
    }

    /// Set feedback amount
    pub fn pratipushti_sthapita(mut self, mana: f64) -> Self {
        self.pratipushti = mana.clamp(0.0, 0.99);
        self
    }

    /// Process sample
    pub fn prakriya(&mut self, namuna: Namuna) -> Namuna {
        let read_pos = (self.write_pos + 44100 - self.delay_samples) % 44100;
        let delayed = self.buffer[read_pos];

        let output = namuna.0 * (1.0 - self.mishran) + delayed * self.mishran;
        self.buffer[self.write_pos] = namuna.0 + delayed * self.pratipushti;

        self.write_pos = (self.write_pos + 1) % 44100;

        Namuna(output)
    }
}

/// Gain/Volume control
#[derive(Debug, Clone, Copy)]
pub struct Laabh(pub f64);

impl Laabh {
    /// Create gain from linear value
    pub fn rekha(mana: f64) -> Self {
        Self(mana)
    }

    /// Create gain from decibels
    pub fn db(mana: f64) -> Self {
        Self(libm::pow(10.0, mana / 20.0))
    }

    /// Apply gain
    pub fn lagao(&self, namuna: Namuna) -> Namuna {
        Namuna(namuna.0 * self.0)
    }

    /// Get dB value
    pub fn db_mana(&self) -> f64 {
        20.0 * libm::log10(self.0)
    }
}

/// Distortion/Clipping effect
#[derive(Debug, Clone, Copy)]
pub struct Vikruti {
    pub matra: f64, // Distortion amount (0-1)
}

impl Vikruti {
    /// Create distortion
    pub fn nava(matra: f64) -> Self {
        Self {
            matra: matra.clamp(0.0, 1.0),
        }
    }

    /// Hard clipping
    pub fn kathina_katan(&self, namuna: Namuna) -> Namuna {
        let threshold = 1.0 - self.matra * 0.8;
        Namuna(namuna.0.clamp(-threshold, threshold) / threshold)
    }

    /// Soft clipping (tanh)
    pub fn komal_katan(&self, namuna: Namuna) -> Namuna {
        let drive = 1.0 + self.matra * 10.0;
        Namuna(libm::tanh(namuna.0 * drive) / libm::tanh(drive))
    }
}

/// Tremolo effect (amplitude modulation)
#[derive(Debug, Clone)]
pub struct Kampan {
    pub dar: f64,     // Rate (Hz)
    pub gahraai: f64, // Depth (0-1)
    kala: f64,        // Current phase
}

impl Kampan {
    /// Create tremolo
    pub fn nava(dar: f64, gahraai: f64) -> Self {
        Self {
            dar,
            gahraai: gahraai.clamp(0.0, 1.0),
            kala: 0.0,
        }
    }

    /// Process sample (call at sample rate)
    pub fn prakriya(&mut self, namuna: Namuna, namuna_dar: u32) -> Namuna {
        let lfo = (libm::sin(self.kala * 2.0 * core::f64::consts::PI) + 1.0) / 2.0;
        let mod_amount = 1.0 - self.gahraai * (1.0 - lfo);

        self.kala += self.dar / namuna_dar as f64;
        if self.kala >= 1.0 {
            self.kala -= 1.0;
        }

        Namuna(namuna.0 * mod_amount)
    }
}

/// Vibrato effect (frequency modulation)
#[derive(Debug, Clone)]
pub struct Spandan {
    pub dar: f64,     // Rate (Hz)
    pub gahraai: f64, // Depth (semitones)
    kala: f64,
}

impl Spandan {
    /// Create vibrato
    pub fn nava(dar: f64, gahraai: f64) -> Self {
        Self {
            dar,
            gahraai,
            kala: 0.0,
        }
    }

    /// Get pitch multiplier
    pub fn gunaka(&mut self, namuna_dar: u32) -> f64 {
        let lfo = libm::sin(self.kala * 2.0 * core::f64::consts::PI);
        let semitones = lfo * self.gahraai;

        self.kala += self.dar / namuna_dar as f64;
        if self.kala >= 1.0 {
            self.kala -= 1.0;
        }

        libm::pow(2.0, semitones / 12.0)
    }
}

/// Compressor
#[derive(Debug, Clone)]
pub struct Sampiidak {
    pub seema: f64,    // Threshold (dB)
    pub anupat: f64,   // Ratio
    pub aakraman: f64, // Attack time (seconds)
    pub mukti: f64,    // Release time (seconds)
    envelope: f64,
}

impl Sampiidak {
    /// Create compressor
    pub fn nava(seema: f64, anupat: f64) -> Self {
        Self {
            seema,
            anupat,
            aakraman: 0.01,
            mukti: 0.1,
            envelope: 0.0,
        }
    }

    /// Process sample
    pub fn prakriya(&mut self, namuna: Namuna, namuna_dar: u32) -> Namuna {
        let input_db = if namuna.0.abs() < 1e-10 {
            -96.0
        } else {
            20.0 * libm::log10(namuna.0.abs())
        };

        // Compute gain reduction
        let gain_reduction = if input_db > self.seema {
            (input_db - self.seema) * (1.0 - 1.0 / self.anupat)
        } else {
            0.0
        };

        // Smooth envelope
        let coeff = if gain_reduction > self.envelope {
            1.0 - libm::exp(-1.0 / (self.aakraman * namuna_dar as f64))
        } else {
            1.0 - libm::exp(-1.0 / (self.mukti * namuna_dar as f64))
        };

        self.envelope = self.envelope + coeff * (gain_reduction - self.envelope);

        // Apply gain
        let gain = libm::pow(10.0, -self.envelope / 20.0);
        Namuna(namuna.0 * gain)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gain_db() {
        let g = Laabh::db(6.0);
        assert!((g.0 - 1.995).abs() < 0.01);

        let g = Laabh::db(-6.0);
        assert!((g.0 - 0.501).abs() < 0.01);
    }

    #[test]
    fn test_low_pass() {
        let mut filter = NimnaParivahan::nava(0.1);
        let out = filter.prakriya(Namuna(1.0));
        assert!(out.0 < 1.0); // Should be filtered
    }

    #[test]
    fn test_distortion() {
        let dist = Vikruti::nava(0.5);
        let out = dist.kathina_katan(Namuna(2.0));
        assert!(out.0 <= 1.0); // Should be clipped
    }
}
