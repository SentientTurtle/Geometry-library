use std::fmt::{Display, Formatter};
use crate::basis::Basis;
use crate::geometry3d::Point3D;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Sphere<T, B: Basis<3>> {
    pub center: Point3D<T, B>,
    pub radius: T
}

impl<T: Display, B: Basis<3>> Display for Sphere<T, B> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Sphere{{{}, radius={}}}", self.center, self.radius)
    }
}
