//! Chakra Architecture
//!
//! 7-layer software architecture based on the 7 chakras:
//! 1. Mūlādhāra (root) - Hardware/OS layer
//! 2. Svādhiṣṭhāna (sacral) - Memory management
//! 3. Maṇipūra (solar plexus) - Processing/CPU
//! 4. Anāhata (heart) - Business logic
//! 5. Viśuddha (throat) - Communication/API
//! 6. Ājñā (third eye) - Monitoring/observability
//! 7. Sahasrāra (crown) - User interface

// Submodules
pub mod optimizer;

// Re-exports
pub use optimizer::{ChakraOptimizer, Optimizable, OptimizationPass, OptimizerConfig};

use crate::traits::{PhilosophicalEnum, SanskritDescribed, SanskritNamed};

/// The 7 Chakras as software layers
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Chakra {
    /// Mūlādhāra (root) - Hardware/OS layer
    Muladhara = 1,

    /// Svādhiṣṭhāna (sacral) - Memory management
    Svadhisthana = 2,

    /// Maṇipūra (solar plexus) - Processing/CPU
    Manipura = 3,

    /// Anāhata (heart) - Business logic
    Anahata = 4,

    /// Viśuddha (throat) - Communication/API
    Vishuddha = 5,

    /// Ājñā (third eye) - Monitoring/observability
    Ajna = 6,

    /// Sahasrāra (crown) - User interface
    Sahasrara = 7,
}

impl Chakra {
    /// Get all 7 Chakras in order (root to crown)
    pub fn all() -> [Chakra; 7] {
        [
            Chakra::Muladhara,
            Chakra::Svadhisthana,
            Chakra::Manipura,
            Chakra::Anahata,
            Chakra::Vishuddha,
            Chakra::Ajna,
            Chakra::Sahasrara,
        ]
    }

    /// Get IAST transliteration
    pub fn iast(&self) -> &'static str {
        match self {
            Self::Muladhara => "Mūlādhāra",
            Self::Svadhisthana => "Svādhiṣṭhāna",
            Self::Manipura => "Maṇipūra",
            Self::Anahata => "Anāhata",
            Self::Vishuddha => "Viśuddha",
            Self::Ajna => "Ājñā",
            Self::Sahasrara => "Sahasrāra",
        }
    }

    /// Get English meaning
    pub fn english(&self) -> &'static str {
        match self {
            Self::Muladhara => "Root Support",
            Self::Svadhisthana => "Own Place",
            Self::Manipura => "Jewel City",
            Self::Anahata => "Unstruck",
            Self::Vishuddha => "Purification",
            Self::Ajna => "Command",
            Self::Sahasrara => "Thousand-Petaled",
        }
    }

    /// Get Sanskrit name
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::Muladhara => "मूलाधार",
            Self::Svadhisthana => "स्वाधिष्ठान",
            Self::Manipura => "मणिपूर",
            Self::Anahata => "अनाहत",
            Self::Vishuddha => "विशुद्ध",
            Self::Ajna => "आज्ञा",
            Self::Sahasrara => "सहस्रार",
        }
    }

    /// Get software layer mapping
    pub fn software_layer(&self) -> &'static str {
        match self {
            Self::Muladhara => "Hardware/OS",
            Self::Svadhisthana => "Memory Management",
            Self::Manipura => "Processing/CPU",
            Self::Anahata => "Business Logic",
            Self::Vishuddha => "Communication/API",
            Self::Ajna => "Monitoring",
            Self::Sahasrara => "User Interface",
        }
    }

    /// Get color (for visualization)
    pub fn color(&self) -> &'static str {
        match self {
            Self::Muladhara => "red",
            Self::Svadhisthana => "orange",
            Self::Manipura => "yellow",
            Self::Anahata => "green",
            Self::Vishuddha => "blue",
            Self::Ajna => "indigo",
            Self::Sahasrara => "violet",
        }
    }

    /// Get element
    pub fn element(&self) -> &'static str {
        match self {
            Self::Muladhara => "Earth",
            Self::Svadhisthana => "Water",
            Self::Manipura => "Fire",
            Self::Anahata => "Air",
            Self::Vishuddha => "Ether",
            Self::Ajna => "Light",
            Self::Sahasrara => "Thought",
        }
    }

    /// Get bīja (seed) mantra
    pub fn bija_mantra(&self) -> &'static str {
        match self {
            Self::Muladhara => "लं (LAM)",
            Self::Svadhisthana => "वं (VAM)",
            Self::Manipura => "रं (RAM)",
            Self::Anahata => "यं (YAM)",
            Self::Vishuddha => "हं (HAM)",
            Self::Ajna => "ॐ (OM)",
            Self::Sahasrara => "अः (AH)",
        }
    }
}

// ============================================================================
// v10.0 Trait Implementations
// ============================================================================

impl SanskritNamed for Chakra {
    fn sanskrit(&self) -> &'static str {
        self.sanskrit_name()
    }

    fn iast(&self) -> &'static str {
        self.iast()
    }

    fn english(&self) -> &'static str {
        self.english()
    }
}

impl SanskritDescribed for Chakra {
    fn meaning(&self) -> &'static str {
        match self {
            Self::Muladhara => "The root foundation, base of the spine, survival instincts",
            Self::Svadhisthana => "One's own dwelling, creativity and emotional flow",
            Self::Manipura => "City of jewels, personal power and transformation",
            Self::Anahata => "The unstruck sound, love and compassion",
            Self::Vishuddha => "Purification center, communication and truth",
            Self::Ajna => "Command center, intuition and wisdom",
            Self::Sahasrara => "Thousand-petaled lotus, pure consciousness",
        }
    }

    fn explanation(&self) -> &'static str {
        match self {
            Self::Muladhara => "Maps to hardware/OS layer - the foundation everything builds upon",
            Self::Svadhisthana => "Maps to memory management - fluid allocation and deallocation",
            Self::Manipura => "Maps to CPU/processing - transformative computation power",
            Self::Anahata => "Maps to business logic - the heart of application functionality",
            Self::Vishuddha => "Maps to APIs - pure communication between components",
            Self::Ajna => "Maps to monitoring - observing and directing the system",
            Self::Sahasrara => "Maps to UI - the crown of user experience",
        }
    }

    fn mantra(&self) -> Option<&'static str> {
        Some(self.bija_mantra())
    }

    fn category(&self) -> &'static str {
        "Chakra System (चक्र)"
    }
}

impl PhilosophicalEnum for Chakra {
    fn all() -> &'static [Self] {
        &[
            Chakra::Muladhara,
            Chakra::Svadhisthana,
            Chakra::Manipura,
            Chakra::Anahata,
            Chakra::Vishuddha,
            Chakra::Ajna,
            Chakra::Sahasrara,
        ]
    }

    fn count() -> usize {
        7
    }

    fn index(&self) -> usize {
        *self as usize - 1
    }

    fn ordinal(&self) -> usize {
        *self as usize
    }

    fn next(&self) -> Self {
        match self {
            Self::Muladhara => Self::Svadhisthana,
            Self::Svadhisthana => Self::Manipura,
            Self::Manipura => Self::Anahata,
            Self::Anahata => Self::Vishuddha,
            Self::Vishuddha => Self::Ajna,
            Self::Ajna => Self::Sahasrara,
            Self::Sahasrara => Self::Muladhara, // Cycle back (kundalini rises and descends)
        }
    }

    fn prev(&self) -> Self {
        match self {
            Self::Muladhara => Self::Sahasrara, // Cycle back
            Self::Svadhisthana => Self::Muladhara,
            Self::Manipura => Self::Svadhisthana,
            Self::Anahata => Self::Manipura,
            Self::Vishuddha => Self::Anahata,
            Self::Ajna => Self::Vishuddha,
            Self::Sahasrara => Self::Ajna,
        }
    }

    fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Muladhara),
            1 => Some(Self::Svadhisthana),
            2 => Some(Self::Manipura),
            3 => Some(Self::Anahata),
            4 => Some(Self::Vishuddha),
            5 => Some(Self::Ajna),
            6 => Some(Self::Sahasrara),
            _ => None,
        }
    }
}

/// Chakra architecture analyzer
pub struct ChakraArchitecture {
    /// Layer assignments
    layers: [Vec<String>; 7],
    /// Inter-layer dependencies
    dependencies: Vec<(Chakra, Chakra, String)>,
}

/// Layer health status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChakraHealth {
    /// Balanced (healthy)
    Balanced,
    /// Underactive (needs attention)
    Underactive,
    /// Overactive (too complex)
    Overactive,
    /// Blocked (critical issue)
    Blocked,
}

impl ChakraArchitecture {
    pub fn new() -> Self {
        Self {
            layers: Default::default(),
            dependencies: Vec::new(),
        }
    }

    /// Assign a component to a chakra layer
    pub fn assign(&mut self, component: String, chakra: Chakra) {
        self.layers[(chakra as usize) - 1].push(component);
    }

    /// Record a dependency between layers
    pub fn add_dependency(&mut self, from: Chakra, to: Chakra, component: String) {
        self.dependencies.push((from, to, component));
    }

    /// Check layer health
    pub fn layer_health(&self, chakra: Chakra) -> ChakraHealth {
        let index = (chakra as usize) - 1;
        let component_count = self.layers[index].len();

        // Check for blocked (critical issues)
        let has_reverse_deps = self
            .dependencies
            .iter()
            .any(|(from, to, _)| *to == chakra && (*from as u8) > (*to as u8));
        if has_reverse_deps {
            return ChakraHealth::Blocked;
        }

        // Check balance
        match component_count {
            0 => ChakraHealth::Underactive,
            1..=10 => ChakraHealth::Balanced,
            11..=50 => ChakraHealth::Overactive,
            _ => ChakraHealth::Blocked,
        }
    }

    /// Check for kundalini flow (proper layer communication)
    pub fn check_kundalini_flow(&self) -> Vec<String> {
        let mut issues = Vec::new();

        for (from, to, component) in &self.dependencies {
            let from_level = *from as i8;
            let to_level = *to as i8;

            // Energy should flow up (or stay same level)
            if from_level > to_level + 1 {
                issues.push(format!(
                    "Blocked flow: {} ({}) → {} ({}): {}",
                    from.sanskrit_name(),
                    from_level,
                    to.sanskrit_name(),
                    to_level,
                    component
                ));
            }
        }

        issues
    }

    /// Generate architecture report
    pub fn report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Chakra Architecture Report ===\n\n");

        for i in (0..7).rev() {
            let chakra: Chakra = unsafe { std::mem::transmute((i + 1) as u8) };
            let health = self.layer_health(chakra);
            let health_symbol = match health {
                ChakraHealth::Balanced => "●",
                ChakraHealth::Underactive => "○",
                ChakraHealth::Overactive => "◉",
                ChakraHealth::Blocked => "✗",
            };

            report.push_str(&format!(
                "{} {} - {} ({} components) {}\n",
                health_symbol,
                chakra.sanskrit_name(),
                chakra.software_layer(),
                self.layers[i].len(),
                chakra.color()
            ));
        }

        let flow_issues = self.check_kundalini_flow();
        if !flow_issues.is_empty() {
            report.push_str("\n⚠️ Flow Issues:\n");
            for issue in flow_issues {
                report.push_str(&format!("  - {}\n", issue));
            }
        }

        report
    }
}

impl Default for ChakraArchitecture {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::{PhilosophicalEnum, SanskritDescribed, SanskritNamed};

    #[test]
    fn test_chakra_count() {
        assert_eq!(Chakra::count(), 7);
        assert_eq!(Chakra::all().len(), 7);
        assert_eq!(Chakra::all().len(), 7);
    }

    #[test]
    fn test_chakra_sanskrit_named_trait() {
        let chakra = Chakra::Muladhara;
        assert_eq!(chakra.sanskrit(), "मूलाधार");
        assert_eq!(chakra.iast(), "Mūlādhāra");
        assert_eq!(chakra.english(), "Root Support");
    }

    #[test]
    fn test_chakra_sanskrit_described_trait() {
        let chakra = Chakra::Anahata;
        assert_eq!(chakra.meaning(), "Application logic, business rules");
        assert!(chakra.explanation().contains("air"));
        assert!(chakra.mantra().is_some());
        assert_eq!(chakra.category(), "Tantra Yoga (तन्त्र योग)");
    }

    #[test]
    fn test_chakra_bija_mantras() {
        assert_eq!(Chakra::Muladhara.bija_mantra(), "LAM (लं)");
        assert_eq!(Chakra::Svadhisthana.bija_mantra(), "VAM (वं)");
        assert_eq!(Chakra::Manipura.bija_mantra(), "RAM (रं)");
        assert_eq!(Chakra::Anahata.bija_mantra(), "YAM (यं)");
        assert_eq!(Chakra::Vishuddha.bija_mantra(), "HAM (हं)");
        assert_eq!(Chakra::Ajna.bija_mantra(), "OM (ॐ)");
        assert_eq!(Chakra::Sahasrara.bija_mantra(), "AH (अः)");
    }

    #[test]
    fn test_chakra_navigation_cycle() {
        // Forward: kundalini rises
        assert_eq!(Chakra::Muladhara.next(), Chakra::Svadhisthana);
        assert_eq!(Chakra::Sahasrara.next(), Chakra::Muladhara); // Cycle back

        // Backward: kundalini descends
        assert_eq!(Chakra::Svadhisthana.prev(), Chakra::Muladhara);
        assert_eq!(Chakra::Muladhara.prev(), Chakra::Sahasrara); // Cycle back
    }

    #[test]
    fn test_chakra_from_index() {
        assert_eq!(Chakra::from_index(0), Some(Chakra::Muladhara));
        assert_eq!(Chakra::from_index(6), Some(Chakra::Sahasrara));
        assert_eq!(Chakra::from_index(7), None);
    }

    #[test]
    fn test_chakra_ordinal_sequence() {
        for (i, chakra) in Chakra::all().iter().enumerate() {
            assert_eq!(chakra.ordinal(), i + 1, "Chakra {:?} ordinal mismatch", chakra);
            assert_eq!(chakra.index(), i, "Chakra {:?} index mismatch", chakra);
        }
    }

    #[test]
    fn test_chakra_software_layers() {
        assert_eq!(Chakra::Muladhara.layer(), SoftwareLayer::Hardware);
        assert_eq!(Chakra::Svadhisthana.layer(), SoftwareLayer::Kernel);
        assert_eq!(Chakra::Manipura.layer(), SoftwareLayer::Runtime);
        assert_eq!(Chakra::Anahata.layer(), SoftwareLayer::Application);
        assert_eq!(Chakra::Vishuddha.layer(), SoftwareLayer::Networking);
        assert_eq!(Chakra::Ajna.layer(), SoftwareLayer::UI);
        assert_eq!(Chakra::Sahasrara.layer(), SoftwareLayer::Transcendent);
    }

    #[test]
    fn test_chakra_architecture_flow() {
        let mut arch = ChakraArchitecture::new();

        // Register components in proper flow order
        arch.register_component(Chakra::Muladhara, "Memory Allocator");
        arch.register_component(Chakra::Svadhisthana, "System Calls");
        arch.register_component(Chakra::Manipura, "Garbage Collector");
        arch.register_component(Chakra::Anahata, "Business Logic");
        arch.register_component(Chakra::Vishuddha, "HTTP Server");
        arch.register_component(Chakra::Ajna, "Web Interface");
        arch.register_component(Chakra::Sahasrara, "AI Integration");

        // Check layer health for each chakra
        for chakra in Chakra::all() {
            let health = arch.layer_health(*chakra);
            assert_eq!(health, ChakraHealth::Balanced, "Chakra {:?} should be balanced", chakra);
        }
    }
}
