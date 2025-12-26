//! Deterministic Builds
//!
//! Ensures reproducible, deterministic compilation.
//! "Same input → Same output, always"

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// Deterministic build configuration
pub struct DeterministicBuild {
    /// Fixed timestamp (or None for current)
    pub timestamp: Option<u64>,
    /// Fixed random seed
    pub seed: u64,
    /// Sorted iteration over collections
    pub sorted_iteration: bool,
    /// Canonical path handling
    pub canonical_paths: bool,
    /// Strip source paths
    pub strip_source_paths: bool,
}

impl DeterministicBuild {
    pub fn new() -> Self {
        Self {
            timestamp: Some(0), // Epoch for reproducibility
            seed: 42,          // Fixed seed
            sorted_iteration: true,
            canonical_paths: true,
            strip_source_paths: true,
        }
    }

    /// Get timestamp
    pub fn timestamp(&self) -> u64 {
        self.timestamp.unwrap_or_else(|| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        })
    }

    /// Get random seed
    pub fn seed(&self) -> u64 {
        self.seed
    }

    /// Normalize path for reproducibility
    pub fn normalize_path(&self, path: &str) -> String {
        if self.strip_source_paths {
            // Keep only filename
            std::path::Path::new(path)
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
                .unwrap_or_else(|| path.to_string())
        } else if self.canonical_paths {
            // Use forward slashes and relative paths
            path.replace('\\', "/")
        } else {
            path.to_string()
        }
    }

    /// Sort collection for deterministic iteration
    pub fn sorted<T: Ord + Clone>(&self, items: &[T]) -> Vec<T> {
        if self.sorted_iteration {
            let mut sorted = items.to_vec();
            sorted.sort();
            sorted
        } else {
            items.to_vec()
        }
    }

    /// Sort hashmap keys for deterministic iteration
    pub fn sorted_keys<K: Ord + Clone, V>(&self, map: &HashMap<K, V>) -> Vec<K> {
        if self.sorted_iteration {
            let mut keys: Vec<K> = map.keys().cloned().collect();
            keys.sort();
            keys
        } else {
            map.keys().cloned().collect()
        }
    }
}

/// Reproducibility checker
pub struct ReproducibilityChecker {
    /// Build hashes
    hashes: Vec<u64>,
    /// Build artifacts
    artifacts: Vec<String>,
}

impl ReproducibilityChecker {
    pub fn new() -> Self {
        Self {
            hashes: Vec::new(),
            artifacts: Vec::new(),
        }
    }

    /// Record a build
    pub fn record_build(&mut self, artifact: &str, content: &[u8]) {
        let hash = self.compute_hash(content);
        self.hashes.push(hash);
        self.artifacts.push(artifact.to_string());
    }

    /// Compute hash of content
    fn compute_hash(&self, content: &[u8]) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        hasher.finish()
    }

    /// Verify reproducibility
    pub fn verify(&self, artifact: &str, content: &[u8]) -> ReproducibilityResult {
        let hash = self.compute_hash(content);

        for (i, (stored_artifact, stored_hash)) in
            self.artifacts.iter().zip(&self.hashes).enumerate()
        {
            if stored_artifact == artifact {
                if *stored_hash == hash {
                    return ReproducibilityResult::Reproducible;
                } else {
                    return ReproducibilityResult::Diverged {
                        expected_hash: *stored_hash,
                        actual_hash: hash,
                        build_index: i,
                    };
                }
            }
        }

        ReproducibilityResult::NotRecorded
    }

    /// Check if all builds are reproducible
    pub fn all_reproducible(&self) -> bool {
        if self.hashes.len() < 2 {
            return true;
        }

        // Group by artifact name
        let mut by_artifact: HashMap<&str, Vec<u64>> = HashMap::new();
        for (artifact, hash) in self.artifacts.iter().zip(&self.hashes) {
            by_artifact.entry(artifact).or_default().push(*hash);
        }

        // Check each artifact has consistent hashes
        for (_, hashes) in by_artifact {
            if hashes.iter().any(|h| *h != hashes[0]) {
                return false;
            }
        }

        true
    }

    /// Generate report
    pub fn report(&self) -> String {
        let mut report = String::new();
        report.push_str("=== Reproducibility Report ===\n\n");

        let reproducible = self.all_reproducible();
        report.push_str(&format!(
            "Status: {}\n",
            if reproducible { "✓ Reproducible" } else { "✗ Non-reproducible" }
        ));

        report.push_str(&format!("Total builds recorded: {}\n\n", self.hashes.len()));

        // Group by artifact
        let mut by_artifact: HashMap<&str, Vec<u64>> = HashMap::new();
        for (artifact, hash) in self.artifacts.iter().zip(&self.hashes) {
            by_artifact.entry(artifact).or_default().push(*hash);
        }

        for (artifact, hashes) in &by_artifact {
            let unique_hashes: std::collections::HashSet<_> = hashes.iter().collect();
            let status = if unique_hashes.len() == 1 { "✓" } else { "✗" };
            report.push_str(&format!(
                "{} {}: {} builds, {} unique hashes\n",
                status,
                artifact,
                hashes.len(),
                unique_hashes.len()
            ));
        }

        report
    }
}

/// Reproducibility result
#[derive(Debug)]
pub enum ReproducibilityResult {
    /// Build is reproducible
    Reproducible,
    /// Build diverged from expected
    Diverged {
        expected_hash: u64,
        actual_hash: u64,
        build_index: usize,
    },
    /// Artifact not previously recorded
    NotRecorded,
}

impl Default for DeterministicBuild {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ReproducibilityChecker {
    fn default() -> Self {
        Self::new()
    }
}
