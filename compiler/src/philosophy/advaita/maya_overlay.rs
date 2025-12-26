//! Māyā Overlay - Type System as Illusion (माया)
//!
//! "The world is māyā (illusion)" - but it's a useful illusion
//!
//! In Advaita Vedānta, māyā is the appearance of differentiation
//! on top of undifferentiated Brahman.
//!
//! In Jagannath's type system:
//! - All memory is ultimately bytes (Brahman)
//! - Types are "overlays" that give meaning (māyā)
//! - Type-punning is "seeing through" the illusion
//! - Unsafe operations acknowledge the underlying unity

use std::collections::HashMap;
use std::any::TypeId;

/// Māyā - The type overlay system
pub struct MayaOverlay {
    /// Type definitions (the illusions we project)
    types: HashMap<String, MayaType>,
    /// Type equivalences (sameness hidden by names)
    equivalences: HashMap<String, String>,
    /// Type transmutations (safe type punning)
    transmutations: Vec<Transmutation>,
}

/// A type definition (an overlay on raw bytes)
#[derive(Debug, Clone)]
pub struct MayaType {
    /// Type name
    pub name: String,
    /// Size in bytes
    pub size: usize,
    /// Alignment requirement
    pub alignment: usize,
    /// Inner structure (if composite)
    pub structure: TypeStructure,
    /// Affixes applied
    pub affixes: Vec<String>,
    /// Is this a "satya" (truth/value) type or "mithya" (illusion/reference)?
    pub kind: TypeKind,
}

/// Structure of a type
#[derive(Debug, Clone)]
pub enum TypeStructure {
    /// Primitive (directly maps to bytes)
    Primitive,
    /// Struct with fields
    Struct(Vec<Field>),
    /// Enum with variants
    Enum(Vec<Variant>),
    /// Array of elements
    Array { element: String, length: usize },
    /// Pointer to another type
    Pointer { target: String, mutable: bool },
    /// Function type
    Function { params: Vec<String>, returns: String },
}

/// Field in a struct
#[derive(Debug, Clone)]
pub struct Field {
    pub name: String,
    pub type_name: String,
    pub offset: usize,
}

/// Variant in an enum
#[derive(Debug, Clone)]
pub struct Variant {
    pub name: String,
    pub tag: usize,
    pub fields: Vec<Field>,
}

/// Type kind - satya (value) or mithya (reference)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TypeKind {
    /// Satya (truth) - value types, copied by default
    Satya,
    /// Mithya (illusion) - reference types, point to data elsewhere
    Mithya,
}

/// Safe transmutation rule
#[derive(Debug, Clone)]
pub struct Transmutation {
    /// Source type
    pub from: String,
    /// Target type
    pub to: String,
    /// Is this always safe?
    pub safe: bool,
    /// Conversion function (if any)
    pub conversion: Option<String>,
}

impl MayaOverlay {
    pub fn new() -> Self {
        let mut overlay = Self {
            types: HashMap::new(),
            equivalences: HashMap::new(),
            transmutations: Vec::new(),
        };
        overlay.add_primitive_types();
        overlay
    }

    /// Add primitive types (direct mappings to Brahman/bytes)
    fn add_primitive_types(&mut self) {
        // Integers
        for (name, size) in [
            ("t8", 1), ("t16", 2), ("t32", 4), ("t64", 8),
            ("u8", 1), ("u16", 2), ("u32", 4), ("u64", 8),
        ] {
            self.types.insert(name.to_string(), MayaType {
                name: name.to_string(),
                size,
                alignment: size,
                structure: TypeStructure::Primitive,
                affixes: vec![],
                kind: TypeKind::Satya,
            });
        }

        // Floats
        self.types.insert("d32".to_string(), MayaType {
            name: "d32".to_string(),
            size: 4,
            alignment: 4,
            structure: TypeStructure::Primitive,
            affixes: vec![],
            kind: TypeKind::Satya,
        });
        self.types.insert("d64".to_string(), MayaType {
            name: "d64".to_string(),
            size: 8,
            alignment: 8,
            structure: TypeStructure::Primitive,
            affixes: vec![],
            kind: TypeKind::Satya,
        });

        // Boolean
        self.types.insert("satya".to_string(), MayaType {
            name: "satya".to_string(),
            size: 1,
            alignment: 1,
            structure: TypeStructure::Primitive,
            affixes: vec![],
            kind: TypeKind::Satya,
        });

        // Unit type (empty)
        self.types.insert("()".to_string(), MayaType {
            name: "()".to_string(),
            size: 0,
            alignment: 1,
            structure: TypeStructure::Primitive,
            affixes: vec![],
            kind: TypeKind::Satya,
        });

        // Add safe transmutations
        // t8 ↔ u8 (same bytes, different interpretation)
        self.transmutations.push(Transmutation {
            from: "t8".to_string(),
            to: "u8".to_string(),
            safe: true,
            conversion: None,
        });
    }

    /// Define a new type (create māyā)
    pub fn define_type(&mut self, maya_type: MayaType) {
        self.types.insert(maya_type.name.clone(), maya_type);
    }

    /// Define type equivalence (two names, same essence)
    pub fn define_equivalence(&mut self, name: String, equivalent_to: String) {
        self.equivalences.insert(name, equivalent_to);
    }

    /// Get the "true" type (resolve through aliases)
    pub fn resolve(&self, name: &str) -> Option<&MayaType> {
        // Follow equivalence chain
        let mut current = name;
        for _ in 0..100 { // Prevent infinite loops
            if let Some(equiv) = self.equivalences.get(current) {
                current = equiv;
            } else {
                break;
            }
        }
        self.types.get(current)
    }

    /// Check if types are "ultimately the same" (same Brahman)
    pub fn same_essence(&self, a: &str, b: &str) -> bool {
        let type_a = self.resolve(a);
        let type_b = self.resolve(b);

        match (type_a, type_b) {
            (Some(ta), Some(tb)) => {
                // Same size and alignment = same essence
                ta.size == tb.size && ta.alignment == tb.alignment
            }
            _ => false,
        }
    }

    /// Can we transmute from one type to another?
    pub fn can_transmute(&self, from: &str, to: &str) -> TransmuteResult {
        // Same type = trivial
        if from == to {
            return TransmuteResult::Safe;
        }

        // Check explicit transmutation rules
        for trans in &self.transmutations {
            if trans.from == from && trans.to == to {
                return if trans.safe {
                    TransmuteResult::Safe
                } else {
                    TransmuteResult::RequiresConversion(trans.conversion.clone())
                };
            }
        }

        // Check if same essence (size/alignment)
        if self.same_essence(from, to) {
            return TransmuteResult::UnsafeSameSize;
        }

        // Check resolved types
        let from_type = self.resolve(from);
        let to_type = self.resolve(to);

        match (from_type, to_type) {
            (Some(f), Some(t)) if f.size == t.size => TransmuteResult::UnsafeSameSize,
            (Some(f), Some(t)) if f.size < t.size => TransmuteResult::UnsafeWiden,
            (Some(f), Some(t)) if f.size > t.size => TransmuteResult::UnsafeTruncate,
            _ => TransmuteResult::Impossible,
        }
    }

    /// "Pierce the veil" - get raw bytes interpretation
    pub fn pierce_maya(&self, type_name: &str) -> MayaPiercing {
        let maya_type = self.resolve(type_name);

        match maya_type {
            Some(t) => MayaPiercing {
                size: t.size,
                alignment: t.alignment,
                byte_layout: self.compute_layout(t),
            },
            None => MayaPiercing {
                size: 0,
                alignment: 1,
                byte_layout: vec![],
            },
        }
    }

    /// Compute byte layout for a type
    fn compute_layout(&self, maya_type: &MayaType) -> Vec<ByteInterpretation> {
        match &maya_type.structure {
            TypeStructure::Primitive => {
                vec![ByteInterpretation {
                    offset: 0,
                    size: maya_type.size,
                    meaning: maya_type.name.clone(),
                }]
            }
            TypeStructure::Struct(fields) => {
                fields.iter().map(|f| {
                    let field_type = self.resolve(&f.type_name);
                    ByteInterpretation {
                        offset: f.offset,
                        size: field_type.map(|t| t.size).unwrap_or(0),
                        meaning: format!("{}.{}", maya_type.name, f.name),
                    }
                }).collect()
            }
            TypeStructure::Array { element, length } => {
                let elem_type = self.resolve(element);
                let elem_size = elem_type.map(|t| t.size).unwrap_or(0);
                (0..*length).map(|i| {
                    ByteInterpretation {
                        offset: i * elem_size,
                        size: elem_size,
                        meaning: format!("{}[{}]", maya_type.name, i),
                    }
                }).collect()
            }
            _ => vec![],
        }
    }

    /// Get type by name
    pub fn get(&self, name: &str) -> Option<&MayaType> {
        self.types.get(name)
    }

    /// Check if type is value type (satya)
    pub fn is_satya(&self, name: &str) -> bool {
        self.resolve(name)
            .map(|t| t.kind == TypeKind::Satya)
            .unwrap_or(false)
    }

    /// Check if type is reference type (mithya)
    pub fn is_mithya(&self, name: &str) -> bool {
        self.resolve(name)
            .map(|t| t.kind == TypeKind::Mithya)
            .unwrap_or(false)
    }
}

/// Result of transmutation check
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TransmuteResult {
    /// Safe transmutation (always works)
    Safe,
    /// Requires explicit conversion function
    RequiresConversion(Option<String>),
    /// Unsafe but same size (reinterpret cast)
    UnsafeSameSize,
    /// Unsafe widening (smaller to larger)
    UnsafeWiden,
    /// Unsafe truncating (larger to smaller)
    UnsafeTruncate,
    /// Cannot transmute
    Impossible,
}

/// Raw byte interpretation of a type
#[derive(Debug)]
pub struct MayaPiercing {
    pub size: usize,
    pub alignment: usize,
    pub byte_layout: Vec<ByteInterpretation>,
}

/// Meaning of bytes at an offset
#[derive(Debug, Clone)]
pub struct ByteInterpretation {
    pub offset: usize,
    pub size: usize,
    pub meaning: String,
}

impl Default for MayaOverlay {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primitive_types() {
        let maya = MayaOverlay::new();

        let t32 = maya.get("t32").unwrap();
        assert_eq!(t32.size, 4);
        assert_eq!(t32.kind, TypeKind::Satya);
    }

    #[test]
    fn test_same_essence() {
        let maya = MayaOverlay::new();

        // t32 and u32 have the same essence (4 bytes)
        assert!(maya.same_essence("t32", "u32"));

        // t32 and t64 do not
        assert!(!maya.same_essence("t32", "t64"));
    }

    #[test]
    fn test_transmutation() {
        let maya = MayaOverlay::new();

        // Same type = safe
        assert_eq!(maya.can_transmute("t32", "t32"), TransmuteResult::Safe);

        // t8 ↔ u8 = explicit safe
        assert_eq!(maya.can_transmute("t8", "u8"), TransmuteResult::Safe);

        // t32 → u32 = unsafe same size
        assert_eq!(maya.can_transmute("t32", "u32"), TransmuteResult::UnsafeSameSize);
    }
}
