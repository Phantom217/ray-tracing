use std::sync::Arc;

use super::{
    hit::{Hit, HitRecord},
    material::Material,
    ray::Ray,
    vec::Point3,
};

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    /// Create a new `Sphere`.
    pub fn new(center: Point3, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length().powi(2);
        let half_b = oc.dot(ray.direction());
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

        let mut hr = HitRecord {
            p: ray.at(root),
            norm: Point3::new(0.0, 0.0, 0.0),
            material: self.material.clone(),
            t: root,
            front_face: false,
        };

        let outward_normal = (hr.p - self.center) / self.radius;
        hr.set_face_normal(ray, outward_normal);

        Some(hr)
    }
}
