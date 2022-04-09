mod camera;
mod hit;
mod material;
mod ray;
mod sphere;
mod vec;

use std::io::{stderr, Write};
use std::sync::Arc;

use rand::Rng;
use rayon::iter::{IntoParallelIterator, ParallelIterator};

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

fn random_scene() -> World {
    let mut rng = rand::thread_rng();
    let mut world = World::new();

    let ground_mat = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

    world.push(Box::new(ground_sphere));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new(
                f64::from(a) + rng.gen_range(0.0..0.9),
                0.2,
                f64::from(b) + rng.gen_range(0.0..0.9),
            );

            if choose_mat < 0.8 {
                // Diffuse
                let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                let sphere_mat = Arc::new(Lambertian::new(albedo));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = Color::random(0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Arc::new(Metal::new(albedo, fuzz));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else {
                // Glass
                let sphere_mat = Arc::new(Dielectric::new(1.5));
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            }
        }
    }

    let mat1 = Arc::new(Dielectric::new(1.5));
    let mat2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    let mat3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));
    world.push(Box::new(sphere3));

    world
}

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: u64 = 1200;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 500;
    const MAX_DEPTH: u64 = 50;

    // World
    let world = random_scene();

    // Camera
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let aperture = 0.1;
    let focus_dist = 10.0;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        focus_dist,
    );

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", j + 1);
        stderr().flush().unwrap();

        let scanline: Vec<Color> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|i| {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _ in 0..SAMPLES_PER_PIXEL {
                    let mut rng = rand::thread_rng();
                    let random_u: f64 = rng.gen();
                    let random_v: f64 = rng.gen();

                    let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                    let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                    let ray = camera.get_ray(u, v);
                    pixel_color += ray_color(&ray, &world, MAX_DEPTH);
                }

                pixel_color
            })
            .collect();

        for pixel_color in scanline {
            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }

    eprint!("\rDone.");
}
