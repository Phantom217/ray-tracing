use std::sync::Arc;

use super::{
    material::Material,
    ray::Ray,
    vec::{Point3, Vec3},
};

pub struct HitRecord {
    pub p: Point3,
    pub norm: Vec3,
    pub material: Arc<dyn Material>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: Vec3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.norm = if self.front_face {
            outward_normal
        } else {
            -1.0 * outward_normal
        };
    }
}

pub trait Hit: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub type World = Vec<Box<dyn Hit>>;

impl Hit for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmp_hr = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(hr) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hr.t;
                tmp_hr = Some(hr);
            }
        }

        tmp_hr
    }
}
