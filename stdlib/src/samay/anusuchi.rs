//! # Anusuchi - Scheduling (अनुसूची)
//!
//! Cron-like scheduling and time-based triggers.

use super::tithi::{Tithi, Vaara};

/// Cron-like schedule expression
#[derive(Debug, Clone)]
pub struct Anusuchi {
    /// Minutes (0-59)
    pub nimish: AnusuchiKshetra,
    /// Hours (0-23)
    pub hora: AnusuchiKshetra,
    /// Day of month (1-31)
    pub divas: AnusuchiKshetra,
    /// Month (1-12)
    pub maas: AnusuchiKshetra,
    /// Day of week (0-6, 0=Sunday)
    pub vaara: AnusuchiKshetra,
}

/// Schedule field
#[derive(Debug, Clone)]
pub enum AnusuchiKshetra {
    /// Any value (*)
    Koi,
    /// Specific value
    Nishchit(u8),
    /// List of values
    Suchi(Vec<u8>),
    /// Range (start-end)
    Paridhi(u8, u8),
    /// Step value (*/n)
    Charana(u8),
    /// Range with step (start-end/n)
    Paridhi_Charana(u8, u8, u8),
}

impl AnusuchiKshetra {
    /// Check if value matches
    pub fn milata(&self, mana: u8) -> bool {
        match self {
            Self::Koi => true,
            Self::Nishchit(v) => *v == mana,
            Self::Suchi(list) => list.contains(&mana),
            Self::Paridhi(start, end) => mana >= *start && mana <= *end,
            Self::Charana(step) => mana % step == 0,
            Self::Paridhi_Charana(start, end, step) => {
                mana >= *start && mana <= *end && (mana - start) % step == 0
            }
        }
    }

    /// Get all matching values in range
    pub fn sabhi_milte(&self, min: u8, max: u8) -> Vec<u8> {
        (min..=max).filter(|&v| self.milata(v)).collect()
    }
}

impl Anusuchi {
    /// Create schedule that matches everything
    pub fn har_nimish() -> Self {
        Self {
            nimish: AnusuchiKshetra::Koi,
            hora: AnusuchiKshetra::Koi,
            divas: AnusuchiKshetra::Koi,
            maas: AnusuchiKshetra::Koi,
            vaara: AnusuchiKshetra::Koi,
        }
    }

    /// Every hour at minute 0
    pub fn har_hora() -> Self {
        Self {
            nimish: AnusuchiKshetra::Nishchit(0),
            hora: AnusuchiKshetra::Koi,
            divas: AnusuchiKshetra::Koi,
            maas: AnusuchiKshetra::Koi,
            vaara: AnusuchiKshetra::Koi,
        }
    }

    /// Daily at specific time
    pub fn pratidin(hora: u8, nimish: u8) -> Self {
        Self {
            nimish: AnusuchiKshetra::Nishchit(nimish),
            hora: AnusuchiKshetra::Nishchit(hora),
            divas: AnusuchiKshetra::Koi,
            maas: AnusuchiKshetra::Koi,
            vaara: AnusuchiKshetra::Koi,
        }
    }

    /// Weekly on specific day at time
    pub fn saptahik(vaara: Vaara, hora: u8, nimish: u8) -> Self {
        Self {
            nimish: AnusuchiKshetra::Nishchit(nimish),
            hora: AnusuchiKshetra::Nishchit(hora),
            divas: AnusuchiKshetra::Koi,
            maas: AnusuchiKshetra::Koi,
            vaara: AnusuchiKshetra::Nishchit(vaara.sankhya()),
        }
    }

    /// Monthly on specific day at time
    pub fn maasik(divas: u8, hora: u8, nimish: u8) -> Self {
        Self {
            nimish: AnusuchiKshetra::Nishchit(nimish),
            hora: AnusuchiKshetra::Nishchit(hora),
            divas: AnusuchiKshetra::Nishchit(divas),
            maas: AnusuchiKshetra::Koi,
            vaara: AnusuchiKshetra::Koi,
        }
    }

    /// Check if schedule matches given time
    pub fn milata(&self, tithi: &Tithi, hora: u8, nimish: u8) -> bool {
        self.nimish.milata(nimish)
            && self.hora.milata(hora)
            && self.divas.milata(tithi.divas)
            && self.maas.milata(tithi.maas)
            && self.vaara.milata(tithi.vaara().sankhya())
    }
}

/// Interval-based trigger
#[derive(Debug, Clone, Copy)]
pub struct Antaraal {
    /// Interval in milliseconds
    pub mili: u64,
    /// Last trigger time (None means never triggered)
    pichla: Option<u64>,
}

impl Antaraal {
    /// Create interval trigger
    pub const fn nava(mili: u64) -> Self {
        Self { mili, pichla: None }
    }

    /// Create from seconds
    pub const fn palak_se(palak: u64) -> Self {
        Self::nava(palak * 1000)
    }

    /// Create from minutes
    pub const fn nimish_se(nimish: u64) -> Self {
        Self::nava(nimish * 60 * 1000)
    }

    /// Check if should trigger (provide current time in ms)
    pub fn chaahiye(&mut self, vartamaan: u64) -> bool {
        match self.pichla {
            None => {
                // First call - trigger immediately
                self.pichla = Some(vartamaan);
                true
            }
            Some(last) if vartamaan >= last + self.mili => {
                self.pichla = Some(vartamaan);
                true
            }
            _ => false,
        }
    }

    /// Time until next trigger
    pub fn shesh(&self, vartamaan: u64) -> u64 {
        match self.pichla {
            None => 0,
            Some(last) => {
                let next = last + self.mili;
                if vartamaan >= next {
                    0
                } else {
                    next - vartamaan
                }
            }
        }
    }

    /// Reset trigger
    pub fn punasthapana(&mut self, vartamaan: u64) {
        self.pichla = Some(vartamaan);
    }
}

/// Debounce trigger (only fires after delay with no activity)
#[derive(Debug, Clone)]
pub struct Vilambit {
    /// Delay in milliseconds
    pub vilamba: u64,
    /// Last activity time
    antim_kriya: Option<u64>,
    /// Whether already fired
    prakaashit: bool,
}

impl Vilambit {
    /// Create debounce trigger
    pub const fn nava(vilamba_ms: u64) -> Self {
        Self {
            vilamba: vilamba_ms,
            antim_kriya: None,
            prakaashit: false,
        }
    }

    /// Signal activity
    pub fn kriya(&mut self, vartamaan: u64) {
        self.antim_kriya = Some(vartamaan);
        self.prakaashit = false;
    }

    /// Check if should fire
    pub fn chaahiye(&mut self, vartamaan: u64) -> bool {
        if self.prakaashit {
            return false;
        }

        if let Some(last) = self.antim_kriya {
            if vartamaan >= last + self.vilamba {
                self.prakaashit = true;
                return true;
            }
        }
        false
    }

    /// Reset
    pub fn punasthapana(&mut self) {
        self.antim_kriya = None;
        self.prakaashit = false;
    }
}

/// Throttle trigger (fires at most once per interval)
#[derive(Debug, Clone)]
pub struct Niyantrit {
    /// Minimum interval in milliseconds
    pub antaraal: u64,
    /// Last fire time
    antim_prakaash: u64,
}

impl Niyantrit {
    /// Create throttle trigger
    pub const fn nava(antaraal_ms: u64) -> Self {
        Self {
            antaraal: antaraal_ms,
            antim_prakaash: 0,
        }
    }

    /// Try to fire (returns true if allowed)
    pub fn prayas(&mut self, vartamaan: u64) -> bool {
        if vartamaan >= self.antim_prakaash + self.antaraal {
            self.antim_prakaash = vartamaan;
            true
        } else {
            false
        }
    }

    /// Time until can fire again
    pub fn shesh(&self, vartamaan: u64) -> u64 {
        let next = self.antim_prakaash + self.antaraal;
        if vartamaan >= next {
            0
        } else {
            next - vartamaan
        }
    }
}

/// Rate limiter with token bucket
#[derive(Debug, Clone)]
pub struct DarNiyamak {
    /// Maximum tokens (bucket capacity)
    pub adhikatam: u32,
    /// Current tokens
    vartamaan: f64,
    /// Tokens per millisecond
    dar: f64,
    /// Last update time
    antim: u64,
}

impl DarNiyamak {
    /// Create rate limiter (tokens_per_second, max_burst)
    pub fn nava(dar_pratisec: f64, adhikatam: u32) -> Self {
        Self {
            adhikatam,
            vartamaan: adhikatam as f64,
            dar: dar_pratisec / 1000.0,
            antim: 0,
        }
    }

    /// Update token count
    fn navikaran(&mut self, vartamaan: u64) {
        let elapsed = vartamaan.saturating_sub(self.antim);
        self.vartamaan = (self.vartamaan + elapsed as f64 * self.dar).min(self.adhikatam as f64);
        self.antim = vartamaan;
    }

    /// Try to consume tokens
    pub fn kharch(&mut self, vartamaan: u64, tokens: u32) -> bool {
        self.navikaran(vartamaan);

        if self.vartamaan >= tokens as f64 {
            self.vartamaan -= tokens as f64;
            true
        } else {
            false
        }
    }

    /// Available tokens
    pub fn uplabdh(&mut self, vartamaan: u64) -> u32 {
        self.navikaran(vartamaan);
        self.vartamaan as u32
    }

    /// Time until n tokens available
    pub fn pratiksha(&mut self, vartamaan: u64, tokens: u32) -> u64 {
        self.navikaran(vartamaan);

        if self.vartamaan >= tokens as f64 {
            0
        } else {
            let needed = tokens as f64 - self.vartamaan;
            (needed / self.dar) as u64
        }
    }
}

/// Retry with exponential backoff
#[derive(Debug, Clone)]
pub struct PunahPrayas {
    /// Base delay in milliseconds
    pub aadhar: u64,
    /// Maximum delay
    pub adhikatam: u64,
    /// Current attempt
    prayas: u32,
    /// Multiplier (typically 2)
    gunaka: f64,
}

impl PunahPrayas {
    /// Create retry handler
    pub fn nava(aadhar_ms: u64, adhikatam_ms: u64) -> Self {
        Self {
            aadhar: aadhar_ms,
            adhikatam: adhikatam_ms,
            prayas: 0,
            gunaka: 2.0,
        }
    }

    /// Get delay for next retry
    pub fn agla_vilamba(&mut self) -> u64 {
        let delay = (self.aadhar as f64 * libm::pow(self.gunaka, self.prayas as f64)) as u64;
        self.prayas += 1;
        delay.min(self.adhikatam)
    }

    /// Reset after success
    pub fn safal(&mut self) {
        self.prayas = 0;
    }

    /// Current attempt number
    pub fn prayas_sankhya(&self) -> u32 {
        self.prayas
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_match() {
        assert!(AnusuchiKshetra::Koi.milata(5));
        assert!(AnusuchiKshetra::Nishchit(5).milata(5));
        assert!(!AnusuchiKshetra::Nishchit(5).milata(6));
        assert!(AnusuchiKshetra::Paridhi(1, 10).milata(5));
        assert!(!AnusuchiKshetra::Paridhi(1, 10).milata(15));
    }

    #[test]
    fn test_interval() {
        let mut trigger = Antaraal::nava(100);
        assert!(trigger.chaahiye(0));
        assert!(!trigger.chaahiye(50));
        assert!(trigger.chaahiye(100));
    }

    #[test]
    fn test_rate_limiter() {
        let mut limiter = DarNiyamak::nava(10.0, 10); // 10/sec, burst 10

        // Should allow initial burst
        assert!(limiter.kharch(0, 5));
        assert!(limiter.kharch(0, 5));
        assert!(!limiter.kharch(0, 1)); // Empty

        // After 1 second, should have 10 tokens again
        assert!(limiter.kharch(1000, 10));
    }

    #[test]
    fn test_exponential_backoff() {
        let mut retry = PunahPrayas::nava(100, 10000);

        assert_eq!(retry.agla_vilamba(), 100);
        assert_eq!(retry.agla_vilamba(), 200);
        assert_eq!(retry.agla_vilamba(), 400);

        retry.safal();
        assert_eq!(retry.agla_vilamba(), 100);
    }
}
