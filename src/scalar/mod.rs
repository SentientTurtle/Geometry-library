use std::fmt::Debug;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// Trait for Real number "scalar" types; Those that implement addition/subtraction/multiplication/division, as well as exponentiation.
///
/// Additionally requires the following other traits:
/// * Sized         (Used a component type)
/// * Copy          (This trait does not support reference arithmetic. HRTBs are too clunky in current rust. This bound may be relaxed in the future)
/// * PartialEq     ("Full" equality is not required to permit usage of floating point types)
/// * PartialOrd    (Real number ordering)
/// * Neg           (Multiplication by integer `-1`)
///
/// Addition and multiplication **must** be commutative
///
/// Does not support non-real scalars/complex vector spaces
pub trait Scalar:
    Sized
    + Debug
    + Copy
    + PartialEq
    + PartialOrd
    + Neg<Output=Self>
    + Add<Self, Output=Self>
    + AddAssign<Self>
    + Sub<Self, Output=Self>
    + SubAssign<Self>
    + Mul<Self, Output=Self>
    + MulAssign<Self>
    + Div<Self, Output=Self>
    + DivAssign<Self>
{
    /// Constant value zero
    const ZERO: Self;

    /// Square root, equivalent to [`f64::sqrt`]
    fn sqrt(self) -> Self;

    /// Exponentiation
    fn pow(self, exponent: Self) -> Self;

    /// Exponentiation with integer exponent, equivalent to [`f64::powi`]
    fn powi(self, exponent: i32) -> Self;

    /// Exponentiation with float exponent, equivalent to [`f64::powf`]
    fn powf(self, exponent: f64) -> Self;

    /// True if 'self' is a finite non-NaN value, equivalent to [`f64::is_finite`]
    ///
    /// For types that do not support infinity, NaN, or similar values, always returns true
    fn is_finite(self) -> bool;

    // Real number properties
    /// Constant value PI
    const PI: Self;

    /// Integer literal
    fn i(literal: i32) -> Self;
    /// Floating point literal
    fn f(literal: f64) -> Self;

    /// Sine, equivalent to [`f64::sin`]
    fn sin(self) -> Self;
    /// Cosine, equivalent to [`f64::cos`]
    fn cos(self) -> Self;
    /// Arc-cosine
    /// Returns `None` if input is out of range
    fn acos(self) -> Option<Self>;
    /// Arc-sine
    /// Returns `None` if input is out of range
    fn asin(self) -> Option<Self>;
}

impl Scalar for f32 {
    const ZERO: Self = 0.0;

    #[inline]
    fn sqrt(self) -> Self { self.sqrt() }
    #[inline]
    fn pow(self, exponent: Self) -> Self { f32::powf(self, exponent) }
    #[inline]
    fn powi(self, exponent: i32) -> Self { f32::powi(self, exponent) }
    #[inline]
    fn powf(self, exponent: f64) -> Self { f32::powf(self, exponent as f32) }
    #[inline]
    fn is_finite(self) -> bool { self.is_finite() }

    const PI: Self = std::f32::consts::PI;

    #[inline]
    fn i(literal: i32) -> Self { literal as f32 }
    #[inline]
    fn f(literal: f64) -> Self { literal as f32 }
    #[inline]
    fn sin(self) -> Self { f32::sin(self) }
    #[inline]
    fn cos(self) -> Self { f32::cos(self) }
    #[inline]
    fn acos(self) -> Option<Self> { Some(f32::acos(self)).filter(|f| !f.is_nan()) }
    #[inline]
    fn asin(self) -> Option<Self> {
        Some(f32::asin(self)).filter(|f| !f.is_nan())
    }
}

impl Scalar for f64 {
    const ZERO: Self = 0.0;

    #[inline]
    fn sqrt(self) -> Self { self.sqrt() }
    #[inline]
    fn pow(self, exponent: Self) -> Self { f64::powf(self, exponent) }
    #[inline]
    fn powi(self, exponent: i32) -> Self { f64::powi(self, exponent) }
    #[inline]
    fn powf(self, exponent: f64) -> Self { f64::powf(self, exponent) }
    #[inline]
    fn is_finite(self) -> bool { self.is_finite() }

    const PI: Self = std::f64::consts::PI;

    #[inline]
    fn i(literal: i32) -> Self { literal as f64 }
    #[inline]
    fn f(literal: f64) -> Self { literal }
    #[inline]
    fn sin(self) -> Self { f64::sin(self) }
    #[inline]
    fn cos(self) -> Self { f64::cos(self) }
    #[inline]
    fn acos(self) -> Option<Self> { Some(f64::acos(self)).filter(|f| !f.is_nan()) }
    #[inline]
    fn asin(self) -> Option<Self> {
        Some(f64::asin(self)).filter(|f| !f.is_nan())
    }
}
