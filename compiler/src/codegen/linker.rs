//! Linker Interface
//!
//! Interface for assembling and linking object files.
//! Supports Windows (MSVC/MinGW) and Unix (GCC/Clang) toolchains.

use std::path::{Path, PathBuf};
use std::process::Command;

/// Assembler for converting .s to .o
pub struct Assembler {
    /// Assembler command
    command: String,
    /// Assembler flags
    flags: Vec<String>,
}

impl Assembler {
    pub fn new() -> Self {
        // Auto-detect platform
        let command = if cfg!(target_os = "windows") {
            // Try to find NASM or use GAS from MinGW
            "nasm".to_string()
        } else {
            "as".to_string()
        };

        Self {
            command,
            flags: Vec::new(),
        }
    }

    /// Create assembler for GAS (GNU Assembler)
    pub fn gas() -> Self {
        Self {
            command: "as".to_string(),
            flags: vec!["--64".to_string()],
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
        }
    }

    /// Create assembler for Clang
    pub fn clang() -> Self {
        Self {
            command: "clang".to_string(),
            flags: vec!["-c".to_string(), "-x".to_string(), "assembler".to_string()],
        }
    }

    /// Assemble source file to object file
    pub fn assemble(&self, input: &Path, output: &Path) -> Result<(), AssemblerError> {
        let mut cmd = Command::new(&self.command);
        cmd.args(&self.flags);

        // Platform-specific output flag
        if self.command.contains("nasm") {
            cmd.arg("-o").arg(output);
            cmd.arg(input);
        } else if self.command.contains("clang") || self.command.contains("gcc") {
            cmd.arg("-o").arg(output);
            cmd.arg(input);
        } else {
            // GNU as
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
}

impl BuildPipeline {
    pub fn new() -> Self {
        Self {
            assembler: Assembler::new(),
            linker: Linker::gcc(),
        }
    }

    /// Create pipeline using clang toolchain
    pub fn clang() -> Self {
        Self {
            assembler: Assembler::clang(),
            linker: Linker::clang(),
        }
    }

    /// Build assembly source to executable
    pub fn build_executable(&self, asm_path: &Path, exe_path: &Path) -> Result<(), BuildError> {
        // First, assemble to object file
        let obj_path = asm_path.with_extension("o");

        self.assembler
            .assemble(asm_path, &obj_path)
            .map_err(|e| BuildError::AssemblyFailed(format!("{:?}", e)))?;

        // Then link to executable
        let mut linker = Linker::gcc();
        linker.add_object(&obj_path);
        linker.add_library("c"); // C runtime for basic functions

        linker
            .link(exe_path, LinkOutput::Executable)
            .map_err(|e| BuildError::LinkFailed(format!("{:?}", e)))?;

        // Clean up object file
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
