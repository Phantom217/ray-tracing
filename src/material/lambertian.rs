use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{Color, Vec3};

/// An opaque material with a matte surface, where lighting is calculated
/// using [Lambertian reflectance][lambert].
///
/// [lambert]: https://en.wikipedia.org/wiki/Lambertian_reflectance
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
    fn scatter(&self, ray_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hr.norm() + Vec3::random_in_unit_sphere().normalized();
        if scatter_direction.near_zero() {
            // catch degenerate scatter direction
            scatter_direction = hr.norm();
        }

        let scattered = Ray::new(hr.p(), scatter_direction, ray_in.time());

        Some((self.albedo, scattered))
    }
}
