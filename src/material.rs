use super::{hit::HitRecord, ray::Ray, vec::Color};

mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

pub trait Material: Send + Sync {
    fn scatter(&self, ray_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)>;
}
