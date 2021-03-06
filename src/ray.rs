use crate::vec3::Vec3;

/// A ray, beginning at `origin` and extending along `direction`.
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    /// Finds the point along the ray at distance `t` from the origin. Positive values of `t`
    /// represent positions forward from the origin, and negative values, behind the origin.
    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}
