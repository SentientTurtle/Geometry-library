//! Specialized items for 3D geometry
use std::ops::{Add, Mul, Sub};
use crate::basis::Basis;
use crate::scalar::Scalar;
use crate::vector::VectorN;

pub type Point3D<T, B> = VectorN<T, 3, B>;
pub type Vector3D<T, B> = VectorN<T, 3, B>;

// Generic bound could be widened to Clone if support for "BigDecimal" types is needed
impl<T: Sub<Output=T> + Mul<Output=T> + Copy, B: Basis<3>> Vector3D<T, B> {
    /// Calculates vector cross product `self × rhs`
    ///
    /// # Arguments
    ///
    /// * `rhs`: Right hand side
    ///
    /// returns: VectorN<T, 3>
    pub fn cross_product(self, rhs: Self) -> Self {
        let [r_1, r_2, r_3] = self.to_array();
        let [l_1, l_2, l_3] = rhs.to_array();
        Vector3D::new([
            (r_2 * l_3) - (r_3 * l_2),
            (r_3 * l_1) - (r_1 * l_3),
            (r_1 * l_2) - (r_2 * l_1)
        ])
    }
}

/// 3D rotation matrix
///
/// Rotations are performed "pre-multiplied" with column vectors when using row-major matrices ([`RotationMatrix::from_row_major`])
/// ```text
/// [[r11, r12, r13],   ⎡x⎤   ⎡(r11 * x) + (r12 * y) + (r13 * z)⎤
///  [r21, r22, r23], . ⎢y⎥ = ⎢(r21 * x) + (r22 * y) + (r23 * z)⎥
///  [r31, r32, r33]]   ⎣z⎦   ⎣(r31 * x) + (r32 * y) + (r33 * z)⎦
/// ```
///
/// Matrix multiplication is performed through the [`Mul`] trait.
///
/// Matrices are represented internally as column-major arrays of Vector3D, such that:
///
/// ```text
/// ⎡⎡r11⎤ ⎡r12⎤ ⎡r13⎤⎤   ⎡x⎤
/// ⎢⎢r21⎥ ⎢r22⎥ ⎢r23⎥⎥ . ⎢y⎥
/// ⎣⎣r31⎦ ⎣r32⎦ ⎣r33⎦⎦   ⎣z⎦
///
///   ⎛    ⎡r11⎤⎞   ⎛    ⎡r12⎤⎞   ⎛    ⎡r13⎤⎞
/// = ⎜x * ⎢r21⎥⎟ + ⎜y * ⎢r22⎥⎟ + ⎜z * ⎢r23⎥⎟
///   ⎝    ⎣r31⎦⎠   ⎝    ⎣r32⎦⎠   ⎝    ⎣r33⎦⎠
///
///   ⎡r11 * x⎤   ⎡r12 * y⎤   ⎡r13 * z⎤
/// = ⎢r21 * x⎥ + ⎢r22 * y⎥ + ⎢r23 * z⎥
///   ⎣r31 * x⎦   ⎣r32 * y⎦   ⎣r33 * z⎦
///
///   ⎡(r11 * x) + (r12 * y) + (r13 * z)⎤
/// = ⎢(r21 * x) + (r22 * y) + (r23 * z)⎥
///   ⎣(r31 * x) + (r32 * y) + (r33 * z)⎦
/// ```
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RotationMatrix<T, B: Basis<3>>([Vector3D<T, B>; 3]);

impl<T, B: Basis<3>> RotationMatrix<T, B> {
    /// Construct a new rotation matrix from a row-major set of 3x3 arrays
    ///
    /// # Arguments
    ///
    /// * `matrix`: Matrix data
    ///
    /// returns: RotMatrix<T>
    ///
    /// # Examples
    /// ```
    /// use unifiedgeometry::geometry3d::RotationMatrix;
    ///
    /// let x;
    /// RotationMatrix::from_row_major([
    ///     [1.0, 0.0, 0.0],
    ///     [0.0, x.cos(), -x.sin()],
    ///     [0.0, x.sin(), x.cos()]
    /// ])
    /// ```
    /// Results in
    /// ```text
    /// ⎡1.0  0.0    0.0   ⎤
    /// ⎢0.0 cos(x) -sin(x)⎥
    /// ⎣0.0 sin(x) cos(x) ⎦
    /// ```
    #[inline]   // Inlining is likely to optimize the transposition away
    pub fn from_row_major(matrix: [[T; 3]; 3]) -> RotationMatrix<T, B> {
        let [
        [r11, r12, r13],
        [r21, r22, r23],
        [r31, r32, r33]
        ] = matrix;

        RotationMatrix([   // Caution: This is intentionally transposed as the vectors are in column-major order
            Vector3D::new([r11, r21, r31]),
            Vector3D::new([r12, r22, r32]),
            Vector3D::new([r13, r23, r33])
        ])
    }

    /// Construct a new rotation matrix from a row-major set of 3x3 arrays
    ///
    /// # Arguments
    ///
    /// * `matrix`: Matrix data
    ///
    /// returns: RotMatrix<T>
    ///
    /// # Examples
    /// Given `matrix`
    /// ```text
    /// ⎡1.0  0.0    0.0   ⎤
    /// ⎢0.0 cos(x) -sin(x)⎥
    /// ⎣0.0 sin(x) cos(x) ⎦
    /// ```
    ///
    /// ```
    /// let (matrix, x);
    /// let [
    ///     [r11, r12, r13],
    ///     [r21, r22, r23],
    ///     [r31, r32, r33]
    /// ] = matrix.to_row_major();
    ///
    /// assert_eq!(r11, 1.0);
    /// assert_eq!(r12, 0.0);
    /// assert_eq!(r13, 0.0);
    ///
    /// assert_eq!(r21, 0.0);
    /// assert_eq!(r22, x.cos());
    /// assert_eq!(r23, -x.sin());
    ///
    /// assert_eq!(r31, 0.0);
    /// assert_eq!(r32, x.sin());
    /// assert_eq!(r33, x.cos());
    /// ```
    #[inline]   // Inlining is likely to optimize the transposition away
    pub fn to_row_major(self) -> [[T; 3]; 3] {
        let [   // Caution: This destructuring is intentionally transposed as the vectors are in column-major order
        Vector3D { array: [r11, r21, r31], .. },
        Vector3D { array: [r12, r22, r32], .. },
        Vector3D { array: [r13, r23, r33], .. }
        ] = self.0;

        [
            [r11, r12, r13],
            [r21, r22, r23],
            [r31, r32, r33]
        ]
    }
}

impl<T: Scalar, B: Basis<3>> RotationMatrix<T, B> {
    /// Apply this rotation to the specified vector
    ///
    /// Rotations are performed "pre-multiplied" with column vectors, when using row-major matrices ([`RotationMatrix::from_row_major`])
    /// ```text
    /// [[r11, r12, r13],   ⎡x⎤   ⎡(r11 * x) + (r12 * y) + (r13 * z)⎤
    ///  [r21, r22, r23], . ⎢y⎥ = ⎢(r21 * x) + (r22 * y) + (r23 * z)⎥
    ///  [r31, r32, r33]]   ⎣z⎦   ⎣(r31 * x) + (r32 * y) + (r33 * z)⎦
    /// ```
    /// # Arguments
    ///
    /// * `vector`: Vector to rotate
    ///
    /// returns: VectorN<T, 3>
    pub fn apply(self, vector: Vector3D<T, B>) -> Vector3D<T, B> {
        let [matrix_x, matrix_y, matrix_z] = self.0;
        let [x, y, z] = vector.to_array();

        (matrix_x * x) + (matrix_y * y) + (matrix_z * z)
    }
}

impl<T: Add<Output=T> + Mul<Output=T> + Copy, B: Basis<3>> Mul for RotationMatrix<T, B> {
    type Output = RotationMatrix<T, B>;

    #[inline]   // Multiplication with literals can often be (partially) optimized
    fn mul(self, rhs: Self) -> Self::Output {
        let [
        [a11, a12, a13],
        [a21, a22, a23],
        [a31, a32, a33]
        ] = self.to_row_major();
        let [
        [b11, b12, b13],
        [b21, b22, b23],
        [b31, b32, b33]
        ] = rhs.to_row_major();

        RotationMatrix([
            Vector3D::new([a11 * b11 + a12 * b21 + a13 * b31, a11 * b12 + a12 * b22 + a13 * b32, a11 * b13 + a12 * b23 + a13 * b33]),
            Vector3D::new([a21 * b11 + a22 * b21 + a23 * b31, a21 * b12 + a22 * b22 + a23 * b32, a21 * b13 + a22 * b23 + a23 * b33]),
            Vector3D::new([a31 * b11 + a32 * b21 + a23 * b31, a31 * b12 + a32 * b22 + a33 * b32, a31 * b13 + a32 * b23 + a33 * b33])
        ])
    }
}

pub mod shapes;

