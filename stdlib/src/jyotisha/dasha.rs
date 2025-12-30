//! # Daśā - Planetary Periods (दशा)
//!
//! > **"दशा ग्रहाणां कालविभागः"**
//! > *"Daśā is the division of time among planets"*
//!
//! The Daśā system divides life/project cycles into periods ruled by different Grahas.
//! The most common system is Viṃśottarī Daśā (120-year cycle).
//!
//! ## Viṃśottarī Daśā Periods (Total: 120 years)
//!
//! | Graha | Years | Cumulative |
//! |-------|-------|------------|
//! | Ketu | 7 | 7 |
//! | Śukra | 20 | 27 |
//! | Sūrya | 6 | 33 |
//! | Chandra | 10 | 43 |
//! | Maṅgala | 7 | 50 |
//! | Rāhu | 18 | 68 |
//! | Guru | 16 | 84 |
//! | Śani | 19 | 103 |
//! | Budha | 17 | 120 |
//!
//! ## Computational Applications
//!
//! - **Project Planning**: Map project phases to Daśā periods
//! - **Resource Allocation**: Planetary strengths guide resource priorities
//! - **Timing Optimization**: Know when different optimizations are most effective

use super::graha::Graha;
use super::nakshatra::Nakshatra;

/// Viṃśottarī Daśā periods (years per Graha)
pub const VIMSHOTTARI_DASHA: [(Graha, u8); 9] = [
    (Graha::Ketu, 7),
    (Graha::Shukra, 20),
    (Graha::Surya, 6),
    (Graha::Chandra, 10),
    (Graha::Mangala, 7),
    (Graha::Rahu, 18),
    (Graha::Guru, 16),
    (Graha::Shani, 19),
    (Graha::Budha, 17),
];

/// Total Viṃśottarī cycle length (years)
pub const VIMSHOTTARI_TOTAL: u16 = 120;

/// Daśā sequence starting from a Nakṣatra
pub const DASHA_SEQUENCE: [Graha; 9] = [
    Graha::Ketu,
    Graha::Shukra,
    Graha::Surya,
    Graha::Chandra,
    Graha::Mangala,
    Graha::Rahu,
    Graha::Guru,
    Graha::Shani,
    Graha::Budha,
];

/// A Mahā Daśā period (major period)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MahaDasha {
    /// Ruling Graha
    pub graha: Graha,
    /// Start time (normalized 0.0-1.0 in 120-year cycle)
    pub start: f64,
    /// Duration in years
    pub years: u8,
}

/// An Antar Daśā period (sub-period within Mahā Daśā)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AntarDasha {
    /// Mahā Daśā lord
    pub maha_lord: Graha,
    /// Antar Daśā lord (within Mahā Daśā)
    pub antar_lord: Graha,
    /// Duration in days (approximate)
    pub days: f64,
}

/// A Pratyāntar Daśā period (sub-sub-period)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PratyantarDasha {
    /// Mahā Daśā lord
    pub maha_lord: Graha,
    /// Antar Daśā lord
    pub antar_lord: Graha,
    /// Pratyāntar Daśā lord
    pub pratyantar_lord: Graha,
    /// Duration in days
    pub days: f64,
}

/// Daśā calculator
#[derive(Debug, Clone)]
pub struct DashaCalculator {
    /// Birth Nakṣatra (determines starting Daśā)
    pub janma_nakshatra: Nakshatra,
    /// Position within Nakṣatra at birth (0.0-1.0)
    pub nakshatra_balance: f64,
}

impl DashaCalculator {
    /// Create a new Daśā calculator from birth Nakṣatra
    pub fn new(janma_nakshatra: Nakshatra, nakshatra_balance: f64) -> Self {
        Self {
            janma_nakshatra,
            nakshatra_balance: nakshatra_balance.clamp(0.0, 1.0),
        }
    }

    /// Get the starting Daśā lord based on birth Nakṣatra
    pub fn starting_dasha_lord(&self) -> Graha {
        self.janma_nakshatra.ruling_graha()
    }

    /// Calculate balance of first Daśā at birth (remaining years)
    pub fn first_dasha_balance(&self) -> f64 {
        let lord = self.starting_dasha_lord();
        let total_years = lord.dasha_years() as f64;
        total_years * (1.0 - self.nakshatra_balance)
    }

    /// Get Mahā Daśā sequence from birth
    pub fn maha_dasha_sequence(&self) -> [MahaDasha; 9] {
        let start_lord = self.starting_dasha_lord();
        let start_idx = DASHA_SEQUENCE
            .iter()
            .position(|&g| g == start_lord)
            .unwrap_or(0);

        let mut result = [MahaDasha {
            graha: Graha::Ketu,
            start: 0.0,
            years: 0,
        }; 9];

        let mut cumulative_start = 0.0;
        let first_balance = self.first_dasha_balance();

        for i in 0..9 {
            let idx = (start_idx + i) % 9;
            let graha = DASHA_SEQUENCE[idx];
            let years = graha.dasha_years();

            let actual_years = if i == 0 { first_balance } else { years as f64 };

            result[i] = MahaDasha {
                graha,
                start: cumulative_start,
                years,
            };

            cumulative_start += actual_years;
        }

        result
    }

    /// Calculate Antar Daśā periods within a Mahā Daśā
    pub fn antar_dasha_in_maha(&self, maha_lord: Graha) -> alloc::vec::Vec<AntarDasha> {
        let maha_years = maha_lord.dasha_years() as f64;
        let start_idx = DASHA_SEQUENCE
            .iter()
            .position(|&g| g == maha_lord)
            .unwrap_or(0);

        let mut result = alloc::vec::Vec::with_capacity(9);

        for i in 0..9 {
            let idx = (start_idx + i) % 9;
            let antar_lord = DASHA_SEQUENCE[idx];
            let antar_years = antar_lord.dasha_years() as f64;

            // Antar Daśā duration = (Mahā years × Antar years) / 120
            let antar_duration_years = (maha_years * antar_years) / 120.0;
            let days = antar_duration_years * 365.25;

            result.push(AntarDasha {
                maha_lord,
                antar_lord,
                days,
            });
        }

        result
    }

    /// Find current Daśā for a given age (in years)
    pub fn dasha_at_age(&self, age: f64) -> Option<(Graha, f64)> {
        let mut remaining = age;
        let start_lord = self.starting_dasha_lord();
        let start_idx = DASHA_SEQUENCE
            .iter()
            .position(|&g| g == start_lord)
            .unwrap_or(0);

        // Handle first partial Daśā
        let first_balance = self.first_dasha_balance();
        if remaining < first_balance {
            let progress = remaining / first_balance;
            return Some((start_lord, progress));
        }
        remaining -= first_balance;

        // Iterate through remaining Daśās
        for i in 1..9 {
            let idx = (start_idx + i) % 9;
            let graha = DASHA_SEQUENCE[idx];
            let years = graha.dasha_years() as f64;

            if remaining < years {
                let progress = remaining / years;
                return Some((graha, progress));
            }
            remaining -= years;
        }

        // Beyond 120 years, wrap around
        self.dasha_at_age(age % 120.0)
    }
}

extern crate alloc;

/// Daśā interpretation for computational contexts
pub fn dasha_optimization_guidance(graha: Graha) -> &'static str {
    match graha {
        Graha::Ketu => "Focus on cleanup, dead code elimination, releasing old patterns",
        Graha::Shukra => "Prioritize elegance, refactoring, code aesthetics",
        Graha::Surya => "Concentrate on core execution, main thread optimization",
        Graha::Chandra => "Optimize memory flow, caching strategies, state management",
        Graha::Mangala => "Push for aggressive optimization, parallelization, performance",
        Graha::Rahu => "Handle async complexity, concurrency, shadow operations",
        Graha::Guru => "Apply wisdom-based optimizations, algorithmic improvements",
        Graha::Shani => "Focus on resource limits, restrictions, minimal footprint",
        Graha::Budha => "Improve type inference, communication, API clarity",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vimshottari_total() {
        let total: u16 = VIMSHOTTARI_DASHA.iter().map(|(_, y)| *y as u16).sum();
        assert_eq!(total, VIMSHOTTARI_TOTAL);
    }

    #[test]
    fn test_dasha_years() {
        assert_eq!(Graha::Ketu.dasha_years(), 7);
        assert_eq!(Graha::Shukra.dasha_years(), 20);
        assert_eq!(Graha::Shani.dasha_years(), 19);
    }

    #[test]
    fn test_dasha_calculator_creation() {
        let calc = DashaCalculator::new(Nakshatra::Ashvini, 0.5);
        assert_eq!(calc.starting_dasha_lord(), Graha::Ketu);
    }

    #[test]
    fn test_first_dasha_balance() {
        let calc = DashaCalculator::new(Nakshatra::Ashvini, 0.0);
        assert_eq!(calc.first_dasha_balance(), 7.0); // Full Ketu Daśā

        let calc2 = DashaCalculator::new(Nakshatra::Ashvini, 0.5);
        assert_eq!(calc2.first_dasha_balance(), 3.5); // Half Ketu Daśā
    }

    #[test]
    fn test_maha_dasha_sequence() {
        let calc = DashaCalculator::new(Nakshatra::Ashvini, 0.0);
        let sequence = calc.maha_dasha_sequence();

        assert_eq!(sequence[0].graha, Graha::Ketu);
        assert_eq!(sequence[1].graha, Graha::Shukra);
        assert_eq!(sequence[8].graha, Graha::Budha);
    }

    #[test]
    fn test_antar_dasha() {
        let calc = DashaCalculator::new(Nakshatra::Ashvini, 0.0);
        let antars = calc.antar_dasha_in_maha(Graha::Shukra);

        assert_eq!(antars.len(), 9);
        assert_eq!(antars[0].maha_lord, Graha::Shukra);
        assert_eq!(antars[0].antar_lord, Graha::Shukra);
    }

    #[test]
    fn test_dasha_at_age() {
        let calc = DashaCalculator::new(Nakshatra::Ashvini, 0.0);

        let (lord, _) = calc.dasha_at_age(3.5).unwrap();
        assert_eq!(lord, Graha::Ketu); // Still in first Daśā

        let (lord2, _) = calc.dasha_at_age(10.0).unwrap();
        assert_eq!(lord2, Graha::Shukra); // Moved to second Daśā
    }
}
