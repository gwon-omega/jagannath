//! Advaita Memory Model
//!
//! Unified memory model based on Advaita Vedānta philosophy.
//! "All memory is ultimately one" - abstracts over stack/heap/registers.
//!
//! Submodules:
//! - `brahman_memory`: Unified memory allocator (the One)
//! - `maya_overlay`: Type system as overlay on raw bytes (illusion)
//! - `atman_optimization`: Identity-based optimizations (self-recognition)

pub mod brahman_memory;
pub mod maya_overlay;
pub mod atman_optimization;

pub use brahman_memory::{BrahmanMemory, BrahmanHandle, AccessFrequency, LifetimeEstimate};
pub use maya_overlay::{MayaOverlay, MayaType, TypeKind, TransmuteResult};
pub use atman_optimization::{AtmanOptimizer, AtmanOptimization};

use std::collections::HashMap;

/// Advaita unified memory view
pub struct AdvaitaMemory {
    /// Memory regions (māyā - apparent distinctions)
    regions: HashMap<String, MemoryRegion>,
    /// Global view (Brahman - ultimate reality)
    global_state: GlobalMemoryState,
}

/// Memory region (apparent distinction)
#[derive(Debug, Clone)]
pub struct MemoryRegion {
    pub name: String,
    pub kind: RegionKind,
    pub size: usize,
    pub allocations: Vec<Allocation>,
}

/// Region kind
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionKind {
    /// Register (fastest, smallest)
    Register,
    /// Stack (fast, limited)
    Stack,
    /// Heap (flexible, slower)
    Heap,
    /// Arena (bulk allocation)
    Arena,
    /// Global (static lifetime)
    Global,
}

/// Memory allocation
#[derive(Debug, Clone)]
pub struct Allocation {
    pub name: String,
    pub size: usize,
    pub alignment: usize,
    pub region: String,
}

/// Global memory state (Brahman view)
#[derive(Debug, Clone)]
pub struct GlobalMemoryState {
    /// Total memory used
    pub total_used: usize,
    /// Peak memory usage
    pub peak_usage: usize,
    /// Active allocations count
    pub active_allocations: usize,
}

impl AdvaitaMemory {
    pub fn new() -> Self {
        Self {
            regions: HashMap::new(),
            global_state: GlobalMemoryState {
                total_used: 0,
                peak_usage: 0,
                active_allocations: 0,
            },
        }
    }

    /// Create a memory region
    pub fn create_region(&mut self, name: String, kind: RegionKind, size: usize) {
        self.regions.insert(name.clone(), MemoryRegion {
            name,
            kind,
            size,
            allocations: Vec::new(),
        });
    }

    /// Allocate memory
    /// The system decides optimal placement (māyā selection)
    pub fn allocate(&mut self, name: String, size: usize, alignment: usize) -> Option<AllocationHandle> {
        // Find best region (Advaita insight - all are ultimately the same)
        let region_name = self.select_region(size, alignment)?;

        let allocation = Allocation {
            name: name.clone(),
            size,
            alignment,
            region: region_name.clone(),
        };

        if let Some(region) = self.regions.get_mut(&region_name) {
            region.allocations.push(allocation);
        }

        self.global_state.total_used += size;
        self.global_state.active_allocations += 1;
        if self.global_state.total_used > self.global_state.peak_usage {
            self.global_state.peak_usage = self.global_state.total_used;
        }

        Some(AllocationHandle {
            name,
            region: region_name,
        })
    }

    /// Select optimal region for allocation
    fn select_region(&self, size: usize, alignment: usize) -> Option<String> {
        // Try in order of speed: register > stack > arena > heap

        // Small values might fit in registers
        if size <= 8 {
            if let Some((name, _)) = self.regions.iter().find(|(_, r)| r.kind == RegionKind::Register) {
                return Some(name.clone());
            }
        }

        // Medium values go to stack
        if size <= 4096 {
            if let Some((name, _)) = self.regions.iter().find(|(_, r)| r.kind == RegionKind::Stack) {
                return Some(name.clone());
            }
        }

        // Larger values go to heap or arena
        if let Some((name, _)) = self.regions.iter().find(|(_, r)| r.kind == RegionKind::Arena || r.kind == RegionKind::Heap) {
            return Some(name.clone());
        }

        None
    }

    /// Free memory
    pub fn free(&mut self, handle: &AllocationHandle) {
        if let Some(region) = self.regions.get_mut(&handle.region) {
            if let Some(pos) = region.allocations.iter().position(|a| a.name == handle.name) {
                let alloc = region.allocations.remove(pos);
                self.global_state.total_used -= alloc.size;
                self.global_state.active_allocations -= 1;
            }
        }
    }

    /// Get global state (Brahman view)
    pub fn global_view(&self) -> &GlobalMemoryState {
        &self.global_state
    }

    /// Realize unity - optimize by recognizing common patterns
    pub fn realize_unity(&mut self) {
        // Merge allocations that are always used together
        // This is the "enlightenment" optimization
        // TODO: Implement allocation coalescing
    }
}

/// Handle to an allocation
#[derive(Debug, Clone)]
pub struct AllocationHandle {
    pub name: String,
    pub region: String,
}

impl Default for AdvaitaMemory {
    fn default() -> Self {
        Self::new()
    }
}
