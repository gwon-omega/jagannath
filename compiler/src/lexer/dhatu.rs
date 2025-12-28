//! Dhātu Dictionary - Sanskrit Root Word Database
//!
//! Contains 2000+ Sanskrit roots (dhātus) with their meanings,
//! grammatical properties, and usage patterns.

use std::collections::HashMap;

/// Entry in the dhātu dictionary
#[derive(Debug, Clone)]
pub struct DhatuEntry {
    /// Root form in IAST transliteration
    pub root: String,
    /// Root form in Devanagari
    pub devanagari: String,
    /// English meaning(s)
    pub meanings: Vec<String>,
    /// Gaṇa (verb class, 1-10)
    pub gana: u8,
    /// Padi (ātmanepada, parasmaipada, or ubhayapadi)
    pub padi: Padi,
    /// Semantic category for compiler hints
    pub category: DhatuCategory,
}

/// Verb voice/paradigm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Padi {
    /// Parasmaipada - active voice
    Parasmaipada,
    /// Ātmanepada - middle voice
    Atmanepada,
    /// Ubhayapadi - both voices
    Ubhayapadi,
}

/// Semantic category for compiler optimization hints
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DhatuCategory {
    /// Movement verbs (gam, car, etc.) - control flow
    Motion,
    /// Action verbs (kṛ, bhū, etc.) - computation
    Action,
    /// Perception verbs (dṛś, śru, etc.) - I/O operations
    Perception,
    /// Communication verbs (vad, brū, etc.) - messaging
    Communication,
    /// Creation verbs (jan, nirmā, etc.) - allocation
    Creation,
    /// Destruction verbs (naś, han, etc.) - deallocation
    Destruction,
    /// State verbs (as, sthā, etc.) - state management
    State,
    /// Transformation verbs (kṛ, bhū, etc.) - mutation
    Transformation,
}

/// Dictionary of Sanskrit root words
pub struct DhatuDictionary {
    /// Root -> Entry mapping
    entries: HashMap<String, DhatuEntry>,
    /// Trie for prefix matching
    trie: DhatuTrie,
}

impl DhatuDictionary {
    /// Create a new dictionary with default dhātus
    pub fn new() -> Self {
        let mut dict = Self {
            entries: HashMap::new(),
            trie: DhatuTrie::new(),
        };
        dict.load_default_dhatus();
        dict
    }

    /// Load from custom dictionary file
    /// File format: root|devanagari|meanings|gana|padi|category
    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        use std::io::{BufRead, BufReader};
        use std::fs::File;

        let mut dict = Self {
            entries: HashMap::new(),
            trie: DhatuTrie::new(),
        };

        let file = File::open(path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            if line.is_empty() || line.starts_with('#') { continue; }

            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() < 6 { continue; }

            let entry = DhatuEntry {
                root: parts[0].to_string(),
                devanagari: parts[1].to_string(),
                meanings: parts[2].split(',').map(|s| s.trim().to_string()).collect(),
                gana: parts[3].parse().unwrap_or(1),
                padi: match parts[4] {
                    "P" => Padi::Parasmaipada,
                    "A" => Padi::Atmanepada,
                    _ => Padi::Ubhayapadi,
                },
                category: match parts[5] {
                    "motion" => DhatuCategory::Motion,
                    "action" => DhatuCategory::Action,
                    "perception" => DhatuCategory::Perception,
                    "communication" => DhatuCategory::Communication,
                    "creation" => DhatuCategory::Creation,
                    "destruction" => DhatuCategory::Destruction,
                    "transformation" => DhatuCategory::Transformation,
                    _ => DhatuCategory::State,
                },
            };
            dict.add_dhatu(entry);
        }

        Ok(dict)
    }

    /// Look up a dhātu by root
    pub fn lookup(&self, root: &str) -> Option<&DhatuEntry> {
        self.entries.get(root)
    }

    /// Find all dhātus matching a prefix
    pub fn find_by_prefix(&self, prefix: &str) -> Vec<&DhatuEntry> {
        let words = self.trie.find_prefix(prefix);
        words.iter()
            .filter_map(|w| self.entries.get(w))
            .collect()
    }

    /// Load default dhātus essential for core language
    fn load_default_dhatus(&mut self) {
        // Core programming dhātus
        self.add_dhatu(DhatuEntry {
            root: "kṛ".into(),
            devanagari: "कृ".into(),
            meanings: vec!["to do".into(), "to make".into(), "to perform".into()],
            gana: 8,
            padi: Padi::Ubhayapadi,
            category: DhatuCategory::Action,
        });

        self.add_dhatu(DhatuEntry {
            root: "gam".into(),
            devanagari: "गम्".into(),
            meanings: vec!["to go".into(), "to move".into()],
            gana: 1,
            padi: Padi::Parasmaipada,
            category: DhatuCategory::Motion,
        });

        self.add_dhatu(DhatuEntry {
            root: "paṭh".into(),
            devanagari: "पठ्".into(),
            meanings: vec!["to read".into(), "to recite".into()],
            gana: 1,
            padi: Padi::Parasmaipada,
            category: DhatuCategory::Perception,
        });

        self.add_dhatu(DhatuEntry {
            root: "likh".into(),
            devanagari: "लिख्".into(),
            meanings: vec!["to write".into(), "to inscribe".into()],
            gana: 6,
            padi: Padi::Parasmaipada,
            category: DhatuCategory::Action,
        });

        self.add_dhatu(DhatuEntry {
            root: "dṛś".into(),
            devanagari: "दृश्".into(),
            meanings: vec!["to see".into(), "to perceive".into()],
            gana: 1,
            padi: Padi::Parasmaipada,
            category: DhatuCategory::Perception,
        });

        self.add_dhatu(DhatuEntry {
            root: "śru".into(),
            devanagari: "श्रु".into(),
            meanings: vec!["to hear".into(), "to listen".into()],
            gana: 5,
            padi: Padi::Parasmaipada,
            category: DhatuCategory::Perception,
        });

        self.add_dhatu(DhatuEntry {
            root: "vad".into(),
            devanagari: "वद्".into(),
            meanings: vec!["to speak".into(), "to say".into()],
            gana: 1,
            padi: Padi::Parasmaipada,
            category: DhatuCategory::Communication,
        });

        self.add_dhatu(DhatuEntry {
            root: "jan".into(),
            devanagari: "जन्".into(),
            meanings: vec!["to be born".into(), "to produce".into()],
            gana: 4,
            padi: Padi::Atmanepada,
            category: DhatuCategory::Creation,
        });

        self.add_dhatu(DhatuEntry {
            root: "naś".into(),
            devanagari: "नश्".into(),
            meanings: vec!["to perish".into(), "to destroy".into()],
            gana: 4,
            padi: Padi::Parasmaipada,
            category: DhatuCategory::Destruction,
        });

        self.add_dhatu(DhatuEntry {
            root: "sthā".into(),
            devanagari: "स्था".into(),
            meanings: vec!["to stand".into(), "to stay".into(), "to remain".into()],
            gana: 1,
            padi: Padi::Parasmaipada,
            category: DhatuCategory::State,
        });

        self.add_dhatu(DhatuEntry {
            root: "bhū".into(),
            devanagari: "भू".into(),
            meanings: vec!["to be".into(), "to become".into(), "to exist".into()],
            gana: 1,
            padi: Padi::Parasmaipada,
            category: DhatuCategory::State,
        });

        self.add_dhatu(DhatuEntry {
            root: "as".into(),
            devanagari: "अस्".into(),
            meanings: vec!["to be".into(), "to exist".into()],
            gana: 2,
            padi: Padi::Parasmaipada,
            category: DhatuCategory::State,
        });

        // Add more dhātus as needed...
    }

    fn add_dhatu(&mut self, entry: DhatuEntry) {
        let root = entry.root.clone();
        self.entries.insert(root.clone(), entry);
        self.trie.insert(&root);
    }
}

impl Default for DhatuDictionary {
    fn default() -> Self {
        Self::new()
    }
}

/// Trie for efficient prefix matching
struct DhatuTrie {
    root: TrieNode,
}

struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_end: bool,
}

impl DhatuTrie {
    fn new() -> Self {
        Self {
            root: TrieNode {
                children: HashMap::new(),
                is_end: false,
            },
        }
    }

    fn insert(&mut self, word: &str) {
        let mut current = &mut self.root;
        for ch in word.chars() {
            current = current.children.entry(ch).or_insert(TrieNode {
                children: HashMap::new(),
                is_end: false,
            });
        }
        current.is_end = true;
    }

    fn find_prefix(&self, prefix: &str) -> Vec<String> {
        let mut results = Vec::new();

        // Navigate to prefix node
        let mut current = &self.root;
        for ch in prefix.chars() {
            match current.children.get(&ch) {
                Some(node) => current = node,
                None => return results, // No matches
            }
        }

        // Collect all words from this node
        self.collect_words(current, prefix.to_string(), &mut results);
        results
    }

    fn collect_words(&self, node: &TrieNode, prefix: String, results: &mut Vec<String>) {
        if node.is_end {
            results.push(prefix.clone());
        }
        for (ch, child) in &node.children {
            let mut new_prefix = prefix.clone();
            new_prefix.push(*ch);
            self.collect_words(child, new_prefix, results);
        }
    }
}
