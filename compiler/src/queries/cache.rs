//! # Query Cache - Smṛti (Memory)
//!
//! Caches query results for incremental computation.
//!
//! # Sanskrit Foundation
//!
//! **Smṛti** (स्मृति) means "memory" or "remembrance".
//! In the compiler, this is where we remember the fruits (phala)
//! of previous computations (karma).

use super::query::QueryKey;
use super::revision::Revision;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;

/// A cached query result
#[derive(Clone)]
pub struct CacheEntry {
    /// The cached value
    pub value: Arc<dyn Any + Send + Sync>,
    /// Revision when computed
    pub revision: Revision,
    /// Dependencies of this query
    pub dependencies: Vec<QueryKey>,
}

impl CacheEntry {
    /// Check if entry is still valid
    pub fn is_valid(&self, current_revision: &Revision) -> bool {
        // For now, simple revision comparison
        // TODO: Check if any dependencies have changed
        self.revision == *current_revision
    }
}

/// Cache statistics
#[derive(Debug, Default, Clone)]
pub struct CacheStats {
    /// Number of entries
    pub entries: usize,
    /// Approximate memory usage
    pub memory_bytes: usize,
    /// Evictions performed
    pub evictions: u64,
}

/// Query result cache
pub struct QueryCache {
    /// Cached entries
    entries: HashMap<QueryKey, CacheEntry>,
    /// Maximum entries (0 = unlimited)
    max_entries: usize,
    /// Statistics
    stats: CacheStats,
}

impl QueryCache {
    /// Create a new cache
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
            max_entries: 10000,
            stats: CacheStats::default(),
        }
    }

    /// Create with capacity limit
    pub fn with_capacity(max_entries: usize) -> Self {
        Self {
            entries: HashMap::with_capacity(max_entries.min(1000)),
            max_entries,
            stats: CacheStats::default(),
        }
    }

    /// Get a cached entry
    pub fn get(&self, key: &QueryKey) -> Option<&CacheEntry> {
        self.entries.get(key)
    }

    /// Insert a cache entry
    pub fn insert(&mut self, key: QueryKey, entry: CacheEntry) -> Option<CacheEntry> {
        // Check capacity
        if self.max_entries > 0 && self.entries.len() >= self.max_entries {
            self.evict_lru();
        }

        self.stats.entries = self.entries.len() + 1;
        self.entries.insert(key, entry)
    }

    /// Remove a cache entry
    pub fn remove(&mut self, key: &QueryKey) -> Option<CacheEntry> {
        let result = self.entries.remove(key);
        self.stats.entries = self.entries.len();
        result
    }

    /// Clear all entries
    pub fn clear(&mut self) {
        self.entries.clear();
        self.stats.entries = 0;
    }

    /// Get statistics
    pub fn stats(&self) -> CacheStats {
        let mut stats = self.stats.clone();
        stats.entries = self.entries.len();
        stats
    }

    /// Evict least recently used entries (simple eviction)
    fn evict_lru(&mut self) {
        // Simple strategy: remove 10% of entries
        let to_remove = self.entries.len() / 10;
        let keys: Vec<_> = self.entries.keys().take(to_remove).cloned().collect();
        for key in keys {
            self.entries.remove(&key);
            self.stats.evictions += 1;
        }
    }

    /// Number of cached entries
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl Default for QueryCache {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::super::QueryId;
    use super::*;

    #[test]
    fn test_cache_creation() {
        let cache = QueryCache::new();
        assert!(cache.is_empty());
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_cache_with_capacity() {
        let cache = QueryCache::with_capacity(100);
        assert_eq!(cache.max_entries, 100);
    }

    #[test]
    fn test_cache_insert_get() {
        let mut cache = QueryCache::new();
        let key = QueryKey::new(QueryId::new("test"), Box::new("key".to_string()));
        let entry = CacheEntry {
            value: Arc::new(42i32) as Arc<dyn Any + Send + Sync>,
            revision: Revision::new(),
            dependencies: Vec::new(),
        };

        cache.insert(key.clone(), entry);
        assert_eq!(cache.len(), 1);
        assert!(cache.get(&key).is_some());
    }

    #[test]
    fn test_cache_remove() {
        let mut cache = QueryCache::new();
        let key = QueryKey::new(QueryId::new("test"), Box::new("key".to_string()));
        let entry = CacheEntry {
            value: Arc::new(42i32) as Arc<dyn Any + Send + Sync>,
            revision: Revision::new(),
            dependencies: Vec::new(),
        };

        cache.insert(key.clone(), entry);
        assert_eq!(cache.len(), 1);

        cache.remove(&key);
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_cache_clear() {
        let mut cache = QueryCache::new();
        for i in 0..10 {
            let key = QueryKey::new(QueryId::new("test"), Box::new(i));
            let entry = CacheEntry {
                value: Arc::new(i) as Arc<dyn Any + Send + Sync>,
                revision: Revision::new(),
                dependencies: Vec::new(),
            };
            cache.insert(key, entry);
        }

        assert_eq!(cache.len(), 10);
        cache.clear();
        assert!(cache.is_empty());
    }

    #[test]
    fn test_entry_validity() {
        let rev = Revision::new();
        let entry = CacheEntry {
            value: Arc::new(42i32) as Arc<dyn Any + Send + Sync>,
            revision: rev.clone(),
            dependencies: Vec::new(),
        };

        assert!(entry.is_valid(&rev));

        let new_rev = rev.next();
        assert!(!entry.is_valid(&new_rev));
    }
}
