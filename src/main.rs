use rand::prelude::*;

use ray_tracing::camera::Camera;
use ray_tracing::vec3::Vec3;
use ray_tracing::*;

fn main() {
    // Image
    const NX: usize = 600;
    const NY: usize = 300;
    const NS: usize = 100;

    let mut rng = rand::rngs::SmallRng::seed_from_u64(0xDEADBEEF);

    // World
    // let world = random_scene(&mut rng);
    let world = simple_light();

    // Camera
    let look_from = Vec3(23., 2., 0.);
    let look_at = Vec3(0., 2., 0.);
    let dist_to_focus = 33.;
    let aperture = 0.1;

    let camera = Camera::look(
        look_from,
        look_at,
        Vec3(0., 1., 0.),
        20.,
        NX as f64 / NY as f64,
        aperture,
        dist_to_focus,
        0. ..1.,
    );

    eprintln!("World contains {} objects.", world.len());
    eprintln!(
        "Parallel casting {} x {} image using {}x oversampling.",
        NX, NY, NS
    );

    let image = par_cast(NX, NY, NS, &camera, &world);
    print_ppm(image);
}
