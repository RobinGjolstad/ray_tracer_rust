use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ray_tracer_rust::ray_tracer::{
    intersections::{prepare_computations, schlick, Intersection, Intersections},
    rays::Ray,
    shapes::{glass_sphere, new_sphere},
    transformations::Transform,
    tuples_new::{new_point, new_vector},
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Intersections");
    group.sample_size(1000);

    group.bench_function(
        "the hit is always the lowest non-negative intersection",
        |b| {
            b.iter(|| {
                let s = new_sphere().build();
                let i1 = Intersection::new(5.0, &s);
                let i2 = Intersection::new(7.0, &s);
                let i3 = Intersection::new(-3.0, &s);
                let i4 = Intersection::new(2.0, &s);
                let xs = black_box(Intersections {
                    list: vec![i1, i2, i3, i4.clone()],
                });
                let _ = xs.hit();
            })
        },
    );

    group.bench_function("precomputing the state of an intersection", |b| {
        b.iter(|| {
            let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
            let shape = new_sphere().build();
            let i = Intersection::new(4.0, &shape);
            let _ = prepare_computations(
                black_box(&i),
                black_box(&r),
                black_box(&Intersections {
                    list: vec![i.clone()],
                }),
            );
        })
    });

    group.bench_function("the hit when an intersection occurs on the outside", |b| {
        b.iter(|| {
            let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
            let shape = new_sphere().build();
            let i = Intersection::new(4.0, &shape);
            let _ = prepare_computations(
                black_box(&i),
                black_box(&r),
                black_box(&Intersections {
                    list: vec![i.clone()],
                }),
            );
        })
    });

    group.bench_function("the hit when an intersection occurs on the inside", |b| {
        b.iter(|| {
            let r = Ray::new(new_point(0.0, 0.0, 0.0), new_vector(0.0, 0.0, 1.0));
            let shape = new_sphere().build();
            let i = Intersection::new(1.0, &shape);
            let _ = prepare_computations(
                black_box(&i),
                black_box(&r),
                black_box(&Intersections {
                    list: vec![i.clone()],
                }),
            );
        })
    });

    group.bench_function("finding n1 and n2 at various intersections", |b| {
        b.iter(|| {
            #[allow(non_snake_case)]
            let A = glass_sphere()
                .scale(2.0, 2.0, 2.0)
                .refractive_index(1.5)
                .build();

            #[allow(non_snake_case)]
            let B = glass_sphere()
                .translate(0.0, 0.0, -0.25)
                .refractive_index(2.0)
                .build();

            #[allow(non_snake_case)]
            let C = glass_sphere()
                .translate(0.0, 0.0, 0.25)
                .refractive_index(2.5)
                .build();

            let r = Ray::new(new_point(0.0, 0.0, -4.0), new_vector(0.0, 0.0, 1.0));
            let xs = Intersections {
                list: vec![
                    Intersection::new(2.0, &A),
                    Intersection::new(2.75, &B),
                    Intersection::new(3.25, &C),
                    Intersection::new(4.75, &B),
                    Intersection::new(5.25, &C),
                    Intersection::new(6.0, &A),
                ],
            };

            let results = [
                [1.0, 1.5],
                [1.5, 2.0],
                [2.0, 2.5],
                [2.5, 2.5],
                [2.5, 1.5],
                [1.5, 1.0],
            ];

            for (i, _) in results.iter().enumerate() {
                let _ = prepare_computations(black_box(&xs.list[i]), black_box(&r), black_box(&xs));
            }
        })
    });

    group.bench_function(
        "the schlick approximation with small angle and n2 greater than n1",
        |b| {
            b.iter(|| {
                let shape = glass_sphere().build();
                let r = Ray::new(new_point(0.0, 0.99, -2.0), new_vector(0.0, 0.0, 1.0));
                let xs = Intersections::new(&[Intersection::new(1.8589, &shape)]);
                let comps =
                    prepare_computations(black_box(&xs.list[0]), black_box(&r), black_box(&xs));
                let _ = schlick(black_box(&comps));
            })
        },
    );

    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
