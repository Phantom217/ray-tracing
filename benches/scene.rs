use criterion::{criterion_group, Criterion};
use rand::prelude::*;

use ray_tracing::vec3::Vec3;
use ray_tracing::*;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("scene/10x10x4", |b| {
        const NX: usize = 10;
        const NY: usize = 10;
        const NS: usize = 4;

        let mut rng = rand::rngs::SmallRng::seed_from_u64(0xDEADBEEF);
        let world = random_scene(&mut rng);

        let look_from = Vec3(13., 2., 3.);
        let look_at = Vec3(0., 0., 0.);
        let dist_to_focus = 10.;
        let aperture = 0.1;

        let camera = Camera::look(
            look_from,
            look_at,
            Vec3(0., 1., 0.),
            20.,
            NX as f64 / NY as f64,
            aperture,
            dist_to_focus,
        );

        b.iter(|| cast(NX, NY, NS, &camera, &world, &mut rng));
    });
}

criterion_group!(benches, criterion_benchmark);
criterion::criterion_main!(benches);
