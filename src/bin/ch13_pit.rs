use clap::Parser;
use ray_tracer_rust::ray_tracer::{
    camera::Camera,
    colors::Color,
    lights::Light,
    shapes::*,
    transformations::Transform,
    tuples_new::{new_point, new_vector},
    world::World,
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

    let mut camera = Camera::new(args.x_axis, args.y_axis, PI / 3.0);
    camera.set_transform(Transform::view_transform(
        &new_point(0.0, 2.5, -5.0),
        &new_point(0.0, 1.0, 0.0),
        &new_vector(0.0, 2.0, 0.0),
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
