use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ray_tracer_rust::ray_tracer::{matrices_new::Matrix, tuples_new::Vector};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Identity Matrix");
    group.sample_size(500);

    group.bench_function("id x id", |b| {
        b.iter(|| {
            let mat_a = Matrix::<3>::identity();
            let mat_b = Matrix::<3>::identity();
            let _ = black_box(mat_a * mat_b);
        })
    });

    group.bench_function("transpose", |b| {
        b.iter(|| {
            let mat = Matrix::<3>::identity();
            let _ = black_box(mat.transpose());
        })
    });

    group.bench_function("determinant", |b| {
        b.iter(|| {
            let mat = Matrix::<3>::identity();
            let _ = black_box(mat.determinant());
        })
    });

    group.bench_function("inverse", |b| {
        b.iter(|| {
            let mut mat = Matrix::<3>::identity();
            let _ = black_box(mat.inverse());
        })
    });

    group.bench_function("eq id", |b| {
        b.iter(|| {
            let mat_a = Matrix::<3>::identity();
            let mat_b = Matrix::<3>::identity();
            let _ = black_box(mat_a == mat_b);
        })
    });

    group.bench_function("mul vector", |b| {
        b.iter(|| {
            let mat = Matrix::<3>::identity();
            let tup = Vector::new(1.0, 2.0, 3.0);
            let _ = black_box(mat * tup);
        })
    });

    group.bench_function("inverse get inverted", |b| {
        b.iter(|| {
            let mut mat = Matrix::<3>::identity();
            let _ = black_box(mat.inverse());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
