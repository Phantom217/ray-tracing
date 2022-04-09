mod camera;
mod hit;
mod material;
mod ray;
mod sphere;
mod vec;

use std::io::{stderr, Write};
use std::rc::Rc;

use rand::Rng;

use camera::Camera;
use hit::{Hit, World};
use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
use vec::{Color, Point3, Vec3};

/// Linearly blends white and blue depending on the height of the `y` coordinate _after_ scaling
/// the ray direction to unit length (`-1.0 < y < 1.0`).
fn ray_color(ray: &Ray, world: &World, depth: u64) -> Color {
    if let Some(hr) = world.hit(ray, 0.001, f64::INFINITY) {
        match hr.material.scatter(ray, &hr) {
            Some((attenuation, scattered)) if depth > 0 => {
                attenuation * ray_color(&scattered, world, depth - 1)
            }
            _ => Color::new(0.0, 0.0, 0.0),
        }
    } else {
        let unit_direction = ray.direction().normalized();
        let t = 0.5 * (unit_direction.y() + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u64 = 256;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 100;
    const MAX_DEPTH: u64 = 5;

    // World
    let r = (std::f64::consts::PI / 4.0).cos();
    let mut world = World::new();

    let mat_left = Rc::new(Lambertian::new(Color::new(0.0, 0.0, 1.0)));
    let mat_right = Rc::new(Lambertian::new(Color::new(1.0, 0.0, 0.0)));

    let sphere_left = Sphere::new(Point3::new(-r, 0.0, -1.0), r, mat_left);
    let sphere_right = Sphere::new(Point3::new(r, 0.0, -1.0), r, mat_right);

    world.push(Box::new(sphere_left));
    world.push(Box::new(sphere_right));

    // Camera
    let camera = Camera::new(90.0, ASPECT_RATIO);

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let mut rng = rand::thread_rng();
    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", j + 1);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let random_u: f64 = rng.gen();
                let random_v: f64 = rng.gen();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, MAX_DEPTH);
            }

            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }

    eprint!("\rDone.");
}
