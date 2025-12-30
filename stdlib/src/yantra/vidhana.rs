//! # Vidhana - Process Management (विधान)
//!
//! Process spawning, management, and exit codes.
//!
//! > **"विधानं कार्यस्य नियमः"**
//! > *"Process is the rule of work"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// PROCESS EXIT
// ============================================================================

/// Exit the process
#[cfg(feature = "std")]
pub fn nirgama(koda: i32) -> ! {
    std::process::exit(koda);
}

/// Exit with success (0)
#[cfg(feature = "std")]
pub fn safala_nirgama() -> ! {
    std::process::exit(0);
}

/// Exit with failure (1)
#[cfg(feature = "std")]
pub fn asafala_nirgama() -> ! {
    std::process::exit(1);
}

/// Abort the process immediately
#[cfg(feature = "std")]
pub fn samapana() -> ! {
    std::process::abort();
}

// ============================================================================
// PROCESS ID
// ============================================================================

/// Get current process ID
#[cfg(feature = "std")]
pub fn prakriya_id() -> u32 {
    std::process::id()
}

// ============================================================================
// COMMAND EXECUTION
// ============================================================================

/// Command result
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct AdeshaPhala {
    /// Exit code
    pub nirgama_koda: Option<i32>,
    /// Standard output
    pub nirgata: String,
    /// Standard error
    pub truti_nirgata: String,
    /// Success status
    pub safala: bool,
}

/// Run a command and capture output
#[cfg(all(feature = "std", feature = "alloc"))]
pub fn adesha_chalana(karyakrama: &str, tarkas: &[&str]) -> Result<AdeshaPhala, VidhanaDosha> {
    let output = std::process::Command::new(karyakrama)
        .args(tarkas)
        .output()
        .map_err(|e| VidhanaDosha::ChalaanaAsafala(e.to_string()))?;

    Ok(AdeshaPhala {
        nirgama_koda: output.status.code(),
        nirgata: String::from_utf8_lossy(&output.stdout).into_owned(),
        truti_nirgata: String::from_utf8_lossy(&output.stderr).into_owned(),
        safala: output.status.success(),
    })
}

/// Run a command with custom directory
#[cfg(all(feature = "std", feature = "alloc"))]
pub fn adesha_chalana_nirdeshika(
    karyakrama: &str,
    tarkas: &[&str],
    nirdeshika: &str,
) -> Result<AdeshaPhala, VidhanaDosha> {
    let output = std::process::Command::new(karyakrama)
        .args(tarkas)
        .current_dir(nirdeshika)
        .output()
        .map_err(|e| VidhanaDosha::ChalaanaAsafala(e.to_string()))?;

    Ok(AdeshaPhala {
        nirgama_koda: output.status.code(),
        nirgata: String::from_utf8_lossy(&output.stdout).into_owned(),
        truti_nirgata: String::from_utf8_lossy(&output.stderr).into_owned(),
        safala: output.status.success(),
    })
}

/// Run a command with environment variables
#[cfg(all(feature = "std", feature = "alloc"))]
pub fn adesha_chalana_parivesh(
    karyakrama: &str,
    tarkas: &[&str],
    parivesh: &[(&str, &str)],
) -> Result<AdeshaPhala, VidhanaDosha> {
    let mut cmd = std::process::Command::new(karyakrama);
    cmd.args(tarkas);
    for (k, v) in parivesh {
        cmd.env(k, v);
    }

    let output = cmd
        .output()
        .map_err(|e| VidhanaDosha::ChalaanaAsafala(e.to_string()))?;

    Ok(AdeshaPhala {
        nirgama_koda: output.status.code(),
        nirgata: String::from_utf8_lossy(&output.stdout).into_owned(),
        truti_nirgata: String::from_utf8_lossy(&output.stderr).into_owned(),
        safala: output.status.success(),
    })
}

// ============================================================================
// COMMAND BUILDER
// ============================================================================

/// Command builder
#[cfg(all(feature = "std", feature = "alloc"))]
pub struct AdeshaNirmata {
    karyakrama: String,
    tarkas: Vec<String>,
    nirdeshika: Option<String>,
    parivesh: Vec<(String, String)>,
}

#[cfg(all(feature = "std", feature = "alloc"))]
impl AdeshaNirmata {
    /// Create new command
    pub fn nava(karyakrama: impl Into<String>) -> Self {
        Self {
            karyakrama: karyakrama.into(),
            tarkas: Vec::new(),
            nirdeshika: None,
            parivesh: Vec::new(),
        }
    }

    /// Add argument
    pub fn tarka(mut self, arg: impl Into<String>) -> Self {
        self.tarkas.push(arg.into());
        self
    }

    /// Add multiple arguments
    pub fn tarkas(mut self, args: &[&str]) -> Self {
        for arg in args {
            self.tarkas.push((*arg).to_string());
        }
        self
    }

    /// Set working directory
    pub fn nirdeshika(mut self, dir: impl Into<String>) -> Self {
        self.nirdeshika = Some(dir.into());
        self
    }

    /// Add environment variable
    pub fn parivesh(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.parivesh.push((key.into(), value.into()));
        self
    }

    /// Run and capture output
    pub fn chalana(self) -> Result<AdeshaPhala, VidhanaDosha> {
        let mut cmd = std::process::Command::new(&self.karyakrama);
        cmd.args(&self.tarkas);

        if let Some(ref dir) = self.nirdeshika {
            cmd.current_dir(dir);
        }

        for (k, v) in &self.parivesh {
            cmd.env(k, v);
        }

        let output = cmd
            .output()
            .map_err(|e| VidhanaDosha::ChalaanaAsafala(e.to_string()))?;

        Ok(AdeshaPhala {
            nirgama_koda: output.status.code(),
            nirgata: String::from_utf8_lossy(&output.stdout).into_owned(),
            truti_nirgata: String::from_utf8_lossy(&output.stderr).into_owned(),
            safala: output.status.success(),
        })
    }

    /// Run without capturing (inherit stdio)
    pub fn chalana_antar(self) -> Result<i32, VidhanaDosha> {
        let mut cmd = std::process::Command::new(&self.karyakrama);
        cmd.args(&self.tarkas);

        if let Some(ref dir) = self.nirdeshika {
            cmd.current_dir(dir);
        }

        for (k, v) in &self.parivesh {
            cmd.env(k, v);
        }

        let status = cmd
            .status()
            .map_err(|e| VidhanaDosha::ChalaanaAsafala(e.to_string()))?;

        Ok(status.code().unwrap_or(-1))
    }
}

// ============================================================================
// SHELL EXECUTION
// ============================================================================

/// Run shell command (uses sh on Unix, cmd on Windows)
#[cfg(all(feature = "std", feature = "alloc", not(target_os = "windows")))]
pub fn kalava_chalana(adesha: &str) -> Result<AdeshaPhala, VidhanaDosha> {
    adesha_chalana("sh", &["-c", adesha])
}

/// Run shell command (uses sh on Unix, cmd on Windows)
#[cfg(all(feature = "std", feature = "alloc", target_os = "windows"))]
pub fn kalava_chalana(adesha: &str) -> Result<AdeshaPhala, VidhanaDosha> {
    adesha_chalana("cmd", &["/C", adesha])
}

// ============================================================================
// ERROR TYPE
// ============================================================================

/// Process error
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub enum VidhanaDosha {
    /// Command execution failed
    ChalaanaAsafala(String),
    /// Process not found
    PrakriyaNahiMilya(String),
    /// Permission denied
    AnumatiNiṣedha(String),
    /// Timeout
    SamayaSeema(String),
}

#[cfg(feature = "alloc")]
impl core::fmt::Display for VidhanaDosha {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            VidhanaDosha::ChalaanaAsafala(s) => write!(f, "Command failed: {}", s),
            VidhanaDosha::PrakriyaNahiMilya(s) => write!(f, "Process not found: {}", s),
            VidhanaDosha::AnumatiNiṣedha(s) => write!(f, "Permission denied: {}", s),
            VidhanaDosha::SamayaSeema(s) => write!(f, "Timeout: {}", s),
        }
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "std")]
    fn test_process_id() {
        let pid = prakriya_id();
        assert!(pid > 0);
    }

    #[test]
    #[cfg(all(feature = "std", feature = "alloc", not(target_os = "windows")))]
    fn test_echo() {
        let result = adesha_chalana("echo", &["hello"]).unwrap();
        assert!(result.safala);
        assert!(result.nirgata.trim() == "hello");
    }

    #[test]
    #[cfg(all(feature = "std", feature = "alloc", target_os = "windows"))]
    fn test_echo_windows() {
        let result = kalava_chalana("echo hello").unwrap();
        assert!(result.safala);
        assert!(result.nirgata.contains("hello"));
    }

    #[test]
    #[cfg(all(feature = "std", feature = "alloc"))]
    fn test_command_builder() {
        #[cfg(not(target_os = "windows"))]
        let result = AdeshaNirmata::nava("echo").tarka("test").chalana().unwrap();

        #[cfg(target_os = "windows")]
        let result = AdeshaNirmata::nava("cmd")
            .tarkas(&["/C", "echo", "test"])
            .chalana()
            .unwrap();

        assert!(result.safala);
    }
}
