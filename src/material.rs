use super::hit::HitRecord;
use super::ray::Ray;
use super::vec::Color;
use super::vec::Vec3;

pub trait Scatter {
    fn scatter(&self, ray_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, ray_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hr.normal + Vec3::random_in_unit_sphere().normalized();
        if scatter_direction.near_zero() {
            // catch degenerate scatter direction
            scatter_direction = hr.normal;
        }

        let scattered = Ray::new(hr.p, scatter_direction);

        Some((self.albedo, scattered))
    }
}
