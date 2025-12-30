//! # Tantra - System Information (तन्त्र)
//!
//! System information, platform detection, and hardware info.
//!
//! > **"तन्त्रं यन्त्रस्य विधानम्"**
//! > *"System is the organization of the machine"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;

// ============================================================================
// PLATFORM DETECTION
// ============================================================================

/// Operating system family
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SanchalanaVyavastha {
    /// Windows
    Windows,
    /// Linux
    Linux,
    /// macOS
    MacOS,
    /// FreeBSD
    FreeBSD,
    /// Android
    Android,
    /// iOS
    IOS,
    /// WebAssembly
    Wasm,
    /// Unknown
    Anyana,
}

impl SanchalanaVyavastha {
    /// Get current OS
    pub fn vartamana() -> Self {
        #[cfg(target_os = "windows")]
        return SanchalanaVyavastha::Windows;

        #[cfg(target_os = "linux")]
        return SanchalanaVyavastha::Linux;

        #[cfg(target_os = "macos")]
        return SanchalanaVyavastha::MacOS;

        #[cfg(target_os = "freebsd")]
        return SanchalanaVyavastha::FreeBSD;

        #[cfg(target_os = "android")]
        return SanchalanaVyavastha::Android;

        #[cfg(target_os = "ios")]
        return SanchalanaVyavastha::IOS;

        #[cfg(target_arch = "wasm32")]
        return SanchalanaVyavastha::Wasm;

        #[cfg(not(any(
            target_os = "windows",
            target_os = "linux",
            target_os = "macos",
            target_os = "freebsd",
            target_os = "android",
            target_os = "ios",
            target_arch = "wasm32"
        )))]
        return SanchalanaVyavastha::Anyana;
    }

    /// Get OS name
    pub fn nama(&self) -> &'static str {
        match self {
            SanchalanaVyavastha::Windows => "Windows",
            SanchalanaVyavastha::Linux => "Linux",
            SanchalanaVyavastha::MacOS => "macOS",
            SanchalanaVyavastha::FreeBSD => "FreeBSD",
            SanchalanaVyavastha::Android => "Android",
            SanchalanaVyavastha::IOS => "iOS",
            SanchalanaVyavastha::Wasm => "WebAssembly",
            SanchalanaVyavastha::Anyana => "Unknown",
        }
    }

    /// Is Unix-like?
    pub fn unix_sadrisham(&self) -> bool {
        matches!(
            self,
            SanchalanaVyavastha::Linux
                | SanchalanaVyavastha::MacOS
                | SanchalanaVyavastha::FreeBSD
                | SanchalanaVyavastha::Android
                | SanchalanaVyavastha::IOS
        )
    }

    /// Is mobile?
    pub fn chalit(&self) -> bool {
        matches!(
            self,
            SanchalanaVyavastha::Android | SanchalanaVyavastha::IOS
        )
    }
}

/// CPU architecture
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vaastukalapan {
    /// x86_64
    X86_64,
    /// x86 (32-bit)
    X86,
    /// ARM64 / AArch64
    Aarch64,
    /// ARM (32-bit)
    Arm,
    /// RISC-V 64-bit
    Riscv64,
    /// RISC-V 32-bit
    Riscv32,
    /// WebAssembly 32-bit
    Wasm32,
    /// WebAssembly 64-bit
    Wasm64,
    /// Unknown
    Anyana,
}

impl Vaastukalapan {
    /// Get current architecture
    pub fn vartamana() -> Self {
        #[cfg(target_arch = "x86_64")]
        return Vaastukalapan::X86_64;

        #[cfg(target_arch = "x86")]
        return Vaastukalapan::X86;

        #[cfg(target_arch = "aarch64")]
        return Vaastukalapan::Aarch64;

        #[cfg(target_arch = "arm")]
        return Vaastukalapan::Arm;

        #[cfg(target_arch = "riscv64")]
        return Vaastukalapan::Riscv64;

        #[cfg(target_arch = "riscv32")]
        return Vaastukalapan::Riscv32;

        #[cfg(target_arch = "wasm32")]
        return Vaastukalapan::Wasm32;

        #[cfg(target_arch = "wasm64")]
        return Vaastukalapan::Wasm64;

        #[cfg(not(any(
            target_arch = "x86_64",
            target_arch = "x86",
            target_arch = "aarch64",
            target_arch = "arm",
            target_arch = "riscv64",
            target_arch = "riscv32",
            target_arch = "wasm32",
            target_arch = "wasm64"
        )))]
        return Vaastukalapan::Anyana;
    }

    /// Get architecture name
    pub fn nama(&self) -> &'static str {
        match self {
            Vaastukalapan::X86_64 => "x86_64",
            Vaastukalapan::X86 => "x86",
            Vaastukalapan::Aarch64 => "aarch64",
            Vaastukalapan::Arm => "arm",
            Vaastukalapan::Riscv64 => "riscv64",
            Vaastukalapan::Riscv32 => "riscv32",
            Vaastukalapan::Wasm32 => "wasm32",
            Vaastukalapan::Wasm64 => "wasm64",
            Vaastukalapan::Anyana => "unknown",
        }
    }

    /// Pointer width in bits
    pub fn suchaka_bits(&self) -> usize {
        match self {
            Vaastukalapan::X86_64
            | Vaastukalapan::Aarch64
            | Vaastukalapan::Riscv64
            | Vaastukalapan::Wasm64 => 64,
            _ => 32,
        }
    }

    /// Is 64-bit?
    pub fn chaunsath_bit(&self) -> bool {
        self.suchaka_bits() == 64
    }
}

// ============================================================================
// COMPILE-TIME CONSTANTS
// ============================================================================

/// Pointer size in bytes (compile-time)
pub const SUCHAKA_AAKARA: usize = core::mem::size_of::<usize>();

/// Is debug build?
pub const DEBUG_NIRMANA: bool = cfg!(debug_assertions);

/// Is release build?
pub const VIMOCHANA_NIRMANA: bool = !cfg!(debug_assertions);

/// Target OS name
pub const LAKSHYA_SANCHALANA: &str = {
    #[cfg(target_os = "windows")]
    {
        "windows"
    }
    #[cfg(target_os = "linux")]
    {
        "linux"
    }
    #[cfg(target_os = "macos")]
    {
        "macos"
    }
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        "unknown"
    }
};

/// Target architecture name
pub const LAKSHYA_VAASTUKALA: &str = {
    #[cfg(target_arch = "x86_64")]
    {
        "x86_64"
    }
    #[cfg(target_arch = "aarch64")]
    {
        "aarch64"
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        "unknown"
    }
};

// ============================================================================
// ENDIANNESS
// ============================================================================

/// Byte order
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaitKrama {
    /// Little endian
    Laghu,
    /// Big endian
    Brihat,
}

impl BaitKrama {
    /// Get native byte order
    pub fn swabhavik() -> Self {
        #[cfg(target_endian = "little")]
        return BaitKrama::Laghu;

        #[cfg(target_endian = "big")]
        return BaitKrama::Brihat;
    }

    /// Is little endian?
    pub fn laghu_hai(&self) -> bool {
        *self == BaitKrama::Laghu
    }
}

// ============================================================================
// CPU FEATURES
// ============================================================================

/// Check if SSE2 is available (x86/x86_64)
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn sse2_upalabdha() -> bool {
    #[cfg(target_feature = "sse2")]
    return true;
    #[cfg(not(target_feature = "sse2"))]
    return false;
}

/// Check if AVX2 is available (x86/x86_64)
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn avx2_upalabdha() -> bool {
    #[cfg(target_feature = "avx2")]
    return true;
    #[cfg(not(target_feature = "avx2"))]
    return false;
}

/// Check if NEON is available (ARM)
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub fn neon_upalabdha() -> bool {
    #[cfg(target_feature = "neon")]
    return true;
    #[cfg(not(target_feature = "neon"))]
    return false;
}

// ============================================================================
// SYSTEM INFO (requires std)
// ============================================================================

/// System information
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct TantraSuchana {
    /// OS family
    pub sanchalana: SanchalanaVyavastha,
    /// CPU architecture
    pub vaastukala: Vaastukalapan,
    /// Byte order
    pub bait_krama: BaitKrama,
    /// Pointer size (bytes)
    pub suchaka_aakara: usize,
    /// Is debug build
    pub debug: bool,
}

#[cfg(feature = "alloc")]
impl TantraSuchana {
    /// Get system info
    pub fn prapta() -> Self {
        Self {
            sanchalana: SanchalanaVyavastha::vartamana(),
            vaastukala: Vaastukalapan::vartamana(),
            bait_krama: BaitKrama::swabhavik(),
            suchaka_aakara: SUCHAKA_AAKARA,
            debug: DEBUG_NIRMANA,
        }
    }
}

/// Get number of CPU cores (logical)
#[cfg(feature = "std")]
pub fn cpu_kendra_sankhya() -> Option<usize> {
    std::thread::available_parallelism().map(|n| n.get()).ok()
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_os_detection() {
        let os = SanchalanaVyavastha::vartamana();
        // Should be one of the known variants
        assert!(!os.nama().is_empty());
    }

    #[test]
    fn test_arch_detection() {
        let arch = Vaastukalapan::vartamana();
        assert!(!arch.nama().is_empty());
    }

    #[test]
    fn test_endianness() {
        let endian = BaitKrama::swabhavik();
        // Most modern systems are little endian
        #[cfg(target_endian = "little")]
        assert!(endian.laghu_hai());
    }

    #[test]
    fn test_pointer_size() {
        let arch = Vaastukalapan::vartamana();
        if arch.chaunsath_bit() {
            assert_eq!(SUCHAKA_AAKARA, 8);
        } else {
            assert_eq!(SUCHAKA_AAKARA, 4);
        }
    }

    #[test]
    #[cfg(feature = "std")]
    fn test_cpu_cores() {
        let cores = cpu_kendra_sankhya();
        assert!(cores.is_some());
        assert!(cores.unwrap() >= 1);
    }
}
