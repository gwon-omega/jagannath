//! # Huffman - Huffman Coding (हफमैन संकेतन)
//!
//! Huffman compression algorithm.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::collections::BTreeMap;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Huffman tree node
#[cfg(feature = "alloc")]
#[derive(Debug)]
pub struct HuffmanGanth {
    /// Frequency
    pub aavriti: usize,
    /// Symbol (if leaf)
    pub pratik: Option<u8>,
    /// Left child
    pub vaam: Option<Box<HuffmanGanth>>,
    /// Right child
    pub dakshin: Option<Box<HuffmanGanth>>,
}

#[cfg(feature = "alloc")]
impl HuffmanGanth {
    /// Create leaf node
    pub fn patti(pratik: u8, aavriti: usize) -> Self {
        Self {
            aavriti,
            pratik: Some(pratik),
            vaam: None,
            dakshin: None,
        }
    }

    /// Create internal node
    pub fn antarik(vaam: HuffmanGanth, dakshin: HuffmanGanth) -> Self {
        Self {
            aavriti: vaam.aavriti + dakshin.aavriti,
            pratik: None,
            vaam: Some(Box::new(vaam)),
            dakshin: Some(Box::new(dakshin)),
        }
    }

    /// Is leaf
    pub fn patti_hai(&self) -> bool {
        self.pratik.is_some()
    }
}

/// Code table entry
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct SanketPravisti {
    /// Bit pattern
    pub bits: Vec<bool>,
}

#[cfg(feature = "alloc")]
impl SanketPravisti {
    pub fn naya() -> Self {
        Self { bits: Vec::new() }
    }

    pub fn lambai(&self) -> usize {
        self.bits.len()
    }
}

/// Build frequency table
#[cfg(feature = "alloc")]
pub fn aavriti_talika(data: &[u8]) -> BTreeMap<u8, usize> {
    let mut freq = BTreeMap::new();
    for &byte in data {
        *freq.entry(byte).or_insert(0) += 1;
    }
    freq
}

/// Build Huffman tree
#[cfg(feature = "alloc")]
pub fn vriksha_banao(freq: &BTreeMap<u8, usize>) -> Option<HuffmanGanth> {
    if freq.is_empty() {
        return None;
    }

    // Create leaf nodes
    let mut nodes: Vec<HuffmanGanth> = freq
        .iter()
        .map(|(&sym, &f)| HuffmanGanth::patti(sym, f))
        .collect();

    // Build tree using simple selection (priority queue would be better)
    while nodes.len() > 1 {
        // Sort by frequency
        nodes.sort_by(|a, b| b.aavriti.cmp(&a.aavriti));

        // Take two smallest
        let right = nodes.pop().unwrap();
        let left = nodes.pop().unwrap();

        // Create parent
        nodes.push(HuffmanGanth::antarik(left, right));
    }

    nodes.pop()
}

/// Build code table from tree
#[cfg(feature = "alloc")]
pub fn sanket_talika(root: &HuffmanGanth) -> BTreeMap<u8, SanketPravisti> {
    let mut table = BTreeMap::new();
    let mut path = SanketPravisti::naya();

    sanket_talika_inner(root, &mut path, &mut table);

    table
}

#[cfg(feature = "alloc")]
fn sanket_talika_inner(
    node: &HuffmanGanth,
    path: &mut SanketPravisti,
    table: &mut BTreeMap<u8, SanketPravisti>,
) {
    if let Some(sym) = node.pratik {
        table.insert(sym, path.clone());
    } else {
        if let Some(ref left) = node.vaam {
            path.bits.push(false);
            sanket_talika_inner(left, path, table);
            path.bits.pop();
        }

        if let Some(ref right) = node.dakshin {
            path.bits.push(true);
            sanket_talika_inner(right, path, table);
            path.bits.pop();
        }
    }
}

/// Encode data using Huffman
#[cfg(feature = "alloc")]
pub fn sanket(data: &[u8]) -> (Vec<u8>, HuffmanGanth, usize) {
    let freq = aavriti_talika(data);
    let tree = vriksha_banao(&freq).unwrap_or(HuffmanGanth::patti(0, 0));
    let table = sanket_talika(&tree);

    // Encode to bits
    let mut bits: Vec<bool> = Vec::new();
    for &byte in data {
        if let Some(code) = table.get(&byte) {
            bits.extend(&code.bits);
        }
    }

    let bit_count = bits.len();

    // Pack bits into bytes
    let mut bytes = Vec::with_capacity((bits.len() + 7) / 8);
    let mut current_byte = 0u8;
    let mut bit_pos = 0;

    for bit in bits {
        if bit {
            current_byte |= 1 << (7 - bit_pos);
        }
        bit_pos += 1;

        if bit_pos == 8 {
            bytes.push(current_byte);
            current_byte = 0;
            bit_pos = 0;
        }
    }

    if bit_pos > 0 {
        bytes.push(current_byte);
    }

    (bytes, tree, bit_count)
}

/// Decode Huffman encoded data
#[cfg(feature = "alloc")]
pub fn visanket(encoded: &[u8], tree: &HuffmanGanth, bit_count: usize) -> Vec<u8> {
    let mut result = Vec::new();
    let mut current = tree;
    let mut bits_read = 0;

    'outer: for &byte in encoded {
        for i in 0..8 {
            if bits_read >= bit_count {
                break 'outer;
            }

            let bit = (byte >> (7 - i)) & 1 == 1;
            bits_read += 1;

            current = if bit {
                current.dakshin.as_ref().map(|b| b.as_ref()).unwrap_or(tree)
            } else {
                current.vaam.as_ref().map(|b| b.as_ref()).unwrap_or(tree)
            };

            if let Some(sym) = current.pratik {
                result.push(sym);
                current = tree;
            }
        }
    }

    result
}

/// Calculate average code length
#[cfg(feature = "alloc")]
pub fn madhya_sanket_lambai(
    freq: &BTreeMap<u8, usize>,
    table: &BTreeMap<u8, SanketPravisti>,
) -> f64 {
    let total: usize = freq.values().sum();
    if total == 0 {
        return 0.0;
    }

    let weighted_sum: f64 = freq
        .iter()
        .map(|(sym, &f)| {
            let len = table.get(sym).map(|c| c.lambai()).unwrap_or(0);
            (f * len) as f64
        })
        .sum();

    weighted_sum / total as f64
}

/// Calculate entropy
#[cfg(feature = "alloc")]
pub fn entropy(freq: &BTreeMap<u8, usize>) -> f64 {
    let total: usize = freq.values().sum();
    if total == 0 {
        return 0.0;
    }

    freq.values()
        .filter(|&&f| f > 0)
        .map(|&f| {
            let p = f as f64 / total as f64;
            -p * libm::log2(p)
        })
        .sum()
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_frequency() {
        let data = b"AABBC";
        let freq = aavriti_talika(data);

        assert_eq!(freq.get(&b'A'), Some(&2));
        assert_eq!(freq.get(&b'B'), Some(&2));
        assert_eq!(freq.get(&b'C'), Some(&1));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_huffman_roundtrip() {
        let data = b"AAABBBCCCCDDDD";
        let (encoded, tree, bit_count) = sanket(data);
        let decoded = visanket(&encoded, &tree, bit_count);

        assert_eq!(data.as_slice(), decoded.as_slice());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_entropy() {
        let freq = aavriti_talika(b"AABB");
        let h = entropy(&freq);

        // Two symbols with equal probability -> entropy = 1.0
        assert!((h - 1.0).abs() < 0.01);
    }
}
