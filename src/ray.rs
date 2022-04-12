use super::vec::{Point3, Vec3};

/// A ray, beginning at `origin` and extending along `direction`.
#[derive(Debug)]
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

    /// Finds the point along the ray at distance `t` from the origin. Positive
    /// values of `t` represent positions forward from the origin, and negative
    /// values, behind the origin.
    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
