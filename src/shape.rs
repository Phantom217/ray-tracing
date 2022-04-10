mod moving_sphere;
mod sphere;

pub use moving_sphere::MovingSphere;
pub use sphere::Sphere;

use crate::vec::Point3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Point4 {
    pos: Point3,
    time: f64,
}
