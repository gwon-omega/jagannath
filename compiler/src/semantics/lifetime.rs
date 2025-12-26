//! Lifetime Checker
//!
//! Checks region/lifetime annotations (^N suffix) and ensures:
//! - No use-after-free
//! - Proper arena deallocation
//! - Lifetime subset relationships

use crate::parser::ast::*;
use std::collections::HashMap;

/// Lifetime/region checker
pub struct LifetimeChecker {
    /// Active regions
    regions: HashMap<u8, Region>,
    /// Current region stack
    region_stack: Vec<u8>,
}

/// Region information
#[derive(Debug, Clone)]
pub struct Region {
    pub id: u8,
    pub parent: Option<u8>,
    pub allocations: Vec<AllocationInfo>,
}

/// Allocation information
#[derive(Debug, Clone)]
pub struct AllocationInfo {
    pub name: String,
    pub ty: String,
    pub region: u8,
}

impl LifetimeChecker {
    pub fn new() -> Self {
        Self {
            regions: HashMap::new(),
            region_stack: vec![0], // Global region
        }
    }

    /// Check lifetimes for a function
    pub fn check_function(&mut self, func: &FunctionDef) -> Result<(), LifetimeError> {
        todo!("Implement lifetime checking")
    }

    /// Enter a new region scope
    pub fn enter_region(&mut self, region_id: u8) {
        let parent = self.region_stack.last().copied();
        self.regions.insert(region_id, Region {
            id: region_id,
            parent,
            allocations: Vec::new(),
        });
        self.region_stack.push(region_id);
    }

    /// Exit current region scope
    pub fn exit_region(&mut self) -> Vec<AllocationInfo> {
        if let Some(region_id) = self.region_stack.pop() {
            if let Some(region) = self.regions.remove(&region_id) {
                return region.allocations;
            }
        }
        Vec::new()
    }

    /// Record an allocation in the current region
    pub fn record_allocation(&mut self, name: String, ty: String, region: u8) {
        if let Some(r) = self.regions.get_mut(&region) {
            r.allocations.push(AllocationInfo { name, ty, region });
        }
    }

    /// Check if a reference outlives its region
    pub fn check_outlives(&self, reference_region: u8, value_region: u8) -> bool {
        // Reference region must be same or shorter-lived than value region
        let mut current = Some(reference_region);

        while let Some(r) = current {
            if r == value_region {
                return true;
            }
            current = self.regions.get(&r).and_then(|reg| reg.parent);
        }

        false
    }

    /// Get current region
    pub fn current_region(&self) -> u8 {
        *self.region_stack.last().unwrap_or(&0)
    }
}

/// Lifetime error
#[derive(Debug)]
pub enum LifetimeError {
    /// Reference outlives its referent
    OutlivesReferent {
        reference: String,
        referent: String,
    },
    /// Unknown region
    UnknownRegion(u8),
    /// Use after region end
    UseAfterRegionEnd {
        name: String,
        region: u8,
    },
}

impl Default for LifetimeChecker {
    fn default() -> Self {
        Self::new()
    }
}
