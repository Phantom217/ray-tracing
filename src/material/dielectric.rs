use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec::Color;

/// A transparent refractive material like glass or water.
#[derive(Debug, PartialEq)]
pub struct Dielectric {
    /// [Refractive index][ref-idx] of the material, which determines how
    /// much light is bent when traveling into or out of an object.
    ///
    /// [ref-idx]: https://en.wikipedia.org/wiki/Refractive_index
    ref_idx: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Self {
        Self {
            ref_idx: index_of_refraction,
        }
    }

    /// [Schlick's approximation][schlick] for computing reflection vs. refraction at a material
    /// surface.
    ///
    /// [schlick]: https://en.wikipedia.org/wiki/Schlick%27s_approximation
    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // Use Schlick's approximation for reflectance
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)> {
        use rand::Rng;

        let refraction_ratio = if hr.front_face() {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = ray_in.direction().normalized();
        let cos_theta = (-1.0 * unit_direction).dot(hr.norm()).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let mut rng = rand::thread_rng();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let will_reflect = rng.gen::<f64>() < Self::reflectance(cos_theta, refraction_ratio);

        let direction = if cannot_refract || will_reflect {
            // must reflect
            unit_direction.reflect(hr.norm())
        } else {
            // can refract
            unit_direction.refract(hr.norm(), refraction_ratio)
        };

        let scattered = Ray::new(hr.p(), direction, ray_in.time());

        Some((Color::new(1.0, 1.0, 1.0), scattered))
    }
}
