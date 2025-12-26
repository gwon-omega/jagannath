//! Advaita Module (अद्वैत) — Non-Duality
//!
//! Provides utilities for unified memory and type abstraction.

use core::marker::PhantomData;

/// Brahman - The unified memory substrate
///
/// In Advaita, all reality is one (Brahman). This type represents
/// unified memory that can manifest as different types.
#[derive(Debug)]
pub struct Brahman<const SIZE: usize> {
    /// Raw unified memory
    memory: [u8; SIZE],
}

impl<const SIZE: usize> Brahman<SIZE> {
    /// Create new unified memory
    pub const fn new() -> Self {
        Self {
            memory: [0u8; SIZE],
        }
    }

    /// Get the size of the unified memory
    pub const fn size(&self) -> usize {
        SIZE
    }

    /// View the memory as a specific type (Maya overlay)
    ///
    /// # Safety
    /// The caller must ensure the type is properly aligned and fits.
    pub unsafe fn as_maya<T>(&self) -> &Maya<T> {
        &*(self.memory.as_ptr() as *const Maya<T>)
    }

    /// Mutably view the memory as a specific type
    ///
    /// # Safety
    /// The caller must ensure the type is properly aligned and fits.
    pub unsafe fn as_maya_mut<T>(&mut self) -> &mut Maya<T> {
        &mut *(self.memory.as_mut_ptr() as *mut Maya<T>)
    }
}

impl<const SIZE: usize> Default for Brahman<SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

/// Maya - The illusion of separate types
///
/// In Advaita, Maya is the illusion that makes Brahman appear as
/// diverse forms. This wrapper represents a type view over unified memory.
#[repr(transparent)]
pub struct Maya<T> {
    inner: T,
}

impl<T> Maya<T> {
    /// Get the inner value
    pub fn value(&self) -> &T {
        &self.inner
    }

    /// Get mutable access to the inner value
    pub fn value_mut(&mut self) -> &mut T {
        &mut self.inner
    }

    /// Realize the true nature (Brahman) behind the appearance
    pub fn realize(&self) -> *const u8 {
        &self.inner as *const T as *const u8
    }
}

/// Ātman - The self/identity within a value
///
/// Ātman is identical to Brahman, but appears individual.
/// This trait marks types that have identity.
pub trait Atman {
    /// Get the identity of this value
    fn atman_id(&self) -> AtmanId;

    /// Check if two values share the same Ātman (are identical)
    fn same_atman(&self, other: &Self) -> bool {
        self.atman_id() == other.atman_id()
    }
}

/// Identity marker
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AtmanId(pub u64);

impl AtmanId {
    /// Create a new unique identity
    pub fn new() -> Self {
        static COUNTER: core::sync::atomic::AtomicU64 =
            core::sync::atomic::AtomicU64::new(0);
        Self(COUNTER.fetch_add(1, core::sync::atomic::Ordering::Relaxed))
    }
}

impl Default for AtmanId {
    fn default() -> Self {
        Self::new()
    }
}

/// Upadhi - Limiting adjunct
///
/// Upadhis are the conditions that make Brahman appear limited.
/// This represents constraints on a type.
#[derive(Debug, Clone)]
pub struct Upadhi<T, C> {
    value: T,
    constraint: PhantomData<C>,
}

impl<T, C> Upadhi<T, C> {
    /// Create a new constrained value
    pub fn new(value: T) -> Self {
        Self {
            value,
            constraint: PhantomData,
        }
    }

    /// Remove the limiting adjunct (realize unity)
    pub fn remove_upadhi(self) -> T {
        self.value
    }

    /// Get reference to the constrained value
    pub fn as_ref(&self) -> &T {
        &self.value
    }
}

/// Vivarta - Apparent transformation
///
/// In Advaita, the world is vivarta (apparent change) of Brahman,
/// not parinama (real change). This represents zero-cost type conversions.
pub trait Vivarta<Target> {
    /// Transform appearance without changing essence
    fn vivarta(&self) -> &Target;
}

/// Adhyasa - Superimposition
///
/// Adhyasa is the error of superimposing one thing onto another.
/// This represents a type overlay that may or may not be valid.
#[derive(Debug)]
pub enum Adhyasa<T, E> {
    /// Valid superimposition (the type matches)
    Valid(T),
    /// Invalid superimposition (type mismatch)
    Invalid(E),
}

impl<T, E> Adhyasa<T, E> {
    /// Check if the superimposition is valid
    pub fn is_valid(&self) -> bool {
        matches!(self, Adhyasa::Valid(_))
    }

    /// Get the value if valid
    pub fn valid(self) -> Option<T> {
        match self {
            Adhyasa::Valid(v) => Some(v),
            Adhyasa::Invalid(_) => None,
        }
    }

    /// Map the valid value
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Adhyasa<U, E> {
        match self {
            Adhyasa::Valid(v) => Adhyasa::Valid(f(v)),
            Adhyasa::Invalid(e) => Adhyasa::Invalid(e),
        }
    }
}

/// Avidya - Ignorance/lack of knowledge
///
/// Avidya causes us to see difference where there is unity.
/// This represents unknown or uninitialized state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Avidya;

/// Jnana - Knowledge/enlightenment
///
/// Jnana dispels Avidya and reveals the unity of Brahman.
/// This represents complete type information.
#[derive(Debug, Clone)]
pub struct Jnana<T> {
    /// The known type
    pub known_type: PhantomData<T>,
    /// Type name
    pub type_name: &'static str,
    /// Size in bytes
    pub size: usize,
    /// Alignment
    pub align: usize,
}

impl<T> Jnana<T> {
    /// Create knowledge about a type
    pub const fn of() -> Self {
        Self {
            known_type: PhantomData,
            type_name: core::any::type_name::<T>(),
            size: core::mem::size_of::<T>(),
            align: core::mem::align_of::<T>(),
        }
    }
}
