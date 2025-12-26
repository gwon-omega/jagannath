//! Brahman Memory - Unified Memory Allocator (ब्रह्मन्)
//!
//! "Sarvam khalvidam brahma" - All this is indeed Brahman
//!
//! The Brahman allocator provides a unified view of all memory:
//! - Registers, stack, heap, arena - all are manifestations of one memory
//! - Allocation decisions are made holistically
//! - Memory layout is optimized across all "apparent" distinctions

use super::{AllocationHandle, RegionKind};
use std::collections::HashMap;

/// Brahman - The unified memory allocator
/// Sees all memory as one undivided whole
pub struct BrahmanMemory {
    /// Total available memory (the One)
    total_capacity: usize,
    /// Current usage
    used: usize,
    /// Memory blocks (names and sizes)
    blocks: HashMap<String, BrahmanBlock>,
    /// Allocation strategy
    strategy: AllocationStrategy,
    /// Cache line size for optimization
    cache_line_size: usize,
}

/// A block of memory in the unified view
#[derive(Debug, Clone)]
pub struct BrahmanBlock {
    /// Block identifier
    pub id: String,
    /// Size in bytes
    pub size: usize,
    /// Alignment requirement
    pub alignment: usize,
    /// Access frequency (for placement optimization)
    pub access_frequency: AccessFrequency,
    /// Lifetime estimate
    pub lifetime: LifetimeEstimate,
    /// Current placement (māyā)
    pub placement: Option<Placement>,
}

/// How frequently is this memory accessed?
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccessFrequency {
    /// Constant (loop-invariant)
    Constant,
    /// Very hot (inner loop)
    Critical,
    /// Hot (frequently accessed)
    Hot,
    /// Warm (moderate access)
    Warm,
    /// Cold (rarely accessed)
    Cold,
    /// Dead (not accessed after this point)
    Dead,
}

/// Estimated lifetime
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifetimeEstimate {
    /// Single expression
    Instant,
    /// Single statement/block
    Block,
    /// Function scope
    Function,
    /// Cross-function (escapes)
    Escaped,
    /// Static/global
    Static,
}

/// Current placement of a block
#[derive(Debug, Clone)]
pub struct Placement {
    /// Which region (register, stack, heap, etc.)
    pub region: RegionKind,
    /// Offset within region
    pub offset: usize,
    /// Actual register (if in register)
    pub register: Option<String>,
}

/// Allocation strategy
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllocationStrategy {
    /// Optimize for speed (prefer registers/stack)
    Speed,
    /// Optimize for memory usage (compact)
    Memory,
    /// Balanced
    Balanced,
    /// Debug mode (all on heap for visibility)
    Debug,
}

impl BrahmanMemory {
    pub fn new(total_capacity: usize) -> Self {
        Self {
            total_capacity,
            used: 0,
            blocks: HashMap::new(),
            strategy: AllocationStrategy::Balanced,
            cache_line_size: 64, // Typical cache line
        }
    }

    /// Set allocation strategy
    pub fn set_strategy(&mut self, strategy: AllocationStrategy) {
        self.strategy = strategy;
    }

    /// Request memory from Brahman
    /// The unified allocator decides optimal placement
    pub fn request(&mut self, id: String, size: usize, alignment: usize) -> Option<BrahmanHandle> {
        if self.used + size > self.total_capacity {
            return None;
        }

        let block = BrahmanBlock {
            id: id.clone(),
            size,
            alignment,
            access_frequency: AccessFrequency::Warm, // Default
            lifetime: LifetimeEstimate::Function,    // Default
            placement: None,
        };

        self.blocks.insert(id.clone(), block);
        self.used += size;

        Some(BrahmanHandle { id })
    }

    /// Update access frequency for a block
    pub fn update_frequency(&mut self, handle: &BrahmanHandle, freq: AccessFrequency) {
        if let Some(block) = self.blocks.get_mut(&handle.id) {
            block.access_frequency = freq;
        }
    }

    /// Update lifetime estimate
    pub fn update_lifetime(&mut self, handle: &BrahmanHandle, lifetime: LifetimeEstimate) {
        if let Some(block) = self.blocks.get_mut(&handle.id) {
            block.lifetime = lifetime;
        }
    }

    /// Materialize - decide actual placement for all blocks
    /// This is the "creation" from Brahman's perspective
    pub fn materialize(&mut self) -> MaterializationResult {
        let mut placements = HashMap::new();
        let mut register_count = 0;
        let mut stack_offset = 0;
        let mut heap_allocations = Vec::new();

        // Sort by access frequency (hottest first)
        let mut sorted_blocks: Vec<_> = self.blocks.values().collect();
        sorted_blocks.sort_by(|a, b| b.access_frequency.cmp(&a.access_frequency));

        for block in sorted_blocks {
            let placement = self.decide_placement(block, register_count, stack_offset);

            match placement.region {
                RegionKind::Register => register_count += 1,
                RegionKind::Stack => stack_offset += self.align_size(block.size, block.alignment),
                RegionKind::Heap | RegionKind::Arena => {
                    heap_allocations.push(block.id.clone());
                }
                RegionKind::Global => {}
            }

            placements.insert(block.id.clone(), placement);
        }

        // Update placements in blocks
        for (id, placement) in &placements {
            if let Some(block) = self.blocks.get_mut(id) {
                block.placement = Some(placement.clone());
            }
        }

        MaterializationResult {
            total_registers: register_count,
            stack_size: stack_offset,
            heap_allocations: heap_allocations.len(),
            placements,
        }
    }

    /// Decide optimal placement for a block
    fn decide_placement(&self, block: &BrahmanBlock, reg_count: usize, stack_offset: usize) -> Placement {
        // Strategy-aware placement
        match self.strategy {
            AllocationStrategy::Debug => {
                // Everything on heap for debugging
                return Placement {
                    region: RegionKind::Heap,
                    offset: 0,
                    register: None,
                };
            }
            AllocationStrategy::Speed => {
                // Aggressively use registers
                if block.size <= 8 && reg_count < 16 {
                    return Placement {
                        region: RegionKind::Register,
                        offset: 0,
                        register: Some(format!("r{}", reg_count)),
                    };
                }
            }
            _ => {}
        }

        // Access-frequency based placement
        match block.access_frequency {
            AccessFrequency::Critical | AccessFrequency::Hot => {
                // Hot data → registers or L1-friendly stack
                if block.size <= 8 && reg_count < 14 {
                    Placement {
                        region: RegionKind::Register,
                        offset: 0,
                        register: Some(format!("r{}", reg_count)),
                    }
                } else {
                    Placement {
                        region: RegionKind::Stack,
                        offset: stack_offset,
                        register: None,
                    }
                }
            }
            AccessFrequency::Warm => {
                // Warm data → stack
                Placement {
                    region: RegionKind::Stack,
                    offset: stack_offset,
                    register: None,
                }
            }
            AccessFrequency::Cold | AccessFrequency::Constant => {
                // Cold/constant → heap or stack (based on lifetime)
                match block.lifetime {
                    LifetimeEstimate::Escaped | LifetimeEstimate::Static => {
                        Placement {
                            region: RegionKind::Heap,
                            offset: 0,
                            register: None,
                        }
                    }
                    _ => {
                        Placement {
                            region: RegionKind::Stack,
                            offset: stack_offset,
                            register: None,
                        }
                    }
                }
            }
            AccessFrequency::Dead => {
                // Dead code → don't allocate (optimize away)
                Placement {
                    region: RegionKind::Register,
                    offset: 0,
                    register: None, // Phantom allocation
                }
            }
        }
    }

    /// Align size to alignment boundary
    fn align_size(&self, size: usize, alignment: usize) -> usize {
        (size + alignment - 1) & !(alignment - 1)
    }

    /// Release memory back to Brahman
    pub fn release(&mut self, handle: &BrahmanHandle) {
        if let Some(block) = self.blocks.remove(&handle.id) {
            self.used -= block.size;
        }
    }

    /// Get statistics
    pub fn stats(&self) -> BrahmanStats {
        let mut register_blocks = 0;
        let mut stack_blocks = 0;
        let mut heap_blocks = 0;

        for block in self.blocks.values() {
            if let Some(p) = &block.placement {
                match p.region {
                    RegionKind::Register => register_blocks += 1,
                    RegionKind::Stack => stack_blocks += 1,
                    RegionKind::Heap | RegionKind::Arena => heap_blocks += 1,
                    RegionKind::Global => {}
                }
            }
        }

        BrahmanStats {
            total_capacity: self.total_capacity,
            used: self.used,
            block_count: self.blocks.len(),
            register_blocks,
            stack_blocks,
            heap_blocks,
        }
    }
}

/// Handle to Brahman-managed memory
#[derive(Debug, Clone)]
pub struct BrahmanHandle {
    pub id: String,
}

/// Result of materialization
#[derive(Debug)]
pub struct MaterializationResult {
    pub total_registers: usize,
    pub stack_size: usize,
    pub heap_allocations: usize,
    pub placements: HashMap<String, Placement>,
}

/// Brahman memory statistics
#[derive(Debug)]
pub struct BrahmanStats {
    pub total_capacity: usize,
    pub used: usize,
    pub block_count: usize,
    pub register_blocks: usize,
    pub stack_blocks: usize,
    pub heap_blocks: usize,
}

impl Default for BrahmanMemory {
    fn default() -> Self {
        Self::new(1024 * 1024 * 1024) // 1GB default
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_allocation() {
        let mut brahman = BrahmanMemory::new(1024);

        let handle = brahman.request("x".to_string(), 8, 8).unwrap();
        brahman.update_frequency(&handle, AccessFrequency::Hot);

        let result = brahman.materialize();
        assert!(result.placements.contains_key("x"));
    }

    #[test]
    fn test_hot_data_registers() {
        let mut brahman = BrahmanMemory::new(1024);
        brahman.set_strategy(AllocationStrategy::Speed);

        let handle = brahman.request("hot".to_string(), 8, 8).unwrap();
        brahman.update_frequency(&handle, AccessFrequency::Critical);

        let result = brahman.materialize();
        let placement = result.placements.get("hot").unwrap();
        assert_eq!(placement.region, RegionKind::Register);
    }
}
