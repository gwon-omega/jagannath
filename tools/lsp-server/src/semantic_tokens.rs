//! Semantic Tokens - Sophisticated syntax highlighting
//!
//! Provides token classification based on semantic analysis.
//! Uses the Varṇa (वर्ण - color/caste) concept for token types.

use tower_lsp::lsp_types::*;

// ============================================================================
// Token Types (Varṇa - वर्ण)
// ============================================================================

/// Jagannath semantic token types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum VarnaType {
    /// Namespace/module (नामस्थान - nāmasthāna)
    Namasthana = 0,
    /// Type (प्रकार - prakāra)
    Prakara = 1,
    /// Class/struct (वर्ग - varga)
    Varga = 2,
    /// Enum (गण - gaṇa)
    Gana = 3,
    /// Interface/trait (लक्षण - lakṣaṇa)
    Lakshana = 4,
    /// Struct (संरचना - saṃracanā)
    Samrachana = 5,
    /// Type parameter (प्रकार-चर - prakāra-cara)
    PrakaraCara = 6,
    /// Parameter (प्राचल - prācala)
    Prachala = 7,
    /// Variable (चर - cara)
    Cara = 8,
    /// Property (गुण - guṇa)
    Guna = 9,
    /// Enum member (गण-सदस्य - gaṇa-sadasya)
    GanaSadasya = 10,
    /// Function (कार्य - kārya)
    Karya = 11,
    /// Method (विधि - vidhi)
    Vidhi = 12,
    /// Macro (मन्त्र - mantra)
    Mantra = 13,
    /// Keyword (कुञ्जिका - kuñjikā)
    Kunjika = 14,
    /// Modifier (संशोधक - saṃśodhaka)
    Samshodhaka = 15,
    /// Comment (टीका - ṭīkā)
    Tika = 16,
    /// String (सूत्र - sūtra)
    Sutra = 17,
    /// Number (संख्या - saṅkhyā)
    Sankhya = 18,
    /// Regexp (नियम - niyama)
    Niyama = 19,
    /// Operator (संकारक - saṃkāraka)
    Samkaraka = 20,
    /// Decorator/attribute (अलंकार - alaṃkāra)
    Alankara = 21,
    /// Label (चिह्न - cihna)
    Chihna = 22,
    /// Lifetime (आयुष्य - āyuṣya)
    Ayushya = 23,
    /// Karaka marker (कारक - kāraka)
    Karaka = 24,
}

impl VarnaType {
    /// Get LSP semantic token type
    pub fn to_lsp(&self) -> SemanticTokenType {
        match self {
            VarnaType::Namasthana => SemanticTokenType::NAMESPACE,
            VarnaType::Prakara => SemanticTokenType::TYPE,
            VarnaType::Varga => SemanticTokenType::CLASS,
            VarnaType::Gana => SemanticTokenType::ENUM,
            VarnaType::Lakshana => SemanticTokenType::INTERFACE,
            VarnaType::Samrachana => SemanticTokenType::STRUCT,
            VarnaType::PrakaraCara => SemanticTokenType::TYPE_PARAMETER,
            VarnaType::Prachala => SemanticTokenType::PARAMETER,
            VarnaType::Cara => SemanticTokenType::VARIABLE,
            VarnaType::Guna => SemanticTokenType::PROPERTY,
            VarnaType::GanaSadasya => SemanticTokenType::ENUM_MEMBER,
            VarnaType::Karya => SemanticTokenType::FUNCTION,
            VarnaType::Vidhi => SemanticTokenType::METHOD,
            VarnaType::Mantra => SemanticTokenType::MACRO,
            VarnaType::Kunjika => SemanticTokenType::KEYWORD,
            VarnaType::Samshodhaka => SemanticTokenType::MODIFIER,
            VarnaType::Tika => SemanticTokenType::COMMENT,
            VarnaType::Sutra => SemanticTokenType::STRING,
            VarnaType::Sankhya => SemanticTokenType::NUMBER,
            VarnaType::Niyama => SemanticTokenType::REGEXP,
            VarnaType::Samkaraka => SemanticTokenType::OPERATOR,
            VarnaType::Alankara => SemanticTokenType::DECORATOR,
            VarnaType::Chihna => SemanticTokenType::new("label"),
            VarnaType::Ayushya => SemanticTokenType::new("lifetime"),
            VarnaType::Karaka => SemanticTokenType::new("karaka"),
        }
    }

    /// Get all token types for legend
    pub fn all_types() -> Vec<SemanticTokenType> {
        vec![
            SemanticTokenType::NAMESPACE,
            SemanticTokenType::TYPE,
            SemanticTokenType::CLASS,
            SemanticTokenType::ENUM,
            SemanticTokenType::INTERFACE,
            SemanticTokenType::STRUCT,
            SemanticTokenType::TYPE_PARAMETER,
            SemanticTokenType::PARAMETER,
            SemanticTokenType::VARIABLE,
            SemanticTokenType::PROPERTY,
            SemanticTokenType::ENUM_MEMBER,
            SemanticTokenType::FUNCTION,
            SemanticTokenType::METHOD,
            SemanticTokenType::MACRO,
            SemanticTokenType::KEYWORD,
            SemanticTokenType::MODIFIER,
            SemanticTokenType::COMMENT,
            SemanticTokenType::STRING,
            SemanticTokenType::NUMBER,
            SemanticTokenType::REGEXP,
            SemanticTokenType::OPERATOR,
            SemanticTokenType::DECORATOR,
            SemanticTokenType::new("label"),
            SemanticTokenType::new("lifetime"),
            SemanticTokenType::new("karaka"),
        ]
    }
}

// ============================================================================
// Token Modifiers (Viśeṣaṇa - विशेषण)
// ============================================================================

/// Token modifiers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum Visheshana {
    /// Declaration (घोषणा - ghoṣaṇā)
    Ghoshana = 0,
    /// Definition (परिभाषा - paribhāṣā)
    Paribhasha = 1,
    /// Readonly (केवलपठन - kevalapaṭhana)
    KevalaPathana = 2,
    /// Static (स्थिर - sthira)
    Sthira = 3,
    /// Deprecated (निन्दित - nindita)
    Nindita = 4,
    /// Abstract (अमूर्त - amūrta)
    Amurta = 5,
    /// Async (अतुल्यकालिक - atulyakālika)
    Atulyakalika = 6,
    /// Modification (परिवर्तन - parivartana)
    Parivartana = 7,
    /// Documentation (प्रलेखन - pralekhana)
    Pralekhana = 8,
    /// Default library (मानक - mānaka)
    Manaka = 9,
    /// Mutable (परिवर्त्य - parivartya)
    Parivartya = 10,
    /// Consumed/moved (गृहीत - gṛhīta)
    Grihita = 11,
}

impl Visheshana {
    /// Get LSP semantic token modifier
    pub fn to_lsp(&self) -> SemanticTokenModifier {
        match self {
            Visheshana::Ghoshana => SemanticTokenModifier::DECLARATION,
            Visheshana::Paribhasha => SemanticTokenModifier::DEFINITION,
            Visheshana::KevalaPathana => SemanticTokenModifier::READONLY,
            Visheshana::Sthira => SemanticTokenModifier::STATIC,
            Visheshana::Nindita => SemanticTokenModifier::DEPRECATED,
            Visheshana::Amurta => SemanticTokenModifier::ABSTRACT,
            Visheshana::Atulyakalika => SemanticTokenModifier::ASYNC,
            Visheshana::Parivartana => SemanticTokenModifier::MODIFICATION,
            Visheshana::Pralekhana => SemanticTokenModifier::DOCUMENTATION,
            Visheshana::Manaka => SemanticTokenModifier::DEFAULT_LIBRARY,
            Visheshana::Parivartya => SemanticTokenModifier::new("mutable"),
            Visheshana::Grihita => SemanticTokenModifier::new("consumed"),
        }
    }

    /// Get all modifiers for legend
    pub fn all_modifiers() -> Vec<SemanticTokenModifier> {
        vec![
            SemanticTokenModifier::DECLARATION,
            SemanticTokenModifier::DEFINITION,
            SemanticTokenModifier::READONLY,
            SemanticTokenModifier::STATIC,
            SemanticTokenModifier::DEPRECATED,
            SemanticTokenModifier::ABSTRACT,
            SemanticTokenModifier::ASYNC,
            SemanticTokenModifier::MODIFICATION,
            SemanticTokenModifier::DOCUMENTATION,
            SemanticTokenModifier::DEFAULT_LIBRARY,
            SemanticTokenModifier::new("mutable"),
            SemanticTokenModifier::new("consumed"),
        ]
    }
}

// ============================================================================
// Semantic Token
// ============================================================================

/// A semantic token with position and type
#[derive(Debug, Clone)]
pub struct SemanticVarna {
    /// Line number (0-indexed)
    pub line: u32,
    /// Start column (0-indexed, in UTF-16 code units)
    pub start: u32,
    /// Length (in UTF-16 code units)
    pub length: u32,
    /// Token type
    pub varna: VarnaType,
    /// Modifiers (bitset)
    pub visheshana: u32,
}

impl SemanticVarna {
    /// Create new semantic token
    pub fn new(line: u32, start: u32, length: u32, varna: VarnaType) -> Self {
        Self {
            line,
            start,
            length,
            varna,
            visheshana: 0,
        }
    }

    /// Add modifier
    pub fn with_modifier(mut self, modifier: Visheshana) -> Self {
        self.visheshana |= 1 << (modifier as u32);
        self
    }
}

// ============================================================================
// Token Builder
// ============================================================================

/// Builds semantic tokens from source
pub struct VarnaBuilder {
    tokens: Vec<SemanticVarna>,
}

impl VarnaBuilder {
    pub fn new() -> Self {
        Self { tokens: Vec::new() }
    }

    /// Add a token
    pub fn push(&mut self, token: SemanticVarna) {
        self.tokens.push(token);
    }

    /// Add keyword token
    pub fn keyword(&mut self, line: u32, start: u32, length: u32) {
        self.push(SemanticVarna::new(line, start, length, VarnaType::Kunjika));
    }

    /// Add type token
    pub fn type_name(&mut self, line: u32, start: u32, length: u32) {
        self.push(SemanticVarna::new(line, start, length, VarnaType::Prakara));
    }

    /// Add function token
    pub fn function(&mut self, line: u32, start: u32, length: u32) {
        self.push(SemanticVarna::new(line, start, length, VarnaType::Karya));
    }

    /// Add variable token
    pub fn variable(&mut self, line: u32, start: u32, length: u32, is_mutable: bool) {
        let token = SemanticVarna::new(line, start, length, VarnaType::Cara);
        let token = if is_mutable {
            token.with_modifier(Visheshana::Parivartya)
        } else {
            token.with_modifier(Visheshana::KevalaPathana)
        };
        self.push(token);
    }

    /// Add karaka marker token
    pub fn karaka(&mut self, line: u32, start: u32, length: u32) {
        self.push(SemanticVarna::new(line, start, length, VarnaType::Karaka));
    }

    /// Add lifetime token
    pub fn lifetime(&mut self, line: u32, start: u32, length: u32) {
        self.push(SemanticVarna::new(line, start, length, VarnaType::Ayushya));
    }

    /// Build LSP semantic tokens
    pub fn build(mut self) -> SemanticTokensResult {
        // Sort by position
        self.tokens.sort_by(|a, b| {
            a.line.cmp(&b.line).then(a.start.cmp(&b.start))
        });

        let mut data = Vec::new();
        let mut prev_line = 0u32;
        let mut prev_start = 0u32;

        for token in &self.tokens {
            let delta_line = token.line - prev_line;
            let delta_start = if delta_line == 0 {
                token.start - prev_start
            } else {
                token.start
            };

            data.push(SemanticToken {
                delta_line,
                delta_start,
                length: token.length,
                token_type: token.varna as u32,
                token_modifiers_bitset: token.visheshana,
            });

            prev_line = token.line;
            prev_start = token.start;
        }

        SemanticTokensResult::Tokens(SemanticTokens {
            result_id: None,
            data,
        })
    }
}

impl Default for VarnaBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Legend
// ============================================================================

/// Get the semantic tokens legend
pub fn semantic_tokens_legend() -> SemanticTokensLegend {
    SemanticTokensLegend {
        token_types: VarnaType::all_types(),
        token_modifiers: Visheshana::all_modifiers(),
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_varna_type_mapping() {
        assert_eq!(VarnaType::Kunjika.to_lsp(), SemanticTokenType::KEYWORD);
        assert_eq!(VarnaType::Prakara.to_lsp(), SemanticTokenType::TYPE);
        assert_eq!(VarnaType::Karya.to_lsp(), SemanticTokenType::FUNCTION);
    }

    #[test]
    fn test_builder_basic() {
        let mut builder = VarnaBuilder::new();
        builder.keyword(0, 0, 4);
        builder.function(0, 5, 3);

        let result = builder.build();
        match result {
            SemanticTokensResult::Tokens(tokens) => {
                assert_eq!(tokens.data.len(), 2);
                assert_eq!(tokens.data[0].delta_line, 0);
                assert_eq!(tokens.data[0].delta_start, 0);
                assert_eq!(tokens.data[1].delta_start, 5);
            }
            _ => panic!("Expected tokens"),
        }
    }

    #[test]
    fn test_modifier() {
        let token = SemanticVarna::new(0, 0, 5, VarnaType::Cara)
            .with_modifier(Visheshana::KevalaPathana)
            .with_modifier(Visheshana::Ghoshana);

        assert_eq!(token.visheshana, (1 << 2) | (1 << 0));
    }

    #[test]
    fn test_legend_completeness() {
        let legend = semantic_tokens_legend();
        assert!(legend.token_types.len() >= 20);
        assert!(legend.token_modifiers.len() >= 10);
    }
}
