use super::vec::{Point3, Vec3};

pub struct Ray {
    origin: Point3,
    direction: Vec3,
}

impl Ray {
    /// Create a new `Ray`.
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    /// Get origin of `Ray`.
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    /// Get direction of `Ray`.
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    /// Get position of ray at given time (`t`).
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
