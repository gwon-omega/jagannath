//! Kośa - File I/O (कोश)
//!
//! File system operations.

use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

/// Read entire file (पठ - paṭha)
pub fn patha<P: AsRef<Path>>(path: P) -> io::Result<String> {
    fs::read_to_string(path)
}

/// Read file as bytes (पठ बाइट - paṭha bytes)
pub fn patha_bytes<P: AsRef<Path>>(path: P) -> io::Result<Vec<u8>> {
    fs::read(path)
}

/// Write to file (लिख - likha)
pub fn likha<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, contents: C) -> io::Result<()> {
    fs::write(path, contents)
}

/// Append to file (योजय - yojaya)
pub fn yojaya<P: AsRef<Path>>(path: P, contents: &str) -> io::Result<()> {
    use std::fs::OpenOptions;
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(path)?;
    file.write_all(contents.as_bytes())
}

/// File exists (अस्ति - asti)
pub fn asti<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().exists()
}

/// Is directory (सञ्चिका - sañcikā)
pub fn sancika<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_dir()
}

/// Is file (प्रलेख - pralekha)
pub fn pralekha<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().is_file()
}

/// Create directory (निर्माण - nirmāṇa)
pub fn nirmana<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::create_dir_all(path)
}

/// Remove file (निष्कासय - niṣkāsaya)
pub fn nishkasaya<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::remove_file(path)
}

/// Remove directory (निष्कासय सञ्चिका - niṣkāsaya sañcikā)
pub fn nishkasaya_sancika<P: AsRef<Path>>(path: P) -> io::Result<()> {
    fs::remove_dir_all(path)
}

/// Copy file (प्रतिलिपि - pratilipi)
pub fn pratilipi<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<u64> {
    fs::copy(from, to)
}

/// Rename/move (नामकरण - nāmakaraṇa)
pub fn namakarana<P: AsRef<Path>, Q: AsRef<Path>>(from: P, to: Q) -> io::Result<()> {
    fs::rename(from, to)
}

/// File handle (Kośa-Dhāraka - कोशधारक)
pub struct KoshaDharaka {
    inner: fs::File,
}

impl KoshaDharaka {
    /// Open for reading (पठनार्थम् - paṭhanārtham)
    pub fn pathanartham<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Ok(Self {
            inner: fs::File::open(path)?
        })
    }

    /// Create for writing (लेखनार्थम् - lekhanārtham)
    pub fn lekhanartham<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        Ok(Self {
            inner: fs::File::create(path)?
        })
    }

    /// Read all (सर्वं पठ - sarvaṃ paṭha)
    pub fn sarvam_patha(&mut self) -> io::Result<String> {
        let mut contents = String::new();
        self.inner.read_to_string(&mut contents)?;
        Ok(contents)
    }

    /// Write all (सर्वं लिख - sarvaṃ likha)
    pub fn sarvam_likha(&mut self, contents: &str) -> io::Result<()> {
        self.inner.write_all(contents.as_bytes())
    }
}
