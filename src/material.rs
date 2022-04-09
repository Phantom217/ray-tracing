use super::hit::HitRecord;
use super::ray::Ray;
use super::vec::Color;

pub trait Scatter {
    fn scatter(&self, ray_in: &Ray, hr: &HitRecord) -> Option<(Color, Ray)>;
}
