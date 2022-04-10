use std::ops::Range;

use super::{
    ray::Ray,
    vec::{Point3, Vec3},
};

use rand::{distributions::Uniform, prelude::*};

pub struct Camera {
    origin: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point3,
    cu: Vec3,
    cv: Vec3,
    lens_radius: f64,
    /// Shutter open/close time
    time_range: Uniform<f64>,
}

impl Camera {
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time_range: Range<f64>,
    ) -> Self {
        assert!(!time_range.is_empty());

        // Vertical field-of-view in degrees
        let theta = std::f64::consts::PI / 180.0 * vfov;
        let viewport_height = 2.0 * (theta / 2.0).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let cw = (lookfrom - lookat).normalized();
        let cu = vup.cross(cw).normalized();
        let cv = cw.cross(cu);

        let horizontal = focus_dist * viewport_width * cu;
        let vertical = focus_dist * viewport_height * cv;

        let lower_left_corner = lookfrom - horizontal / 2.0 - vertical / 2.0 - focus_dist * cw;

        Self {
            origin: lookfrom,
            horizontal,
            vertical,
            lower_left_corner,
            cu,
            cv,
            lens_radius: aperture / 2.0,
            time_range: Uniform::from(time_range),
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.cu * rd.x() + self.cv * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
            thread_rng().sample(self.time_range),
        )
    }
}
