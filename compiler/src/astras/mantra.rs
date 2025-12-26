//! # Mantra System
//!
//! Sacred mantras for invoking divine weapons.
//! Each Astra has its own invocation mantra.

/// A sacred invocation mantra
#[derive(Debug, Clone)]
pub struct Mantra {
    /// The Sanskrit text of the mantra
    text: String,
    /// The deity being invoked
    deity: String,
    /// Power multiplier (1.0 = normal)
    power_multiplier: f64,
}

impl Mantra {
    pub fn new(text: &str, deity: &str) -> Self {
        Self {
            text: text.to_string(),
            deity: deity.to_string(),
            power_multiplier: 1.0,
        }
    }

    pub fn with_power(mut self, multiplier: f64) -> Self {
        self.power_multiplier = multiplier;
        self
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn deity(&self) -> &str {
        &self.deity
    }

    pub fn power(&self) -> f64 {
        self.power_multiplier
    }

    /// Brahmastra invocation
    pub fn brahmastra() -> Self {
        Self::new("Om Brahmāstrāya Phaṭ", "Brahma").with_power(10.0)
    }

    /// Agneyastra invocation
    pub fn agneyastra() -> Self {
        Self::new("Om Agnaye Svāhā", "Agni").with_power(7.0)
    }

    /// Varunastra invocation
    pub fn varunastra() -> Self {
        Self::new("Om Varuṇāya Namaḥ", "Varuna").with_power(7.0)
    }

    /// Vayuastra invocation
    pub fn vayuastra() -> Self {
        Self::new("Om Vāyave Namaḥ", "Vayu").with_power(7.0)
    }

    /// Pashupatastra invocation
    pub fn pashupatastra() -> Self {
        Self::new("Om Namaḥ Śivāya Paśupataye", "Shiva").with_power(10.0)
    }

    /// Nagastra invocation
    pub fn nagastra() -> Self {
        Self::new("Om Nāgāya Namaḥ", "Nagas").with_power(6.0)
    }

    /// Garudastra invocation
    pub fn garudastra() -> Self {
        Self::new("Om Garuḍāya Namaḥ", "Garuda").with_power(7.0)
    }

    /// Sudarshana Chakra invocation
    pub fn sudarshana() -> Self {
        Self::new("Om Sudarśanāya Vidmahe Mahājvālāya Dhīmahi", "Vishnu").with_power(9.0)
    }

    /// Indrastra invocation
    pub fn indrastra() -> Self {
        Self::new("Om Indrāya Namaḥ Vajrapāṇaye", "Indra").with_power(8.0)
    }

    /// Narayanastra invocation
    pub fn narayanastra() -> Self {
        Self::new("Om Namo Nārāyaṇāya", "Vishnu").with_power(9.0)
    }

    /// Trishula invocation
    pub fn trishula() -> Self {
        Self::new("Om Tryambakāya Triśūlāya", "Shiva").with_power(9.0)
    }
}

/// Record of a mantra invocation
#[derive(Debug, Clone)]
pub struct MantraInvocation {
    /// The mantra used
    pub mantra: Mantra,
    /// When it was invoked
    pub timestamp: std::time::Instant,
    /// Result of invocation
    pub success: bool,
    /// Number of transformations
    pub transformations: usize,
}

impl MantraInvocation {
    pub fn new(mantra: Mantra) -> Self {
        Self {
            mantra,
            timestamp: std::time::Instant::now(),
            success: false,
            transformations: 0,
        }
    }

    pub fn complete(mut self, success: bool, transformations: usize) -> Self {
        self.success = success;
        self.transformations = transformations;
        self
    }
}

/// Log of all mantra invocations (for audit trail)
pub struct MantraLog {
    invocations: Vec<MantraInvocation>,
}

impl MantraLog {
    pub fn new() -> Self {
        Self {
            invocations: Vec::new(),
        }
    }

    pub fn record(&mut self, invocation: MantraInvocation) {
        self.invocations.push(invocation);
    }

    pub fn total_transformations(&self) -> usize {
        self.invocations
            .iter()
            .filter(|i| i.success)
            .map(|i| i.transformations)
            .sum()
    }

    pub fn success_rate(&self) -> f64 {
        if self.invocations.is_empty() {
            return 1.0;
        }
        let successes = self.invocations.iter().filter(|i| i.success).count();
        successes as f64 / self.invocations.len() as f64
    }
}

impl Default for MantraLog {
    fn default() -> Self {
        Self::new()
    }
}
