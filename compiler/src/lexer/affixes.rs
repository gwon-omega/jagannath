//! Affix System - Sanskrit Pratyaya (Suffix) Encoding
//!
//! Maps Sanskrit affixes to compiler semantics:
//! - Mutability (-a/-ā)
//! - Storage class (-k/-g/-l/-h/-b)
//! - Type width (-t8/-t16/-t32/-t64/-f32/-f64)
//! - Layout (-p/-v/-s)
//! - Lifetime (^N)
//! - Concurrency (-sūtra/-eka)
//! - Security (-guhya/-sarvajnika)

/// Complete affix enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Affix {
    // ========================================================================
    // Mutability
    // ========================================================================
    /// -a → immutable (const)
    A,
    /// -ā → mutable
    Aa,

    // ========================================================================
    // Storage Class
    // ========================================================================
    /// -k → stack allocation
    K,
    /// -g → global/static/pooled
    G,
    /// -l → linear/owned (unique ownership)
    L,
    /// -h → heap (manual malloc/free)
    H,
    /// -b → borrowed (reference, non-owning)
    B,

    // ========================================================================
    // Type Width
    // ========================================================================
    /// -t8 → int8_t
    T8,
    /// -t16 → int16_t
    T16,
    /// -t32 → int32_t
    T32,
    /// -t64 → int64_t
    T64,
    /// -f32 → float
    F32,
    /// -f64 → double
    F64,
    /// -t1 → bool (1 bit)
    T1,

    // ========================================================================
    // Layout
    // ========================================================================
    /// -p → packed struct (__attribute__((packed)))
    P,
    /// -v → vtable (dynamic dispatch)
    V,
    /// -s → sized array (compile-time known)
    S,

    // ========================================================================
    // Lifetime
    // ========================================================================
    /// ^N → arena/lifetime region (1-255)
    Region(u8),

    // ========================================================================
    // Compile-time
    // ========================================================================
    /// # → constant fold at compile-time
    Hash,
    /// ## → macro expansion (sandhi-based)
    HashHash,

    // ========================================================================
    // Concurrency
    // ========================================================================
    /// -sūtra → thread-safe (Arc<Mutex<T>> equivalent)
    Sutra,
    /// -eka → single-threaded (Rc<T> equivalent)
    Eka,

    // ========================================================================
    // Security
    // ========================================================================
    /// -guhya → secret/tainted (for information flow)
    Guhya,
    /// -sarvajnika → public/clean
    Sarvajnika,

    // ========================================================================
    // Kāraka Roles (v1.0 semantic hints)
    // ========================================================================
    /// ^kartṛ → agent (doer)
    Kartr,
    /// ^karman → patient (object)
    Karman,
    /// ^karaṇa → instrument (means)
    Karana,
    /// ^sampradāna → recipient (beneficiary)
    Sampradana,
    /// ^apādāna → source (origin)
    Apadana,
    /// ^adhikaraṇa → locus (location)
    Adhikarana,

    // ========================================================================
    // Philosophy Affixes (v3.0)
    // ========================================================================
    /// -pratyakṣa → explicit type (direct perception)
    Pratyaksha,
    /// -anumāna → inferred type (logical deduction)
    Anumana,
    /// -upamāna → pattern match type (analogy)
    Upamana,
    /// -śabda → contract type (documentation)
    Shabda,

    // ========================================================================
    // Kosha Affixes (v3.0 memory tiers)
    // ========================================================================
    /// -anna → Register/L1 (hottest)
    Anna,
    /// -prāṇa → L2/L3 cache
    Prana,
    /// -manas → RAM
    Manas,
    /// -vijñāna → Disk
    Vijnana,
    /// -ānanda → Network
    Ananda,

    // ========================================================================
    // Guṇa Affixes (v3.0 optimization modes)
    // ========================================================================
    /// -sattva → correctness-first
    Sattva,
    /// -rajas → speed-first
    Rajas,
    /// -tamas → memory-first
    Tamas,
}

impl Affix {
    /// Parse affix from string
    pub fn parse(s: &str) -> Option<Self> {
        match s {
            // Mutability
            "a" => Some(Affix::A),
            "ā" => Some(Affix::Aa),

            // Storage
            "k" => Some(Affix::K),
            "g" => Some(Affix::G),
            "l" => Some(Affix::L),
            "h" => Some(Affix::H),
            "b" => Some(Affix::B),

            // Type width
            "t8" => Some(Affix::T8),
            "t16" => Some(Affix::T16),
            "t32" => Some(Affix::T32),
            "t64" => Some(Affix::T64),
            "f32" => Some(Affix::F32),
            "f64" => Some(Affix::F64),
            "t1" => Some(Affix::T1),

            // Layout
            "p" => Some(Affix::P),
            "v" => Some(Affix::V),
            "s" => Some(Affix::S),

            // Concurrency
            "sūtra" | "sutra" => Some(Affix::Sutra),
            "eka" => Some(Affix::Eka),

            // Security
            "guhya" => Some(Affix::Guhya),
            "sarvajnika" => Some(Affix::Sarvajnika),

            // Kāraka
            "kartṛ" | "kartr" => Some(Affix::Kartr),
            "karman" => Some(Affix::Karman),
            "karaṇa" | "karana" => Some(Affix::Karana),
            "sampradāna" | "sampradana" => Some(Affix::Sampradana),
            "apādāna" | "apadana" => Some(Affix::Apadana),
            "adhikaraṇa" | "adhikarana" => Some(Affix::Adhikarana),

            // Philosophy
            "pratyakṣa" | "pratyaksha" => Some(Affix::Pratyaksha),
            "anumāna" | "anumana" => Some(Affix::Anumana),
            "upamāna" | "upamana" => Some(Affix::Upamana),
            "śabda" | "shabda" => Some(Affix::Shabda),

            // Kosha
            "anna" => Some(Affix::Anna),
            "prāṇa" | "prana" => Some(Affix::Prana),
            "manas" => Some(Affix::Manas),
            "vijñāna" | "vijnana" => Some(Affix::Vijnana),
            "ānanda" | "ananda" => Some(Affix::Ananda),

            // Guṇa
            "sattva" => Some(Affix::Sattva),
            "rajas" => Some(Affix::Rajas),
            "tamas" => Some(Affix::Tamas),

            // Lifetime region
            s if s.starts_with('^') => s[1..].parse::<u8>().ok().map(Affix::Region),

            _ => None,
        }
    }

    /// Check if this affix is compatible with another
    pub fn is_compatible_with(&self, other: &Affix) -> bool {
        use Affix::*;

        // Incompatible combinations
        let incompatible = matches!(
            (self, other),
            // Can't be both linear AND borrowed
            (L, B) | (B, L) |
            // Can't be both stack AND heap
            (K, H) | (H, K) |
            // Can't be both immutable AND mutable
            (A, Aa) | (Aa, A) |
            // Can't have multiple storage classes
            (K, G) | (G, K) | (K, L) | (L, K) | (K, B) | (B, K) |
            (G, L) | (L, G) | (G, B) | (B, G) | (G, H) | (H, G) |
            (L, H) | (H, L) | (B, H) | (H, B)
        );

        !incompatible
    }
}

/// Sequence of affixes attached to a word
#[derive(Debug, Clone, Default)]
pub struct AffixSequence {
    pub affixes: Vec<Affix>,
}

impl AffixSequence {
    pub fn new() -> Self {
        Self {
            affixes: Vec::new(),
        }
    }

    /// Add an affix to the sequence
    pub fn push(&mut self, affix: Affix) -> Result<(), String> {
        // Validate compatibility with existing affixes
        for existing in &self.affixes {
            if !existing.is_compatible_with(&affix) {
                return Err(format!(
                    "Incompatible affixes: {:?} and {:?}",
                    existing, affix
                ));
            }
        }
        self.affixes.push(affix);
        Ok(())
    }

    /// Check if sequence contains a specific affix
    pub fn contains(&self, affix: &Affix) -> bool {
        self.affixes.contains(affix)
    }

    /// Get mutability affix
    pub fn mutability(&self) -> Option<Affix> {
        self.affixes
            .iter()
            .find(|a| matches!(a, Affix::A | Affix::Aa))
            .copied()
    }

    /// Get storage class affix
    pub fn storage_class(&self) -> Option<Affix> {
        self.affixes
            .iter()
            .find(|a| matches!(a, Affix::K | Affix::G | Affix::L | Affix::H | Affix::B))
            .copied()
    }

    /// Get type width affix
    pub fn type_width(&self) -> Option<Affix> {
        self.affixes
            .iter()
            .find(|a| {
                matches!(
                    a,
                    Affix::T8
                        | Affix::T16
                        | Affix::T32
                        | Affix::T64
                        | Affix::F32
                        | Affix::F64
                        | Affix::T1
                )
            })
            .copied()
    }

    /// Get lifetime region
    pub fn lifetime_region(&self) -> Option<u8> {
        self.affixes.iter().find_map(|a| {
            if let Affix::Region(n) = a {
                Some(*n)
            } else {
                None
            }
        })
    }

    /// Get kāraka role
    pub fn karaka(&self) -> Option<Affix> {
        self.affixes
            .iter()
            .find(|a| {
                matches!(
                    a,
                    Affix::Kartr
                        | Affix::Karman
                        | Affix::Karana
                        | Affix::Sampradana
                        | Affix::Apadana
                        | Affix::Adhikarana
                )
            })
            .copied()
    }

    /// Check if variable is thread-safe (has -sūtra affix)
    pub fn has_thread_safe(&self) -> bool {
        self.affixes.contains(&Affix::Sutra)
    }

    /// Check if variable is secret/tainted (has -guhya affix)
    pub fn has_secret(&self) -> bool {
        self.affixes.contains(&Affix::Guhya)
    }
}
