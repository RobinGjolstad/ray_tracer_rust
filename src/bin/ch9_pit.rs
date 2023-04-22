use clap::Parser;
use ray_tracer_rust::ray_tracer::{
    camera::Camera, colors::Color, lights::Light, materials::Material, shapes::Object,
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
}

fn main() {
    // Start measuring runtime
    let start = Instant::now();

    let args = Args::parse();
    dbg!(args);

    let mut floor = Object::new_plane();
    floor.set_transform(&Transform::scaling(1.0, 1.0, 1.0));
    let mut material = Material::new();
    material.color = Color::new(1.0, 0.9, 0.9);
    material.specular = 0.0;
    floor.set_material(&material);

    let mut middle = Object::new_sphere();
    middle.set_transform(&Transform::translate(-0.5, 1.0, 0.5));
    material = Material::new();
    material.color = Color::new(0.1, 1.0, 0.5);
    material.diffuse = 0.7;
    material.specular = 0.3;
    middle.set_material(&material);

    let mut right = Object::new_sphere();
    let trans = Transform::translate(1.5, 0.5, -0.5) * Transform::scaling(0.5, 0.5, 0.5);
    right.set_transform(&trans);
    material = Material::new();
    material.color = Color::new(0.5, 1.0, 0.1);
    material.diffuse = 0.7;
    material.specular = 0.3;
    right.set_material(&material);

    let mut left = Object::new_sphere();
    let trans = Transform::translate(-1.5, 0.33, -0.75) * Transform::scaling(0.33, 0.33, 0.33);
    left.set_transform(&trans);
    material = Material::new();
    material.color = Color::new(1.0, 0.8, 0.1);
    material.diffuse = 0.7;
    material.specular = 0.3;
    left.set_material(&material);

    let mut world = World::new();
    world.lights.push(Light::point_light(
        &Tuple::new_point(-10.0, 10.0, -10.0),
        &Color::new(1.0, 1.0, 1.0),
    ));
    world.objects = vec![floor, left, middle, right];

    let mut camera = Camera::new(args.x_axis, args.y_axis, PI / 3.0);
    camera.set_transform(Transform::view_transform(
        &Tuple::new_point(0.0, 1.5, -5.0),
        &Tuple::new_point(0.0, 1.0, 0.0),
        &Tuple::new_vector(0.0, 1.0, 0.0),
    ));

    let mut elapsed = start.elapsed();
    println!("Starting render: {:?}", elapsed);

    /*
    let mut img = camera.render(&world);

    elapsed = start.elapsed();
    println!("Saving render: {:?}", elapsed);
    img.save(&String::from(format!(
        "images/ch9_pit/ch9_pit_{}x{}.ppm",
        img.width(),
        img.height()
    )));
    */

    let thread_number = args.jobs;
    let mut img = camera.render_multithreaded(&world, thread_number.clone());

    elapsed = start.elapsed();
    println!("Saving render: {:?}", elapsed);
    img.save(&String::from(format!(
        "images/ch9_pit/ch9_pit_{}x{}_{}-threads.ppm",
        img.width(),
        img.height(),
        thread_number
    )));

    elapsed = start.elapsed();
    println!("Time elapsed: {:?}", elapsed);
}
