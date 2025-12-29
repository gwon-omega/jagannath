//! Śabda - Verbal Testimony (शब्द)
//!
//! The third pramāṇa: knowledge from authoritative testimony.
//! In programming: documentation, contracts, and API specifications.
//!
//! Sources of śabda:
//! 1. Doc comments (/// and //!)
//! 2. Type contracts (@type, @returns, @param)
//! 3. Design-by-contract annotations (yatra = requires, phala = ensures)
//! 4. External documentation (API specs, RFCs)

use super::{Pramana, TypeEvidence};
use std::collections::HashMap;

/// Śabda documentation analyzer
pub struct ShabdaAnalyzer {
    /// Parsed contracts
    contracts: HashMap<String, Contract>,
    /// Type hints from documentation
    doc_hints: HashMap<String, Vec<DocHint>>,
    /// Āpta (authoritative sources)
    apta_sources: Vec<AptaSource>,
}

/// Contract from documentation
#[derive(Debug, Clone)]
pub struct Contract {
    /// Function/type name
    pub name: String,
    /// Parameter types
    pub params: Vec<ParamContract>,
    /// Return type
    pub returns: Option<String>,
    /// Preconditions (yatra = requires)
    pub preconditions: Vec<Condition>,
    /// Postconditions (phala = ensures)
    pub postconditions: Vec<Condition>,
    /// Certainty of the documentation
    pub certainty: f32,
}

/// Parameter contract
#[derive(Debug, Clone)]
pub struct ParamContract {
    /// Parameter name
    pub name: String,
    /// Expected type
    pub type_name: String,
    /// Role (kāraka)
    pub karaka: Option<String>,
    /// Description
    pub description: String,
}

/// Condition (pre/post)
#[derive(Debug, Clone)]
pub struct Condition {
    /// Condition expression
    pub expression: String,
    /// Human description
    pub description: String,
    /// Is this verifiable at compile time?
    pub compile_time: bool,
}

/// Documentation hint
#[derive(Debug, Clone)]
pub struct DocHint {
    /// Hint type
    pub kind: DocHintKind,
    /// Value
    pub value: String,
    /// Source line
    pub line: u32,
}

/// Types of documentation hints
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocHintKind {
    /// @type annotation
    TypeAnnotation,
    /// @returns annotation
    Returns,
    /// @param annotation
    Param,
    /// @throws/@error annotation
    Throws,
    /// Example code
    Example,
    /// Generic type hint
    Generic,
}

/// Āpta - Authoritative source
#[derive(Debug, Clone)]
pub struct AptaSource {
    /// Source name (e.g., "std::io")
    pub name: String,
    /// Type signatures from this source
    pub signatures: HashMap<String, TypeSignature>,
    /// Trust level (0.0 - 1.0)
    pub trust_level: f32,
}

/// Type signature from āpta
#[derive(Debug, Clone)]
pub struct TypeSignature {
    pub params: Vec<String>,
    pub returns: Option<String>,
}

impl ShabdaAnalyzer {
    pub fn new() -> Self {
        let mut analyzer = Self {
            contracts: HashMap::new(),
            doc_hints: HashMap::new(),
            apta_sources: Vec::new(),
        };
        analyzer.add_stdlib_apta();
        analyzer
    }

    /// Add standard library as āpta source
    fn add_stdlib_apta(&mut self) {
        let mut stdlib_sigs = HashMap::new();

        // mudraṇa (print) - returns unit
        stdlib_sigs.insert("mudraṇa".to_string(), TypeSignature {
            params: vec!["Sūtra".to_string()],
            returns: Some("()".to_string()),
        });

        // paṭha (read) - returns Result<Sūtra, Truṭi>
        stdlib_sigs.insert("paṭha".to_string(), TypeSignature {
            params: vec!["Sūtra".to_string()], // filename
            returns: Some("Phala<Sūtra, Truṭi>".to_string()),
        });

        // likha (write) - returns Result<(), Truṭi>
        stdlib_sigs.insert("likha".to_string(), TypeSignature {
            params: vec!["Sūtra".to_string(), "Sūtra".to_string()],
            returns: Some("Phala<(), Truṭi>".to_string()),
        });

        self.apta_sources.push(AptaSource {
            name: "मानक पुस्तकालय".to_string(), // Standard library
            signatures: stdlib_sigs,
            trust_level: 0.95,
        });
    }

    /// Parse documentation for type hints
    pub fn parse_doc(&mut self, name: &str, doc: &str) {
        let mut hints = Vec::new();

        for (line_num, line) in doc.lines().enumerate() {
            let trimmed = line.trim();

            // @type T
            if let Some(type_str) = trimmed.strip_prefix("@type ") {
                hints.push(DocHint {
                    kind: DocHintKind::TypeAnnotation,
                    value: type_str.trim().to_string(),
                    line: line_num as u32,
                });
            }

            // @returns T
            if let Some(ret_str) = trimmed.strip_prefix("@returns ").or(trimmed.strip_prefix("@return ")) {
                hints.push(DocHint {
                    kind: DocHintKind::Returns,
                    value: ret_str.trim().to_string(),
                    line: line_num as u32,
                });
            }

            // @param name: T
            if let Some(param_str) = trimmed.strip_prefix("@param ") {
                if let Some((name, type_desc)) = param_str.split_once(':') {
                    hints.push(DocHint {
                        kind: DocHintKind::Param,
                        value: format!("{}:{}", name.trim(), type_desc.trim()),
                        line: line_num as u32,
                    });
                }
            }

            // -> T (return type in signature)
            if let Some(idx) = trimmed.find("-> ") {
                let ret_type: String = trimmed[idx + 3..]
                    .chars()
                    .take_while(|c| c.is_alphanumeric() || *c == '<' || *c == '>' || *c == '_')
                    .collect();
                if !ret_type.is_empty() {
                    hints.push(DocHint {
                        kind: DocHintKind::Returns,
                        value: ret_type,
                        line: line_num as u32,
                    });
                }
            }
        }

        if !hints.is_empty() {
            self.doc_hints.insert(name.to_string(), hints);
        }
    }

    /// Record a contract
    pub fn record_contract(&mut self, contract: Contract) {
        self.contracts.insert(contract.name.clone(), contract);
    }

    /// Infer type from śabda
    pub fn infer(&self, name: &str, doc: &str) -> Option<TypeEvidence> {
        // 1. Check explicit contracts
        if let Some(contract) = self.contracts.get(name) {
            if let Some(ret) = &contract.returns {
                return Some(TypeEvidence {
                    type_name: ret.clone(),
                    pramana: Pramana::Shabda,
                    certainty: contract.certainty,
                    evidence: vec![
                        format!("शब्द (testimony): Contract specifies return type"),
                        format!("Type: {}", ret),
                    ],
                });
            }
        }

        // 2. Check doc hints
        if let Some(hints) = self.doc_hints.get(name) {
            for hint in hints {
                if hint.kind == DocHintKind::Returns || hint.kind == DocHintKind::TypeAnnotation {
                    return Some(TypeEvidence {
                        type_name: hint.value.clone(),
                        pramana: Pramana::Shabda,
                        certainty: 0.90,
                        evidence: vec![
                            format!("शब्द: Documentation hint at line {}", hint.line),
                            format!("Type: {}", hint.value),
                        ],
                    });
                }
            }
        }

        // 3. Check āpta sources
        for apta in &self.apta_sources {
            if let Some(sig) = apta.signatures.get(name) {
                if let Some(ret) = &sig.returns {
                    return Some(TypeEvidence {
                        type_name: ret.clone(),
                        pramana: Pramana::Shabda,
                        certainty: apta.trust_level,
                        evidence: vec![
                            format!("शब्द: Authoritative source '{}'", apta.name),
                            format!("Type: {}", ret),
                        ],
                    });
                }
            }
        }

        // 4. Parse doc inline
        let mut inline_hints = Vec::new();
        for line in doc.lines() {
            let trimmed = line.trim();
            if let Some(ret_str) = trimmed.strip_prefix("@returns ") {
                inline_hints.push(ret_str.trim().to_string());
            }
        }

        if let Some(ret) = inline_hints.first() {
            return Some(TypeEvidence {
                type_name: ret.clone(),
                pramana: Pramana::Shabda,
                certainty: 0.85,
                evidence: vec![
                    "शब्द: Inline documentation".to_string(),
                    format!("Type: {}", ret),
                ],
            });
        }

        None
    }

    /// Validate contracts at call site
    pub fn validate_contract(&self, name: &str, args: &[(String, String)]) -> ContractValidation {
        let Some(contract) = self.contracts.get(name) else {
            return ContractValidation::NoContract;
        };

        let mut errors = Vec::new();

        // Check argument types
        for (i, (_arg_name, arg_type)) in args.iter().enumerate() {
            if let Some(param) = contract.params.get(i) {
                if param.type_name != *arg_type && param.type_name != "*" {
                    errors.push(format!(
                        "Argument '{}' expected type '{}', got '{}'",
                        param.name, param.type_name, arg_type
                    ));
                }
            }
        }

        if errors.is_empty() {
            ContractValidation::Valid
        } else {
            ContractValidation::Invalid(errors)
        }
    }

    /// Add āpta source
    pub fn add_apta(&mut self, source: AptaSource) {
        self.apta_sources.push(source);
    }
}

/// Contract validation result
#[derive(Debug)]
pub enum ContractValidation {
    NoContract,
    Valid,
    Invalid(Vec<String>),
}

impl Default for ShabdaAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_parsing() {
        let mut analyzer = ShabdaAnalyzer::new();

        analyzer.parse_doc("test_fn", r#"
            This function does something.
            @returns t32
            @param x: t32 - the input
        "#);

        let result = analyzer.infer("test_fn", "");
        assert!(result.is_some());
        let evidence = result.unwrap();
        assert_eq!(evidence.type_name, "t32");
    }

    #[test]
    fn test_stdlib_apta() {
        let analyzer = ShabdaAnalyzer::new();

        let result = analyzer.infer("paṭha", "");
        assert!(result.is_some());
        let evidence = result.unwrap();
        assert!(evidence.type_name.contains("Phala"));
    }

    #[test]
    fn test_contract_validation() {
        let mut analyzer = ShabdaAnalyzer::new();

        analyzer.record_contract(Contract {
            name: "add".to_string(),
            params: vec![
                ParamContract {
                    name: "a".to_string(),
                    type_name: "t32".to_string(),
                    karaka: None,
                    description: "First number".to_string(),
                },
                ParamContract {
                    name: "b".to_string(),
                    type_name: "t32".to_string(),
                    karaka: None,
                    description: "Second number".to_string(),
                },
            ],
            returns: Some("t32".to_string()),
            preconditions: Vec::new(),
            postconditions: Vec::new(),
            certainty: 1.0,
        });

        let result = analyzer.validate_contract("add", &[
            ("a".to_string(), "t32".to_string()),
            ("b".to_string(), "t32".to_string()),
        ]);
        assert!(matches!(result, ContractValidation::Valid));
    }
}
