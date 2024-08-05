#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use std::fmt::Debug;
use crate::basis::Basis;
use crate::scalar::{Scalar};
use crate::shapes::triangle::formulas::triangle_area;
use crate::utility::MaybeTwo;
use crate::vector::{PointN};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum InvalidTriangleError {
    InvalidLength,
    InvalidAngle,
    AngleTooLarge,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Triangle<T: Scalar, const N: usize, B: Basis<N>> {
    A: PointN<T, N, B>,
    B: PointN<T, N, B>,
    C: PointN<T, N, B>,
}

impl<T: Scalar, const N: usize, B: Basis<N>> Triangle<T, N, B> {
    pub fn new(A: PointN<T, N, B>, B: PointN<T, N, B>, C: PointN<T, N, B>) -> Result<Self, InvalidTriangleError> {
        if A == B || B == C || C == A || !A.is_finite() || !B.is_finite() || !C.is_finite() {
            Err(InvalidTriangleError::InvalidLength)
        } else {
            Ok(Self { A, B, C })
        }
    }

    pub fn A(self) -> PointN<T, N, B> {
        self.A
    }

    pub fn B(self) -> PointN<T, N, B> {
        self.B
    }

    pub fn C(self) -> PointN<T, N, B> {
        self.C
    }
}

impl<T: Scalar, const N: usize, B: Basis<N>> AbstractTriangle<T> for Triangle<T, N, B> {
    type len_a_solutions = T;
    #[inline]
    fn length_a(self) -> T { (self.B - self.C).magnitude() }

    type len_b_solutions = T;
    #[inline]
    fn length_b(self) -> T { (self.A - self.C).magnitude() }

    type len_c_solutions = T;
    #[inline]
    fn length_c(self) -> T { (self.A - self.B).magnitude() }

    type angle_alpha_solutions = T;
    #[inline]
    fn angle_alpha(self) -> T {
        law_of_cosines::alpha_from_abc(self.length_a(), self.length_b(), self.length_c())
            .expect("triangle constructed from points should always be valid!")
    }

    type angle_beta_solutions = T;
    #[inline]
    fn angle_beta(self) -> T {
        law_of_cosines::beta_from_abc(self.length_a(), self.length_b(), self.length_c())
            .expect("triangle constructed from points should always be valid!")
    }

    type angle_gamma_solutions = T;
    #[inline]
    fn angle_gamma(self) -> T {
        law_of_cosines::gamma_from_abc(self.length_a(), self.length_b(), self.length_c())
            .expect("triangle constructed from points should always be valid!")
    }

    type area_solutions = T;
    #[inline]
    fn area(self) -> Self::area_solutions {
        triangle_area(self.length_a(), self.length_b(), self.length_c())
            .expect("triangle constructed from points should always be valid!")
    }

    type altitude_a_solutions = T;
    #[inline]
    fn altitude_a(self) -> Self::altitude_a_solutions {
        T::i(2) * self.area() / self.length_a()
    }

    type altitude_b_solutions = T;
    #[inline]
    fn altitude_b(self) -> Self::altitude_b_solutions {
        T::i(2) * self.area() / self.length_b()
    }

    type altitude_c_solutions = T;
    #[inline]
    fn altitude_c(self) -> Self::altitude_c_solutions {
        T::i(2) * self.area() / self.length_c()
    }
}

/// Macro for instantiating abstract triangles
///
/// Parameters in order of lengths then angles, each in alphabetical order
#[macro_export]
macro_rules! abstract_triangle {
    {a: $a:expr, b: $b:expr, c: $c:expr} => { $crate::shapes::triangle::AbstractTriangle_abc::new($a, $b, $c) };

    {a: $a:expr, b: $b:expr, alpha: $alpha:expr} => { $crate::shapes::triangle::AbstractTriangle_abα::new($a, $b, $alpha) };
    {a: $a:expr, c: $c:expr, alpha: $alpha:expr} => { $crate::shapes::triangle::AbstractTriangle_acα::new($a, $c, $alpha ) };
    {b: $b:expr, c: $c:expr, alpha: $alpha:expr} => { $crate::shapes::triangle::AbstractTriangle_bcα::new($b, $c, $alpha ) };
    {a: $a:expr, b: $b:expr, beta: $beta:expr} => { $crate::shapes::triangle::AbstractTriangle_abβ::new($a, $b, $beta ) };
    {a: $a:expr, c: $c:expr, beta: $beta:expr} => { $crate::shapes::triangle::AbstractTriangle_acβ::new($a, $c, $beta ) };
    {b: $b:expr, c: $c:expr, beta: $beta:expr} => { $crate::shapes::triangle::AbstractTriangle_bcβ::new($b, $c, $beta ) };
    {a: $a:expr, b: $b:expr, gamma: $gamma:expr} => { $crate::shapes::triangle::AbstractTriangle_abγ::new($a, $b, $gamma ) };
    {a: $a:expr, c: $c:expr, gamma: $gamma:expr} => { $crate::shapes::triangle::AbstractTriangle_acγ::new($a, $c, $gamma ) };
    {b: $b:expr, c: $c:expr, gamma: $gamma:expr} => { $crate::shapes::triangle::AbstractTriangle_bcγ::new($b, $c, $gamma ) };

    {a: $a:expr, alpha: $alpha:expr, beta: $beta:expr} => { $crate::shapes::triangle::AbstractTriangle_aαβ::new($a, $alpha, $beta) };
    {b: $b:expr, alpha: $alpha:expr, beta: $beta:expr} => { $crate::shapes::triangle::AbstractTriangle_bαβ::new($b, $alpha, $beta) };
    {c: $c:expr, alpha: $alpha:expr, beta: $beta:expr} => { $crate::shapes::triangle::AbstractTriangle_cαβ::new($c, $alpha, $beta) };
    {a: $a:expr, alpha: $alpha:expr, gamma: $gamma:expr} => { $crate::shapes::triangle::AbstractTriangle_aαγ::new($a, $alpha, $gamma) };
    {b: $b:expr, alpha: $alpha:expr, gamma: $gamma:expr} => { $crate::shapes::triangle::AbstractTriangle_bαγ::new($b, $alpha, $gamma) };
    {c: $c:expr, alpha: $alpha:expr, gamma: $gamma:expr} => { $crate::shapes::triangle::AbstractTriangle_cαγ::new($c, $alpha, $gamma) };
    {a: $a:expr, beta: $beta:expr, gamma: $gamma:expr} => { $crate::shapes::triangle::AbstractTriangle_aβγ::new($a, $beta, $gamma) };
    {b: $b:expr, beta: $beta:expr, gamma: $gamma:expr} => { $crate::shapes::triangle::AbstractTriangle_bβγ::new($b, $beta, $gamma) };
    {c: $c:expr, beta: $beta:expr, gamma: $gamma:expr} => { $crate::shapes::triangle::AbstractTriangle_cβγ::new($c, $beta, $gamma) };

    // A triangle with only angles known cannot be solved
}

#[inline]
fn chain_solution<T, F: Fn(T) -> T>(solution: (T, Option<T>), f: F) -> (T, Option<T>) {
    (f(solution.0), solution.1.map(f))
}

/// Trait for "abstract" triangles; Defined by their measurements instead of 3 known points.
/// 3 lengths, 2 lengths and 1 angle, or 1 length and 2 angles must be known to fully define a triangle.
///
/// This trait provides methods to calculate the other properties of a triangle. In certain configurations, an abstract triangle has ambiguity about certain properties.
/// Where this is the case, the respective methods return `(T, Option<T>)`.
///
/// Instances can be created through the [`abstract_triangle!`] macro, which will provide a specific type for each kind
pub trait AbstractTriangle<T: Scalar>: Copy {
    type len_a_solutions: MaybeTwo<T>;
    fn length_a(self) -> Self::len_a_solutions;

    type len_b_solutions: MaybeTwo<T>;
    fn length_b(self) -> Self::len_b_solutions;

    type len_c_solutions: MaybeTwo<T>;
    fn length_c(self) -> Self::len_c_solutions;

    type angle_alpha_solutions: MaybeTwo<T>;
    fn angle_alpha(self) -> Self::angle_alpha_solutions;

    type angle_beta_solutions: MaybeTwo<T>;
    fn angle_beta(self) -> Self::angle_beta_solutions;

    type angle_gamma_solutions: MaybeTwo<T>;
    fn angle_gamma(self) -> Self::angle_gamma_solutions;

    type area_solutions: MaybeTwo<T>;   // At most 1 length is ambiguous, so there can only be at most 2 options for an ambiguous area
    fn area(self) -> Self::area_solutions;

    type altitude_a_solutions: MaybeTwo<T>;
    fn altitude_a(self) -> Self::altitude_a_solutions;

    type altitude_b_solutions: MaybeTwo<T>;
    fn altitude_b(self) -> Self::altitude_b_solutions;

    type altitude_c_solutions: MaybeTwo<T>;
    fn altitude_c(self) -> Self::altitude_c_solutions;
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AbstractTriangle_abc<T: Scalar> { a: T, b: T, c: T }

impl<T: Scalar> AbstractTriangle_abc<T> {
    #[inline]
    pub fn new(a: T, b: T, c: T) -> Result<Self, InvalidTriangleError> {
        if a + b < c || a + c < b || b + c < a || a <= T::ZERO || b <= T::ZERO || c <= T::ZERO || !a.is_finite() || !b.is_finite() || !c.is_finite() {
            Err(InvalidTriangleError::InvalidLength)
        } else {
            Ok(Self { a, b, c })
        }
    }
}

impl<T: Scalar> AbstractTriangle<T> for AbstractTriangle_abc<T> {
    type len_a_solutions = T;
    #[inline]
    fn length_a(self) -> Self::len_a_solutions { self.a }

    type len_b_solutions = T;
    #[inline]
    fn length_b(self) -> Self::len_b_solutions { self.b }

    type len_c_solutions = T;
    #[inline]
    fn length_c(self) -> Self::len_c_solutions { self.c }

    type angle_alpha_solutions = T;
    #[inline]
    fn angle_alpha(self) -> Self::angle_alpha_solutions {
        law_of_cosines::alpha_from_abc(self.a, self.b, self.c).expect("triangle must be valid")
    }

    type angle_beta_solutions = T;
    #[inline]
    fn angle_beta(self) -> Self::angle_beta_solutions {
        law_of_cosines::beta_from_abc(self.a, self.b, self.c).expect("triangle must be valid")
    }

    type angle_gamma_solutions = T;
    #[inline]
    fn angle_gamma(self) -> Self::angle_gamma_solutions {
        law_of_cosines::gamma_from_abc(self.a, self.b, self.c).expect("triangle must be valid")
    }

    type area_solutions = T;
    #[inline]
    fn area(self) -> Self::area_solutions {
        triangle_area(self.length_a(), self.length_b(), self.length_c())
            .expect("triangle constructed from points should always be valid!")
    }

    type altitude_a_solutions = T;
    #[inline]
    fn altitude_a(self) -> Self::altitude_a_solutions {
        T::i(2) * self.area() / self.length_a()
    }

    type altitude_b_solutions = T;
    #[inline]
    fn altitude_b(self) -> Self::altitude_b_solutions {
        T::i(2) * self.area() / self.length_b()
    }

    type altitude_c_solutions = T;
    #[inline]
    fn altitude_c(self) -> Self::altitude_c_solutions {
        T::i(2) * self.area() / self.length_c()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AbstractTriangle_abα<T> { a: T, b: T, alpha: T }

impl<T: Scalar> AbstractTriangle_abα<T> {
    #[inline]
    pub fn new(a: T, b: T, alpha: T) -> Result<Self, InvalidTriangleError> {
        if a <= T::ZERO || b <= T::ZERO || !a.is_finite() || !b.is_finite() {
            Err(InvalidTriangleError::InvalidLength)
        } else if alpha <= T::ZERO || alpha >= T::PI || !alpha.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else {
            if let Some(tangent_angle) = (a/b).asin() { // If A > B, all angles are valid so we don't need to test
                if alpha > tangent_angle {
                    return Err(InvalidTriangleError::AngleTooLarge)
                }
            }
            Ok(Self { a, b, alpha })
        }
    }
}

impl<T: Scalar> AbstractTriangle<T> for AbstractTriangle_abα<T> {
    type len_a_solutions = T;
    #[inline]
    fn length_a(self) -> Self::len_a_solutions { self.a }

    type len_b_solutions = T;
    #[inline]
    fn length_b(self) -> Self::len_b_solutions { self.b }

    type len_c_solutions = (T, Option<T>);
    #[inline]
    fn length_c(self) -> Self::len_c_solutions {
        law_of_cosines::c_from_abα(self.a, self.b, self.alpha).expect("triangle must be valid")
    }

    type angle_alpha_solutions = T;
    #[inline]
    fn angle_alpha(self) -> Self::angle_alpha_solutions { self.alpha }

    type angle_beta_solutions = (T, Option<T>);
    #[inline]
    fn angle_beta(self) -> Self::angle_beta_solutions {
        chain_solution(
            self.length_c(),
            |c| law_of_cosines::beta_from_abc(self.a, self.b, c).expect("triangle must be valid")
        )
    }

    type angle_gamma_solutions = (T, Option<T>);
    #[inline]
    fn angle_gamma(self) -> Self::angle_gamma_solutions {
        chain_solution(
            self.length_c(),
            |c| law_of_cosines::gamma_from_abc(self.a, self.b, c).expect("triangle must be valid")
        )
    }

    type area_solutions = (T, Option<T>);
    #[inline]
    fn area(self) -> Self::area_solutions {
        chain_solution(
            self.length_c(),
            |c| triangle_area(self.length_a(), self.length_b(), c)
                .expect("triangle constructed from points should always be valid!")
        )
    }

    type altitude_a_solutions = (T, Option<T>);
    #[inline]
    fn altitude_a(self) -> Self::altitude_a_solutions {
        chain_solution(
            self.length_c(),
            |c| {
                let a = self.length_a();
                let b = self.length_b();
                let area = triangle_area(a, b, c)
                    .expect("triangle constructed from points should always be valid!");
                T::i(2) * area / a
            }
        )
    }

    type altitude_b_solutions = (T, Option<T>);
    #[inline]
    fn altitude_b(self) -> Self::altitude_b_solutions {
        chain_solution(
            self.length_c(),
            |c| {
                let a = self.length_a();
                let b = self.length_b();
                let area = triangle_area(a, b, c)
                    .expect("triangle constructed from points should always be valid!");
                T::i(2) * area / b
            }
        )
    }

    type altitude_c_solutions = T;
    #[inline]
    fn altitude_c(self) -> Self::altitude_c_solutions {
        let a = self.length_a();
        let b = self.length_b();
        let c = self.length_c().first();
        let area = triangle_area(a, b, c)
            .expect("triangle constructed from points should always be valid!");
        T::i(2) * area / c
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AbstractTriangle_acα<T> { a: T, c: T, alpha: T }

impl<T: Scalar> AbstractTriangle_acα<T> {
    pub fn new(a: T, c: T, alpha: T) -> Result<Self, InvalidTriangleError> {
        if a <= T::ZERO || c <= T::ZERO || !a.is_finite() || !c.is_finite() {
            Err(InvalidTriangleError::InvalidLength)
        } else if alpha <= T::ZERO || alpha >= T::PI || !alpha.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else {
            if let Some(tangent_angle) = (a/c).asin() { // If A > C, all angles are valid so we don't need to test
                if alpha > tangent_angle {
                    return Err(InvalidTriangleError::AngleTooLarge)
                }
            }
            Ok(Self { a, c, alpha })
        }
    }
}

impl<T: Scalar> AbstractTriangle<T> for AbstractTriangle_acα<T> {
    type len_a_solutions = T;
    #[inline]
    fn length_a(self) -> Self::len_a_solutions { self.a }

    type len_b_solutions = (T, Option<T>);
    #[inline]
    fn length_b(self) -> Self::len_b_solutions {
        law_of_cosines::b_from_acα(self.a, self.c, self.alpha).expect("triangle must be valid")
    }

    type len_c_solutions = T;
    #[inline]
    fn length_c(self) -> Self::len_c_solutions { self.c }

    type angle_alpha_solutions = T;
    #[inline]
    fn angle_alpha(self) -> Self::angle_alpha_solutions { self.alpha }

    type angle_beta_solutions = (T, Option<T>);
    #[inline]
    fn angle_beta(self) -> Self::angle_beta_solutions {
        chain_solution(
            self.length_b(),
            |b| law_of_cosines::beta_from_abc(self.a, b, self.c).expect("triangle must be valid")
        )
    }

    type angle_gamma_solutions = (T, Option<T>);
    #[inline]
    fn angle_gamma(self) -> Self::angle_gamma_solutions {
        chain_solution(
            self.length_b(),
            |b| law_of_cosines::gamma_from_abc(self.a, b, self.c).expect("triangle must be valid")
        )
    }

    type area_solutions = (T, Option<T>);
    #[inline]
    fn area(self) -> Self::area_solutions {
        chain_solution(
            self.length_b(),
            |b| triangle_area(self.length_a(), b, self.length_c())
                .expect("triangle constructed from points should always be valid!")
        )
    }

    type altitude_a_solutions = (T, Option<T>);
    #[inline]
    fn altitude_a(self) -> Self::altitude_a_solutions {
        chain_solution(
            self.length_b(),
            |b| {
                let a = self.length_a();
                let c = self.length_c();
                let area = triangle_area(a, b, c)
                    .expect("triangle constructed from points should always be valid!");
                T::i(2) * area / a
            }
        )
    }

    type altitude_b_solutions = T;
    #[inline]
    fn altitude_b(self) -> Self::altitude_b_solutions {
        let a = self.length_a();
        let b = self.length_b().first();
        let c = self.length_c();
        let area = triangle_area(a, b, c)
            .expect("triangle constructed from points should always be valid!");
        T::i(2) * area / b
    }

    type altitude_c_solutions = (T, Option<T>);
    #[inline]
    fn altitude_c(self) -> Self::altitude_c_solutions {
        chain_solution(
            self.length_b(),
            |b| {
                let a = self.length_a();
                let c = self.length_c();
                let area = triangle_area(a, b, c)
                    .expect("triangle constructed from points should always be valid!");
                T::i(2) * area / c
            }
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AbstractTriangle_bcα<T> { b: T, c: T, alpha: T }

impl<T: Scalar> AbstractTriangle_bcα<T> {
    pub fn new(b: T, c: T, alpha: T) -> Result<Self, InvalidTriangleError> {
        if b <= T::ZERO || c <= T::ZERO || !b.is_finite() || !c.is_finite() {
            Err(InvalidTriangleError::InvalidLength)
        } else if alpha <= T::ZERO || alpha >= T::PI || !alpha.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else {
            Ok(Self { b, c, alpha })
        }
    }
}

impl<T: Scalar> AbstractTriangle<T> for AbstractTriangle_bcα<T> {
    type len_a_solutions = T;
    #[inline]
    fn length_a(self) -> Self::len_a_solutions { law_of_cosines::a_from_bcα(self.b, self.c, self.alpha).expect("triangle must be valid") }

    type len_b_solutions = T;
    #[inline]
    fn length_b(self) -> Self::len_b_solutions { self.b }

    type len_c_solutions = T;
    #[inline]
    fn length_c(self) -> Self::len_c_solutions { self.c }

    type angle_alpha_solutions = T;
    #[inline]
    fn angle_alpha(self) -> Self::angle_alpha_solutions { self.alpha }

    type angle_beta_solutions = T;
    #[inline]
    fn angle_beta(self) -> Self::angle_beta_solutions {
        law_of_cosines::beta_from_abc(self.length_a(), self.b, self.c).expect("triangle must be valid")
    }

    type angle_gamma_solutions = T;
    #[inline]
    fn angle_gamma(self) -> Self::angle_gamma_solutions {
        law_of_cosines::gamma_from_abc(self.length_a(), self.b, self.c).expect("triangle must be valid")
    }

    type area_solutions = T;
    #[inline]
    fn area(self) -> Self::area_solutions {
        triangle_area(self.length_a(), self.length_b(), self.length_c())
            .expect("triangle constructed from points should always be valid!")
    }

    type altitude_a_solutions = T;
    #[inline]
    fn altitude_a(self) -> Self::altitude_a_solutions {
        T::i(2) * self.area() / self.length_a()
    }

    type altitude_b_solutions = T;
    #[inline]
    fn altitude_b(self) -> Self::altitude_b_solutions {
        T::i(2) * self.area() / self.length_b()
    }

    type altitude_c_solutions = T;
    #[inline]
    fn altitude_c(self) -> Self::altitude_c_solutions {
        T::i(2) * self.area() / self.length_c()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AbstractTriangle_abβ<T> { a: T, b: T, beta: T }

impl<T: Scalar> AbstractTriangle_abβ<T> {
    pub fn new(a: T, b: T, beta: T) -> Result<Self, InvalidTriangleError> {
        if a <= T::ZERO || b <= T::ZERO || !a.is_finite() || !b.is_finite() {
            Err(InvalidTriangleError::InvalidLength)
        } else if beta <= T::ZERO || beta >= T::PI || !beta.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else {
            if let Some(tangent_angle) = (b/a).asin() { // If B > A, all angles are valid so we don't need to test
                if beta > tangent_angle {
                    return Err(InvalidTriangleError::AngleTooLarge)
                }
            }
            Ok(Self { a, b, beta })
        }
    }
}

impl<T: Scalar> AbstractTriangle<T> for AbstractTriangle_abβ<T> {
    type len_a_solutions = T;
    #[inline]
    fn length_a(self) -> Self::len_a_solutions { self.a }

    type len_b_solutions = T;
    #[inline]
    fn length_b(self) -> Self::len_b_solutions { self.b }

    type len_c_solutions = (T, Option<T>);
    #[inline]
    fn length_c(self) -> Self::len_c_solutions {
        law_of_cosines::c_from_abβ(self.a, self.b, self.beta).expect("triangle must be valid")
    }

    type angle_alpha_solutions = (T, Option<T>);
    #[inline]
    fn angle_alpha(self) -> Self::angle_alpha_solutions {
        chain_solution(
            self.length_c(),
            |c| law_of_cosines::alpha_from_abc(self.a, self.b, c).expect("triangle must be valid")
        )
    }

    type angle_beta_solutions = T;
    #[inline]
    fn angle_beta(self) -> Self::angle_beta_solutions { self.beta }

    type angle_gamma_solutions = (T, Option<T>);
    #[inline]
    fn angle_gamma(self) -> Self::angle_gamma_solutions {
        chain_solution(
            self.length_c(),
            |c| law_of_cosines::gamma_from_abc(self.a, self.b, c).expect("triangle must be valid")
        )
    }

    type area_solutions = (T, Option<T>);
    #[inline]
    fn area(self) -> Self::area_solutions {
        chain_solution(
            self.length_c(),
            |c| triangle_area(self.length_a(), self.length_b(), c)
                .expect("triangle constructed from points should always be valid!")
        )
    }

    type altitude_a_solutions = (T, Option<T>);
    #[inline]
    fn altitude_a(self) -> Self::altitude_a_solutions {
        chain_solution(
            self.length_c(),
            |c| {
                let a = self.length_a();
                let b = self.length_b();
                let area = triangle_area(a, b, c)
                    .expect("triangle constructed from points should always be valid!");
                T::i(2) * area / a
            }
        )
    }

    type altitude_b_solutions = (T, Option<T>);
    #[inline]
    fn altitude_b(self) -> Self::altitude_b_solutions {
        chain_solution(
            self.length_c(),
            |c| {
                let a = self.length_a();
                let b = self.length_b();
                let area = triangle_area(a, b, c)
                    .expect("triangle constructed from points should always be valid!");
                T::i(2) * area / b
            }
        )
    }

    type altitude_c_solutions = T;
    #[inline]
    fn altitude_c(self) -> Self::altitude_c_solutions {
        let a = self.length_a();
        let b = self.length_b();
        let c = self.length_c().first();
        let area = triangle_area(a, b, c)
            .expect("triangle constructed from points should always be valid!");
        T::i(2) * area / c
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AbstractTriangle_acβ<T> { a: T, c: T, beta: T }

impl<T: Scalar> AbstractTriangle_acβ<T> {
    pub fn new(a: T, c: T, beta: T) -> Result<Self, InvalidTriangleError> {
        if a <= T::ZERO || c <= T::ZERO || !a.is_finite() || !c.is_finite(){
            Err(InvalidTriangleError::InvalidLength)
        } else if beta <= T::ZERO || beta >= T::PI || !beta.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else {
            Ok(Self { a, c, beta })
        }
    }
}

impl<T: Scalar> AbstractTriangle<T> for AbstractTriangle_acβ<T> {
    type len_a_solutions = T;
    #[inline]
    fn length_a(self) -> Self::len_a_solutions { self.a }

    type len_b_solutions = T;
    #[inline]
    fn length_b(self) -> Self::len_b_solutions {
        law_of_cosines::b_from_acβ(self.a, self.c, self.beta).expect("triangle must be valid")
    }

    type len_c_solutions = T;
    #[inline]
    fn length_c(self) -> Self::len_c_solutions { self.c }

    type angle_alpha_solutions = T;
    #[inline]
    fn angle_alpha(self) -> Self::angle_alpha_solutions {
        law_of_cosines::alpha_from_abc(self.a, self.length_b(), self.c).expect("triangle must be valid")
    }

    type angle_beta_solutions = T;
    #[inline]
    fn angle_beta(self) -> Self::angle_beta_solutions { self.beta }

    type angle_gamma_solutions = T;
    #[inline]
    fn angle_gamma(self) -> Self::angle_gamma_solutions {
        law_of_cosines::gamma_from_abc(self.a, self.length_b(), self.c).expect("triangle must be valid")
    }

    type area_solutions = T;
    #[inline]
    fn area(self) -> Self::area_solutions {
        triangle_area(self.length_a(), self.length_b(), self.length_c())
            .expect("triangle constructed from points should always be valid!")
    }

    type altitude_a_solutions = T;
    #[inline]
    fn altitude_a(self) -> Self::altitude_a_solutions {
        T::i(2) * self.area() / self.length_a()
    }

    type altitude_b_solutions = T;
    #[inline]
    fn altitude_b(self) -> Self::altitude_b_solutions {
        T::i(2) * self.area() / self.length_b()
    }

    type altitude_c_solutions = T;
    #[inline]
    fn altitude_c(self) -> Self::altitude_c_solutions {
        T::i(2) * self.area() / self.length_c()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AbstractTriangle_bcβ<T> { b: T, c: T, beta: T }

impl<T: Scalar> AbstractTriangle_bcβ<T> {
    pub fn new(b: T, c: T, beta: T) -> Result<Self, InvalidTriangleError> {
        if b <= T::ZERO || c <= T::ZERO || !b.is_finite() || !c.is_finite() {
            Err(InvalidTriangleError::InvalidLength)
        } else if beta <= T::ZERO || beta >= T::PI || !beta.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else {
            if let Some(tangent_angle) = (b/c).asin() { // If B > C, all angles are valid so we don't need to test
                if beta > tangent_angle {
                    return Err(InvalidTriangleError::AngleTooLarge)
                }
            }
            Ok(Self { b, c, beta })
        }
    }
}

impl<T: Scalar> AbstractTriangle<T> for AbstractTriangle_bcβ<T> {
    type len_a_solutions = (T, Option<T>);
    #[inline]
    fn length_a(self) -> Self::len_a_solutions {
        law_of_cosines::a_from_bcβ(self.b, self.c, self.beta).expect("triangle must be valid")
    }

    type len_b_solutions = T;
    #[inline]
    fn length_b(self) -> Self::len_b_solutions { self.b }

    type len_c_solutions = T;
    #[inline]
    fn length_c(self) -> Self::len_c_solutions { self.c }

    type angle_alpha_solutions = (T, Option<T>);
    #[inline]
    fn angle_alpha(self) -> Self::angle_alpha_solutions {
        chain_solution(
            self.length_a(),
            |a| law_of_cosines::alpha_from_abc(a, self.b, self.c).expect("triangle must be valid")
        )
    }

    type angle_beta_solutions = T;
    #[inline]
    fn angle_beta(self) -> Self::angle_beta_solutions { self.beta }

    type angle_gamma_solutions = (T, Option<T>);
    #[inline]
    fn angle_gamma(self) -> Self::angle_gamma_solutions {
        chain_solution(
            self.length_a(),
            |a| law_of_cosines::gamma_from_abc(a, self.b, self.c).expect("triangle must be valid")
        )
    }

    type area_solutions = (T, Option<T>);
    #[inline]
    fn area(self) -> Self::area_solutions {
        chain_solution(
            self.length_a(),
            |a| triangle_area(a, self.length_b(), self.length_c())
            .expect("triangle constructed from points should always be valid!")
        )
    }

    type altitude_a_solutions = T;
    #[inline]
    fn altitude_a(self) -> Self::altitude_a_solutions {
        let a = self.length_a().first();
        let b = self.length_b();
        let c = self.length_c();
        let area = triangle_area(a, b, c)
            .expect("triangle constructed from points should always be valid!");
        T::i(2) * area / a
    }

    type altitude_b_solutions = (T, Option<T>);
    #[inline]
    fn altitude_b(self) -> Self::altitude_b_solutions {
        chain_solution(
            self.length_a(),
            |a| {
                let b = self.length_b();
                let c = self.length_c();
                let area = triangle_area(a, b, c)
                    .expect("triangle constructed from points should always be valid!");
                T::i(2) * area / b
            }
        )
    }

    type altitude_c_solutions = (T, Option<T>);
    #[inline]
    fn altitude_c(self) -> Self::altitude_c_solutions {
        chain_solution(
            self.length_a(),
            |a| {
                let b = self.length_b();
                let c = self.length_c();
                let area = triangle_area(a, b, c)
                    .expect("triangle constructed from points should always be valid!");
                T::i(2) * area / c
            }
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AbstractTriangle_abγ<T> { a: T, b: T, gamma: T }

impl<T: Scalar> AbstractTriangle_abγ<T> {
    pub fn new(a: T, b: T, gamma: T) -> Result<Self, InvalidTriangleError> {
        if a <= T::ZERO || b <= T::ZERO || !a.is_finite() || !b.is_finite() {
            Err(InvalidTriangleError::InvalidLength)
        } else if gamma <= T::ZERO || gamma >= T::PI || !gamma.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else {
            Ok(Self { a, b, gamma })
        }
    }
}

impl<T: Scalar> AbstractTriangle<T> for AbstractTriangle_abγ<T> {
    type len_a_solutions = T;
    #[inline]
    fn length_a(self) -> Self::len_a_solutions { self.a }

    type len_b_solutions = T;
    #[inline]
    fn length_b(self) -> Self::len_b_solutions { self.b }

    type len_c_solutions = T;
    #[inline]
    fn length_c(self) -> Self::len_c_solutions {
        law_of_cosines::c_from_abγ(self.a, self.b, self.gamma).expect("triangle must be valid")
    }

    type angle_alpha_solutions = T;
    #[inline]
    fn angle_alpha(self) -> Self::angle_alpha_solutions {
        law_of_cosines::alpha_from_abc(self.a, self.b, self.length_c()).expect("triangle must be valid")
    }

    type angle_beta_solutions = T;
    #[inline]
    fn angle_beta(self) -> Self::angle_beta_solutions {
        law_of_cosines::beta_from_abc(self.a, self.b, self.length_c()).expect("triangle must be valid")
    }

    type angle_gamma_solutions = T;
    #[inline]
    fn angle_gamma(self) -> Self::angle_gamma_solutions { self.gamma }

    type area_solutions = T;
    #[inline]
    fn area(self) -> Self::area_solutions {
        triangle_area(self.length_a(), self.length_b(), self.length_c())
            .expect("triangle constructed from points should always be valid!")
    }

    type altitude_a_solutions = T;
    #[inline]
    fn altitude_a(self) -> Self::altitude_a_solutions {
        T::i(2) * self.area() / self.length_a()
    }

    type altitude_b_solutions = T;
    #[inline]
    fn altitude_b(self) -> Self::altitude_b_solutions {
        T::i(2) * self.area() / self.length_b()
    }

    type altitude_c_solutions = T;
    #[inline]
    fn altitude_c(self) -> Self::altitude_c_solutions {
        T::i(2) * self.area() / self.length_c()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AbstractTriangle_acγ<T> { a: T, c: T, gamma: T }

impl<T: Scalar> AbstractTriangle_acγ<T> {
    pub fn new(a: T, c: T, gamma: T) -> Result<Self, InvalidTriangleError> {
        if a <= T::ZERO || c <= T::ZERO || !a.is_finite() || !c.is_finite() {
            Err(InvalidTriangleError::InvalidLength)
        } else if gamma <= T::ZERO || gamma >= T::PI || !gamma.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else {
            if let Some(tangent_angle) = (c/a).asin() { // If C > A, all angles are valid so we don't need to test
                if gamma > tangent_angle {
                    return Err(InvalidTriangleError::AngleTooLarge)
                }
            }
            Ok(Self { a, c, gamma })
        }
    }
}

impl<T: Scalar> AbstractTriangle<T> for AbstractTriangle_acγ<T> {
    type len_a_solutions = T;
    #[inline]
    fn length_a(self) -> Self::len_a_solutions { self.a }

    type len_b_solutions = (T, Option<T>);
    #[inline]
    fn length_b(self) -> Self::len_b_solutions {
        law_of_cosines::b_from_acγ(self.a, self.c, self.gamma).expect("triangle must be valid")
    }

    type len_c_solutions = T;
    #[inline]
    fn length_c(self) -> Self::len_c_solutions { self.c }

    type angle_alpha_solutions = (T, Option<T>);
    #[inline]
    fn angle_alpha(self) -> Self::angle_alpha_solutions {
        chain_solution(
            self.length_b(),
            |b| law_of_cosines::alpha_from_abc(self.a, b, self.c).expect("triangle must be valid")
        )
    }

    type angle_beta_solutions = (T, Option<T>);
    #[inline]
    fn angle_beta(self) -> Self::angle_beta_solutions {
        chain_solution(
            self.length_b(),
            |b| law_of_cosines::beta_from_abc(self.a, b, self.c).expect("triangle must be valid")
        )
    }

    type angle_gamma_solutions = T;
    #[inline]
    fn angle_gamma(self) -> Self::angle_gamma_solutions { self.gamma }

    type area_solutions = (T, Option<T>);
    #[inline]
    fn area(self) -> Self::area_solutions {
        chain_solution(
            self.length_b(),
            |b| triangle_area(self.length_a(), b, self.length_c())
                .expect("triangle constructed from points should always be valid!")
        )
    }

    type altitude_a_solutions = (T, Option<T>);
    #[inline]
    fn altitude_a(self) -> Self::altitude_a_solutions {
        chain_solution(
            self.length_b(),
            |b| {
                let a = self.length_a();
                let c = self.length_c();
                let area = triangle_area(a, b, c)
                    .expect("triangle constructed from points should always be valid!");
                T::i(2) * area / a
            }
        )
    }

    type altitude_b_solutions = T;
    #[inline]
    fn altitude_b(self) -> Self::altitude_b_solutions {
        let a = self.length_a();
        let b = self.length_b().first();
        let c = self.length_c();
        let area = triangle_area(a, b, c)
            .expect("triangle constructed from points should always be valid!");
        T::i(2) * area / b
    }

    type altitude_c_solutions = (T, Option<T>);
    #[inline]
    fn altitude_c(self) -> Self::altitude_c_solutions {
        chain_solution(
            self.length_b(),
            |b| {
                let a = self.length_a();
                let c = self.length_c();
                let area = triangle_area(a, b, c)
                    .expect("triangle constructed from points should always be valid!");
                T::i(2) * area / c
            }
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AbstractTriangle_bcγ<T> { b: T, c: T, gamma: T }

impl<T: Scalar> AbstractTriangle_bcγ<T> {
    pub fn new(b: T, c: T, gamma: T) -> Result<Self, InvalidTriangleError> {
        if b <= T::ZERO || c <= T::ZERO || !b.is_finite() || !c.is_finite() {
            Err(InvalidTriangleError::InvalidLength)
        } else if gamma <= T::ZERO || gamma >= T::PI || !gamma.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else {
            if let Some(tangent_angle) = (c/b).asin() { // If C > B, all angles are valid so we don't need to test
                if gamma > tangent_angle {
                    return Err(InvalidTriangleError::AngleTooLarge)
                }
            }
            Ok(Self { b, c, gamma })
        }
    }
}

impl<T: Scalar> AbstractTriangle<T> for AbstractTriangle_bcγ<T> {
    type len_a_solutions = (T, Option<T>);
    #[inline]
    fn length_a(self) -> Self::len_a_solutions {
        law_of_cosines::a_from_bcγ(self.b, self.c, self.gamma).expect("triangle must be valid")
    }

    type len_b_solutions = T;
    #[inline]
    fn length_b(self) -> Self::len_b_solutions { self.b }

    type len_c_solutions = T;
    #[inline]
    fn length_c(self) -> Self::len_c_solutions { self.c }

    type angle_alpha_solutions = (T, Option<T>);
    #[inline]
    fn angle_alpha(self) -> Self::angle_alpha_solutions {
        chain_solution(
            self.length_a(),
            |a| law_of_cosines::alpha_from_abc(a, self.b, self.c).expect("triangle must be valid")
        )
    }

    type angle_beta_solutions = (T, Option<T>);
    #[inline]
    fn angle_beta(self) -> Self::angle_beta_solutions {
        chain_solution(
            self.length_a(),
            |a| law_of_cosines::beta_from_abc(a, self.b, self.c).expect("triangle must be valid")
        )
    }

    type angle_gamma_solutions = T;
    #[inline]
    fn angle_gamma(self) -> Self::angle_gamma_solutions { self.gamma }

    type area_solutions = (T, Option<T>);
    #[inline]
    fn area(self) -> Self::area_solutions {
        chain_solution(
            self.length_a(),
            |a| triangle_area(a, self.length_b(), self.length_c())
                .expect("triangle constructed from points should always be valid!")
        )
    }

    type altitude_a_solutions = T;
    #[inline]
    fn altitude_a(self) -> Self::altitude_a_solutions {
        let a = self.length_a().first();
        let b = self.length_b();
        let c = self.length_c();
        let area = triangle_area(a, b, c)
            .expect("triangle constructed from points should always be valid!");
        T::i(2) * area / a
    }

    type altitude_b_solutions = (T, Option<T>);
    #[inline]
    fn altitude_b(self) -> Self::altitude_b_solutions {
        chain_solution(
            self.length_a(),
            |a| {
                let b = self.length_b();
                let c = self.length_c();
                let area = triangle_area(a, b, c)
                    .expect("triangle constructed from points should always be valid!");
                T::i(2) * area / b
            }
        )
    }

    type altitude_c_solutions = (T, Option<T>);
    #[inline]
    fn altitude_c(self) -> Self::altitude_c_solutions {
        chain_solution(
            self.length_a(),
            |a| {
                let b = self.length_b();
                let c = self.length_c();
                let area = triangle_area(a, b, c)
                    .expect("triangle constructed from points should always be valid!");
                T::i(2) * area / c
            }
        )
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AbstractTriangle_aαβ<T> { a: T, alpha: T, beta: T }

impl<T: Scalar> AbstractTriangle_aαβ<T> {
    pub fn new(a: T, alpha: T, beta: T) -> Result<Self, InvalidTriangleError> {
        if a <= T::ZERO || !a.is_finite() {
            Err(InvalidTriangleError::InvalidLength)
        } else if alpha <= T::ZERO || alpha >= T::PI || !alpha.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else if beta <= T::ZERO || beta >= T::PI || !beta.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else if (alpha + beta) >= T::PI {
            Err(InvalidTriangleError::InvalidAngle)
        } else {
            Ok(Self { a, alpha, beta })
        }
    }
}

impl<T: Scalar> AbstractTriangle<T> for AbstractTriangle_aαβ<T> {
    type len_a_solutions = T;
    #[inline]
    fn length_a(self) -> Self::len_a_solutions { self.a }

    type len_b_solutions = T;
    #[inline]
    fn length_b(self) -> Self::len_b_solutions {
        law_of_sines::b_from_aαβ(self.a, self.alpha, self.beta).expect("triangle must be valid")
    }

    type len_c_solutions = T;
    #[inline]
    fn length_c(self) -> Self::len_c_solutions {
        let gamma = T::PI - (self.alpha + self.beta);
        law_of_sines::c_from_aαγ(self.a, self.alpha, gamma).expect("triangle must be valid")
    }

    type angle_alpha_solutions = T;
    #[inline]
    fn angle_alpha(self) -> Self::angle_alpha_solutions { self.alpha }

    type angle_beta_solutions = T;
    #[inline]
    fn angle_beta(self) -> Self::angle_beta_solutions { self.beta }

    type angle_gamma_solutions = T;
    #[inline]
    fn angle_gamma(self) -> Self::angle_gamma_solutions {
        T::PI - (self.alpha + self.beta)
    }

    type area_solutions = T;
    #[inline]
    fn area(self) -> Self::area_solutions {
        triangle_area(self.length_a(), self.length_b(), self.length_c())
            .expect("triangle constructed from points should always be valid!")
    }

    type altitude_a_solutions = T;
    #[inline]
    fn altitude_a(self) -> Self::altitude_a_solutions {
        T::i(2) * self.area() / self.length_a()
    }

    type altitude_b_solutions = T;
    #[inline]
    fn altitude_b(self) -> Self::altitude_b_solutions {
        T::i(2) * self.area() / self.length_b()
    }

    type altitude_c_solutions = T;
    #[inline]
    fn altitude_c(self) -> Self::altitude_c_solutions {
        T::i(2) * self.area() / self.length_c()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AbstractTriangle_bαβ<T> { b: T, alpha: T, beta: T }

impl<T: Scalar> AbstractTriangle_bαβ<T> {
    pub fn new(b: T, alpha: T, beta: T) -> Result<Self, InvalidTriangleError> {
        if b <= T::ZERO || !b.is_finite() {
            Err(InvalidTriangleError::InvalidLength)
        } else if alpha <= T::ZERO || alpha >= T::PI || !alpha.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else if beta <= T::ZERO || beta >= T::PI || !beta.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else if (alpha + beta) >= T::PI {
            Err(InvalidTriangleError::InvalidAngle)
        } else {
            Ok(Self { b, alpha, beta })
        }
    }
}

impl<T: Scalar> AbstractTriangle<T> for AbstractTriangle_bαβ<T> {
    type len_a_solutions = T;
    #[inline]
    fn length_a(self) -> Self::len_a_solutions { law_of_sines::a_from_bαβ(self.b, self.alpha, self.beta).expect("triangle must be valid") }

    type len_b_solutions = T;
    #[inline]
    fn length_b(self) -> Self::len_b_solutions { self.b }

    type len_c_solutions = T;
    #[inline]
    fn length_c(self) -> Self::len_c_solutions {
        let gamma = T::PI - (self.alpha + self.beta);
        law_of_sines::c_from_bβγ(self.b, self.beta, gamma).expect("triangle must be valid")
    }

    type angle_alpha_solutions = T;
    #[inline]
    fn angle_alpha(self) -> Self::angle_alpha_solutions { self.alpha }

    type angle_beta_solutions = T;
    #[inline]
    fn angle_beta(self) -> Self::angle_beta_solutions { self.beta }

    type angle_gamma_solutions = T;
    #[inline]
    fn angle_gamma(self) -> Self::angle_gamma_solutions {
        T::PI - (self.alpha + self.beta)
    }

    type area_solutions = T;
    #[inline]
    fn area(self) -> Self::area_solutions {
        triangle_area(self.length_a(), self.length_b(), self.length_c())
            .expect("triangle constructed from points should always be valid!")
    }

    type altitude_a_solutions = T;
    #[inline]
    fn altitude_a(self) -> Self::altitude_a_solutions {
        T::i(2) * self.area() / self.length_a()
    }

    type altitude_b_solutions = T;
    #[inline]
    fn altitude_b(self) -> Self::altitude_b_solutions {
        T::i(2) * self.area() / self.length_b()
    }

    type altitude_c_solutions = T;
    #[inline]
    fn altitude_c(self) -> Self::altitude_c_solutions {
        T::i(2) * self.area() / self.length_c()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AbstractTriangle_cαβ<T> { c: T, alpha: T, beta: T }

impl<T: Scalar> AbstractTriangle_cαβ<T> {
    pub fn new(c: T, alpha: T, beta: T) -> Result<Self, InvalidTriangleError> {
        if c <= T::ZERO || !c.is_finite() {
            Err(InvalidTriangleError::InvalidLength)
        } else if alpha <= T::ZERO || alpha >= T::PI || !alpha.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else if beta <= T::ZERO || beta >= T::PI || !beta.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else if (alpha + beta) >= T::PI {
            Err(InvalidTriangleError::InvalidAngle)
        } else {
            Ok(Self { c, alpha, beta })
        }
    }
}

impl<T: Scalar> AbstractTriangle<T> for AbstractTriangle_cαβ<T> {
    type len_a_solutions = T;
    #[inline]
    fn length_a(self) -> Self::len_a_solutions {
        let gamma = T::PI - (self.alpha + self.beta);
        law_of_sines::a_from_cαγ(self.c, self.alpha, gamma).expect("triangle must be valid")
    }

    type len_b_solutions = T;
    #[inline]
    fn length_b(self) -> Self::len_b_solutions {
        let gamma = T::PI - (self.alpha + self.beta);
        law_of_sines::b_from_cβγ(self.c, self.beta, gamma).expect("triangle must be valid")
    }

    type len_c_solutions = T;
    #[inline]
    fn length_c(self) -> Self::len_c_solutions { self.c }

    type angle_alpha_solutions = T;
    #[inline]
    fn angle_alpha(self) -> Self::angle_alpha_solutions { self.alpha }

    type angle_beta_solutions = T;
    #[inline]
    fn angle_beta(self) -> Self::angle_beta_solutions { self.beta }

    type angle_gamma_solutions = T;
    #[inline]
    fn angle_gamma(self) -> Self::angle_gamma_solutions {
        T::PI - (self.alpha + self.beta)
    }

    type area_solutions = T;
    #[inline]
    fn area(self) -> Self::area_solutions {
        triangle_area(self.length_a(), self.length_b(), self.length_c())
            .expect("triangle constructed from points should always be valid!")
    }

    type altitude_a_solutions = T;
    #[inline]
    fn altitude_a(self) -> Self::altitude_a_solutions {
        T::i(2) * self.area() / self.length_a()
    }

    type altitude_b_solutions = T;
    #[inline]
    fn altitude_b(self) -> Self::altitude_b_solutions {
        T::i(2) * self.area() / self.length_b()
    }

    type altitude_c_solutions = T;
    #[inline]
    fn altitude_c(self) -> Self::altitude_c_solutions {
        T::i(2) * self.area() / self.length_c()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AbstractTriangle_aαγ<T> { a: T, alpha: T, gamma: T }

impl<T: Scalar> AbstractTriangle_aαγ<T> {
    pub fn new(a: T, alpha: T, gamma: T) -> Result<Self, InvalidTriangleError> {
        if a <= T::ZERO || !a.is_finite() {
            Err(InvalidTriangleError::InvalidLength)
        } else if alpha <= T::ZERO || alpha >= T::PI || !alpha.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else if gamma <= T::ZERO || gamma >= T::PI || !gamma.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else if (alpha + gamma) >= T::PI {
            Err(InvalidTriangleError::InvalidAngle)
        } else {
            Ok(Self { a, alpha, gamma })
        }
    }
}

impl<T: Scalar> AbstractTriangle<T> for AbstractTriangle_aαγ<T> {
    type len_a_solutions = T;
    #[inline]
    fn length_a(self) -> Self::len_a_solutions { self.a }

    type len_b_solutions = T;
    #[inline]
    fn length_b(self) -> Self::len_b_solutions {
        let beta = T::PI - (self.alpha + self.gamma);
        law_of_sines::b_from_aαβ(self.a, self.alpha, beta).expect("triangle must be valid")
    }

    type len_c_solutions = T;
    #[inline]
    fn length_c(self) -> Self::len_c_solutions {
        law_of_sines::c_from_aαγ(self.a, self.alpha, self.gamma).expect("triangle must be valid")
    }

    type angle_alpha_solutions = T;
    #[inline]
    fn angle_alpha(self) -> Self::angle_alpha_solutions { self.alpha }

    type angle_beta_solutions = T;
    #[inline]
    fn angle_beta(self) -> Self::angle_beta_solutions {
        T::PI - (self.alpha + self.gamma)
    }

    type angle_gamma_solutions = T;
    #[inline]
    fn angle_gamma(self) -> Self::angle_gamma_solutions { self.gamma }

    type area_solutions = T;
    #[inline]
    fn area(self) -> Self::area_solutions {
        triangle_area(self.length_a(), self.length_b(), self.length_c())
            .expect("triangle constructed from points should always be valid!")
    }

    type altitude_a_solutions = T;
    #[inline]
    fn altitude_a(self) -> Self::altitude_a_solutions {
        T::i(2) * self.area() / self.length_a()
    }

    type altitude_b_solutions = T;
    #[inline]
    fn altitude_b(self) -> Self::altitude_b_solutions {
        T::i(2) * self.area() / self.length_b()
    }

    type altitude_c_solutions = T;
    #[inline]
    fn altitude_c(self) -> Self::altitude_c_solutions {
        T::i(2) * self.area() / self.length_c()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AbstractTriangle_bαγ<T> { b: T, alpha: T, gamma: T }

impl<T: Scalar> AbstractTriangle_bαγ<T> {
    pub fn new(b: T, alpha: T, gamma: T) -> Result<Self, InvalidTriangleError> {
        if b <= T::ZERO || !b.is_finite() {
            Err(InvalidTriangleError::InvalidLength)
        } else if alpha <= T::ZERO || alpha >= T::PI || !alpha.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else if gamma <= T::ZERO || gamma >= T::PI || !gamma.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else if (alpha + gamma) >= T::PI {
            Err(InvalidTriangleError::InvalidAngle)
        } else {
            Ok(Self { b, alpha, gamma })
        }
    }
}

impl<T: Scalar> AbstractTriangle<T> for AbstractTriangle_bαγ<T> {
    type len_a_solutions = T;
    #[inline]
    fn length_a(self) -> Self::len_a_solutions {
        let beta = T::PI - (self.alpha + self.gamma);
        law_of_sines::a_from_bαβ(self.b, self.alpha, beta).expect("triangle must be valid")
    }

    type len_b_solutions = T;
    #[inline]
    fn length_b(self) -> Self::len_b_solutions { self.b }

    type len_c_solutions = T;
    #[inline]
    fn length_c(self) -> Self::len_c_solutions {
        let beta = T::PI - (self.alpha + self.gamma);
        law_of_sines::c_from_bβγ(self.b, beta, self.gamma).expect("triangle must be valid")
    }

    type angle_alpha_solutions = T;
    #[inline]
    fn angle_alpha(self) -> Self::angle_alpha_solutions { self.alpha }

    type angle_beta_solutions = T;
    #[inline]
    fn angle_beta(self) -> Self::angle_beta_solutions {
        T::PI - (self.alpha + self.gamma)
    }

    type angle_gamma_solutions = T;
    #[inline]
    fn angle_gamma(self) -> Self::angle_gamma_solutions { self.gamma }

    type area_solutions = T;
    #[inline]
    fn area(self) -> Self::area_solutions {
        triangle_area(self.length_a(), self.length_b(), self.length_c())
            .expect("triangle constructed from points should always be valid!")
    }

    type altitude_a_solutions = T;
    #[inline]
    fn altitude_a(self) -> Self::altitude_a_solutions {
        T::i(2) * self.area() / self.length_a()
    }

    type altitude_b_solutions = T;
    #[inline]
    fn altitude_b(self) -> Self::altitude_b_solutions {
        T::i(2) * self.area() / self.length_b()
    }

    type altitude_c_solutions = T;
    #[inline]
    fn altitude_c(self) -> Self::altitude_c_solutions {
        T::i(2) * self.area() / self.length_c()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AbstractTriangle_cαγ<T> { c: T, alpha: T, gamma: T }

impl<T: Scalar> AbstractTriangle_cαγ<T> {
    pub fn new(c: T, alpha: T, gamma: T) -> Result<Self, InvalidTriangleError> {
        if c <= T::ZERO || !c.is_finite() {
            Err(InvalidTriangleError::InvalidLength)
        } else if alpha <= T::ZERO || alpha >= T::PI || !alpha.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else if gamma <= T::ZERO || gamma >= T::PI || !gamma.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else if (alpha + gamma) >= T::PI {
            Err(InvalidTriangleError::InvalidAngle)
        } else {
            Ok(Self { c, alpha, gamma })
        }
    }
}

impl<T: Scalar> AbstractTriangle<T> for AbstractTriangle_cαγ<T> {
    type len_a_solutions = T;
    #[inline]
    fn length_a(self) -> Self::len_a_solutions { law_of_sines::a_from_cαγ(self.c, self.alpha, self.gamma).expect("triangle must be valid") }

    type len_b_solutions = T;
    #[inline]
    fn length_b(self) -> Self::len_b_solutions {
        let beta = T::PI - (self.alpha + self.gamma);
        law_of_sines::b_from_cβγ(self.c, beta, self.gamma).expect("triangle must be valid")
    }

    type len_c_solutions = T;
    #[inline]
    fn length_c(self) -> Self::len_c_solutions { self.c }

    type angle_alpha_solutions = T;
    #[inline]
    fn angle_alpha(self) -> Self::angle_alpha_solutions { self.alpha }

    type angle_beta_solutions = T;
    #[inline]
    fn angle_beta(self) -> Self::angle_beta_solutions {
        T::PI - (self.alpha + self.gamma)
    }

    type angle_gamma_solutions = T;
    #[inline]
    fn angle_gamma(self) -> Self::angle_gamma_solutions { self.gamma }

    type area_solutions = T;
    #[inline]
    fn area(self) -> Self::area_solutions {
        triangle_area(self.length_a(), self.length_b(), self.length_c())
            .expect("triangle constructed from points should always be valid!")
    }

    type altitude_a_solutions = T;
    #[inline]
    fn altitude_a(self) -> Self::altitude_a_solutions {
        T::i(2) * self.area() / self.length_a()
    }

    type altitude_b_solutions = T;
    #[inline]
    fn altitude_b(self) -> Self::altitude_b_solutions {
        T::i(2) * self.area() / self.length_b()
    }

    type altitude_c_solutions = T;
    #[inline]
    fn altitude_c(self) -> Self::altitude_c_solutions {
        T::i(2) * self.area() / self.length_c()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AbstractTriangle_aβγ<T> { a: T, beta: T, gamma: T }

impl<T: Scalar> AbstractTriangle_aβγ<T> {
    pub fn new(a: T, beta: T, gamma: T) -> Result<Self, InvalidTriangleError> {
        if a <= T::ZERO || !a.is_finite() {
            Err(InvalidTriangleError::InvalidLength)
        } else if beta <= T::ZERO || beta >= T::PI || !beta.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else if gamma <= T::ZERO || gamma >= T::PI || !gamma.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else if (beta + gamma) >= T::PI {
            Err(InvalidTriangleError::InvalidAngle)
        } else {
            Ok(Self { a, beta, gamma })
        }
    }
}

impl<T: Scalar> AbstractTriangle<T> for AbstractTriangle_aβγ<T> {
    type len_a_solutions = T;
    #[inline]
    fn length_a(self) -> Self::len_a_solutions { self.a }

    type len_b_solutions = T;
    #[inline]
    fn length_b(self) -> Self::len_b_solutions {
        let alpha = T::PI - (self.beta + self.gamma);
        law_of_sines::b_from_aαβ(self.a, alpha, self.beta).expect("triangle must be valid")
    }

    type len_c_solutions = T;
    #[inline]
    fn length_c(self) -> Self::len_c_solutions {
        let alpha = T::PI - (self.beta + self.gamma);
        law_of_sines::c_from_aαγ(self.a, alpha, self.gamma).expect("triangle must be valid")
    }

    type angle_alpha_solutions = T;
    #[inline]
    fn angle_alpha(self) -> Self::angle_alpha_solutions {
        T::PI - (self.beta + self.gamma)
    }

    type angle_beta_solutions = T;
    #[inline]
    fn angle_beta(self) -> Self::angle_beta_solutions { self.beta }

    type angle_gamma_solutions = T;
    #[inline]
    fn angle_gamma(self) -> Self::angle_gamma_solutions { self.gamma }

    type area_solutions = T;
    #[inline]
    fn area(self) -> Self::area_solutions {
        triangle_area(self.length_a(), self.length_b(), self.length_c())
            .expect("triangle constructed from points should always be valid!")
    }

    type altitude_a_solutions = T;
    #[inline]
    fn altitude_a(self) -> Self::altitude_a_solutions {
        T::i(2) * self.area() / self.length_a()
    }

    type altitude_b_solutions = T;
    #[inline]
    fn altitude_b(self) -> Self::altitude_b_solutions {
        T::i(2) * self.area() / self.length_b()
    }

    type altitude_c_solutions = T;
    #[inline]
    fn altitude_c(self) -> Self::altitude_c_solutions {
        T::i(2) * self.area() / self.length_c()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AbstractTriangle_bβγ<T> { b: T, beta: T, gamma: T }

impl<T: Scalar> AbstractTriangle_bβγ<T> {
    pub fn new(b: T, beta: T, gamma: T) -> Result<Self, InvalidTriangleError> {
        if b <= T::ZERO || !b.is_finite() {
            Err(InvalidTriangleError::InvalidLength)
        } else if beta <= T::ZERO || beta >= T::PI || !beta.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else if gamma <= T::ZERO || gamma >= T::PI || !gamma.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else if (beta + gamma) >= T::PI {
            Err(InvalidTriangleError::InvalidAngle)
        } else {
            Ok(Self { b, beta, gamma })
        }
    }
}

impl<T: Scalar> AbstractTriangle<T> for AbstractTriangle_bβγ<T> {
    type len_a_solutions = T;
    #[inline]
    fn length_a(self) -> Self::len_a_solutions {
        let alpha = T::PI - (self.beta + self.gamma);
        law_of_sines::a_from_bαβ(self.b, alpha, self.beta).expect("triangle must be valid")
    }

    type len_b_solutions = T;
    #[inline]
    fn length_b(self) -> Self::len_b_solutions { self.b }

    type len_c_solutions = T;
    #[inline]
    fn length_c(self) -> Self::len_c_solutions {
        law_of_sines::c_from_bβγ(self.b, self.beta, self.gamma).expect("triangle must be valid")
    }

    type angle_alpha_solutions = T;
    #[inline]
    fn angle_alpha(self) -> Self::angle_alpha_solutions {
        T::PI - (self.beta + self.gamma)
    }

    type angle_beta_solutions = T;
    #[inline]
    fn angle_beta(self) -> Self::angle_beta_solutions { self.beta }

    type angle_gamma_solutions = T;
    #[inline]
    fn angle_gamma(self) -> Self::angle_gamma_solutions { self.gamma }

    type area_solutions = T;
    #[inline]
    fn area(self) -> Self::area_solutions {
        triangle_area(self.length_a(), self.length_b(), self.length_c())
            .expect("triangle constructed from points should always be valid!")
    }

    type altitude_a_solutions = T;
    #[inline]
    fn altitude_a(self) -> Self::altitude_a_solutions {
        T::i(2) * self.area() / self.length_a()
    }

    type altitude_b_solutions = T;
    #[inline]
    fn altitude_b(self) -> Self::altitude_b_solutions {
        T::i(2) * self.area() / self.length_b()
    }

    type altitude_c_solutions = T;
    #[inline]
    fn altitude_c(self) -> Self::altitude_c_solutions {
        T::i(2) * self.area() / self.length_c()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct AbstractTriangle_cβγ<T> { c: T, beta: T, gamma: T }

impl<T: Scalar> AbstractTriangle_cβγ<T> {
    pub fn new(c: T, beta: T, gamma: T) -> Result<Self, InvalidTriangleError> {
        if c <= T::ZERO || !c.is_finite() {
            Err(InvalidTriangleError::InvalidLength)
        } else if beta <= T::ZERO || beta >= T::PI || !beta.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else if gamma <= T::ZERO || gamma >= T::PI || !gamma.is_finite() {
            Err(InvalidTriangleError::InvalidAngle)
        } else if (beta + gamma) >= T::PI {
            Err(InvalidTriangleError::InvalidAngle)
        } else {
            Ok(Self { c, beta, gamma })
        }
    }
}

impl<T: Scalar> AbstractTriangle<T> for AbstractTriangle_cβγ<T> {
    type len_a_solutions = T;
    #[inline]
    fn length_a(self) -> Self::len_a_solutions {
        let alpha = T::PI - (self.beta + self.gamma);
        law_of_sines::a_from_cαγ(self.c, alpha, self.gamma).expect("triangle must be valid")
    }

    type len_b_solutions = T;
    #[inline]
    fn length_b(self) -> Self::len_b_solutions {
        law_of_sines::b_from_cβγ(self.c, self.beta, self.gamma).expect("triangle must be valid")
    }

    type len_c_solutions = T;
    #[inline]
    fn length_c(self) -> Self::len_c_solutions { self.c }

    type angle_alpha_solutions = T;
    #[inline]
    fn angle_alpha(self) -> Self::angle_alpha_solutions {
        T::PI - (self.beta + self.gamma)
    }

    type angle_beta_solutions = T;
    #[inline]
    fn angle_beta(self) -> Self::angle_beta_solutions { self.beta }

    type angle_gamma_solutions = T;
    #[inline]
    fn angle_gamma(self) -> Self::angle_gamma_solutions { self.gamma }

    type area_solutions = T;
    #[inline]
    fn area(self) -> Self::area_solutions {
        triangle_area(self.length_a(), self.length_b(), self.length_c())
            .expect("triangle constructed from points should always be valid!")
    }

    type altitude_a_solutions = T;
    #[inline]
    fn altitude_a(self) -> Self::altitude_a_solutions {
        T::i(2) * self.area() / self.length_a()
    }

    type altitude_b_solutions = T;
    #[inline]
    fn altitude_b(self) -> Self::altitude_b_solutions {
        T::i(2) * self.area() / self.length_b()
    }

    type altitude_c_solutions = T;
    #[inline]
    fn altitude_c(self) -> Self::altitude_c_solutions {
        T::i(2) * self.area() / self.length_c()
    }
}

pub mod formulas {
    use crate::scalar::Scalar;
    use crate::utility::InvalidInput;

    #[inline]
    pub fn triangle_area<T: Scalar>(a: T, b: T, c: T) -> Result<T, InvalidInput> {
        let s = T::f(0.5)*(a + b + c);  // Using two steps is probably better for floating point accuracy?
        let A2 = s*(s-a)*(s-b)*(s-c);
        Ok(A2.sqrt())
    }
}
pub mod law_of_sines {
    use crate::scalar::Scalar;
    use crate::utility::InvalidInput;

    // TODO: Optimized versions for retrieving multiple values

    /// Calculates length `a` from length `b` + angles `alpha` and `beta` (in radians)
    ///
    /// Always returns Ok() where  `b`, `alpha`, `beta` > 0 and `alpha`, `beta` < PI and `alpha` + `beta` < PI
    #[inline]
    pub fn a_from_bαβ<T: Scalar>(b: T, alpha: T, beta: T) -> Result<T, InvalidInput> {
        Some(b*(alpha.sin()/beta.sin()))
            .filter(|v| *v >= T::ZERO && v.is_finite()) // If `a` length is negative/infinity/NaN, there is no solution.
            .ok_or(InvalidInput)    // `a` is positive for all valid input
    }

    /// Calculates length `a` from length `c` + angles `alpha` and `gamma` (in radians)
    ///
    /// Always returns Ok() where  `c`, `alpha`, `gamma` > 0 and `alpha`, `gamma` < PI and `alpha` + `gamma` < PI
    #[inline]
    pub fn a_from_cαγ<T: Scalar>(c: T, alpha: T, gamma: T) -> Result<T, InvalidInput> {
        Some(c*(alpha.sin()/gamma.sin()))
            .filter(|v| *v >= T::ZERO && v.is_finite()) // If `a` length is negative/infinity/NaN, there is no solution.
            .ok_or(InvalidInput)    // `a` is positive for all valid input
    }

    /// Calculates length `b` from length `a` + angles `alpha` and `beta` (in radians)
    ///
    /// Always returns Ok() where  `a`, `alpha`, `beta` > 0 and `alpha`, `beta` < PI and `alpha` + `beta` < PI
    #[inline]
    pub fn b_from_aαβ<T: Scalar>(a: T, alpha: T, beta: T) -> Result<T, InvalidInput> {
        Some(a*(beta.sin()/alpha.sin()))
            .filter(|v| *v >= T::ZERO && v.is_finite()) // If `b` length is negative/infinity/NaN, there is no solution.
            .ok_or(InvalidInput)    // `b` is positive for all valid input
    }

    /// Calculates length `b` from length `c` + angles `beta` and `gamma` (in radians)
    ///
    /// Always returns Ok() where  `c`, `beta`, `gamma` > 0 and `beta`, `gamma` < PI and `beta` + `gamma` < PI
    #[inline]
    pub fn b_from_cβγ<T: Scalar>(c: T, beta: T, gamma: T) -> Result<T, InvalidInput> {
        Some(c*(beta.sin()/gamma.sin()))
            .filter(|v| *v >= T::ZERO && v.is_finite()) // If `b` length is negative/infinity/NaN, there is no solution.
            .ok_or(InvalidInput)    // `b` is positive for all valid input
    }

    /// Calculates length `c` from length `a` + angles `alpha` and `gamma` (in radians)
    ///
    /// Always returns Ok() where  `a`, `alpha`, `gamma` > 0 and `alpha`, `gamma` < PI and `alpha` + `gamma` < PI
    #[inline]
    pub fn c_from_aαγ<T: Scalar>(a: T, alpha: T, gamma: T) -> Result<T, InvalidInput> {
        Some(a*(gamma.sin()/alpha.sin()))
            .filter(|v| *v >= T::ZERO && v.is_finite()) // If `c` length is negative/infinity/NaN, there is no solution.
            .ok_or(InvalidInput)    // `c` is positive for all valid input
    }

    /// Calculates length `c` from length `b` + angles `beta` and `gamma` (in radians)
    ///
    /// Always returns Ok() where  `b`, `beta`, `gamma` > 0 and `beta`, `gamma` < PI and `beta` + `gamma` < PI
    #[inline]
    pub fn c_from_bβγ<T: Scalar>(b: T, beta: T, gamma: T) -> Result<T, InvalidInput> {
        Some(b*(gamma.sin() / beta.sin()))
            .filter(|v| *v >= T::ZERO && v.is_finite()) // If `c` length is negative/infinity/NaN, there is no solution.
            .ok_or(InvalidInput)    // `c` is positive for all valid input
    }
}
pub mod law_of_cosines {
    use crate::scalar::Scalar;
    use crate::utility::InvalidInput;

    /// Returns positive results of (`value` ± √`squared`)
    #[inline]
    fn return_solutions<T: Scalar>(value: T, squared: T) -> Result<(T, Option<T>), InvalidInput> {
        if squared < T::ZERO || !value.is_finite() || !squared.is_finite() {    // "Fail-fast" on NaN/infinity by returning InvalidInput rather than a NaN Ok result
            return Err(InvalidInput);
        }

        let root = squared.sqrt();
        let one = value + root;
        let two = value - root;

        match (one > T::ZERO, two > T::ZERO, one == two) {
            (true, false, false) => Ok((one, None)),
            (false, true, false) => Ok((two, None)),
            (true, true, false) => Ok((one, Some(two))),
            (true, true, true) => Ok((one, None)),
            (_, _, _) => Err(InvalidInput)
        }
    }

    /// Calculates length `a` from lengths `b` and `c` + angle `alpha` (in radians)
    ///
    /// Always returns Ok() where  `b`, `c`, `alpha` > 0 and `alpha` < PI
    #[inline]
    pub fn a_from_bcα<T: Scalar>(b: T, c: T, alpha: T) -> Result<T, InvalidInput> {
        let a_squared = b.powi(2) + c.powi(2) - (T::i(2) * b * c * alpha.cos());
        if a_squared > T::ZERO && a_squared.is_finite() {    // Return None instead of NaN if there are no solutions
            Ok(a_squared.sqrt())
        } else {
            Err(InvalidInput)
        }
    }

    /// Calculates length `a` from lengths `b` and `c` + angle `beta` (in radians)
    ///
    /// Always returns Ok() where  `b`, `c`, `beta` > 0 and `beta' < PI
    #[inline]
    pub fn a_from_bcβ<T: Scalar>(b: T, c: T, beta: T) -> Result<(T, Option<T>), InvalidInput> {
        let val = c * beta.cos();
        let squared = b.powi(2) + (c.powi(2) * beta.cos().powi(2)) - c.powi(2);
        return_solutions(val, squared)
    }

    /// Calculates length `a` from lengths `b` and `c` + angle `gamma` (in radians)
    ///
    /// Always returns Ok() where  `b`, `c`, `gamma` > 0 and `beta' < PI
    #[inline]
    pub fn a_from_bcγ<T: Scalar>(b: T, c: T, gamma: T) -> Result<(T, Option<T>), InvalidInput> {
        let val = b * gamma.cos();
        let squared = b.powi(2) * gamma.cos().powi(2) - b.powi(2) + c.powi(2);
        return_solutions(val, squared)
    }

    /// Calculates length `b` from lengths `a` and `c` + angle `alpha` (in radians)
    ///
    /// Always returns Ok() where  `a`, `c`, `alpha` > 0 and `beta' < PI
    #[inline]
    pub fn b_from_acα<T: Scalar>(a: T, c: T, alpha: T) -> Result<(T, Option<T>), InvalidInput> {
        let val = c * alpha.cos();
        let squared = a.powi(2) + (c.powi(2) * alpha.cos().powi(2)) - c.powi(2);
        return_solutions(val, squared)
    }

    /// Calculates length `b` from lengths `a` and `c` + angle `beta` (in radians)
    ///
    /// Always returns Ok() where  `a`, `c`, `beta` > 0 and `beta` < PI
    #[inline]
    pub fn b_from_acβ<T: Scalar>(a: T, c: T, beta: T) -> Result<T, InvalidInput> {
        let b_squared = a.powi(2) + c.powi(2) - (T::i(2) * a * c * beta.cos());
        if b_squared > T::ZERO && b_squared.is_finite() {    // Return None instead of NaN if there are no solutions
            Ok(b_squared.sqrt())
        } else {
            Err(InvalidInput)
        }
    }

    /// Calculates length `b` from lengths `a` and `c` + angle `gamma` (in radians)
    ///
    /// Always returns Ok() where  `a`, `c`, `gamma` > 0 and `beta' < PI
    #[inline]
    pub fn b_from_acγ<T: Scalar>(a: T, c: T, gamma: T) -> Result<(T, Option<T>), InvalidInput> {
        let val = a * gamma.cos();
        let squared = c.powi(2) + (a.powi(2) * gamma.cos().powi(2)) - a.powi(2);
        return_solutions(val, squared)
    }

    /// Calculates length `c` from lengths `a` and `b` + angle `alpha` (in radians)
    ///
    /// Always returns Ok() where  `a`, `b`, `alpha` > 0 and `beta' < PI
    #[inline]
    pub fn c_from_abα<T: Scalar>(a: T, b: T, alpha: T) -> Result<(T, Option<T>), InvalidInput> {
        let val = b * alpha.cos();
        let squared = a.powi(2) + (b.powi(2) * alpha.cos().powi(2)) - b.powi(2);
        return_solutions(val, squared)
    }

    /// Calculates length `c` from lengths `a` and `b` + angle `beta` (in radians)
    ///
    /// Always returns Ok() where  `a`, `b`, `beta` > 0 and `beta' < PI
    #[inline]
    pub fn c_from_abβ<T: Scalar>(a: T, b: T, beta: T) -> Result<(T, Option<T>), InvalidInput> {
        let val = a * beta.cos();
        let squared = b.powi(2) + (a.powi(2) * beta.cos().powi(2)) - a.powi(2);
        return_solutions(val, squared)
    }

    /// Calculates length `c` from lengths `a` and `b` + angle `gamma` (in radians)
    ///
    /// Always returns Ok() where  `a`, `b`, `gamma` > 0 and `gamma` < PI
    #[inline]
    pub fn c_from_abγ<T: Scalar>(a: T, b: T, gamma: T) -> Result<T, InvalidInput> {
        let c_squared = a.powi(2) + b.powi(2) - (T::i(2) * a * b * gamma.cos());
        if c_squared > T::ZERO && c_squared.is_finite() {    // Return None instead of NaN if there are no solutions
            Ok(c_squared.sqrt())
        } else {
            Err(InvalidInput)
        }
    }

    /// Calculates angle `alpha` (in radians) from lengths `a`, `b` and `c`
    ///
    /// Always returns Ok() where  `a`, `b`, `c` > 0 and the sum of two lengths is greater than the third length
    #[inline]
    pub fn alpha_from_abc<T: Scalar>(a: T, b: T, c: T) -> Result<T, InvalidInput> {
        T::acos((-a.powi(2) + b.powi(2) + c.powi(2)) / (T::i(2) * b * c))
            .ok_or(InvalidInput)
    }


    /// Calculates angle `beta` (in radians) from lengths `a`, `b` and `c`
    ///
    /// Always returns Ok() where  `a`, `b`, `c` > 0 and the sum of two lengths is greater than the third length
    #[inline]
    pub fn beta_from_abc<T: Scalar>(a: T, b: T, c: T) -> Result<T, InvalidInput> {
        T::acos((a.powi(2) - b.powi(2) + c.powi(2)) / (T::i(2) * a * c))
            .ok_or(InvalidInput)
    }

    /// Calculates angle `gamma` (in radians) from lengths `a`, `b` and `c`
    ///
    /// Always returns Ok() where  `a`, `b`, `c` > 0 and the sum of two lengths is greater than the third length
    #[inline]
    pub fn gamma_from_abc<T: Scalar>(a: T, b: T, c: T) -> Result<T, InvalidInput> {
        T::acos((a.powi(2) + b.powi(2) - c.powi(2)) / (T::i(2) * a * b))
            .ok_or(InvalidInput)
    }
}
