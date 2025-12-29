//! # Query System - Karma-Driven Incremental Computation
//!
//! A memoization and incremental computation framework for compiler analysis,
//! inspired by Salsa and aligned with the Karma (action/result) philosophy.
//!
//! # Sanskrit Foundation
//!
//! **Karma** (कर्म) - action that produces results:
//! - Every query is a karman (action)
//! - Results are phalas (fruits of action)
//! - Dependencies form the karma-bandha (action-bond chain)
//!
//! **Smṛti** (स्मृति) - memory/remembrance:
//! - Cached results are stored in smṛti (memory)
//! - Re-execution only when inputs change
//!
//! # Architecture
//!
//! The query system follows the demand-driven incremental model:
//! 1. **Input Queries**: Base data (source files, configuration)
//! 2. **Derived Queries**: Computed from other queries
//! 3. **Dependency Tracking**: Automatic via phantom reads
//! 4. **Invalidation**: Cascading when inputs change
//!
//! # Usage
//!
//! ```ignore
//! // Define a query
//! query_system.define_query("parse", |db, path| {
//!     let source = db.query("read_file", path)?;
//!     parse_source(&source)
//! });
//!
//! // Execute query (result is cached)
//! let ast = query_system.query("parse", "main.jag")?;
//! ```

mod cache;
mod dependency;
mod query;
mod revision;

pub use cache::{QueryCache, CacheEntry, CacheStats};
pub use dependency::{DependencyGraph, DependencyTracker};
pub use query::{Query, QueryId, QueryKey, QueryResult, QueryError};
pub use revision::{Revision, RevisionGuard};

use std::any::Any;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// The central query database - Karma Kosha (action repository)
///
/// Manages all queries, their dependencies, and cached results.
pub struct KarmaKosha {
    /// Current revision (saṃskāra - impression/change)
    revision: RwLock<Revision>,

    /// Query definitions
    queries: RwLock<HashMap<QueryId, Arc<dyn QueryExecutor>>>,

    /// Query cache (smṛti - memory)
    smriti: RwLock<QueryCache>,

    /// Dependency graph (karma-bandha - action bonds)
    karma_bandha: RwLock<DependencyGraph>,

    /// Active query stack (for cycle detection)
    active_stack: RwLock<Vec<QueryKey>>,

    /// Statistics
    stats: RwLock<QueryStats>,
}

/// Query execution statistics
#[derive(Debug, Default, Clone)]
pub struct QueryStats {
    /// Total queries executed
    pub total_queries: u64,
    /// Cache hits
    pub cache_hits: u64,
    /// Cache misses
    pub cache_misses: u64,
    /// Invalidations
    pub invalidations: u64,
    /// Cycles detected
    pub cycles_detected: u64,
}

impl QueryStats {
    /// Cache hit ratio
    pub fn hit_ratio(&self) -> f64 {
        if self.total_queries == 0 {
            return 0.0;
        }
        self.cache_hits as f64 / self.total_queries as f64
    }
}

/// Trait for query execution
pub trait QueryExecutor: Send + Sync {
    /// Execute the query
    fn execute(&self, db: &KarmaKosha, key: &dyn Any) -> QueryResult<Box<dyn Any + Send + Sync>>;

    /// Query name for debugging
    fn name(&self) -> &str;
}

impl KarmaKosha {
    /// Create a new query database
    pub fn new() -> Self {
        Self {
            revision: RwLock::new(Revision::new()),
            queries: RwLock::new(HashMap::new()),
            smriti: RwLock::new(QueryCache::new()),
            karma_bandha: RwLock::new(DependencyGraph::new()),
            active_stack: RwLock::new(Vec::new()),
            stats: RwLock::new(QueryStats::default()),
        }
    }

    /// Register a query
    pub fn register_query(&self, id: QueryId, executor: Arc<dyn QueryExecutor>) {
        let mut queries = self.queries.write().unwrap();
        queries.insert(id, executor);
    }

    /// Execute a query (with caching)
    pub fn query<K, V>(&self, id: &str, key: K) -> QueryResult<Arc<V>>
    where
        K: std::hash::Hash + Eq + Clone + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        let query_id = QueryId::new(id);
        let query_key = QueryKey::new(query_id.clone(), Box::new(key.clone()));

        // Update stats
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_queries += 1;
        }

        // Check cache first
        {
            let smriti = self.smriti.read().unwrap();
            if let Some(entry) = smriti.get(&query_key) {
                let current_rev = self.revision.read().unwrap().clone();
                if entry.is_valid(&current_rev) {
                    let mut stats = self.stats.write().unwrap();
                    stats.cache_hits += 1;

                    // Downcast and return
                    if let Some(value) = entry.value.downcast_ref::<Arc<V>>() {
                        return Ok(value.clone());
                    }
                }
            }
        }

        // Cache miss - execute query
        {
            let mut stats = self.stats.write().unwrap();
            stats.cache_misses += 1;
        }

        // Check for cycles (saṃsāra - cycle of rebirth)
        {
            let active = self.active_stack.read().unwrap();
            if active.iter().any(|k| k == &query_key) {
                let mut stats = self.stats.write().unwrap();
                stats.cycles_detected += 1;
                return Err(QueryError::CyclicDependency {
                    query: id.to_string(),
                    cycle: active.iter().map(|k| k.query_id.name.clone()).collect(),
                });
            }
        }

        // Push to active stack
        {
            let mut active = self.active_stack.write().unwrap();
            active.push(query_key.clone());
        }

        // Execute the query
        let result = self.execute_query::<K, V>(&query_id, &key);

        // Pop from active stack
        {
            let mut active = self.active_stack.write().unwrap();
            active.pop();
        }

        // Cache the result
        if let Ok(ref value) = result {
            let current_rev = self.revision.read().unwrap().clone();
            let entry = CacheEntry {
                value: Arc::new(value.clone()) as Arc<dyn Any + Send + Sync>,
                revision: current_rev,
                dependencies: Vec::new(), // TODO: track dependencies
            };

            let mut smriti = self.smriti.write().unwrap();
            smriti.insert(query_key, entry);
        }

        result
    }

    /// Execute a query without caching
    fn execute_query<K, V>(&self, id: &QueryId, key: &K) -> QueryResult<Arc<V>>
    where
        K: 'static,
        V: Send + Sync + 'static,
    {
        let queries = self.queries.read().unwrap();
        let executor = queries.get(id)
            .ok_or_else(|| QueryError::UnknownQuery(id.name.clone()))?;

        let result = executor.execute(self, key as &dyn Any)?;

        // Downcast the result
        result.downcast::<Arc<V>>()
            .map(|b| *b)
            .map_err(|_| QueryError::TypeMismatch {
                expected: std::any::type_name::<V>().to_string(),
                query: id.name.clone(),
            })
    }

    /// Invalidate all queries depending on an input
    pub fn invalidate(&self, input_key: &QueryKey) {
        let mut stats = self.stats.write().unwrap();
        stats.invalidations += 1;

        // Increment revision
        {
            let mut rev = self.revision.write().unwrap();
            rev.increment();
        }

        // Get dependent queries
        let dependents = {
            let karma_bandha = self.karma_bandha.read().unwrap();
            karma_bandha.get_dependents(input_key)
        };

        // Invalidate cache entries
        {
            let mut smriti = self.smriti.write().unwrap();
            smriti.remove(input_key);
            for dep in dependents {
                smriti.remove(&dep);
            }
        }
    }

    /// Set an input value (base query)
    pub fn set_input<K, V>(&self, id: &str, key: K, value: V)
    where
        K: std::hash::Hash + Eq + Clone + Send + Sync + 'static,
        V: Send + Sync + 'static,
    {
        let query_id = QueryId::new(id);
        let query_key = QueryKey::new(query_id, Box::new(key));

        // Create cache entry
        let current_rev = self.revision.read().unwrap().clone();
        let entry = CacheEntry {
            value: Arc::new(Arc::new(value)) as Arc<dyn Any + Send + Sync>,
            revision: current_rev,
            dependencies: Vec::new(),
        };

        // Insert and invalidate dependents
        let old_exists = {
            let mut smriti = self.smriti.write().unwrap();
            let old = smriti.insert(query_key.clone(), entry);
            old.is_some()
        };

        if old_exists {
            self.invalidate(&query_key);
        }
    }

    /// Get statistics
    pub fn stats(&self) -> QueryStats {
        self.stats.read().unwrap().clone()
    }

    /// Clear all caches
    pub fn clear_cache(&self) {
        let mut smriti = self.smriti.write().unwrap();
        smriti.clear();

        let mut rev = self.revision.write().unwrap();
        rev.increment();
    }
}

impl Default for KarmaKosha {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_karma_kosha_creation() {
        let kosha = KarmaKosha::new();
        let stats = kosha.stats();
        assert_eq!(stats.total_queries, 0);
        assert_eq!(stats.cache_hits, 0);
    }

    #[test]
    fn test_query_stats_hit_ratio() {
        let mut stats = QueryStats::default();
        assert_eq!(stats.hit_ratio(), 0.0);

        stats.total_queries = 100;
        stats.cache_hits = 75;
        assert!((stats.hit_ratio() - 0.75).abs() < 0.001);
    }

    #[test]
    fn test_input_queries() {
        let kosha = KarmaKosha::new();
        kosha.set_input("source", "main.jag".to_string(), "fn main() {}".to_string());
        // Input is now cached
    }
}
