//! # Prastutikaran - Graph Representations (प्रस्तुतीकरण)
//!
//! Graph data structures: adjacency list, adjacency matrix, edge list.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::collections::BTreeMap;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Edge with weight (किनारा)
#[derive(Debug, Clone, Copy)]
pub struct Kinara {
    pub lakshya: usize, // Target vertex
    pub bhaar: f64,     // Weight
}

/// Adjacency List Graph (संलग्नता सूची आलेख)
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct SanlagnataSuci {
    pub sheersh: Vec<Vec<Kinara>>, // Vertices with adjacency lists
    pub sankhya: usize,            // Number of vertices
    pub dishit: bool,              // Directed?
}

#[cfg(feature = "alloc")]
impl SanlagnataSuci {
    /// Create empty graph with n vertices
    pub fn naya(n: usize, dishit: bool) -> Self {
        Self {
            sheersh: vec![Vec::new(); n],
            sankhya: n,
            dishit,
        }
    }

    /// Add edge u -> v with weight w
    pub fn kinara_jodo(&mut self, u: usize, v: usize, w: f64) {
        if u < self.sankhya && v < self.sankhya {
            self.sheersh[u].push(Kinara {
                lakshya: v,
                bhaar: w,
            });

            if !self.dishit && u != v {
                self.sheersh[v].push(Kinara {
                    lakshya: u,
                    bhaar: w,
                });
            }
        }
    }

    /// Add unweighted edge
    pub fn kinara_jodo_abhar(&mut self, u: usize, v: usize) {
        self.kinara_jodo(u, v, 1.0);
    }

    /// Get neighbors of vertex u
    pub fn padosi(&self, u: usize) -> &[Kinara] {
        if u < self.sankhya {
            &self.sheersh[u]
        } else {
            &[]
        }
    }

    /// Get degree of vertex
    pub fn ghatank(&self, u: usize) -> usize {
        if u < self.sankhya {
            self.sheersh[u].len()
        } else {
            0
        }
    }

    /// Get number of edges
    pub fn kinara_sankhya(&self) -> usize {
        let total: usize = self.sheersh.iter().map(|adj| adj.len()).sum();
        if self.dishit {
            total
        } else {
            total / 2
        }
    }

    /// Check if edge exists
    pub fn kinara_hai(&self, u: usize, v: usize) -> bool {
        if u >= self.sankhya {
            return false;
        }

        self.sheersh[u].iter().any(|k| k.lakshya == v)
    }

    /// Get edge weight (returns None if no edge)
    pub fn kinara_bhaar(&self, u: usize, v: usize) -> Option<f64> {
        if u >= self.sankhya {
            return None;
        }

        self.sheersh[u]
            .iter()
            .find(|k| k.lakshya == v)
            .map(|k| k.bhaar)
    }

    /// Get all edges as (u, v, weight)
    pub fn kinara_suchi(&self) -> Vec<(usize, usize, f64)> {
        let mut edges = Vec::new();

        for u in 0..self.sankhya {
            for k in &self.sheersh[u] {
                if self.dishit || u <= k.lakshya {
                    edges.push((u, k.lakshya, k.bhaar));
                }
            }
        }

        edges
    }

    /// Transpose (reverse edges)
    pub fn parivart(&self) -> Self {
        let mut transposed = Self::naya(self.sankhya, self.dishit);

        for u in 0..self.sankhya {
            for k in &self.sheersh[u] {
                transposed.sheersh[k.lakshya].push(Kinara {
                    lakshya: u,
                    bhaar: k.bhaar,
                });
            }
        }

        transposed
    }
}

/// Adjacency Matrix Graph (संलग्नता मैट्रिक्स आलेख)
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct SanlagnataMatrix {
    pub matrix: Vec<Vec<Option<f64>>>,
    pub sankhya: usize,
    pub dishit: bool,
}

#[cfg(feature = "alloc")]
impl SanlagnataMatrix {
    /// Create empty graph with n vertices
    pub fn naya(n: usize, dishit: bool) -> Self {
        Self {
            matrix: vec![vec![None; n]; n],
            sankhya: n,
            dishit,
        }
    }

    /// Add edge u -> v with weight w
    pub fn kinara_jodo(&mut self, u: usize, v: usize, w: f64) {
        if u < self.sankhya && v < self.sankhya {
            self.matrix[u][v] = Some(w);

            if !self.dishit {
                self.matrix[v][u] = Some(w);
            }
        }
    }

    /// Remove edge
    pub fn kinara_hatao(&mut self, u: usize, v: usize) {
        if u < self.sankhya && v < self.sankhya {
            self.matrix[u][v] = None;

            if !self.dishit {
                self.matrix[v][u] = None;
            }
        }
    }

    /// Get edge weight
    pub fn kinara_bhaar(&self, u: usize, v: usize) -> Option<f64> {
        if u < self.sankhya && v < self.sankhya {
            self.matrix[u][v]
        } else {
            None
        }
    }

    /// Check if edge exists
    pub fn kinara_hai(&self, u: usize, v: usize) -> bool {
        self.kinara_bhaar(u, v).is_some()
    }

    /// Get degree of vertex
    pub fn ghatank(&self, u: usize) -> usize {
        if u >= self.sankhya {
            return 0;
        }

        self.matrix[u].iter().filter(|w| w.is_some()).count()
    }

    /// Get neighbors of vertex
    pub fn padosi(&self, u: usize) -> Vec<Kinara> {
        if u >= self.sankhya {
            return Vec::new();
        }

        self.matrix[u]
            .iter()
            .enumerate()
            .filter_map(|(v, w)| w.map(|bhaar| Kinara { lakshya: v, bhaar }))
            .collect()
    }

    /// Convert to adjacency list
    pub fn suci_me(&self) -> SanlagnataSuci {
        let mut graph = SanlagnataSuci::naya(self.sankhya, self.dishit);

        for u in 0..self.sankhya {
            for v in 0..self.sankhya {
                if let Some(w) = self.matrix[u][v] {
                    if self.dishit || u <= v {
                        graph.sheersh[u].push(Kinara {
                            lakshya: v,
                            bhaar: w,
                        });
                        if !self.dishit && u != v {
                            graph.sheersh[v].push(Kinara {
                                lakshya: u,
                                bhaar: w,
                            });
                        }
                    }
                }
            }
        }

        graph
    }
}

/// Edge List representation (किनारा सूची)
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct KinaraSuci {
    pub kinaare: Vec<(usize, usize, f64)>,
    pub sankhya: usize,
    pub dishit: bool,
}

#[cfg(feature = "alloc")]
impl KinaraSuci {
    pub fn naya(n: usize, dishit: bool) -> Self {
        Self {
            kinaare: Vec::new(),
            sankhya: n,
            dishit,
        }
    }

    pub fn kinara_jodo(&mut self, u: usize, v: usize, w: f64) {
        if u < self.sankhya && v < self.sankhya {
            self.kinaare.push((u, v, w));
        }
    }

    /// Convert to adjacency list
    pub fn suci_me(&self) -> SanlagnataSuci {
        let mut graph = SanlagnataSuci::naya(self.sankhya, self.dishit);

        for &(u, v, w) in &self.kinaare {
            graph.kinara_jodo(u, v, w);
        }

        graph
    }

    /// Sort edges by weight
    pub fn bhaar_se_krama(&mut self) {
        self.kinaare
            .sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(core::cmp::Ordering::Equal));
    }
}

/// Union-Find / Disjoint Set Union (संयोग खोज)
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct SanyogKhoj {
    pub janaka: Vec<usize>, // Parent array
    pub kram: Vec<usize>,   // Rank for union by rank
}

#[cfg(feature = "alloc")]
impl SanyogKhoj {
    pub fn naya(n: usize) -> Self {
        Self {
            janaka: (0..n).collect(),
            kram: vec![0; n],
        }
    }

    /// Find root with path compression
    pub fn khojo(&mut self, x: usize) -> usize {
        if self.janaka[x] != x {
            self.janaka[x] = self.khojo(self.janaka[x]);
        }
        self.janaka[x]
    }

    /// Union by rank
    pub fn sanyog(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.khojo(x);
        let root_y = self.khojo(y);

        if root_x == root_y {
            return false; // Already in same set
        }

        if self.kram[root_x] < self.kram[root_y] {
            self.janaka[root_x] = root_y;
        } else if self.kram[root_x] > self.kram[root_y] {
            self.janaka[root_y] = root_x;
        } else {
            self.janaka[root_y] = root_x;
            self.kram[root_x] += 1;
        }

        true
    }

    /// Check if in same component
    pub fn samaan_me(&mut self, x: usize, y: usize) -> bool {
        self.khojo(x) == self.khojo(y)
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_adjacency_list() {
        let mut graph = SanlagnataSuci::naya(5, false);
        graph.kinara_jodo(0, 1, 1.0);
        graph.kinara_jodo(0, 2, 2.0);
        graph.kinara_jodo(1, 2, 3.0);

        assert_eq!(graph.kinara_sankhya(), 3);
        assert!(graph.kinara_hai(0, 1));
        assert!(graph.kinara_hai(1, 0)); // Undirected
        assert!(!graph.kinara_hai(0, 3));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_adjacency_matrix() {
        let mut graph = SanlagnataMatrix::naya(4, true);
        graph.kinara_jodo(0, 1, 5.0);
        graph.kinara_jodo(1, 2, 3.0);

        assert!(graph.kinara_hai(0, 1));
        assert!(!graph.kinara_hai(1, 0)); // Directed
        assert_eq!(graph.kinara_bhaar(0, 1), Some(5.0));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_union_find() {
        let mut uf = SanyogKhoj::naya(5);

        uf.sanyog(0, 1);
        uf.sanyog(2, 3);
        uf.sanyog(1, 2);

        assert!(uf.samaan_me(0, 3));
        assert!(!uf.samaan_me(0, 4));
    }
}
