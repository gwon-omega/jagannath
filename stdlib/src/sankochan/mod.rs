//! # Sankochan - Compression (संकोचन)
//!
//! Data compression algorithms.

pub mod rle;      // Run-length encoding
pub mod huffman;  // Huffman coding
pub mod lz;       // LZ-style compression

// Re-exports
pub use rle::*;
pub use huffman::*;
pub use lz::*;
