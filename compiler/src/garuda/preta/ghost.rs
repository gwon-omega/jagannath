//! # Hungry Ghost
//!
//! Representation of a leaked resource.

use super::ResourceType;
use super::detector::HungerLevel;
use crate::errors::Span;

/// A hungry ghost - an unfreed resource
#[derive(Debug, Clone)]
pub struct HungryGhost {
    /// What type of resource
    pub resource_type: ResourceType,
    /// Where it was born (allocated)
    pub allocation_site: Span,
    /// Variable name
    pub variable_name: String,
    /// How severe the leak is
    pub hunger_level: HungerLevel,
}

impl HungryGhost {
    /// Get a description of this ghost
    pub fn describe(&self) -> String {
        format!(
            "Hungry ghost '{}' ({}) wandering since allocation. Feed it with {}().",
            self.variable_name,
            self.resource_type.sanskrit_name(),
            self.resource_type.cleanup_function()
        )
    }

    /// Get redemption advice
    pub fn redemption(&self) -> String {
        format!(
            "To release this preta, call {}() on '{}' before it goes out of scope.",
            self.resource_type.cleanup_function(),
            self.variable_name
        )
    }
}
