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

pub mod two {
    use std::marker::PhantomData;
    use std::mem;
    use std::ops::Neg;
    use crate::basis::Basis;
    use crate::geometry2d::{Vector2D};
    
    pub trait OrthoBasis2: Basis<2> {
        const DIRECTIONS: Directions2D<Self>;

        fn generic_mapper<T: Copy + Neg<Output=T>, O: Basis<2>>(to: Directions2D<O>) -> fn([T; 2]) -> [T; 2] {
            Directions2D::<Self>::generic_mapper(Self::DIRECTIONS, to)
        }

        fn vector_mapper<T: Copy + Neg<Output=T>, O: Basis<2>>(to: Directions2D<O>) -> fn(Vector2D<T, Self>) -> Vector2D<T, O> {
            Directions2D::<Self>::vector_mapper(Self::DIRECTIONS, to)
        }
    }

    /// Cardinal direction enum
    ///
    /// Used in conjunction with [`Directions2D`] to encode coordinate system directions.
    ///
    /// bool indicates whether a direction is in the negative of an axis. (E.g. True for -X, -Y, or -Z. False for +X, +Y or +Z)
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]   // Partialeq is provided as a const impl until const-derive is a thing
    pub enum Direction2 {
        Up { invert: bool },
        Right { invert: bool }
    }

    impl Direction2 {
        #[rustfmt::skip]
        pub const fn opposite(self) -> Self {
            match self {
                Direction2::Up { invert } => Direction2::Up { invert: !invert },
                Direction2::Right { invert } => Direction2::Right { invert: !invert },
            }
        }

        #[rustfmt::skip]
        pub const fn const_eq(&self, other: &Self) -> bool {
            match self {
                Direction2::Up { invert: invert_self } => if let Direction2::Up { invert: invert_other } = other { *invert_self == *invert_other } else { false }
                Direction2::Right { invert: invert_self } => if let Direction2::Right { invert: invert_other } = other { *invert_self == *invert_other } else { false }
            }
        }
    }

    /// Struct to encode the directions of the axes in a 2D coordinate system
    ///
    /// # Example
    ///
    /// For axes [X, Y, Z]
    ///
    /// Y up -Z forward:
    ///
    /// [Direction::Right_East { axis_is_negative: false }, Direction::Up { axis_is_negative: false }, Direction::Forward_North { axis_is_negative: true }]
    ///
    /// Y forward, Z up:
    ///
    /// [Direction::Right_East { axis_is_negative: false }, Direction::Forward_North { axis_is_negative: false }, Direction::Up { axis_is_negative: false }]
    ///
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Directions2D<B: Basis<2>> { // TODO: Rename
        one: Direction2,
        two: Direction2,
        _basis: PhantomData<B>,
    }

    impl<B: Basis<2>> Directions2D<B> {
        /// Create a new coordinate-directions value at const compile time
        ///
        /// Panics if an invalid set of directions is passed
        ///
        /// # Arguments
        ///
        /// * `directions`: Directions in order of their axes'
        ///
        /// returns: CoordinateDirections2D
        #[rustfmt::skip]
        pub const fn new_const(directions: [Direction2; 2]) -> Self {
            if directions[0].const_eq(&directions[1]) || directions[0].const_eq(&directions[1].opposite()) {
                panic!("overlapping directions 0 and 1")
            } else {
                Directions2D {
                    one: directions[0],
                    two: directions[1],
                    _basis: PhantomData,
                }
            }
        }

        /// Create a new coordinate-directions value at runtime
        ///
        /// Yields and error if an invalid set of directions is passed
        ///
        /// # Arguments
        ///
        /// * `directions`: Directions in order of their axes'
        ///
        /// returns: CoordinateDirections2D
        #[rustfmt::skip]
        pub fn new(directions: [Direction2; 2]) -> Result<Self, &'static str> {
            if mem::discriminant(&directions[0]) == mem::discriminant(&directions[1]) {
                Err("overlapping directions 0 and 1")
            } else {
                Ok(Directions2D {
                    one: directions[0],
                    two: directions[1],
                    _basis: PhantomData,
                })
            }
        }

        #[rustfmt::skip]
        const fn index_of(direction: Direction2, coordinates: Directions2D<B>) -> (u8, bool) {
            match direction {
                Direction2::Up { invert: invert_direction } => {
                    if let Direction2::Up { invert: invert_coord } = coordinates.one {
                        (1, invert_direction != invert_coord)
                    } else if let Direction2::Up { invert: invert_coord } = coordinates.two {
                        (2, invert_direction != invert_coord)
                    } else {
                        unreachable!()
                    }
                }
                Direction2::Right { invert: invert_direction } => {
                    if let Direction2::Right { invert: invert_coord } = coordinates.one {
                        (1, invert_direction != invert_coord)
                    } else if let Direction2::Right { invert: invert_coord } = coordinates.two {
                        (2, invert_direction != invert_coord)
                    } else {
                        unreachable!()
                    }
                }
            }
        }


        /// Returns a mapper function to convert generic arrays from one basis to another
        ///
        /// 'const' mapper functions may be inlined better
        ///
        /// # Arguments
        ///
        /// * `from`: Input basis Direction2D
        /// * `to`: Output basis Direction2D
        ///
        /// returns: mapper function pointer
        #[rustfmt::skip]
        pub const fn generic_mapper<T: Copy + Neg<Output=T>, I: Basis<2>, O: Basis<2>>(from: Directions2D<I>, to: Directions2D<O>) -> fn([T; 2]) -> [T; 2] {
            let (index_1, invert_one) = Directions2D::index_of(to.one, from);
            let (index_2, invert_two) = Directions2D::index_of(to.two, from);

            #[allow(unused)]
            let func_ptr: fn([T; 2]) -> [T; 2] = match ((index_1, index_2), (invert_one, invert_two)) {
                ((1, 2), (false, false)) => |[one, two]| [one, two],
                ((1, 2), (true, false)) => |[one, two]| [-one, two],
                ((1, 2), (false, true)) => |[one, two]| [one, -two],
                ((1, 2), (true, true)) => |[one, two]| [-one, -two],
                ((2, 1), (false, false)) => |[one, two]| [two, one],
                ((2, 1), (true, false)) => |[one, two]| [two, -one],
                ((2, 1), (false, true)) => |[one, two]| [-two, one],
                ((2, 1), (true, true)) => |[one, two]| [-two, -one],
                _ => unreachable!(),
            };

            func_ptr
        }

        /// Returns a mapper function to convert vectors from one basis to another
        ///
        /// 'const' mapper functions may be inlined better
        ///
        /// # Arguments
        ///
        /// * `from`: Input basis Direction2D
        /// * `to`: Output basis Direction2D
        ///
        /// returns: mapper function pointer
        #[rustfmt::skip]
        pub const fn vector_mapper<T: Copy + Neg<Output=T>, I: Basis<2>, O: Basis<2>>(from: Directions2D<I>, to: Directions2D<O>) -> fn(Vector2D<T, I>) -> Vector2D<T, O> {
            let (index_1, invert_one) = Directions2D::index_of(to.one, from);
            let (index_2, invert_two) = Directions2D::index_of(to.two, from);

            #[allow(unused)]
            let func_ptr: fn(Vector2D<T, I>) -> Vector2D<T, O> = match ((index_1, index_2), (invert_one, invert_two)) {
                ((1, 2), (false, false)) => |vector| { let [one, two] = vector.to_array(); Vector2D::new([one, two]) },
                ((1, 2), (true, false)) => |vector| { let [one, two] = vector.to_array(); Vector2D::new([-one, two]) },
                ((1, 2), (false, true)) => |vector| { let [one, two] = vector.to_array(); Vector2D::new([one, -two]) },
                ((1, 2), (true, true)) => |vector| { let [one, two] = vector.to_array(); Vector2D::new([-one, -two]) },
                ((2, 1), (false, false)) => |vector| { let [one, two] = vector.to_array(); Vector2D::new([two, one]) },
                ((2, 1), (true, false)) => |vector| { let [one, two] = vector.to_array(); Vector2D::new([two, -one]) },
                ((2, 1), (false, true)) => |vector| { let [one, two] = vector.to_array(); Vector2D::new([-two, one]) },
                ((2, 1), (true, true)) => |vector| { let [one, two] = vector.to_array(); Vector2D::new([-two, -one]) },
                _ => unreachable!(),
            };

            func_ptr
        }
    }

    #[macro_export]
    macro_rules! vector_basis2 {
        ($name:ident, $dir1:expr, $dir2:expr) => {
            #[allow(non_camel_case_types)]
            #[derive(Copy, Clone, PartialEq, Eq, Debug)]
            pub struct $name;
            impl $crate::basis::Basis<2> for $name {}
            impl $crate::basis::two::OrthoBasis2 for $name {
                const DIRECTIONS: $crate::basis::two::Directions2D<Self> = $crate::basis::two::Directions2D::new_const([$dir1, $dir2]);
            }

            impl $name {
                pub const fn directions_const() -> $crate::basis::two::Directions2D<Self> {
                    $crate::basis::two::Directions2D::new_const([$dir1, $dir2])
                }
            }
        };
    }

    vector_basis2!(OriginTopLeft, Direction2::Right { invert: false }, Direction2::Up { invert: true });
    vector_basis2!(OriginTopRight, Direction2::Right { invert: true }, Direction2::Up { invert: true });
    vector_basis2!(OriginBottomLeft, Direction2::Right { invert: false }, Direction2::Up { invert: false });
    vector_basis2!(OriginBottomRight, Direction2::Right { invert: true }, Direction2::Up { invert: false });
}

pub mod three {
    use std::marker::PhantomData;
    use std::mem;
    use std::ops::Neg;
    use crate::basis::Basis;
    use crate::geometry3d::{RotationMatrix, Vector3D};
    
    pub trait OrthoBasis3: Basis<3> {
        const DIRECTIONS: Directions3D<Self>;

        fn generic_mapper<T: Copy + Neg<Output=T>, O: Basis<3>>(to: Directions3D<O>) -> fn([T; 3]) -> [T; 3] {
            Directions3D::<Self>::generic_mapper(Self::DIRECTIONS, to)
        }

        fn vector_mapper<T: Copy + Neg<Output=T>, O: Basis<3>>(to: Directions3D<O>) -> fn(Vector3D<T, Self>) -> Vector3D<T, O> {
            Directions3D::<Self>::vector_mapper(Self::DIRECTIONS, to)
        }

        fn rotationmatrix_mapper<T: Copy + Neg<Output=T>, O: Basis<3>>(to: Directions3D<O>) -> fn(RotationMatrix<T, Self>) -> RotationMatrix<T, O> {
            Directions3D::<Self>::rotationmatrix_mapper(Self::DIRECTIONS, to)
        }
    }

    /// Cardinal direction enum
    ///
    /// Used in conjunction with [`Directions3D`] to encode coordinate system directions.
    ///
    /// bool indicates whether a direction is in the negative of an axis. (E.g. True for -X, -Y, or -Z. False for +X, +Y or +Z)
    #[allow(non_camel_case_types)]
    #[derive(Copy, Clone, PartialEq, Eq, Debug)]   // Partialeq is provided as a const impl until const-derive is a thing
    pub enum Direction3 {
        Up { invert: bool },
        Right { invert: bool },
        Forward { invert: bool },
    }

    impl Direction3 {
        #[rustfmt::skip]
        pub const fn opposite(self) -> Self {
            match self {
                Direction3::Up { invert } => Direction3::Up { invert: !invert },
                Direction3::Right { invert } => Direction3::Right { invert: !invert },
                Direction3::Forward { invert } => Direction3::Forward { invert: !invert }
            }
        }

        #[rustfmt::skip]
        pub const fn const_eq(&self, other: &Self) -> bool {
            match self {
                Direction3::Up { invert: invert_self } => if let Direction3::Up { invert: invert_other } = other { *invert_self == *invert_other } else { false }
                Direction3::Right { invert: invert_self } => if let Direction3::Right { invert: invert_other } = other { *invert_self == *invert_other } else { false }
                Direction3::Forward { invert: invert_self } => if let Direction3::Forward { invert: invert_other } = other { *invert_self == *invert_other } else { false }
            }
        }
    }

    /// Struct to encode the directions of the axes in a 3D coordinate system
    ///
    /// # Example
    ///
    /// For axes [X, Y, Z]
    ///
    /// Y up -Z forward:
    ///
    /// [Direction::Right_East { axis_is_negative: false }, Direction::Up { axis_is_negative: false }, Direction::Forward_North { axis_is_negative: true }]
    ///
    /// Y forward, Z up:
    ///
    /// [Direction::Right_East { axis_is_negative: false }, Direction::Forward_North { axis_is_negative: false }, Direction::Up { axis_is_negative: false }]
    ///
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct Directions3D<B: Basis<3>> { // TODO: Rename
        one: Direction3,
        two: Direction3,
        three: Direction3,
        _basis: PhantomData<B>,
    }

    impl<B: Basis<3>> Directions3D<B> {
        /// Create a new coordinate-directions value at const compile time
        ///
        /// Panics if an invalid set of directions is passed
        ///
        /// # Arguments
        ///
        /// * `directions`: Directions in order of their axes'
        ///
        /// returns: CoordinateDirections3D
        #[rustfmt::skip]
        pub const fn new_const(directions: [Direction3; 3]) -> Self {
            if directions[0].const_eq(&directions[1]) || directions[0].const_eq(&directions[1].opposite()) {
                panic!("overlapping directions 0 and 1")
            } else if directions[1].const_eq(&directions[2]) || directions[1].const_eq(&directions[2].opposite()) {
                panic!("overlapping directions 1 and 2")
            } else if directions[0].const_eq(&directions[2]) || directions[0].const_eq(&directions[2].opposite()) {
                panic!("overlapping directions 0 and 2")
            } else {
                Directions3D {
                    one: directions[0],
                    two: directions[1],
                    three: directions[2],
                    _basis: PhantomData,
                }
            }
        }

        /// Create a new coordinate-directions value at runtime
        ///
        /// Yields and error if an invalid set of directions is passed
        ///
        /// # Arguments
        ///
        /// * `directions`: Directions in order of their axes'
        ///
        /// returns: CoordinateDirections3D
        #[rustfmt::skip]
        pub fn new(directions: [Direction3; 3]) -> Result<Self, &'static str> {
            if mem::discriminant(&directions[0]) == mem::discriminant(&directions[1]) {
                Err("overlapping directions 0 and 1")
            } else if mem::discriminant(&directions[1]) == mem::discriminant(&directions[2]) {
                Err("overlapping directions 1 and 2")
            } else if mem::discriminant(&directions[0]) == mem::discriminant(&directions[2]) {
                Err("overlapping directions 0 and 2")
            } else {
                Ok(Directions3D {
                    one: directions[0],
                    two: directions[1],
                    three: directions[2],
                    _basis: PhantomData,
                })
            }
        }

        #[rustfmt::skip]
        const fn index_of(direction: Direction3, coordinates: Directions3D<B>) -> (u8, bool) {
            match direction {
                Direction3::Up { invert: invert_direction } => {
                    if let Direction3::Up { invert: invert_coord } = coordinates.one {
                        (1, invert_direction != invert_coord)
                    } else if let Direction3::Up { invert: invert_coord } = coordinates.two {
                        (2, invert_direction != invert_coord)
                    } else if let Direction3::Up { invert: invert_coord } = coordinates.three {
                        (3, invert_direction != invert_coord)
                    } else {
                        unreachable!()
                    }
                }
                Direction3::Right { invert: invert_direction } => {
                    if let Direction3::Right { invert: invert_coord } = coordinates.one {
                        (1, invert_direction != invert_coord)
                    } else if let Direction3::Right { invert: invert_coord } = coordinates.two {
                        (2, invert_direction != invert_coord)
                    } else if let Direction3::Right { invert: invert_coord } = coordinates.three {
                        (3, invert_direction != invert_coord)
                    } else {
                        unreachable!()
                    }
                }
                Direction3::Forward { invert: invert_direction } => {
                    if let Direction3::Forward { invert: invert_coord } = coordinates.one {
                        (1, invert_direction != invert_coord)
                    } else if let Direction3::Forward { invert: invert_coord } = coordinates.two {
                        (2, invert_direction != invert_coord)
                    } else if let Direction3::Forward { invert: invert_coord } = coordinates.three {
                        (3, invert_direction != invert_coord)
                    } else {
                        unreachable!()
                    }
                }
            }
        }


        /// Returns a mapper function to convert generic arrays from one basis to another
        ///
        /// 'const' mapper functions may be inlined better
        ///
        /// # Arguments
        ///
        /// * `from`: Input basis Direction3D
        /// * `to`: Output basis Direction3D
        ///
        /// returns: mapper function pointer
        #[rustfmt::skip]
        pub const fn generic_mapper<T: Copy + Neg<Output=T>, I: Basis<3>, O: Basis<3>>(from: Directions3D<I>, to: Directions3D<O>) -> fn([T; 3]) -> [T; 3] {
            let (index_1, invert_one) = Directions3D::index_of(to.one, from);
            let (index_2, invert_two) = Directions3D::index_of(to.two, from);
            let (index_3, invert_three) = Directions3D::index_of(to.three, from);

            #[allow(unused)]
            let func_ptr: fn([T; 3]) -> [T; 3] = match ((index_1, index_2, index_3), (invert_one, invert_two, invert_three)) {
                ((3, 1, 2), (false, false, false)) => |[one, two, three]| { [three, one, two] },
                ((3, 1, 2), (false, false, true)) => |[one, two, three]| { [three, one, -two] },
                ((3, 1, 2), (false, true, false)) => |[one, two, three]| { [three, -one, two] },
                ((3, 1, 2), (false, true, true)) => |[one, two, three]| { [three, -one, -two] },
                ((3, 1, 2), (true, false, false)) => |[one, two, three]| { [-three, one, two] },
                ((3, 1, 2), (true, false, true)) => |[one, two, three]| { [-three, one, -two] },
                ((3, 1, 2), (true, true, false)) => |[one, two, three]| { [-three, -one, two] },
                ((3, 1, 2), (true, true, true)) => |[one, two, three]| { [-three, -one, -two] },
                ((2, 3, 1), (false, false, false)) => |[one, two, three]| { [two, three, one] },
                ((2, 3, 1), (false, false, true)) => |[one, two, three]| { [two, three, -one] },
                ((2, 3, 1), (false, true, false)) => |[one, two, three]| { [two, -three, one] },
                ((2, 3, 1), (false, true, true)) => |[one, two, three]| { [two, -three, -one] },
                ((2, 3, 1), (true, false, false)) => |[one, two, three]| { [-two, three, one] },
                ((2, 3, 1), (true, false, true)) => |[one, two, three]| { [-two, three, -one] },
                ((2, 3, 1), (true, true, false)) => |[one, two, three]| { [-two, -three, one] },
                ((2, 3, 1), (true, true, true)) => |[one, two, three]| { [-two, -three, -one] },
                ((3, 2, 1), (false, false, false)) => |[one, two, three]| { [three, two, one] },
                ((3, 2, 1), (false, false, true)) => |[one, two, three]| { [three, two, -one] },
                ((3, 2, 1), (false, true, false)) => |[one, two, three]| { [three, -two, one] },
                ((3, 2, 1), (false, true, true)) => |[one, two, three]| { [three, -two, -one] },
                ((3, 2, 1), (true, false, false)) => |[one, two, three]| { [-three, two, one] },
                ((3, 2, 1), (true, false, true)) => |[one, two, three]| { [-three, two, -one] },
                ((3, 2, 1), (true, true, false)) => |[one, two, three]| { [-three, -two, one] },
                ((3, 2, 1), (true, true, true)) => |[one, two, three]| { [-three, -two, -one] },
                ((2, 1, 3), (false, false, false)) => |[one, two, three]| { [two, one, three] },
                ((2, 1, 3), (false, false, true)) => |[one, two, three]| { [two, one, -three] },
                ((2, 1, 3), (false, true, false)) => |[one, two, three]| { [two, -one, three] },
                ((2, 1, 3), (false, true, true)) => |[one, two, three]| { [two, -one, -three] },
                ((2, 1, 3), (true, false, false)) => |[one, two, three]| { [-two, one, three] },
                ((2, 1, 3), (true, false, true)) => |[one, two, three]| { [-two, one, -three] },
                ((2, 1, 3), (true, true, false)) => |[one, two, three]| { [-two, -one, three] },
                ((2, 1, 3), (true, true, true)) => |[one, two, three]| { [-two, -one, -three] },
                ((1, 3, 2), (false, false, false)) => |[one, two, three]| { [one, three, two] },
                ((1, 3, 2), (false, false, true)) => |[one, two, three]| { [one, three, -two] },
                ((1, 3, 2), (false, true, false)) => |[one, two, three]| { [one, -three, two] },
                ((1, 3, 2), (false, true, true)) => |[one, two, three]| { [one, -three, -two] },
                ((1, 3, 2), (true, false, false)) => |[one, two, three]| { [-one, three, two] },
                ((1, 3, 2), (true, false, true)) => |[one, two, three]| { [-one, three, -two] },
                ((1, 3, 2), (true, true, false)) => |[one, two, three]| { [-one, -three, two] },
                ((1, 3, 2), (true, true, true)) => |[one, two, three]| { [-one, -three, -two] },
                ((1, 2, 3), (false, false, false)) => |[one, two, three]| { [one, two, three] },
                ((1, 2, 3), (false, false, true)) => |[one, two, three]| { [one, two, -three] },
                ((1, 2, 3), (false, true, false)) => |[one, two, three]| { [one, -two, three] },
                ((1, 2, 3), (false, true, true)) => |[one, two, three]| { [one, -two, -three] },
                ((1, 2, 3), (true, false, false)) => |[one, two, three]| { [-one, two, three] },
                ((1, 2, 3), (true, false, true)) => |[one, two, three]| { [-one, two, -three] },
                ((1, 2, 3), (true, true, false)) => |[one, two, three]| { [-one, -two, three] },
                ((1, 2, 3), (true, true, true)) => |[one, two, three]| { [-one, -two, -three] },
                _ => unreachable!(),
            };

            func_ptr
        }

        /// Returns a mapper function to convert vectors from one basis to another
        ///
        /// 'const' mapper functions may be inlined better
        ///
        /// # Arguments
        ///
        /// * `from`: Input basis Direction3D
        /// * `to`: Output basis Direction3D
        ///
        /// returns: mapper function pointer
        #[rustfmt::skip]
        pub const fn vector_mapper<T: Copy + Neg<Output=T>, I: Basis<3>, O: Basis<3>>(from: Directions3D<I>, to: Directions3D<O>) -> fn(Vector3D<T, I>) -> Vector3D<T, O> {
            let (index_1, invert_one) = Directions3D::index_of(to.one, from);
            let (index_2, invert_two) = Directions3D::index_of(to.two, from);
            let (index_3, invert_three) = Directions3D::index_of(to.three, from);

            #[allow(unused)]
            let func_ptr: fn(Vector3D<T, I>) -> Vector3D<T, O> = match ((index_1, index_2, index_3), (invert_one, invert_two, invert_three)) {
                ((3, 1, 2), (false, false, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([three, one, two]) },
                ((3, 1, 2), (false, false, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([three, one, -two]) },
                ((3, 1, 2), (false, true, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([three, -one, two]) },
                ((3, 1, 2), (false, true, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([three, -one, -two]) },
                ((3, 1, 2), (true, false, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-three, one, two]) },
                ((3, 1, 2), (true, false, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-three, one, -two]) },
                ((3, 1, 2), (true, true, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-three, -one, two]) },
                ((3, 1, 2), (true, true, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-three, -one, -two]) },
                ((2, 3, 1), (false, false, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([two, three, one]) },
                ((2, 3, 1), (false, false, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([two, three, -one]) },
                ((2, 3, 1), (false, true, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([two, -three, one]) },
                ((2, 3, 1), (false, true, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([two, -three, -one]) },
                ((2, 3, 1), (true, false, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-two, three, one]) },
                ((2, 3, 1), (true, false, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-two, three, -one]) },
                ((2, 3, 1), (true, true, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-two, -three, one]) },
                ((2, 3, 1), (true, true, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-two, -three, -one]) },
                ((3, 2, 1), (false, false, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([three, two, one]) },
                ((3, 2, 1), (false, false, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([three, two, -one]) },
                ((3, 2, 1), (false, true, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([three, -two, one]) },
                ((3, 2, 1), (false, true, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([three, -two, -one]) },
                ((3, 2, 1), (true, false, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-three, two, one]) },
                ((3, 2, 1), (true, false, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-three, two, -one]) },
                ((3, 2, 1), (true, true, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-three, -two, one]) },
                ((3, 2, 1), (true, true, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-three, -two, -one]) },
                ((2, 1, 3), (false, false, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([two, one, three]) },
                ((2, 1, 3), (false, false, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([two, one, -three]) },
                ((2, 1, 3), (false, true, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([two, -one, three]) },
                ((2, 1, 3), (false, true, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([two, -one, -three]) },
                ((2, 1, 3), (true, false, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-two, one, three]) },
                ((2, 1, 3), (true, false, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-two, one, -three]) },
                ((2, 1, 3), (true, true, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-two, -one, three]) },
                ((2, 1, 3), (true, true, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-two, -one, -three]) },
                ((1, 3, 2), (false, false, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([one, three, two]) },
                ((1, 3, 2), (false, false, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([one, three, -two]) },
                ((1, 3, 2), (false, true, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([one, -three, two]) },
                ((1, 3, 2), (false, true, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([one, -three, -two]) },
                ((1, 3, 2), (true, false, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-one, three, two]) },
                ((1, 3, 2), (true, false, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-one, three, -two]) },
                ((1, 3, 2), (true, true, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-one, -three, two]) },
                ((1, 3, 2), (true, true, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-one, -three, -two]) },
                ((1, 2, 3), (false, false, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([one, two, three]) },
                ((1, 2, 3), (false, false, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([one, two, -three]) },
                ((1, 2, 3), (false, true, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([one, -two, three]) },
                ((1, 2, 3), (false, true, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([one, -two, -three]) },
                ((1, 2, 3), (true, false, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-one, two, three]) },
                ((1, 2, 3), (true, false, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-one, two, -three]) },
                ((1, 2, 3), (true, true, false)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-one, -two, three]) },
                ((1, 2, 3), (true, true, true)) => |vector| { let [one, two, three] = vector.array; Vector3D::new([-one, -two, -three]) },
                _ => unreachable!(),
            };

            func_ptr
        }

        /// Returns a mapper function to convert rotation matrices from one basis to another
        ///
        /// 'const' mapper functions may be inlined better
        ///
        /// # Arguments
        ///
        /// * `from`: Input basis Direction3D
        /// * `to`: Output basis Direction3D
        ///
        /// returns: mapper function pointer
        #[rustfmt::skip]
        pub const fn rotationmatrix_mapper<T: Copy + Neg<Output=T>, I: Basis<3>, O: Basis<3>>(from: Directions3D<I>, to: Directions3D<O>) -> fn(RotationMatrix<T, I>) -> RotationMatrix<T, O> {
            let (index_1, invert_one) = Directions3D::index_of(to.one, from);
            let (index_2, invert_two) = Directions3D::index_of(to.two, from);
            let (index_3, invert_three) = Directions3D::index_of(to.three, from);

            #[allow(unused)]
            let func_ptr: fn(RotationMatrix<T, I>) -> RotationMatrix<T, O> = match ((index_1, index_2, index_3), (invert_one, invert_two, invert_three)) {
                ((3, 1, 2), (false, false, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([three, one, two]) },
                ((3, 1, 2), (false, false, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([three, one, -two]) },
                ((3, 1, 2), (false, true, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([three, -one, two]) },
                ((3, 1, 2), (false, true, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([three, -one, -two]) },
                ((3, 1, 2), (true, false, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-three, one, two]) },
                ((3, 1, 2), (true, false, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-three, one, -two]) },
                ((3, 1, 2), (true, true, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-three, -one, two]) },
                ((3, 1, 2), (true, true, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-three, -one, -two]) },
                ((2, 3, 1), (false, false, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([two, three, one]) },
                ((2, 3, 1), (false, false, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([two, three, -one]) },
                ((2, 3, 1), (false, true, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([two, -three, one]) },
                ((2, 3, 1), (false, true, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([two, -three, -one]) },
                ((2, 3, 1), (true, false, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-two, three, one]) },
                ((2, 3, 1), (true, false, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-two, three, -one]) },
                ((2, 3, 1), (true, true, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-two, -three, one]) },
                ((2, 3, 1), (true, true, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-two, -three, -one]) },
                ((3, 2, 1), (false, false, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([three, two, one]) },
                ((3, 2, 1), (false, false, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([three, two, -one]) },
                ((3, 2, 1), (false, true, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([three, -two, one]) },
                ((3, 2, 1), (false, true, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([three, -two, -one]) },
                ((3, 2, 1), (true, false, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-three, two, one]) },
                ((3, 2, 1), (true, false, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-three, two, -one]) },
                ((3, 2, 1), (true, true, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-three, -two, one]) },
                ((3, 2, 1), (true, true, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-three, -two, -one]) },
                ((2, 1, 3), (false, false, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([two, one, three]) },
                ((2, 1, 3), (false, false, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([two, one, -three]) },
                ((2, 1, 3), (false, true, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([two, -one, three]) },
                ((2, 1, 3), (false, true, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([two, -one, -three]) },
                ((2, 1, 3), (true, false, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-two, one, three]) },
                ((2, 1, 3), (true, false, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-two, one, -three]) },
                ((2, 1, 3), (true, true, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-two, -one, three]) },
                ((2, 1, 3), (true, true, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-two, -one, -three]) },
                ((1, 3, 2), (false, false, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([one, three, two]) },
                ((1, 3, 2), (false, false, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([one, three, -two]) },
                ((1, 3, 2), (false, true, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([one, -three, two]) },
                ((1, 3, 2), (false, true, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([one, -three, -two]) },
                ((1, 3, 2), (true, false, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-one, three, two]) },
                ((1, 3, 2), (true, false, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-one, three, -two]) },
                ((1, 3, 2), (true, true, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-one, -three, two]) },
                ((1, 3, 2), (true, true, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-one, -three, -two]) },
                ((1, 2, 3), (false, false, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([one, two, three]) },
                ((1, 2, 3), (false, false, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([one, two, -three]) },
                ((1, 2, 3), (false, true, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([one, -two, three]) },
                ((1, 2, 3), (false, true, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([one, -two, -three]) },
                ((1, 2, 3), (true, false, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-one, two, three]) },
                ((1, 2, 3), (true, false, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-one, two, -three]) },
                ((1, 2, 3), (true, true, false)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-one, -two, three]) },
                ((1, 2, 3), (true, true, true)) => |matrix| { let [one, two, three] = matrix.to_inner(); RotationMatrix::from_inner([-one, -two, -three]) },
                _ => unreachable!(),
            };

            func_ptr
        }
    }

    #[macro_export]
    macro_rules! vector_basis3 {
        ($name:ident, $dir1:expr, $dir2:expr, $dir3:expr) => {
            #[allow(non_camel_case_types)]
            #[derive(Copy, Clone, PartialEq, Eq, Debug)]
            pub struct $name;
            impl $crate::basis::Basis<3> for $name {}
            impl $crate::basis::three::OrthoBasis3 for $name {
                const DIRECTIONS: $crate::basis::three::Directions3D<Self> = $crate::basis::three::Directions3D::new_const([$dir1, $dir2, $dir3]);
            }

            impl $name {
                pub const fn directions_const() -> $crate::basis::three::Directions3D<Self> {
                    $crate::basis::three::Directions3D::new_const([$dir1, $dir2, $dir3])
                }
            }
        };
    }

    vector_basis3!(YUpRight, Direction3::Right { invert: false }, Direction3::Up { invert: false }, Direction3::Forward { invert: false });
    vector_basis3!(YUpLeft, Direction3::Right { invert: false }, Direction3::Up { invert: false }, Direction3::Forward { invert: true });

    vector_basis3!(ZUpRight, Direction3::Forward { invert: false }, Direction3::Right { invert: false }, Direction3::Up { invert: false });
    vector_basis3!(ZUpLeft, Direction3::Forward { invert: false }, Direction3::Right { invert: true }, Direction3::Up { invert: false });
}