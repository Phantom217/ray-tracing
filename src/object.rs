mod sphere;

pub use sphere::{MovingSphere, Sphere};

use crate::vec::Point3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point4 {
    pos: Point3,
    time: f64,
}

impl Point4 {
    pub fn new(pos: Point3, time: f64) -> Self {
        Self { pos, time }
    }
}
