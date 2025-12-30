//! # Lekhāgaṇita - Graph Algorithms (लेखागणित)
//!
//! Graph data structures and algorithms.
//!
//! > **"सर्वं सम्बन्धमयम्"**
//! > *"Everything is connected"*
//!
//! ## Data Structures
//!
//! - [`Lekha`] - Graph (लेखा)
//! - [`Kinara`] - Edge (किनारा)
//!
//! ## Algorithms
//!
//! - BFS, DFS traversal
//! - Dijkstra's shortest path
//! - Topological sort
//! - Minimum spanning tree (Kruskal, Prim)
//! - Connected components

use core::cmp::Ordering;

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::collections::{BinaryHeap, VecDeque};
#[cfg(feature = "alloc")]
use alloc::vec;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

// ============================================================================
// GRAPH REPRESENTATION (लेखा प्रतिनिधित्व)
// ============================================================================

/// Edge in a graph (किनारा)
///
/// # Etymology
/// किनारा (kinārā) = edge, border
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg(feature = "alloc")]
pub struct Kinara {
    /// Source vertex (स्रोत)
    pub shrota: usize,
    /// Destination vertex (गन्तव्य)
    pub gantavya: usize,
    /// Weight (भार)
    pub bhara: f64,
}

#[cfg(feature = "alloc")]
impl Kinara {
    pub fn new(shrota: usize, gantavya: usize, bhara: f64) -> Self {
        Self {
            shrota,
            gantavya,
            bhara,
        }
    }

    pub fn unweighted(shrota: usize, gantavya: usize) -> Self {
        Self {
            shrota,
            gantavya,
            bhara: 1.0,
        }
    }
}

/// Graph using adjacency list (लेखा)
///
/// # Etymology
/// लेखा (lekhā) = graph, diagram, writing
#[derive(Debug, Clone)]
#[cfg(feature = "alloc")]
pub struct Lekha {
    /// Number of vertices (शीर्ष संख्या)
    shirsha_sankhya: usize,
    /// Adjacency list (आसन्न सूची)
    asanna_suchi: Vec<Vec<(usize, f64)>>,
    /// Directed graph? (दिशात्मक?)
    dishatmaka: bool,
}

#[cfg(feature = "alloc")]
impl Lekha {
    /// Create new graph (नव लेखा)
    pub fn nava(shirsha_sankhya: usize, dishatmaka: bool) -> Self {
        Self {
            shirsha_sankhya,
            asanna_suchi: vec![Vec::new(); shirsha_sankhya],
            dishatmaka,
        }
    }

    /// Create undirected graph (अदिशात्मक लेखा)
    pub fn adishatmaka(shirsha_sankhya: usize) -> Self {
        Self::nava(shirsha_sankhya, false)
    }

    /// Create directed graph (दिशात्मक लेखा)
    pub fn dishatmaka(shirsha_sankhya: usize) -> Self {
        Self::nava(shirsha_sankhya, true)
    }

    /// Number of vertices (शीर्ष संख्या)
    pub fn shirsha_sankhya(&self) -> usize {
        self.shirsha_sankhya
    }

    /// Add edge (किनारा जोड़ना)
    pub fn kinara_jodana(&mut self, shrota: usize, gantavya: usize, bhara: f64) {
        self.asanna_suchi[shrota].push((gantavya, bhara));
        if !self.dishatmaka {
            self.asanna_suchi[gantavya].push((shrota, bhara));
        }
    }

    /// Add unweighted edge (अभारित किनारा)
    pub fn abharit_kinara(&mut self, shrota: usize, gantavya: usize) {
        self.kinara_jodana(shrota, gantavya, 1.0);
    }

    /// Get neighbors of vertex (पड़ोसी)
    pub fn padosi(&self, shirsha: usize) -> &[(usize, f64)] {
        &self.asanna_suchi[shirsha]
    }

    /// Check if edge exists (किनारा अस्तित्व)
    pub fn kinara_astitva(&self, shrota: usize, gantavya: usize) -> bool {
        self.asanna_suchi[shrota]
            .iter()
            .any(|(v, _)| *v == gantavya)
    }
}

// ============================================================================
// TRAVERSAL ALGORITHMS (भ्रमण अल्गोरिदम)
// ============================================================================

/// Breadth-First Search (विस्तार-प्रथम अन्वेषण)
///
/// Visits all vertices level by level.
///
/// # Etymology
/// विस्तार (vistāra) = breadth, expansion
/// प्रथम (prathama) = first
///
/// # Complexity
/// - Time: O(V + E)
/// - Space: O(V)
#[cfg(feature = "alloc")]
pub fn vistar_prathama_anveshan(lekha: &Lekha, arambha: usize) -> Vec<usize> {
    let n = lekha.shirsha_sankhya();
    let mut darshita = vec![false; n];
    let mut krama = Vec::new();
    let mut pankti: VecDeque<usize> = VecDeque::new();

    darshita[arambha] = true;
    pankti.push_back(arambha);

    while let Some(shirsha) = pankti.pop_front() {
        krama.push(shirsha);

        for &(padosi, _) in lekha.padosi(shirsha) {
            if !darshita[padosi] {
                darshita[padosi] = true;
                pankti.push_back(padosi);
            }
        }
    }

    krama
}

/// Depth-First Search (गहराई-प्रथम अन्वेषण)
///
/// Visits vertices by going as deep as possible.
///
/// # Etymology
/// गहराई (gaharāī) = depth
///
/// # Complexity
/// - Time: O(V + E)
/// - Space: O(V)
#[cfg(feature = "alloc")]
pub fn gaharai_prathama_anveshan(lekha: &Lekha, arambha: usize) -> Vec<usize> {
    let n = lekha.shirsha_sankhya();
    let mut darshita = vec![false; n];
    let mut krama = Vec::new();

    dfs_helper(lekha, arambha, &mut darshita, &mut krama);
    krama
}

#[cfg(feature = "alloc")]
fn dfs_helper(lekha: &Lekha, shirsha: usize, darshita: &mut [bool], krama: &mut Vec<usize>) {
    darshita[shirsha] = true;
    krama.push(shirsha);

    for &(padosi, _) in lekha.padosi(shirsha) {
        if !darshita[padosi] {
            dfs_helper(lekha, padosi, darshita, krama);
        }
    }
}

/// Iterative DFS (पुनरावृत्त गहराई अन्वेषण)
#[cfg(feature = "alloc")]
pub fn gaharai_prathama_punaravrtta(lekha: &Lekha, arambha: usize) -> Vec<usize> {
    let n = lekha.shirsha_sankhya();
    let mut darshita = vec![false; n];
    let mut krama = Vec::new();
    let mut stambha: Vec<usize> = vec![arambha];

    while let Some(shirsha) = stambha.pop() {
        if darshita[shirsha] {
            continue;
        }
        darshita[shirsha] = true;
        krama.push(shirsha);

        for &(padosi, _) in lekha.padosi(shirsha).iter().rev() {
            if !darshita[padosi] {
                stambha.push(padosi);
            }
        }
    }

    krama
}

// ============================================================================
// SHORTEST PATH (लघुतम मार्ग)
// ============================================================================

/// State for Dijkstra's algorithm
#[derive(Clone, Copy)]
#[cfg(feature = "alloc")]
struct DijkstraState {
    duri: f64,
    shirsha: usize,
}

#[cfg(feature = "alloc")]
impl Eq for DijkstraState {}

#[cfg(feature = "alloc")]
impl PartialEq for DijkstraState {
    fn eq(&self, other: &Self) -> bool {
        self.shirsha == other.shirsha
    }
}

#[cfg(feature = "alloc")]
impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse for min-heap
        other
            .duri
            .partial_cmp(&self.duri)
            .unwrap_or(Ordering::Equal)
    }
}

#[cfg(feature = "alloc")]
impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Dijkstra's shortest path algorithm (डिज्क्स्ट्रा मार्ग)
///
/// Finds shortest paths from source to all vertices.
///
/// # Etymology
/// लघुतम (laghutama) = shortest
/// मार्ग (mārga) = path
///
/// # Returns
/// - `duri`: Distances from source
/// - `purva`: Previous vertex in shortest path (-1 if unreachable)
///
/// # Complexity
/// - Time: O((V + E) log V)
/// - Space: O(V)
#[cfg(feature = "alloc")]
pub fn dijkstra_marga(lekha: &Lekha, shrota: usize) -> (Vec<f64>, Vec<isize>) {
    let n = lekha.shirsha_sankhya();
    let mut duri = vec![f64::INFINITY; n];
    let mut purva = vec![-1isize; n];
    let mut dheri: BinaryHeap<DijkstraState> = BinaryHeap::new();

    duri[shrota] = 0.0;
    dheri.push(DijkstraState {
        duri: 0.0,
        shirsha: shrota,
    });

    while let Some(DijkstraState {
        duri: d,
        shirsha: u,
    }) = dheri.pop()
    {
        if d > duri[u] {
            continue; // Skip outdated entries
        }

        for &(v, bhara) in lekha.padosi(u) {
            let naya_duri = duri[u] + bhara;
            if naya_duri < duri[v] {
                duri[v] = naya_duri;
                purva[v] = u as isize;
                dheri.push(DijkstraState {
                    duri: naya_duri,
                    shirsha: v,
                });
            }
        }
    }

    (duri, purva)
}

/// Reconstruct path from Dijkstra result (मार्ग पुनर्निर्माण)
#[cfg(feature = "alloc")]
pub fn marga_punarnirman(purva: &[isize], gantavya: usize) -> Option<Vec<usize>> {
    if purva[gantavya] == -1 && gantavya != 0 {
        return None; // Unreachable
    }

    let mut marga = Vec::new();
    let mut vartamana = gantavya as isize;

    while vartamana != -1 {
        marga.push(vartamana as usize);
        vartamana = purva[vartamana as usize];
    }

    marga.reverse();
    Some(marga)
}

/// Bellman-Ford for graphs with negative edges (बेलमैन-फोर्ड)
///
/// Handles negative edge weights, detects negative cycles.
///
/// # Returns
/// - `None` if negative cycle detected
/// - `Some((duri, purva))` otherwise
#[cfg(feature = "alloc")]
pub fn bellman_ford_marga(lekha: &Lekha, shrota: usize) -> Option<(Vec<f64>, Vec<isize>)> {
    let n = lekha.shirsha_sankhya();
    let mut duri = vec![f64::INFINITY; n];
    let mut purva = vec![-1isize; n];

    duri[shrota] = 0.0;

    // Relax all edges V-1 times
    for _ in 0..n - 1 {
        for u in 0..n {
            for &(v, bhara) in lekha.padosi(u) {
                if duri[u] != f64::INFINITY && duri[u] + bhara < duri[v] {
                    duri[v] = duri[u] + bhara;
                    purva[v] = u as isize;
                }
            }
        }
    }

    // Check for negative cycles
    for u in 0..n {
        for &(v, bhara) in lekha.padosi(u) {
            if duri[u] != f64::INFINITY && duri[u] + bhara < duri[v] {
                return None; // Negative cycle detected
            }
        }
    }

    Some((duri, purva))
}

// ============================================================================
// TOPOLOGICAL SORT (स्थलाकृतिक क्रमण)
// ============================================================================

/// Topological sort using DFS (स्थलाकृतिक क्रमण)
///
/// Orders vertices so that for every edge u→v, u comes before v.
/// Only works on DAGs (Directed Acyclic Graphs).
///
/// # Etymology
/// स्थलाकृतिक (sthalākṛtika) = topological
///
/// # Returns
/// `None` if graph has a cycle
#[cfg(feature = "alloc")]
pub fn sthalakrtik_kramana(lekha: &Lekha) -> Option<Vec<usize>> {
    let n = lekha.shirsha_sankhya();
    let mut darshita = vec![0u8; n]; // 0: unvisited, 1: in progress, 2: done
    let mut krama = Vec::with_capacity(n);

    for shirsha in 0..n {
        if darshita[shirsha] == 0 {
            if !topo_dfs(lekha, shirsha, &mut darshita, &mut krama) {
                return None; // Cycle detected
            }
        }
    }

    krama.reverse();
    Some(krama)
}

#[cfg(feature = "alloc")]
fn topo_dfs(lekha: &Lekha, shirsha: usize, darshita: &mut [u8], krama: &mut Vec<usize>) -> bool {
    darshita[shirsha] = 1; // In progress

    for &(padosi, _) in lekha.padosi(shirsha) {
        if darshita[padosi] == 1 {
            return false; // Back edge = cycle
        }
        if darshita[padosi] == 0 {
            if !topo_dfs(lekha, padosi, darshita, krama) {
                return false;
            }
        }
    }

    darshita[shirsha] = 2; // Done
    krama.push(shirsha);
    true
}

/// Kahn's algorithm for topological sort (काह्न क्रमण)
///
/// Uses in-degree counting.
#[cfg(feature = "alloc")]
pub fn kahn_kramana(lekha: &Lekha) -> Option<Vec<usize>> {
    let n = lekha.shirsha_sankhya();
    let mut in_degree = vec![0usize; n];

    // Calculate in-degrees
    for u in 0..n {
        for &(v, _) in lekha.padosi(u) {
            in_degree[v] += 1;
        }
    }

    // Start with vertices of in-degree 0
    let mut pankti: VecDeque<usize> = VecDeque::new();
    for (v, &deg) in in_degree.iter().enumerate() {
        if deg == 0 {
            pankti.push_back(v);
        }
    }

    let mut krama = Vec::with_capacity(n);

    while let Some(u) = pankti.pop_front() {
        krama.push(u);

        for &(v, _) in lekha.padosi(u) {
            in_degree[v] -= 1;
            if in_degree[v] == 0 {
                pankti.push_back(v);
            }
        }
    }

    if krama.len() == n {
        Some(krama)
    } else {
        None // Cycle detected
    }
}

// ============================================================================
// CONNECTED COMPONENTS (संयुक्त घटक)
// ============================================================================

/// Find connected components in undirected graph (संयुक्त घटक)
///
/// # Returns
/// Vector where `ghtak[i]` is the component ID of vertex i
#[cfg(feature = "alloc")]
pub fn samyukta_ghatak(lekha: &Lekha) -> Vec<usize> {
    let n = lekha.shirsha_sankhya();
    let mut ghatak = vec![usize::MAX; n];
    let mut ghatak_id = 0;

    for shirsha in 0..n {
        if ghatak[shirsha] == usize::MAX {
            // BFS to mark all vertices in this component
            let mut pankti: VecDeque<usize> = VecDeque::new();
            pankti.push_back(shirsha);
            ghatak[shirsha] = ghatak_id;

            while let Some(u) = pankti.pop_front() {
                for &(v, _) in lekha.padosi(u) {
                    if ghatak[v] == usize::MAX {
                        ghatak[v] = ghatak_id;
                        pankti.push_back(v);
                    }
                }
            }

            ghatak_id += 1;
        }
    }

    ghatak
}

/// Count number of connected components (घटक गणना)
#[cfg(feature = "alloc")]
pub fn ghatak_ganana(lekha: &Lekha) -> usize {
    let ghatak = samyukta_ghatak(lekha);
    ghatak.iter().max().map(|&m| m + 1).unwrap_or(0)
}

// ============================================================================
// MINIMUM SPANNING TREE (न्यूनतम विस्तार वृक्ष)
// ============================================================================

/// Union-Find data structure (संघ-खोज)
#[cfg(feature = "alloc")]
pub struct SanghKhoj {
    pita: Vec<usize>,
    shreni: Vec<usize>,
}

#[cfg(feature = "alloc")]
impl SanghKhoj {
    pub fn nava(n: usize) -> Self {
        Self {
            pita: (0..n).collect(),
            shreni: vec![0; n],
        }
    }

    /// Find root with path compression (खोज)
    pub fn khoj(&mut self, x: usize) -> usize {
        if self.pita[x] != x {
            self.pita[x] = self.khoj(self.pita[x]);
        }
        self.pita[x]
    }

    /// Union by rank (संघ)
    pub fn sangh(&mut self, x: usize, y: usize) -> bool {
        let px = self.khoj(x);
        let py = self.khoj(y);

        if px == py {
            return false; // Already in same set
        }

        if self.shreni[px] < self.shreni[py] {
            self.pita[px] = py;
        } else if self.shreni[px] > self.shreni[py] {
            self.pita[py] = px;
        } else {
            self.pita[py] = px;
            self.shreni[px] += 1;
        }
        true
    }
}

/// Kruskal's MST algorithm (क्रुस्कल वृक्ष)
///
/// Finds minimum spanning tree using sorted edges.
///
/// # Returns
/// Edges in MST and total weight
#[cfg(feature = "alloc")]
pub fn kruskal_vrksha(lekha: &Lekha) -> (Vec<Kinara>, f64) {
    let n = lekha.shirsha_sankhya();

    // Collect all edges
    let mut kinaras: Vec<Kinara> = Vec::new();
    for u in 0..n {
        for &(v, bhara) in lekha.padosi(u) {
            if u < v {
                // Avoid duplicates in undirected graph
                kinaras.push(Kinara::new(u, v, bhara));
            }
        }
    }

    // Sort by weight
    kinaras.sort_by(|a, b| a.bhara.partial_cmp(&b.bhara).unwrap());

    let mut uf = SanghKhoj::nava(n);
    let mut mst = Vec::new();
    let mut kul_bhara = 0.0;

    for kinara in kinaras {
        if uf.sangh(kinara.shrota, kinara.gantavya) {
            kul_bhara += kinara.bhara;
            mst.push(kinara);

            if mst.len() == n - 1 {
                break;
            }
        }
    }

    (mst, kul_bhara)
}

// ============================================================================
// CYCLE DETECTION (चक्र पहचान)
// ============================================================================

/// Detect cycle in undirected graph (अदिशात्मक चक्र)
#[cfg(feature = "alloc")]
pub fn chakra_pahchan_adishatmaka(lekha: &Lekha) -> bool {
    let n = lekha.shirsha_sankhya();
    let mut darshita = vec![false; n];

    for shirsha in 0..n {
        if !darshita[shirsha] {
            if chakra_dfs_undirected(lekha, shirsha, usize::MAX, &mut darshita) {
                return true;
            }
        }
    }
    false
}

#[cfg(feature = "alloc")]
fn chakra_dfs_undirected(
    lekha: &Lekha,
    shirsha: usize,
    pita: usize,
    darshita: &mut [bool],
) -> bool {
    darshita[shirsha] = true;

    for &(padosi, _) in lekha.padosi(shirsha) {
        if !darshita[padosi] {
            if chakra_dfs_undirected(lekha, padosi, shirsha, darshita) {
                return true;
            }
        } else if padosi != pita {
            return true; // Back edge to non-parent = cycle
        }
    }
    false
}

/// Detect cycle in directed graph (दिशात्मक चक्र)
#[cfg(feature = "alloc")]
pub fn chakra_pahchan_dishatmaka(lekha: &Lekha) -> bool {
    // If topological sort fails, there's a cycle
    sthalakrtik_kramana(lekha).is_none()
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_lekha_basic() {
        let mut g = Lekha::adishatmaka(5);
        g.abharit_kinara(0, 1);
        g.abharit_kinara(0, 2);
        g.abharit_kinara(1, 2);

        assert_eq!(g.shirsha_sankhya(), 5);
        assert!(g.kinara_astitva(0, 1));
        assert!(g.kinara_astitva(1, 0)); // Undirected
        assert!(!g.kinara_astitva(0, 3));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_bfs() {
        let mut g = Lekha::adishatmaka(5);
        g.abharit_kinara(0, 1);
        g.abharit_kinara(0, 2);
        g.abharit_kinara(1, 3);
        g.abharit_kinara(2, 4);

        let krama = vistar_prathama_anveshan(&g, 0);
        assert_eq!(krama.len(), 5);
        assert_eq!(krama[0], 0); // Start with source
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_dfs() {
        let mut g = Lekha::adishatmaka(4);
        g.abharit_kinara(0, 1);
        g.abharit_kinara(0, 2);
        g.abharit_kinara(1, 3);

        let krama = gaharai_prathama_anveshan(&g, 0);
        assert_eq!(krama.len(), 4);
        assert_eq!(krama[0], 0);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_dijkstra() {
        let mut g = Lekha::dishatmaka(5);
        g.kinara_jodana(0, 1, 10.0);
        g.kinara_jodana(0, 3, 5.0);
        g.kinara_jodana(1, 2, 1.0);
        g.kinara_jodana(1, 3, 2.0);
        g.kinara_jodana(3, 1, 3.0);
        g.kinara_jodana(3, 2, 9.0);
        g.kinara_jodana(3, 4, 2.0);
        g.kinara_jodana(2, 4, 4.0);
        g.kinara_jodana(4, 2, 6.0);

        let (duri, purva) = dijkstra_marga(&g, 0);

        assert_eq!(duri[0], 0.0);
        assert_eq!(duri[1], 8.0); // 0→3→1
        assert_eq!(duri[2], 9.0); // 0→3→1→2
        assert_eq!(duri[3], 5.0); // 0→3
        assert_eq!(duri[4], 7.0); // 0→3→4

        // Test path reconstruction
        let marga = marga_punarnirman(&purva, 2).unwrap();
        assert_eq!(marga[0], 0); // Start
        assert_eq!(*marga.last().unwrap(), 2); // End
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_topological_sort() {
        let mut g = Lekha::dishatmaka(6);
        g.abharit_kinara(5, 2);
        g.abharit_kinara(5, 0);
        g.abharit_kinara(4, 0);
        g.abharit_kinara(4, 1);
        g.abharit_kinara(2, 3);
        g.abharit_kinara(3, 1);

        let krama = sthalakrtik_kramana(&g);
        assert!(krama.is_some());

        let krama = krama.unwrap();
        assert_eq!(krama.len(), 6);

        // Verify topological order
        let pos: Vec<usize> = {
            let mut p = vec![0; 6];
            for (i, &v) in krama.iter().enumerate() {
                p[v] = i;
            }
            p
        };

        // For each edge u→v, u should come before v
        assert!(pos[5] < pos[2]);
        assert!(pos[5] < pos[0]);
        assert!(pos[2] < pos[3]);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_cycle_detection() {
        // Undirected with cycle
        let mut g1 = Lekha::adishatmaka(3);
        g1.abharit_kinara(0, 1);
        g1.abharit_kinara(1, 2);
        g1.abharit_kinara(2, 0);
        assert!(chakra_pahchan_adishatmaka(&g1));

        // Undirected without cycle (tree)
        let mut g2 = Lekha::adishatmaka(3);
        g2.abharit_kinara(0, 1);
        g2.abharit_kinara(1, 2);
        assert!(!chakra_pahchan_adishatmaka(&g2));

        // Directed with cycle
        let mut g3 = Lekha::dishatmaka(3);
        g3.abharit_kinara(0, 1);
        g3.abharit_kinara(1, 2);
        g3.abharit_kinara(2, 0);
        assert!(chakra_pahchan_dishatmaka(&g3));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_connected_components() {
        let mut g = Lekha::adishatmaka(6);
        // Component 1: 0-1-2
        g.abharit_kinara(0, 1);
        g.abharit_kinara(1, 2);
        // Component 2: 3-4
        g.abharit_kinara(3, 4);
        // Component 3: 5 (isolated)

        let count = ghatak_ganana(&g);
        assert_eq!(count, 3);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_kruskal_mst() {
        let mut g = Lekha::adishatmaka(4);
        g.kinara_jodana(0, 1, 10.0);
        g.kinara_jodana(0, 2, 6.0);
        g.kinara_jodana(0, 3, 5.0);
        g.kinara_jodana(1, 3, 15.0);
        g.kinara_jodana(2, 3, 4.0);

        let (mst, kul_bhara) = kruskal_vrksha(&g);

        assert_eq!(mst.len(), 3); // V-1 edges
        assert_eq!(kul_bhara, 19.0); // 4 + 5 + 10
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_union_find() {
        let mut uf = SanghKhoj::nava(5);

        assert!(uf.sangh(0, 1));
        assert!(uf.sangh(2, 3));
        assert_eq!(uf.khoj(0), uf.khoj(1));
        assert_ne!(uf.khoj(0), uf.khoj(2));

        assert!(uf.sangh(1, 2)); // Merge two components
        assert_eq!(uf.khoj(0), uf.khoj(3));

        assert!(!uf.sangh(0, 3)); // Already in same set
    }
}
