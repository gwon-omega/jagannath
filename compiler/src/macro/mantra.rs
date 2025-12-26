//! Mantra - Compile-time Incantations
//!
//! Built-in macros (mantras) for the language.

/// Built-in mantras
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuiltinMantra {
    /// oṁ! - Assert (sacred syllable)
    Om,
    /// mudraṇa! - Print debug info
    Mudrana,
    /// sthira! - Compile-time constant
    Sthira,
    /// prakāśa! - Include file
    Prakasha,
    /// parikṣā! - Debug assert
    Pariksha,
    /// vinyāsa! - Format string
    Vinyasa,
    /// kalpanā! - Create struct literal
    Kalpana,
    /// vistāra! - Derive trait
    Vistara,
}

impl BuiltinMantra {
    pub fn from_name(name: &str) -> Option<Self> {
        match name {
            "oṁ" | "om" => Some(Self::Om),
            "mudraṇa" | "mudrana" => Some(Self::Mudrana),
            "sthira" => Some(Self::Sthira),
            "prakāśa" | "prakasha" => Some(Self::Prakasha),
            "parikṣā" | "pariksha" => Some(Self::Pariksha),
            "vinyāsa" | "vinyasa" => Some(Self::Vinyasa),
            "kalpanā" | "kalpana" => Some(Self::Kalpana),
            "vistāra" | "vistara" => Some(Self::Vistara),
            _ => None,
        }
    }

    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            Self::Om => "ॐ",
            Self::Mudrana => "मुद्रण",
            Self::Sthira => "स्थिर",
            Self::Prakasha => "प्रकाश",
            Self::Pariksha => "परीक्षा",
            Self::Vinyasa => "विन्यास",
            Self::Kalpana => "कल्पना",
            Self::Vistara => "विस्तार",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Om => "Assert that condition is true",
            Self::Mudrana => "Print debug information",
            Self::Sthira => "Compile-time constant evaluation",
            Self::Prakasha => "Include file contents",
            Self::Pariksha => "Debug-only assertion",
            Self::Vinyasa => "Format string with arguments",
            Self::Kalpana => "Create struct literal",
            Self::Vistara => "Derive trait implementation",
        }
    }
}

/// Mantra invocation
#[derive(Debug, Clone)]
pub struct MantraInvocation {
    pub mantra: BuiltinMantra,
    pub arguments: Vec<MantraArg>,
    pub span: (usize, usize),
}

/// Mantra argument
#[derive(Debug, Clone)]
pub enum MantraArg {
    /// String literal
    String(String),
    /// Expression
    Expr(String),
    /// Identifier
    Ident(String),
    /// Token tree
    TokenTree(Vec<MantraArg>),
}
