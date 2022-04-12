use crate::{hittable::HitRecord, ray::Ray, vec::Color};

mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Material: std::fmt::Debug + Send + Sync {
    /// Performs surface scattering from a material.
    ///
    /// When light traveling along `ray` reaches a surface made out of this
    /// material (intersection described by `hit`), some of it will be absorbed,
    /// and the rest will either be reflected or refracted. If 100% of the light
    /// is absorbed, `scatter` returns `None`; otherwise, it returns a new `Ray`
    /// giving the reflected/refracted direction of the light, and a `Vec3` with
    /// the amount of energy reflected/refracted in each of red, green, and
    /// blue.
    ///
    /// (In reality, light would be *both* reflected and refracted, but we
    /// choose one or the other randomly and use over-sampling to produce a
    /// blend.)
    fn scatter(&self, ray_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)>;
}
