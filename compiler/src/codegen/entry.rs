//! Runtime Entry Point Generator
//!
//! Generates platform-specific entry points that initialize the runtime
//! and call the user's `mukhya` (main) function.
//!
//! ## Platform Support
//! - Linux: Uses `_start` and raw syscalls
//! - Windows: Uses `mainCRTStartup` or `main` with CRT
//! - macOS: Uses `_main` with libc

/// Runtime entry point configuration
#[derive(Debug, Clone)]
pub struct RuntimeEntry {
    /// Target platform
    pub platform: Platform,
    /// Whether to link with C runtime
    pub use_crt: bool,
    /// Main function name in user code
    pub main_fn: String,
}

/// Target platform
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Platform {
    LinuxX86_64,
    WindowsX86_64,
    MacOSX86_64,
    LinuxAArch64,
    MacOSAArch64,
}

impl RuntimeEntry {
    /// Create entry point for current platform
    pub fn for_current_platform() -> Self {
        #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
        let platform = Platform::LinuxX86_64;

        #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
        let platform = Platform::WindowsX86_64;

        #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
        let platform = Platform::MacOSX86_64;

        #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
        let platform = Platform::LinuxAArch64;

        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        let platform = Platform::MacOSAArch64;

        #[cfg(not(any(
            all(target_os = "linux", target_arch = "x86_64"),
            all(target_os = "windows", target_arch = "x86_64"),
            all(target_os = "macos", target_arch = "x86_64"),
            all(target_os = "linux", target_arch = "aarch64"),
            all(target_os = "macos", target_arch = "aarch64"),
        )))]
        let platform = Platform::LinuxX86_64; // Default fallback

        Self {
            platform,
            use_crt: true,
            main_fn: "mukhya".to_string(),
        }
    }

    /// Create Linux bare-metal entry (no CRT)
    pub fn linux_bare() -> Self {
        Self {
            platform: Platform::LinuxX86_64,
            use_crt: false,
            main_fn: "mukhya".to_string(),
        }
    }

    /// Create Windows CRT entry
    pub fn windows_crt() -> Self {
        Self {
            platform: Platform::WindowsX86_64,
            use_crt: true,
            main_fn: "mukhya".to_string(),
        }
    }

    /// Generate the entry point assembly
    pub fn generate(&self) -> String {
        match (self.platform, self.use_crt) {
            (Platform::LinuxX86_64, false) => self.linux_x86_64_bare(),
            (Platform::LinuxX86_64, true) => self.linux_x86_64_crt(),
            (Platform::WindowsX86_64, false) => self.windows_x86_64_bare(),
            (Platform::WindowsX86_64, true) => self.windows_x86_64_crt(),
            (Platform::MacOSX86_64, _) => self.macos_x86_64(),
            (Platform::LinuxAArch64, _) => self.linux_aarch64(),
            (Platform::MacOSAArch64, _) => self.macos_aarch64(),
        }
    }

    /// Linux x86-64 bare entry (no libc)
    fn linux_x86_64_bare(&self) -> String {
        format!(
            r#"; Jagannath Runtime Entry Point (Linux x86-64 bare)
; जगन्नाथ रनटाइम प्रवेश बिंदु
.intel_syntax noprefix

.section .text
.global _start
.type _start, @function

_start:
    ; Clear frame pointer for debugger
    xor rbp, rbp

    ; Call user's main function (mukhya)
    call {main_fn}

    ; Exit with return value from mukhya
    ; rax contains return code
    mov rdi, rax        ; exit code
    mov rax, 60         ; sys_exit
    syscall

.size _start, .-_start
"#,
            main_fn = self.main_fn
        )
    }

    /// Linux x86-64 with C runtime
    fn linux_x86_64_crt(&self) -> String {
        format!(
            r#"; Jagannath Runtime Entry Point (Linux x86-64 with CRT)
; जगन्नाथ रनटाइम प्रवेश बिंदु
.intel_syntax noprefix

.section .text
.global main
.type main, @function

main:
    push rbp
    mov rbp, rsp

    ; Call user's main function (mukhya)
    call {main_fn}

    ; Return value already in rax
    pop rbp
    ret

.size main, .-main
"#,
            main_fn = self.main_fn
        )
    }

    /// Windows x86-64 bare entry (no CRT)
    fn windows_x86_64_bare(&self) -> String {
        format!(
            r#"; Jagannath Runtime Entry Point (Windows x86-64 bare)
; जगन्नाथ रनटाइम प्रवेश बिंदु

.code

PUBLIC mainCRTStartup
mainCRTStartup PROC
    sub rsp, 40         ; Shadow space + alignment

    ; Call user's main function (mukhya)
    call {main_fn}

    ; Exit process with return value
    mov ecx, eax        ; exit code
    call ExitProcess

    ; Never reached
    ret
mainCRTStartup ENDP

END
"#,
            main_fn = self.main_fn
        )
    }

    /// Windows x86-64 with C runtime
    fn windows_x86_64_crt(&self) -> String {
        format!(
            r#"; Jagannath Runtime Entry Point (Windows x86-64 with CRT)
; जगन्नाथ रनटाइम प्रवेश बिंदु
.intel_syntax noprefix

.section .text
.global main

main:
    push rbp
    mov rbp, rsp
    sub rsp, 32         ; Shadow space

    ; Call user's main function (mukhya)
    call {main_fn}

    ; Return value already in eax
    add rsp, 32
    pop rbp
    ret

"#,
            main_fn = self.main_fn
        )
    }

    /// macOS x86-64 entry
    fn macos_x86_64(&self) -> String {
        format!(
            r#"; Jagannath Runtime Entry Point (macOS x86-64)
; जगन्नाथ रनटाइम प्रवेश बिंदु
.intel_syntax noprefix

.section __TEXT,__text
.global _main

_main:
    push rbp
    mov rbp, rsp

    ; Call user's main function (mukhya)
    call _{main_fn}

    ; Return value already in eax
    pop rbp
    ret

"#,
            main_fn = self.main_fn
        )
    }

    /// Linux AArch64 entry
    fn linux_aarch64(&self) -> String {
        format!(
            r#"; Jagannath Runtime Entry Point (Linux AArch64)
; जगन्नाथ रनटाइम प्रवेश बिंदु

.section .text
.global main
.type main, %function

main:
    stp x29, x30, [sp, #-16]!
    mov x29, sp

    ; Call user's main function (mukhya)
    bl {main_fn}

    ; Return value already in x0/w0
    ldp x29, x30, [sp], #16
    ret

.size main, .-main
"#,
            main_fn = self.main_fn
        )
    }

    /// macOS AArch64 entry
    fn macos_aarch64(&self) -> String {
        format!(
            r#"; Jagannath Runtime Entry Point (macOS AArch64)
; जगन्नाथ रनटाइम प्रवेश बिंदु

.section __TEXT,__text
.global _main

_main:
    stp x29, x30, [sp, #-16]!
    mov x29, sp

    ; Call user's main function (mukhya)
    bl _{main_fn}

    ; Return value already in x0/w0
    ldp x29, x30, [sp], #16
    ret

"#,
            main_fn = self.main_fn
        )
    }
}

impl Default for RuntimeEntry {
    fn default() -> Self {
        Self::for_current_platform()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linux_bare_entry() {
        let entry = RuntimeEntry::linux_bare();
        let asm = entry.generate();
        assert!(asm.contains("_start"));
        assert!(asm.contains("call mukhya"));
        assert!(asm.contains("sys_exit"));
    }

    #[test]
    fn test_windows_crt_entry() {
        let entry = RuntimeEntry::windows_crt();
        let asm = entry.generate();
        assert!(asm.contains("main"));
        assert!(asm.contains("call mukhya"));
    }
}
