//! # Pravriti - Graphs (प्रवृत्ति)
//!
//! Graph data structures and algorithms.

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::collections::VecDeque;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

/// Edge representation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Kinara {
    /// Destination vertex
    pub lakshya: usize,
    /// Weight
    pub bhaar: f64,
}

impl Kinara {
    pub fn naya(lakshya: usize, bhaar: f64) -> Self {
        Self { lakshya, bhaar }
    }

    pub fn abharit(lakshya: usize) -> Self {
        Self {
            lakshya,
            bhaar: 1.0,
        }
    }
}

/// Graph representation (adjacency list)
#[cfg(feature = "alloc")]
pub struct Graph {
    /// Adjacency list
    padosi: Vec<Vec<Kinara>>,
    /// Is directed
    nirdeshit: bool,
}

#[cfg(feature = "alloc")]
impl Graph {
    /// Create directed graph
    pub fn nirdeshit(sheersh: usize) -> Self {
        Self {
            padosi: vec![Vec::new(); sheersh],
            nirdeshit: true,
        }
    }

    /// Create undirected graph
    pub fn anirdeshit(sheersh: usize) -> Self {
        Self {
            padosi: vec![Vec::new(); sheersh],
            nirdeshit: false,
        }
    }

    /// Number of vertices
    pub fn sheersh_sankhya(&self) -> usize {
        self.padosi.len()
    }

    /// Add edge
    pub fn kinara_jodo(&mut self, srot: usize, lakshya: usize, bhaar: f64) {
        if srot < self.padosi.len() && lakshya < self.padosi.len() {
            self.padosi[srot].push(Kinara::naya(lakshya, bhaar));
            if !self.nirdeshit {
                self.padosi[lakshya].push(Kinara::naya(srot, bhaar));
            }
        }
    }

    /// Add unweighted edge
    pub fn sadha_kinara(&mut self, srot: usize, lakshya: usize) {
        self.kinara_jodo(srot, lakshya, 1.0);
    }

    /// Get neighbors
    pub fn padosiyan(&self, v: usize) -> &[Kinara] {
        &self.padosi[v]
    }

    /// BFS traversal
    pub fn vistrit_pahle(&self, shuru: usize) -> Vec<usize> {
        let mut visited = vec![false; self.padosi.len()];
        let mut result = Vec::new();
        let mut queue = VecDeque::new();

        visited[shuru] = true;
        queue.push_back(shuru);

        while let Some(v) = queue.pop_front() {
            result.push(v);

            for edge in &self.padosi[v] {
                if !visited[edge.lakshya] {
                    visited[edge.lakshya] = true;
                    queue.push_back(edge.lakshya);
                }
            }
        }

        result
    }

    /// DFS traversal
    pub fn gahri_pahle(&self, shuru: usize) -> Vec<usize> {
        let mut visited = vec![false; self.padosi.len()];
        let mut result = Vec::new();
        self.dfs_inner(shuru, &mut visited, &mut result);
        result
    }

    fn dfs_inner(&self, v: usize, visited: &mut Vec<bool>, result: &mut Vec<usize>) {
        visited[v] = true;
        result.push(v);

        for edge in &self.padosi[v] {
            if !visited[edge.lakshya] {
                self.dfs_inner(edge.lakshya, visited, result);
            }
        }
    }

    /// Shortest path using Dijkstra
    pub fn laghu_marg(&self, shuru: usize) -> (Vec<f64>, Vec<Option<usize>>) {
        let n = self.padosi.len();
        let mut dist = vec![f64::INFINITY; n];
        let mut parent = vec![None; n];
        let mut visited = vec![false; n];

        dist[shuru] = 0.0;

        for _ in 0..n {
            // Find minimum unvisited
            let mut min_v = None;
            let mut min_dist = f64::INFINITY;

            for v in 0..n {
                if !visited[v] && dist[v] < min_dist {
                    min_dist = dist[v];
                    min_v = Some(v);
                }
            }

            let u = match min_v {
                Some(v) => v,
                None => break,
            };

            visited[u] = true;

            for edge in &self.padosi[u] {
                let new_dist = dist[u] + edge.bhaar;
                if new_dist < dist[edge.lakshya] {
                    dist[edge.lakshya] = new_dist;
                    parent[edge.lakshya] = Some(u);
                }
            }
        }

        (dist, parent)
    }

    /// Reconstruct path
    pub fn marg_banao(&self, parent: &[Option<usize>], lakshya: usize) -> Vec<usize> {
        let mut path = Vec::new();
        let mut current = Some(lakshya);

        while let Some(v) = current {
            path.push(v);
            current = parent[v];
        }

        path.reverse();
        path
    }

    /// Topological sort (for DAG)
    pub fn kramik_vibhajan(&self) -> Option<Vec<usize>> {
        if !self.nirdeshit {
            return None;
        }

        let n = self.padosi.len();
        let mut in_degree = vec![0usize; n];

        // Calculate in-degrees
        for edges in &self.padosi {
            for edge in edges {
                in_degree[edge.lakshya] += 1;
            }
        }

        // Start with zero in-degree vertices
        let mut queue: VecDeque<usize> = in_degree
            .iter()
            .enumerate()
            .filter(|(_, &d)| d == 0)
            .map(|(i, _)| i)
            .collect();

        let mut result = Vec::new();

        while let Some(v) = queue.pop_front() {
            result.push(v);

            for edge in &self.padosi[v] {
                in_degree[edge.lakshya] -= 1;
                if in_degree[edge.lakshya] == 0 {
                    queue.push_back(edge.lakshya);
                }
            }
        }

        if result.len() == n {
            Some(result)
        } else {
            None // Cycle detected
        }
    }

    /// Check if graph has cycle
    pub fn chakra_hai(&self) -> bool {
        let n = self.padosi.len();
        let mut visited = vec![false; n];
        let mut rec_stack = vec![false; n];

        for v in 0..n {
            if self.chakra_dfs(v, &mut visited, &mut rec_stack) {
                return true;
            }
        }

        false
    }

    fn chakra_dfs(&self, v: usize, visited: &mut Vec<bool>, rec_stack: &mut Vec<bool>) -> bool {
        if !visited[v] {
            visited[v] = true;
            rec_stack[v] = true;

            for edge in &self.padosi[v] {
                if !visited[edge.lakshya] {
                    if self.chakra_dfs(edge.lakshya, visited, rec_stack) {
                        return true;
                    }
                } else if rec_stack[edge.lakshya] {
                    return true;
                }
            }
        }

        rec_stack[v] = false;
        false
    }

    /// Connected components (for undirected)
    pub fn sambandhit_ghataka(&self) -> Vec<Vec<usize>> {
        let n = self.padosi.len();
        let mut visited = vec![false; n];
        let mut components = Vec::new();

        for v in 0..n {
            if !visited[v] {
                let mut component = Vec::new();
                self.dfs_inner(v, &mut visited, &mut component);
                components.push(component);
            }
        }

        components
    }

    /// Check if bipartite
    pub fn dwidali_hai(&self) -> bool {
        let n = self.padosi.len();
        let mut color = vec![-1i8; n];

        for start in 0..n {
            if color[start] == -1 {
                let mut queue = VecDeque::new();
                queue.push_back(start);
                color[start] = 0;

                while let Some(v) = queue.pop_front() {
                    for edge in &self.padosi[v] {
                        let u = edge.lakshya;
                        if color[u] == -1 {
                            color[u] = 1 - color[v];
                            queue.push_back(u);
                        } else if color[u] == color[v] {
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}

// ============================================================================
// UNION-FIND (संयोजन-खोज)
// ============================================================================

/// Union-Find / Disjoint Set Union
#[cfg(feature = "alloc")]
pub struct SanyojanKhoj {
    parent: Vec<usize>,
    rank: Vec<usize>,
}

#[cfg(feature = "alloc")]
impl SanyojanKhoj {
    /// Create with n elements
    pub fn naya(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }

    /// Find root with path compression
    pub fn khojo(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.khojo(self.parent[x]);
        }
        self.parent[x]
    }

    /// Union by rank
    pub fn jodo(&mut self, x: usize, y: usize) -> bool {
        let rx = self.khojo(x);
        let ry = self.khojo(y);

        if rx == ry {
            return false; // Already in same set
        }

        if self.rank[rx] < self.rank[ry] {
            self.parent[rx] = ry;
        } else if self.rank[rx] > self.rank[ry] {
            self.parent[ry] = rx;
        } else {
            self.parent[ry] = rx;
            self.rank[rx] += 1;
        }

        true
    }

    /// Check if same set
    pub fn samaan(&mut self, x: usize, y: usize) -> bool {
        self.khojo(x) == self.khojo(y)
    }

    /// Count distinct sets
    pub fn ginti(&mut self) -> usize {
        let n = self.parent.len();
        let mut count = 0;
        for i in 0..n {
            if self.khojo(i) == i {
                count += 1;
            }
        }
        count
    }
}

// ============================================================================
// MINIMUM SPANNING TREE (न्यूनतम फैलाव वृक्ष)
// ============================================================================

/// Kruskal's MST result
#[cfg(feature = "alloc")]
pub struct NyunatamVriksha {
    /// Edges in MST
    pub kinare: Vec<(usize, usize, f64)>,
    /// Total weight
    pub kul_bhaar: f64,
}

#[cfg(feature = "alloc")]
impl Graph {
    /// Kruskal's MST algorithm
    pub fn kruskal_mst(&self) -> NyunatamVriksha {
        let n = self.sheersh_sankhya();
        let mut edges: Vec<(usize, usize, f64)> = Vec::new();

        // Collect all edges
        for u in 0..n {
            for edge in &self.padosi[u] {
                if u < edge.lakshya || self.nirdeshit {
                    edges.push((u, edge.lakshya, edge.bhaar));
                }
            }
        }

        // Sort by weight
        edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(core::cmp::Ordering::Equal));

        let mut uf = SanyojanKhoj::naya(n);
        let mut mst_edges = Vec::new();
        let mut total = 0.0;

        for (u, v, w) in edges {
            if uf.jodo(u, v) {
                mst_edges.push((u, v, w));
                total += w;

                if mst_edges.len() == n - 1 {
                    break;
                }
            }
        }

        NyunatamVriksha {
            kinare: mst_edges,
            kul_bhaar: total,
        }
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
    fn test_graph_bfs() {
        let mut g = Graph::anirdeshit(5);
        g.sadha_kinara(0, 1);
        g.sadha_kinara(0, 2);
        g.sadha_kinara(1, 3);
        g.sadha_kinara(2, 4);

        let bfs = g.vistrit_pahle(0);
        assert_eq!(bfs[0], 0);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_graph_dfs() {
        let mut g = Graph::anirdeshit(5);
        g.sadha_kinara(0, 1);
        g.sadha_kinara(0, 2);
        g.sadha_kinara(1, 3);
        g.sadha_kinara(2, 4);

        let dfs = g.gahri_pahle(0);
        assert_eq!(dfs[0], 0);
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_dijkstra() {
        let mut g = Graph::nirdeshit(4);
        g.kinara_jodo(0, 1, 1.0);
        g.kinara_jodo(0, 2, 4.0);
        g.kinara_jodo(1, 2, 2.0);
        g.kinara_jodo(1, 3, 5.0);
        g.kinara_jodo(2, 3, 1.0);

        let (dist, _) = g.laghu_marg(0);
        assert_eq!(dist[3], 4.0); // 0->1->2->3
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_topological_sort() {
        let mut g = Graph::nirdeshit(4);
        g.sadha_kinara(0, 1);
        g.sadha_kinara(0, 2);
        g.sadha_kinara(1, 3);
        g.sadha_kinara(2, 3);

        let topo = g.kramik_vibhajan();
        assert!(topo.is_some());
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_union_find() {
        let mut uf = SanyojanKhoj::naya(5);
        uf.jodo(0, 1);
        uf.jodo(2, 3);

        assert!(uf.samaan(0, 1));
        assert!(!uf.samaan(0, 2));
        assert_eq!(uf.ginti(), 3);

        uf.jodo(1, 2);
        assert!(uf.samaan(0, 3));
    }

    #[cfg(feature = "alloc")]
    #[test]
    fn test_kruskal() {
        let mut g = Graph::anirdeshit(4);
        g.kinara_jodo(0, 1, 1.0);
        g.kinara_jodo(0, 2, 4.0);
        g.kinara_jodo(1, 2, 2.0);
        g.kinara_jodo(1, 3, 5.0);
        g.kinara_jodo(2, 3, 3.0);

        let mst = g.kruskal_mst();
        assert_eq!(mst.kinare.len(), 3);
        assert_eq!(mst.kul_bhaar, 6.0);
    }
}
