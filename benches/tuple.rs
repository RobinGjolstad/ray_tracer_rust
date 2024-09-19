use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ray_tracer_rust::ray_tracer::tuples::{new_vector, Vector};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Tuple");
    group.sample_size(1000);

    group.bench_function("magnitude", |b| {
        b.iter(|| {
            let vec = new_vector(1.0, 2.0, 3.0);
            let _ = black_box(vec.magnitude());
        })
    });

    group.bench_function("normalize", |b| {
        b.iter(|| {
            let vec = new_vector(1.0, 2.0, 3.0);
            let _ = black_box(vec.normalize());
        })
    });

    group.bench_function("dot", |b| {
        b.iter(|| {
            let vec_a = new_vector(1.0, 2.0, 3.0);
            let vec_b = new_vector(2.0, 3.0, 4.0);
            let _ = black_box(Vector::dot(&vec_a, &vec_b));
        })
    });

    group.bench_function("cross", |b| {
        b.iter(|| {
            let vec_a = new_vector(1.0, 2.0, 3.0);
            let vec_b = new_vector(2.0, 3.0, 4.0);
            let _ = black_box(Vector::cross(&vec_a, &vec_b));
        })
    });

    group.bench_function("reflect", |b| {
        b.iter(|| {
            let vec = new_vector(1.0, -1.0, 0.0);
            let normal = new_vector(0.0, 1.0, 0.0);
            let _ = black_box(Vector::reflect(&vec, &normal));
        })
    });

    group.bench_function("add", |b| {
        b.iter(|| {
            let vec_a = new_vector(1.0, 2.0, 3.0);
            let vec_b = new_vector(2.0, 3.0, 4.0);
            let _ = black_box(vec_a + vec_b);
        })
    });

    group.bench_function("sub", |b| {
        b.iter(|| {
            let vec_a = new_vector(1.0, 2.0, 3.0);
            let vec_b = new_vector(2.0, 3.0, 4.0);
            let _ = black_box(vec_a - vec_b);
        })
    });

    group.bench_function("neg", |b| {
        b.iter(|| {
            let vec = new_vector(1.0, 2.0, 3.0);
            let _ = black_box(-vec);
        })
    });

    group.bench_function("mul", |b| {
        b.iter(|| {
            let vec = new_vector(1.0, 2.0, 3.0);
            let _ = black_box(vec * 2.0);
        })
    });

    group.bench_function("div", |b| {
        b.iter(|| {
            let vec = new_vector(1.0, 2.0, 3.0);
            let _ = black_box(vec / 2.0);
        })
    });

    group.bench_function("eq", |b| {
        b.iter(|| {
            let vec_a = new_vector(1.0, 2.0, 3.0);
            let vec_b = new_vector(1.0, 2.0, 3.0);
            let _ = black_box(vec_a == vec_b);
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
