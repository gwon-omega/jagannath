//! # 28 Narakas Error Taxonomy
//!
//! Garuda Purana's 28 Hells mapped to compiler error classifications.
//!
//! Each Naraka represents a specific category of code violation:
//! - Memory violations (1-10)
//! - Concurrency violations (11-16)
//! - Security violations (17-23)
//! - Resource violations (24-28)
//!
//! Implements v10.0 unified traits: SanskritNamed, SanskritDescribed, PhilosophicalEnum

use crate::traits::{PhilosophicalEnum, SanskritDescribed, SanskritNamed};

mod andhakupa;
mod andhatamisram;
mod asipatravana;
mod avichi;
mod ayahpana;
mod dandasuka;
mod kalasutra;
mod krimibhaksha;
mod ksharakardama;
mod kumbhipaka;
mod lalabhaksha;
mod maharaurava;
mod paryavartana;
mod pranarodha;
mod puyoda;
mod raksogana;
mod raurava;
mod sandamsha;
mod sarameyadana;
mod suchimukha;
mod sukaramukha;
mod sulaprota;
mod tamisram;
mod taptasurmi;
mod vaitarani_naraka;
mod vajrakantaka;
mod vatarodha;
mod visasana;

pub use andhakupa::AndhakupaChecker;
pub use andhatamisram::AndhatamisramChecker;
pub use asipatravana::AsipatravanaChecker;
pub use avichi::AvichiChecker;
pub use ayahpana::AyahpanaChecker;
pub use dandasuka::DandasukaChecker;
pub use kalasutra::KalasutraChecker;
pub use krimibhaksha::KrimibhakshaChecker;
pub use ksharakardama::KsharakardamaChecker;
pub use kumbhipaka::KumbhipakaChecker;
pub use lalabhaksha::LalabhakshaChecker;
pub use maharaurava::MaharauravaChecker;
pub use paryavartana::ParyavartanaChecker;
pub use pranarodha::PranarodhaChecker;
pub use puyoda::PuyodaChecker;
pub use raksogana::RaksoganaChecker;
pub use raurava::RauravaChecker;
pub use sandamsha::SandamshaChecker;
pub use sarameyadana::SarameyaDanaChecker;
pub use suchimukha::SuchimukhaChecker;
pub use sukaramukha::SukaramukhaChecker;
pub use sulaprota::SulaprotaChecker;
pub use tamisram::TamisramChecker;
pub use taptasurmi::TaptasurmiChecker;
pub use vaitarani_naraka::VaitaraniNarakaChecker;
pub use vajrakantaka::VajrakantakaChecker;
pub use vatarodha::VatarodhaChecker;
pub use visasana::VisasanaChecker;

use crate::errors::Span;
use std::fmt;

/// Severity level of a Naraka violation
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Severity {
    /// Hints and suggestions (optional fixes)
    Hint,
    /// Code smell, potential issue (can ignore)
    Warning,
    /// Will cause runtime failure (blocks build)
    Error,
    /// Security/memory safety critical (must fix)
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Hint => write!(f, "HINT"),
            Severity::Warning => write!(f, "WARNING"),
            Severity::Error => write!(f, "ERROR"),
            Severity::Critical => write!(f, "CRITICAL"),
        }
    }
}

/// Duration of punishment (how long error persists)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Duration {
    /// Temporary - can be ignored
    Temporary,
    /// Until fixed - blocks build
    UntilFixed,
    /// Permanent - won't compile at all
    Permanent,
}

/// Garuda Purana's 28 Narakas mapped to compiler errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Naraka {
    // ═══════════════════════════════════════════════════════════════════════
    // MEMORY VIOLATIONS (Hells 1-10)
    // ═══════════════════════════════════════════════════════════════════════
    /// 1. Tamisram - Heavy flogging
    /// Sin: Stealing others' wealth
    /// Code: Memory theft (use-after-free, double-free)
    Tamisram = 1,

    /// 2. Andhatamisram - Darkness flogging
    /// Sin: Betraying spouse/partner
    /// Code: API contract violation, breaking promises
    Andhatamisram = 2,

    /// 3. Raurava - Screaming
    /// Sin: Violence causing suffering
    /// Code: Panic/crash causing process death
    Raurava = 3,

    /// 4. Maharaurava - Great screaming
    /// Sin: Killing living beings
    /// Code: Killing child processes, forced termination
    Maharaurava = 4,

    /// 5. Kumbhipaka - Cooking in pot
    /// Sin: Cooking/boiling sins
    /// Code: Resource exhaustion (CPU/memory burning)
    Kumbhipaka = 5,

    /// 6. Kalasutra - Black thread
    /// Sin: Disrespecting elders (threads)
    /// Code: Thread safety violations, deadlocks
    Kalasutra = 6,

    /// 7. Asipatravana - Sword-leaf forest
    /// Sin: Abandoning dharma
    /// Code: Buffer overflow (sharp edges cut)
    Asipatravana = 7,

    /// 8. Sukaramukha - Pig-faced
    /// Sin: Oppressing subjects (rulers)
    /// Code: Code smell, dirty/unmaintainable code
    Sukaramukha = 8,

    /// 9. Andhakupa - Dark well
    /// Sin: Oppressing good people
    /// Code: Null pointer dereference (dark abyss)
    Andhakupa = 9,

    /// 10. Krimibhaksha - Worm-eating
    /// Sin: Dishonoring guests
    /// Code: Memory corruption (worms eating data)
    Krimibhaksha = 10,

    // ═══════════════════════════════════════════════════════════════════════
    // CONCURRENCY VIOLATIONS (Hells 11-16)
    // ═══════════════════════════════════════════════════════════════════════
    /// 11. Sandamsha - Tongs torture
    /// Sin: Adultery
    /// Code: Race condition (multiple access to same resource)
    Sandamsha = 11,

    /// 12. Taptasurmi - Hot iron
    /// Sin: Unnatural intercourse
    /// Code: Type confusion, casting violations
    Taptasurmi = 12,

    /// 13. Vajrakantaka - Diamond needles
    /// Sin: Intercourse with animals
    /// Code: Accessing foreign memory (FFI violations)
    Vajrakantaka = 13,

    /// 14. Vaitarani - Filthy river
    /// Sin: Abusing power, adultery
    /// Code: Tainted data crossing security boundary
    Vaitarani = 14,

    /// 15. Puyoda - Pus well
    /// Sin: Deceiving women, false promises
    /// Code: Data corruption, malformed structures
    Puyoda = 15,

    /// 16. Pranarodha - Breath stoppage
    /// Sin: Imprisoning innocents
    /// Code: Deadlock (process cannot breathe/continue)
    Pranarodha = 16,

    // ═══════════════════════════════════════════════════════════════════════
    // SECURITY VIOLATIONS (Hells 17-23)
    // ═══════════════════════════════════════════════════════════════════════
    /// 17. Visasana - Slaughterhouse
    /// Sin: Selling wife, imprisoning
    /// Code: Forced process termination, kill -9
    Visasana = 17,

    /// 18. Lalabhaksha - Semen sea
    /// Sin: Lustful acts with wife
    /// Code: Inappropriate data exposure
    Lalabhaksha = 18,

    /// 19. Sarameyadana - Dog-bite
    /// Sin: Poisoning food, mass slaughter
    /// Code: Wild pointer, dangling reference
    Sarameyadana = 19,

    /// 20. Avichi - Waveless
    /// Sin: False witness, perjury
    /// Code: Stack overflow (no waves = no stack space)
    Avichi = 20,

    /// 21. Ayahpana - Drinking molten iron
    /// Sin: Consuming alcohol
    /// Code: Consuming poisoned/malicious data
    Ayahpana = 21,

    /// 22. Ksharakardama - Alkali mud
    /// Sin: Pride, false teaching
    /// Code: Insecure credential storage (plaintext passwords)
    Ksharakardama = 22,

    /// 23. Raksogana - Demon gang
    /// Sin: Sacrificing humans/animals
    /// Code: Malicious code injection, RCE
    Raksogana = 23,

    // ═══════════════════════════════════════════════════════════════════════
    // RESOURCE VIOLATIONS (Hells 24-28)
    // ═══════════════════════════════════════════════════════════════════════
    /// 24. Sulaprota - Spear impalement
    /// Sin: Killing animals for pleasure
    /// Code: Code injection attack (spear through body)
    Sulaprota = 24,

    /// 25. Dandasuka - Snake biting
    /// Sin: Imprisoning/starving people
    /// Code: Logic error causing starvation (deadlock variant)
    Dandasuka = 25,

    /// 26. Vatarodha - Weapon torture
    /// Sin: Persecuting forest animals
    /// Code: Denial of service attack
    Vatarodha = 26,

    /// 27. Paryavartana - Bird torture
    /// Sin: Denying food to hungry
    /// Code: Resource denial (refusing allocation)
    Paryavartana = 27,

    /// 28. Suchimukha - Needle torture
    /// Sin: Pride, miserliness, not repaying debts
    /// Code: Memory leak (allocated but never freed = unpaid debt)
    Suchimukha = 28,
}

impl Naraka {
    /// Get the Sanskrit name of this Naraka
    pub fn name(&self) -> &'static str {
        match self {
            Naraka::Tamisram => "तमिस्रम् (Tamisram)",
            Naraka::Andhatamisram => "अन्धतमिस्रम् (Andhatamisram)",
            Naraka::Raurava => "रौरव (Raurava)",
            Naraka::Maharaurava => "महारौरव (Maharaurava)",
            Naraka::Kumbhipaka => "कुम्भीपाक (Kumbhipaka)",
            Naraka::Kalasutra => "कालसूत्र (Kalasutra)",
            Naraka::Asipatravana => "असिपत्रवन (Asipatravana)",
            Naraka::Sukaramukha => "सूकरमुख (Sukaramukha)",
            Naraka::Andhakupa => "अन्धकूप (Andhakupa)",
            Naraka::Krimibhaksha => "क्रिमिभक्ष (Krimibhaksha)",
            Naraka::Sandamsha => "सन्दंश (Sandamsha)",
            Naraka::Taptasurmi => "तप्तसूर्मि (Taptasurmi)",
            Naraka::Vajrakantaka => "वज्रकण्टक (Vajrakantaka)",
            Naraka::Vaitarani => "वैतरणी (Vaitarani)",
            Naraka::Puyoda => "पूयोद (Puyoda)",
            Naraka::Pranarodha => "प्राणरोध (Pranarodha)",
            Naraka::Visasana => "विशसन (Visasana)",
            Naraka::Lalabhaksha => "लालाभक्ष (Lalabhaksha)",
            Naraka::Sarameyadana => "सारमेयादन (Sarameyadana)",
            Naraka::Avichi => "अवीचि (Avichi)",
            Naraka::Ayahpana => "अयःपान (Ayahpana)",
            Naraka::Ksharakardama => "क्षारकर्दम (Ksharakardama)",
            Naraka::Raksogana => "रक्षोगण (Raksogana)",
            Naraka::Sulaprota => "शूलप्रोत (Sulaprota)",
            Naraka::Dandasuka => "दण्डशूक (Dandasuka)",
            Naraka::Vatarodha => "वातरोध (Vatarodha)",
            Naraka::Paryavartana => "पर्यावर्तन (Paryavartana)",
            Naraka::Suchimukha => "सूचीमुख (Suchimukha)",
        }
    }

    /// Alias for name() - get the Sanskrit name of this Naraka
    pub fn sanskrit_name(&self) -> &'static str {
        self.name()
    }

    /// Get severity level of this Naraka
    pub fn severity(&self) -> Severity {
        match self {
            // CRITICAL - Security/Memory safety
            Naraka::Raurava
            | Naraka::Maharaurava
            | Naraka::Vaitarani
            | Naraka::Raksogana
            | Naraka::Sulaprota => Severity::Critical,

            // ERROR - Will cause runtime failure
            Naraka::Tamisram
            | Naraka::Asipatravana
            | Naraka::Andhakupa
            | Naraka::Sandamsha
            | Naraka::Avichi
            | Naraka::Pranarodha
            | Naraka::Kalasutra
            | Naraka::Sarameyadana
            | Naraka::Krimibhaksha => Severity::Error,

            // WARNING - Code smell, potential issue
            Naraka::Andhatamisram
            | Naraka::Sukaramukha
            | Naraka::Suchimukha
            | Naraka::Puyoda
            | Naraka::Lalabhaksha => Severity::Warning,

            // Default to Error for others
            _ => Severity::Error,
        }
    }

    /// Get punishment duration
    pub fn duration(&self) -> Duration {
        match self.severity() {
            Severity::Critical => Duration::Permanent,
            Severity::Error => Duration::UntilFixed,
            Severity::Warning => Duration::Temporary,
            Severity::Hint => Duration::Temporary,
        }
    }

    /// Get the sin description (what the code did wrong)
    pub fn sin_description(&self) -> &'static str {
        match self {
            Naraka::Tamisram => "Memory theft: use-after-free or double-free",
            Naraka::Andhatamisram => "API contract violation: breaking promises",
            Naraka::Raurava => "Violence: panic/crash causing process death",
            Naraka::Maharaurava => "Killing: forced child process termination",
            Naraka::Kumbhipaka => "Resource exhaustion: CPU/memory burning",
            Naraka::Kalasutra => "Thread violation: deadlock or thread safety",
            Naraka::Asipatravana => "Buffer overflow: writing beyond bounds",
            Naraka::Sukaramukha => "Code smell: dirty/unmaintainable code",
            Naraka::Andhakupa => "Null dereference: falling into the dark well",
            Naraka::Krimibhaksha => "Memory corruption: data worms eating memory",
            Naraka::Sandamsha => "Race condition: multiple access to same resource",
            Naraka::Taptasurmi => "Type confusion: invalid type casting",
            Naraka::Vajrakantaka => "FFI violation: accessing foreign memory unsafely",
            Naraka::Vaitarani => "Tainted data: crossing security boundary unpurified",
            Naraka::Puyoda => "Data corruption: malformed or invalid structures",
            Naraka::Pranarodha => "Deadlock: process cannot breathe/continue",
            Naraka::Visasana => "Forced termination: kill -9 or abort",
            Naraka::Lalabhaksha => "Data exposure: inappropriate data leakage",
            Naraka::Sarameyadana => "Wild pointer: dangling reference bite",
            Naraka::Avichi => "Stack overflow: no stack space remaining",
            Naraka::Ayahpana => "Poisoned data: consuming malicious input",
            Naraka::Ksharakardama => "Insecure storage: plaintext credentials",
            Naraka::Raksogana => "Code injection: RCE or malicious code",
            Naraka::Sulaprota => "Injection attack: spear through the code",
            Naraka::Dandasuka => "Starvation: resource starvation deadlock",
            Naraka::Vatarodha => "DoS attack: denial of service",
            Naraka::Paryavartana => "Resource denial: refusing allocation",
            Naraka::Suchimukha => "Memory leak: allocated but never freed (unpaid debt)",
        }
    }

    /// Get the moksha (redemption) path
    pub fn redemption_path(&self) -> &'static str {
        match self {
            Naraka::Tamisram => "Use borrowed reference (-b) instead of stealing ownership",
            Naraka::Andhatamisram => "Honor API contracts; implement all required traits",
            Naraka::Raurava => "Add proper error handling; use Phala<T, Truṭi> instead of panic",
            Naraka::Maharaurava => "Use graceful shutdown; wait for child processes",
            Naraka::Kumbhipaka => "Add resource limits; use bounded queues and timeouts",
            Naraka::Kalasutra => "Use proper synchronization; add -sūtra (thread-safe) suffix",
            Naraka::Asipatravana => "Check bounds before access; use bounds-checked indexing",
            Naraka::Sukaramukha => "Refactor code; follow naming conventions; add documentation",
            Naraka::Andhakupa => "Add null check or use Option<T> type; use -pratyakṣa validation",
            Naraka::Krimibhaksha => "Use memory-safe operations; validate before write",
            Naraka::Sandamsha => "Add proper locking; use atomic operations or -sūtra suffix",
            Naraka::Taptasurmi => "Use proper type casting; validate with -pratyakṣa",
            Naraka::Vajrakantaka => "Wrap FFI calls in unsafe block; validate foreign data",
            Naraka::Vaitarani => "Apply śuddhi-kri() sanitizer before crossing boundary",
            Naraka::Puyoda => "Validate data structures; use constructor validation",
            Naraka::Pranarodha => "Use lock ordering; add timeout to blocking operations",
            Naraka::Visasana => "Use graceful shutdown; handle signals properly",
            Naraka::Lalabhaksha => "Encrypt sensitive data; use proper access controls",
            Naraka::Sarameyadana => "Initialize pointers; use Option<&T> instead of raw pointers",
            Naraka::Avichi => "Reduce recursion depth; use iterative algorithms",
            Naraka::Ayahpana => "Validate all external input; use input sanitization",
            Naraka::Ksharakardama => "Use secure credential storage; hash passwords",
            Naraka::Raksogana => "Validate and sanitize all user input; use parameterized queries",
            Naraka::Sulaprota => "Use input validation; escape special characters",
            Naraka::Dandasuka => "Use fair scheduling; add resource quotas",
            Naraka::Vatarodha => "Add rate limiting; use request throttling",
            Naraka::Paryavartana => "Handle allocation failure; use fallback allocators",
            Naraka::Suchimukha => "Call mukta() to free allocated memory; use RAII patterns",
        }
    }

    /// Get all Narakas in order
    pub fn all() -> &'static [Naraka] {
        &[
            Naraka::Tamisram,
            Naraka::Andhatamisram,
            Naraka::Raurava,
            Naraka::Maharaurava,
            Naraka::Kumbhipaka,
            Naraka::Kalasutra,
            Naraka::Asipatravana,
            Naraka::Sukaramukha,
            Naraka::Andhakupa,
            Naraka::Krimibhaksha,
            Naraka::Sandamsha,
            Naraka::Taptasurmi,
            Naraka::Vajrakantaka,
            Naraka::Vaitarani,
            Naraka::Puyoda,
            Naraka::Pranarodha,
            Naraka::Visasana,
            Naraka::Lalabhaksha,
            Naraka::Sarameyadana,
            Naraka::Avichi,
            Naraka::Ayahpana,
            Naraka::Ksharakardama,
            Naraka::Raksogana,
            Naraka::Sulaprota,
            Naraka::Dandasuka,
            Naraka::Vatarodha,
            Naraka::Paryavartana,
            Naraka::Suchimukha,
        ]
    }

    /// Get the Sanskrit script name only
    fn sanskrit_script(&self) -> &'static str {
        match self {
            Naraka::Tamisram => "तमिस्रम्",
            Naraka::Andhatamisram => "अन्धतमिस्रम्",
            Naraka::Raurava => "रौरव",
            Naraka::Maharaurava => "महारौरव",
            Naraka::Kumbhipaka => "कुम्भीपाक",
            Naraka::Kalasutra => "कालसूत्र",
            Naraka::Asipatravana => "असिपत्रवन",
            Naraka::Sukaramukha => "सूकरमुख",
            Naraka::Andhakupa => "अन्धकूप",
            Naraka::Krimibhaksha => "क्रिमिभक्ष",
            Naraka::Sandamsha => "सन्दंश",
            Naraka::Taptasurmi => "तप्तसूर्मि",
            Naraka::Vajrakantaka => "वज्रकण्टक",
            Naraka::Vaitarani => "वैतरणी",
            Naraka::Puyoda => "पूयोद",
            Naraka::Pranarodha => "प्राणरोध",
            Naraka::Visasana => "विशसन",
            Naraka::Lalabhaksha => "लालाभक्ष",
            Naraka::Sarameyadana => "सारमेयादन",
            Naraka::Avichi => "अवीचि",
            Naraka::Ayahpana => "अयःपान",
            Naraka::Ksharakardama => "क्षारकर्दम",
            Naraka::Raksogana => "रक्षोगण",
            Naraka::Sulaprota => "शूलप्रोत",
            Naraka::Dandasuka => "दण्डशूक",
            Naraka::Vatarodha => "वातरोध",
            Naraka::Paryavartana => "पर्यावर्तन",
            Naraka::Suchimukha => "सूचीमुख",
        }
    }

    /// Get the IAST transliteration only
    fn iast_name(&self) -> &'static str {
        match self {
            Naraka::Tamisram => "Tamisram",
            Naraka::Andhatamisram => "Andhatamisram",
            Naraka::Raurava => "Raurava",
            Naraka::Maharaurava => "Mahāraurava",
            Naraka::Kumbhipaka => "Kumbhīpāka",
            Naraka::Kalasutra => "Kālasūtra",
            Naraka::Asipatravana => "Asipattravana",
            Naraka::Sukaramukha => "Sūkaramukha",
            Naraka::Andhakupa => "Andhakūpa",
            Naraka::Krimibhaksha => "Krimibhakṣa",
            Naraka::Sandamsha => "Sandaṃśa",
            Naraka::Taptasurmi => "Taptasūrmi",
            Naraka::Vajrakantaka => "Vajrakaṇṭaka",
            Naraka::Vaitarani => "Vaitaraṇī",
            Naraka::Puyoda => "Pūyoda",
            Naraka::Pranarodha => "Prāṇarodha",
            Naraka::Visasana => "Viśasana",
            Naraka::Lalabhaksha => "Lālābhakṣa",
            Naraka::Sarameyadana => "Sārameyādana",
            Naraka::Avichi => "Avīci",
            Naraka::Ayahpana => "Ayaḥpāna",
            Naraka::Ksharakardama => "Kṣārakardama",
            Naraka::Raksogana => "Rakṣogaṇa",
            Naraka::Sulaprota => "Śūlaprota",
            Naraka::Dandasuka => "Daṇḍaśūka",
            Naraka::Vatarodha => "Vātarodha",
            Naraka::Paryavartana => "Paryāvartana",
            Naraka::Suchimukha => "Sūcīmukha",
        }
    }

    /// Get English description of the punishment
    fn english_name(&self) -> &'static str {
        match self {
            Naraka::Tamisram => "Heavy Flogging",
            Naraka::Andhatamisram => "Darkness Flogging",
            Naraka::Raurava => "Screaming Hell",
            Naraka::Maharaurava => "Great Screaming",
            Naraka::Kumbhipaka => "Pot Cooking",
            Naraka::Kalasutra => "Black Thread",
            Naraka::Asipatravana => "Sword-Leaf Forest",
            Naraka::Sukaramukha => "Pig Face",
            Naraka::Andhakupa => "Dark Well",
            Naraka::Krimibhaksha => "Worm Eating",
            Naraka::Sandamsha => "Tongs Torture",
            Naraka::Taptasurmi => "Hot Iron",
            Naraka::Vajrakantaka => "Diamond Needles",
            Naraka::Vaitarani => "Filthy River",
            Naraka::Puyoda => "Pus Well",
            Naraka::Pranarodha => "Breath Stoppage",
            Naraka::Visasana => "Slaughterhouse",
            Naraka::Lalabhaksha => "Semen Sea",
            Naraka::Sarameyadana => "Dog Bite",
            Naraka::Avichi => "Waveless",
            Naraka::Ayahpana => "Molten Iron Drink",
            Naraka::Ksharakardama => "Alkali Mud",
            Naraka::Raksogana => "Demon Gang",
            Naraka::Sulaprota => "Spear Impalement",
            Naraka::Dandasuka => "Snake Biting",
            Naraka::Vatarodha => "Weapon Torture",
            Naraka::Paryavartana => "Bird Torture",
            Naraka::Suchimukha => "Needle Torture",
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// v10.0 UNIFIED TRAIT IMPLEMENTATIONS
// ═══════════════════════════════════════════════════════════════════════════════

impl SanskritNamed for Naraka {
    fn sanskrit(&self) -> &'static str {
        self.sanskrit_script()
    }

    fn iast(&self) -> &'static str {
        self.iast_name()
    }

    fn english(&self) -> &'static str {
        self.english_name()
    }
}

impl SanskritDescribed for Naraka {
    fn meaning(&self) -> &'static str {
        self.sin_description()
    }

    fn explanation(&self) -> &'static str {
        self.redemption_path()
    }

    fn mantra(&self) -> Option<&'static str> {
        // Each Naraka has a purification mantra
        Some(match self {
            // Memory violations - Vishnu mantras for protection
            Naraka::Tamisram => "ॐ नमो नारायणाय (Om Namo Nārāyaṇāya)",
            Naraka::Andhakupa => "ॐ विष्णवे नमः (Om Viṣṇave Namaḥ)",
            Naraka::Asipatravana => "ॐ दामोदराय नमः (Om Dāmodarāya Namaḥ)",
            Naraka::Krimibhaksha => "ॐ वासुदेवाय नमः (Om Vāsudevāya Namaḥ)",
            Naraka::Suchimukha => "ॐ मुक्ताय नमः (Om Muktāya Namaḥ)",

            // Concurrency violations - Shiva mantras for transformation
            Naraka::Kalasutra => "ॐ नमः शिवाय (Om Namaḥ Śivāya)",
            Naraka::Pranarodha => "ॐ त्र्यम्बकं यजामहे (Om Tryambakaṃ Yajāmahe)",
            Naraka::Sandamsha => "ॐ महादेवाय नमः (Om Mahādevāya Namaḥ)",

            // Security violations - Durga mantras for protection
            Naraka::Vaitarani => "ॐ दुं दुर्गायै नमः (Om Duṃ Durgāyai Namaḥ)",
            Naraka::Raksogana => "ॐ ऐं ह्रीं क्लीं (Om Aiṃ Hrīṃ Klīṃ)",
            Naraka::Ksharakardama => "ॐ शक्तये नमः (Om Śaktaye Namaḥ)",

            // Process violations - Yama mantras for judgment
            Naraka::Raurava | Naraka::Maharaurava => "ॐ यमाय नमः (Om Yamāya Namaḥ)",
            Naraka::Visasana => "ॐ धर्मराजाय नमः (Om Dharmarājāya Namaḥ)",

            // Default - generic purification
            _ => "ॐ शुद्धि कुरु स्वाहा (Om Śuddhi Kuru Svāhā)",
        })
    }

    fn category(&self) -> &'static str {
        match self {
            // Memory violations (1-10)
            Naraka::Tamisram
            | Naraka::Andhatamisram
            | Naraka::Raurava
            | Naraka::Maharaurava
            | Naraka::Kumbhipaka
            | Naraka::Kalasutra
            | Naraka::Asipatravana
            | Naraka::Sukaramukha
            | Naraka::Andhakupa
            | Naraka::Krimibhaksha => "Memory Violations (स्मृति)",

            // Concurrency violations (11-16)
            Naraka::Sandamsha
            | Naraka::Taptasurmi
            | Naraka::Vajrakantaka
            | Naraka::Vaitarani
            | Naraka::Puyoda
            | Naraka::Pranarodha => "Concurrency Violations (सूत्र)",

            // Security violations (17-23)
            Naraka::Visasana
            | Naraka::Lalabhaksha
            | Naraka::Sarameyadana
            | Naraka::Avichi
            | Naraka::Ayahpana
            | Naraka::Ksharakardama
            | Naraka::Raksogana => "Security Violations (सुरक्षा)",

            // Resource violations (24-28)
            Naraka::Sulaprota
            | Naraka::Dandasuka
            | Naraka::Vatarodha
            | Naraka::Paryavartana
            | Naraka::Suchimukha => "Resource Violations (संसाधन)",
        }
    }
}

impl PhilosophicalEnum for Naraka {
    fn all() -> &'static [Self] {
        Naraka::all()
    }

    fn count() -> usize {
        28
    }

    fn index(&self) -> usize {
        (*self as u8 - 1) as usize
    }

    fn ordinal(&self) -> usize {
        *self as usize
    }

    fn next(&self) -> Self {
        let idx = self.index();
        Self::all()[(idx + 1) % 28]
    }

    fn prev(&self) -> Self {
        let idx = self.index();
        Self::all()[(idx + 28 - 1) % 28]
    }

    fn from_index(index: usize) -> Option<Self> {
        Self::all().get(index).copied()
    }
}

/// Garuda-style error message
#[derive(Debug, Clone)]
pub struct NarakaError {
    /// Which Naraka this violation falls into
    pub naraka: Naraka,
    /// Location in source code
    pub location: Span,
    /// What the code did wrong (sin)
    pub sin: String,
    /// Consequence (punishment)
    pub punishment: String,
    /// How to fix (penance)
    pub penance: String,
    /// Additional context
    pub context: Option<String>,
}

impl NarakaError {
    /// Create a new NarakaError
    pub fn new(naraka: Naraka, location: Span, sin: impl Into<String>) -> Self {
        Self {
            naraka,
            location,
            sin: sin.into(),
            punishment: Self::default_punishment(naraka),
            penance: naraka.redemption_path().to_string(),
            context: None,
        }
    }

    /// Create from a violation
    pub fn from_violation(violation: &super::yama::Violation, naraka: Naraka) -> Self {
        Self::new(naraka, violation.location.clone(), &violation.evidence)
    }

    /// Create from a Preta violation
    pub fn from_preta(preta: &super::preta::PretaViolation) -> Self {
        Self::new(
            Naraka::Suchimukha,
            preta.allocated_at.clone(),
            format!("Memory leak: '{}' allocated but never freed", preta.symbol),
        )
    }

    /// Create from a Ghost detection
    pub fn from_ghost(ghost: &super::preta::Ghost) -> Self {
        Self::new(
            Naraka::Suchimukha,
            ghost.location.clone().into(),
            &ghost.description,
        )
    }

    /// Create from a Vaitarani violation
    pub fn from_vaitarani(violation: &super::vaitarani::VaitaraniViolation) -> Self {
        Self::new(
            Naraka::Vaitarani,
            violation.location.clone(),
            &violation.message,
        )
    }

    fn default_punishment(naraka: Naraka) -> String {
        match naraka.severity() {
            Severity::Critical => "Compilation blocked; security violation".to_string(),
            Severity::Error => "Compilation blocked; fix required".to_string(),
            Severity::Warning => "Warning issued; fix recommended".to_string(),
            Severity::Hint => "Suggestion provided".to_string(),
        }
    }

    /// Add context to the error
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }
}

impl fmt::Display for NarakaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "╔═══════════════════════════════════════════════════════════════╗"
        )?;
        writeln!(
            f,
            "║ NARAKA: {} ({})",
            self.naraka.name(),
            self.naraka.severity()
        )?;
        writeln!(
            f,
            "║ LOCATION: {:?}:{}",
            self.location.source, self.location.start
        )?;
        writeln!(
            f,
            "╠═══════════════════════════════════════════════════════════════╣"
        )?;
        writeln!(f, "║ SIN (Violation):")?;
        writeln!(f, "║   {}", self.sin)?;
        writeln!(f, "║")?;
        writeln!(f, "║ PUNISHMENT (Consequence):")?;
        writeln!(f, "║   {}", self.punishment)?;
        writeln!(f, "║")?;
        writeln!(f, "║ PENANCE (Fix):")?;
        writeln!(f, "║   {}", self.penance)?;
        if let Some(ctx) = &self.context {
            writeln!(f, "║")?;
            writeln!(f, "║ CONTEXT:")?;
            writeln!(f, "║   {}", ctx)?;
        }
        writeln!(
            f,
            "╚═══════════════════════════════════════════════════════════════╝"
        )
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    // ═══════════════════════════════════════════════════════════════════════════
    // BASIC NARAKA TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_naraka_count() {
        assert_eq!(Naraka::all().len(), 28);
    }

    #[test]
    fn test_naraka_name_contains_both_scripts() {
        // name() returns "Sanskrit (IAST)" format
        let name = Naraka::Tamisram.name();
        assert!(name.contains("तमिस्रम्"));
        assert!(name.contains("Tamisram"));
    }

    #[test]
    fn test_naraka_severity_levels() {
        assert_eq!(Naraka::Vaitarani.severity(), Severity::Critical);
        assert_eq!(Naraka::Tamisram.severity(), Severity::Error);
        assert_eq!(Naraka::Sukaramukha.severity(), Severity::Warning);
    }

    #[test]
    fn test_naraka_ordinal_values() {
        // Each Naraka has repr(u8) with specific values
        assert_eq!(Naraka::Tamisram as u8, 1);
        assert_eq!(Naraka::Suchimukha as u8, 28);
        assert_eq!(Naraka::Vaitarani as u8, 14);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // v10.0 TRAIT IMPLEMENTATION TESTS
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_naraka_sanskrit_named_trait() {
        let n = Naraka::Tamisram;
        assert_eq!(SanskritNamed::sanskrit(&n), "तमिस्रम्");
        assert_eq!(SanskritNamed::iast(&n), "Tamisram");
        assert_eq!(SanskritNamed::english(&n), "Heavy Flogging");

        let n2 = Naraka::Vaitarani;
        assert_eq!(SanskritNamed::sanskrit(&n2), "वैतरणी");
        assert_eq!(SanskritNamed::iast(&n2), "Vaitaraṇī");
        assert_eq!(SanskritNamed::english(&n2), "Filthy River");
    }

    #[test]
    fn test_naraka_sanskrit_described_trait() {
        let n = Naraka::Suchimukha;
        assert!(n.meaning().contains("Memory leak"));
        assert!(n.explanation().contains("mukta()"));
        assert!(n.mantra().is_some());
        assert_eq!(n.category(), "Resource Violations (संसाधन)");
    }

    #[test]
    fn test_naraka_philosophical_enum_trait() {
        // Test count
        assert_eq!(Naraka::count(), 28);

        // Test index (0-based)
        assert_eq!(Naraka::Tamisram.index(), 0);
        assert_eq!(Naraka::Suchimukha.index(), 27);

        // Test ordinal (1-based, same as repr)
        assert_eq!(Naraka::Tamisram.ordinal(), 1);
        assert_eq!(Naraka::Suchimukha.ordinal(), 28);

        // Test navigation (wrapping)
        assert_eq!(Naraka::Tamisram.next(), Naraka::Andhatamisram);
        assert_eq!(Naraka::Suchimukha.next(), Naraka::Tamisram);
        assert_eq!(Naraka::Tamisram.prev(), Naraka::Suchimukha);
        assert_eq!(Naraka::Andhatamisram.prev(), Naraka::Tamisram);

        // Test from_index
        assert_eq!(Naraka::from_index(0), Some(Naraka::Tamisram));
        assert_eq!(Naraka::from_index(27), Some(Naraka::Suchimukha));
        assert_eq!(Naraka::from_index(28), None);
    }

    #[test]
    fn test_naraka_all_have_mantras() {
        for naraka in Naraka::all() {
            assert!(naraka.mantra().is_some(), "Missing mantra for {:?}", naraka);
        }
    }

    #[test]
    fn test_naraka_categories_coverage() {
        let mut memory_count = 0;
        let mut concurrency_count = 0;
        let mut security_count = 0;
        let mut resource_count = 0;

        for naraka in Naraka::all() {
            match naraka.category() {
                "Memory Violations (स्मृति)" => memory_count += 1,
                "Concurrency Violations (सूत्र)" => concurrency_count += 1,
                "Security Violations (सुरक्षा)" => security_count += 1,
                "Resource Violations (संसाधन)" => resource_count += 1,
                other => panic!("Unknown category: {}", other),
            }
        }

        assert_eq!(memory_count, 10, "Memory violations should be 1-10");
        assert_eq!(
            concurrency_count, 6,
            "Concurrency violations should be 11-16"
        );
        assert_eq!(security_count, 7, "Security violations should be 17-23");
        assert_eq!(resource_count, 5, "Resource violations should be 24-28");
    }

    #[test]
    fn test_naraka_all_have_redemption_paths() {
        for naraka in Naraka::all() {
            let path = naraka.redemption_path();
            assert!(!path.is_empty(), "Empty redemption for {:?}", naraka);
            // Redemption paths should contain some instructional word (any form of guidance)
            assert!(
                path.len() > 10,
                "Redemption should be descriptive, got: {}",
                path
            );
        }
    }

    #[test]
    fn test_naraka_error_creation() {
        use crate::errors::{SourceId, Span};

        let error = NarakaError::new(
            Naraka::Suchimukha,
            Span::new(SourceId(0), 0, 10),
            "Memory leak detected",
        );

        assert_eq!(error.naraka, Naraka::Suchimukha);
        assert!(error.sin.contains("Memory leak"));
        assert!(error.penance.contains("mukta()"));
    }

    #[test]
    fn test_naraka_navigation_all_28() {
        // Starting from first, navigate through all 28 and return to start
        let mut current = Naraka::Tamisram;
        for _ in 0..27 {
            current = current.next();
            assert_ne!(current, Naraka::Tamisram, "Should not loop before 28 steps");
        }
        current = current.next();
        assert_eq!(
            current,
            Naraka::Tamisram,
            "Should return to start after 28 steps"
        );
    }
}
