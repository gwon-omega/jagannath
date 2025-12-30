//! # Vriksha - Spanning Tree Algorithms (वृक्ष)
//!
//! Minimum spanning tree: Kruskal, Prim.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::collections::BinaryHeap;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use super::prastutikaran::{KinaraSuci, SanlagnataSuci, SanyogKhoj};

/// MST result
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct MstPhala {
    pub kinaare: Vec<(usize, usize, f64)>, // Edges in MST
    pub kul_bhaar: f64,                    // Total weight
}

/// Kruskal's algorithm (क्रुस्कल)
#[cfg(feature = "alloc")]
pub fn kruskal(graph: &SanlagnataSuci) -> MstPhala {
    let n = graph.sankhya;
    let mut edges = graph.kinara_suchi();

    // Sort edges by weight
    edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(core::cmp::Ordering::Equal));

    let mut uf = SanyogKhoj::naya(n);
    let mut mst: Vec<(usize, usize, f64)> = Vec::new();
    let mut kul_bhaar = 0.0;

    for (u, v, w) in edges {
        if uf.sanyog(u, v) {
            mst.push((u, v, w));
            kul_bhaar += w;

            if mst.len() == n - 1 {
                break;
            }
        }
    }

    MstPhala {
        kinaare: mst,
        kul_bhaar,
    }
}

/// Prim's algorithm (प्रिम)
#[cfg(feature = "alloc")]
pub fn prim(graph: &SanlagnataSuci) -> MstPhala {
    let n = graph.sankhya;

    if n == 0 {
        return MstPhala {
            kinaare: Vec::new(),
            kul_bhaar: 0.0,
        };
    }

    #[derive(Copy, Clone, PartialEq)]
    struct PrimNode {
        bhaar: f64,
        sheersh: usize,
        janaka: usize,
    }

    impl Eq for PrimNode {}

    impl PartialOrd for PrimNode {
        fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for PrimNode {
        fn cmp(&self, other: &Self) -> core::cmp::Ordering {
            other
                .bhaar
                .partial_cmp(&self.bhaar)
                .unwrap_or(core::cmp::Ordering::Equal)
        }
    }

    let mut in_mst = vec![false; n];
    let mut mst: Vec<(usize, usize, f64)> = Vec::new();
    let mut kul_bhaar = 0.0;

    let mut heap = BinaryHeap::new();
    heap.push(PrimNode {
        bhaar: 0.0,
        sheersh: 0,
        janaka: 0,
    });

    while let Some(PrimNode {
        bhaar,
        sheersh: u,
        janaka,
    }) = heap.pop()
    {
        if in_mst[u] {
            continue;
        }

        in_mst[u] = true;

        if u != janaka {
            mst.push((janaka, u, bhaar));
            kul_bhaar += bhaar;
        }

        for kinara in graph.padosi(u) {
            let v = kinara.lakshya;
            if !in_mst[v] {
                heap.push(PrimNode {
                    bhaar: kinara.bhaar,
                    sheersh: v,
                    janaka: u,
                });
            }
        }
    }

    MstPhala {
        kinaare: mst,
        kul_bhaar,
    }
}

/// Borůvka's algorithm (बोरुव्का)
#[cfg(feature = "alloc")]
pub fn boruvka(graph: &SanlagnataSuci) -> MstPhala {
    let n = graph.sankhya;
    let edges = graph.kinara_suchi();

    let mut uf = SanyogKhoj::naya(n);
    let mut mst: Vec<(usize, usize, f64)> = Vec::new();
    let mut kul_bhaar = 0.0;
    let mut num_components = n;

    while num_components > 1 {
        // Find minimum edge for each component
        let mut cheapest: Vec<Option<(usize, usize, f64)>> = vec![None; n];

        for &(u, v, w) in &edges {
            let comp_u = uf.khojo(u);
            let comp_v = uf.khojo(v);

            if comp_u != comp_v {
                // Check if this edge is cheaper for component u
                let update_u = match cheapest[comp_u] {
                    None => true,
                    Some((_, _, cw)) => w < cw,
                };
                if update_u {
                    cheapest[comp_u] = Some((u, v, w));
                }

                // Check if this edge is cheaper for component v
                let update_v = match cheapest[comp_v] {
                    None => true,
                    Some((_, _, cw)) => w < cw,
                };
                if update_v {
                    cheapest[comp_v] = Some((u, v, w));
                }
            }
        }

        // Add cheapest edges to MST
        for i in 0..n {
            if let Some((u, v, w)) = cheapest[i] {
                if uf.sanyog(u, v) {
                    mst.push((u, v, w));
                    kul_bhaar += w;
                    num_components -= 1;
                }
            }
        }

        // No progress means disconnected graph
        if cheapest.iter().all(|c| c.is_none()) {
            break;
        }
    }

    MstPhala {
        kinaare: mst,
        kul_bhaar,
    }
}

/// Check if graph is connected
#[cfg(feature = "alloc")]
pub fn sanyukt_hai(graph: &SanlagnataSuci) -> bool {
    let n = graph.sankhya;
    if n == 0 {
        return true;
    }

    let mut dekha = vec![false; n];
    let mut stek = vec![0usize];
    dekha[0] = true;
    let mut count = 1;

    while let Some(u) = stek.pop() {
        for kinara in graph.padosi(u) {
            let v = kinara.lakshya;
            if !dekha[v] {
                dekha[v] = true;
                count += 1;
                stek.push(v);
            }
        }
    }

    count == n
}

/// Find second-best MST
#[cfg(feature = "alloc")]
pub fn dwitiya_mst(graph: &SanlagnataSuci) -> Option<MstPhala> {
    let mst = kruskal(graph);

    if mst.kinaare.len() < graph.sankhya - 1 {
        return None; // Graph not connected
    }

    let mut best_second = None;
    let mut best_cost = f64::INFINITY;

    // Try removing each MST edge and finding new MST
    for i in 0..mst.kinaare.len() {
        // Create graph without this edge
        let (eu, ev, ew) = mst.kinaare[i];

        let mut temp_graph = SanlagnataSuci::naya(graph.sankhya, graph.dishit);

        for u in 0..graph.sankhya {
            for kinara in graph.padosi(u) {
                let v = kinara.lakshya;
                let w = kinara.bhaar;

                // Skip the removed edge
                if (u == eu && v == ev) || (!graph.dishit && u == ev && v == eu) {
                    continue;
                }

                if graph.dishit || u < v {
                    temp_graph.kinara_jodo(u, v, w);
                }
            }
        }

        let new_mst = kruskal(&temp_graph);

        if new_mst.kinaare.len() == graph.sankhya - 1 {
            if new_mst.kul_bhaar < best_cost {
                best_cost = new_mst.kul_bhaar;
                best_second = Some(new_mst);
            }
        }
    }

    best_second
}

/// Steiner tree approximation (2-approximation)
/// Finds minimum tree connecting specified terminals
#[cfg(feature = "alloc")]
pub fn steiner_vriksha(graph: &SanlagnataSuci, terminals: &[usize]) -> MstPhala {
    use super::laghu_patha::dijkstra;

    if terminals.is_empty() {
        return MstPhala {
            kinaare: Vec::new(),
            kul_bhaar: 0.0,
        };
    }

    let t = terminals.len();

    // Build metric closure on terminals
    let mut metric_graph = SanlagnataSuci::naya(t, false);
    let mut paths: Vec<Vec<(usize, usize, f64)>> = Vec::new();

    for i in 0..t {
        let result = dijkstra(graph, terminals[i]);

        for j in (i + 1)..t {
            let dist = result.doori[terminals[j]];
            if dist < f64::INFINITY {
                metric_graph.kinara_jodo(i, j, dist);
            }
        }
    }

    // Find MST on metric closure
    let metric_mst = kruskal(&metric_graph);

    // For simplicity, return the metric MST
    // (Full implementation would reconstruct paths)
    let kinaare: Vec<(usize, usize, f64)> = metric_mst
        .kinaare
        .iter()
        .map(|&(i, j, w)| (terminals[i], terminals[j], w))
        .collect();

    MstPhala {
        kinaare,
        kul_bhaar: metric_mst.kul_bhaar,
    }
}

/// Maximum spanning tree (using negated weights)
#[cfg(feature = "alloc")]
pub fn adhikatam_vriksha(graph: &SanlagnataSuci) -> MstPhala {
    // Create graph with negated weights
    let mut neg_graph = SanlagnataSuci::naya(graph.sankhya, graph.dishit);

    for u in 0..graph.sankhya {
        for kinara in graph.padosi(u) {
            if graph.dishit || u < kinara.lakshya {
                neg_graph.kinara_jodo(u, kinara.lakshya, -kinara.bhaar);
            }
        }
    }

    let mut result = kruskal(&neg_graph);

    // Negate back
    result.kul_bhaar = -result.kul_bhaar;
    for edge in &mut result.kinaare {
        edge.2 = -edge.2;
    }

    result
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    fn create_test_graph() -> SanlagnataSuci {
        let mut graph = SanlagnataSuci::naya(4, false);
        graph.kinara_jodo(0, 1, 1.0);
        graph.kinara_jodo(1, 2, 2.0);
        graph.kinara_jodo(2, 3, 3.0);
        graph.kinara_jodo(3, 0, 4.0);
        graph.kinara_jodo(0, 2, 5.0);
        graph
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_kruskal() {
        let graph = create_test_graph();
        let mst = kruskal(&graph);

        assert_eq!(mst.kinaare.len(), 3);
        assert_eq!(mst.kul_bhaar, 6.0); // 1 + 2 + 3
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_prim() {
        let graph = create_test_graph();
        let mst = prim(&graph);

        assert_eq!(mst.kinaare.len(), 3);
        assert_eq!(mst.kul_bhaar, 6.0);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_boruvka() {
        let graph = create_test_graph();
        let mst = boruvka(&graph);

        assert_eq!(mst.kinaare.len(), 3);
        assert_eq!(mst.kul_bhaar, 6.0);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_connected() {
        let graph = create_test_graph();
        assert!(sanyukt_hai(&graph));

        let mut disconnected = SanlagnataSuci::naya(4, false);
        disconnected.kinara_jodo(0, 1, 1.0);
        // 2 and 3 are isolated
        assert!(!sanyukt_hai(&disconnected));
    }
}
