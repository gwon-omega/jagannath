//! # Preta Module - Resource Leak Detection
//!
//! Preta (प्रेत) = "Hungry Ghost" - resources that linger after death.
//! Detects resource leaks: unclosed handles, unfreed memory, orphaned connections.

mod detector;
mod ghost;

pub use detector::{Ghost, GhostType, HungerLevel, PretaDetector};
pub use ghost::HungryGhost;

use crate::errors::Span;

/// A resource leak (hungry ghost)
#[derive(Debug, Clone)]
pub struct PretaViolation {
    /// Location where resource was allocated
    pub allocated_at: Span,
    /// Variable/symbol name
    pub symbol: String,
    /// Location where resource should have been freed
    pub expected_free_site: Option<Span>,
    /// Type of resource
    pub resource_type: ResourceType,
    /// Description
    pub message: String,
    /// Sanskrit description
    pub sanskrit_name: String,
}

/// Types of resources that can become preta (ghosts)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceType {
    /// Memory allocation
    Memory,
    /// File handle
    FileHandle,
    /// Network socket
    Socket,
    /// Database connection
    DatabaseConnection,
    /// Lock/mutex
    Lock,
    /// Thread handle
    ThreadHandle,
    /// Generic handle
    Handle,
}

impl ResourceType {
    /// Get Sanskrit name for this resource type
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            ResourceType::Memory => "smṛti-kośa",     // memory cell
            ResourceType::FileHandle => "koṣa-dvāra", // file door
            ResourceType::Socket => "jāla-bandha",    // network binding
            ResourceType::DatabaseConnection => "sāraṇī-yoga", // table connection
            ResourceType::Lock => "tāla",             // lock
            ResourceType::ThreadHandle => "tantu-sūtra", // thread handle
            ResourceType::Handle => "graha",          // grasp/handle
        }
    }

    /// Get cleanup function name
    pub fn cleanup_function(&self) -> &'static str {
        match self {
            ResourceType::Memory => "mukta",              // free
            ResourceType::FileHandle => "bandha",         // close
            ResourceType::Socket => "viyoga",             // disconnect
            ResourceType::DatabaseConnection => "viyoga", // disconnect
            ResourceType::Lock => "mukta-tāla",           // unlock
            ResourceType::ThreadHandle => "pratīkṣa",     // join/wait
            ResourceType::Handle => "tyaja",              // release
        }
    }
}
