use super::vec::{Point3, Vec3};

/// Type to represent a ray of light.
pub struct Ray {
    origin: Point3,
    direction: Vec3,
    time: f64,
}

impl Ray {
    /// Create a new `Ray`.
    pub fn new(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self {
            origin,
            direction,
            time,
        }
    }

    /// Get origin of the `Ray`.
    pub fn origin(&self) -> Point3 {
        self.origin
    }

    /// Get direction of the `Ray`.
    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    /// Get point in time in which the `Ray` exists.
    pub fn time(&self) -> f64 {
        self.time
    }

    /// Get position of the `Ray` at given time (`t`).
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
