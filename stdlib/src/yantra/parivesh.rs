//! # Parivesh - Environment Variables (परिवेश)
//!
//! Access and manipulate environment variables.
//!
//! > **"परिवेशः परिस्थितिः"**
//! > *"Environment is circumstance"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::collections::BTreeMap;
#[cfg(feature = "alloc")]
use alloc::string::String;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// ENVIRONMENT ACCESS
// ============================================================================

/// Get environment variable
#[cfg(feature = "std")]
pub fn prapta(nama: &str) -> Option<String> {
    std::env::var(nama).ok()
}

/// Get environment variable with default
#[cfg(feature = "std")]
pub fn prapta_ya_nyunatam(nama: &str, nyunatam: &str) -> String {
    std::env::var(nama).unwrap_or_else(|_| nyunatam.to_string())
}

/// Set environment variable
#[cfg(feature = "std")]
pub fn sthapita(nama: &str, mana: &str) {
    std::env::set_var(nama, mana);
}

/// Remove environment variable
#[cfg(feature = "std")]
pub fn hatana(nama: &str) {
    std::env::remove_var(nama);
}

/// Check if environment variable exists
#[cfg(feature = "std")]
pub fn vidyamana(nama: &str) -> bool {
    std::env::var(nama).is_ok()
}

/// Get all environment variables
#[cfg(feature = "std")]
pub fn sabhi() -> BTreeMap<String, String> {
    std::env::vars().collect()
}

// ============================================================================
// PATH ENVIRONMENT
// ============================================================================

/// Get PATH entries as vector
#[cfg(feature = "std")]
pub fn patha_suci() -> Vec<String> {
    #[cfg(target_os = "windows")]
    let separator = ';';
    #[cfg(not(target_os = "windows"))]
    let separator = ':';

    prapta("PATH")
        .map(|p| p.split(separator).map(String::from).collect())
        .unwrap_or_default()
}

/// Add directory to PATH
#[cfg(feature = "std")]
pub fn patha_jodna(dir: &str) {
    #[cfg(target_os = "windows")]
    let separator = ";";
    #[cfg(not(target_os = "windows"))]
    let separator = ":";

    let current = prapta("PATH").unwrap_or_default();
    let new_path = if current.is_empty() {
        dir.to_string()
    } else {
        format!("{}{}{}", dir, separator, current)
    };
    sthapita("PATH", &new_path);
}

// ============================================================================
// CURRENT DIRECTORY
// ============================================================================

/// Get current directory
#[cfg(feature = "std")]
pub fn vartamana_nirdeshika() -> Option<String> {
    std::env::current_dir()
        .ok()
        .map(|p| p.to_string_lossy().into_owned())
}

/// Set current directory
#[cfg(feature = "std")]
pub fn nirdeshika_sthapita(path: &str) -> Result<(), PariveshDosha> {
    std::env::set_current_dir(path).map_err(|e| PariveshDosha::NirdeshikaParivartana(e.to_string()))
}

// ============================================================================
// HOME & TEMP
// ============================================================================

/// Get home directory
#[cfg(feature = "std")]
pub fn griha_nirdeshika() -> Option<String> {
    #[cfg(target_os = "windows")]
    {
        prapta("USERPROFILE").or_else(|| {
            prapta("HOMEDRIVE")
                .zip(prapta("HOMEPATH"))
                .map(|(d, p)| d + &p)
        })
    }
    #[cfg(not(target_os = "windows"))]
    {
        prapta("HOME")
    }
}

/// Get temp directory
#[cfg(feature = "std")]
pub fn aasthayee_nirdeshika() -> String {
    std::env::temp_dir().to_string_lossy().into_owned()
}

/// Get executable path
#[cfg(feature = "std")]
pub fn nishpadan_patha() -> Option<String> {
    std::env::current_exe()
        .ok()
        .map(|p| p.to_string_lossy().into_owned())
}

// ============================================================================
// COMMAND LINE ARGUMENTS
// ============================================================================

/// Get command line arguments
#[cfg(feature = "std")]
pub fn tarka_suci() -> Vec<String> {
    std::env::args().collect()
}

/// Get arguments (skip program name)
#[cfg(feature = "std")]
pub fn tarka_kevala() -> Vec<String> {
    std::env::args().skip(1).collect()
}

/// Check if argument present
#[cfg(feature = "std")]
pub fn tarka_vidyamana(tarka: &str) -> bool {
    std::env::args().any(|a| a == tarka)
}

/// Get argument by index
#[cfg(feature = "std")]
pub fn tarka_prapta(suci: usize) -> Option<String> {
    std::env::args().nth(suci)
}

// ============================================================================
// ERROR TYPE
// ============================================================================

/// Environment error
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub enum PariveshDosha {
    /// Variable not found
    NahiMilya(String),
    /// Directory change failed
    NirdeshikaParivartana(String),
    /// Invalid value
    AmanyadMana(String),
}

#[cfg(feature = "alloc")]
impl core::fmt::Display for PariveshDosha {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            PariveshDosha::NahiMilya(s) => write!(f, "Variable not found: {}", s),
            PariveshDosha::NirdeshikaParivartana(s) => write!(f, "Directory change failed: {}", s),
            PariveshDosha::AmanyadMana(s) => write!(f, "Invalid value: {}", s),
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
    fn test_set_get() {
        let key = "_JAGANNATH_TEST_VAR";
        sthapita(key, "test_value");
        assert_eq!(prapta(key), Some("test_value".to_string()));
        hatana(key);
        assert_eq!(prapta(key), None);
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_default() {
        let result = prapta_ya_nyunatam("_NONEXISTENT_VAR_12345", "default");
        assert_eq!(result, "default");
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_path_suci() {
        let paths = patha_suci();
        assert!(!paths.is_empty());
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_vartamana() {
        let current = vartamana_nirdeshika();
        assert!(current.is_some());
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_tarka() {
        let args = tarka_suci();
        assert!(!args.is_empty()); // At least program name
    }
}
