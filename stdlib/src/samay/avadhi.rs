//! # Avadhi - Duration Arithmetic (अवधि)
//!
//! Time duration utilities and arithmetic.

use core::ops::{Add, Sub, Mul, Div};

/// Time unit
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SamayEkak {
    Nanominut,      // Nanoseconds
    Minut,          // Microseconds
    Miliminut,      // Milliseconds
    Palak,          // Seconds
    Nimish,         // Minutes
    Hora,           // Hours
    Divas,          // Days
    Saptah,         // Weeks
}

/// Duration value
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct Avadhi {
    /// Total nanoseconds
    nano: i128,
}

impl Avadhi {
    /// Zero duration
    pub const fn shunya() -> Self {
        Self { nano: 0 }
    }

    /// Create from nanoseconds
    pub const fn nano_se(n: i128) -> Self {
        Self { nano: n }
    }

    /// Create from microseconds
    pub const fn minut_se(n: i64) -> Self {
        Self { nano: n as i128 * 1000 }
    }

    /// Create from milliseconds
    pub const fn mili_se(n: i64) -> Self {
        Self { nano: n as i128 * 1_000_000 }
    }

    /// Create from seconds
    pub const fn palak_se(n: i64) -> Self {
        Self { nano: n as i128 * 1_000_000_000 }
    }

    /// Create from minutes
    pub const fn nimish_se(n: i64) -> Self {
        Self { nano: n as i128 * 60 * 1_000_000_000 }
    }

    /// Create from hours
    pub const fn hora_se(n: i64) -> Self {
        Self { nano: n as i128 * 60 * 60 * 1_000_000_000 }
    }

    /// Create from days
    pub const fn divas_se(n: i64) -> Self {
        Self { nano: n as i128 * 24 * 60 * 60 * 1_000_000_000 }
    }

    /// Create from weeks
    pub const fn saptah_se(n: i64) -> Self {
        Self { nano: n as i128 * 7 * 24 * 60 * 60 * 1_000_000_000 }
    }

    /// Create from fractional seconds
    pub fn palak_se_f64(secs: f64) -> Self {
        Self { nano: (secs * 1_000_000_000.0) as i128 }
    }

    /// Get total nanoseconds
    pub const fn nano_mein(&self) -> i128 {
        self.nano
    }

    /// Get total microseconds
    pub const fn minut_mein(&self) -> i64 {
        (self.nano / 1000) as i64
    }

    /// Get total milliseconds
    pub const fn mili_mein(&self) -> i64 {
        (self.nano / 1_000_000) as i64
    }

    /// Get total seconds (truncated)
    pub const fn palak_mein(&self) -> i64 {
        (self.nano / 1_000_000_000) as i64
    }

    /// Get fractional seconds
    pub fn palak_f64(&self) -> f64 {
        self.nano as f64 / 1_000_000_000.0
    }

    /// Get total minutes (truncated)
    pub const fn nimish_mein(&self) -> i64 {
        self.palak_mein() / 60
    }

    /// Get total hours (truncated)
    pub const fn hora_mein(&self) -> i64 {
        self.palak_mein() / 3600
    }

    /// Get total days (truncated)
    pub const fn divas_mein(&self) -> i64 {
        self.palak_mein() / 86400
    }

    /// Get total weeks (truncated)
    pub const fn saptah_mein(&self) -> i64 {
        self.divas_mein() / 7
    }

    /// Absolute value
    pub fn nirapeksha(&self) -> Self {
        Self { nano: if self.nano < 0 { -self.nano } else { self.nano } }
    }

    /// Check if zero
    pub const fn shunya_hai(&self) -> bool {
        self.nano == 0
    }

    /// Check if negative
    pub const fn rin_hai(&self) -> bool {
        self.nano < 0
    }

    /// Check if positive
    pub const fn dhana_hai(&self) -> bool {
        self.nano > 0
    }

    /// Split into components (days, hours, minutes, seconds, milliseconds)
    pub fn vibhajit(&self) -> (i64, u8, u8, u8, u16) {
        let total_ms = self.mili_mein().abs();
        let ms = (total_ms % 1000) as u16;
        let total_sec = total_ms / 1000;
        let sec = (total_sec % 60) as u8;
        let total_min = total_sec / 60;
        let min = (total_min % 60) as u8;
        let total_hr = total_min / 60;
        let hr = (total_hr % 24) as u8;
        let days = total_hr / 24;

        (if self.nano < 0 { -days } else { days }, hr, min, sec, ms)
    }

    /// Minimum of two durations
    pub fn nyunatam(&self, other: &Self) -> Self {
        if self.nano < other.nano { *self } else { *other }
    }

    /// Maximum of two durations
    pub fn adhikatam(&self, other: &Self) -> Self {
        if self.nano > other.nano { *self } else { *other }
    }
}

impl Add for Avadhi {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self { nano: self.nano + rhs.nano }
    }
}

impl Sub for Avadhi {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self { nano: self.nano - rhs.nano }
    }
}

impl Mul<i64> for Avadhi {
    type Output = Self;
    fn mul(self, rhs: i64) -> Self {
        Self { nano: self.nano * rhs as i128 }
    }
}

impl Mul<f64> for Avadhi {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self { nano: (self.nano as f64 * rhs) as i128 }
    }
}

impl Div<i64> for Avadhi {
    type Output = Self;
    fn div(self, rhs: i64) -> Self {
        Self { nano: self.nano / rhs as i128 }
    }
}

impl Div<Avadhi> for Avadhi {
    type Output = f64;
    fn div(self, rhs: Avadhi) -> f64 {
        self.nano as f64 / rhs.nano as f64
    }
}

/// Stopwatch for measuring elapsed time
#[derive(Debug, Clone)]
pub struct Ghadi {
    prarambh: Option<i128>,
    sankalita: i128,
    chal_raha: bool,
}

impl Ghadi {
    /// Create new stopwatch
    pub fn nava() -> Self {
        Self {
            prarambh: None,
            sankalita: 0,
            chal_raha: false,
        }
    }

    /// Start the stopwatch (provide current timestamp in nanoseconds)
    pub fn shuru(&mut self, vartamaan: i128) {
        if !self.chal_raha {
            self.prarambh = Some(vartamaan);
            self.chal_raha = true;
        }
    }

    /// Stop the stopwatch
    pub fn rok(&mut self, vartamaan: i128) {
        if self.chal_raha {
            if let Some(start) = self.prarambh {
                self.sankalita += vartamaan - start;
            }
            self.chal_raha = false;
            self.prarambh = None;
        }
    }

    /// Get elapsed time
    pub fn vyatit(&self, vartamaan: i128) -> Avadhi {
        let running_time = if self.chal_raha {
            self.prarambh.map(|s| vartamaan - s).unwrap_or(0)
        } else {
            0
        };
        Avadhi::nano_se(self.sankalita + running_time)
    }

    /// Reset stopwatch
    pub fn punasthapana(&mut self) {
        self.prarambh = None;
        self.sankalita = 0;
        self.chal_raha = false;
    }

    /// Is running?
    pub fn chal_raha_hai(&self) -> bool {
        self.chal_raha
    }
}

/// Timer that counts down
#[derive(Debug, Clone)]
pub struct Antarganak {
    lakshya: i128,        // Target end time (nanoseconds)
    avadhi: i128,         // Total duration
    shuru_samay: i128,    // Start timestamp
    sthagit: bool,
}

impl Antarganak {
    /// Create timer with duration (provide start timestamp)
    pub fn nava(avadhi: Avadhi, vartamaan: i128) -> Self {
        Self {
            lakshya: vartamaan + avadhi.nano,
            avadhi: avadhi.nano,
            shuru_samay: vartamaan,
            sthagit: false,
        }
    }

    /// Time remaining
    pub fn shesh(&self, vartamaan: i128) -> Avadhi {
        if self.sthagit {
            Avadhi::nano_se(self.lakshya - self.shuru_samay)
        } else {
            let remaining = self.lakshya - vartamaan;
            Avadhi::nano_se(if remaining > 0 { remaining } else { 0 })
        }
    }

    /// Check if expired
    pub fn samaapt(&self, vartamaan: i128) -> bool {
        !self.sthagit && vartamaan >= self.lakshya
    }

    /// Progress (0.0 to 1.0)
    pub fn pragati(&self, vartamaan: i128) -> f64 {
        if self.avadhi == 0 {
            return 1.0;
        }
        let elapsed = vartamaan - self.shuru_samay;
        let progress = elapsed as f64 / self.avadhi as f64;
        if progress < 0.0 { 0.0 } else if progress > 1.0 { 1.0 } else { progress }
    }

    /// Pause timer
    pub fn sthagit_karo(&mut self) {
        self.sthagit = true;
    }

    /// Resume timer (provide current time)
    pub fn punah_shuru(&mut self, vartamaan: i128) {
        if self.sthagit {
            self.lakshya = vartamaan + (self.lakshya - self.shuru_samay);
            self.shuru_samay = vartamaan;
            self.sthagit = false;
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
    fn test_duration_creation() {
        assert_eq!(Avadhi::palak_se(1).mili_mein(), 1000);
        assert_eq!(Avadhi::nimish_se(1).palak_mein(), 60);
        assert_eq!(Avadhi::hora_se(1).nimish_mein(), 60);
    }

    #[test]
    fn test_duration_arithmetic() {
        let a = Avadhi::palak_se(10);
        let b = Avadhi::palak_se(5);
        assert_eq!((a + b).palak_mein(), 15);
        assert_eq!((a - b).palak_mein(), 5);
        assert_eq!((a * 2).palak_mein(), 20);
    }

    #[test]
    fn test_split() {
        let d = Avadhi::hora_se(25) + Avadhi::nimish_se(30) + Avadhi::palak_se(45);
        let (days, hrs, mins, secs, _) = d.vibhajit();
        assert_eq!(days, 1);
        assert_eq!(hrs, 1);
        assert_eq!(mins, 30);
        assert_eq!(secs, 45);
    }

    #[test]
    fn test_stopwatch() {
        let mut sw = Ghadi::nava();
        sw.shuru(0);
        let elapsed = sw.vyatit(1_000_000_000);
        assert_eq!(elapsed.palak_mein(), 1);

        sw.rok(2_000_000_000);
        let elapsed = sw.vyatit(5_000_000_000);
        assert_eq!(elapsed.palak_mein(), 2);
    }
}
