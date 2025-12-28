//! Śṛṅkhalā - Linked List (श्रृंखला - Chain)
//!
//! A doubly-linked list with Sanskrit naming.

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

use core::marker::PhantomData;
use core::ptr::NonNull;

/// Linked list node (Granth - ग्रन्थ)
struct Granth<T> {
    /// Data (Tathya - तथ्य)
    tathya: T,
    /// Previous (Pūrva - पूर्व)
    purva: Option<NonNull<Granth<T>>>,
    /// Next (Uttara - उत्तर)
    uttara: Option<NonNull<Granth<T>>>,
}

/// Doubly-linked list (Śṛṅkhalā - श्रृंखला)
///
/// A chain of nodes connected in both directions.
#[cfg(feature = "alloc")]
pub struct Srinkhala<T> {
    /// Head (Shīrṣa - शीर्ष)
    shirsha: Option<NonNull<Granth<T>>>,
    /// Tail (Puccha - पुच्छ)
    puccha: Option<NonNull<Granth<T>>>,
    /// Length (Dīrghatā - दीर्घता)
    dirghata: usize,
    /// Marker
    _marker: PhantomData<T>,
}

#[cfg(feature = "alloc")]
impl<T> Srinkhala<T> {
    /// Create empty list (नव - nava)
    pub const fn nava() -> Self {
        Self {
            shirsha: None,
            puccha: None,
            dirghata: 0,
            _marker: PhantomData,
        }
    }

    /// Is empty (रिक्त - rikta)
    pub fn rikta(&self) -> bool {
        self.shirsha.is_none()
    }

    /// Length (दीर्घता - dīrghatā)
    pub fn dirghata(&self) -> usize {
        self.dirghata
    }

    /// Push front (अग्रे योजय - agre yojaya)
    pub fn agre_yojaya(&mut self, tathya: T) {
        let node = Box::new(Granth {
            tathya,
            purva: None,
            uttara: self.shirsha,
        });
        let node_ptr = NonNull::new(Box::into_raw(node));

        match self.shirsha {
            Some(old_head) => unsafe {
                (*old_head.as_ptr()).purva = node_ptr;
            },
            None => {
                self.puccha = node_ptr;
            }
        }

        self.shirsha = node_ptr;
        self.dirghata += 1;
    }

    /// Push back (पश्चात् योजय - paścāt yojaya)
    pub fn paschat_yojaya(&mut self, tathya: T) {
        let node = Box::new(Granth {
            tathya,
            purva: self.puccha,
            uttara: None,
        });
        let node_ptr = NonNull::new(Box::into_raw(node));

        match self.puccha {
            Some(old_tail) => unsafe {
                (*old_tail.as_ptr()).uttara = node_ptr;
            },
            None => {
                self.shirsha = node_ptr;
            }
        }

        self.puccha = node_ptr;
        self.dirghata += 1;
    }

    /// Pop front (अग्रतः निष्कासय - agrataḥ niṣkāsaya)
    pub fn agratah_nishkasaya(&mut self) -> Option<T> {
        self.shirsha.map(|node| unsafe {
            let boxed = Box::from_raw(node.as_ptr());
            self.shirsha = boxed.uttara;

            match self.shirsha {
                Some(new_head) => (*new_head.as_ptr()).purva = None,
                None => self.puccha = None,
            }

            self.dirghata -= 1;
            boxed.tathya
        })
    }

    /// Pop back (पश्चात् निष्कासय - paścāt niṣkāsaya)
    pub fn paschat_nishkasaya(&mut self) -> Option<T> {
        self.puccha.map(|node| unsafe {
            let boxed = Box::from_raw(node.as_ptr());
            self.puccha = boxed.purva;

            match self.puccha {
                Some(new_tail) => (*new_tail.as_ptr()).uttara = None,
                None => self.shirsha = None,
            }

            self.dirghata -= 1;
            boxed.tathya
        })
    }

    /// Peek front (अग्रे दृश् - agre dṛś)
    pub fn agre_drsh(&self) -> Option<&T> {
        self.shirsha.map(|node| unsafe { &(*node.as_ptr()).tathya })
    }

    /// Peek back (पश्चात् दृश् - paścāt dṛś)
    pub fn paschat_drsh(&self) -> Option<&T> {
        self.puccha.map(|node| unsafe { &(*node.as_ptr()).tathya })
    }

    /// Clear (शुद्ध - śuddha)
    pub fn shuddha(&mut self) {
        while self.agratah_nishkasaya().is_some() {}
    }
}

#[cfg(feature = "alloc")]
impl<T> Drop for Srinkhala<T> {
    fn drop(&mut self) {
        self.shuddha();
    }
}

#[cfg(feature = "alloc")]
impl<T> Default for Srinkhala<T> {
    fn default() -> Self {
        Self::nava()
    }
}

// Iterator (Yātrī - यात्री)
#[cfg(feature = "alloc")]
pub struct Yatri<'a, T> {
    current: Option<NonNull<Granth<T>>>,
    _marker: PhantomData<&'a T>,
}

#[cfg(feature = "alloc")]
impl<'a, T> Iterator for Yatri<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| unsafe {
            let node_ref = node.as_ref();
            self.current = node_ref.uttara;
            &node_ref.tathya
        })
    }
}

#[cfg(feature = "alloc")]
impl<T> Srinkhala<T> {
    /// Get iterator (यात्री - yātrī)
    pub fn yatri(&self) -> Yatri<'_, T> {
        Yatri {
            current: self.shirsha,
            _marker: PhantomData,
        }
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
#[cfg(feature = "alloc")]
mod tests {
    use super::*;

    #[test]
    fn test_empty_list() {
        let list: Srinkhala<i32> = Srinkhala::nava();
        assert!(list.rikta());
        assert_eq!(list.dirghata(), 0);
    }

    #[test]
    fn test_push_pop_front() {
        let mut list = Srinkhala::nava();
        list.agre_yojaya(1);
        list.agre_yojaya(2);
        list.agre_yojaya(3);

        assert_eq!(list.dirghata(), 3);
        assert_eq!(list.agratah_nishkasaya(), Some(3));
        assert_eq!(list.agratah_nishkasaya(), Some(2));
        assert_eq!(list.agratah_nishkasaya(), Some(1));
        assert!(list.rikta());
    }

    #[test]
    fn test_push_pop_back() {
        let mut list = Srinkhala::nava();
        list.paschat_yojaya(1);
        list.paschat_yojaya(2);
        list.paschat_yojaya(3);

        assert_eq!(list.dirghata(), 3);
        assert_eq!(list.paschat_nishkasaya(), Some(3));
        assert_eq!(list.paschat_nishkasaya(), Some(2));
        assert_eq!(list.paschat_nishkasaya(), Some(1));
        assert!(list.rikta());
    }

    #[test]
    fn test_peek() {
        let mut list = Srinkhala::nava();
        list.paschat_yojaya(1);
        list.paschat_yojaya(2);

        assert_eq!(list.agre_drsh(), Some(&1));
        assert_eq!(list.paschat_drsh(), Some(&2));
    }

    #[test]
    fn test_iterator() {
        let mut list = Srinkhala::nava();
        list.paschat_yojaya(1);
        list.paschat_yojaya(2);
        list.paschat_yojaya(3);

        let items: Vec<_> = list.yatri().copied().collect();
        assert_eq!(items, vec![1, 2, 3]);
    }
}
