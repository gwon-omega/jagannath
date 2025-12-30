//! Jagannath Standard Library (जगन्नाथ मानक पुस्तकालय)
//!
//! All APIs use Sanskrit naming conventions.

#![cfg_attr(not(feature = "std"), no_std)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

#[cfg(feature = "alloc")]
extern crate alloc;

// Core types
pub mod prakara; // Types (प्रकार)
pub mod sankhya; // Numbers (संख्या)
pub mod suci;
pub mod sutra; // Strings (सूत्र) // Collections (सूची)

// Mathematics (गणित)
pub mod ganita; // Mathematics (गणित)

// Algorithms (क्रम)
pub mod krama; // Algorithms (क्रम)

// Advanced Collections (संग्रह)
pub mod samgraha; // Advanced collections (संग्रह)

// Cryptography (गुप्त)
pub mod gupta; // Cryptography (गुप्त)

// Async Runtime (प्रवाह)
#[cfg(feature = "alloc")]
pub mod pravaah; // Async runtime (प्रवाह)

// Serialization (क्रमण)
#[cfg(feature = "alloc")]
pub mod kramana; // Serialization (क्रमण)

// Validation (नियम)
#[cfg(feature = "alloc")]
pub mod niyama; // Validation (नियम)

// Path utilities (पथ)
#[cfg(feature = "alloc")]
pub mod patha; // Path manipulation (पथ)

// Logging/Diagnostics (विवृति)
#[cfg(feature = "alloc")]
pub mod vivriti; // Logging (विवृति)

// System utilities (यन्त्र)
#[cfg(feature = "std")]
pub mod yantra; // System utilities (यन्त्र)

// Parsing utilities (विश्लेषण)
#[cfg(feature = "alloc")]
pub mod vishleshan; // Parsing (विश्लेषण)

// Identifiers (सङ्केत)
#[cfg(feature = "alloc")]
pub mod sanket; // Identifiers (सङ्केत)

// Testing (परीक्षण)
#[cfg(feature = "alloc")]
pub mod parikshan; // Testing utilities (परीक्षण)

// HTTP/Networking utilities (संचार)
#[cfg(feature = "alloc")]
pub mod sanchar; // HTTP utilities (संचार)

// Physics/Dynamics (गतिशील)
pub mod gatishil; // Physics and vectors (गतिशील)

// Graphics (चित्रण)
pub mod chitran; // Graphics primitives (चित्रण)

// Audio (ध्वनि)
pub mod dhwani; // Audio/Sound utilities (ध्वनि)

// Time utilities (समय)
#[cfg(feature = "alloc")]
pub mod samay; // Time/scheduling utilities (समय)

// Computation/Algorithms (सङ्गणक)
#[cfg(feature = "alloc")]
pub mod sanganak; // Computation algorithms (सङ्गणक)

// Bio-inspired algorithms (जनावरी)
#[cfg(feature = "alloc")]
pub mod jaanavari; // Genetic/swarm algorithms (जनावरी)

// Text processing (शब्द)
#[cfg(feature = "alloc")]
pub mod shabda; // Text/NLP utilities (शब्द)

// Data Structures (संरचना)
#[cfg(feature = "alloc")]
pub mod sanrachana; // Trees, Heaps, Graphs (संरचना)

// Compression (संकोचन)
#[cfg(feature = "alloc")]
pub mod sankochan; // RLE, Huffman, LZ (संकोचन)

// Randomness (यादृच्छिक)
pub mod yaadrchik; // PRNGs, Distributions (यादृच्छिक)

// Computational Geometry (ज्यामिति)
#[cfg(feature = "alloc")]
pub mod jyamiti; // Points, Shapes, Algorithms (ज्यामिति)

// Signal Processing (तरंग)
#[cfg(feature = "alloc")]
pub mod taranga; // DSP, FFT, Filters (तरंग)

// Linear Algebra (रेखागणित)
#[cfg(feature = "alloc")]
pub mod rekhaganit; // Vectors, Matrices, Decompositions (रेखागणित)

// Optimization (इष्टम्)
#[cfg(feature = "alloc")]
pub mod ishtam; // 1D, Multi-D, Constrained optimization (इष्टम्)

// Statistics (सांख्यिकी)
#[cfg(feature = "alloc")]
pub mod saankhyiki; // Descriptive, Distributions, Inference (सांख्यिकी)

// Numerical Methods (संख्यिक)
#[cfg(feature = "alloc")]
pub mod sankhyik; // Integration, Differentiation, ODEs (संख्यिक)

// Graph Algorithms (आलेख)
#[cfg(feature = "alloc")]
pub mod aalekh; // Graph representations, BFS, DFS, Dijkstra, MST (आलेख)

// I/O
#[cfg(feature = "std")]
pub mod jala;
#[cfg(feature = "std")]
pub mod kosha; // File I/O (कोश) // Network (जाल)

// Concurrency
#[cfg(feature = "std")]
pub mod tala;
#[cfg(feature = "std")]
pub mod tantu; // Threads (तन्तु) // Synchronization (ताल)

// Time
#[cfg(feature = "std")]
pub mod kala; // Time/Duration (काल)

// Memory
pub mod smriti; // Memory allocation (स्मृति)

// Philosophy (दर्शन)
pub mod darshana; // Hindu philosophy utilities (दर्शन)

// Prelude
pub mod upakrama; // Prelude (उपक्रम)

/// Common imports
pub mod prelude {
    pub use super::upakrama::*;
}
