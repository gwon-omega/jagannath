//! # Khoj - Graph Search Algorithms (खोज)
//!
//! Graph traversal: BFS, DFS, topological sort.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::collections::VecDeque;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use super::prastutikaran::SanlagnataSuci;

/// BFS result
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct BfsPhala {
    pub doori: Vec<i64>,  // Distance from source (-1 = unreachable)
    pub janaka: Vec<i64>, // Parent in BFS tree (-1 = root/none)
    pub kram: Vec<usize>, // Visit order
}

/// Breadth-First Search (विस्तार पहले खोज)
#[cfg(feature = "alloc")]
pub fn vistar_pahle_khoj(graph: &SanlagnataSuci, srot: usize) -> BfsPhala {
    let n = graph.sankhya;
    let mut doori = vec![-1i64; n];
    let mut janaka = vec![-1i64; n];
    let mut kram = Vec::new();

    if srot >= n {
        return BfsPhala {
            doori,
            janaka,
            kram,
        };
    }

    let mut panki: VecDeque<usize> = VecDeque::new();
    doori[srot] = 0;
    panki.push_back(srot);

    while let Some(u) = panki.pop_front() {
        kram.push(u);

        for kinara in graph.padosi(u) {
            let v = kinara.lakshya;
            if doori[v] == -1 {
                doori[v] = doori[u] + 1;
                janaka[v] = u as i64;
                panki.push_back(v);
            }
        }
    }

    BfsPhala {
        doori,
        janaka,
        kram,
    }
}

/// DFS result
#[cfg(feature = "alloc")]
#[derive(Debug, Clone)]
pub struct DfsPhala {
    pub pravesh: Vec<usize>, // Discovery time
    pub nirgam: Vec<usize>,  // Finish time
    pub janaka: Vec<i64>,    // Parent
    pub kram: Vec<usize>,    // Visit order
}

/// Depth-First Search - recursive helper
#[cfg(feature = "alloc")]
fn dfs_dekho(
    graph: &SanlagnataSuci,
    u: usize,
    dekha: &mut [bool],
    pravesh: &mut [usize],
    nirgam: &mut [usize],
    janaka: &mut [i64],
    kram: &mut Vec<usize>,
    samay: &mut usize,
) {
    dekha[u] = true;
    *samay += 1;
    pravesh[u] = *samay;
    kram.push(u);

    for kinara in graph.padosi(u) {
        let v = kinara.lakshya;
        if !dekha[v] {
            janaka[v] = u as i64;
            dfs_dekho(graph, v, dekha, pravesh, nirgam, janaka, kram, samay);
        }
    }

    *samay += 1;
    nirgam[u] = *samay;
}

/// Depth-First Search (गहराई पहले खोज)
#[cfg(feature = "alloc")]
pub fn gaharai_pahle_khoj(graph: &SanlagnataSuci, srot: usize) -> DfsPhala {
    let n = graph.sankhya;
    let mut dekha = vec![false; n];
    let mut pravesh = vec![0; n];
    let mut nirgam = vec![0; n];
    let mut janaka = vec![-1i64; n];
    let mut kram = Vec::new();
    let mut samay = 0;

    if srot < n {
        dfs_dekho(
            graph,
            srot,
            &mut dekha,
            &mut pravesh,
            &mut nirgam,
            &mut janaka,
            &mut kram,
            &mut samay,
        );
    }

    DfsPhala {
        pravesh,
        nirgam,
        janaka,
        kram,
    }
}

/// Full DFS (visits all components)
#[cfg(feature = "alloc")]
pub fn sampurna_dfs(graph: &SanlagnataSuci) -> DfsPhala {
    let n = graph.sankhya;
    let mut dekha = vec![false; n];
    let mut pravesh = vec![0; n];
    let mut nirgam = vec![0; n];
    let mut janaka = vec![-1i64; n];
    let mut kram = Vec::new();
    let mut samay = 0;

    for u in 0..n {
        if !dekha[u] {
            dfs_dekho(
                graph,
                u,
                &mut dekha,
                &mut pravesh,
                &mut nirgam,
                &mut janaka,
                &mut kram,
                &mut samay,
            );
        }
    }

    DfsPhala {
        pravesh,
        nirgam,
        janaka,
        kram,
    }
}

/// Topological Sort (स्थलाकृतिक क्रम)
#[cfg(feature = "alloc")]
pub fn sthalakritik_kram(graph: &SanlagnataSuci) -> Option<Vec<usize>> {
    let n = graph.sankhya;

    // Calculate in-degrees
    let mut in_degree = vec![0usize; n];
    for u in 0..n {
        for kinara in graph.padosi(u) {
            in_degree[kinara.lakshya] += 1;
        }
    }

    // Kahn's algorithm
    let mut panki: VecDeque<usize> = VecDeque::new();
    for u in 0..n {
        if in_degree[u] == 0 {
            panki.push_back(u);
        }
    }

    let mut kram = Vec::with_capacity(n);

    while let Some(u) = panki.pop_front() {
        kram.push(u);

        for kinara in graph.padosi(u) {
            let v = kinara.lakshya;
            in_degree[v] -= 1;
            if in_degree[v] == 0 {
                panki.push_back(v);
            }
        }
    }

    if kram.len() == n {
        Some(kram)
    } else {
        None // Graph has cycle
    }
}

/// Check if graph has cycle (चक्र जाँच)
#[cfg(feature = "alloc")]
pub fn chakra_hai(graph: &SanlagnataSuci) -> bool {
    if !graph.dishit {
        // Undirected: use DFS
        let n = graph.sankhya;
        let mut dekha = vec![false; n];

        fn has_cycle_undirected(
            graph: &SanlagnataSuci,
            u: usize,
            parent: i64,
            dekha: &mut [bool],
        ) -> bool {
            dekha[u] = true;

            for kinara in graph.padosi(u) {
                let v = kinara.lakshya;
                if !dekha[v] {
                    if has_cycle_undirected(graph, v, u as i64, dekha) {
                        return true;
                    }
                } else if v as i64 != parent {
                    return true;
                }
            }

            false
        }

        for u in 0..n {
            if !dekha[u] {
                if has_cycle_undirected(graph, u, -1, &mut dekha) {
                    return true;
                }
            }
        }

        false
    } else {
        // Directed: use topological sort
        sthalakritik_kram(graph).is_none()
    }
}

/// Find connected components (संयुक्त घटक)
#[cfg(feature = "alloc")]
pub fn sanyukt_ghatak(graph: &SanlagnataSuci) -> Vec<Vec<usize>> {
    let n = graph.sankhya;
    let mut dekha = vec![false; n];
    let mut ghatak: Vec<Vec<usize>> = Vec::new();

    for u in 0..n {
        if !dekha[u] {
            let mut component = Vec::new();
            let mut panki: VecDeque<usize> = VecDeque::new();

            panki.push_back(u);
            dekha[u] = true;

            while let Some(v) = panki.pop_front() {
                component.push(v);

                for kinara in graph.padosi(v) {
                    let w = kinara.lakshya;
                    if !dekha[w] {
                        dekha[w] = true;
                        panki.push_back(w);
                    }
                }
            }

            ghatak.push(component);
        }
    }

    ghatak
}

/// Strongly connected components using Kosaraju's algorithm
#[cfg(feature = "alloc")]
pub fn dridh_sanyukt_ghatak(graph: &SanlagnataSuci) -> Vec<Vec<usize>> {
    let n = graph.sankhya;

    // First DFS to get finish order
    let mut dekha = vec![false; n];
    let mut stek: Vec<usize> = Vec::new();

    fn dfs1(graph: &SanlagnataSuci, u: usize, dekha: &mut [bool], stek: &mut Vec<usize>) {
        dekha[u] = true;
        for kinara in graph.padosi(u) {
            if !dekha[kinara.lakshya] {
                dfs1(graph, kinara.lakshya, dekha, stek);
            }
        }
        stek.push(u);
    }

    for u in 0..n {
        if !dekha[u] {
            dfs1(graph, u, &mut dekha, &mut stek);
        }
    }

    // Transpose graph
    let transposed = graph.parivart();

    // Second DFS in reverse finish order
    dekha.fill(false);
    let mut ghatak: Vec<Vec<usize>> = Vec::new();

    fn dfs2(graph: &SanlagnataSuci, u: usize, dekha: &mut [bool], component: &mut Vec<usize>) {
        dekha[u] = true;
        component.push(u);
        for kinara in graph.padosi(u) {
            if !dekha[kinara.lakshya] {
                dfs2(graph, kinara.lakshya, dekha, component);
            }
        }
    }

    while let Some(u) = stek.pop() {
        if !dekha[u] {
            let mut component = Vec::new();
            dfs2(&transposed, u, &mut dekha, &mut component);
            ghatak.push(component);
        }
    }

    ghatak
}

/// Find articulation points (joints) (संधि बिंदु)
#[cfg(feature = "alloc")]
pub fn sandhi_bindu(graph: &SanlagnataSuci) -> Vec<usize> {
    let n = graph.sankhya;
    let mut dekha = vec![false; n];
    let mut disc = vec![0; n];
    let mut low = vec![0; n];
    let mut janaka = vec![-1i64; n];
    let mut ap = vec![false; n];
    let mut samay = 0;

    fn dfs_ap(
        graph: &SanlagnataSuci,
        u: usize,
        dekha: &mut [bool],
        disc: &mut [usize],
        low: &mut [usize],
        janaka: &mut [i64],
        ap: &mut [bool],
        samay: &mut usize,
    ) {
        let mut bachche = 0;
        dekha[u] = true;
        *samay += 1;
        disc[u] = *samay;
        low[u] = *samay;

        for kinara in graph.padosi(u) {
            let v = kinara.lakshya;

            if !dekha[v] {
                bachche += 1;
                janaka[v] = u as i64;
                dfs_ap(graph, v, dekha, disc, low, janaka, ap, samay);

                low[u] = low[u].min(low[v]);

                // u is AP if:
                // 1. u is root and has >= 2 children
                // 2. u is not root and low[v] >= disc[u]
                if janaka[u] == -1 && bachche >= 2 {
                    ap[u] = true;
                }
                if janaka[u] != -1 && low[v] >= disc[u] {
                    ap[u] = true;
                }
            } else if v as i64 != janaka[u] {
                low[u] = low[u].min(disc[v]);
            }
        }
    }

    for u in 0..n {
        if !dekha[u] {
            dfs_ap(
                graph,
                u,
                &mut dekha,
                &mut disc,
                &mut low,
                &mut janaka,
                &mut ap,
                &mut samay,
            );
        }
    }

    ap.iter()
        .enumerate()
        .filter(|(_, &is_ap)| is_ap)
        .map(|(u, _)| u)
        .collect()
}

/// Find bridges (सेतु)
#[cfg(feature = "alloc")]
pub fn setu(graph: &SanlagnataSuci) -> Vec<(usize, usize)> {
    let n = graph.sankhya;
    let mut dekha = vec![false; n];
    let mut disc = vec![0; n];
    let mut low = vec![0; n];
    let mut janaka = vec![-1i64; n];
    let mut bridges: Vec<(usize, usize)> = Vec::new();
    let mut samay = 0;

    fn dfs_bridge(
        graph: &SanlagnataSuci,
        u: usize,
        dekha: &mut [bool],
        disc: &mut [usize],
        low: &mut [usize],
        janaka: &mut [i64],
        bridges: &mut Vec<(usize, usize)>,
        samay: &mut usize,
    ) {
        dekha[u] = true;
        *samay += 1;
        disc[u] = *samay;
        low[u] = *samay;

        for kinara in graph.padosi(u) {
            let v = kinara.lakshya;

            if !dekha[v] {
                janaka[v] = u as i64;
                dfs_bridge(graph, v, dekha, disc, low, janaka, bridges, samay);

                low[u] = low[u].min(low[v]);

                if low[v] > disc[u] {
                    bridges.push((u, v));
                }
            } else if v as i64 != janaka[u] {
                low[u] = low[u].min(disc[v]);
            }
        }
    }

    for u in 0..n {
        if !dekha[u] {
            dfs_bridge(
                graph,
                u,
                &mut dekha,
                &mut disc,
                &mut low,
                &mut janaka,
                &mut bridges,
                &mut samay,
            );
        }
    }

    bridges
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "alloc")]
    #[test]
    fn test_bfs() {
        let mut graph = SanlagnataSuci::naya(5, false);
        graph.kinara_jodo_abhar(0, 1);
        graph.kinara_jodo_abhar(0, 2);
        graph.kinara_jodo_abhar(1, 3);
        graph.kinara_jodo_abhar(2, 4);

        let result = vistar_pahle_khoj(&graph, 0);

        assert_eq!(result.doori[0], 0);
        assert_eq!(result.doori[1], 1);
        assert_eq!(result.doori[3], 2);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_dfs() {
        let mut graph = SanlagnataSuci::naya(4, true);
        graph.kinara_jodo_abhar(0, 1);
        graph.kinara_jodo_abhar(0, 2);
        graph.kinara_jodo_abhar(1, 3);

        let result = gaharai_pahle_khoj(&graph, 0);

        assert_eq!(result.kram[0], 0);
        assert!(result.pravesh[0] < result.nirgam[0]);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_topological_sort() {
        let mut graph = SanlagnataSuci::naya(4, true);
        graph.kinara_jodo_abhar(0, 1);
        graph.kinara_jodo_abhar(0, 2);
        graph.kinara_jodo_abhar(1, 3);
        graph.kinara_jodo_abhar(2, 3);

        let order = sthalakritik_kram(&graph).unwrap();

        assert_eq!(order[0], 0); // 0 must come first
        assert_eq!(*order.last().unwrap(), 3); // 3 must come last
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_connected_components() {
        let mut graph = SanlagnataSuci::naya(6, false);
        graph.kinara_jodo_abhar(0, 1);
        graph.kinara_jodo_abhar(1, 2);
        graph.kinara_jodo_abhar(3, 4);
        // 5 is isolated

        let components = sanyukt_ghatak(&graph);
        assert_eq!(components.len(), 3);
    }
}
