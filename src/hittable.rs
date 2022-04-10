use super::ray::Ray;

mod hitrecord;
pub use hitrecord::HitRecord;

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub type World = Vec<Box<dyn Hittable>>;

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut tmp_hr = None;
        let mut closest_so_far = t_max;

        for object in self {
            if let Some(hr) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = hr.t();
                tmp_hr = Some(hr);
            }
        }

        tmp_hr
    }
}
