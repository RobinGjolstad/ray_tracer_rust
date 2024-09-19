use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ray_tracer_rust::ray_tracer::{
    intersections::{prepare_computations, schlick, Intersection, Intersections},
    rays::Ray,
    shapes::{glass_sphere, new_sphere},
    transformations::Transform,
    tuples::{new_point, new_vector},
};

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("Intersections");
    group.sample_size(1000);

    group.bench_function(
        "the hit is always the lowest non-negative intersection",
        |b| {
            b.iter(|| {
                let s = new_sphere();
                let i1 = Intersection::new(5.0, s.clone());
                let i2 = Intersection::new(7.0, s.clone());
                let i3 = Intersection::new(-3.0, s.clone());
                let i4 = Intersection::new(2.0, s);
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
            let shape = new_sphere();
            let i = Intersection::new(4.0, shape);
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
            let shape = new_sphere();
            let i = Intersection::new(4.0, shape);
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
            let shape = new_sphere();
            let i = Intersection::new(1.0, shape);
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
            let mut A = glass_sphere();
            A.set_transform(&Transform::scaling(2.0, 2.0, 2.0));
            let mut mat = A.get_material();
            mat.refractive_index = 1.5;
            A.set_material(&mat);

            #[allow(non_snake_case)]
            let mut B = glass_sphere();
            B.set_transform(&Transform::translate(0.0, 0.0, -0.25));
            mat = B.get_material();
            mat.refractive_index = 2.0;
            B.set_material(&mat);

            #[allow(non_snake_case)]
            let mut C = glass_sphere();
            C.set_transform(&Transform::translate(0.0, 0.0, 0.25));
            mat = C.get_material();
            mat.refractive_index = 2.5;
            C.set_material(&mat);

            let r = Ray::new(new_point(0.0, 0.0, -4.0), new_vector(0.0, 0.0, 1.0));
            let xs = Intersections {
                list: vec![
                    Intersection::new(2.0, A.clone()),
                    Intersection::new(2.75, B.clone()),
                    Intersection::new(3.25, C.clone()),
                    Intersection::new(4.75, B.clone()),
                    Intersection::new(5.25, C.clone()),
                    Intersection::new(6.0, A.clone()),
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
                let shape = glass_sphere();
                let r = Ray::new(new_point(0.0, 0.99, -2.0), new_vector(0.0, 0.0, 1.0));
                let xs = Intersections::new(&[Intersection::new(1.8589, shape)]);
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
