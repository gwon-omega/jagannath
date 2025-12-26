//! Linker Interface
//!
//! Interface for linking object files.

use std::path::Path;

/// Linker
pub struct Linker {
    /// Linker command
    command: String,
    /// Linker flags
    flags: Vec<String>,
    /// Object files to link
    objects: Vec<String>,
    /// Libraries to link
    libraries: Vec<String>,
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
        Self {
            command: "ld".to_string(),
            flags: Vec::new(),
            objects: Vec::new(),
            libraries: Vec::new(),
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
    pub fn add_object(&mut self, path: &str) {
        self.objects.push(path.to_string());
    }

    /// Add a library
    pub fn add_library(&mut self, name: &str) {
        self.libraries.push(name.to_string());
    }

    /// Link to produce output
    pub fn link(&self, output: &Path, kind: LinkOutput) -> Result<(), LinkerError> {
        // TODO: Execute linker
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
                return vec!["ar".to_string(), "rcs".to_string(), output.to_string_lossy().to_string()];
            }
            LinkOutput::Object => {
                args.push("-r".to_string());
            }
        }

        // Output file
        args.push("-o".to_string());
        args.push(output.to_string_lossy().to_string());

        // Object files
        args.extend(self.objects.iter().cloned());

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
