//! # Nava Durga - 9 Goddess Security Layers (v6.0)
//!
//! This module implements the 9 forms of Goddess Durga as
//! progressive security defense layers.
//!
//! ## The 9 Durgas:
//!
//! | Durga | Meaning | Security Layer |
//! |-------|---------|----------------|
//! | Shailaputri | Mountain's Daughter | Hardware Security |
//! | Brahmacharini | Seeker of Brahman | Authentication |
//! | Chandraghanta | Moon Bell | Encryption |
//! | Kushmanda | Creator of Universe | Access Control |
//! | Skandamata | Mother of Skanda | Process Isolation |
//! | Katyayani | Daughter of Katya | Input Validation |
//! | Kalaratri | Dark Night | Intrusion Detection |
//! | Mahagauri | Great White | Audit Logging |
//! | Siddhidatri | Giver of Perfection | Formal Verification |
//!
//! ## Defense in Depth
//!
//! Code must pass through ALL 9 layers to be secure.
//! Each layer progressively hardens the code against attacks.

use tracing::{debug, info, warn};

pub mod shailaputri;
pub mod brahmacharini;
pub mod chandraghanta;
pub mod kushmanda;
pub mod skandamata;
pub mod katyayani;
pub mod kalaratri;
pub mod mahagauri;
pub mod siddhidatri;

pub use shailaputri::Shailaputri;
pub use brahmacharini::Brahmacharini;
pub use chandraghanta::Chandraghanta;
pub use kushmanda::Kushmanda;
pub use skandamata::Skandamata;
pub use katyayani::Katyayani;
pub use kalaratri::Kalaratri;
pub use mahagauri::Mahagauri;
pub use siddhidatri::Siddhidatri;

/// Names of the 9 Durgas
pub const NAVA_DURGA: [&str; 9] = [
    "Śailaputrī",      // 1. Hardware security
    "Brahmacāriṇī",    // 2. Authentication
    "Candraghaṇṭā",    // 3. Encryption
    "Kūṣmāṇḍā",        // 4. Access control
    "Skandamātā",      // 5. Process isolation
    "Kātyāyanī",       // 6. Input validation
    "Kālarātrī",       // 7. Intrusion detection
    "Mahāgaurī",       // 8. Audit logging
    "Siddhidātrī",     // 9. Formal verification
];

/// Result of passing through a Durga layer
#[derive(Debug, Clone)]
pub enum DurgaDefense {
    /// Passed the security check
    Passed,
    /// Blocked by the security check
    Blocked { reason: String },
    /// Warning issued but allowed to pass
    Warning { message: String },
}

/// Core trait for all Durga security layers
pub trait DurgaLayer: Send + Sync {
    /// The name of this Durga
    fn name(&self) -> &'static str;

    /// The Sanskrit name
    fn sanskrit_name(&self) -> &'static str;

    /// The security function
    fn security_function(&self) -> &'static str;

    /// Layer number (1-9)
    fn layer(&self) -> u8;

    /// Defend against threats (analyze code)
    fn defend(&self, code: &SecurityContext) -> DurgaDefense;

    /// Whether this layer can be bypassed
    fn mandatory(&self) -> bool {
        true
    }
}

/// Context for security analysis
#[derive(Debug, Clone, Default)]
pub struct SecurityContext {
    /// Source code being analyzed
    pub source: String,
    /// Trust level (0.0 = untrusted, 1.0 = fully trusted)
    pub trust_level: f64,
    /// Security annotations found
    pub annotations: Vec<SecurityAnnotation>,
    /// Vulnerabilities detected
    pub vulnerabilities: Vec<Vulnerability>,
}

impl SecurityContext {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            trust_level: 0.0,
            annotations: Vec::new(),
            vulnerabilities: Vec::new(),
        }
    }

    pub fn with_trust(mut self, level: f64) -> Self {
        self.trust_level = level.clamp(0.0, 1.0);
        self
    }

    pub fn add_vulnerability(&mut self, vuln: Vulnerability) {
        self.vulnerabilities.push(vuln);
    }

    pub fn is_secure(&self) -> bool {
        self.vulnerabilities.is_empty()
    }
}

/// A security annotation in code
#[derive(Debug, Clone)]
pub struct SecurityAnnotation {
    pub kind: AnnotationKind,
    pub location: usize,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnnotationKind {
    Trusted,
    Untrusted,
    Validated,
    Encrypted,
    Audited,
}

/// A detected vulnerability
#[derive(Debug, Clone)]
pub struct Vulnerability {
    pub severity: VulnerabilitySeverity,
    pub kind: VulnerabilityKind,
    pub location: usize,
    pub description: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum VulnerabilitySeverity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VulnerabilityKind {
    BufferOverflow,
    UseAfterFree,
    NullPointer,
    Injection,
    Authentication,
    Authorization,
    Encryption,
    InformationLeak,
    DenialOfService,
    Other,
}

/// The complete Nava Durga defense system
pub struct NavaDurgaDefense {
    pub layers: [Box<dyn DurgaLayer>; 9],
}

impl NavaDurgaDefense {
    /// Create new defense system with all 9 layers
    pub fn new() -> Self {
        Self {
            layers: [
                Box::new(Shailaputri::new()),
                Box::new(Brahmacharini::new()),
                Box::new(Chandraghanta::new()),
                Box::new(Kushmanda::new()),
                Box::new(Skandamata::new()),
                Box::new(Katyayani::new()),
                Box::new(Kalaratri::new()),
                Box::new(Mahagauri::new()),
                Box::new(Siddhidatri::new()),
            ],
        }
    }

    /// Run code through all 9 defense layers
    pub fn protect(&self, context: &mut SecurityContext) -> SecurityResult {
        let mut results = Vec::new();

        for layer in &self.layers {
            let defense = layer.defend(context);

            match &defense {
                DurgaDefense::Passed => {
                    debug!("Layer {}: {} - PASSED", layer.layer(), layer.name());
                }
                DurgaDefense::Blocked { reason } => {
                    warn!("Layer {}: {} - BLOCKED: {}",
                             layer.layer(), layer.name(), reason);

                    if layer.mandatory() {
                        return SecurityResult::Blocked {
                            layer: layer.layer(),
                            goddess: layer.name(),
                            reason: reason.clone(),
                        };
                    }
                }
                DurgaDefense::Warning { message } => {
                    info!("Layer {}: {} - WARNING: {}",
                             layer.layer(), layer.name(), message);
                }
            }

            results.push((layer.layer(), defense));
        }

        // All 9 layers passed = Siddhidatri grants perfection
        SecurityResult::Perfect {
            layers_passed: 9,
            trust_level: context.trust_level,
        }
    }
}

impl Default for NavaDurgaDefense {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of security analysis
#[derive(Debug, Clone)]
pub enum SecurityResult {
    /// All layers passed - code is secure
    Perfect {
        layers_passed: u8,
        trust_level: f64,
    },
    /// Blocked by a layer
    Blocked {
        layer: u8,
        goddess: &'static str,
        reason: String,
    },
}
