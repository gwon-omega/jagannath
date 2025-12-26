//! # Śūdra Privilege - Sandboxed Execution
//!
//! Maximum isolation with minimal capabilities.
//!
//! ## Philosophy
//!
//! "शूद्रस्य तु सतां शुश्रूषा" (Manusmriti)
//! "Shudra serves the virtuous"
//!
//! Shudra code is completely sandboxed and can only perform
//! pre-approved operations. This is the most restricted level.

use super::{Capability, Varna, VarnaViolation};

/// Shudra privilege level - Sandboxed execution
pub struct ShudraPrivilege {
    /// Explicitly granted capabilities (very limited)
    capabilities: Vec<Capability>,
    /// Sandbox restrictions
    restrictions: SandboxRestrictions,
}

/// Restrictions applied to sandboxed code
#[derive(Debug, Clone)]
pub struct SandboxRestrictions {
    /// Maximum memory allocation allowed
    pub max_memory: usize,
    /// Maximum CPU time allowed (milliseconds)
    pub max_cpu_time: u64,
    /// Allowed file paths (whitelist)
    pub allowed_paths: Vec<String>,
    /// Allowed network hosts (whitelist)
    pub allowed_hosts: Vec<String>,
    /// Maximum number of file descriptors
    pub max_fds: usize,
    /// Allow network access
    pub allow_network: bool,
    /// Allow filesystem access
    pub allow_filesystem: bool,
}

impl Default for SandboxRestrictions {
    fn default() -> Self {
        Self {
            max_memory: 64 * 1024 * 1024, // 64 MB
            max_cpu_time: 10000,          // 10 seconds
            allowed_paths: Vec::new(),
            allowed_hosts: Vec::new(),
            max_fds: 16,
            allow_network: false,
            allow_filesystem: false,
        }
    }
}

impl SandboxRestrictions {
    /// Create minimal restrictions (very limited)
    pub fn minimal() -> Self {
        Self {
            max_memory: 16 * 1024 * 1024, // 16 MB
            max_cpu_time: 1000,           // 1 second
            allowed_paths: Vec::new(),
            allowed_hosts: Vec::new(),
            max_fds: 4,
            allow_network: false,
            allow_filesystem: false,
        }
    }

    /// Create standard sandbox restrictions
    pub fn standard() -> Self {
        Self::default()
    }

    /// Create permissive restrictions (for testing)
    pub fn permissive() -> Self {
        Self {
            max_memory: 512 * 1024 * 1024, // 512 MB
            max_cpu_time: 60000,           // 60 seconds
            allowed_paths: vec!["/tmp".to_string()],
            allowed_hosts: vec!["localhost".to_string()],
            max_fds: 64,
            allow_network: false,
            allow_filesystem: true,
        }
    }
}

impl Default for ShudraPrivilege {
    fn default() -> Self {
        Self::new()
    }
}

impl ShudraPrivilege {
    /// Create a new Shudra privilege context (most restricted)
    pub fn new() -> Self {
        Self {
            capabilities: vec![
                Capability::Random, // Only random is always allowed
            ],
            restrictions: SandboxRestrictions::default(),
        }
    }

    /// Create with specific restrictions
    pub fn with_restrictions(restrictions: SandboxRestrictions) -> Self {
        let mut capabilities = vec![Capability::Random];

        // Add capabilities based on restrictions
        if restrictions.allow_filesystem {
            capabilities.push(Capability::FileSystem);
        }
        if restrictions.allow_network {
            capabilities.push(Capability::Network);
        }

        Self {
            capabilities,
            restrictions,
        }
    }

    /// Get the Varna level
    pub fn varna(&self) -> Varna {
        Varna::Shudra
    }

    /// Get the restrictions
    pub fn restrictions(&self) -> &SandboxRestrictions {
        &self.restrictions
    }

    /// Check if a capability is available
    pub fn has_capability(&self, capability: Capability) -> bool {
        self.capabilities.contains(&capability)
    }

    /// Verify capability and restrictions
    fn verify_operation(&self, capability: Capability) -> Result<(), VarnaViolation> {
        if self.has_capability(capability) {
            Ok(())
        } else {
            Err(VarnaViolation {
                current_varna: Varna::Shudra,
                required_varna: capability.minimum_varna(),
                capability: Some(capability),
                message: format!(
                    "Shudra (sandboxed) does not have {:?} capability",
                    capability
                ),
                location: None,
                suggestion: "This operation is not allowed in sandboxed mode".to_string(),
            })
        }
    }

    /// Check if a path is allowed
    fn is_path_allowed(&self, path: &str) -> bool {
        self.restrictions
            .allowed_paths
            .iter()
            .any(|allowed| path.starts_with(allowed))
    }

    /// Check if a host is allowed
    fn is_host_allowed(&self, host: &str) -> bool {
        self.restrictions
            .allowed_hosts
            .iter()
            .any(|allowed| host == allowed)
    }

    /// Read a file (if allowed)
    pub fn read_file(&self, path: &str) -> Result<Vec<u8>, VarnaViolation> {
        self.verify_operation(Capability::FileSystem)?;

        if !self.is_path_allowed(path) {
            return Err(VarnaViolation {
                current_varna: Varna::Shudra,
                required_varna: Varna::Vaishya,
                capability: Some(Capability::FileSystem),
                message: format!("Path not in whitelist: {}", path),
                location: None,
                suggestion: format!(
                    "Add path to allowed_paths or use a path under: {:?}",
                    self.restrictions.allowed_paths
                ),
            });
        }

        // Debug: Shudra reading allowed file
        let _ = path; // Suppress unused warning
        Ok(Vec::new())
    }

    /// Connect to network (if allowed)
    pub fn connect(&self, host: &str) -> Result<u32, VarnaViolation> {
        self.verify_operation(Capability::Network)?;

        if !self.is_host_allowed(host) {
            return Err(VarnaViolation {
                current_varna: Varna::Shudra,
                required_varna: Varna::Vaishya,
                capability: Some(Capability::Network),
                message: format!("Host not in whitelist: {}", host),
                location: None,
                suggestion: format!(
                    "Add host to allowed_hosts or use one of: {:?}",
                    self.restrictions.allowed_hosts
                ),
            });
        }

        // Debug: Shudra connecting to allowed host
        let _ = host; // Suppress unused warning
        Ok(0)
    }

    /// Allocate memory (within limits)
    pub fn allocate(&self, size: usize) -> Result<*mut u8, VarnaViolation> {
        if size > self.restrictions.max_memory {
            return Err(VarnaViolation {
                current_varna: Varna::Shudra,
                required_varna: Varna::Vaishya,
                capability: None,
                message: format!(
                    "Memory request {} exceeds limit {}",
                    size, self.restrictions.max_memory
                ),
                location: None,
                suggestion: "Reduce memory usage or request higher privilege".to_string(),
            });
        }

        // Debug: Shudra allocating bytes
        let _ = size; // Suppress unused warning
        Ok(std::ptr::null_mut())
    }

    /// Get random bytes (always allowed)
    pub fn get_random(&self, buffer: &mut [u8]) -> Result<(), VarnaViolation> {
        self.verify_operation(Capability::Random)?;
        for byte in buffer.iter_mut() {
            *byte = 0; // Placeholder
        }
        Ok(())
    }
}

/// Sandboxed operations with strict limits
pub mod sandbox_ops {
    use super::SandboxRestrictions;

    /// Track resource usage
    #[derive(Debug, Default)]
    pub struct ResourceUsage {
        pub memory_used: usize,
        pub cpu_time_used: u64,
        pub fds_used: usize,
    }

    /// Check if operation is within limits
    pub fn check_limits(
        usage: &ResourceUsage,
        restrictions: &SandboxRestrictions,
    ) -> Result<(), &'static str> {
        if usage.memory_used > restrictions.max_memory {
            return Err("Memory limit exceeded");
        }
        if usage.cpu_time_used > restrictions.max_cpu_time {
            return Err("CPU time limit exceeded");
        }
        if usage.fds_used > restrictions.max_fds {
            return Err("File descriptor limit exceeded");
        }
        Ok(())
    }

    /// Pure computation (always allowed)
    pub fn pure_compute<T, F: FnOnce() -> T>(f: F) -> T {
        f()
    }

    /// Sandboxed print (to capture output)
    pub fn sandboxed_print(message: &str, output_buffer: &mut String) {
        output_buffer.push_str(message);
        output_buffer.push('\n');
    }
}
