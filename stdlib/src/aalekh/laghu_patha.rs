//! # Laghu Patha - Shortest Path Algorithms (लघु पथ)
//!
//! Shortest path algorithms: Dijkstra, Bellman-Ford, Floyd-Warshall, A*.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::collections::BinaryHeap;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use super::prastutikaran::SanlagnataSuci;

/// Shortest path result
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct PathaPhala {
    pub doori: Vec<f64>,  // Distance from source
    pub janaka: Vec<i64>, // Parent in shortest path tree
}

impl PathaPhala {
    /// Reconstruct path from source to target
    #[cfg(feature = "alloc")]
    pub fn patha(&self, lakshya: usize) -> Vec<usize> {
        let mut path = Vec::new();
        let mut current = lakshya as i64;

        while current != -1 {
            path.push(current as usize);
            current = self.janaka[current as usize];
        }

        path.reverse();
        path
    }
}

/// Node for priority queue (min-heap)
#[derive(Copy, Clone, PartialEq)]
struct HeapNode {
    doori: f64,
    sheersh: usize,
}

impl Eq for HeapNode {}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        // Reverse for min-heap
        other
            .doori
            .partial_cmp(&self.doori)
            .unwrap_or(core::cmp::Ordering::Equal)
    }
}

/// Dijkstra's algorithm (डिजस्ट्रा)
/// Note: Assumes non-negative edge weights
#[cfg(feature = "alloc")]
pub fn dijkstra(graph: &SanlagnataSuci, srot: usize) -> PathaPhala {
    let n = graph.sankhya;
    let mut doori = vec![f64::INFINITY; n];
    let mut janaka = vec![-1i64; n];
    let mut dekha = vec![false; n];

    if srot >= n {
        return PathaPhala { doori, janaka };
    }

    let mut heap = BinaryHeap::new();
    doori[srot] = 0.0;
    heap.push(HeapNode {
        doori: 0.0,
        sheersh: srot,
    });

    while let Some(HeapNode {
        doori: d,
        sheersh: u,
    }) = heap.pop()
    {
        if dekha[u] {
            continue;
        }
        dekha[u] = true;

        for kinara in graph.padosi(u) {
            let v = kinara.lakshya;
            let naya_doori = d + kinara.bhaar;

            if naya_doori < doori[v] {
                doori[v] = naya_doori;
                janaka[v] = u as i64;
                heap.push(HeapNode {
                    doori: naya_doori,
                    sheersh: v,
                });
            }
        }
    }

    PathaPhala { doori, janaka }
}

/// Bellman-Ford algorithm (बेलमैन-फोर्ड)
/// Handles negative edge weights, detects negative cycles
#[cfg(feature = "alloc")]
pub fn bellman_ford(graph: &SanlagnataSuci, srot: usize) -> Option<PathaPhala> {
    let n = graph.sankhya;
    let mut doori = vec![f64::INFINITY; n];
    let mut janaka = vec![-1i64; n];

    if srot >= n {
        return Some(PathaPhala { doori, janaka });
    }

    doori[srot] = 0.0;

    // Relax all edges n-1 times
    for _ in 0..(n - 1) {
        for u in 0..n {
            if doori[u] == f64::INFINITY {
                continue;
            }

            for kinara in graph.padosi(u) {
                let v = kinara.lakshya;
                let naya_doori = doori[u] + kinara.bhaar;

                if naya_doori < doori[v] {
                    doori[v] = naya_doori;
                    janaka[v] = u as i64;
                }
            }
        }
    }

    // Check for negative cycle
    for u in 0..n {
        if doori[u] == f64::INFINITY {
            continue;
        }

        for kinara in graph.padosi(u) {
            let v = kinara.lakshya;
            if doori[u] + kinara.bhaar < doori[v] {
                return None; // Negative cycle detected
            }
        }
    }

    Some(PathaPhala { doori, janaka })
}

/// Floyd-Warshall all-pairs shortest path (फ्लोयड-वारशॉल)
#[cfg(feature = "alloc")]
pub fn floyd_warshall(graph: &SanlagnataSuci) -> (Vec<Vec<f64>>, Vec<Vec<i64>>) {
    let n = graph.sankhya;

    // Initialize distance matrix
    let mut doori: Vec<Vec<f64>> = vec![vec![f64::INFINITY; n]; n];
    let mut agla: Vec<Vec<i64>> = vec![vec![-1; n]; n];

    // Self loops have distance 0
    for i in 0..n {
        doori[i][i] = 0.0;
    }

    // Initialize with direct edges
    for u in 0..n {
        for kinara in graph.padosi(u) {
            let v = kinara.lakshya;
            doori[u][v] = kinara.bhaar;
            agla[u][v] = v as i64;
        }
    }

    // Dynamic programming
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if doori[i][k] != f64::INFINITY && doori[k][j] != f64::INFINITY {
                    let through_k = doori[i][k] + doori[k][j];
                    if through_k < doori[i][j] {
                        doori[i][j] = through_k;
                        agla[i][j] = agla[i][k];
                    }
                }
            }
        }
    }

    (doori, agla)
}

/// Reconstruct path from Floyd-Warshall result
#[cfg(feature = "alloc")]
pub fn fw_patha(agla: &[Vec<i64>], srot: usize, lakshya: usize) -> Vec<usize> {
    let mut path = Vec::new();

    if agla[srot][lakshya] == -1 {
        return path;
    }

    let mut current = srot;
    path.push(current);

    while current != lakshya {
        current = agla[current][lakshya] as usize;
        path.push(current);
    }

    path
}

/// A* search algorithm (ए* खोज)
#[cfg(feature = "alloc")]
pub fn a_tara<H>(
    graph: &SanlagnataSuci,
    srot: usize,
    lakshya: usize,
    heuristic: H,
) -> Option<(Vec<usize>, f64)>
where
    H: Fn(usize) -> f64,
{
    let n = graph.sankhya;

    if srot >= n || lakshya >= n {
        return None;
    }

    let mut g_score = vec![f64::INFINITY; n];
    let mut janaka = vec![-1i64; n];
    let mut dekha = vec![false; n];

    #[derive(Copy, Clone, PartialEq)]
    struct AStarNode {
        f_score: f64,
        g_score: f64,
        sheersh: usize,
    }

    impl Eq for AStarNode {}

    impl PartialOrd for AStarNode {
        fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for AStarNode {
        fn cmp(&self, other: &Self) -> core::cmp::Ordering {
            other
                .f_score
                .partial_cmp(&self.f_score)
                .unwrap_or(core::cmp::Ordering::Equal)
        }
    }

    let mut heap = BinaryHeap::new();
    g_score[srot] = 0.0;
    heap.push(AStarNode {
        f_score: heuristic(srot),
        g_score: 0.0,
        sheersh: srot,
    });

    while let Some(AStarNode {
        f_score: _,
        g_score: g,
        sheersh: u,
    }) = heap.pop()
    {
        if u == lakshya {
            // Reconstruct path
            let mut path = Vec::new();
            let mut current = lakshya as i64;

            while current != -1 {
                path.push(current as usize);
                current = janaka[current as usize];
            }

            path.reverse();
            return Some((path, g_score[lakshya]));
        }

        if dekha[u] {
            continue;
        }
        dekha[u] = true;

        for kinara in graph.padosi(u) {
            let v = kinara.lakshya;
            let tentative_g = g + kinara.bhaar;

            if tentative_g < g_score[v] {
                g_score[v] = tentative_g;
                janaka[v] = u as i64;

                let f = tentative_g + heuristic(v);
                heap.push(AStarNode {
                    f_score: f,
                    g_score: tentative_g,
                    sheersh: v,
                });
            }
        }
    }

    None // No path found
}

/// Bidirectional Dijkstra (द्विदिशीय डिजस्ट्रा)
#[cfg(feature = "alloc")]
pub fn dwidishi_dijkstra(
    graph: &SanlagnataSuci,
    srot: usize,
    lakshya: usize,
) -> Option<(Vec<usize>, f64)> {
    let n = graph.sankhya;

    if srot >= n || lakshya >= n {
        return None;
    }

    let reversed = graph.parivart();

    // Forward search
    let mut d_forward = vec![f64::INFINITY; n];
    let mut p_forward = vec![-1i64; n];
    let mut dekha_forward = vec![false; n];

    // Backward search
    let mut d_backward = vec![f64::INFINITY; n];
    let mut p_backward = vec![-1i64; n];
    let mut dekha_backward = vec![false; n];

    d_forward[srot] = 0.0;
    d_backward[lakshya] = 0.0;

    let mut forward_heap = BinaryHeap::new();
    let mut backward_heap = BinaryHeap::new();

    forward_heap.push(HeapNode {
        doori: 0.0,
        sheersh: srot,
    });
    backward_heap.push(HeapNode {
        doori: 0.0,
        sheersh: lakshya,
    });

    let mut best_dist = f64::INFINITY;
    let mut meeting_point = None;

    while !forward_heap.is_empty() || !backward_heap.is_empty() {
        // Forward step
        if let Some(HeapNode {
            doori: d,
            sheersh: u,
        }) = forward_heap.pop()
        {
            if !dekha_forward[u] {
                dekha_forward[u] = true;

                if dekha_backward[u] {
                    let total = d + d_backward[u];
                    if total < best_dist {
                        best_dist = total;
                        meeting_point = Some(u);
                    }
                }

                for kinara in graph.padosi(u) {
                    let v = kinara.lakshya;
                    let naya = d + kinara.bhaar;

                    if naya < d_forward[v] {
                        d_forward[v] = naya;
                        p_forward[v] = u as i64;
                        forward_heap.push(HeapNode {
                            doori: naya,
                            sheersh: v,
                        });
                    }
                }
            }
        }

        // Backward step
        if let Some(HeapNode {
            doori: d,
            sheersh: u,
        }) = backward_heap.pop()
        {
            if !dekha_backward[u] {
                dekha_backward[u] = true;

                if dekha_forward[u] {
                    let total = d + d_forward[u];
                    if total < best_dist {
                        best_dist = total;
                        meeting_point = Some(u);
                    }
                }

                for kinara in reversed.padosi(u) {
                    let v = kinara.lakshya;
                    let naya = d + kinara.bhaar;

                    if naya < d_backward[v] {
                        d_backward[v] = naya;
                        p_backward[v] = u as i64;
                        backward_heap.push(HeapNode {
                            doori: naya,
                            sheersh: v,
                        });
                    }
                }
            }
        }

        // Early termination check
        if best_dist != f64::INFINITY {
            let min_forward = forward_heap
                .peek()
                .map(|n| n.doori)
                .unwrap_or(f64::INFINITY);
            let min_backward = backward_heap
                .peek()
                .map(|n| n.doori)
                .unwrap_or(f64::INFINITY);

            if min_forward + min_backward >= best_dist {
                break;
            }
        }
    }

    meeting_point.map(|m| {
        // Reconstruct path
        let mut path = Vec::new();

        // Forward part
        let mut current = m as i64;
        let mut forward_part = Vec::new();
        while current != -1 {
            forward_part.push(current as usize);
            current = p_forward[current as usize];
        }
        forward_part.reverse();
        path.extend(forward_part);

        // Backward part (skip meeting point)
        current = p_backward[m];
        while current != -1 {
            path.push(current as usize);
            current = p_backward[current as usize];
        }

        (path, best_dist)
    })
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_dijkstra() {
        let mut graph = SanlagnataSuci::naya(5, true);
        graph.kinara_jodo(0, 1, 4.0);
        graph.kinara_jodo(0, 2, 1.0);
        graph.kinara_jodo(2, 1, 2.0);
        graph.kinara_jodo(1, 3, 1.0);
        graph.kinara_jodo(2, 3, 5.0);

        let result = dijkstra(&graph, 0);

        assert_eq!(result.doori[0], 0.0);
        assert_eq!(result.doori[1], 3.0); // 0 -> 2 -> 1
        assert_eq!(result.doori[3], 4.0); // 0 -> 2 -> 1 -> 3
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_bellman_ford() {
        let mut graph = SanlagnataSuci::naya(4, true);
        graph.kinara_jodo(0, 1, 1.0);
        graph.kinara_jodo(1, 2, 2.0);
        graph.kinara_jodo(0, 2, 4.0);

        let result = bellman_ford(&graph, 0).unwrap();

        assert_eq!(result.doori[2], 3.0); // 0 -> 1 -> 2
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_floyd_warshall() {
        let mut graph = SanlagnataSuci::naya(3, true);
        graph.kinara_jodo(0, 1, 1.0);
        graph.kinara_jodo(1, 2, 1.0);
        graph.kinara_jodo(0, 2, 3.0);

        let (doori, _) = floyd_warshall(&graph);

        assert_eq!(doori[0][2], 2.0); // 0 -> 1 -> 2
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_a_star() {
        let mut graph = SanlagnataSuci::naya(4, false);
        graph.kinara_jodo(0, 1, 1.0);
        graph.kinara_jodo(1, 2, 1.0);
        graph.kinara_jodo(2, 3, 1.0);
        graph.kinara_jodo(0, 3, 5.0);

        let result = a_tara(&graph, 0, 3, |_| 0.0);

        assert!(result.is_some());
        let (path, dist) = result.unwrap();
        assert_eq!(dist, 3.0);
        assert_eq!(path, vec![0, 1, 2, 3]);
    }
}
