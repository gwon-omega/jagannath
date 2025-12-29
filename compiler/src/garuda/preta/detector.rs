//! # Preta Detector - Ghost Process Detection
//!
//! Detects "ghost" resources - dangling references, leaked handles,
//! zombie processes, and orphaned resources.
//!
//! # Sanskrit Foundation
//! Preta (प्रेत) are "hungry ghosts" - spirits of the deceased that
//! haven't achieved liberation due to unfinished karma. Similarly,
//! leaked resources are "undead" - allocated but never freed.
//!
//! # Philosophy: Karma of Resources
//! Every resource has karma (action):
//! - Good karma: allocate → use → free (reaches moksha)
//! - Bad karma: allocate → use → never free (becomes preta)
//!
//! # Algorithm
//! Uses forward dataflow analysis to track resource states:
//! - GEN: allocation sites create resources
//! - KILL: deallocation sites destroy resources
//! - At function exit: any still-allocated resources are leaks

use crate::lexer::token::Span;
use crate::parser::ast::{Ast, Block, Expr, FunctionDef, Item, Stmt};
use std::collections::{HashMap, HashSet};

/// Hunger level for ghost resources
#[derive(Debug, Clone, PartialEq)]
pub enum HungerLevel {
    /// Minor leak, not urgent
    Mild,
    /// Moderate leak
    Hungry,
    /// Severe leak
    Starving,
    /// Critical leak
    Ravenous,
}

impl HungerLevel {
    /// Determine hunger level from context
    pub fn from_context(in_loop: bool, multiple_paths: bool, resource_type: &GhostType) -> Self {
        match (in_loop, multiple_paths, resource_type) {
            (true, _, GhostType::MemoryLeak) => HungerLevel::Ravenous, // Loop leak = critical
            (true, _, _) => HungerLevel::Starving,
            (_, true, GhostType::MemoryLeak) => HungerLevel::Hungry,
            (_, _, GhostType::DeadCode) => HungerLevel::Mild,
            _ => HungerLevel::Hungry,
        }
    }
}

/// Types of ghost resources
#[derive(Debug, Clone, PartialEq)]
pub enum GhostType {
    /// Dangling pointer/reference
    DanglingReference,
    /// Leaked memory
    MemoryLeak,
    /// Unclosed file handle
    FileHandleLeak,
    /// Orphaned socket
    SocketLeak,
    /// Zombie process
    ZombieProcess,
    /// Abandoned lock
    AbandonedLock,
    /// Unreachable code
    DeadCode,
}

impl GhostType {
    /// Get Sanskrit name for this ghost type
    pub fn sanskrit_name(&self) -> &'static str {
        match self {
            GhostType::DanglingReference => "मृत-सन्दर्भ", // dead reference
            GhostType::MemoryLeak => "स्मृति-प्रेत",        // memory ghost
            GhostType::FileHandleLeak => "कोष-प्रेत",     // file ghost
            GhostType::SocketLeak => "जाल-प्रेत",         // network ghost
            GhostType::ZombieProcess => "मृत-प्रक्रिया",   // zombie process
            GhostType::AbandonedLock => "त्यक्त-ताल",     // abandoned lock
            GhostType::DeadCode => "मृत-कोड",            // dead code
        }
    }

    /// Get the associated Naraka (hell) for this ghost type
    pub fn naraka_name(&self) -> &'static str {
        match self {
            GhostType::DanglingReference => "Tamisram", // Darkness (use-after-free)
            GhostType::MemoryLeak => "Suchimukha",      // Needle torture (slow death)
            GhostType::FileHandleLeak => "Suchimukha",
            GhostType::SocketLeak => "Suchimukha",
            GhostType::ZombieProcess => "Pranarodha", // Life blocking
            GhostType::AbandonedLock => "Kalasutra",  // Time binding (deadlock)
            GhostType::DeadCode => "Avichi",          // Waveless (unreachable)
        }
    }
}

/// A detected ghost resource
#[derive(Debug, Clone)]
pub struct Ghost {
    /// Type of ghost
    pub ghost_type: GhostType,
    /// Location in code
    pub location: Span,
    /// Description of the ghost
    pub description: String,
    /// Suggested exorcism (fix)
    pub exorcism: String,
    /// Hunger level
    pub hunger: HungerLevel,
    /// Variable name
    pub variable_name: Option<String>,
}

impl Ghost {
    pub fn new(
        ghost_type: GhostType,
        location: Span,
        description: String,
        exorcism: String,
        variable_name: Option<String>,
    ) -> Self {
        Self {
            hunger: HungerLevel::Hungry,
            ghost_type,
            location,
            description,
            exorcism,
            variable_name,
        }
    }

    pub fn with_hunger(mut self, hunger: HungerLevel) -> Self {
        self.hunger = hunger;
        self
    }
}

/// Resource allocation info
#[derive(Debug, Clone)]
struct ResourceAllocation {
    /// Variable name
    name: String,
    /// Allocation location
    location: Span,
    /// Resource type
    resource_type: GhostType,
    /// Is it in a loop?
    in_loop: bool,
}

/// Preta (Ghost) Detector
///
/// Uses dataflow analysis to detect resource leaks.
pub struct PretaDetector {
    /// Detected ghosts
    ghosts: Vec<Ghost>,
    /// Resource tracking
    resources: HashMap<String, ResourceAllocation>,
    /// Freed resources (for double-free detection)
    freed: HashSet<String>,
    /// Current loop depth
    loop_depth: usize,
    /// Known allocation functions
    alloc_functions: HashSet<String>,
    /// Known deallocation functions
    dealloc_functions: HashSet<String>,
}

impl Default for PretaDetector {
    fn default() -> Self {
        Self::new()
    }
}

impl PretaDetector {
    pub fn new() -> Self {
        let mut alloc_functions = HashSet::new();
        let mut dealloc_functions = HashSet::new();

        // Sanskrit allocation functions
        alloc_functions.insert("nirmā".to_string()); // create/construct
        alloc_functions.insert("nava".to_string()); // new
        alloc_functions.insert("āvṛtti".to_string()); // allocate
        alloc_functions.insert("graha".to_string()); // grab/acquire
        alloc_functions.insert("sambandha".to_string()); // connect
        alloc_functions.insert("udghaṭana".to_string()); // open
        alloc_functions.insert("tāla_graha".to_string()); // acquire lock

        // English allocation functions
        alloc_functions.insert("new".to_string());
        alloc_functions.insert("alloc".to_string());
        alloc_functions.insert("malloc".to_string());
        alloc_functions.insert("open".to_string());
        alloc_functions.insert("connect".to_string());
        alloc_functions.insert("acquire".to_string());
        alloc_functions.insert("lock".to_string());

        // Sanskrit deallocation functions
        dealloc_functions.insert("mukta".to_string()); // free/liberate
        dealloc_functions.insert("tyaja".to_string()); // release
        dealloc_functions.insert("viyoga".to_string()); // disconnect
        dealloc_functions.insert("samāpana".to_string()); // close
        dealloc_functions.insert("tāla_mukta".to_string()); // release lock
        dealloc_functions.insert("naśa".to_string()); // destroy

        // English deallocation functions
        dealloc_functions.insert("free".to_string());
        dealloc_functions.insert("release".to_string());
        dealloc_functions.insert("close".to_string());
        dealloc_functions.insert("disconnect".to_string());
        dealloc_functions.insert("drop".to_string());
        dealloc_functions.insert("unlock".to_string());
        dealloc_functions.insert("dispose".to_string());

        Self {
            ghosts: Vec::new(),
            resources: HashMap::new(),
            freed: HashSet::new(),
            loop_depth: 0,
            alloc_functions,
            dealloc_functions,
        }
    }

    /// Analyze code for ghost resources
    pub fn analyze(&mut self, ast: &Ast) -> Vec<Ghost> {
        self.ghosts.clear();
        self.resources.clear();
        self.freed.clear();

        // Visit each function
        for item in &ast.items {
            if let Item::Function(func) = item {
                self.analyze_function(func);
            }
        }

        self.ghosts.clone()
    }

    /// Detect ghost resources (alias for analyze)
    pub fn detect(&mut self, ast: &Ast) -> Vec<Ghost> {
        self.analyze(ast)
    }

    /// Analyze a single function for resource leaks
    fn analyze_function(&mut self, func: &FunctionDef) {
        // Reset per-function state
        self.resources.clear();
        self.freed.clear();
        self.loop_depth = 0;

        // Collect allocations and deallocations from statements
        self.visit_block(&func.body);

        // At function end, check for leaks
        for (name, alloc) in &self.resources {
            if !self.freed.contains(name) {
                let hunger = HungerLevel::from_context(
                    alloc.in_loop,
                    false, // Would need more analysis for multiple paths
                    &alloc.resource_type,
                );

                let ghost = Ghost::new(
                    alloc.resource_type.clone(),
                    alloc.location.clone(),
                    format!(
                        "Resource '{}' allocated but never freed - becomes preta (hungry ghost)",
                        name
                    ),
                    format!(
                        "Call {}() on '{}' before function exit, or use RAII pattern",
                        self.get_dealloc_function(&alloc.resource_type),
                        name
                    ),
                    Some(name.clone()),
                )
                .with_hunger(hunger);

                self.ghosts.push(ghost);
            }
        }
    }

    /// Count basic blocks in a block (simplified)
    fn count_blocks(&self, block: &Block) -> usize {
        let mut count = 1;
        for stmt in &block.stmts {
            count += self.count_stmt_blocks(stmt);
        }
        count
    }

    fn count_stmt_blocks(&self, stmt: &Stmt) -> usize {
        match stmt {
            Stmt::If {
                then_block,
                else_block,
                ..
            } => {
                let mut count = self.count_blocks(then_block);
                if let Some(eb) = else_block {
                    count += self.count_blocks(eb);
                }
                count
            }
            Stmt::Loop { body, .. } => self.count_blocks(body),
            Stmt::Match { arms, .. } => arms.len(),
            _ => 0,
        }
    }

    /// Visit a block and track resources
    fn visit_block(&mut self, block: &Block) {
        for stmt in &block.stmts {
            self.visit_stmt(stmt);
        }
    }

    /// Visit a statement
    fn visit_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let {
                name,
                value: Some(value),
                span,
                ..
            } => {
                // Check if this is an allocation
                if let Some(alloc_type) = self.check_allocation(value) {
                    self.resources.insert(
                        name.name.clone(),
                        ResourceAllocation {
                            name: name.name.clone(),
                            location: span.clone(),
                            resource_type: alloc_type,
                            in_loop: self.loop_depth > 0,
                        },
                    );
                }
            }
            Stmt::Expr(expr) => {
                // Check for deallocation calls
                self.check_deallocation(expr);
            }
            Stmt::Loop { body, .. } => {
                self.loop_depth += 1;
                self.visit_block(body);
                self.loop_depth -= 1;
            }
            Stmt::If {
                then_block,
                else_block,
                ..
            } => {
                self.visit_block(then_block);
                if let Some(eb) = else_block {
                    self.visit_block(eb);
                }
            }
            Stmt::Match { arms, .. } => {
                for arm in arms {
                    if let Expr::Block(block) = &arm.body {
                        for stmt in &block.stmts {
                            self.visit_stmt(stmt);
                        }
                    }
                }
            }
            Stmt::Return {
                value: Some(value), ..
            } => {
                // Returning a resource means it escapes
                if let Expr::Identifier(ident) = value {
                    self.freed.insert(ident.name.clone()); // Mark as "escaped" (not leaked)
                }
            }
            _ => {}
        }
    }

    /// Check if an expression is an allocation
    fn check_allocation(&self, expr: &Expr) -> Option<GhostType> {
        match expr {
            Expr::Call { callee, .. } => {
                if let Expr::Identifier(ident) = callee.as_ref() {
                    let name = &ident.name;
                    if self.alloc_functions.contains(name) {
                        return Some(self.guess_resource_type(name));
                    }
                }
            }
            Expr::MethodCall { method, .. } => {
                if self.alloc_functions.contains(&method.name) {
                    return Some(self.guess_resource_type(&method.name));
                }
            }
            _ => {}
        }
        None
    }

    /// Check if an expression is a deallocation
    fn check_deallocation(&mut self, expr: &Expr) {
        match expr {
            Expr::Call { callee, args, .. } => {
                if let Expr::Identifier(ident) = callee.as_ref() {
                    if self.dealloc_functions.contains(&ident.name) {
                        // First argument is typically the resource to free
                        if let Some(Expr::Identifier(arg_ident)) = args.first() {
                            // Check for double-free
                            if self.freed.contains(&arg_ident.name) {
                                self.ghosts.push(Ghost::new(
                                    GhostType::DanglingReference,
                                    ident.span.clone(),
                                    format!(
                                        "Resource '{}' freed multiple times - double-free vulnerability",
                                        arg_ident.name
                                    ),
                                    "Remove duplicate free, or check if already freed".to_string(),
                                    Some(arg_ident.name.clone()),
                                ));
                            }
                            self.freed.insert(arg_ident.name.clone());
                        }
                    }
                }
            }
            Expr::MethodCall {
                receiver, method, ..
            } => {
                if self.dealloc_functions.contains(&method.name) {
                    if let Expr::Identifier(ident) = receiver.as_ref() {
                        if self.freed.contains(&ident.name) {
                            self.ghosts.push(Ghost::new(
                                GhostType::DanglingReference,
                                ident.span.clone(),
                                format!(
                                    "Resource '{}' freed multiple times - double-free vulnerability",
                                    ident.name
                                ),
                                "Remove duplicate close/free, or check if already closed".to_string(),
                                Some(ident.name.clone()),
                            ));
                        }
                        self.freed.insert(ident.name.clone());
                    }
                }
            }
            _ => {}
        }
    }

    /// Guess resource type from function name
    fn guess_resource_type(&self, func_name: &str) -> GhostType {
        let name_lower = func_name.to_lowercase();
        if name_lower.contains("file") || name_lower.contains("open") || name_lower.contains("koṣa")
        {
            GhostType::FileHandleLeak
        } else if name_lower.contains("socket")
            || name_lower.contains("connect")
            || name_lower.contains("jāla")
        {
            GhostType::SocketLeak
        } else if name_lower.contains("lock") || name_lower.contains("tāla") {
            GhostType::AbandonedLock
        } else if name_lower.contains("thread") || name_lower.contains("tantu") {
            GhostType::ZombieProcess
        } else {
            GhostType::MemoryLeak
        }
    }

    /// Get appropriate deallocation function for a resource type
    fn get_dealloc_function(&self, resource_type: &GhostType) -> &'static str {
        match resource_type {
            GhostType::MemoryLeak => "mukta",         // free
            GhostType::FileHandleLeak => "samāpana",  // close
            GhostType::SocketLeak => "viyoga",        // disconnect
            GhostType::AbandonedLock => "tāla_mukta", // unlock
            GhostType::ZombieProcess => "pratīkṣa",   // join
            _ => "tyaja",                             // release
        }
    }

    /// Get detected ghosts
    pub fn ghosts(&self) -> &[Ghost] {
        &self.ghosts
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preta_detector_new() {
        let detector = PretaDetector::new();
        assert!(detector.ghosts.is_empty());
        assert!(detector.alloc_functions.contains("new"));
        assert!(detector.alloc_functions.contains("nirmā"));
        assert!(detector.dealloc_functions.contains("free"));
        assert!(detector.dealloc_functions.contains("mukta"));
    }

    #[test]
    fn test_ghost_type_sanskrit_names() {
        assert_eq!(GhostType::MemoryLeak.sanskrit_name(), "स्मृति-प्रेत");
        assert_eq!(GhostType::FileHandleLeak.sanskrit_name(), "कोष-प्रेत");
    }

    #[test]
    fn test_ghost_type_naraka_mapping() {
        assert_eq!(GhostType::MemoryLeak.naraka_name(), "Suchimukha");
        assert_eq!(GhostType::DanglingReference.naraka_name(), "Tamisram");
        assert_eq!(GhostType::AbandonedLock.naraka_name(), "Kalasutra");
    }

    #[test]
    fn test_hunger_level_from_context() {
        // Loop + memory leak = critical
        let hunger = HungerLevel::from_context(true, false, &GhostType::MemoryLeak);
        assert_eq!(hunger, HungerLevel::Ravenous);

        // No loop = hungry
        let hunger = HungerLevel::from_context(false, false, &GhostType::MemoryLeak);
        assert_eq!(hunger, HungerLevel::Hungry);

        // Dead code = mild
        let hunger = HungerLevel::from_context(false, false, &GhostType::DeadCode);
        assert_eq!(hunger, HungerLevel::Mild);
    }
}
