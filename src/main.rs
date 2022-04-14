use std::ops::Range;
use std::time::Instant;

use rand::prelude::*;

use ray_tracing::camera::Camera;
use ray_tracing::object::Object;
use ray_tracing::vec3::Vec3;
use ray_tracing::*;

#[allow(unused)]
fn cornell_box_scene(nx: usize, ny: usize) -> (Vec<Object>, Camera, Range<f64>) {
    let look_from = Vec3(278., 278., -800.);
    let look_at = Vec3(278., 278., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.0;
    let exposure = 0. ..1.;

    let camera = Camera::look(
        look_from,
        look_at,
        Vec3(0., 1., 0.),
        40.,
        nx as f64 / ny as f64,
        aperture,
        dist_to_focus,
        exposure.clone(),
    );

    let world = cornell_box();

    (world, camera, exposure)
}

fn simple_light_scene(
    nx: usize,
    ny: usize,
    rng: &mut impl Rng,
) -> (Vec<Object>, Camera, Range<f64>) {
    let look_from = Vec3(278., 278., -800.);
    let look_at = Vec3(278., 278., 0.);
    let dist_to_focus = 10.;
    let aperture = 0.0;
    let exposure = 0. ..1.;

    let camera = Camera::look(
        look_from,
        look_at,
        Vec3(0., 1., 0.),
        40.,
        nx as f64 / ny as f64,
        aperture,
        dist_to_focus,
        exposure.clone(),
    );

    use ray_tracing::material::Material;

    let mut world = cornell_box();

    const SPHERES: usize = 1000;
    for _ in 0..SPHERES {
        world.push(Object::Sphere {
            center: 277. + 257. * rng.gen::<Vec3>(),
            radius: 20.,
            material: Material::Lambertian {
                albedo: ray_tracing::texture::constant(Vec3::from(0.3)),
            },
            motion: Vec3::default(),
        });
    }

    (world, camera, exposure)
}

const USE_BVH: bool = true;

fn main() {
    const NX: usize = 300;
    const NY: usize = 300;
    const NS: usize = 500;

    eprintln!(
        "Parallel casting {} x {} image using {}x oversampling.",
        NX, NY, NS
    );

    let mut rng = rand::rngs::SmallRng::seed_from_u64(0xDEADBEEF);

    // World
    let (world, camera, exposure) = simple_light_scene(NX, NY, &mut rng);

    let (image, time) = if USE_BVH {
        eprintln!("Generating bounding volume hierarchy.");
        let world = ray_tracing::bvh::from_scene(world, exposure, &mut rng);
        eprintln!("Done.");
        let start = Instant::now();
        (par_cast(NX, NY, NS, &camera, world), start.elapsed())
    } else {
        eprintln!("Testing every ray against every object.");
        let world: &[Object] = &world;
        let start = Instant::now();
        (par_cast(NX, NY, NS, &camera, world), start.elapsed())
    };

    eprintln!("Took {:?} wall time", time);

    print_ppm(image);
}
