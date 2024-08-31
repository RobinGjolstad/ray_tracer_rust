use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ray_tracer_rust::ray_tracer::{matrices::Matrix, tuples::Tuple};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Identity Matrix");
    group.sample_size(500);

    group.bench_function("id x id", |b| {
        b.iter(|| {
            let mat_a = Matrix::new_identity();
            let mat_b = Matrix::new_identity();
            let _ = black_box(mat_a * mat_b);
        })
    });

    group.bench_function("transpose", |b| {
        b.iter(|| {
            let mat = Matrix::new_identity();
            let _ = black_box(mat.transpose());
        })
    });

    group.bench_function("determinant", |b| {
        b.iter(|| {
            let mat = Matrix::new_identity();
            let _ = black_box(mat.determinant());
        })
    });

    group.bench_function("inverse", |b| {
        b.iter(|| {
            let mut mat = Matrix::new_identity();
            let _ = black_box(mat.calculate_inverse());
        })
    });

    group.bench_function("eq id", |b| {
        b.iter(|| {
            let mat_a = Matrix::new_identity();
            let mat_b = Matrix::new_identity();
            let _ = black_box(mat_a == mat_b);
        })
    });

    group.bench_function("mul tuple", |b| {
        b.iter(|| {
            let mat = Matrix::new_identity();
            let tup = Tuple::new(1.0, 2.0, 3.0, 4.0);
            let _ = black_box(mat * tup);
        })
    });

    group.bench_function("inverse get inverted", |b| {
        b.iter(|| {
            let mut mat = Matrix::new_identity();
            let _ = mat.calculate_inverse();
            let _ = black_box(mat.get_inverted());
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
