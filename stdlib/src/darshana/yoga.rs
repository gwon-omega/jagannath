//! Yoga Module (योग) — Union/Integration
//!
//! Provides utilities for the integration of code components.

use core::fmt;

/// The three Guṇas (qualities/modes)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Guna {
    /// Sattva (सत्त्व) - Purity, correctness-first
    Sattva,
    /// Rajas (रजस्) - Activity, speed-first (default)
    #[default]
    Rajas,
    /// Tamas (तमस्) - Inertia, memory-first
    Tamas,
}

impl Guna {
    /// Get the Sanskrit name
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Guna::Sattva => "सत्त्व",
            Guna::Rajas => "रजस्",
            Guna::Tamas => "तमस्",
        }
    }

    /// Get the optimization priority
    pub fn priority(&self) -> GunaPriority {
        match self {
            Guna::Sattva => GunaPriority::Correctness,
            Guna::Rajas => GunaPriority::Speed,
            Guna::Tamas => GunaPriority::Memory,
        }
    }
}

impl fmt::Display for Guna {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({})", self, self.sanskrit_name())
    }
}

/// Optimization priority based on guṇa
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GunaPriority {
    /// Prioritize correctness and safety
    Correctness,
    /// Prioritize execution speed
    Speed,
    /// Prioritize memory efficiency
    Memory,
}

/// The seven Chakras (energy centers)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Chakra {
    /// Mūlādhāra (मूलाधार) - Root - Hardware layer
    Muladhara = 1,
    /// Svādhiṣṭhāna (स्वाधिष्ठान) - Sacral - OS layer
    Svadhisthana = 2,
    /// Maṇipūra (मणिपूर) - Solar Plexus - Runtime layer
    Manipura = 3,
    /// Anāhata (अनाहत) - Heart - Business logic layer
    Anahata = 4,
    /// Viśuddha (विशुद्ध) - Throat - API layer
    Vishuddha = 5,
    /// Ājñā (आज्ञा) - Third Eye - UI logic layer
    Ajna = 6,
    /// Sahasrāra (सहस्रार) - Crown - UX layer
    Sahasrara = 7,
}

impl Chakra {
    /// Get the level (1-7) of this chakra
    pub fn level(&self) -> u8 {
        *self as u8
    }

    /// Get the Sanskrit name
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Chakra::Muladhara => "मूलाधार",
            Chakra::Svadhisthana => "स्वाधिष्ठान",
            Chakra::Manipura => "मणिपूर",
            Chakra::Anahata => "अनाहत",
            Chakra::Vishuddha => "विशुद्ध",
            Chakra::Ajna => "आज्ञा",
            Chakra::Sahasrara => "सहस्रार",
        }
    }

    /// Get the software layer this chakra represents
    pub fn software_layer(&self) -> &'static str {
        match self {
            Chakra::Muladhara => "Hardware/Drivers",
            Chakra::Svadhisthana => "OS/Kernel",
            Chakra::Manipura => "Runtime/Framework",
            Chakra::Anahata => "Business Logic",
            Chakra::Vishuddha => "API/Communication",
            Chakra::Ajna => "UI Logic",
            Chakra::Sahasrara => "User Experience",
        }
    }

    /// Get the element associated with this chakra
    pub fn element(&self) -> &'static str {
        match self {
            Chakra::Muladhara => "Earth",
            Chakra::Svadhisthana => "Water",
            Chakra::Manipura => "Fire",
            Chakra::Anahata => "Air",
            Chakra::Vishuddha => "Ether",
            Chakra::Ajna => "Light",
            Chakra::Sahasrara => "Thought",
        }
    }
}

impl fmt::Display for Chakra {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({}) - {}",
            self,
            self.sanskrit_name(),
            self.software_layer()
        )
    }
}

/// The eight limbs of Ashtanga Yoga
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum AshtangaLimb {
    /// Yama (यम) - Ethical constraints / Code standards
    Yama = 1,
    /// Niyama (नियम) - Observances / Best practices
    Niyama = 2,
    /// Āsana (आसन) - Posture / Stable architecture
    Asana = 3,
    /// Prāṇāyāma (प्राणायाम) - Breath control / Data flow
    Pranayama = 4,
    /// Pratyāhāra (प्रत्याहार) - Sense withdrawal / Input validation
    Pratyahara = 5,
    /// Dhāraṇā (धारणा) - Concentration / Focused optimization
    Dharana = 6,
    /// Dhyāna (ध्यान) - Meditation / Continuous monitoring
    Dhyana = 7,
    /// Samādhi (समाधि) - Absorption / Perfect release
    Samadhi = 8,
}

impl AshtangaLimb {
    /// Get the SDLC phase this limb represents
    pub fn sdlc_phase(&self) -> &'static str {
        match self {
            AshtangaLimb::Yama => "Code Standards",
            AshtangaLimb::Niyama => "Best Practices",
            AshtangaLimb::Asana => "Architecture Design",
            AshtangaLimb::Pranayama => "Data Flow Design",
            AshtangaLimb::Pratyahara => "Security & Validation",
            AshtangaLimb::Dharana => "Performance Optimization",
            AshtangaLimb::Dhyana => "Monitoring & Observability",
            AshtangaLimb::Samadhi => "Deployment & Release",
        }
    }

    /// Get the Sanskrit name
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            AshtangaLimb::Yama => "यम",
            AshtangaLimb::Niyama => "नियम",
            AshtangaLimb::Asana => "आसन",
            AshtangaLimb::Pranayama => "प्राणायाम",
            AshtangaLimb::Pratyahara => "प्रत्याहार",
            AshtangaLimb::Dharana => "धारणा",
            AshtangaLimb::Dhyana => "ध्यान",
            AshtangaLimb::Samadhi => "समाधि",
        }
    }
}

impl fmt::Display for AshtangaLimb {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ({}) - {}",
            self,
            self.sanskrit_name(),
            self.sdlc_phase()
        )
    }
}

/// The five Yamas (ethical restraints)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Yama {
    /// Ahiṃsā (अहिंसा) - Non-violence / No harmful code
    Ahimsa,
    /// Satya (सत्य) - Truthfulness / Honest interfaces
    Satya,
    /// Asteya (अस्तेय) - Non-stealing / Respect ownership
    Asteya,
    /// Brahmacarya (ब्रह्मचर्य) - Conservation / Efficient resources
    Brahmacharya,
    /// Aparigraha (अपरिग्रह) - Non-attachment / Release resources
    Aparigraha,
}

/// The five Niyamas (positive observances)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Niyama {
    /// Śauca (शौच) - Purity / Clean code
    Shauca,
    /// Santoṣa (संतोष) - Contentment / Simple solutions
    Santosha,
    /// Tapas (तपस्) - Discipline / Rigorous testing
    Tapas,
    /// Svādhyāya (स्वाध्याय) - Self-study / Documentation
    Svadhyaya,
    /// Īśvarapraṇidhāna (ईश्वरप्रणिधान) - Surrender / Higher purpose
    Ishvarapranidhana,
}

/// Chitta Vṛtti (mental modifications) for determinism
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChittaVritti {
    /// Pramāṇa (प्रमाण) - Right knowledge / Correct compilation
    Pramana,
    /// Viparyaya (विपर्यय) - Misconception / Compiler error
    Viparyaya,
    /// Vikalpa (विकल्प) - Imagination / Speculation
    Vikalpa,
    /// Nidrā (निद्रा) - Sleep / Cached/dormant state
    Nidra,
    /// Smṛti (स्मृति) - Memory / Build artifacts
    Smriti,
}

impl ChittaVritti {
    /// Check if this vritti represents correct state
    pub fn is_correct(&self) -> bool {
        matches!(self, ChittaVritti::Pramana)
    }

    /// Check if this vritti is an error state
    pub fn is_error(&self) -> bool {
        matches!(self, ChittaVritti::Viparyaya)
    }
}
