//! # Upahasa - Mocking (उपहास)
//!
//! Mock and spy utilities for testing.
//!
//! > **"उपहासः परीक्षणस्य कला"**
//! > *"Mocking is the art of testing"*

#[cfg(feature = "alloc")]
extern crate alloc;
#[cfg(feature = "alloc")]
use alloc::boxed::Box;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;
use core::cell::Cell;
use core::cell::RefCell;

// ============================================================================
// CALL RECORDER
// ============================================================================

/// Records function calls
#[cfg(feature = "alloc")]
pub struct AahvanAbhilekha<T> {
    calls: RefCell<Vec<T>>,
}

#[cfg(feature = "alloc")]
impl<T: Clone> AahvanAbhilekha<T> {
    /// Create new recorder
    pub fn nava() -> Self {
        Self {
            calls: RefCell::new(Vec::new()),
        }
    }

    /// Record a call
    pub fn abhilekha(&self, args: T) {
        self.calls.borrow_mut().push(args);
    }

    /// Get all calls
    pub fn sabhi_aahvan(&self) -> Vec<T> {
        self.calls.borrow().clone()
    }

    /// Get call count
    pub fn aahvan_sankhya(&self) -> usize {
        self.calls.borrow().len()
    }

    /// Check if called
    pub fn aahvan_hua(&self) -> bool {
        !self.calls.borrow().is_empty()
    }

    /// Get nth call
    pub fn aahvan(&self, n: usize) -> Option<T> {
        self.calls.borrow().get(n).cloned()
    }

    /// Get last call
    pub fn antim_aahvan(&self) -> Option<T> {
        self.calls.borrow().last().cloned()
    }

    /// Clear recorded calls
    pub fn shuddhikarana(&self) {
        self.calls.borrow_mut().clear();
    }
}

// ============================================================================
// MOCK VALUE
// ============================================================================

/// Mock that returns preset values
#[cfg(feature = "alloc")]
pub struct ChalMulya<T> {
    values: RefCell<Vec<T>>,
    index: Cell<usize>,
    cycle: bool,
}

#[cfg(feature = "alloc")]
impl<T: Clone> ChalMulya<T> {
    /// Create mock with values
    pub fn nava(values: Vec<T>) -> Self {
        Self {
            values: RefCell::new(values),
            index: Cell::new(0),
            cycle: false,
        }
    }

    /// Create cycling mock
    pub fn chakra(values: Vec<T>) -> Self {
        Self {
            values: RefCell::new(values),
            index: Cell::new(0),
            cycle: true,
        }
    }

    /// Get next value
    pub fn agla(&self) -> Option<T> {
        let values = self.values.borrow();
        let idx = self.index.get();

        if idx >= values.len() {
            if self.cycle && !values.is_empty() {
                self.index.set(0);
                Some(values[0].clone())
            } else {
                None
            }
        } else {
            self.index.set(idx + 1);
            Some(values[idx].clone())
        }
    }

    /// Reset to start
    pub fn punasthapana(&self) {
        self.index.set(0);
    }

    /// Get remaining count
    pub fn shesh_sankhya(&self) -> usize {
        let values = self.values.borrow();
        values.len().saturating_sub(self.index.get())
    }
}

// ============================================================================
// MOCK FUNCTION
// ============================================================================

/// Mock function with configurable behavior
#[cfg(feature = "alloc")]
pub struct ChalKarya<A, R> {
    handler: RefCell<Box<dyn Fn(A) -> R>>,
    calls: RefCell<Vec<A>>,
}

#[cfg(feature = "alloc")]
impl<A: Clone + 'static, R: Default + 'static> ChalKarya<A, R> {
    /// Create mock with default return
    pub fn nava() -> Self {
        Self {
            handler: RefCell::new(Box::new(|_| R::default())),
            calls: RefCell::new(Vec::new()),
        }
    }
}

#[cfg(feature = "alloc")]
impl<A: Clone + 'static, R: 'static> ChalKarya<A, R> {
    /// Create mock with handler
    pub fn nava_vyavahari(handler: impl Fn(A) -> R + 'static) -> Self {
        Self {
            handler: RefCell::new(Box::new(handler)),
            calls: RefCell::new(Vec::new()),
        }
    }

    /// Call the mock
    pub fn aahvan(&self, args: A) -> R {
        self.calls.borrow_mut().push(args.clone());
        (self.handler.borrow())(args)
    }

    /// Set handler
    pub fn vyavahari_sthapita(&self, handler: impl Fn(A) -> R + 'static) {
        *self.handler.borrow_mut() = Box::new(handler);
    }

    /// Get call count
    pub fn aahvan_sankhya(&self) -> usize {
        self.calls.borrow().len()
    }

    /// Get calls
    pub fn sabhi_aahvan(&self) -> Vec<A> {
        self.calls.borrow().clone()
    }
}

// ============================================================================
// COUNTER
// ============================================================================

/// Simple counter for tracking
pub struct Ganaka {
    count: Cell<usize>,
}

impl Ganaka {
    /// Create new counter
    pub const fn nava() -> Self {
        Self {
            count: Cell::new(0),
        }
    }

    /// Increment and return new value
    pub fn vriddhi(&self) -> usize {
        let new = self.count.get() + 1;
        self.count.set(new);
        new
    }

    /// Get current value
    pub fn mana(&self) -> usize {
        self.count.get()
    }

    /// Reset to zero
    pub fn punasthapana(&self) {
        self.count.set(0);
    }

    /// Set specific value
    pub fn sthapita(&self, mana: usize) {
        self.count.set(mana);
    }
}

impl Default for Ganaka {
    fn default() -> Self {
        Self::nava()
    }
}

// ============================================================================
// FLAG
// ============================================================================

/// Boolean flag for tracking
pub struct Dhvaja {
    value: Cell<bool>,
}

impl Dhvaja {
    /// Create new flag (false)
    pub const fn nava() -> Self {
        Self {
            value: Cell::new(false),
        }
    }

    /// Create flag with initial value
    pub const fn nava_mana(mana: bool) -> Self {
        Self {
            value: Cell::new(mana),
        }
    }

    /// Set to true
    pub fn upar(&self) {
        self.value.set(true);
    }

    /// Set to false
    pub fn niche(&self) {
        self.value.set(false);
    }

    /// Toggle
    pub fn badalna(&self) -> bool {
        let new = !self.value.get();
        self.value.set(new);
        new
    }

    /// Get value
    pub fn mana(&self) -> bool {
        self.value.get()
    }

    /// Check if raised
    pub fn upar_hai(&self) -> bool {
        self.value.get()
    }
}

impl Default for Dhvaja {
    fn default() -> Self {
        Self::nava()
    }
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[cfg(feature = "alloc")]
    fn test_call_recorder() {
        let recorder: AahvanAbhilekha<i32> = AahvanAbhilekha::nava();

        recorder.abhilekha(1);
        recorder.abhilekha(2);
        recorder.abhilekha(3);

        assert_eq!(recorder.aahvan_sankhya(), 3);
        assert_eq!(recorder.aahvan(0), Some(1));
        assert_eq!(recorder.antim_aahvan(), Some(3));
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_mock_value() {
        let mock = ChalMulya::nava(vec![1, 2, 3]);

        assert_eq!(mock.agla(), Some(1));
        assert_eq!(mock.agla(), Some(2));
        assert_eq!(mock.agla(), Some(3));
        assert_eq!(mock.agla(), None);
    }

    #[test]
    #[cfg(feature = "alloc")]
    fn test_cycling_mock() {
        let mock = ChalMulya::chakra(vec![1, 2]);

        assert_eq!(mock.agla(), Some(1));
        assert_eq!(mock.agla(), Some(2));
        assert_eq!(mock.agla(), Some(1)); // cycles
    }

    #[test]
    fn test_counter() {
        let counter = Ganaka::nava();

        assert_eq!(counter.mana(), 0);
        assert_eq!(counter.vriddhi(), 1);
        assert_eq!(counter.vriddhi(), 2);
        assert_eq!(counter.mana(), 2);
    }

    #[test]
    fn test_flag() {
        let flag = Dhvaja::nava();

        assert!(!flag.upar_hai());
        flag.upar();
        assert!(flag.upar_hai());
        flag.badalna();
        assert!(!flag.upar_hai());
    }
}
