use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::{Color, Vec3};

/// A reflective material that looks like polished or frosted metal.
#[derive(Debug, PartialEq)]
pub struct Metal {
    /// The amount of light energy reflected in each color component, so `Color(1., 1., 1.)` is a
    /// white surface, and `Color(0., 0., 0.)` is totally black.
    albedo: Color,
    /// The amount of randomness introduced into reflected rays. A `fuzz` of 0 makes the surface
    /// look polished and mirror-smooth, while a `fuzz` of 1 produces a frosted, almost matte
    /// surface.
    fuzz: f64,
}

impl Metal {
    /// Create a new `Metal` material.
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
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
