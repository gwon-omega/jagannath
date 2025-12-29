//! Kāraka Analysis - Semantic Role Analysis
//!
//! Analyzes kāraka (semantic roles) for:
//! - Register allocation hints
//! - Memory layout optimization
//! - Aliasing analysis

use crate::parser::ast::{FunctionDef, Karaka, Parameter};
use std::collections::HashMap;

/// Kāraka analyzer
pub struct KarakaAnalyzer {
    /// Role assignments for each parameter
    role_assignments: HashMap<String, KarakaRole>,
}

/// Extended kāraka role with compiler hints
#[derive(Debug, Clone)]
pub struct KarakaRole {
    /// Base kāraka
    pub karaka: Karaka,
    /// Register allocation hint
    pub register_hint: RegisterHint,
    /// Aliasing properties
    pub aliasing: AliasingProperty,
    /// Mutability
    pub mutable: bool,
}

/// Register allocation hint based on kāraka
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegisterHint {
    /// Use callee-saved registers (preserve across calls)
    CalleeSaved,
    /// Use caller-saved registers (can be clobbered)
    CallerSaved,
    /// Use output registers
    Output,
    /// Use general-purpose registers
    GeneralPurpose,
}

/// Aliasing properties
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AliasingProperty {
    /// No aliasing possible (unique reference)
    NoAlias,
    /// May alias with other references
    MayAlias,
    /// Definitely aliases with another parameter
    Aliases(usize),
}

impl KarakaAnalyzer {
    pub fn new() -> Self {
        Self {
            role_assignments: HashMap::new(),
        }
    }

    /// Analyze a function's parameters for kāraka roles
    pub fn analyze_function(&mut self, func: &FunctionDef) -> Vec<KarakaRole> {
        let mut roles = Vec::new();

        for param in &func.params {
            let role = self.analyze_parameter(param);
            self.role_assignments
                .insert(param.name.name.clone(), role.clone());
            roles.push(role);
        }

        roles
    }

    /// Analyze a single parameter
    fn analyze_parameter(&self, param: &Parameter) -> KarakaRole {
        let karaka = param.karaka.unwrap_or(Karaka::Karman);

        let register_hint = match karaka {
            // Agent (doer) - preserve value, use callee-saved
            Karaka::Kartr => RegisterHint::CalleeSaved,
            // Patient (object) - may modify, use output registers
            Karaka::Karman => RegisterHint::Output,
            // Instrument (means) - consume, use caller-saved
            Karaka::Karana => RegisterHint::CallerSaved,
            // Recipient (beneficiary) - output location
            Karaka::Sampradana => RegisterHint::Output,
            // Source (origin) - read-only input
            Karaka::Apadana => RegisterHint::CalleeSaved,
            // Locus (location) - context, general purpose
            Karaka::Adhikarana => RegisterHint::GeneralPurpose,
        };

        let aliasing = match karaka {
            Karaka::Kartr => AliasingProperty::NoAlias,
            Karaka::Karana => AliasingProperty::NoAlias,
            _ => AliasingProperty::MayAlias,
        };

        let mutable = matches!(karaka, Karaka::Karman | Karaka::Sampradana);

        KarakaRole {
            karaka,
            register_hint,
            aliasing,
            mutable,
        }
    }

    /// Infer kāraka from vibhakti (case ending)
    pub fn infer_from_vibhakti(&self, vibhakti: Vibhakti) -> Option<Karaka> {
        match vibhakti {
            Vibhakti::Nominative => Some(Karaka::Kartr),
            Vibhakti::Accusative => Some(Karaka::Karman),
            Vibhakti::Instrumental => Some(Karaka::Karana),
            Vibhakti::Dative => Some(Karaka::Sampradana),
            Vibhakti::Ablative => Some(Karaka::Apadana),
            Vibhakti::Locative => Some(Karaka::Adhikarana),
            Vibhakti::Genitive | Vibhakti::Vocative => None,
        }
    }
}

/// Sanskrit vibhakti (case endings)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Vibhakti {
    /// Prathamā - Nominative (subject)
    Nominative,
    /// Dvitīyā - Accusative (direct object)
    Accusative,
    /// Tṛtīyā - Instrumental (by means of)
    Instrumental,
    /// Caturthī - Dative (to, for)
    Dative,
    /// Pañcamī - Ablative (from)
    Ablative,
    /// Ṣaṣṭhī - Genitive (of, 's)
    Genitive,
    /// Saptamī - Locative (in, at, on)
    Locative,
    /// Sambodhana - Vocative (direct address)
    Vocative,
}

impl Default for KarakaAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}
