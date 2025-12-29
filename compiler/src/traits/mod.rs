//! # Shared Traits (Lakṣaṇa - लक्षण)
//!
//! Common traits used across all philosophical and cosmic modules.
//!
//! > **"लक्षणं प्रमाणम्"**
//! > *"Definition is the measure of truth"*
//!
//! This module provides unified abstractions for:
//! - Sanskrit naming conventions (SanskritNamed)
//! - Philosophical enumerations (PhilosophicalEnum)
//! - Cosmic patterns (CosmicPattern)
//! - Optimization strategies (OptimizationAdvice)
//!
//! ## Benefits
//! - Reduces code duplication across v3.0-v9.0 modules
//! - Ensures consistent naming patterns
//! - Enables generic algorithms over philosophical concepts
//! - Simplifies testing and documentation
//!
//! ## Architecture
//!
//! ```text
//! traits/
//! ├── naming.rs       # SanskritNamed, SanskritDescribed, CompilationDomain
//! ├── enumeration.rs  # PhilosophicalEnum, CategorizedVariant, CyclicVariant
//! ├── cosmic.rs       # CosmicPattern, CelestialBody, LunarMansion, ZodiacSign
//! └── optimization.rs # OptimizationAdvice, PurusharthaBalance, GunaMode, TapasRefinement
//! ```
//!
//! ## Usage Example
//!
//! ```rust
//! use jagannath_compiler::traits::{SanskritNamed, PhilosophicalEnum};
//!
//! // Generic function over any Sanskrit-named concept
//! fn display_info<T: SanskritNamed>(item: &T) {
//!     println!("{} ({}): {}", item.sanskrit(), item.iast(), item.english());
//! }
//!
//! // Generic function over any philosophical enumeration
//! fn list_all<T: PhilosophicalEnum>() {
//!     for (i, item) in T::all().iter().enumerate() {
//!         println!("{}: {}", i + 1, item.english());
//!     }
//! }
//! ```

pub mod cosmic;
pub mod enumeration;
pub mod naming;
pub mod optimization;

// ============================================================================
// Re-exports from naming.rs
// ============================================================================
pub use naming::{CelestialConcept, CompilationDomain, SanskritDescribed, SanskritNamed};

// ============================================================================
// Re-exports from enumeration.rs
// ============================================================================
pub use enumeration::{
    ActionableVariant, CategorizedVariant, CyclicVariant, GradedVariant, GroupedVariant,
    PhilosophicalEnum,
};

// ============================================================================
// Re-exports from cosmic.rs
// ============================================================================
pub use cosmic::{
    CelestialBody, CodePattern, CompilationContext, CompilationPhase, CosmicDomain, CosmicPattern,
    Element, InfluenceArea, LunarMansion, Modality, ResourceBudget, TargetCharacteristics,
    ZodiacSign,
};

// ============================================================================
// Re-exports from optimization.rs
// ============================================================================
pub use optimization::{
    CodeStyle, Guna, GunaMode, OptimizationAdvice, OptimizationContext, OptimizationDomain,
    OptimizationStrategy, OptimizationTechnique, PurusharthaBalance, RefinementResult,
    TapasRefinement, Tradeoff,
};
