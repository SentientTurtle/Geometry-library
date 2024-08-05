use std::fmt::{Display, Formatter};
use std::marker::PhantomData;
use crate::basis::Basis;

use crate::scalar::Scalar;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// N-dimensional vector
///
/// Caution: Vectors are context-sensitive; Vectors from one context may not share the same coordinate system as vectors from another.
/// E.g. Vectors from a Y-up coordinate system, and vectors from a Z-up coordinate system may both be stored as [X, Y, Z]
/// This context may be encoded into types through use of the `Basis` type parameter.
///
/// Type aliases for 2D and 3D ([`Vector3D`] , [`Vector2D`]) are provided by the `geometry2d` and `geometry3d` modules
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct VectorN<T, const N: usize, B: Basis<N>> {
    pub(crate) array: [T; N],
    pub(crate) basis: PhantomData<B>
}

/// Type alias for VectorN that is used as a point
pub type PointN<T, const N: usize, B> = VectorN<T, N, B>;

impl<T, const N: usize, B: Basis<N>> VectorN<T, N, B> {
    /// Create a new vector from an array of components
    ///
    /// Vectors retain the order in which components were passed in.
    ///
    /// # Arguments
    ///
    /// * `array`: Array of components
    ///
    /// returns: VectorN<T, { N }>
    #[inline]
    pub fn new(array: [T; N]) -> Self {
        VectorN {
            array,
            basis: PhantomData,
        }
    }

    /// Convert to a borrowed array
    #[inline]
    pub fn as_array(&self) -> &[T; N] {
        &self.array
    }

    /// "Unwraps" this vector, yielding an array
    ///
    /// Acts as inverse to [`VectorN::new`]
    #[inline]
    pub fn to_array(self) -> [T; N] {
        self.array
    }

    /// Performs a component-wise operation on each component of this vector
    ///
    /// # Arguments
    ///
    /// * `f`: Operation to apply
    ///
    /// returns: VectorN<U, { N }>
    ///
    /// # Examples
    ///
    /// ```
    /// use unifiedgeometry::vector::VectorN;
    ///
    /// let vector = VectorN::new([1, 2, 3]);
    /// let scaled = vector.map(|v| v * 2);
    /// assert_eq!(scaled, [2, 4, 6]);
    /// ```
    #[inline]
    fn map<U, F: FnMut(T) -> U>(self, f: F) -> VectorN<U, N, B> {
        VectorN { array: self.array.map(f), basis: PhantomData }
    }


    /// Performs a component-wise "assign" operation on each component of this vector
    ///
    /// # Arguments
    ///
    /// * `f`: Operation to apply
    ///
    /// # Examples
    ///
    /// ```
    /// use unifiedgeometry::vector::VectorN;
    ///
    /// let mut vector = VectorN::new([1, 2, 3]);
    /// vector.op_assign(|v| *v *= 2);
    /// assert_eq!(vector, [2, 4, 6]);
    /// ```
    #[inline]
    fn op_assign<F: FnMut(&mut T)>(&mut self, f: F) {
        self.array.iter_mut().for_each(f)
    }

    /// Performs a component-wise operation on each component of this and another vector
    ///
    ///
    /// # Arguments
    ///
    /// * `rhs`: Right-hand side vector
    /// * `f`: Operation to apply
    ///
    /// returns: VectorN<V, { N }>
    ///
    /// # Examples
    ///
    /// ```
    /// use unifiedgeometry::vector::VectorN;
    ///
    /// let left_vector = VectorN::new([1, 2, 3]);
    /// let right_vector = VectorN::new([4, 5, 6]);
    /// let sum_vector = left_vector.binary_op(right_vector, |lhs, rhs| lhs + rhs);
    /// assert_eq!(sum_vector, [5, 7, 9]);
    /// ```
    #[inline]
    fn binary_op<U, V>(self, rhs: VectorN<U, N, B>, f: fn(T, U) -> V) -> VectorN<V, N, B> {
        let mut iter = self.array.into_iter()
            .zip(rhs.array.into_iter())
            .map(|(lhs, rhs)| f(lhs, rhs));
        VectorN::new(std::array::from_fn(|_index| iter.next().unwrap()))    // This is a stupid hack, but for small arrays this optimizes well; Using MaybeUninit to consume the arrays is unstable and clunky
    }

    /// Performs a component-wise "assign" operation on each component of this and another vector
    ///
    ///
    /// # Arguments
    ///
    /// * `rhs`: Right-hand side vector
    /// * `f`: Operation to apply
    ///
    /// # Examples
    ///
    /// ```
    /// use unifiedgeometry::vector::VectorN;
    ///
    /// let mut left_vector = VectorN::new([1, 2, 3]);
    /// let right_vector = VectorN::new([4, 5, 6]);
    /// left_vector.binary_assign(right_vector, |lhs, rhs| *lhs += rhs);
    /// assert_eq!(left_vector, [5, 7, 9]);
    /// ```
    #[inline]
    fn binary_assign<U, V>(&mut self, rhs: VectorN<U, N, B>, f: fn(&mut T, U)) {
        self.array.iter_mut()
            .zip(rhs.array.into_iter())
            .for_each(|(lhs, rhs)| f(lhs, rhs))
    }
}

impl<T: PartialEq, const N: usize, B: Basis<N>> PartialEq<[T; N]> for VectorN<T, N, B> {
    fn eq(&self, other: &[T; N]) -> bool {
        self.array.eq(other)
    }
}

impl<T: Display, const N: usize, B: Basis<N>> Display for VectorN<T, N, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(")?;
        for i in 0..self.array.len() {
            write!(f, "{}", &self.array[i])?;
            if i != (self.array.len() - 1) {
                write!(f, ", ")?;
            }
        }
        write!(f, ")")
    }
}

impl<T, const N: usize, B: Basis<N>> IntoIterator for VectorN<T, N, B> {
    type Item = <[T; N] as IntoIterator>::Item;
    type IntoIter = <[T; N] as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.array.into_iter()
    }
}

// TODO: Reference arithmetic

// Vector-Vector Arithmetic
impl<T: Scalar, const N: usize, B: Basis<N>> Neg for VectorN<T, N, B> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        self.map(T::neg)
    }
}

impl<T: Scalar, const N: usize, B: Basis<N>> Add for VectorN<T, N, B> {
    type Output = Self;

    fn add(self, rhs: VectorN<T, N, B>) -> Self::Output {
        self.binary_op(rhs, T::add)
    }
}

impl<T: Scalar, const N: usize, B: Basis<N>> AddAssign for VectorN<T, N, B> {
    fn add_assign(&mut self, rhs: VectorN<T, N, B>) {
        self.binary_assign::<T, T>(rhs, T::add_assign)
    }
}

impl<T: Scalar, const N: usize, B: Basis<N>> Sub for VectorN<T, N, B> {
    type Output = Self;

    fn sub(self, rhs: VectorN<T, N, B>) -> Self::Output {
        self.binary_op(rhs, T::sub)
    }
}

impl<T: Scalar, const N: usize, B: Basis<N>> SubAssign for VectorN<T, N, B> {
    fn sub_assign(&mut self, rhs: VectorN<T, N, B>) {
        self.binary_assign::<T, T>(rhs, T::sub_assign)
    }
}

// Vector-Scalar arithmetic
impl<T: Scalar, const N: usize, B: Basis<N>> Add<T> for VectorN<T, N, B> {
    type Output = Self;

    fn add(self, rhs: T) -> Self::Output {
        self.map(|v| T::add(v, rhs))
    }
}

impl<T: Scalar, const N: usize, B: Basis<N>> AddAssign<T> for VectorN<T, N, B> {
    fn add_assign(&mut self, rhs: T) {
        self.op_assign(|v| T::add_assign(v, rhs))
    }
}

impl<T: Scalar, const N: usize, B: Basis<N>> Sub<T> for VectorN<T, N, B> {
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        self.map(|v| T::sub(v, rhs))
    }
}

impl<T: Scalar, const N: usize, B: Basis<N>> SubAssign<T> for VectorN<T, N, B> {
    fn sub_assign(&mut self, rhs: T) {
        self.op_assign(|v| T::sub_assign(v, rhs))
    }
}

impl<T: Scalar, const N: usize, B: Basis<N>> Mul<T> for VectorN<T, N, B> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        self.map(|v| T::mul(v, rhs))
    }
}

impl<T: Scalar, const N: usize, B: Basis<N>> MulAssign<T> for VectorN<T, N, B> {
    fn mul_assign(&mut self, rhs: T) {
        self.op_assign(|v| T::mul_assign(v, rhs))
    }
}

impl<T: Scalar, const N: usize, B: Basis<N>> Div<T> for VectorN<T, N, B> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        self.map(|v| T::div(v, rhs))
    }
}

impl<T: Scalar, const N: usize, B: Basis<N>> DivAssign<T> for VectorN<T, N, B> {
    fn div_assign(&mut self, rhs: T) {
        self.op_assign(|v| T::div_assign(v, rhs))
    }
}

// Scalar-Vector arithmetic
// This is provided by a macro, as we cannot provide a blanket implementation for T
macro_rules! impl_scalar_vector_arithmetic {
    ($foreign_type:ty) => {
        impl<const N: usize, B: Basis<N>> Add<VectorN<$foreign_type, N, B>> for $foreign_type {
            type Output = VectorN<$foreign_type, N, B>;

            fn add(self, rhs: VectorN<$foreign_type, N, B>) -> Self::Output {
                rhs.map(|v| <$foreign_type>::add(v, self))
            }
        }

        impl<const N: usize, B: Basis<N>> Sub<VectorN<$foreign_type, N, B>> for $foreign_type {
            type Output = VectorN<<$foreign_type as Sub>::Output, N, B>;

            fn sub(self, rhs: VectorN<$foreign_type, N, B> ) -> Self::Output {
                rhs.map(|v| <$foreign_type>::sub(v, self))
            }
        }

        impl<const N: usize, B: Basis<N>> Mul<VectorN<$foreign_type, N, B>> for $foreign_type {
            type Output = VectorN<<$foreign_type as Mul>::Output, N, B>;

            fn mul(self, rhs: VectorN<$foreign_type, N, B>) -> Self::Output {
                rhs.map(|v| <$foreign_type>::mul(v, self))
            }
        }

        impl<const N: usize, B: Basis<N>> Div<VectorN<$foreign_type, N, B>> for $foreign_type {
            type Output = VectorN<<$foreign_type as Div>::Output, N, B>;

            fn div(self, rhs: VectorN<$foreign_type, N, B>) -> Self::Output {
                rhs.map(|v| <$foreign_type>::div(v, self))
            }
        }
    };
}

impl_scalar_vector_arithmetic!(f32);
impl_scalar_vector_arithmetic!(f64);

// Other operations
impl<T: Scalar, const N: usize, B: Basis<N>> VectorN<T, N, B> {
    /// Returns true if all components of this vector are finite, false if any component is an infinity or NaN.
    pub fn is_finite(self) -> bool {
        self.array.into_iter().all(T::is_finite)
    }

    /// Vector length/magnitude
    ///
    /// Special case: T::ZERO for 0-element vectors
    #[inline]
    pub fn magnitude(self) -> T {
        self.into_iter()
            .map(|scalar| scalar.powi(2))
            .fold(T::ZERO, T::add)
            .sqrt()
    }

    /// Calculates the "scalar" dot product between this and another equally-sized vector
    #[inline]
    pub fn dot(self, rhs: Self) -> T {
        self.into_iter()
            .zip(rhs.into_iter())
            .map(|(lhs, rhs)| lhs * rhs)
            .fold(T::ZERO, T::add)
    }

    /// Returns vector with same direction, but unit (1) length
    #[inline]
    pub fn with_unit_length(self) -> Self {
        self / self.magnitude()
    }

    /// Returns vector with same direction, but with specified length
    #[inline]
    pub fn with_length(self, new_length: T) -> Self {
        (self / self.magnitude()) * new_length
    }
}

impl<T: Scalar, const N: usize, B: Basis<N>> PointN<T, N, B> {
    /// Returns relative vector from this point to another point
    ///
    /// Equivalent to `target - self`. This function may be used to provide greater clarity of intent
    #[inline]
    pub fn vector_to(self, target: PointN<T, N, B>) -> VectorN<T, N, B> {
        target - self
    }
}
