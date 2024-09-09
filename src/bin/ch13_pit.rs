use clap::Parser;
use ray_tracer_rust::ray_tracer::{
    camera::Camera, colors::Color, lights::Light, materials::Material, shapes::*,
    transformations::Transform, tuples::Tuple, world::World,
};
use std::{f64::consts::PI, time::Instant};

#[derive(Debug, Clone, Copy, clap::Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Number of parallell jobs
    #[arg(short, long, default_value_t = 1)]
    jobs: usize,

    /// Horizontal number of pixels
    #[arg(short, long, default_value_t = 480)]
    x_axis: usize,

    /// Vertical number of pixels
    #[arg(short, long, default_value_t = 480)]
    y_axis: usize,

    /// Number of times light can reflect
    #[arg(short, long, default_value_t = 5)]
    reflect: usize,
}

fn main() {
    // Start measuring runtime
    let start = Instant::now();

    let args = Args::parse();
    dbg!(args);

    let mut world_builder = World::builder();

    let mut floor = new_plane();
    floor.set_transform(&Transform::scaling(1.0, 1.0, 1.0));
    let mut material = Material::new();
    material.color = Color::new(1.0, 0.75, 0.75);
    material.specular = 0.0;
    material.reflective = 0.25;
    floor.set_material(&material);
    world_builder.object(floor);

    let mut cylinder = new_cylinder(Some((1.0, 0.0)));
    let mut trans = Transform::scaling(0.25, 1.0, 0.25);
    cylinder.set_transform(&trans);
    material = Material::new();
    material.color = Color::new(0.545098, 0.270588, 0.07451);
    material.reflective = 0.0;
    cylinder.set_material(&material);
    world_builder.object(cylinder);

    let mut cone = new_cone(Some((0.0, -1.0)));
    trans = Transform::translate(0.0, 3.0, 0.0) * Transform::scaling(0.75, 2.0, 0.75);
    cone.set_transform(&trans);
    material = Material::new();
    material.color = Color::new(0.133333, 0.545098, 0.133333);
    cone.set_material(&material);
    world_builder.object(cone);

    world_builder.light(Light::point_light(
        &Tuple::new_point(-10.0, 10.0, -10.0),
        &Color::new(1.0, 1.0, 1.0),
    ));

    let world = world_builder.build();

    let mut camera = Camera::new(args.x_axis, args.y_axis, PI / 3.0);
    camera.set_transform(Transform::view_transform(
        &Tuple::new_point(0.0, 2.5, -5.0),
        &Tuple::new_point(0.0, 1.0, 0.0),
        &Tuple::new_vector(0.0, 2.0, 0.0),
    ));

    let mut elapsed = start.elapsed();
    println!("Starting render: {:?}", elapsed);

    let thread_number = args.jobs;
    let mut img = camera.render_multithreaded(&world, thread_number, args.reflect);

    elapsed = start.elapsed();
    println!("Saving render: {:?}", elapsed);
    img.save(&format!(
        "images/ch13_pit/ch13_pit_{}x{}_{}-threads_{}-reflect.ppm",
        img.width(),
        img.height(),
        thread_number,
        args.reflect
    ));

    elapsed = start.elapsed();
    println!("Time elapsed: {:?}", elapsed);
}
