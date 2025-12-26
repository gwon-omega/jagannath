//! # Cosmic Compiler - Life-Aligned Compilation
//!
//! The highest-level coordinator for v7.0 systems.
//! Integrates Marga, Varna, and Purushartha for optimal compilation.
//!
//! ## Philosophy
//!
//! "सर्वं खल्विदं ब्रह्म" (Chandogya Upanishad 3.14.1)
//! "All this is indeed Brahman"
//!
//! The cosmic compiler sees the code holistically - understanding its
//! path (Marga), privileges (Varna), and goals (Purushartha).
//! When all align, Moksha (liberation/optimal compilation) is achieved.

use super::{
    marga_varna_bridge::MargaVarnaBridge, purushartha_balancer::PurusharthaBalancer,
    LifeAlignedResult, LifeAlignment,
};
use crate::margas::{path_selector::MargaSelector, Marga};
use crate::mir::types::MirFunction;
use crate::purusharthas::{OptimizationMetrics, PurusharthaWeights};
use crate::varnas::Varna;

/// The Cosmic Compiler - coordinates all v7.0 systems
pub struct CosmicCompiler {
    /// Marga-Varna bridge
    bridge: MargaVarnaBridge,
    /// Purushartha balancer
    balancer: PurusharthaBalancer,
    /// Path selector
    path_selector: MargaSelector,
    /// Current alignment
    alignment: LifeAlignment,
    /// Verbose output
    verbose: bool,
}

impl Default for CosmicCompiler {
    fn default() -> Self {
        Self::new()
    }
}

impl CosmicCompiler {
    /// Create a new cosmic compiler
    pub fn new() -> Self {
        Self {
            bridge: MargaVarnaBridge::new(),
            balancer: PurusharthaBalancer::new(),
            path_selector: MargaSelector::new(),
            alignment: LifeAlignment::default(),
            verbose: false,
        }
    }

    /// Create with verbose output
    pub fn verbose() -> Self {
        Self {
            bridge: MargaVarnaBridge::new(),
            balancer: PurusharthaBalancer::new(),
            path_selector: MargaSelector::verbose(),
            alignment: LifeAlignment::default(),
            verbose: true,
        }
    }

    /// Create with specific alignment
    pub fn with_alignment(alignment: LifeAlignment) -> Self {
        let balancer = PurusharthaBalancer::with_context(alignment.varna, alignment.marga);
        Self {
            bridge: MargaVarnaBridge::new(),
            balancer,
            path_selector: MargaSelector::new(),
            alignment,
            verbose: false,
        }
    }

    /// Get current alignment
    pub fn alignment(&self) -> &LifeAlignment {
        &self.alignment
    }

    /// Set alignment
    pub fn set_alignment(&mut self, alignment: LifeAlignment) {
        self.alignment = alignment.clone();
        self.balancer = PurusharthaBalancer::with_context(alignment.varna, alignment.marga);
    }

    /// Auto-detect optimal alignment for a function
    pub fn auto_align(&mut self, func: &MirFunction) -> LifeAlignment {
        // 1. Detect Marga from code style
        let marga = self.path_selector.select_path(func);

        // 2. Detect Varna from function characteristics
        let varna = self.detect_varna(func);

        // 3. Calculate Purushartha weights
        let purushartha = PurusharthaBalancer::weights_for_varna_marga(varna, marga);

        let alignment = LifeAlignment::new(marga, varna, purushartha);

        if self.verbose {
            eprintln!(
                "CosmicCompiler: Auto-aligned '{}' to {:?} path, {} varna",
                func.name,
                marga,
                varna.sanskrit_name()
            );
        }

        self.alignment = alignment.clone();
        alignment
    }

    /// Detect appropriate Varna from function characteristics
    fn detect_varna(&self, func: &MirFunction) -> Varna {
        // Check for kernel-level indicators
        if func.name.contains("kernel")
            || func.name.contains("interrupt")
            || func.name.contains("page_")
            || func.name.starts_with("kern_")
        {
            return Varna::Brahmin;
        }

        // Check for system-level indicators
        if func.name.contains("driver")
            || func.name.contains("service")
            || func.name.starts_with("sys_")
            || func.name.contains("daemon")
        {
            return Varna::Kshatriya;
        }

        // Check for sandboxed indicators
        if func.name.contains("sandbox")
            || func.name.contains("untrusted")
            || func.name.contains("plugin")
            || func.name.starts_with("wasm_")
        {
            return Varna::Shudra;
        }

        // Default to user mode
        Varna::Vaishya
    }

    /// Compile a function with life alignment
    pub fn compile(
        &mut self,
        func: &mut MirFunction,
        baseline: &OptimizationMetrics,
    ) -> LifeAlignedResult {
        // 1. Auto-align if needed
        if !self.alignment.is_valid() {
            self.auto_align(func);
        }

        if self.verbose {
            eprintln!(
                "CosmicCompiler: Compiling '{}' with alignment: {}",
                func.name,
                self.alignment.strategy_description()
            );
        }

        // 2. Check Varna privileges
        let varna_result = self.bridge.varna().analyze_function(func);
        let varna_violations = varna_result.len();

        // 3. Optimize with Marga
        let marga_result = self
            .bridge
            .optimize_with_checks(func, self.alignment.marga, self.alignment.varna);

        // 4. Balance Purusharthas
        let balancer_result = self.balancer.optimize(func, baseline);

        // 5. Check for Moksha
        let moksha_achieved = balancer_result.moksha_achieved()
            && varna_violations == 0
            && marga_result.is_clean();

        // 6. Collect messages
        let mut messages = Vec::new();

        if !marga_result.warnings.is_empty() {
            messages.extend(marga_result.warnings);
        }

        if !balancer_result.is_well_aligned() {
            messages.push(format!(
                "Alignment warning: Varna={:.0}%, Marga={:.0}%",
                balancer_result.varna_alignment * 100.0,
                balancer_result.marga_alignment * 100.0
            ));
        }

        messages.extend(self.balancer.get_recommendations());

        // Update alignment with Moksha status
        let mut alignment = self.alignment.clone();
        if moksha_achieved {
            alignment.achieve_moksha();
        }

        LifeAlignedResult {
            alignment,
            marga_result: format!("{:?}", marga_result.marga_result),
            varna_violations,
            artha_score: balancer_result.triangle_result.artha_score,
            kama_score: balancer_result.triangle_result.kama_score,
            dharma_score: balancer_result.triangle_result.dharma_score,
            success: varna_violations == 0,
            messages,
        }
    }

    /// Compile for specific profile
    pub fn compile_for_profile(
        &mut self,
        func: &mut MirFunction,
        baseline: &OptimizationMetrics,
        profile: CompilationProfile,
    ) -> LifeAlignedResult {
        let alignment = match profile {
            CompilationProfile::Debug => LifeAlignment {
                marga: Marga::Jnana,
                varna: Varna::Vaishya,
                purushartha: PurusharthaWeights::dharma_focused(),
                moksha_achieved: false,
            },
            CompilationProfile::Release => LifeAlignment {
                marga: Marga::RajaYoga,
                varna: Varna::Vaishya,
                purushartha: PurusharthaWeights::kama_focused(),
                moksha_achieved: false,
            },
            CompilationProfile::Size => LifeAlignment {
                marga: Marga::Bhakti,
                varna: Varna::Vaishya,
                purushartha: PurusharthaWeights::artha_focused(),
                moksha_achieved: false,
            },
            CompilationProfile::Safety => LifeAlignment {
                marga: Marga::Jnana,
                varna: Varna::Vaishya,
                purushartha: PurusharthaWeights::dharma_focused(),
                moksha_achieved: false,
            },
            CompilationProfile::Kernel => LifeAlignment::kernel(),
            CompilationProfile::Embedded => LifeAlignment::embedded(),
            CompilationProfile::Sandboxed => LifeAlignment::sandboxed(),
        };

        self.set_alignment(alignment);
        self.compile(func, baseline)
    }

    /// Achieve Moksha - iteratively optimize until balanced
    pub fn achieve_moksha(
        &mut self,
        func: &mut MirFunction,
        baseline: &OptimizationMetrics,
        max_iterations: usize,
    ) -> LifeAlignedResult {
        let mut best_result = self.compile(func, baseline);

        if best_result.achieved_moksha() {
            return best_result;
        }

        // Try different alignments
        let alignments = [
            LifeAlignment::balanced(),
            LifeAlignment::new(Marga::RajaYoga, Varna::Vaishya, PurusharthaWeights::balanced()),
            LifeAlignment::new(Marga::Karma, Varna::Vaishya, PurusharthaWeights::balanced()),
            LifeAlignment::new(Marga::Jnana, Varna::Vaishya, PurusharthaWeights::balanced()),
        ];

        for (i, alignment) in alignments.iter().enumerate() {
            if i >= max_iterations {
                break;
            }

            self.set_alignment(alignment.clone());
            let result = self.compile(func, baseline);

            if result.achieved_moksha() {
                return result;
            }

            // Keep best result
            let current_score = result.artha_score + result.kama_score + result.dharma_score;
            let best_score =
                best_result.artha_score + best_result.kama_score + best_result.dharma_score;

            if current_score > best_score {
                best_result = result;
            }
        }

        best_result
    }
}

/// Predefined compilation profiles
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompilationProfile {
    /// Debug build - prioritize safety and debuggability
    Debug,
    /// Release build - balanced with performance focus
    Release,
    /// Size-optimized build
    Size,
    /// Safety-optimized build
    Safety,
    /// Kernel/OS build
    Kernel,
    /// Embedded/constrained build
    Embedded,
    /// Sandboxed/untrusted build
    Sandboxed,
}

impl CompilationProfile {
    /// Get all profiles
    pub fn all() -> &'static [CompilationProfile] {
        &[
            CompilationProfile::Debug,
            CompilationProfile::Release,
            CompilationProfile::Size,
            CompilationProfile::Safety,
            CompilationProfile::Kernel,
            CompilationProfile::Embedded,
            CompilationProfile::Sandboxed,
        ]
    }

    /// Get description
    pub fn description(&self) -> &'static str {
        match self {
            CompilationProfile::Debug => "Debug: Safety-focused, debuggable",
            CompilationProfile::Release => "Release: Balanced with performance focus",
            CompilationProfile::Size => "Size: Minimize binary size and memory",
            CompilationProfile::Safety => "Safety: Maximum correctness guarantees",
            CompilationProfile::Kernel => "Kernel: OS/ring-0 code",
            CompilationProfile::Embedded => "Embedded: Resource-constrained targets",
            CompilationProfile::Sandboxed => "Sandboxed: Untrusted/plugin code",
        }
    }
}
