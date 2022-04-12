use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
    ray::Ray,
    object::Point4,
    vec::Point3,
};

/// A sphere
#[derive(Debug)]
pub struct Sphere<M: Material> {
    center: Point3,
    radius: f64,
    material: M,
}

impl<M: Material> Sphere<M> {
    /// Create a new `Sphere`.
    pub fn new(center: Point3, radius: f64, material: M) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl<M: Material> Hittable for Sphere<M> {
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

        let time = root;
        let p = ray.at(time);
        let outward_normal = (p - self.center) / self.radius;

        Some(HitRecord::new(ray, p, outward_normal, &self.material, time))
    }
}

#[derive(Debug)]
pub struct MovingSphere<M: Material> {
    center0: Point4,
    center1: Point4,
    radius: f64,
    material: M,
}

impl<M: Material> MovingSphere<M> {
    /// Create new `MovingSphere`.
    pub fn new(center0: Point4, center1: Point4, radius: f64, material: M) -> Self {
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

impl<M: Material> Hittable for MovingSphere<M> {
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

        Some(HitRecord::new(ray, p, outward_normal, &self.material, time))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod moving_sphere {
        use super::*;
        use crate::material::Dielectric;

        #[test]
        fn test_center() {
            let center0 = Point4 {
                pos: Point3::new(0.0, 0.0, 0.0),
                time: 0.0,
            };
            let center1 = Point4 {
                pos: Point3::new(1.0, 0.0, 0.0),
                time: 1.0,
            };
            let ms = MovingSphere::new(center0, center1, 1.0, Dielectric::new(1.5));

            let actual = ms.center(0.5);
            let expected = Point3::new(0.5, 0.0, 0.0);

            assert_eq!(actual, expected);
        }
    }
}
