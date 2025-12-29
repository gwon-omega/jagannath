//! # Yama Dharmaraja - The Supreme Judge
//!
//! Static analyzer that judges code and assigns violations to Narakas.

use super::{Violation, ViolationKind, Yamaduta};
use super::yamadutas::{MemoryYamaduta, SecurityYamaduta, ConcurrencyYamaduta};
use crate::garuda::narakas::Naraka;
use crate::parser::ast::Ast;

/// Yama Dharmaraja - Judge of the dead (code analyzer)
pub struct YamaDharmaraja {
    /// Yamadutas (enforcement agents)
    yamadutas: Vec<Box<dyn Yamaduta>>,

    /// Dharma rules for judgment
    dharma_rules: Vec<DharmaRule>,
}

/// Rule for judging violations
pub struct DharmaRule {
    pub name: String,
    pub description: String,
    pub enabled: bool,
}

impl YamaDharmaraja {
    /// Create a new Yama with default Yamadutas
    pub fn new() -> Self {
        let mut yamadutas: Vec<Box<dyn Yamaduta>> = Vec::new();
        yamadutas.push(Box::new(MemoryYamaduta::new()));
        yamadutas.push(Box::new(SecurityYamaduta::new()));
        yamadutas.push(Box::new(ConcurrencyYamaduta::new()));

        Self {
            yamadutas,
            dharma_rules: Self::default_rules(),
        }
    }

    fn default_rules() -> Vec<DharmaRule> {
        vec![
            DharmaRule {
                name: "memory-safety".to_string(),
                description: "Check for memory safety violations".to_string(),
                enabled: true,
            },
            DharmaRule {
                name: "thread-safety".to_string(),
                description: "Check for thread safety violations".to_string(),
                enabled: true,
            },
            DharmaRule {
                name: "security".to_string(),
                description: "Check for security vulnerabilities".to_string(),
                enabled: true,
            },
        ]
    }

    /// Judge code and collect all violations
    pub fn judge(&self, ast: &Ast) -> Vec<Violation> {
        let mut violations = Vec::new();

        // Dispatch Yamadutas to inspect code
        for yamaduta in &self.yamadutas {
            let findings = yamaduta.inspect(ast);
            violations.extend(findings);
        }

        violations
    }

    /// Determine which Naraka a violation belongs to
    pub fn determine_naraka(&self, violation: &Violation) -> Naraka {
        match violation.kind {
            // Memory violations
            ViolationKind::UseAfterFree | ViolationKind::DoubleFree => Naraka::Tamisram,
            ViolationKind::NullDeref => Naraka::Andhakupa,
            ViolationKind::BufferOverflow => Naraka::Asipatravana,
            ViolationKind::MemoryCorruption => Naraka::Krimibhaksha,
            ViolationKind::MemoryLeak => Naraka::Suchimukha,
            ViolationKind::DanglingPointer => Naraka::Sarameyadana,
            ViolationKind::StackOverflow => Naraka::Avichi,

            // Concurrency violations
            ViolationKind::Deadlock => Naraka::Pranarodha,
            ViolationKind::RaceCondition => Naraka::Sandamsha,
            ViolationKind::ThreadUnsafe => Naraka::Kalasutra,
            ViolationKind::Starvation => Naraka::Dandasuka,

            // Security violations
            ViolationKind::TaintedData => Naraka::Vaitarani,
            ViolationKind::CodeInjection => Naraka::Raksogana,
            ViolationKind::InjectionAttack => Naraka::Sulaprota,
            ViolationKind::InsecureStorage => Naraka::Ksharakardama,
            ViolationKind::DataExposure => Naraka::Lalabhaksha,
            ViolationKind::PoisonedData => Naraka::Ayahpana,
            ViolationKind::DoS => Naraka::Vatarodha,
            ViolationKind::ResourceDenial => Naraka::Paryavartana,

            // Type violations
            ViolationKind::TypeConfusion => Naraka::Taptasurmi,
            ViolationKind::ContractViolation => Naraka::Andhatamisram,
            ViolationKind::FfiViolation => Naraka::Vajrakantaka,
            ViolationKind::DataCorruption => Naraka::Puyoda,

            // Resource violations
            ViolationKind::ResourceExhaustion => Naraka::Kumbhipaka,
            ViolationKind::ForcedTermination => Naraka::Visasana,
            ViolationKind::Panic => Naraka::Raurava,

            // Code quality
            ViolationKind::CodeSmell => Naraka::Sukaramukha,
        }
    }

    /// Add a custom Yamaduta
    pub fn add_yamaduta(&mut self, yamaduta: Box<dyn Yamaduta>) {
        self.yamadutas.push(yamaduta);
    }

    /// Enable or disable a dharma rule
    pub fn set_rule_enabled(&mut self, name: &str, enabled: bool) {
        for rule in &mut self.dharma_rules {
            if rule.name == name {
                rule.enabled = enabled;
                break;
            }
        }
    }
}

impl Default for YamaDharmaraja {
    fn default() -> Self {
        Self::new()
    }
}
