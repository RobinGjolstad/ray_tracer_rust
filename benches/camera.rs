use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ray_tracer_rust::ray_tracer::camera::Camera;

pub fn criterion_benchmark(c: &mut Criterion) {
    let camera = Camera::new(160, 120, std::f64::consts::PI / 3.0);
    let mut group = c.benchmark_group("Camera");
    group.sample_size(1000);

    group.bench_function("ray for pixel", |b| {
        b.iter(|| {
            let _ = camera.ray_for_pixel(black_box(100), black_box(50));
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
