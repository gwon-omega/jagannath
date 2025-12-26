//! Saṅkhyā - Numbers (संख्या)
//!
//! Numeric types and operations.

/// Signed integer types
pub mod ankita {
    /// 8-bit signed (aṣṭaka - अष्टक)
    pub type Ashtaka = i8;
    /// 16-bit signed (ṣoḍaśaka - षोडशक)
    pub type Shodashaka = i16;
    /// 32-bit signed (dvātriṃśaka - द्वात्रिंशक)
    pub type Dvatrimshaka = i32;
    /// 64-bit signed (catuḥṣaṣṭika - चतुःषष्टिक)
    pub type Chatuhshashtika = i64;
    /// 128-bit signed
    pub type Ashtavimshatishata = i128;
    /// Pointer-sized signed
    pub type Suchyanka = isize;
}

/// Unsigned integer types
pub mod anankita {
    /// 8-bit unsigned
    pub type Ashtaka = u8;
    /// 16-bit unsigned
    pub type Shodashaka = u16;
    /// 32-bit unsigned
    pub type Dvatrimshaka = u32;
    /// 64-bit unsigned
    pub type Chatuhshashtika = u64;
    /// 128-bit unsigned
    pub type Ashtavimshatishata = u128;
    /// Pointer-sized unsigned
    pub type Suchyanka = usize;
}

/// Floating point types
pub mod bhinna {
    /// 32-bit float (ardha-śuddhi - अर्धशुद्धि)
    pub type ArdhaShuddhi = f32;
    /// 64-bit float (pūrṇa-śuddhi - पूर्णशुद्धि)
    pub type PurnaShuddhi = f64;
}

/// Mathematical constants
pub mod sthira {
    /// Pi (पाई)
    pub const PI: f64 = std::f64::consts::PI;
    /// Euler's number (e)
    pub const E: f64 = std::f64::consts::E;
    /// Golden ratio (सुवर्ण अनुपात)
    pub const SUVARNA_ANUPATA: f64 = 1.618033988749895;
    /// Square root of 2
    pub const DVA_MULA: f64 = std::f64::consts::SQRT_2;
}

/// Number operations trait
pub trait Ganita {
    /// Add (योग)
    fn yoga(self, other: Self) -> Self;
    /// Subtract (व्यवकलन)
    fn vyavakalana(self, other: Self) -> Self;
    /// Multiply (गुणन)
    fn gunana(self, other: Self) -> Self;
    /// Divide (भाग)
    fn bhaga(self, other: Self) -> Self;
    /// Remainder (शेष)
    fn shesha(self, other: Self) -> Self;
}

macro_rules! impl_ganita {
    ($($t:ty),*) => {
        $(
            impl Ganita for $t {
                fn yoga(self, other: Self) -> Self { self + other }
                fn vyavakalana(self, other: Self) -> Self { self - other }
                fn gunana(self, other: Self) -> Self { self * other }
                fn bhaga(self, other: Self) -> Self { self / other }
                fn shesha(self, other: Self) -> Self { self % other }
            }
        )*
    };
}

impl_ganita!(i8, i16, i32, i64, i128, isize);
impl_ganita!(u8, u16, u32, u64, u128, usize);
impl_ganita!(f32, f64);
