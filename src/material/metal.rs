use super::Material;
use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec::{Color, Vec3},
};

#[derive(Debug, PartialEq)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    /// Create a new `Metal` material.
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray_in.direction().reflect(hr.norm()).normalized();
        let scattered = Ray::new(
            hr.p(),
            reflected + self.fuzz * Vec3::random_in_unit_sphere(),
            ray_in.time(),
        );

        if scattered.direction().dot(hr.norm()) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
