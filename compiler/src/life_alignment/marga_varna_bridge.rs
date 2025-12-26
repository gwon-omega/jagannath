//! # Marga-Varna Bridge
//!
//! Connects optimization paths (Marga) with privilege levels (Varna).
//!
//! ## Philosophy
//!
//! Each Marga has natural affinity with certain Varnas:
//! - Karma (action) ↔ Brahmin/Kshatriya (system-level work)
//! - Jnana (knowledge) ↔ Vaishya/Shudra (pure computation)
//! - Bhakti (devotion) ↔ Any (domain-specific)
//! - Raja Yoga (balance) ↔ Any (universal)

use crate::margas::{CaturMarga, Marga, MargaResult};
use crate::mir::types::MirFunction;
use crate::varnas::{Varna, VarnaChecker, VarnaViolation};

/// Bridge between Marga (paths) and Varna (privileges)
pub struct MargaVarnaBridge {
    /// Marga system
    marga_system: CaturMarga,
    /// Varna checker
    varna_checker: VarnaChecker,
    /// Affinity matrix
    affinities: MargaVarnaAffinity,
}

/// Affinity scores between Margas and Varnas
#[derive(Debug, Clone)]
pub struct MargaVarnaAffinity {
    /// Karma path affinities (action/imperative)
    pub karma: VarnaAffinities,
    /// Jnana path affinities (knowledge/functional)
    pub jnana: VarnaAffinities,
    /// Bhakti path affinities (devotion/domain)
    pub bhakti: VarnaAffinities,
    /// Raja Yoga path affinities (balance)
    pub raja: VarnaAffinities,
}

/// Affinity scores for each Varna
#[derive(Debug, Clone, Copy)]
pub struct VarnaAffinities {
    /// Brahmin (kernel) affinity
    pub brahmin: f32,
    /// Kshatriya (system) affinity
    pub kshatriya: f32,
    /// Vaishya (user) affinity
    pub vaishya: f32,
    /// Shudra (sandbox) affinity
    pub shudra: f32,
}

impl Default for MargaVarnaAffinity {
    fn default() -> Self {
        Self {
            // Karma path: good for action-oriented system code
            karma: VarnaAffinities {
                brahmin: 0.9,   // Kernel code is very action-oriented
                kshatriya: 0.8, // System services too
                vaishya: 0.5,   // User code can be imperative
                shudra: 0.3,    // Sandboxed code has limited actions
            },
            // Jnana path: good for pure, functional code
            jnana: VarnaAffinities {
                brahmin: 0.3,   // Kernel rarely pure functional
                kshatriya: 0.4, // System services sometimes
                vaishya: 0.8,   // User code can be functional
                shudra: 0.9,    // Sandboxed code should be pure
            },
            // Bhakti path: domain-specific, works everywhere
            bhakti: VarnaAffinities {
                brahmin: 0.7,   // Domain: hardware drivers
                kshatriya: 0.7, // Domain: system services
                vaishya: 0.7,   // Domain: applications
                shudra: 0.7,    // Domain: plugins
            },
            // Raja Yoga: balanced, universal
            raja: VarnaAffinities {
                brahmin: 0.7,
                kshatriya: 0.7,
                vaishya: 0.7,
                shudra: 0.7,
            },
        }
    }
}

impl MargaVarnaAffinity {
    /// Get affinity score for a Marga-Varna pair
    pub fn get_affinity(&self, marga: Marga, varna: Varna) -> f32 {
        let affinities = match marga {
            Marga::Karma => &self.karma,
            Marga::Jnana => &self.jnana,
            Marga::Bhakti => &self.bhakti,
            Marga::RajaYoga => &self.raja,
        };

        match varna {
            Varna::Brahmin => affinities.brahmin,
            Varna::Kshatriya => affinities.kshatriya,
            Varna::Vaishya => affinities.vaishya,
            Varna::Shudra => affinities.shudra,
        }
    }

    /// Get best Varna for a Marga
    pub fn best_varna_for(&self, marga: Marga) -> Varna {
        let affinities = match marga {
            Marga::Karma => &self.karma,
            Marga::Jnana => &self.jnana,
            Marga::Bhakti => &self.bhakti,
            Marga::RajaYoga => &self.raja,
        };

        // Find max affinity
        let max = affinities
            .brahmin
            .max(affinities.kshatriya)
            .max(affinities.vaishya)
            .max(affinities.shudra);

        if max == affinities.brahmin {
            Varna::Brahmin
        } else if max == affinities.kshatriya {
            Varna::Kshatriya
        } else if max == affinities.vaishya {
            Varna::Vaishya
        } else {
            Varna::Shudra
        }
    }

    /// Get best Marga for a Varna
    pub fn best_marga_for(&self, varna: Varna) -> Marga {
        let scores = [
            (Marga::Karma, self.get_affinity(Marga::Karma, varna)),
            (Marga::Jnana, self.get_affinity(Marga::Jnana, varna)),
            (Marga::Bhakti, self.get_affinity(Marga::Bhakti, varna)),
            (Marga::RajaYoga, self.get_affinity(Marga::RajaYoga, varna)),
        ];

        scores
            .iter()
            .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap())
            .map(|(m, _)| *m)
            .unwrap_or(Marga::RajaYoga)
    }
}

impl Default for MargaVarnaBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl MargaVarnaBridge {
    /// Create a new bridge
    pub fn new() -> Self {
        Self {
            marga_system: CaturMarga::new(),
            varna_checker: VarnaChecker::new(),
            affinities: MargaVarnaAffinity::default(),
        }
    }

    /// Get the Marga system
    pub fn marga(&mut self) -> &mut CaturMarga {
        &mut self.marga_system
    }

    /// Get the Varna checker
    pub fn varna(&mut self) -> &mut VarnaChecker {
        &mut self.varna_checker
    }

    /// Get affinity between Marga and Varna
    pub fn affinity(&self, marga: Marga, varna: Varna) -> f32 {
        self.affinities.get_affinity(marga, varna)
    }

    /// Check if a Marga-Varna combination is recommended
    pub fn is_recommended(&self, marga: Marga, varna: Varna) -> bool {
        self.affinity(marga, varna) >= 0.5
    }

    /// Get warnings for low-affinity combinations
    pub fn get_warnings(&self, marga: Marga, varna: Varna) -> Vec<String> {
        let mut warnings = Vec::new();
        let affinity = self.affinity(marga, varna);

        if affinity < 0.5 {
            warnings.push(format!(
                "Low affinity ({:.0}%) between {:?} path and {} varna",
                affinity * 100.0,
                marga,
                varna.sanskrit_name()
            ));

            let recommended_varna = self.affinities.best_varna_for(marga);
            let recommended_marga = self.affinities.best_marga_for(varna);

            warnings.push(format!(
                "Consider: {:?} path works better with {} varna",
                marga,
                recommended_varna.sanskrit_name()
            ));
            warnings.push(format!(
                "Consider: {} varna works better with {:?} path",
                varna.sanskrit_name(),
                recommended_marga
            ));
        }

        warnings
    }

    /// Optimize function with both Marga and Varna checking
    pub fn optimize_with_checks(
        &mut self,
        func: &mut MirFunction,
        marga: Marga,
        varna: Varna,
    ) -> BridgeResult {
        let mut warnings = self.get_warnings(marga, varna);

        // Check Varna violations
        let varna_violations = self.varna_checker.analyze_function(func);

        // Optimize with selected Marga
        let marga_result = self.marga_system.optimize_with_marga(func, marga);

        // Check for privilege issues
        if let Err(ref e) = self.varna_checker.check_privilege(varna) {
            warnings.push(format!("Privilege warning: {}", e));
        }

        BridgeResult {
            marga_result,
            varna_violations,
            affinity: self.affinity(marga, varna),
            warnings,
            recommended_marga: self.affinities.best_marga_for(varna),
            recommended_varna: self.affinities.best_varna_for(marga),
        }
    }

    /// Auto-select best Marga for current Varna
    pub fn auto_optimize(&mut self, func: &mut MirFunction) -> BridgeResult {
        let varna = self.varna_checker.current_varna();
        let marga = self.affinities.best_marga_for(varna);

        self.optimize_with_checks(func, marga, varna)
    }
}

/// Result of bridge optimization
#[derive(Debug)]
pub struct BridgeResult {
    /// Marga optimization result
    pub marga_result: MargaResult,
    /// Varna violations found
    pub varna_violations: Vec<VarnaViolation>,
    /// Affinity score for the combination
    pub affinity: f32,
    /// Warnings generated
    pub warnings: Vec<String>,
    /// Recommended Marga for current Varna
    pub recommended_marga: Marga,
    /// Recommended Varna for current Marga
    pub recommended_varna: Varna,
}

impl BridgeResult {
    /// Check if result is clean (no violations, high affinity)
    pub fn is_clean(&self) -> bool {
        self.varna_violations.is_empty() && self.affinity >= 0.7 && self.marga_result.success
    }
}
