use std::sync::Arc;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    shape::Point4,
    vec::Point3,
};

#[derive(Debug)]
pub struct MovingSphere {
    center0: Point4,
    center1: Point4,
    radius: f64,
    material: Arc<dyn Material>,
}

impl MovingSphere {
    /// Create new `MovingSphere`.
    pub fn new(center0: Point4, center1: Point4, radius: f64, material: Arc<dyn Material>) -> Self {
        Self {
            center0,
            center1,
            radius,
            material,
        }
    }

    /// Get location of center at given `time`.
    pub fn center(&self, time: f64) -> Point3 {
        self.center0.pos
            + ((time - self.center0.time) / (self.center1.time - self.center0.time))
                * (self.center1.pos - self.center0.pos)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center(ray.time());
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

        let time = root;
        let p = ray.at(time);
        let outward_normal = (p - self.center(ray.time())) / self.radius;
        let material = Arc::clone(&self.material);

        Some(HitRecord::new(ray, p, outward_normal, material, time))
    }
}

#[cfg(test)]
mod tests {
    use crate::material::Dielectric;

    use super::*;

    #[test]
    fn moving_sphere_center() {
        let center0 = Point4 {
            pos: Point3::new(0.0, 0.0, 0.0),
            time: 0.0,
        };
        let center1 = Point4 {
            pos: Point3::new(1.0, 0.0, 0.0),
            time: 1.0,
        };
        let ms = MovingSphere::new(center0, center1, 1.0, Arc::new(Dielectric::new(1.5)));

        let actual = ms.center(0.5);
        let expected = Point3::new(0.5, 0.0, 0.0);

        assert_eq!(actual, expected);
    }
}
