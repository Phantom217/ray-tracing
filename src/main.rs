mod hit;
mod ray;
mod sphere;
mod vec;

use std::io::{stderr, Write};

use ray::Ray;
use vec::{Color, Point3, Vec3};

fn hit_sphere(center: Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().length().powi(2);
    let half_b = oc.dot(ray.direction());
    let c = oc.length().powi(2) - radius.powi(2);
    let discriminant = half_b.powi(2) - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - discriminant.sqrt()) / (2.0 * a)
    }
}

/// Linearly blends white and blue depending on the height of the `y` coordinate _after_ scaling
/// the ray direction to unit length (`-1.0 < y < 1.0`).
fn ray_color(ray: &Ray) -> Color {
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, ray);
    if t > 0.0 {
        let n = (ray.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalized();
        return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
    }

    let unit_direction = ray.direction().normalized();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((256 as f64) / ASPECT_RATIO) as u64;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j - 1);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&ray);

            println!("{}", pixel_color.format_color());
        }
    }

    eprintln!("\nDone.")
}
