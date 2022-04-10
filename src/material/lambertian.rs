use super::Material;
use crate::{
    hittable::HitRecord,
    ray::Ray,
    vec::{Color, Vec3},
};

#[derive(Debug, PartialEq)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    /// Create a new `Lambertian` material.
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hr.norm() + Vec3::random_in_unit_sphere().normalized();
        if scatter_direction.near_zero() {
            // catch degenerate scatter direction
            scatter_direction = hr.norm();
        }

        let scattered = Ray::new(hr.p(), scatter_direction, 0.0);

        Some((self.albedo, scattered))
    }
}
