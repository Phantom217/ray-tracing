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
    fn scatter(&self, _ray_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)> {
        let mut scatter_direction = hr.norm + Vec3::random_in_unit_sphere().normalized();
        if scatter_direction.near_zero() {
            // catch degenerate scatter direction
            scatter_direction = hr.norm;
        }

        let scattered = Ray::new(hr.p, scatter_direction);

        Some((self.albedo, scattered))
    }
}

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

impl Scatter for Metal {
    fn scatter(&self, ray_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)> {
        let reflected = ray_in.direction().reflect(hr.norm).normalized();
        let scattered = Ray::new(hr.p, reflected + self.fuzz * Vec3::random_in_unit_sphere());

        if scattered.direction().dot(hr.norm) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    /// Index of refraction
    ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ir: index_of_refraction,
        }
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, ray_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if hr.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray_in.direction().normalized();
        let refracted = unit_direction.refract(hr.norm, refraction_ratio);
        let scattered = Ray::new(hr.p, refracted);

        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}
