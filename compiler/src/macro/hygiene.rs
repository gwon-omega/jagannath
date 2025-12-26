//! Macro Hygiene
//!
//! Ensures macros don't accidentally capture or leak identifiers.

use std::collections::HashMap;

/// Hygiene mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HygieneMode {
    /// Full hygiene (default) - macros can't access call-site names
    Full,
    /// Transparent - macros see call-site scope
    Transparent,
    /// Semi-transparent - explicit unhygiene markers needed
    Semi,
}

/// Hygiene context
#[derive(Debug, Clone)]
pub struct HygieneContext {
    /// Current scope ID
    current_scope: ScopeId,
    /// Scope stack
    scope_stack: Vec<ScopeId>,
    /// Scope counter
    scope_counter: usize,
    /// Name bindings per scope
    bindings: HashMap<ScopeId, HashMap<String, String>>,
}

/// Scope identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ScopeId(usize);

impl HygieneContext {
    pub fn new() -> Self {
        Self {
            current_scope: ScopeId(0),
            scope_stack: vec![ScopeId(0)],
            scope_counter: 1,
            bindings: HashMap::new(),
        }
    }

    /// Enter a new macro scope
    pub fn enter_macro(&mut self) -> ScopeId {
        let new_scope = ScopeId(self.scope_counter);
        self.scope_counter += 1;
        self.scope_stack.push(new_scope);
        self.current_scope = new_scope;
        new_scope
    }

    /// Exit macro scope
    pub fn exit_macro(&mut self) {
        self.scope_stack.pop();
        self.current_scope = *self.scope_stack.last().unwrap_or(&ScopeId(0));
    }

    /// Generate hygienic name
    pub fn gensym(&mut self, base: &str) -> String {
        format!("{}__hygiene_{}", base, self.scope_counter)
    }

    /// Bind name in current scope
    pub fn bind(&mut self, name: &str, hygienic_name: &str) {
        self.bindings
            .entry(self.current_scope)
            .or_default()
            .insert(name.to_string(), hygienic_name.to_string());
    }

    /// Resolve name
    pub fn resolve(&self, name: &str) -> Option<&str> {
        // Search from current scope up to root
        for scope in self.scope_stack.iter().rev() {
            if let Some(bindings) = self.bindings.get(scope) {
                if let Some(resolved) = bindings.get(name) {
                    return Some(resolved);
                }
            }
        }
        None
    }

    /// Check if name is in scope
    pub fn in_scope(&self, name: &str) -> bool {
        self.resolve(name).is_some()
    }
}

impl Default for HygieneContext {
    fn default() -> Self {
        Self::new()
    }
}
