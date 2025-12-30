//! # Vistar - Path Extension Helpers (विस्तार)
//!
//! File extension utilities and MIME type detection.
//!
//! > **"विस्तारेण ज्ञायते प्रकारः"**
//! > *"The type is known by the extension"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::string::String;

// ============================================================================
// COMMON EXTENSIONS
// ============================================================================

/// Text file extensions
pub const PATHA_VISTAR: &[&str] = &["txt", "text", "log", "md", "markdown", "rst"];

/// Code file extensions
pub const SANKET_VISTAR: &[&str] = &[
    "rs", "py", "js", "ts", "c", "cpp", "h", "hpp", "java", "go", "rb", "php", "cs", "swift", "kt",
    "jag", // Jagannath!
];

/// Data file extensions
pub const ANKADA_VISTAR: &[&str] = &["json", "yaml", "yml", "xml", "csv", "tsv", "toml"];

/// Image file extensions
pub const CHITRA_VISTAR: &[&str] = &["png", "jpg", "jpeg", "gif", "bmp", "svg", "webp", "ico"];

/// Audio file extensions
pub const DHVANI_VISTAR: &[&str] = &["mp3", "wav", "ogg", "flac", "aac", "m4a", "wma"];

/// Video file extensions
pub const CHALCHITRA_VISTAR: &[&str] = &["mp4", "avi", "mkv", "mov", "wmv", "flv", "webm"];

/// Archive file extensions
pub const SANGRAHA_VISTAR: &[&str] = &["zip", "tar", "gz", "bz2", "xz", "7z", "rar"];

/// Document file extensions
pub const DASTAVEJ_VISTAR: &[&str] = &[
    "pdf", "doc", "docx", "odt", "rtf", "xls", "xlsx", "ppt", "pptx",
];

// ============================================================================
// FILE TYPE
// ============================================================================

/// File type category (फाइल प्रकार)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PhailaPrakara {
    /// Text files (पाठ)
    Patha,
    /// Source code (कोड)
    Sanket,
    /// Data files (आंकड़ा)
    Ankada,
    /// Images (चित्र)
    Chitra,
    /// Audio (ध्वनि)
    Dhvani,
    /// Video (चलचित्र)
    Chalchitra,
    /// Archives (संग्रह)
    Sangraha,
    /// Documents (दस्तावेज)
    Dastavej,
    /// Unknown (अज्ञात)
    Agyata,
}

impl PhailaPrakara {
    /// Detect file type from extension
    pub fn vishleshan(ext: &str) -> Self {
        let ext_lower = ext.to_ascii_lowercase();
        let ext_str = ext_lower.as_str();

        if PATHA_VISTAR.contains(&ext_str) {
            PhailaPrakara::Patha
        } else if SANKET_VISTAR.contains(&ext_str) {
            PhailaPrakara::Sanket
        } else if ANKADA_VISTAR.contains(&ext_str) {
            PhailaPrakara::Ankada
        } else if CHITRA_VISTAR.contains(&ext_str) {
            PhailaPrakara::Chitra
        } else if DHVANI_VISTAR.contains(&ext_str) {
            PhailaPrakara::Dhvani
        } else if CHALCHITRA_VISTAR.contains(&ext_str) {
            PhailaPrakara::Chalchitra
        } else if SANGRAHA_VISTAR.contains(&ext_str) {
            PhailaPrakara::Sangraha
        } else if DASTAVEJ_VISTAR.contains(&ext_str) {
            PhailaPrakara::Dastavej
        } else {
            PhailaPrakara::Agyata
        }
    }

    /// Check if type is text-based
    pub fn patha_adharit(&self) -> bool {
        matches!(
            self,
            PhailaPrakara::Patha | PhailaPrakara::Sanket | PhailaPrakara::Ankada
        )
    }

    /// Check if type is binary
    pub fn dvyanka_adharit(&self) -> bool {
        matches!(
            self,
            PhailaPrakara::Chitra
                | PhailaPrakara::Dhvani
                | PhailaPrakara::Chalchitra
                | PhailaPrakara::Sangraha
        )
    }
}

// ============================================================================
// MIME TYPES
// ============================================================================

/// Get MIME type for extension
#[cfg(feature = "alloc")]
pub fn mime_prakara(ext: &str) -> &'static str {
    match ext.to_ascii_lowercase().as_str() {
        // Text
        "txt" | "text" => "text/plain",
        "html" | "htm" => "text/html",
        "css" => "text/css",
        "csv" => "text/csv",
        "md" | "markdown" => "text/markdown",

        // Code
        "js" => "text/javascript",
        "json" => "application/json",
        "xml" => "application/xml",
        "yaml" | "yml" => "application/yaml",

        // Images
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        "ico" => "image/x-icon",
        "bmp" => "image/bmp",

        // Audio
        "mp3" => "audio/mpeg",
        "wav" => "audio/wav",
        "ogg" => "audio/ogg",
        "flac" => "audio/flac",
        "m4a" => "audio/mp4",

        // Video
        "mp4" => "video/mp4",
        "webm" => "video/webm",
        "avi" => "video/x-msvideo",
        "mov" => "video/quicktime",
        "mkv" => "video/x-matroska",

        // Documents
        "pdf" => "application/pdf",
        "doc" => "application/msword",
        "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
        "xls" => "application/vnd.ms-excel",
        "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
        "ppt" => "application/vnd.ms-powerpoint",
        "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",

        // Archives
        "zip" => "application/zip",
        "tar" => "application/x-tar",
        "gz" | "gzip" => "application/gzip",
        "7z" => "application/x-7z-compressed",
        "rar" => "application/vnd.rar",

        // Fonts
        "ttf" => "font/ttf",
        "otf" => "font/otf",
        "woff" => "font/woff",
        "woff2" => "font/woff2",

        // Binary
        "bin" => "application/octet-stream",
        "exe" => "application/x-msdownload",
        "wasm" => "application/wasm",

        _ => "application/octet-stream",
    }
}

/// Get extension for MIME type
#[cfg(feature = "alloc")]
pub fn vistar_se_mime(mime: &str) -> Option<&'static str> {
    match mime.to_ascii_lowercase().as_str() {
        "text/plain" => Some("txt"),
        "text/html" => Some("html"),
        "text/css" => Some("css"),
        "text/csv" => Some("csv"),
        "text/javascript" | "application/javascript" => Some("js"),
        "application/json" => Some("json"),
        "application/xml" | "text/xml" => Some("xml"),
        "image/png" => Some("png"),
        "image/jpeg" => Some("jpg"),
        "image/gif" => Some("gif"),
        "image/svg+xml" => Some("svg"),
        "image/webp" => Some("webp"),
        "audio/mpeg" => Some("mp3"),
        "audio/wav" => Some("wav"),
        "audio/ogg" => Some("ogg"),
        "video/mp4" => Some("mp4"),
        "video/webm" => Some("webm"),
        "application/pdf" => Some("pdf"),
        "application/zip" => Some("zip"),
        _ => None,
    }
}

// ============================================================================
// LANGUAGE DETECTION
// ============================================================================

/// Programming language
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BhashaKrama {
    Jagannath,
    Rust,
    Python,
    JavaScript,
    TypeScript,
    C,
    Cpp,
    Java,
    Go,
    Ruby,
    Php,
    CSharp,
    Swift,
    Kotlin,
    Html,
    Css,
    Sql,
    Shell,
    Other,
}

impl BhashaKrama {
    /// Detect language from extension
    pub fn vishleshan(ext: &str) -> Self {
        match ext.to_ascii_lowercase().as_str() {
            "jag" => BhashaKrama::Jagannath,
            "rs" => BhashaKrama::Rust,
            "py" | "pyw" => BhashaKrama::Python,
            "js" | "mjs" | "cjs" => BhashaKrama::JavaScript,
            "ts" | "tsx" => BhashaKrama::TypeScript,
            "c" | "h" => BhashaKrama::C,
            "cpp" | "cc" | "cxx" | "hpp" => BhashaKrama::Cpp,
            "java" => BhashaKrama::Java,
            "go" => BhashaKrama::Go,
            "rb" => BhashaKrama::Ruby,
            "php" => BhashaKrama::Php,
            "cs" => BhashaKrama::CSharp,
            "swift" => BhashaKrama::Swift,
            "kt" | "kts" => BhashaKrama::Kotlin,
            "html" | "htm" => BhashaKrama::Html,
            "css" | "scss" | "sass" | "less" => BhashaKrama::Css,
            "sql" => BhashaKrama::Sql,
            "sh" | "bash" | "zsh" => BhashaKrama::Shell,
            _ => BhashaKrama::Other,
        }
    }

    /// Get typical extension
    pub fn vistar(&self) -> &'static str {
        match self {
            BhashaKrama::Jagannath => "jag",
            BhashaKrama::Rust => "rs",
            BhashaKrama::Python => "py",
            BhashaKrama::JavaScript => "js",
            BhashaKrama::TypeScript => "ts",
            BhashaKrama::C => "c",
            BhashaKrama::Cpp => "cpp",
            BhashaKrama::Java => "java",
            BhashaKrama::Go => "go",
            BhashaKrama::Ruby => "rb",
            BhashaKrama::Php => "php",
            BhashaKrama::CSharp => "cs",
            BhashaKrama::Swift => "swift",
            BhashaKrama::Kotlin => "kt",
            BhashaKrama::Html => "html",
            BhashaKrama::Css => "css",
            BhashaKrama::Sql => "sql",
            BhashaKrama::Shell => "sh",
            BhashaKrama::Other => "",
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
    fn test_file_type_detection() {
        assert_eq!(PhailaPrakara::vishleshan("txt"), PhailaPrakara::Patha);
        assert_eq!(PhailaPrakara::vishleshan("rs"), PhailaPrakara::Sanket);
        assert_eq!(PhailaPrakara::vishleshan("jag"), PhailaPrakara::Sanket);
        assert_eq!(PhailaPrakara::vishleshan("json"), PhailaPrakara::Ankada);
        assert_eq!(PhailaPrakara::vishleshan("png"), PhailaPrakara::Chitra);
        assert_eq!(PhailaPrakara::vishleshan("mp3"), PhailaPrakara::Dhvani);
        assert_eq!(PhailaPrakara::vishleshan("mp4"), PhailaPrakara::Chalchitra);
        assert_eq!(PhailaPrakara::vishleshan("zip"), PhailaPrakara::Sangraha);
        assert_eq!(PhailaPrakara::vishleshan("pdf"), PhailaPrakara::Dastavej);
        assert_eq!(PhailaPrakara::vishleshan("xyz"), PhailaPrakara::Agyata);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_mime_type() {
        assert_eq!(mime_prakara("json"), "application/json");
        assert_eq!(mime_prakara("png"), "image/png");
        assert_eq!(mime_prakara("mp4"), "video/mp4");
    }

    #[test]
    fn test_language_detection() {
        assert_eq!(BhashaKrama::vishleshan("jag"), BhashaKrama::Jagannath);
        assert_eq!(BhashaKrama::vishleshan("rs"), BhashaKrama::Rust);
        assert_eq!(BhashaKrama::vishleshan("py"), BhashaKrama::Python);
        assert_eq!(BhashaKrama::vishleshan("js"), BhashaKrama::JavaScript);
    }
}
