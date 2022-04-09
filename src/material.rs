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
    /// Create a new `Lambertian` material.
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

pub struct Metal {
    albedo: Color,
}

impl Metal {
    /// Create a new `Metal` material.
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray_in.direction().reflect(hr.normal).normalized();
        let scattered = Ray::new(hr.p, reflected);

        if scattered.direction().dot(hr.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}
