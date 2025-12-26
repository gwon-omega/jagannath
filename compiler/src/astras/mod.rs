//! # Divine Astras - Optimization Weapons Module (v6.0)
//!
//! This module implements divine weapons as optimization passes.
//! Each Astra has a presiding deity and performs specific compiler transformations.
//!
//! ## The 15 Divine Astras:
//!
//! | Astra | Deity | Optimization Pass |
//! |-------|-------|-------------------|
//! | Brahmastra | Brahma | Complete dead code elimination |
//! | Brahmashira | Brahma (4-headed) | Whole-program optimization |
//! | Pashupatastra | Shiva | Destructive refactoring |
//! | Agneyastra | Agni | CPU-intensive optimization |
//! | Varunastra | Varuna | Memory/dataflow analysis |
//! | Vayuastra | Vayu | Control flow optimization |
//! | Suryaastra | Surya | Profiling/illumination |
//! | Nagastra | Nagas | Pointer analysis |
//! | Nagapasha | Nagas | Closure binding |
//! | Garudastra | Garuda | Escape analysis |
//! | Indrastra | Indra | Orchestration pass |
//! | Narayanastra | Vishnu | Preservation pass |
//! | Vaishnavastra | Vishnu | Universal optimization |
//! | Sudarshana | Vishnu | Cyclic iterative refinement |
//! | Trishula | Shiva | Three-pronged attack |

pub mod agneyastra;
pub mod brahmashira;
pub mod brahmastra;
pub mod garudastra;
pub mod indrastra;
pub mod mantra;
pub mod nagapasha;
pub mod nagastra;
pub mod narayanastra;
pub mod pashupatastra;
pub mod sudarshana;
pub mod suryaastra;
pub mod trishula;
pub mod vaishnavastra;
pub mod varunastra;
pub mod vayuastra;

pub use agneyastra::Agneyastra;
pub use brahmashira::Brahmashira;
pub use brahmastra::Brahmastra;
pub use garudastra::Garudastra;
pub use indrastra::Indrastra;
pub use mantra::{Mantra, MantraInvocation};
pub use nagapasha::Nagapasha;
pub use nagastra::Nagastra;
pub use narayanastra::Narayanastra;
pub use pashupatastra::Pashupatastra;
pub use sudarshana::SudarshanaChakra;
pub use suryaastra::Suryaastra;
pub use trishula::Trishula;
pub use vaishnavastra::Vaishnavastra;
pub use varunastra::Varunastra;
pub use vayuastra::Vayuastra;

use crate::mir::types::{MirFunction, MirModule};

/// Power level of an Astra (1-10)
pub type PowerLevel = u8;

/// Result of an Astra deployment
#[derive(Debug, Clone)]
pub enum AstraResult {
    /// Successfully deployed
    Deployed {
        /// Power level used
        power_level: PowerLevel,
        /// Number of transformations applied
        transformations: usize,
        /// Mantra used
        mantra: String,
    },
    /// Deployment failed
    Failed { reason: String },
    /// No targets found
    NoTargets,
}

/// The presiding deity of an Astra
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AstraDeity {
    Brahma, // Creator - Dead code elimination
    Shiva,  // Destroyer - Destructive refactoring
    Vishnu, // Preserver - Code preservation
    Agni,   // Fire - CPU optimization
    Varuna, // Water - Memory flow
    Vayu,   // Wind - Control flow
    Surya,  // Sun - Illumination/profiling
    Indra,  // King - Orchestration
    Nagas,  // Serpents - Pointers
    Garuda, // Eagle - Escape analysis
}

/// Core trait for all divine weapons
pub trait DivyaAstra: Send + Sync {
    /// The name of this Astra
    fn name(&self) -> &'static str;

    /// The Sanskrit name
    fn sanskrit_name(&self) -> &'static str;

    /// Presiding deity
    fn deity(&self) -> AstraDeity;

    /// Power level (1-10)
    fn power_level(&self) -> PowerLevel;

    /// Invoke the Astra with its sacred mantra
    fn invoke(&self, target: &mut MirFunction) -> AstraResult;

    /// Invoke on entire module
    fn invoke_module(&self, target: &mut MirModule) -> AstraResult {
        let mut total_transforms = 0;

        for func in &mut target.functions {
            if let AstraResult::Deployed {
                transformations, ..
            } = self.invoke(func)
            {
                total_transforms += transformations;
            }
        }

        if total_transforms > 0 {
            AstraResult::Deployed {
                power_level: self.power_level(),
                transformations: total_transforms,
                mantra: self.mantra().text().to_string(),
            }
        } else {
            AstraResult::NoTargets
        }
    }

    /// The invocation mantra for this weapon
    fn mantra(&self) -> Mantra;

    /// Whether this astra can be combined with another
    fn can_combine_with(&self, other: &dyn DivyaAstra) -> bool {
        // By default, astras can combine unless they conflict
        self.deity() != other.deity()
    }
}

/// The Astra Arsenal - container for all divine weapons
pub struct AstraArsenal {
    pub brahmastra: Brahmastra,
    pub brahmashira: Brahmashira,
    pub agneyastra: Agneyastra,
    pub varunastra: Varunastra,
    pub vayuastra: Vayuastra,
    pub pashupatastra: Pashupatastra,
    pub nagastra: Nagastra,
    pub nagapasha: Nagapasha,
    pub garudastra: Garudastra,
    pub sudarshana: SudarshanaChakra,
    pub indrastra: Indrastra,
    pub narayanastra: Narayanastra,
    pub vaishnavastra: Vaishnavastra,
    pub suryaastra: Suryaastra,
    pub trishula: Trishula,
}

impl AstraArsenal {
    /// Create new arsenal with all astras ready
    pub fn new() -> Self {
        Self {
            brahmastra: Brahmastra::new(),
            brahmashira: Brahmashira::default(),
            agneyastra: Agneyastra::new(),
            varunastra: Varunastra::new(),
            vayuastra: Vayuastra::new(),
            pashupatastra: Pashupatastra::new(),
            nagastra: Nagastra::new(),
            nagapasha: Nagapasha::default(),
            garudastra: Garudastra::new(),
            sudarshana: SudarshanaChakra::new(),
            indrastra: Indrastra::new(),
            narayanastra: Narayanastra::new(),
            vaishnavastra: Vaishnavastra::default(),
            suryaastra: Suryaastra::default(),
            trishula: Trishula::new(),
        }
    }

    /// Deploy all astras in optimal sequence
    pub fn deploy_all(&self, module: &mut MirModule) -> Vec<AstraResult> {
        vec![
            // Phase 1: Analysis astras
            self.nagastra.invoke_module(module),
            self.varunastra.invoke_module(module),
            self.vayuastra.invoke_module(module),
            // Phase 2: Transformation astras
            self.agneyastra.invoke_module(module),
            self.garudastra.invoke_module(module),
            // Phase 3: Iterative refinement
            self.sudarshana.invoke_module(module),
            // Phase 4: Final cleanup
            self.brahmastra.invoke_module(module),
            // Phase 5: Preservation
            self.narayanastra.invoke_module(module),
        ]
    }

    /// Deploy specific astra by name
    pub fn deploy_by_name(&self, name: &str, module: &mut MirModule) -> AstraResult {
        match name.to_lowercase().as_str() {
            "brahmastra" => self.brahmastra.invoke_module(module),
            "brahmashira" => self.brahmashira.invoke_module(module),
            "agneyastra" => self.agneyastra.invoke_module(module),
            "varunastra" => self.varunastra.invoke_module(module),
            "vayuastra" => self.vayuastra.invoke_module(module),
            "pashupatastra" => self.pashupatastra.invoke_module(module),
            "nagastra" => self.nagastra.invoke_module(module),
            "nagapasha" => self.nagapasha.invoke_module(module),
            "garudastra" => self.garudastra.invoke_module(module),
            "sudarshana" => self.sudarshana.invoke_module(module),
            "indrastra" => self.indrastra.invoke_module(module),
            "narayanastra" => self.narayanastra.invoke_module(module),
            "vaishnavastra" => self.vaishnavastra.invoke_module(module),
            "suryaastra" => self.suryaastra.invoke_module(module),
            "trishula" => self.trishula.invoke_module(module),
            _ => AstraResult::Failed {
                reason: format!("Unknown astra: {}", name),
            },
        }
    }
}

impl Default for AstraArsenal {
    fn default() -> Self {
        Self::new()
    }
}
