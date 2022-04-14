use crate::ray::Ray;
use crate::vec3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn merge(self, other: Aabb) -> Self {
        Self {
            min: self.min.zip_with(other.min, f64::min),
            max: self.max.zip_with(other.max, f64::max),
        }
    }

    pub fn hit(&self, ray: &Ray, t_range: std::ops::Range<f64>) -> bool {
        let inv_d = ray.direction.map(|x| 1. / x);
        let t0 = (self.min - ray.origin) * inv_d;
        let t1 = (self.max - ray.origin) * inv_d;

        let (t0, t1) = (
            inv_d.zip_with3(t0, t1, |i, a, b| if i < 0. { b } else { a }),
            inv_d.zip_with3(t0, t1, |i, a, b| if i < 0. { a } else { b }),
        );

        let start = t_range.start.max(t0.reduce(f64::max));
        let end = t_range.end.min(t1.reduce(f64::min));

        end > start
    }
}