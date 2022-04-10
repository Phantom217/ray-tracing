use std::sync::Arc;

use crate::{
    material::Material,
    ray::Ray,
    vec::{Point3, Vec3},
};

pub struct HitRecord {
    p: Point3,
    norm: Vec3,
    material: Arc<dyn Material>,
    t: f64,
    front_face: bool,
}

impl HitRecord {
    pub fn new(
        ray: &Ray,
        p: Point3,
        outward_normal: Vec3,
        material: Arc<dyn Material>,
        t: f64,
    ) -> Self {
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        let norm = if front_face {
            outward_normal
        } else {
            -1.0 * outward_normal
        };

        Self {
            p,
            norm,
            material,
            t,
            front_face,
        }
    }

    pub const fn p(&self) -> Point3 {
        self.p
    }

    pub const fn norm(&self) -> Vec3 {
        self.norm
    }

    pub const fn t(&self) -> f64 {
        self.t
    }

    pub const fn front_face(&self) -> bool {
        self.front_face
    }

    pub fn material(&self) -> Arc<dyn Material> {
        Arc::clone(&self.material)
    }
}
