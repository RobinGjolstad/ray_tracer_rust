use std::f64::consts::PI;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use ray_tracer_rust::ray_tracer::{
    camera::Camera,
    colors::Color,
    lights::Light,
    materials::Material,
    shapes::*,
    transformations::Transform,
    tuples_new::{new_point, new_vector},
    world::World,
};

fn build_scene(x: usize, y: usize) -> (World, Camera) {
    let mut world_builder = World::builder();

    let floor = new_plane()
        .scale(1.0, 1.0, 1.0)
        .color(&Color::new(1.0, 0.75, 0.75))
        .specular(0.0)
        .reflective(0.25)
        .build();
    world_builder.object(floor);

    let cylinder = new_cylinder(Some((1.0, 0.0)))
        .scale(0.25, 1.0, 0.25)
        .color(&Color::new(0.545098, 0.270588, 0.07451))
        .reflective(0.0)
        .build();
    world_builder.object(cylinder);

    let cone = new_cone(Some((0.0, -1.0)))
        .translate(0.0, 3.0, 0.0)
        .scale(0.75, 2.0, 0.75)
        .color(&Color::new(0.133333, 0.545098, 0.133333))
        .build();
    world_builder.object(cone);

    world_builder.light(Light::point_light(
        &new_point(-10.0, 10.0, -10.0),
        &Color::new(1.0, 1.0, 1.0),
    ));

    let world = world_builder.build();

    let mut camera = Camera::new(x, y, PI / 3.0);
    camera.set_transform(Transform::view_transform(
        &new_point(0.0, 2.5, -5.0),
        &new_point(0.0, 1.0, 0.0),
        &new_vector(0.0, 2.0, 0.0),
    ));

    (world, camera)
}

type Scene = (World, Camera);

pub fn render_25(c: &mut Criterion) {
    let mut group = c.benchmark_group("Render 25 x 25");
    group.sample_size(500);

    #[allow(non_snake_case)]
    let SCENE: Scene = black_box(build_scene(25, 25));

    group.bench_function("0 Reflections", |b| {
        b.iter(|| {
            let _ = SCENE.1.render(black_box(&SCENE.0), black_box(0));
        })
    });

    group.bench_function("3 Reflections", |b| {
        b.iter(|| {
            let _ = SCENE.1.render(black_box(&SCENE.0), black_box(3));
        })
    });

    group.finish();
}

pub fn render_50(c: &mut Criterion) {
    let mut group = c.benchmark_group("Render 50 x 50");
    group.sample_size(500);

    #[allow(non_snake_case)]
    let SCENE: Scene = black_box(build_scene(50, 50));

    group.bench_function("0 Reflections", |b| {
        b.iter(|| {
            let _ = SCENE.1.render(black_box(&SCENE.0), black_box(0));
        })
    });

    group.bench_function("3 Reflections", |b| {
        b.iter(|| {
            let _ = SCENE.1.render(black_box(&SCENE.0), black_box(3));
        })
    });

    group.finish();
}

pub fn render_100(c: &mut Criterion) {
    let mut group = c.benchmark_group("Render 100 x 100");
    group.sample_size(500);

    #[allow(non_snake_case)]
    let SCENE: Scene = black_box(build_scene(100, 100));

    group.bench_function("0 Reflections", |b| {
        b.iter(|| {
            let _ = SCENE.1.render(black_box(&SCENE.0), black_box(0));
        })
    });

    group.bench_function("3 Reflections", |b| {
        b.iter(|| {
            let _ = SCENE.1.render(black_box(&SCENE.0), black_box(3));
        })
    });

    group.finish();
}

criterion_group!(benches, render_25, render_50, render_100);
criterion_main!(benches);
