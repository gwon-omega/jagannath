//! # Swar - Musical Notes (स्वर)
//!
//! Musical note utilities, scales, and frequency calculations.

use core::f64::consts;

/// Standard tuning A4 = 440 Hz
pub const A4_AAVRITI: f64 = 440.0;

/// MIDI note number for A4
pub const A4_MIDI: u8 = 69;

/// Semitones in octave
pub const SWAR_PER_SAPTAK: u8 = 12;

/// Note names (Western)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwarNaam {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
}

/// Indian swaras (Hindustani)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BharatiyaSwar {
    Sa,         // Shadja
    ReKomal,    // Komal Rishabh
    ReShuddha,  // Shuddha Rishabh
    GaKomal,    // Komal Gandhar
    GaShuddha,  // Shuddha Gandhar
    Ma,         // Shuddha Madhyam
    MaTivra,    // Tivra Madhyam
    Pa,         // Pancham
    DhaKomal,   // Komal Dhaivat
    DhaShuddha, // Shuddha Dhaivat
    NiKomal,    // Komal Nishad
    NiShuddha,  // Shuddha Nishad
}

impl SwarNaam {
    /// Get semitone offset from C
    pub const fn offset(&self) -> i32 {
        match self {
            Self::C => 0,
            Self::Cs => 1,
            Self::D => 2,
            Self::Ds => 3,
            Self::E => 4,
            Self::F => 5,
            Self::Fs => 6,
            Self::G => 7,
            Self::Gs => 8,
            Self::A => 9,
            Self::As => 10,
            Self::B => 11,
        }
    }

    /// From semitone offset
    pub const fn offset_se(offset: i32) -> Self {
        match offset.rem_euclid(12) {
            0 => Self::C,
            1 => Self::Cs,
            2 => Self::D,
            3 => Self::Ds,
            4 => Self::E,
            5 => Self::F,
            6 => Self::Fs,
            7 => Self::G,
            8 => Self::Gs,
            9 => Self::A,
            10 => Self::As,
            _ => Self::B,
        }
    }
}

/// Musical note with octave
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Swar {
    pub naam: SwarNaam,
    pub saptak: i32, // Octave (4 is middle)
}

impl Swar {
    /// Create note
    pub const fn nava(naam: SwarNaam, saptak: i32) -> Self {
        Self { naam, saptak }
    }

    /// Middle C
    pub const fn madhya_c() -> Self {
        Self::nava(SwarNaam::C, 4)
    }

    /// Concert A
    pub const fn sangeet_a() -> Self {
        Self::nava(SwarNaam::A, 4)
    }

    /// Get MIDI note number
    pub const fn midi(&self) -> i32 {
        12 + self.saptak * 12 + self.naam.offset()
    }

    /// Create from MIDI note number
    pub const fn midi_se(midi: i32) -> Self {
        let saptak = (midi / 12) - 1;
        let note_idx = midi % 12;
        Self {
            naam: SwarNaam::offset_se(note_idx),
            saptak,
        }
    }

    /// Get frequency in Hz (equal temperament)
    pub fn aavriti(&self) -> f64 {
        let semitones_from_a4 = self.midi() - A4_MIDI as i32;
        A4_AAVRITI * libm::pow(2.0, semitones_from_a4 as f64 / 12.0)
    }

    /// Transpose by semitones
    pub const fn sthanan(&self, ardhaswar: i32) -> Self {
        Self::midi_se(self.midi() + ardhaswar)
    }

    /// Get interval to another note
    pub const fn antar(&self, other: &Self) -> i32 {
        other.midi() - self.midi()
    }
}

/// Scale patterns (intervals from root)
pub mod maapdand {
    /// Major scale intervals
    pub const PRAMUKH: [i32; 7] = [0, 2, 4, 5, 7, 9, 11];

    /// Natural minor scale
    pub const GAUNIKA: [i32; 7] = [0, 2, 3, 5, 7, 8, 10];

    /// Harmonic minor
    pub const SWARVADI_GAUNIKA: [i32; 7] = [0, 2, 3, 5, 7, 8, 11];

    /// Melodic minor (ascending)
    pub const RAAGIK_GAUNIKA: [i32; 7] = [0, 2, 3, 5, 7, 9, 11];

    /// Pentatonic major
    pub const PANCHA_PRAMUKH: [i32; 5] = [0, 2, 4, 7, 9];

    /// Pentatonic minor
    pub const PANCHA_GAUNIKA: [i32; 5] = [0, 3, 5, 7, 10];

    /// Blues scale
    pub const NEELA: [i32; 6] = [0, 3, 5, 6, 7, 10];

    /// Chromatic
    pub const RANGEEN: [i32; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];

    /// Whole tone
    pub const PURN_SWAR: [i32; 6] = [0, 2, 4, 6, 8, 10];

    /// Dorian mode
    pub const DORIAN: [i32; 7] = [0, 2, 3, 5, 7, 9, 10];

    /// Phrygian mode
    pub const PHRYGIAN: [i32; 7] = [0, 1, 3, 5, 7, 8, 10];

    /// Lydian mode
    pub const LYDIAN: [i32; 7] = [0, 2, 4, 6, 7, 9, 11];

    /// Mixolydian mode
    pub const MIXOLYDIAN: [i32; 7] = [0, 2, 4, 5, 7, 9, 10];
}

/// Common Indian Ragas (just the thaat/parent scale)
pub mod raaga {
    /// Bilaval thaat (Major)
    pub const BILAVAL: [i32; 7] = [0, 2, 4, 5, 7, 9, 11];

    /// Khamaj thaat
    pub const KHAMAJ: [i32; 7] = [0, 2, 4, 5, 7, 9, 10];

    /// Kafi thaat
    pub const KAFI: [i32; 7] = [0, 2, 3, 5, 7, 9, 10];

    /// Asavari thaat
    pub const ASAVARI: [i32; 7] = [0, 2, 3, 5, 7, 8, 10];

    /// Bhairavi thaat
    pub const BHAIRAVI: [i32; 7] = [0, 1, 3, 5, 7, 8, 10];

    /// Bhairav thaat
    pub const BHAIRAV: [i32; 7] = [0, 1, 4, 5, 7, 8, 11];

    /// Kalyan thaat (Lydian)
    pub const KALYAN: [i32; 7] = [0, 2, 4, 6, 7, 9, 11];

    /// Marwa thaat
    pub const MARWA: [i32; 7] = [0, 1, 4, 6, 7, 9, 11];

    /// Poorvi thaat
    pub const POORVI: [i32; 7] = [0, 1, 4, 6, 7, 8, 11];

    /// Todi thaat
    pub const TODI: [i32; 7] = [0, 1, 3, 6, 7, 8, 11];
}

/// Chord intervals from root
pub mod taranga {
    /// Major triad
    pub const PRAMUKH: [i32; 3] = [0, 4, 7];

    /// Minor triad
    pub const GAUNIKA: [i32; 3] = [0, 3, 7];

    /// Diminished triad
    pub const HRASTA: [i32; 3] = [0, 3, 6];

    /// Augmented triad
    pub const VRIDDHI: [i32; 3] = [0, 4, 8];

    /// Major seventh
    pub const PRAMUKH7: [i32; 4] = [0, 4, 7, 11];

    /// Minor seventh
    pub const GAUNIKA7: [i32; 4] = [0, 3, 7, 10];

    /// Dominant seventh
    pub const PRABAL7: [i32; 4] = [0, 4, 7, 10];

    /// Suspended 2nd
    pub const SUS2: [i32; 3] = [0, 2, 7];

    /// Suspended 4th
    pub const SUS4: [i32; 3] = [0, 5, 7];
}

/// Convert frequency to MIDI note (float for pitch bend)
pub fn aavriti_se_midi(aavriti: f64) -> f64 {
    A4_MIDI as f64 + 12.0 * libm::log2(aavriti / A4_AAVRITI)
}

/// Convert frequency to note and cents offset
pub fn aavriti_se_swar(aavriti: f64) -> (Swar, f64) {
    let midi_float = aavriti_se_midi(aavriti);
    let midi_int = libm::round(midi_float) as i32;
    let cents = (midi_float - midi_int as f64) * 100.0;
    (Swar::midi_se(midi_int), cents)
}

/// Beat frequency between two tones
pub fn taal_aavriti(f1: f64, f2: f64) -> f64 {
    (f1 - f2).abs()
}

/// Calculate tempo in BPM from beat period
pub fn taal_gati(avadhi_ms: f64) -> f64 {
    60000.0 / avadhi_ms
}

/// Calculate period from tempo
pub fn taal_avadhi(bpm: f64) -> f64 {
    60000.0 / bpm
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a4_frequency() {
        let a4 = Swar::sangeet_a();
        assert!((a4.aavriti() - 440.0).abs() < 0.01);
    }

    #[test]
    fn test_middle_c() {
        let c4 = Swar::madhya_c();
        assert!((c4.aavriti() - 261.63).abs() < 0.1);
    }

    #[test]
    fn test_octave() {
        let a4 = Swar::sangeet_a();
        let a5 = a4.sthanan(12);
        assert!((a5.aavriti() - 880.0).abs() < 0.01);
    }

    #[test]
    fn test_midi_conversion() {
        let note = Swar::midi_se(60);
        assert_eq!(note.naam, SwarNaam::C);
        assert_eq!(note.saptak, 4);
    }

    #[test]
    fn test_freq_to_midi() {
        let midi = aavriti_se_midi(440.0);
        assert!((midi - 69.0).abs() < 0.01);
    }
}
