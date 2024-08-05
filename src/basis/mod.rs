/// Trait for Vector spaces Basis
///
/// Used with the `B` type parameter on [`VectorN`](crate::vector::VectorN) and related types.
/// This type parameter encodes the Basis of the vector, and rejects operations on vectors of differing bases.
///
/// As a default/"generic"/unknown basis, [`()`] may be used
pub trait Basis<const N: usize>: Copy + PartialEq + Eq {
    // A basis conversion function may be added later, but should probably have a default-no implementation?

    // Maybe handedness?
}

/// "Unknown"/default vector basis. In most cases where only 1 geometry context is used, there is no need to explicitly define a basis
impl<const N: usize> Basis<N> for () {}
