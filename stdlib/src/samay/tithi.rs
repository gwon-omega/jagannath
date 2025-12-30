//! # Tithi - Calendar Dates (तिथि)
//!
//! Calendar date utilities and manipulation.

/// Helper function: days in month (accounting for leap year)
fn din_maas_mein(varsh: i32, maas: u8) -> u8 {
    match maas {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            let leap = (varsh % 4 == 0 && varsh % 100 != 0) || (varsh % 400 == 0);
            if leap { 29 } else { 28 }
        }
        _ => 30,
    }
}

/// Days of week
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Vaara {
    Ravivaar,   // Sunday
    Somvaar,    // Monday
    Mangalvaar, // Tuesday
    Budhvaar,   // Wednesday
    Guruvaar,   // Thursday
    Shukravaar, // Friday
    Shanivaar,  // Saturday
}

impl Vaara {
    /// Get day number (0 = Sunday)
    pub const fn sankhya(&self) -> u8 {
        match self {
            Self::Ravivaar => 0,
            Self::Somvaar => 1,
            Self::Mangalvaar => 2,
            Self::Budhvaar => 3,
            Self::Guruvaar => 4,
            Self::Shukravaar => 5,
            Self::Shanivaar => 6,
        }
    }

    /// From day number
    pub const fn sankhya_se(n: u8) -> Self {
        match n % 7 {
            0 => Self::Ravivaar,
            1 => Self::Somvaar,
            2 => Self::Mangalvaar,
            3 => Self::Budhvaar,
            4 => Self::Guruvaar,
            5 => Self::Shukravaar,
            _ => Self::Shanivaar,
        }
    }

    /// Is weekend?
    pub const fn saptahant(&self) -> bool {
        matches!(self, Self::Shanivaar | Self::Ravivaar)
    }

    /// Is weekday?
    pub const fn karyakar_divas(&self) -> bool {
        !self.saptahant()
    }

    /// Next day
    pub const fn agla(&self) -> Self {
        Self::sankhya_se(self.sankhya() + 1)
    }

    /// Previous day
    pub const fn pichla(&self) -> Self {
        Self::sankhya_se(self.sankhya().wrapping_sub(1))
    }
}

/// Months of year
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Maas {
    Maagh,       // January
    Phaalgun,    // February
    Chaitra,     // March
    Vaishaakh,   // April
    Jyeshtha,    // May
    Aashaadh,    // June
    Shraavan,    // July
    Bhaadrapad,  // August
    Ashwin,      // September
    Kaartik,     // October
    Maargshirsh, // November
    Paush,       // December
}

impl Maas {
    /// Get month number (1-12)
    pub const fn sankhya(&self) -> u8 {
        match self {
            Self::Maagh => 1,
            Self::Phaalgun => 2,
            Self::Chaitra => 3,
            Self::Vaishaakh => 4,
            Self::Jyeshtha => 5,
            Self::Aashaadh => 6,
            Self::Shraavan => 7,
            Self::Bhaadrapad => 8,
            Self::Ashwin => 9,
            Self::Kaartik => 10,
            Self::Maargshirsh => 11,
            Self::Paush => 12,
        }
    }

    /// From month number (1-12)
    pub const fn sankhya_se(n: u8) -> Option<Self> {
        match n {
            1 => Some(Self::Maagh),
            2 => Some(Self::Phaalgun),
            3 => Some(Self::Chaitra),
            4 => Some(Self::Vaishaakh),
            5 => Some(Self::Jyeshtha),
            6 => Some(Self::Aashaadh),
            7 => Some(Self::Shraavan),
            8 => Some(Self::Bhaadrapad),
            9 => Some(Self::Ashwin),
            10 => Some(Self::Kaartik),
            11 => Some(Self::Maargshirsh),
            12 => Some(Self::Paush),
            _ => None,
        }
    }

    /// Days in month (non-leap year)
    pub const fn divas_sankhya(&self) -> u8 {
        match self {
            Self::Maagh
            | Self::Chaitra
            | Self::Jyeshtha
            | Self::Shraavan
            | Self::Kaartik
            | Self::Paush => 31,
            Self::Vaishaakh | Self::Aashaadh | Self::Ashwin | Self::Maargshirsh => 30,
            Self::Phaalgun => 28,
            Self::Bhaadrapad => 31,
        }
    }

    /// Days in month accounting for leap year
    pub const fn divas_varsh_mein(&self, adhik_varsh: bool) -> u8 {
        if matches!(self, Self::Phaalgun) && adhik_varsh {
            29
        } else {
            self.divas_sankhya()
        }
    }
}

/// Calendar date
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tithi {
    pub varsh: i32, // Year
    pub maas: u8,   // Month (1-12)
    pub divas: u8,  // Day (1-31)
}

impl Tithi {
    /// Create new date
    pub const fn nava(varsh: i32, maas: u8, divas: u8) -> Option<Self> {
        if maas < 1 || maas > 12 || divas < 1 || divas > 31 {
            return None;
        }
        Some(Self { varsh, maas, divas })
    }

    /// Check if year is leap year
    pub const fn adhik_varsh_hai(varsh: i32) -> bool {
        (varsh % 4 == 0 && varsh % 100 != 0) || (varsh % 400 == 0)
    }

    /// Is this date's year a leap year?
    pub const fn adhik_varsh(&self) -> bool {
        Self::adhik_varsh_hai(self.varsh)
    }

    /// Days in month for this date
    pub const fn maas_divas(&self) -> u8 {
        match self.maas {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11 => 30,
            2 => {
                if self.adhik_varsh() {
                    29
                } else {
                    28
                }
            }
            _ => 0,
        }
    }

    /// Validate date
    pub const fn vaidh_hai(&self) -> bool {
        self.maas >= 1 && self.maas <= 12 && self.divas >= 1 && self.divas <= self.maas_divas()
    }

    /// Day of year (1-366)
    pub fn varsh_divas(&self) -> u16 {
        let days_before_month = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];
        let mut day = days_before_month[(self.maas - 1) as usize] + self.divas as u16;
        if self.maas > 2 && self.adhik_varsh() {
            day += 1;
        }
        day
    }

    /// Day of week (Zeller's congruence)
    pub fn vaara(&self) -> Vaara {
        let mut y = self.varsh;
        let mut m = self.maas as i32;

        if m < 3 {
            m += 12;
            y -= 1;
        }

        let q = self.divas as i32;
        let k = y % 100;
        let j = y / 100;

        let h = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
        let h = ((h + 7) % 7) as u8;

        // Convert: 0=Sat, 1=Sun, 2=Mon...
        Vaara::sankhya_se((h + 6) % 7)
    }

    /// Days since epoch (January 1, year 1)
    pub fn yugantar_divas(&self) -> i64 {
        let y = self.varsh as i64 - 1;
        let mut days = y * 365 + y / 4 - y / 100 + y / 400;
        days += self.varsh_divas() as i64;
        days
    }

    /// Add days
    pub fn divas_jod(&self, n: i32) -> Self {
        // Simple iterative approach
        let mut varsh = self.varsh;
        let mut maas = self.maas;
        let mut divas = self.divas as i32 + n;

        // Handle positive days
        while divas > 0 {
            let days_in_month = din_maas_mein(varsh, maas) as i32;
            if divas <= days_in_month {
                break;
            }
            divas -= days_in_month;
            maas += 1;
            if maas > 12 {
                maas = 1;
                varsh += 1;
            }
        }

        // Handle negative days (going backward)
        while divas <= 0 {
            maas = if maas == 1 { 12 } else { maas - 1 };
            if maas == 12 {
                varsh -= 1;
            }
            divas += din_maas_mein(varsh, maas) as i32;
        }

        Self {
            varsh,
            maas,
            divas: divas as u8,
        }
    }

    /// Subtract days
    pub fn divas_ghatao(&self, n: i32) -> Self {
        self.divas_jod(-n)
    }

    /// Days between two dates
    pub fn divas_antar(&self, other: &Self) -> i64 {
        other.yugantar_divas() - self.yugantar_divas()
    }

    /// Next day
    pub fn agla(&self) -> Self {
        self.divas_jod(1)
    }

    /// Previous day
    pub fn pichla(&self) -> Self {
        self.divas_ghatao(1)
    }

    /// Is before another date?
    pub fn pehle(&self, other: &Self) -> bool {
        self.yugantar_divas() < other.yugantar_divas()
    }

    /// Is after another date?
    pub fn baad(&self, other: &Self) -> bool {
        self.yugantar_divas() > other.yugantar_divas()
    }
}

impl PartialOrd for Tithi {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.yugantar_divas().cmp(&other.yugantar_divas()))
    }
}

impl Ord for Tithi {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.yugantar_divas().cmp(&other.yugantar_divas())
    }
}

/// Week number in year (ISO 8601)
pub fn saptah_sankhya(tithi: &Tithi) -> u8 {
    let jan1 = Tithi::nava(tithi.varsh, 1, 1).unwrap();
    let jan1_vaara = jan1.vaara().sankhya();

    // ISO week starts on Monday (1)
    let correction = if jan1_vaara == 0 { 6 } else { jan1_vaara - 1 };

    let day_of_year = tithi.varsh_divas() as i32;
    let week = (day_of_year + correction as i32 - 1) / 7 + 1;

    week as u8
}

/// Quarter of year (1-4)
pub const fn trimaas(maas: u8) -> u8 {
    (maas - 1) / 3 + 1
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leap_year() {
        assert!(Tithi::adhik_varsh_hai(2000));
        assert!(!Tithi::adhik_varsh_hai(1900));
        assert!(Tithi::adhik_varsh_hai(2024));
        assert!(!Tithi::adhik_varsh_hai(2023));
    }

    #[test]
    fn test_day_of_week() {
        // January 1, 2024 is Monday
        let t = Tithi::nava(2024, 1, 1).unwrap();
        assert_eq!(t.vaara(), Vaara::Somvaar);
    }

    #[test]
    fn test_add_days() {
        let t = Tithi::nava(2024, 1, 30).unwrap();
        let next = t.divas_jod(5);
        assert_eq!(next.maas, 2);
        assert_eq!(next.divas, 4);
    }

    #[test]
    fn test_day_of_year() {
        let jan1 = Tithi::nava(2024, 1, 1).unwrap();
        assert_eq!(jan1.varsh_divas(), 1);

        let dec31 = Tithi::nava(2024, 12, 31).unwrap();
        assert_eq!(dec31.varsh_divas(), 366); // Leap year
    }

    #[test]
    fn test_days_between() {
        let t1 = Tithi::nava(2024, 1, 1).unwrap();
        let t2 = Tithi::nava(2024, 1, 10).unwrap();
        assert_eq!(t1.divas_antar(&t2), 9);
    }
}
