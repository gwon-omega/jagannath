//! # Mārga Selector - Automatic Path Detection
//!
//! Automatically selects the optimal optimization path based on code analysis.
//!
//! ## Philosophy
//!
//! "योग: कर्मसु कौशलम्" (Bhagavad Gita 2.50)
//! "Yoga is skill in action"
//!
//! The selector analyzes code characteristics and chooses the most
//! appropriate Marga for optimization.

use super::{
    bhakti::BhaktiMarga, jnana::JnanaMarga, karma::KarmaMarga, raja_yoga::RajaYogaMarga,
    CodeStyle, Domain, Marga, MargaOptimizer, MargaResult,
};
use crate::mir::types::MirFunction;

/// Automatic path selector
pub struct MargaSelector {
    /// Enable verbose logging
    verbose: bool,
    /// Preference weight for each path
    preferences: PathPreferences,
}

/// User preferences for path selection
#[derive(Debug, Clone)]
pub struct PathPreferences {
    /// Bias towards Karma (action)
    pub karma_bias: f32,
    /// Bias towards Jnana (knowledge)
    pub jnana_bias: f32,
    /// Bias towards Bhakti (devotion)
    pub bhakti_bias: f32,
    /// Bias towards Raja Yoga (balance)
    pub raja_bias: f32,
}

impl Default for PathPreferences {
    fn default() -> Self {
        Self {
            karma_bias: 1.0,
            jnana_bias: 1.0,
            bhakti_bias: 1.0,
            raja_bias: 1.0,
        }
    }
}

/// Analysis results for a function
#[derive(Debug, Clone)]
pub struct CodeAnalysis {
    /// Dominant code style detected
    pub style: CodeStyle,
    /// Detected domain (if any)
    pub domain: Option<Domain>,
    /// Imperative score (loops, mutations, side effects)
    pub imperative_score: f32,
    /// Functional score (pure functions, composition)
    pub functional_score: f32,
    /// Domain-specific score
    pub domain_score: f32,
    /// Complexity score
    pub complexity: f32,
}

impl Default for MargaSelector {
    fn default() -> Self {
        Self::new()
    }
}

impl MargaSelector {
    /// Create a new path selector
    pub fn new() -> Self {
        Self {
            verbose: false,
            preferences: PathPreferences::default(),
        }
    }

    /// Create with verbose logging
    pub fn verbose() -> Self {
        Self {
            verbose: true,
            preferences: PathPreferences::default(),
        }
    }

    /// Create with custom preferences
    pub fn with_preferences(preferences: PathPreferences) -> Self {
        Self {
            verbose: false,
            preferences,
        }
    }

    /// Analyze a function and detect code characteristics
    pub fn analyze(&self, func: &MirFunction) -> CodeAnalysis {
        let imperative_score = self.analyze_imperative(func);
        let functional_score = self.analyze_functional(func);
        let domain = self.detect_domain(func);
        let domain_score = if domain.is_some() { 0.8 } else { 0.0 };
        let complexity = self.analyze_complexity(func);

        let style = self.determine_style(imperative_score, functional_score, domain_score);

        CodeAnalysis {
            style,
            domain,
            imperative_score,
            functional_score,
            domain_score,
            complexity,
        }
    }

    /// Analyze imperative characteristics
    fn analyze_imperative(&self, func: &MirFunction) -> f32 {
        let mut score: f64 = 0.0;

        // Check for loops
        if func.name.contains("loop")
            || func.name.contains("while")
            || func.name.contains("for")
            || func.name.contains("iter")
        {
            score += 0.3;
        }

        // Check for mutations
        if func.name.contains("mut") || func.name.contains("set") || func.name.contains("update") {
            score += 0.3;
        }

        // Check for state
        if func.name.contains("state") || func.name.contains("machine") {
            score += 0.2;
        }

        // Check for side effects
        if func.name.contains("print")
            || func.name.contains("write")
            || func.name.contains("log")
            || func.name.contains("send")
        {
            score += 0.2;
        }

        score.min(1.0) as f32
    }

    /// Analyze functional characteristics
    fn analyze_functional(&self, func: &MirFunction) -> f32 {
        let mut score: f64 = 0.0;

        // Check for pure function indicators
        if func.name.contains("pure")
            || func.name.contains("const")
            || func.name.contains("immut")
            || func.name.contains("get")
        {
            score += 0.3;
        }

        // Check for higher-order patterns
        if func.name.contains("map")
            || func.name.contains("fold")
            || func.name.contains("filter")
            || func.name.contains("reduce")
        {
            score += 0.3;
        }

        // Check for composition
        if func.name.contains("compose")
            || func.name.contains("pipe")
            || func.name.contains("chain")
        {
            score += 0.2;
        }

        // Check for recursion
        if func.name.contains("recursive") || func.name.contains("rec") {
            score += 0.2;
        }

        score.min(1.0) as f32
    }

    /// Detect domain-specific code
    fn detect_domain(&self, func: &MirFunction) -> Option<Domain> {
        // GPU domain
        if func.name.contains("gpu")
            || func.name.contains("cuda")
            || func.name.contains("kernel")
            || func.name.contains("simd")
        {
            return Some(Domain::GPU);
        }

        // Embedded domain
        if func.name.contains("embedded")
            || func.name.contains("mcu")
            || func.name.contains("bare_metal")
            || func.name.contains("no_std")
        {
            return Some(Domain::Embedded);
        }

        // ML domain
        if func.name.contains("tensor")
            || func.name.contains("nn")
            || func.name.contains("layer")
            || func.name.contains("model")
        {
            return Some(Domain::MachineLearning);
        }

        // Network domain
        if func.name.contains("socket")
            || func.name.contains("http")
            || func.name.contains("tcp")
            || func.name.contains("async")
        {
            return Some(Domain::Network);
        }

        // DSL domain
        if func.name.contains("parse")
            || func.name.contains("ast")
            || func.name.contains("compile")
            || func.name.contains("eval")
        {
            return Some(Domain::DSL);
        }

        None
    }

    /// Analyze code complexity
    fn analyze_complexity(&self, func: &MirFunction) -> f32 {
        // Simple placeholder based on name length
        // Real implementation would analyze CFG
        let base = func.name.len() as f32 / 50.0;
        base.min(1.0)
    }

    /// Determine dominant code style
    fn determine_style(
        &self,
        imperative: f32,
        functional: f32,
        domain: f32,
    ) -> CodeStyle {
        if domain > 0.5 {
            return CodeStyle::DomainSpecific;
        }

        let diff = (imperative - functional).abs();
        if diff < 0.2 {
            return CodeStyle::Mixed;
        }

        if imperative > functional {
            CodeStyle::Imperative
        } else {
            CodeStyle::Functional
        }
    }

    /// Select the optimal path for a function
    pub fn select_path(&self, func: &MirFunction) -> Marga {
        let analysis = self.analyze(func);

        if self.verbose {
            eprintln!(
                "MargaSelector: Analyzing '{}' - Style: {:?}, Domain: {:?}",
                func.name, analysis.style, analysis.domain
            );
        }

        // Calculate scores for each path
        let karma_score = analysis.imperative_score * self.preferences.karma_bias;
        let jnana_score = analysis.functional_score * self.preferences.jnana_bias;
        let bhakti_score = analysis.domain_score * self.preferences.bhakti_bias;
        let raja_score = if analysis.style == CodeStyle::Mixed {
            0.7 * self.preferences.raja_bias
        } else {
            0.3 * self.preferences.raja_bias
        };

        if self.verbose {
            eprintln!(
                "  Scores: Karma={:.2}, Jnana={:.2}, Bhakti={:.2}, Raja={:.2}",
                karma_score, jnana_score, bhakti_score, raja_score
            );
        }

        // Select highest scoring path
        let max_score = karma_score
            .max(jnana_score)
            .max(bhakti_score)
            .max(raja_score);

        if max_score == bhakti_score && analysis.domain.is_some() {
            Marga::Bhakti
        } else if max_score == jnana_score {
            Marga::Jnana
        } else if max_score == karma_score {
            Marga::Karma
        } else {
            Marga::RajaYoga
        }
    }

    /// Optimize using automatically selected path
    pub fn optimize_auto(&self, func: &mut MirFunction) -> MargaResult {
        let path = self.select_path(func);

        if self.verbose {
            eprintln!("MargaSelector: Selected {:?} for '{}'", path, func.name);
        }

        match path {
            Marga::Karma => KarmaMarga::new().optimize(func),
            Marga::Jnana => JnanaMarga::new().optimize(func),
            Marga::Bhakti => {
                let domain = self.detect_domain(func).unwrap_or(Domain::General);
                BhaktiMarga::new(domain).optimize(func)
            }
            Marga::RajaYoga => RajaYogaMarga::new().optimize(func),
        }
    }
}

/// Builder pattern for configuring the selector
pub struct MargaSelectorBuilder {
    verbose: bool,
    preferences: PathPreferences,
}

impl MargaSelectorBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        Self {
            verbose: false,
            preferences: PathPreferences::default(),
        }
    }

    /// Enable verbose logging
    pub fn verbose(mut self, enable: bool) -> Self {
        self.verbose = enable;
        self
    }

    /// Set karma bias
    pub fn karma_bias(mut self, bias: f32) -> Self {
        self.preferences.karma_bias = bias;
        self
    }

    /// Set jnana bias
    pub fn jnana_bias(mut self, bias: f32) -> Self {
        self.preferences.jnana_bias = bias;
        self
    }

    /// Set bhakti bias
    pub fn bhakti_bias(mut self, bias: f32) -> Self {
        self.preferences.bhakti_bias = bias;
        self
    }

    /// Set raja yoga bias
    pub fn raja_bias(mut self, bias: f32) -> Self {
        self.preferences.raja_bias = bias;
        self
    }

    /// Build the selector
    pub fn build(self) -> MargaSelector {
        MargaSelector {
            verbose: self.verbose,
            preferences: self.preferences,
        }
    }
}

impl Default for MargaSelectorBuilder {
    fn default() -> Self {
        Self::new()
    }
}
