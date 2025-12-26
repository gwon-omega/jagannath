//! # Varṇa System - Privilege Rings Architecture
//!
//! Implements CPU privilege ring model using Hindu Varna system.
//!
//! ## Philosophy
//!
//! The four Varnas map to CPU protection rings:
//! - **Brāhmaṇa** (Brahmin) → Ring 0 - Kernel mode, full hardware access
//! - **Kṣatriya** → Ring 1-2 - System services, drivers
//! - **Vaiśya** → Ring 3 - User mode, normal applications
//! - **Śūdra** → Ring 4+ - Sandboxed, maximum isolation
//!
//! Each Varna has specific capabilities and restrictions.

pub mod brahmin;
pub mod kshatriya;
pub mod shudra;
pub mod vaishya;
pub mod varna_checker;

pub use brahmin::BrahminPrivilege;
pub use kshatriya::KshatriyaPrivilege;
pub use shudra::ShudraPrivilege;
pub use vaishya::VaishyaPrivilege;
pub use varna_checker::VarnaChecker;

/// The four Varnas (privilege levels)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Varna {
    /// Brahmin - Ring 0, kernel mode, full hardware access
    /// Highest privilege, can access everything
    Brahmin,

    /// Kshatriya - Ring 1-2, system services, drivers
    /// Can perform system-level operations, manage resources
    Kshatriya,

    /// Vaishya - Ring 3, user mode, normal applications
    /// Standard application privilege level
    Vaishya,

    /// Shudra - Ring 4+, sandboxed, maximum isolation
    /// Most restricted, can only access approved resources
    Shudra,
}

impl Varna {
    /// Get the CPU ring number for this Varna
    pub fn ring(&self) -> u8 {
        match self {
            Varna::Brahmin => 0,
            Varna::Kshatriya => 1, // Can also be 2
            Varna::Vaishya => 3,
            Varna::Shudra => 4, // Pseudo-ring (sandboxed)
        }
    }

    /// Get the Sanskrit name
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Varna::Brahmin => "ब्राह्मण",
            Varna::Kshatriya => "क्षत्रिय",
            Varna::Vaishya => "वैश्य",
            Varna::Shudra => "शूद्र",
        }
    }

    /// Get the traditional role description
    pub fn traditional_role(&self) -> &'static str {
        match self {
            Varna::Brahmin => "Priest/Teacher - Knowledge keeper",
            Varna::Kshatriya => "Warrior/Ruler - Protector",
            Varna::Vaishya => "Merchant/Farmer - Commerce",
            Varna::Shudra => "Laborer/Servant - Service",
        }
    }

    /// Get the system role description
    pub fn system_role(&self) -> &'static str {
        match self {
            Varna::Brahmin => "Kernel mode - Full hardware access",
            Varna::Kshatriya => "System services - Drivers & daemons",
            Varna::Vaishya => "User mode - Normal applications",
            Varna::Shudra => "Sandboxed - Maximum isolation",
        }
    }

    /// Check if this Varna can access another Varna's resources
    pub fn can_access(&self, target: Varna) -> bool {
        // Higher privilege (lower number) can access lower privilege
        self.ring() <= target.ring()
    }

    /// Check if this Varna can call into another Varna
    pub fn can_call(&self, target: Varna) -> bool {
        match (self, target) {
            // Brahmin can call anyone
            (Varna::Brahmin, _) => true,

            // Kshatriya can call same or lower
            (Varna::Kshatriya, Varna::Brahmin) => false, // Need syscall
            (Varna::Kshatriya, _) => true,

            // Vaishya can call same or lower, or syscall to higher
            (Varna::Vaishya, Varna::Brahmin) => false, // Need syscall
            (Varna::Vaishya, Varna::Kshatriya) => false, // Need syscall
            (Varna::Vaishya, _) => true,

            // Shudra can only call same level
            (Varna::Shudra, Varna::Shudra) => true,
            (Varna::Shudra, _) => false, // Need syscall
        }
    }

    /// Get required syscall for privilege elevation
    pub fn elevation_syscall(&self, target: Varna) -> Option<&'static str> {
        if self.can_call(target) {
            None // No syscall needed
        } else {
            Some(match target {
                Varna::Brahmin => "syscall_brahmin",     // Kernel call
                Varna::Kshatriya => "syscall_kshatriya", // System call
                Varna::Vaishya => "syscall_vaishya",     // User call
                Varna::Shudra => "syscall_shudra",       // Sandbox call
            })
        }
    }
}

/// Capabilities that can be granted to code
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    /// Access to raw hardware (ports, memory-mapped IO)
    HardwareAccess,
    /// Ability to execute privileged instructions
    PrivilegedInstructions,
    /// Interrupt handling
    InterruptHandling,
    /// Memory management (page tables, etc.)
    MemoryManagement,
    /// Process/thread management
    ProcessManagement,
    /// File system access
    FileSystem,
    /// Network access
    Network,
    /// IPC (inter-process communication)
    IPC,
    /// Timer/clock access
    Timer,
    /// Random number generation
    Random,
    /// Cryptographic operations
    Crypto,
    /// GUI/Display access
    Display,
    /// Audio access
    Audio,
    /// Input devices (keyboard, mouse)
    Input,
    /// USB/external devices
    ExternalDevices,
}

impl Capability {
    /// Get the minimum Varna required for this capability
    pub fn minimum_varna(&self) -> Varna {
        match self {
            // Brahmin-only capabilities
            Capability::HardwareAccess
            | Capability::PrivilegedInstructions
            | Capability::InterruptHandling
            | Capability::MemoryManagement => Varna::Brahmin,

            // Kshatriya capabilities
            Capability::ProcessManagement | Capability::Timer => Varna::Kshatriya,

            // Vaishya capabilities
            Capability::FileSystem
            | Capability::Network
            | Capability::IPC
            | Capability::Random
            | Capability::Crypto
            | Capability::Display
            | Capability::Audio
            | Capability::Input
            | Capability::ExternalDevices => Varna::Vaishya,
        }
    }
}

/// Violation of privilege boundaries
#[derive(Debug, Clone)]
pub struct VarnaViolation {
    /// Current Varna level of the code
    pub current_varna: Varna,
    /// Required Varna for the operation
    pub required_varna: Varna,
    /// The capability that was requested
    pub capability: Option<Capability>,
    /// Human-readable message
    pub message: String,
    /// Location in source code
    pub location: Option<String>,
    /// Suggested fix
    pub suggestion: String,
}

impl std::fmt::Display for VarnaViolation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "VarnaViolation: {} ({}) cannot perform {} operations. {}",
            self.current_varna.sanskrit_name(),
            self.current_varna.system_role(),
            self.required_varna.sanskrit_name(),
            self.suggestion
        )
    }
}

impl std::error::Error for VarnaViolation {}

/// Transition between Varna levels
#[derive(Debug, Clone)]
pub struct VarnaTransition {
    /// Source Varna
    pub from: Varna,
    /// Target Varna
    pub to: Varna,
    /// Whether this requires a syscall
    pub requires_syscall: bool,
    /// The syscall name if required
    pub syscall: Option<&'static str>,
}

impl VarnaTransition {
    /// Create a new transition
    pub fn new(from: Varna, to: Varna) -> Self {
        let requires_syscall = !from.can_call(to);
        let syscall = from.elevation_syscall(to);

        Self {
            from,
            to,
            requires_syscall,
            syscall,
        }
    }

    /// Check if this transition is allowed
    pub fn is_allowed(&self) -> bool {
        // Upward transitions (to higher privilege) require syscall
        // Downward transitions are always allowed
        self.from.ring() >= self.to.ring() || self.requires_syscall
    }
}

/// Context for privilege checking
#[derive(Debug, Clone)]
pub struct VarnaContext {
    /// Current privilege level
    pub current: Varna,
    /// Granted capabilities
    pub capabilities: Vec<Capability>,
    /// Whether we're in a syscall
    pub in_syscall: bool,
    /// Audit trail of transitions
    pub transitions: Vec<VarnaTransition>,
}

impl Default for VarnaContext {
    fn default() -> Self {
        Self {
            current: Varna::Vaishya, // Default to user mode
            capabilities: Vec::new(),
            in_syscall: false,
            transitions: Vec::new(),
        }
    }
}

impl VarnaContext {
    /// Create a new context with a specific Varna
    pub fn new(varna: Varna) -> Self {
        Self {
            current: varna,
            capabilities: Vec::new(),
            in_syscall: false,
            transitions: Vec::new(),
        }
    }

    /// Create a kernel mode context
    pub fn kernel() -> Self {
        Self::new(Varna::Brahmin)
    }

    /// Create a user mode context
    pub fn user() -> Self {
        Self::new(Varna::Vaishya)
    }

    /// Create a sandboxed context
    pub fn sandboxed() -> Self {
        Self::new(Varna::Shudra)
    }

    /// Grant a capability
    pub fn grant(&mut self, capability: Capability) -> Result<(), VarnaViolation> {
        let required = capability.minimum_varna();
        if self.current.can_access(required) {
            self.capabilities.push(capability);
            Ok(())
        } else {
            Err(VarnaViolation {
                current_varna: self.current,
                required_varna: required,
                capability: Some(capability),
                message: format!("Cannot grant {:?} capability", capability),
                location: None,
                suggestion: format!(
                    "Elevate to {} or request capability through syscall",
                    required.sanskrit_name()
                ),
            })
        }
    }

    /// Check if a capability is available
    pub fn has_capability(&self, capability: Capability) -> bool {
        self.capabilities.contains(&capability)
            || self.current.can_access(capability.minimum_varna())
    }

    /// Transition to a new Varna
    pub fn transition_to(&mut self, target: Varna) -> Result<(), VarnaViolation> {
        let transition = VarnaTransition::new(self.current, target);

        if transition.is_allowed() {
            self.transitions.push(transition.clone());
            self.current = target;
            self.in_syscall = transition.requires_syscall;
            Ok(())
        } else {
            Err(VarnaViolation {
                current_varna: self.current,
                required_varna: target,
                capability: None,
                message: format!("Cannot transition from {:?} to {:?}", self.current, target),
                location: None,
                suggestion: "Use appropriate syscall for privilege elevation".to_string(),
            })
        }
    }
}
