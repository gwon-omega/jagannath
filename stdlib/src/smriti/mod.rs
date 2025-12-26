//! Smṛti - Memory Management (स्मृति)
//!
//! Memory allocation and smart pointers.

#[cfg(feature = "alloc")]
use alloc::boxed::Box as AllocBox;
#[cfg(feature = "alloc")]
use alloc::rc::Rc as AllocRc;
#[cfg(feature = "alloc")]
use alloc::sync::Arc as AllocArc;

/// Box/Heap allocation (Peṭī - पेटी)
#[cfg(feature = "alloc")]
pub type Peti<T> = AllocBox<T>;

/// Reference counted (Gaṇanā - गणना)
#[cfg(feature = "alloc")]
pub type Ganana<T> = AllocRc<T>;

/// Atomic reference counted (Paramāṇu-Gaṇanā - परमाणुगणना)
#[cfg(feature = "alloc")]
pub type ParamanuGanana<T> = AllocArc<T>;

/// Cell (Koṣṭha - कोष्ठ)
#[cfg(feature = "std")]
pub type Koshtha<T> = std::cell::Cell<T>;

/// RefCell (Sandarbha-Koṣṭha - सन्दर्भकोष्ठ)
#[cfg(feature = "std")]
pub type SandarbhaKoshtha<T> = std::cell::RefCell<T>;

/// Mutex (Rakṣaka - रक्षक)
#[cfg(feature = "std")]
pub type Rakshaka<T> = std::sync::Mutex<T>;

/// RwLock (Paṭhana-Lekhana-Rakṣaka - पठनलेखनरक्षक)
#[cfg(feature = "std")]
pub type PathanaLekhanaRakshaka<T> = std::sync::RwLock<T>;

/// Memory allocation trait
pub trait SmritiVidhi {
    /// Allocate (आबन्धन)
    fn abandhana(size: usize) -> *mut u8;

    /// Deallocate (विमोचन)
    unsafe fn vimochana(ptr: *mut u8, size: usize);

    /// Reallocate (पुनराबन्धन)
    unsafe fn punarabandhan(ptr: *mut u8, old_size: usize, new_size: usize) -> *mut u8;
}

/// Global allocator wrapper
pub struct Vibhajaka;

#[cfg(feature = "alloc")]
impl SmritiVidhi for Vibhajaka {
    fn abandhana(size: usize) -> *mut u8 {
        use alloc::alloc::{alloc, Layout};
        unsafe {
            let layout = Layout::from_size_align_unchecked(size, 8);
            alloc(layout)
        }
    }

    unsafe fn vimochana(ptr: *mut u8, size: usize) {
        use alloc::alloc::{dealloc, Layout};
        let layout = Layout::from_size_align_unchecked(size, 8);
        dealloc(ptr, layout)
    }

    unsafe fn punarabandhan(ptr: *mut u8, old_size: usize, new_size: usize) -> *mut u8 {
        use alloc::alloc::{realloc, Layout};
        let layout = Layout::from_size_align_unchecked(old_size, 8);
        realloc(ptr, layout, new_size)
    }
}
