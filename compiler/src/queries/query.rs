//! # Query Types - Karman (Action) and Phala (Result)
//!
//! Core types for the query system.
//!
//! # Sanskrit Foundation
//!
//! **Karman** (कर्मन्) - action/deed:
//! A query is an action that computes a result.
//!
//! **Phala** (फल) - fruit/result:
//! The result of a query is the fruit of the action.

use std::any::Any;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

/// Query identifier
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct QueryId {
    /// Query name
    pub name: String,
}

impl QueryId {
    /// Create a new query ID
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

/// Query key - combines query ID with input key
#[derive(Clone)]
pub struct QueryKey {
    /// The query being executed
    pub query_id: QueryId,

    /// The input key (type-erased)
    key: Arc<dyn Any + Send + Sync>,

    /// Hash of the key for fast comparison
    key_hash: u64,
}

impl QueryKey {
    /// Create a new query key
    pub fn new<K: Hash + Send + Sync + 'static>(query_id: QueryId, key: Box<K>) -> Self {
        use std::collections::hash_map::DefaultHasher;

        let mut hasher = DefaultHasher::new();
        query_id.hash(&mut hasher);
        key.hash(&mut hasher);
        let key_hash = hasher.finish();

        Self {
            query_id,
            key: Arc::new(key) as Arc<dyn Any + Send + Sync>,
            key_hash,
        }
    }

    /// Get the key as a specific type
    pub fn key<K: 'static>(&self) -> Option<&K> {
        // The key is stored as Box<K>, so downcast to Box<K> first
        self.key
            .downcast_ref::<Box<K>>()
            .map(|boxed| boxed.as_ref())
    }
}

impl std::fmt::Debug for QueryKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QueryKey")
            .field("query_id", &self.query_id)
            .field("key_hash", &self.key_hash)
            .finish()
    }
}

impl PartialEq for QueryKey {
    fn eq(&self, other: &Self) -> bool {
        self.query_id == other.query_id && self.key_hash == other.key_hash
    }
}

impl Eq for QueryKey {}

impl Hash for QueryKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.query_id.hash(state);
        self.key_hash.hash(state);
    }
}

/// Query result type
pub type QueryResult<T> = Result<T, QueryError>;

/// Query execution errors
#[derive(Debug, Clone)]
pub enum QueryError {
    /// Unknown query
    UnknownQuery(String),

    /// Cyclic dependency detected (saṃsāra!)
    CyclicDependency {
        /// Query that detected the cycle
        query: String,
        /// The cycle path
        cycle: Vec<String>,
    },

    /// Type mismatch in result
    TypeMismatch {
        /// Expected type
        expected: String,
        /// Query that had wrong type
        query: String,
    },

    /// Query execution failed
    ExecutionFailed {
        /// Query that failed
        query: String,
        /// Error message
        message: String,
    },

    /// Input not found
    InputNotFound {
        /// Query name
        query: String,
        /// Missing key description
        key: String,
    },
}

impl std::fmt::Display for QueryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QueryError::UnknownQuery(name) => {
                write!(f, "Unknown query: '{}'", name)
            }
            QueryError::CyclicDependency { query, cycle } => {
                write!(
                    f,
                    "Cyclic dependency (saṃsāra) detected at '{}': {}",
                    query,
                    cycle.join(" -> ")
                )
            }
            QueryError::TypeMismatch { expected, query } => {
                write!(
                    f,
                    "Type mismatch: expected {} from query '{}'",
                    expected, query
                )
            }
            QueryError::ExecutionFailed { query, message } => {
                write!(f, "Query '{}' failed: {}", query, message)
            }
            QueryError::InputNotFound { query, key } => {
                write!(f, "Input not found for query '{}': {}", query, key)
            }
        }
    }
}

impl std::error::Error for QueryError {}

/// A query definition
pub struct Query<K, V, F>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Send + Sync + 'static,
    F: Fn(&super::KarmaKosha, &K) -> QueryResult<V> + Send + Sync,
{
    /// Query name
    pub name: String,

    /// Query function
    pub func: F,

    /// Phantom data
    _phantom: std::marker::PhantomData<(K, V)>,
}

impl<K, V, F> Query<K, V, F>
where
    K: Hash + Eq + Clone + Send + Sync + 'static,
    V: Send + Sync + 'static,
    F: Fn(&super::KarmaKosha, &K) -> QueryResult<V> + Send + Sync,
{
    /// Create a new query
    pub fn new(name: &str, func: F) -> Self {
        Self {
            name: name.to_string(),
            func,
            _phantom: std::marker::PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_id_creation() {
        let id = QueryId::new("parse");
        assert_eq!(id.name, "parse");
    }

    #[test]
    fn test_query_id_equality() {
        let id1 = QueryId::new("parse");
        let id2 = QueryId::new("parse");
        let id3 = QueryId::new("typecheck");

        assert_eq!(id1, id2);
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_query_key_creation() {
        let id = QueryId::new("parse");
        let key = QueryKey::new(id.clone(), Box::new("main.jag".to_string()));

        assert_eq!(key.query_id, id);
    }

    #[test]
    fn test_query_key_equality() {
        let id = QueryId::new("parse");
        let key1 = QueryKey::new(id.clone(), Box::new("main.jag".to_string()));
        let key2 = QueryKey::new(id.clone(), Box::new("main.jag".to_string()));
        let key3 = QueryKey::new(id.clone(), Box::new("lib.jag".to_string()));

        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
    }

    #[test]
    fn test_query_error_display() {
        let err = QueryError::CyclicDependency {
            query: "typecheck".to_string(),
            cycle: vec![
                "parse".to_string(),
                "resolve".to_string(),
                "parse".to_string(),
            ],
        };

        let msg = err.to_string();
        assert!(msg.contains("saṃsāra"));
        assert!(msg.contains("typecheck"));
    }

    #[test]
    fn test_key_extraction() {
        let id = QueryId::new("test");
        let key = QueryKey::new(id, Box::new(42i32));

        assert_eq!(key.key::<i32>(), Some(&42));
        assert_eq!(key.key::<String>(), None);
    }
}
