//! Borrow Checker
//!
//! Implements linear type checking for:
//! - -l (linear/owned) types: must be used exactly once
//! - -b (borrowed) types: cannot outlive owner
//! - Ensuring no use-after-free

use crate::parser::ast::*;
use std::collections::HashMap;

/// Borrow checker
pub struct BorrowChecker {
    /// Owned values and their state
    owned: HashMap<String, OwnershipState>,
    /// Active borrows
    borrows: Vec<BorrowInfo>,
}

/// Ownership state for a value
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OwnershipState {
    /// Value is owned and valid
    Owned,
    /// Value has been moved
    Moved,
    /// Value is borrowed
    Borrowed { mutable: bool },
    /// Value has been consumed (linear type)
    Consumed,
}

/// Information about an active borrow
#[derive(Debug, Clone)]
pub struct BorrowInfo {
    pub borrower: String,
    pub owner: String,
    pub mutable: bool,
    pub region: u8,
}

impl BorrowChecker {
    pub fn new() -> Self {
        Self {
            owned: HashMap::new(),
            borrows: Vec::new(),
        }
    }

    /// Check borrow rules for a function
    pub fn check_function(&mut self, func: &FunctionDef) -> Result<(), BorrowError> {
        todo!("Implement borrow checking")
    }

    /// Record a new owned value
    pub fn record_owned(&mut self, name: String) {
        self.owned.insert(name, OwnershipState::Owned);
    }

    /// Record a move
    pub fn record_move(&mut self, from: &str, to: String) -> Result<(), BorrowError> {
        match self.owned.get(from) {
            Some(OwnershipState::Owned) => {
                self.owned.insert(from.to_string(), OwnershipState::Moved);
                self.owned.insert(to, OwnershipState::Owned);
                Ok(())
            }
            Some(OwnershipState::Moved) => Err(BorrowError::UseAfterMove {
                name: from.to_string(),
            }),
            Some(OwnershipState::Consumed) => Err(BorrowError::UseAfterConsume {
                name: from.to_string(),
            }),
            Some(OwnershipState::Borrowed { .. }) => Err(BorrowError::MoveWhileBorrowed {
                name: from.to_string(),
            }),
            None => Err(BorrowError::UnknownValue(from.to_string())),
        }
    }

    /// Record a borrow
    pub fn record_borrow(
        &mut self,
        borrower: String,
        owner: &str,
        mutable: bool,
        region: u8,
    ) -> Result<(), BorrowError> {
        match self.owned.get(owner) {
            Some(OwnershipState::Owned) => {
                // Check for conflicting borrows
                for borrow in &self.borrows {
                    if borrow.owner == owner {
                        if mutable || borrow.mutable {
                            return Err(BorrowError::ConflictingBorrow {
                                owner: owner.to_string(),
                                existing: borrow.borrower.clone(),
                                new: borrower,
                            });
                        }
                    }
                }

                self.owned.insert(
                    owner.to_string(),
                    OwnershipState::Borrowed { mutable },
                );
                self.borrows.push(BorrowInfo {
                    borrower,
                    owner: owner.to_string(),
                    mutable,
                    region,
                });
                Ok(())
            }
            Some(OwnershipState::Moved) => Err(BorrowError::BorrowAfterMove {
                name: owner.to_string(),
            }),
            _ => Err(BorrowError::UnknownValue(owner.to_string())),
        }
    }

    /// Release a borrow
    pub fn release_borrow(&mut self, borrower: &str) {
        self.borrows.retain(|b| b.borrower != borrower);

        // Update owner state if no more borrows
        let owner_to_update: Vec<_> = self
            .owned
            .iter()
            .filter(|(_, state)| matches!(state, OwnershipState::Borrowed { .. }))
            .map(|(name, _)| name.clone())
            .collect();

        for owner in owner_to_update {
            if !self.borrows.iter().any(|b| b.owner == owner) {
                self.owned.insert(owner, OwnershipState::Owned);
            }
        }
    }

    /// Consume a linear value
    pub fn consume_linear(&mut self, name: &str) -> Result<(), BorrowError> {
        match self.owned.get(name) {
            Some(OwnershipState::Owned) => {
                self.owned.insert(name.to_string(), OwnershipState::Consumed);
                Ok(())
            }
            Some(OwnershipState::Consumed) => Err(BorrowError::DoubleConsume {
                name: name.to_string(),
            }),
            Some(OwnershipState::Moved) => Err(BorrowError::UseAfterMove {
                name: name.to_string(),
            }),
            _ => Err(BorrowError::UnknownValue(name.to_string())),
        }
    }

    /// Check that all linear values have been consumed
    pub fn check_linear_consumed(&self) -> Result<(), BorrowError> {
        for (name, state) in &self.owned {
            if *state == OwnershipState::Owned {
                // Should check if it's a linear type and wasn't consumed
                // For now, we'll skip this check
            }
        }
        Ok(())
    }
}

/// Borrow error
#[derive(Debug)]
pub enum BorrowError {
    UseAfterMove { name: String },
    UseAfterConsume { name: String },
    MoveWhileBorrowed { name: String },
    BorrowAfterMove { name: String },
    ConflictingBorrow {
        owner: String,
        existing: String,
        new: String,
    },
    DoubleConsume { name: String },
    UnknownValue(String),
    LinearNotConsumed { name: String },
}

impl Default for BorrowChecker {
    fn default() -> Self {
        Self::new()
    }
}
