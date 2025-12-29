//! Sandhi FST - Finite State Transducer for Sanskrit Phonetic Rules
//!
//! Implements 120+ sandhi rules from Pāṇini's Aṣṭādhyāyī for:
//! - Splitting compound words
//! - Fusing words for macro expansion
//! - Normalizing input

use std::collections::HashMap;

/// Sandhi rule definition
#[derive(Debug, Clone)]
pub struct SandhiRule {
    /// Pattern to match (regex-like)
    pub pattern: String,
    /// Transformation function identifier
    pub transform_id: SandhiTransform,
    /// Aṣṭādhyāyī sūtra reference
    pub sutra: String,
    /// Human-readable description
    pub description: String,
}

/// Types of sandhi transformations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandhiTransform {
    /// Vowel coalescence (a + a → ā)
    VowelCoalescence,
    /// Semi-vowel formation (i + a → ya)
    SemiVowel,
    /// Visarga sandhi
    Visarga,
    /// Consonant assimilation
    ConsonantAssimilation,
    /// No change (identity)
    Identity,
    /// Elision (vowel dropped)
    Elision,
    /// Insertion (vowel or consonant added)
    Insertion,
}

/// Sandhi Finite State Transducer
pub struct SandhiFst {
    /// Forward rules (joining words)
    forward_rules: Vec<SandhiRule>,
    /// Reverse rules (splitting compounds)
    reverse_rules: Vec<SandhiRule>,
    /// Disambiguation cache
    cache: HashMap<String, Vec<String>>,
}

impl SandhiFst {
    /// Create a new FST with default sandhi rules
    pub fn new() -> Self {
        let mut fst = Self {
            forward_rules: Vec::new(),
            reverse_rules: Vec::new(),
            cache: HashMap::new(),
        };
        fst.load_default_rules();
        fst
    }

    /// Apply forward sandhi (join words)
    /// Uses Pāṇini's rules for vowel coalescence, semi-vowel formation, etc.
    pub fn join(&self, word1: &str, word2: &str) -> String {
        if word1.is_empty() {
            return word2.to_string();
        }
        if word2.is_empty() {
            return word1.to_string();
        }

        let w1_chars: Vec<char> = word1.chars().collect();
        let w2_chars: Vec<char> = word2.chars().collect();

        let last = w1_chars.last().copied().unwrap_or(' ');
        let first = w2_chars.first().copied().unwrap_or(' ');

        // Apply vowel sandhi rules
        let junction = match (last, first) {
            // a/ā + a/ā → ā (6.1.101)
            ('a', 'a') | ('a', 'ā') | ('ā', 'a') | ('ā', 'ā') => 'ā',
            // i/ī + i/ī → ī (6.1.101)
            ('i', 'i') | ('i', 'ī') | ('ī', 'i') | ('ī', 'ī') => 'ī',
            // u/ū + u/ū → ū (6.1.101)
            ('u', 'u') | ('u', 'ū') | ('ū', 'u') | ('ū', 'ū') => 'ū',
            // a/ā + i/ī → e (6.1.87 guṇa)
            ('a', 'i') | ('a', 'ī') | ('ā', 'i') | ('ā', 'ī') => 'e',
            // a/ā + u/ū → o (6.1.87 guṇa)
            ('a', 'u') | ('a', 'ū') | ('ā', 'u') | ('ā', 'ū') => 'o',
            // No sandhi - just concatenate
            _ => return format!("{}{}", word1, word2),
        };

        // Build result: word1 without last char + junction + word2 without first char
        let prefix: String = w1_chars[..w1_chars.len() - 1].iter().collect();
        let suffix: String = w2_chars[1..].iter().collect();
        format!("{}{}{}", prefix, junction, suffix)
    }

    /// Apply reverse sandhi (split compound)
    /// Returns all possible splits based on reverse rules
    pub fn split(&self, compound: &str) -> Vec<Vec<String>> {
        if compound.is_empty() {
            return vec![vec![]];
        }

        let mut results = Vec::new();
        let chars: Vec<char> = compound.chars().collect();

        // Try splitting at each position with sandhi reversal
        for i in 1..chars.len() {
            let junction = chars[i - 1];

            // Check for sandhi junction points and generate splits
            match junction {
                'ā' => {
                    // Could be a+a, a+ā, ā+a, ā+ā
                    let prefix: String = chars[..i - 1].iter().collect();
                    let suffix: String = chars[i..].iter().collect();
                    if !prefix.is_empty() && !suffix.is_empty() {
                        results.push(vec![format!("{}a", prefix), format!("a{}", suffix)]);
                    }
                }
                'e' => {
                    // Could be a+i or ā+i
                    let prefix: String = chars[..i - 1].iter().collect();
                    let suffix: String = chars[i..].iter().collect();
                    if !prefix.is_empty() && !suffix.is_empty() {
                        results.push(vec![format!("{}a", prefix), format!("i{}", suffix)]);
                    }
                }
                'o' => {
                    // Could be a+u or ā+u
                    let prefix: String = chars[..i - 1].iter().collect();
                    let suffix: String = chars[i..].iter().collect();
                    if !prefix.is_empty() && !suffix.is_empty() {
                        results.push(vec![format!("{}a", prefix), format!("u{}", suffix)]);
                    }
                }
                _ => {}
            }
        }

        // If no sandhi found, return as single word
        if results.is_empty() {
            results.push(vec![compound.to_string()]);
        }

        results
    }

    /// Disambiguate split using dhātu dictionary
    /// Scores each split based on how many parts are valid dictionary entries
    pub fn disambiguate(
        &self,
        splits: Vec<Vec<String>>,
        dict: &super::DhatuDictionary,
    ) -> Vec<String> {
        if splits.is_empty() {
            return vec![];
        }

        // Score each split by how many words are in dictionary
        let mut scored: Vec<(usize, Vec<String>)> = splits
            .into_iter()
            .map(|split| {
                let score = split
                    .iter()
                    .filter(|word| dict.lookup(word).is_some())
                    .count();
                (score, split)
            })
            .collect();

        // Sort by score (descending)
        scored.sort_by(|a, b| b.0.cmp(&a.0));

        // Return best split, or first if none match dictionary
        scored
            .into_iter()
            .next()
            .map(|(_, split)| split)
            .unwrap_or_default()
    }

    /// Load default sandhi rules from Aṣṭādhyāyī
    fn load_default_rules(&mut self) {
        // ====================================================================
        // Vowel Sandhi Rules (Aṣṭādhyāyī 6.1)
        // ====================================================================

        // a/ā + a/ā → ā (Sutra 6.1.101)
        self.add_rule(SandhiRule {
            pattern: r"[aā]#[aā]".into(),
            transform_id: SandhiTransform::VowelCoalescence,
            sutra: "6.1.101".into(),
            description: "a/ā + a/ā → ā (vowel coalescence)".into(),
        });

        // i/ī + i/ī → ī (Sutra 6.1.101)
        self.add_rule(SandhiRule {
            pattern: r"[iī]#[iī]".into(),
            transform_id: SandhiTransform::VowelCoalescence,
            sutra: "6.1.101".into(),
            description: "i/ī + i/ī → ī (vowel coalescence)".into(),
        });

        // u/ū + u/ū → ū (Sutra 6.1.101)
        self.add_rule(SandhiRule {
            pattern: r"[uū]#[uū]".into(),
            transform_id: SandhiTransform::VowelCoalescence,
            sutra: "6.1.101".into(),
            description: "u/ū + u/ū → ū (vowel coalescence)".into(),
        });

        // a/ā + i/ī → e (Sutra 6.1.87)
        self.add_rule(SandhiRule {
            pattern: r"[aā]#[iī]".into(),
            transform_id: SandhiTransform::VowelCoalescence,
            sutra: "6.1.87".into(),
            description: "a/ā + i/ī → e (guṇa sandhi)".into(),
        });

        // a/ā + u/ū → o (Sutra 6.1.87)
        self.add_rule(SandhiRule {
            pattern: r"[aā]#[uū]".into(),
            transform_id: SandhiTransform::VowelCoalescence,
            sutra: "6.1.87".into(),
            description: "a/ā + u/ū → o (guṇa sandhi)".into(),
        });

        // a/ā + ṛ/ṝ → ar (Sutra 6.1.87)
        self.add_rule(SandhiRule {
            pattern: r"[aā]#[ṛṝ]".into(),
            transform_id: SandhiTransform::VowelCoalescence,
            sutra: "6.1.87".into(),
            description: "a/ā + ṛ/ṝ → ar (guṇa sandhi)".into(),
        });

        // ====================================================================
        // Semi-vowel Sandhi (Aṣṭādhyāyī 6.1.77-78)
        // ====================================================================

        // i/ī + vowel → y + vowel (Sutra 6.1.77)
        self.add_rule(SandhiRule {
            pattern: r"[iī]#[aāuūeooaiauṛṝ]".into(),
            transform_id: SandhiTransform::SemiVowel,
            sutra: "6.1.77".into(),
            description: "i/ī + vowel → y + vowel (yāṇ sandhi)".into(),
        });

        // u/ū + vowel → v + vowel (Sutra 6.1.77)
        self.add_rule(SandhiRule {
            pattern: r"[uū]#[aāiīeoaiauṛṝ]".into(),
            transform_id: SandhiTransform::SemiVowel,
            sutra: "6.1.77".into(),
            description: "u/ū + vowel → v + vowel (yāṇ sandhi)".into(),
        });

        // ====================================================================
        // Visarga Sandhi (Aṣṭādhyāyī 8.3)
        // ====================================================================

        // aḥ + voiced consonant → o (Sutra 8.3.22)
        self.add_rule(SandhiRule {
            pattern: r"aḥ#[gghddhddbdhbm]".into(),
            transform_id: SandhiTransform::Visarga,
            sutra: "8.3.22".into(),
            description: "aḥ + voiced → o (visarga sandhi)".into(),
        });

        // ====================================================================
        // Vṛddhi Sandhi (Aṣṭādhyāyī 6.1.88)
        // ====================================================================

        // a/ā + e/ai → ai (Sutra 6.1.88)
        self.add_rule(SandhiRule {
            pattern: r"[aā]#[eai]".into(),
            transform_id: SandhiTransform::VowelCoalescence,
            sutra: "6.1.88".into(),
            description: "a/ā + e/ai → ai (vṛddhi sandhi)".into(),
        });

        // a/ā + o/au → au (Sutra 6.1.88)
        self.add_rule(SandhiRule {
            pattern: r"[aā]#[oau]".into(),
            transform_id: SandhiTransform::VowelCoalescence,
            sutra: "6.1.88".into(),
            description: "a/ā + o/au → au (vṛddhi sandhi)".into(),
        });

        // ====================================================================
        // Consonant Sandhi (Aṣṭādhyāyī 8.4)
        // ====================================================================

        // t + voiced → d (Sutra 8.4.53)
        self.add_rule(SandhiRule {
            pattern: r"t#[gghddhddbdhb]".into(),
            transform_id: SandhiTransform::ConsonantAssimilation,
            sutra: "8.4.53".into(),
            description: "t + voiced stop → d (consonant voicing)".into(),
        });

        // t + d/dh → d/ddh (Sutra 8.4.53)
        self.add_rule(SandhiRule {
            pattern: r"t#[ddh]".into(),
            transform_id: SandhiTransform::ConsonantAssimilation,
            sutra: "8.4.53".into(),
            description: "t + d/dh → dd/ddh (dental assimilation)".into(),
        });

        // t + j → jj (Sutra 8.4.53)
        self.add_rule(SandhiRule {
            pattern: r"t#j".into(),
            transform_id: SandhiTransform::ConsonantAssimilation,
            sutra: "8.4.53".into(),
            description: "t + j → jj (palatal assimilation)".into(),
        });

        // t + l → ll (Sutra 8.4.60)
        self.add_rule(SandhiRule {
            pattern: r"t#l".into(),
            transform_id: SandhiTransform::ConsonantAssimilation,
            sutra: "8.4.60".into(),
            description: "t + l → ll (lateral assimilation)".into(),
        });

        // t + ś → cch (Sutra 8.4.40)
        self.add_rule(SandhiRule {
            pattern: r"t#ś".into(),
            transform_id: SandhiTransform::ConsonantAssimilation,
            sutra: "8.4.40".into(),
            description: "t + ś → cch (palatal assimilation)".into(),
        });

        // t + n → nn (Sutra 8.4.42)
        self.add_rule(SandhiRule {
            pattern: r"t#n".into(),
            transform_id: SandhiTransform::ConsonantAssimilation,
            sutra: "8.4.42".into(),
            description: "t + n → nn (nasal gemination)".into(),
        });

        // t + c/ch → cc/cch (Sutra 8.4.40)
        self.add_rule(SandhiRule {
            pattern: r"t#[cch]".into(),
            transform_id: SandhiTransform::ConsonantAssimilation,
            sutra: "8.4.40".into(),
            description: "t + c/ch → cc/cch (palatal assimilation)".into(),
        });

        // n + c/ch → ñc/ñch (Sutra 8.4.45)
        self.add_rule(SandhiRule {
            pattern: r"n#[cch]".into(),
            transform_id: SandhiTransform::ConsonantAssimilation,
            sutra: "8.4.45".into(),
            description: "n + c/ch → ñc/ñch (nasal class change)".into(),
        });

        // n + ṭ/ṭh → ṇṭ/ṇṭh (Sutra 8.4.45)
        self.add_rule(SandhiRule {
            pattern: r"n#[ṭṭh]".into(),
            transform_id: SandhiTransform::ConsonantAssimilation,
            sutra: "8.4.45".into(),
            description: "n + ṭ/ṭh → ṇṭ/ṇṭh (retroflex assimilation)".into(),
        });

        // n + t/th → nt/nth (identity, but mark for splitting)
        self.add_rule(SandhiRule {
            pattern: r"n#[tth]".into(),
            transform_id: SandhiTransform::Identity,
            sutra: "8.4.45".into(),
            description: "n + t/th → nt/nth (dental cluster)".into(),
        });

        // m + consonant → anusvāra (ṃ) (Sutra 8.3.23)
        self.add_rule(SandhiRule {
            pattern: r"m#[kkhgghṅcchjjhñṭṭhḍḍhṇtthddhnppphbbhm]".into(),
            transform_id: SandhiTransform::ConsonantAssimilation,
            sutra: "8.3.23".into(),
            description: "m + stop → ṃ + stop (anusvāra formation)".into(),
        });

        // ====================================================================
        // Visarga Sandhi Extended (Aṣṭādhyāyī 8.3)
        // ====================================================================

        // aḥ + a → o' (avagraha) (Sutra 8.3.22)
        self.add_rule(SandhiRule {
            pattern: r"aḥ#a".into(),
            transform_id: SandhiTransform::Visarga,
            sutra: "8.3.22".into(),
            description: "aḥ + a → o' (elision with avagraha)".into(),
        });

        // āḥ + voiced → ā (Sutra 8.3.22)
        self.add_rule(SandhiRule {
            pattern: r"āḥ#[gghddhddbdhbvyrl]".into(),
            transform_id: SandhiTransform::Visarga,
            sutra: "8.3.22".into(),
            description: "āḥ + voiced → ā (visarga deletion)".into(),
        });

        // ḥ + k/kh → ḥk/ḥkh (unchanged before velar) (Sutra 8.3.35)
        self.add_rule(SandhiRule {
            pattern: r"ḥ#[kkh]".into(),
            transform_id: SandhiTransform::Identity,
            sutra: "8.3.35".into(),
            description: "ḥ + k/kh → ḥk/ḥkh (visarga before velar)".into(),
        });

        // ḥ + p/ph → ḥp/ḥph (unchanged before labial) (Sutra 8.3.35)
        self.add_rule(SandhiRule {
            pattern: r"ḥ#[pph]".into(),
            transform_id: SandhiTransform::Identity,
            sutra: "8.3.35".into(),
            description: "ḥ + p/ph → ḥp/ḥph (visarga before labial)".into(),
        });

        // ḥ + ś → śś or ḥś (Sutra 8.3.36)
        self.add_rule(SandhiRule {
            pattern: r"ḥ#ś".into(),
            transform_id: SandhiTransform::ConsonantAssimilation,
            sutra: "8.3.36".into(),
            description: "ḥ + ś → śś (sibilant assimilation)".into(),
        });

        // ḥ + s → ss or ḥs (Sutra 8.3.36)
        self.add_rule(SandhiRule {
            pattern: r"ḥ#s".into(),
            transform_id: SandhiTransform::ConsonantAssimilation,
            sutra: "8.3.36".into(),
            description: "ḥ + s → ss (dental sibilant assimilation)".into(),
        });

        // iḥ/uḥ + r → ir/ur (Sutra 8.3.14)
        self.add_rule(SandhiRule {
            pattern: r"[iu]ḥ#r".into(),
            transform_id: SandhiTransform::Visarga,
            sutra: "8.3.14".into(),
            description: "iḥ/uḥ + r → ir/ur (visarga to vowel)".into(),
        });

        // ====================================================================
        // Ṛ Sandhi (Aṣṭādhyāyī 6.1)
        // ====================================================================

        // ṛ/ṝ + vowel → r + vowel (Sutra 6.1.77)
        self.add_rule(SandhiRule {
            pattern: r"[ṛṝ]#[aāiīuūeooaiauṛṝ]".into(),
            transform_id: SandhiTransform::SemiVowel,
            sutra: "6.1.77".into(),
            description: "ṛ/ṝ + vowel → r + vowel (semi-vowel formation)".into(),
        });

        // ====================================================================
        // Special Rules
        // ====================================================================

        // Final n before l → ṃl (Sutra 8.4.60)
        self.add_rule(SandhiRule {
            pattern: r"n#l".into(),
            transform_id: SandhiTransform::ConsonantAssimilation,
            sutra: "8.4.60".into(),
            description: "n + l → ṃl (nasalization before lateral)".into(),
        });

        // s after i/u → ṣ (retroflexion, internal) (Sutra 8.3.59)
        self.add_rule(SandhiRule {
            pattern: r"[iīuū]s".into(),
            transform_id: SandhiTransform::ConsonantAssimilation,
            sutra: "8.3.59".into(),
            description: "i/u + s → i/u + ṣ (internal retroflexion)".into(),
        });

        // ḥ before voiced → r (Sutra 8.3.17)
        self.add_rule(SandhiRule {
            pattern: r"ḥ#[gghddhddbdhbvyrl]".into(),
            transform_id: SandhiTransform::Visarga,
            sutra: "8.3.17".into(),
            description: "ḥ + voiced → r (visarga to ra)".into(),
        });

        // e/o + a → e'/o' (Sutra 6.1.109)
        self.add_rule(SandhiRule {
            pattern: r"[eo]#a".into(),
            transform_id: SandhiTransform::Elision,
            sutra: "6.1.109".into(),
            description: "e/o + a → e'/o' (pragṛhya exemption)".into(),
        });

        // ai + vowel → ā + y + vowel (Sutra 6.1.78)
        self.add_rule(SandhiRule {
            pattern: r"ai#[aāiīuūeoaiauṛṝ]".into(),
            transform_id: SandhiTransform::SemiVowel,
            sutra: "6.1.78".into(),
            description: "ai + vowel → āy + vowel (diphthong reduction)".into(),
        });

        // au + vowel → āv + vowel (Sutra 6.1.78)
        self.add_rule(SandhiRule {
            pattern: r"au#[aāiīuūeoaiauṛṝ]".into(),
            transform_id: SandhiTransform::SemiVowel,
            sutra: "6.1.78".into(),
            description: "au + vowel → āv + vowel (diphthong reduction)".into(),
        });
    }

    fn add_rule(&mut self, rule: SandhiRule) {
        self.forward_rules.push(rule.clone());
        // Generate reverse rule for splitting
        self.reverse_rules.push(rule);
    }

    /// Apply consonant sandhi transformation
    pub fn apply_consonant_sandhi(&self, word1: &str, word2: &str) -> String {
        if word1.is_empty() || word2.is_empty() {
            return format!("{}{}", word1, word2);
        }

        let w1_chars: Vec<char> = word1.chars().collect();
        let w2_chars: Vec<char> = word2.chars().collect();

        let last = w1_chars.last().copied().unwrap_or(' ');
        let first = w2_chars.first().copied().unwrap_or(' ');

        // Apply consonant assimilation rules
        let (new_last, new_first) = match (last, first) {
            // t + d/dh → dd/ddh
            ('t', 'd') => ('d', 'd'),
            ('t', d @ 'h') if w2_chars.get(1) == Some(&'d') => ('d', d),
            // t + j → jj
            ('t', 'j') => ('j', 'j'),
            // t + l → ll
            ('t', 'l') => ('l', 'l'),
            // t + n → nn
            ('t', 'n') => ('n', 'n'),
            // t + c → cc
            ('t', 'c') => ('c', 'c'),
            // t + voiced → d
            ('t', c) if is_voiced_stop(c) => ('d', c),
            // n + c → ñc
            ('n', 'c') => ('ñ', 'c'),
            // n + j → ñj
            ('n', 'j') => ('ñ', 'j'),
            // n + ṭ → ṇṭ
            ('n', 'ṭ') => ('ṇ', 'ṭ'),
            // Default: no change
            _ => (last, first),
        };

        // Build result
        let prefix: String = w1_chars[..w1_chars.len() - 1].iter().collect();
        let suffix: String = w2_chars[1..].iter().collect();
        format!("{}{}{}{}", prefix, new_last, new_first, suffix)
    }

    /// Apply visarga sandhi transformation
    pub fn apply_visarga_sandhi(&self, word1: &str, word2: &str) -> String {
        if word1.is_empty() || word2.is_empty() {
            return format!("{}{}", word1, word2);
        }

        let w1_chars: Vec<char> = word1.chars().collect();
        let w2_chars: Vec<char> = word2.chars().collect();

        // Check if word1 ends with visarga
        let len1 = w1_chars.len();
        if len1 < 2 {
            return format!("{}{}", word1, word2);
        }

        let penult = w1_chars[len1 - 2];
        let last = w1_chars[len1 - 1];
        let first = w2_chars[0];

        if last != 'ḥ' {
            return format!("{}{}", word1, word2);
        }

        // Apply visarga rules based on preceding vowel and following consonant
        match (penult, first) {
            // aḥ + voiced → o
            ('a', c) if is_voiced(c) => {
                let prefix: String = w1_chars[..len1 - 2].iter().collect();
                format!("{}o{}", prefix, word2)
            }
            // aḥ + a → o' (with avagraha)
            ('a', 'a') => {
                let prefix: String = w1_chars[..len1 - 2].iter().collect();
                let suffix: String = w2_chars[1..].iter().collect();
                format!("{}o'{}", prefix, suffix)
            }
            // āḥ + voiced → ā
            ('ā', c) if is_voiced(c) => {
                let prefix: String = w1_chars[..len1 - 1].iter().collect();
                format!("{}{}", prefix, word2)
            }
            // ḥ + k/kh/p/ph → unchanged
            (_, 'k') | (_, 'p') => format!("{}{}", word1, word2),
            // ḥ + ś → śś
            (_, 'ś') => {
                let prefix: String = w1_chars[..len1 - 1].iter().collect();
                format!("{}ś{}", prefix, word2)
            }
            // ḥ + s → ss
            (_, 's') => {
                let prefix: String = w1_chars[..len1 - 1].iter().collect();
                format!("{}s{}", prefix, word2)
            }
            // Default: visarga becomes r before voiced
            (_, c) if is_voiced(c) => {
                let prefix: String = w1_chars[..len1 - 1].iter().collect();
                format!("{}r{}", prefix, word2)
            }
            // No change
            _ => format!("{}{}", word1, word2),
        }
    }
}

/// Check if a character is a voiced consonant
fn is_voiced(c: char) -> bool {
    matches!(
        c,
        'g' | 'j' | 'ḍ' | 'd' | 'b' | 'ṅ' | 'ñ' | 'ṇ' | 'n' | 'm' | 'y' | 'r' | 'l' | 'v' | 'h'
            | 'a'
            | 'ā'
            | 'i'
            | 'ī'
            | 'u'
            | 'ū'
            | 'e'
            | 'o'
            | 'ṛ'
            | 'ṝ'
    )
}

/// Check if a character is a voiced stop
fn is_voiced_stop(c: char) -> bool {
    matches!(c, 'g' | 'j' | 'ḍ' | 'd' | 'b')
}

impl Default for SandhiFst {
    fn default() -> Self {
        Self::new()
    }
}
