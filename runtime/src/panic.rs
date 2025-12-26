//! Naraka Panic Handler (नरक भय प्रबन्धन)
//!
//! Error handling based on Garuda Purana's 28 Narakas (hells).
//! Each type of programming error corresponds to a specific Naraka,
//! providing meaningful error messages with philosophical context.
//!
//! ## Philosophy
//! In Hindu cosmology, Naraka is not eternal damnation but a temporary
//! purification process. Similarly, our panic handler aims to:
//! 1. Identify the "sin" (error type)
//! 2. Prescribe the "penance" (fix suggestion)
//! 3. Enable "liberation" (recovery or graceful shutdown)

#[cfg(feature = "std")]
use std::backtrace::Backtrace;
#[cfg(feature = "std")]
use std::panic::{self, PanicHookInfo};

/// Naraka (नरक) - Classification of programming sins
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Naraka {
    // Memory-related Narakas
    /// Tamisra (तामिस्र) - Darkness - Null pointer dereference
    Tamisra = 0,
    /// Andhatamisra (अन्धतामिस्र) - Blinding darkness - Dangling pointer
    Andhatamisra = 1,
    /// Raurava (रौरव) - Place of weeping - Memory leak (Preta)
    Raurava = 2,
    /// Maharaurava (महारौरव) - Great weeping - Massive leak
    Maharaurava = 3,

    // Type-related Narakas
    /// Kumbhipaka (कुम्भीपाक) - Boiling pot - Type mismatch
    Kumbhipaka = 4,
    /// Kalasutra (कालसूत्र) - Thread of time - Lifetime violation
    Kalasutra = 5,

    // Concurrency Narakas
    /// Asipatravana (असिपत्रवन) - Forest of swords - Data race
    Asipatravana = 6,
    /// Sukaramukha (शूकरमुख) - Pig-faced - Deadlock
    Sukaramukha = 7,

    // Bounds/Overflow Narakas
    /// Andhakupa (अन्धकूप) - Blind well - Array bounds violation
    Andhakupa = 8,
    /// Krimibhojana (कृमिभोजन) - Worm-eaten - Integer overflow
    Krimibhojana = 9,

    // I/O Narakas
    /// Sandansa (सन्दंश) - Pincers - File handle exhaustion
    Sandansa = 10,
    /// Taptasurmi (तप्तसूर्मि) - Hot iron - Network timeout
    Taptasurmi = 11,

    // Logic Narakas
    /// Vajrakantaka (वज्रकण्टक) - Diamond thorns - Assertion failure
    Vajrakantaka = 12,
    /// Salmali (शाल्मली) - Silk-cotton tree - Unreachable code reached
    Salmali = 13,

    // Resource Narakas
    /// Paryavartana (पर्यावर्तन) - Revolving - Stack overflow
    Paryavartana = 14,
    /// Kudmala (कुद्मल) - Bud - OOM (Out of Memory)
    Kudmala = 15,

    // General
    /// Avichi (अवीचि) - Waveless - Unspecified panic
    Avichi = 27,
}

impl Naraka {
    /// Get Sanskrit name with meaning
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Tamisra => "तामिस्र (Tamisra) - Realm of Darkness",
            Self::Andhatamisra => "अन्धतामिस्र (Andhatamisra) - Blinding Darkness",
            Self::Raurava => "रौरव (Raurava) - Place of Weeping",
            Self::Maharaurava => "महारौरव (Maharaurava) - Great Suffering",
            Self::Kumbhipaka => "कुम्भीपाक (Kumbhipaka) - Boiling Pot",
            Self::Kalasutra => "कालसूत्र (Kalasutra) - Thread of Time",
            Self::Asipatravana => "असिपत्रवन (Asipatravana) - Forest of Swords",
            Self::Sukaramukha => "शूकरमुख (Sukaramukha) - Realm of Deadlock",
            Self::Andhakupa => "अन्धकूप (Andhakupa) - Blind Well",
            Self::Krimibhojana => "कृमिभोजन (Krimibhojana) - Overflow Torment",
            Self::Sandansa => "सन्दंश (Sandansa) - Pincers",
            Self::Taptasurmi => "तप्तसूर्मि (Taptasurmi) - Hot Iron",
            Self::Vajrakantaka => "वज्रकण्टक (Vajrakantaka) - Diamond Thorns",
            Self::Salmali => "शाल्मली (Salmali) - Unreachable Realm",
            Self::Paryavartana => "पर्यावर्तन (Paryavartana) - Revolving Realm",
            Self::Kudmala => "कुद्मल (Kudmala) - Realm of Exhaustion",
            Self::Avichi => "अवीचि (Avichi) - Waveless Void",
        }
    }

    /// Get the programming sin (error description)
    pub const fn sin(&self) -> &'static str {
        match self {
            Self::Tamisra => "Attempted to access null/uninitialized memory",
            Self::Andhatamisra => "Used memory after it was freed (use-after-free)",
            Self::Raurava => "Memory was allocated but never freed (leak)",
            Self::Maharaurava => "Catastrophic memory leak consuming all resources",
            Self::Kumbhipaka => "Types do not match - incompatible conversion",
            Self::Kalasutra => "Lifetime violation - borrowed value does not live long enough",
            Self::Asipatravana => "Data race - multiple threads accessing without synchronization",
            Self::Sukaramukha => "Deadlock - threads waiting on each other forever",
            Self::Andhakupa => "Array index out of bounds",
            Self::Krimibhojana => "Integer overflow/underflow",
            Self::Sandansa => "Too many open file handles",
            Self::Taptasurmi => "Network operation timed out",
            Self::Vajrakantaka => "Assertion failed - invariant violated",
            Self::Salmali => "Reached code that should be unreachable",
            Self::Paryavartana => "Stack overflow - too deep recursion",
            Self::Kudmala => "Out of memory - allocation failed",
            Self::Avichi => "Unspecified error",
        }
    }

    /// Get the penance (fix suggestion)
    pub const fn penance(&self) -> &'static str {
        match self {
            Self::Tamisra => "Check for None/null before access. Use Option<T> safely.",
            Self::Andhatamisra => "Ensure proper ownership. Don't use references after drop.",
            Self::Raurava => "Pair every allocation with deallocation. Use RAII patterns.",
            Self::Maharaurava => "Audit allocation patterns. Consider memory pooling.",
            Self::Kumbhipaka => "Verify type compatibility. Use explicit conversions.",
            Self::Kalasutra => "Extend the lifetime or restructure ownership.",
            Self::Asipatravana => "Use Mutex, RwLock, or atomic operations for shared data.",
            Self::Sukaramukha => "Review lock ordering. Use try_lock with timeout.",
            Self::Andhakupa => "Check array length before access. Use .get() for safety.",
            Self::Krimibhojana => "Use checked_add/checked_mul or saturating operations.",
            Self::Sandansa => "Close files when done. Use RAII for handle management.",
            Self::Taptasurmi => "Implement retry logic with exponential backoff.",
            Self::Vajrakantaka => "Review the assertion condition. Fix the invariant.",
            Self::Salmali => "Remove unreachable code or fix control flow.",
            Self::Paryavartana => "Convert to iteration or increase stack size.",
            Self::Kudmala => "Free unused memory. Use streaming for large data.",
            Self::Avichi => "Review the panic message for details.",
        }
    }

    /// Classify a panic message into Naraka
    pub fn classify(message: &str) -> Self {
        let msg_lower = message.to_lowercase();

        if msg_lower.contains("null") || msg_lower.contains("none") {
            Self::Tamisra
        } else if msg_lower.contains("use after free") || msg_lower.contains("dangling") {
            Self::Andhatamisra
        } else if msg_lower.contains("leak") || msg_lower.contains("preta") {
            Self::Raurava
        } else if msg_lower.contains("type") || msg_lower.contains("mismatch") {
            Self::Kumbhipaka
        } else if msg_lower.contains("lifetime") || msg_lower.contains("borrow") {
            Self::Kalasutra
        } else if msg_lower.contains("race") || msg_lower.contains("concurrent") {
            Self::Asipatravana
        } else if msg_lower.contains("deadlock") {
            Self::Sukaramukha
        } else if msg_lower.contains("index")
            || msg_lower.contains("bounds")
            || msg_lower.contains("range")
        {
            Self::Andhakupa
        } else if msg_lower.contains("overflow") || msg_lower.contains("underflow") {
            Self::Krimibhojana
        } else if msg_lower.contains("file") || msg_lower.contains("handle") {
            Self::Sandansa
        } else if msg_lower.contains("timeout") || msg_lower.contains("network") {
            Self::Taptasurmi
        } else if msg_lower.contains("assert") {
            Self::Vajrakantaka
        } else if msg_lower.contains("unreachable") {
            Self::Salmali
        } else if msg_lower.contains("stack overflow") || msg_lower.contains("recursion") {
            Self::Paryavartana
        } else if msg_lower.contains("memory")
            || msg_lower.contains("alloc")
            || msg_lower.contains("oom")
        {
            Self::Kudmala
        } else {
            Self::Avichi
        }
    }
}

/// Yama's Judgment (यम का न्याय) - The panic report
#[derive(Debug)]
pub struct YamaJudgment {
    pub naraka: Naraka,
    pub message: String,
    pub location: Option<String>,
    #[cfg(feature = "std")]
    pub backtrace: Option<String>,
}

impl YamaJudgment {
    /// Create judgment from panic info
    #[cfg(feature = "std")]
    pub fn from_panic(info: &PanicHookInfo) -> Self {
        let message = if let Some(s) = info.payload().downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = info.payload().downcast_ref::<String>() {
            s.clone()
        } else {
            "Unknown panic".to_string()
        };

        let location = info
            .location()
            .map(|loc| format!("{}:{}:{}", loc.file(), loc.line(), loc.column()));

        let naraka = Naraka::classify(&message);

        Self {
            naraka,
            message,
            location,
            backtrace: Some(format!("{:?}", Backtrace::capture())),
        }
    }

    /// Format the judgment for display
    pub fn format(&self) -> String {
        let mut output = String::new();

        // Header with Devanagari
        output.push_str("\n╔══════════════════════════════════════════════════════════════════╗\n");
        output.push_str("║  🕉️  जगन्नाथ नरक प्रवेश (Jagannath Naraka Entry)  🕉️           ║\n");
        output.push_str("╚══════════════════════════════════════════════════════════════════╝\n\n");

        // Naraka classification
        output.push_str(&format!("📿 नरक (Naraka): {}\n", self.naraka.name()));
        output.push_str(&format!("📿 पाप (Sin): {}\n", self.naraka.sin()));
        output.push_str(&format!(
            "📿 प्रायश्चित (Penance): {}\n\n",
            self.naraka.penance()
        ));

        // Error details
        output.push_str("━━━ Error Details ━━━\n");
        output.push_str(&format!("Message: {}\n", self.message));
        if let Some(ref loc) = self.location {
            output.push_str(&format!("Location: {}\n", loc));
        }

        // Backtrace (truncated for readability)
        #[cfg(feature = "std")]
        if let Some(ref bt) = self.backtrace {
            output.push_str("\n━━━ Backtrace (कर्म पथ) ━━━\n");
            // Take first 10 lines
            for line in bt.lines().take(15) {
                output.push_str(line);
                output.push('\n');
            }
            output.push_str("... (use RUST_BACKTRACE=full for complete trace)\n");
        }

        output.push_str("\n╭──────────────────────────────────────────────────────────────────╮\n");
        output.push_str("│  मोक्ष संभव है। Fix the sin, achieve liberation.               │\n");
        output.push_str("╰──────────────────────────────────────────────────────────────────╯\n");

        output
    }
}

/// Initialize Naraka panic handler
#[cfg(feature = "std")]
pub fn init() {
    panic::set_hook(Box::new(|info| {
        let judgment = YamaJudgment::from_panic(info);
        eprintln!("{}", judgment.format());
    }));
}

/// Panic with specific Naraka classification
#[cfg(feature = "std")]
pub fn enter_naraka(naraka: Naraka, message: &str) -> ! {
    panic!("[{}] {}: {}", naraka.name(), naraka.sin(), message);
}

/// Panic function for no_std environments
#[cfg(not(feature = "std"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    // In no_std, we can't do much - just halt
    loop {
        // Could toggle an LED or write to debug port here
        core::hint::spin_loop();
    }
}

// ============================================================================
// Sanskrit API (संस्कृत एपीआई)
// ============================================================================

/// नरक प्रवेश (naraka praveśa) - Enter Naraka (panic with classification)
#[cfg(feature = "std")]
pub fn naraka_pravesha(naraka: Naraka, sandesh: &str) -> ! {
    enter_naraka(naraka, sandesh)
}

/// भय प्रारम्भ (bhaya prārambha) - Initialize panic handler
#[cfg(feature = "std")]
pub fn bhaya_prarambha() {
    init();
}
