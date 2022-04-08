use super::hit::{Hit, HitRecord};
use super::ray::Ray;
use super::vec::{Point3, Vec3};

pub struct Sphere {
    center: Point3,
    radius: f64,
}

impl Sphere {
    /// Create a new `Sphere`.
    pub fn new(center: Point3, radius: f64) -> Self {
        Self { center, radius }
    }
}

impl Hit for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length().powi(2);
        let half_b = oc.dot(r.direction());
        let c = oc.length().powi(2) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return None;
        }

        // Find the nearest root that lies in the acceptable range
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = r.at(root);
        let hr = HitRecord {
            t: root,
            p,
            normal: (p - self.center) / self.radius,
        };

        Some(hr)
    }
}
