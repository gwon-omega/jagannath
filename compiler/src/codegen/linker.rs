//! Linker Interface
//!
//! Interface for assembling and linking object files.
//! Supports Windows (MSVC/MinGW) and Unix (GCC/Clang) toolchains.

use super::entry::{Platform, RuntimeEntry};
use std::path::{Path, PathBuf};
use std::process::Command;

/// Assembler for converting .s to .o
pub struct Assembler {
    /// Assembler command
    command: String,
    /// Assembler flags
    flags: Vec<String>,
    /// Target platform
    platform: Platform,
}

impl Assembler {
    pub fn new() -> Self {
        // Auto-detect platform
        let (command, platform) = if cfg!(target_os = "windows") {
            // Try to find GCC from MinGW first, then NASM
            ("gcc".to_string(), Platform::WindowsX86_64)
        } else if cfg!(target_os = "macos") {
            ("clang".to_string(), Platform::MacOSX86_64)
        } else {
            ("as".to_string(), Platform::LinuxX86_64)
        };

        Self {
            command,
            flags: Vec::new(),
            platform,
        }
    }

    /// Create assembler for GAS (GNU Assembler)
    pub fn gas() -> Self {
        Self {
            command: "as".to_string(),
            flags: vec!["--64".to_string()],
            platform: Platform::LinuxX86_64,
        }
    }

    /// Create assembler for NASM
    pub fn nasm() -> Self {
        Self {
            command: "nasm".to_string(),
            flags: vec![
                "-f".to_string(),
                if cfg!(target_os = "windows") {
                    "win64".to_string()
                } else {
                    "elf64".to_string()
                },
            ],
            platform: if cfg!(target_os = "windows") {
                Platform::WindowsX86_64
            } else {
                Platform::LinuxX86_64
            },
        }
    }

    /// Create assembler using GCC driver (recommended for portability)
    pub fn gcc() -> Self {
        Self {
            command: "gcc".to_string(),
            flags: vec!["-c".to_string(), "-x".to_string(), "assembler".to_string()],
            platform: if cfg!(target_os = "windows") {
                Platform::WindowsX86_64
            } else if cfg!(target_os = "macos") {
                Platform::MacOSX86_64
            } else {
                Platform::LinuxX86_64
            },
        }
    }

    /// Create assembler for Clang
    pub fn clang() -> Self {
        Self {
            command: "clang".to_string(),
            flags: vec!["-c".to_string(), "-x".to_string(), "assembler".to_string()],
            platform: if cfg!(target_os = "windows") {
                Platform::WindowsX86_64
            } else if cfg!(target_os = "macos") {
                Platform::MacOSX86_64
            } else {
                Platform::LinuxX86_64
            },
        }
    }

    /// Assemble source file to object file
    pub fn assemble(&self, input: &Path, output: &Path) -> Result<(), AssemblerError> {
        let mut cmd = Command::new(&self.command);

        // Platform-specific handling
        if self.command.contains("nasm") {
            cmd.args(&self.flags);
            cmd.arg("-o").arg(output);
            cmd.arg(input);
        } else if self.command == "gcc" || self.command == "clang" {
            // GCC/Clang driver: gcc -c -x assembler input.s -o output.o
            cmd.args(&self.flags);
            cmd.arg(input);
            cmd.arg("-o").arg(output);
        } else {
            // GNU as
            cmd.args(&self.flags);
            cmd.arg("-o").arg(output);
            cmd.arg(input);
        }

        let result = cmd.output().map_err(|e| AssemblerError::IoError(e))?;

        if !result.status.success() {
            return Err(AssemblerError::AssemblyFailed {
                exit_code: result.status.code().unwrap_or(-1),
                stderr: String::from_utf8_lossy(&result.stderr).to_string(),
            });
        }

        Ok(())
    }

    /// Check if assembler is available
    pub fn is_available(&self) -> bool {
        Command::new(&self.command)
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }
}

impl Default for Assembler {
    fn default() -> Self {
        Self::new()
    }
}

/// Assembler error
#[derive(Debug)]
pub enum AssemblerError {
    /// Command not found
    CommandNotFound(String),
    /// Assembly failed
    AssemblyFailed { exit_code: i32, stderr: String },
    /// I/O error
    IoError(std::io::Error),
}

/// Linker
pub struct Linker {
    /// Linker command
    command: String,
    /// Linker flags
    flags: Vec<String>,
    /// Object files to link
    objects: Vec<PathBuf>,
    /// Libraries to link
    libraries: Vec<String>,
    /// Library search paths
    lib_paths: Vec<PathBuf>,
}

/// Link output type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinkOutput {
    /// Executable
    Executable,
    /// Shared library
    SharedLib,
    /// Static library
    StaticLib,
    /// Object file (just assemble)
    Object,
}

impl Linker {
    pub fn new() -> Self {
        // Auto-detect platform
        let command = if cfg!(target_os = "windows") {
            "link.exe".to_string() // MSVC linker
        } else if cfg!(target_os = "macos") {
            "ld".to_string()
        } else {
            "ld".to_string()
        };

        Self {
            command,
            flags: Vec::new(),
            objects: Vec::new(),
            libraries: Vec::new(),
            lib_paths: Vec::new(),
        }
    }

    /// Create linker using GCC driver
    pub fn gcc() -> Self {
        Self {
            command: "gcc".to_string(),
            flags: vec!["-no-pie".to_string()],
            objects: Vec::new(),
            libraries: Vec::new(),
            lib_paths: Vec::new(),
        }
    }

    /// Create linker using Clang driver
    pub fn clang() -> Self {
        Self {
            command: "clang".to_string(),
            flags: Vec::new(),
            objects: Vec::new(),
            libraries: Vec::new(),
            lib_paths: Vec::new(),
        }
    }

    /// Set linker command
    pub fn set_command(&mut self, cmd: &str) {
        self.command = cmd.to_string();
    }

    /// Add a linker flag
    pub fn add_flag(&mut self, flag: &str) {
        self.flags.push(flag.to_string());
    }

    /// Add an object file
    pub fn add_object(&mut self, path: &Path) {
        self.objects.push(path.to_path_buf());
    }

    /// Add a library
    pub fn add_library(&mut self, name: &str) {
        self.libraries.push(name.to_string());
    }

    /// Add library search path
    pub fn add_lib_path(&mut self, path: &Path) {
        self.lib_paths.push(path.to_path_buf());
    }

    /// Link to produce output
    pub fn link(&self, output: &Path, kind: LinkOutput) -> Result<(), LinkerError> {
        let mut cmd = Command::new(&self.command);
        cmd.args(&self.flags);

        // Output type flags
        match kind {
            LinkOutput::Executable => {}
            LinkOutput::SharedLib => {
                if self.command.contains("gcc") || self.command.contains("clang") {
                    cmd.arg("-shared");
                } else {
                    cmd.arg("-shared");
                }
            }
            LinkOutput::StaticLib => {
                // Use ar instead
                let mut ar = Command::new("ar");
                ar.args(["rcs", &output.to_string_lossy()]);
                for obj in &self.objects {
                    ar.arg(obj);
                }
                let result = ar.output().map_err(|e| LinkerError::IoError(e))?;
                if !result.status.success() {
                    return Err(LinkerError::LinkFailed {
                        exit_code: result.status.code().unwrap_or(-1),
                        stderr: String::from_utf8_lossy(&result.stderr).to_string(),
                    });
                }
                return Ok(());
            }
            LinkOutput::Object => {
                cmd.arg("-r");
            }
        }

        // Output file
        cmd.arg("-o").arg(output);

        // Library search paths
        for path in &self.lib_paths {
            cmd.arg("-L").arg(path);
        }

        // Object files
        for obj in &self.objects {
            cmd.arg(obj);
        }

        // Libraries
        for lib in &self.libraries {
            cmd.arg(format!("-l{}", lib));
        }

        let result = cmd.output().map_err(|e| LinkerError::IoError(e))?;

        if !result.status.success() {
            return Err(LinkerError::LinkFailed {
                exit_code: result.status.code().unwrap_or(-1),
                stderr: String::from_utf8_lossy(&result.stderr).to_string(),
            });
        }

        Ok(())
    }

    /// Get linker command line
    pub fn command_line(&self, output: &Path, kind: LinkOutput) -> Vec<String> {
        let mut args = vec![self.command.clone()];

        // Add flags
        args.extend(self.flags.iter().cloned());

        // Add output type flags
        match kind {
            LinkOutput::Executable => {}
            LinkOutput::SharedLib => args.push("-shared".to_string()),
            LinkOutput::StaticLib => {
                // Use ar instead
                return vec![
                    "ar".to_string(),
                    "rcs".to_string(),
                    output.to_string_lossy().to_string(),
                ];
            }
            LinkOutput::Object => {
                args.push("-r".to_string());
            }
        }

        // Output file
        args.push("-o".to_string());
        args.push(output.to_string_lossy().to_string());

        // Object files
        for obj in &self.objects {
            args.push(obj.to_string_lossy().to_string());
        }

        // Libraries
        for lib in &self.libraries {
            args.push(format!("-l{}", lib));
        }

        args
    }
}

/// Linker error
#[derive(Debug)]
pub enum LinkerError {
    /// Command not found
    CommandNotFound(String),
    /// Linking failed
    LinkFailed { exit_code: i32, stderr: String },
    /// I/O error
    IoError(std::io::Error),
}

impl Default for Linker {
    fn default() -> Self {
        Self::new()
    }
}

/// Build pipeline - coordinates assembling and linking
pub struct BuildPipeline {
    assembler: Assembler,
    linker: Linker,
    /// Platform for runtime entry
    platform: Platform,
    /// Whether to use C runtime
    use_crt: bool,
}

impl BuildPipeline {
    pub fn new() -> Self {
        let platform = if cfg!(target_os = "windows") {
            Platform::WindowsX86_64
        } else if cfg!(target_os = "macos") {
            Platform::MacOSX86_64
        } else {
            Platform::LinuxX86_64
        };

        Self {
            assembler: Assembler::gcc(),
            linker: Linker::gcc(),
            platform,
            use_crt: true,
        }
    }

    /// Create pipeline using clang toolchain
    pub fn clang() -> Self {
        let platform = if cfg!(target_os = "windows") {
            Platform::WindowsX86_64
        } else if cfg!(target_os = "macos") {
            Platform::MacOSX86_64
        } else {
            Platform::LinuxX86_64
        };

        Self {
            assembler: Assembler::clang(),
            linker: Linker::clang(),
            platform,
            use_crt: true,
        }
    }

    /// Create pipeline for bare-metal (no CRT)
    pub fn bare() -> Self {
        let mut pipeline = Self::new();
        pipeline.use_crt = false;
        pipeline
    }

    /// Build assembly source to executable with runtime entry
    pub fn build_executable(&self, asm_path: &Path, exe_path: &Path) -> Result<(), BuildError> {
        // Create temporary directory for build artifacts
        let temp_dir = std::env::temp_dir().join("jagannath_build");
        std::fs::create_dir_all(&temp_dir)
            .map_err(|e| BuildError::AssemblyFailed(format!("Failed to create temp dir: {}", e)))?;

        // Generate runtime entry point
        let entry = RuntimeEntry {
            platform: self.platform,
            use_crt: self.use_crt,
            main_fn: "mukhya".to_string(),
        };
        let entry_asm = entry.generate();

        // Write entry point to temp file
        let entry_path = temp_dir.join("_entry.s");
        std::fs::write(&entry_path, &entry_asm)
            .map_err(|e| BuildError::AssemblyFailed(format!("Failed to write entry: {}", e)))?;

        // Read user assembly and combine with entry
        let user_asm = std::fs::read_to_string(asm_path)
            .map_err(|e| BuildError::AssemblyFailed(format!("Failed to read user asm: {}", e)))?;

        let combined_asm = format!("{}\n\n{}", entry_asm, user_asm);
        let combined_path = temp_dir.join("combined.s");
        std::fs::write(&combined_path, &combined_asm)
            .map_err(|e| BuildError::AssemblyFailed(format!("Failed to write combined: {}", e)))?;

        // Assemble combined file
        let obj_path = temp_dir.join("combined.o");
        self.assembler
            .assemble(&combined_path, &obj_path)
            .map_err(|e| BuildError::AssemblyFailed(format!("{:?}", e)))?;

        // Link to executable
        let mut linker = if self.use_crt {
            Linker::gcc()
        } else {
            let mut l = Linker::new();
            l.add_flag("-nostdlib");
            l.add_flag("-static");
            l
        };

        linker.add_object(&obj_path);

        // On Windows with MinGW, GCC automatically links the C runtime
        // On Linux/Unix, we need to explicitly request libc
        #[cfg(not(target_os = "windows"))]
        if self.use_crt {
            linker.add_library("c"); // C runtime (Linux/Unix)
        }

        linker
            .link(exe_path, LinkOutput::Executable)
            .map_err(|e| BuildError::LinkFailed(format!("{:?}", e)))?;

        // Clean up temp files
        let _ = std::fs::remove_file(&entry_path);
        let _ = std::fs::remove_file(&combined_path);
        let _ = std::fs::remove_file(&obj_path);

        Ok(())
    }

    /// Build assembly source to object file only
    pub fn build_object(&self, asm_path: &Path, obj_path: &Path) -> Result<(), BuildError> {
        self.assembler
            .assemble(asm_path, obj_path)
            .map_err(|e| BuildError::AssemblyFailed(format!("{:?}", e)))?;
        Ok(())
    }

    /// Build with verbose output
    pub fn build_executable_verbose(
        &self,
        asm_path: &Path,
        exe_path: &Path,
    ) -> Result<BuildInfo, BuildError> {
        let start = std::time::Instant::now();

        // Create temporary directory for build artifacts
        let temp_dir = std::env::temp_dir().join("jagannath_build");
        std::fs::create_dir_all(&temp_dir)
            .map_err(|e| BuildError::AssemblyFailed(format!("Failed to create temp dir: {}", e)))?;

        // Generate runtime entry point
        let entry = RuntimeEntry {
            platform: self.platform,
            use_crt: self.use_crt,
            main_fn: "mukhya".to_string(),
        };
        let entry_asm = entry.generate();

        // Read user assembly and combine with entry
        let user_asm = std::fs::read_to_string(asm_path)
            .map_err(|e| BuildError::AssemblyFailed(format!("Failed to read user asm: {}", e)))?;

        let combined_asm = format!("{}\n\n{}", entry_asm, user_asm);
        let combined_path = temp_dir.join("combined.s");
        std::fs::write(&combined_path, &combined_asm)
            .map_err(|e| BuildError::AssemblyFailed(format!("Failed to write combined: {}", e)))?;

        let assembly_time = start.elapsed();

        // Assemble combined file
        let asm_start = std::time::Instant::now();
        let obj_path = temp_dir.join("combined.o");
        self.assembler
            .assemble(&combined_path, &obj_path)
            .map_err(|e| BuildError::AssemblyFailed(format!("{:?}", e)))?;
        let assemble_time = asm_start.elapsed();

        // Link to executable
        let link_start = std::time::Instant::now();
        let mut linker = if self.use_crt {
            Linker::gcc()
        } else {
            let mut l = Linker::new();
            l.add_flag("-nostdlib");
            l.add_flag("-static");
            l
        };

        linker.add_object(&obj_path);
        if self.use_crt {
            linker.add_library("c");
        }

        linker
            .link(exe_path, LinkOutput::Executable)
            .map_err(|e| BuildError::LinkFailed(format!("{:?}", e)))?;
        let link_time = link_start.elapsed();

        // Clean up
        let _ = std::fs::remove_file(&combined_path);
        let _ = std::fs::remove_file(&obj_path);

        Ok(BuildInfo {
            assembly_time,
            assemble_time,
            link_time,
            total_time: start.elapsed(),
            exe_size: std::fs::metadata(exe_path).map(|m| m.len()).unwrap_or(0),
        })
    }
}

/// Build information
#[derive(Debug)]
pub struct BuildInfo {
    pub assembly_time: std::time::Duration,
    pub assemble_time: std::time::Duration,
    pub link_time: std::time::Duration,
    pub total_time: std::time::Duration,
    pub exe_size: u64,
}

impl Default for BuildPipeline {
    fn default() -> Self {
        Self::new()
    }
}

/// Build error
#[derive(Debug)]
pub enum BuildError {
    AssemblyFailed(String),
    LinkFailed(String),
}

impl std::fmt::Display for BuildError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BuildError::AssemblyFailed(s) => write!(f, "Assembly failed: {}", s),
            BuildError::LinkFailed(s) => write!(f, "Linking failed: {}", s),
        }
    }
}

impl std::error::Error for BuildError {}
