//! Jagannath I/O Primitives (इनपुट/आउटपुट प्राथमिक)
//!
//! Sanskrit-named I/O operations for the Jagannath language.
//!
//! ## Sanskrit Naming Convention
//! - मुद्रय (mudraya) - Print/stamp (to console)
//! - पठ (paṭha) - Read/recite
//! - लिख (likha) - Write/inscribe
//! - कोश (kosha) - Treasury/file storage

#[cfg(feature = "std")]
use std::io::{self, Read, Write, BufRead, BufReader};
#[cfg(feature = "std")]
use std::fs::{self, File, OpenOptions};
#[cfg(feature = "std")]
use std::path::Path;

// ============================================================================
// Console I/O (मुद्रय - Printing/Stamping)
// ============================================================================

/// Print to stdout (मुद्रय/mudraya)
#[cfg(feature = "std")]
pub fn print(s: &str) {
    let _ = io::stdout().write_all(s.as_bytes());
    let _ = io::stdout().flush();
}

/// Print line to stdout (मुद्रय पंक्ति/mudraya paṅkti)
#[cfg(feature = "std")]
pub fn println(s: &str) {
    let _ = writeln!(io::stdout(), "{}", s);
}

/// Print to stderr (त्रुटि मुद्रय/truṭi mudraya)
#[cfg(feature = "std")]
pub fn eprint(s: &str) {
    let _ = io::stderr().write_all(s.as_bytes());
    let _ = io::stderr().flush();
}

/// Print line to stderr (त्रुटि मुद्रय पंक्ति)
#[cfg(feature = "std")]
pub fn eprintln(s: &str) {
    let _ = writeln!(io::stderr(), "{}", s);
}

/// Read line from stdin (पठ पंक्ति/paṭha paṅkti)
#[cfg(feature = "std")]
pub fn read_line() -> io::Result<String> {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer)?;
    // Remove trailing newline
    if buffer.ends_with('\n') {
        buffer.pop();
        if buffer.ends_with('\r') {
            buffer.pop();
        }
    }
    Ok(buffer)
}

/// Read line with prompt (प्रश्न पठ/praśna paṭha)
#[cfg(feature = "std")]
pub fn prompt(message: &str) -> io::Result<String> {
    print(message);
    read_line()
}

// ============================================================================
// File I/O (कोश - Treasury/Storage)
// ============================================================================

/// Read entire file as string (कोश पठ/kosha paṭha)
#[cfg(feature = "std")]
pub fn kosha_patha(path: &str) -> io::Result<String> {
    fs::read_to_string(path)
}

/// Read file as bytes (कोश बाइट पठ)
#[cfg(feature = "std")]
pub fn kosha_bytes(path: &str) -> io::Result<Vec<u8>> {
    fs::read(path)
}

/// Write string to file (कोश लिख/kosha likha)
#[cfg(feature = "std")]
pub fn kosha_likha(path: &str, content: &str) -> io::Result<()> {
    fs::write(path, content)
}

/// Write bytes to file (कोश बाइट लिख)
#[cfg(feature = "std")]
pub fn kosha_likha_bytes(path: &str, content: &[u8]) -> io::Result<()> {
    fs::write(path, content)
}

/// Append to file (कोश योग/kosha yoga)
#[cfg(feature = "std")]
pub fn kosha_yoga(path: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;
    file.write_all(content.as_bytes())
}

/// Check if file exists (कोश अस्ति/kosha asti)
#[cfg(feature = "std")]
pub fn kosha_asti(path: &str) -> bool {
    Path::new(path).exists()
}

/// Delete file (कोश नाश/kosha nāśa)
#[cfg(feature = "std")]
pub fn kosha_nasha(path: &str) -> io::Result<()> {
    fs::remove_file(path)
}

/// Create directory (संग्रह निर्माण/saṅgraha nirmāṇa)
#[cfg(feature = "std")]
pub fn sangraha_nirmana(path: &str) -> io::Result<()> {
    fs::create_dir_all(path)
}

/// List directory contents (संग्रह सूची/saṅgraha sūcī)
#[cfg(feature = "std")]
pub fn sangraha_suchi(path: &str) -> io::Result<Vec<String>> {
    let entries = fs::read_dir(path)?;
    let mut names = Vec::new();
    for entry in entries {
        let entry = entry?;
        if let Some(name) = entry.file_name().to_str() {
            names.push(name.to_string());
        }
    }
    Ok(names)
}

// ============================================================================
// Buffered I/O (धारा - Stream)
// ============================================================================

/// File reader with buffering (कोश धारा/kosha dhārā)
#[cfg(feature = "std")]
pub struct KoshaDhara {
    reader: BufReader<File>,
}

#[cfg(feature = "std")]
impl KoshaDhara {
    /// Open file for reading
    pub fn khola(path: &str) -> io::Result<Self> {
        let file = File::open(path)?;
        Ok(Self {
            reader: BufReader::new(file),
        })
    }

    /// Read next line
    pub fn patha_pankti(&mut self) -> io::Result<Option<String>> {
        let mut line = String::new();
        let bytes = self.reader.read_line(&mut line)?;
        if bytes == 0 {
            Ok(None)
        } else {
            // Remove trailing newline
            if line.ends_with('\n') {
                line.pop();
                if line.ends_with('\r') {
                    line.pop();
                }
            }
            Ok(Some(line))
        }
    }

    /// Read all remaining content
    pub fn patha_sarva(&mut self) -> io::Result<String> {
        let mut content = String::new();
        self.reader.read_to_string(&mut content)?;
        Ok(content)
    }
}

/// File writer with buffering (कोश लेखक/kosha lekhaka)
#[cfg(feature = "std")]
pub struct KoshaLekhaka {
    writer: io::BufWriter<File>,
}

#[cfg(feature = "std")]
impl KoshaLekhaka {
    /// Create/truncate file for writing
    pub fn nirmana(path: &str) -> io::Result<Self> {
        let file = File::create(path)?;
        Ok(Self {
            writer: io::BufWriter::new(file),
        })
    }

    /// Open file for appending
    pub fn yoga(path: &str) -> io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        Ok(Self {
            writer: io::BufWriter::new(file),
        })
    }

    /// Write string
    pub fn likha(&mut self, s: &str) -> io::Result<()> {
        self.writer.write_all(s.as_bytes())
    }

    /// Write line
    pub fn likha_pankti(&mut self, s: &str) -> io::Result<()> {
        writeln!(self.writer, "{}", s)
    }

    /// Flush buffer
    pub fn prakshalan(&mut self) -> io::Result<()> {
        self.writer.flush()
    }
}

#[cfg(feature = "std")]
impl Drop for KoshaLekhaka {
    fn drop(&mut self) {
        let _ = self.writer.flush();
    }
}

// ============================================================================
// Sanskrit Aliases (संस्कृत उपनाम)
// ============================================================================

// Console aliases
#[cfg(feature = "std")]
pub use print as mudraya;
#[cfg(feature = "std")]
pub use println as mudraya_pankti;
#[cfg(feature = "std")]
pub use read_line as patha_pankti;
#[cfg(feature = "std")]
pub use prompt as prashna_patha;

// File aliases
#[cfg(feature = "std")]
pub use kosha_patha as patha_kosha;
#[cfg(feature = "std")]
pub use kosha_likha as likha_kosha;
#[cfg(feature = "std")]
pub use kosha_asti as asti_kosha;

// ============================================================================
// Formatted Output (स्वरूपित मुद्रण)
// ============================================================================

/// Format and print (similar to Rust's print! macro)
#[cfg(feature = "std")]
#[macro_export]
macro_rules! mudraya_fmt {
    ($($arg:tt)*) => {{
        use std::io::Write;
        let _ = write!(std::io::stdout(), $($arg)*);
        let _ = std::io::stdout().flush();
    }};
}

/// Format and print line (similar to Rust's println! macro)
#[cfg(feature = "std")]
#[macro_export]
macro_rules! mudraya_pankti_fmt {
    ($($arg:tt)*) => {{
        println!($($arg)*);
    }};
}
