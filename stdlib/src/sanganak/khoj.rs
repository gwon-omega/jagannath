//! # Khoj - Search Algorithms (खोज)
//!
//! Various search algorithms for finding elements.

/// Binary search result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KhojPhala<T> {
    /// Found at index
    Mila(usize),
    /// Not found, would insert at index
    NahiMila(usize),
    /// Payload found
    MilaMana(T),
}

impl<T> KhojPhala<T> {
    /// Check if found
    pub fn mila(&self) -> bool {
        matches!(self, Self::Mila(_) | Self::MilaMana(_))
    }

    /// Get index if found
    pub fn suchi(&self) -> Option<usize> {
        match self {
            Self::Mila(i) => Some(*i),
            _ => None,
        }
    }
}

/// Binary search
pub fn dvichhinna_khoj<T: Ord>(slice: &[T], lakshya: &T) -> KhojPhala<()> {
    if slice.is_empty() {
        return KhojPhala::NahiMila(0);
    }

    let mut vaama = 0;
    let mut dakshina = slice.len();

    while vaama < dakshina {
        let madhya = vaama + (dakshina - vaama) / 2;

        match slice[madhya].cmp(lakshya) {
            core::cmp::Ordering::Less => vaama = madhya + 1,
            core::cmp::Ordering::Greater => dakshina = madhya,
            core::cmp::Ordering::Equal => return KhojPhala::Mila(madhya),
        }
    }

    KhojPhala::NahiMila(vaama)
}

/// Binary search with custom comparator
pub fn dvichhinna_khoj_tulana<T, F>(slice: &[T], mut tulana: F) -> KhojPhala<()>
where
    F: FnMut(&T) -> core::cmp::Ordering,
{
    if slice.is_empty() {
        return KhojPhala::NahiMila(0);
    }

    let mut vaama = 0;
    let mut dakshina = slice.len();

    while vaama < dakshina {
        let madhya = vaama + (dakshina - vaama) / 2;

        match tulana(&slice[madhya]) {
            core::cmp::Ordering::Less => vaama = madhya + 1,
            core::cmp::Ordering::Greater => dakshina = madhya,
            core::cmp::Ordering::Equal => return KhojPhala::Mila(madhya),
        }
    }

    KhojPhala::NahiMila(vaama)
}

/// Find lower bound (first element >= value)
pub fn nimna_seema<T: Ord>(slice: &[T], lakshya: &T) -> usize {
    let mut vaama = 0;
    let mut dakshina = slice.len();

    while vaama < dakshina {
        let madhya = vaama + (dakshina - vaama) / 2;

        if slice[madhya] < *lakshya {
            vaama = madhya + 1;
        } else {
            dakshina = madhya;
        }
    }

    vaama
}

/// Find upper bound (first element > value)
pub fn uchha_seema<T: Ord>(slice: &[T], lakshya: &T) -> usize {
    let mut vaama = 0;
    let mut dakshina = slice.len();

    while vaama < dakshina {
        let madhya = vaama + (dakshina - vaama) / 2;

        if slice[madhya] <= *lakshya {
            vaama = madhya + 1;
        } else {
            dakshina = madhya;
        }
    }

    vaama
}

/// Count occurrences in sorted array
pub fn ginti<T: Ord>(slice: &[T], lakshya: &T) -> usize {
    let lower = nimna_seema(slice, lakshya);
    let upper = uchha_seema(slice, lakshya);
    upper - lower
}

/// Linear search
pub fn rekha_khoj<T: PartialEq>(slice: &[T], lakshya: &T) -> Option<usize> {
    slice.iter().position(|x| x == lakshya)
}

/// Linear search with predicate
pub fn rekha_khoj_tulana<T, F>(slice: &[T], mut pred: F) -> Option<usize>
where
    F: FnMut(&T) -> bool,
{
    slice.iter().position(|x| pred(x))
}

/// Find all indices matching predicate
pub fn sabhi_khoj<T, F>(slice: &[T], mut pred: F) -> impl Iterator<Item = usize> + '_
where
    F: FnMut(&T) -> bool + 'static,
{
    slice.iter().enumerate()
        .filter(move |(_, x)| pred(x))
        .map(|(i, _)| i)
}

/// Interpolation search (for uniformly distributed data)
pub fn antarganana_khoj(slice: &[i64], lakshya: i64) -> KhojPhala<()> {
    if slice.is_empty() {
        return KhojPhala::NahiMila(0);
    }

    let mut vaama = 0;
    let mut dakshina = slice.len() - 1;

    while vaama <= dakshina && lakshya >= slice[vaama] && lakshya <= slice[dakshina] {
        if vaama == dakshina {
            return if slice[vaama] == lakshya {
                KhojPhala::Mila(vaama)
            } else {
                KhojPhala::NahiMila(vaama)
            };
        }

        // Interpolate position
        let range = slice[dakshina] - slice[vaama];
        if range == 0 {
            return if slice[vaama] == lakshya {
                KhojPhala::Mila(vaama)
            } else {
                KhojPhala::NahiMila(vaama)
            };
        }

        let pos = vaama + (((dakshina - vaama) as i64 * (lakshya - slice[vaama])) / range) as usize;
        let pos = pos.min(dakshina);

        if slice[pos] == lakshya {
            return KhojPhala::Mila(pos);
        } else if slice[pos] < lakshya {
            vaama = pos + 1;
        } else {
            if pos == 0 {
                return KhojPhala::NahiMila(0);
            }
            dakshina = pos - 1;
        }
    }

    KhojPhala::NahiMila(vaama)
}

/// Jump search (for sorted array)
pub fn kood_khoj<T: Ord>(slice: &[T], lakshya: &T) -> Option<usize> {
    if slice.is_empty() {
        return None;
    }

    let n = slice.len();
    let kood = libm::sqrt(n as f64) as usize;
    let kood = kood.max(1);

    let mut prev = 0;
    let mut curr = kood;

    // Find block
    while curr < n && slice[curr] < *lakshya {
        prev = curr;
        curr += kood;
    }

    // Linear search in block
    for i in prev..curr.min(n) {
        if slice[i] == *lakshya {
            return Some(i);
        }
    }

    None
}

/// Exponential search
pub fn ghataankiya_khoj<T: Ord>(slice: &[T], lakshya: &T) -> KhojPhala<()> {
    if slice.is_empty() {
        return KhojPhala::NahiMila(0);
    }

    if slice[0] == *lakshya {
        return KhojPhala::Mila(0);
    }

    // Find range
    let mut bound = 1;
    while bound < slice.len() && slice[bound] < *lakshya {
        bound *= 2;
    }

    // Binary search in range
    let start = bound / 2;
    let end = bound.min(slice.len());

    match dvichhinna_khoj(&slice[start..end], lakshya) {
        KhojPhala::Mila(i) => KhojPhala::Mila(start + i),
        KhojPhala::NahiMila(i) => KhojPhala::NahiMila(start + i),
        other => other,
    }
}

/// Ternary search (for unimodal functions)
pub fn traya_khoj_adhikatam<F>(vaama: f64, dakshina: f64, mut f: F, prakriya: usize) -> f64
where
    F: FnMut(f64) -> f64,
{
    let mut lo = vaama;
    let mut hi = dakshina;

    for _ in 0..prakriya {
        let m1 = lo + (hi - lo) / 3.0;
        let m2 = hi - (hi - lo) / 3.0;

        if f(m1) < f(m2) {
            lo = m1;
        } else {
            hi = m2;
        }
    }

    (lo + hi) / 2.0
}

/// Find minimum using ternary search
pub fn traya_khoj_nyunatam<F>(vaama: f64, dakshina: f64, mut f: F, prakriya: usize) -> f64
where
    F: FnMut(f64) -> f64,
{
    traya_khoj_adhikatam(vaama, dakshina, |x| -f(x), prakriya)
}

/// Find peak in mountain array (bitonic search)
pub fn shikhar_khoj<T: Ord>(slice: &[T]) -> Option<usize> {
    if slice.len() < 3 {
        return None;
    }

    let mut lo = 0;
    let mut hi = slice.len() - 1;

    while lo < hi {
        let mid = lo + (hi - lo) / 2;

        if slice[mid] < slice[mid + 1] {
            lo = mid + 1;
        } else {
            hi = mid;
        }
    }

    Some(lo)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let arr = [1, 3, 5, 7, 9, 11, 13];

        assert_eq!(dvichhinna_khoj(&arr, &7), KhojPhala::Mila(3));
        assert_eq!(dvichhinna_khoj(&arr, &8), KhojPhala::NahiMila(4));
    }

    #[test]
    fn test_bounds() {
        let arr = [1, 2, 2, 2, 3, 4, 5];

        assert_eq!(nimna_seema(&arr, &2), 1);
        assert_eq!(uchha_seema(&arr, &2), 4);
        assert_eq!(ginti(&arr, &2), 3);
    }

    #[test]
    fn test_jump_search() {
        let arr = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19];

        assert_eq!(kood_khoj(&arr, &11), Some(5));
        assert_eq!(kood_khoj(&arr, &12), None);
    }

    #[test]
    fn test_peak() {
        let arr = [1, 3, 5, 7, 6, 4, 2];
        assert_eq!(shikhar_khoj(&arr), Some(3));
    }
}
