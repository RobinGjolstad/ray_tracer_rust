use clap::Parser;
use ray_tracer_rust::ray_tracer::{
    camera::Camera, colors::Color, lights::Light, materials::Material, patterns::Pattern,
    shapes::Object, transformations::Transform, tuples::Tuple, world::World,
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

    let mut world = World::new();

    let mut floor = Object::new_plane();
    floor.set_transform(&Transform::scaling(1.0, 1.0, 1.0));
    let mut material = Material::new();
    material.color = Color::new(1.0, 0.9, 0.9);
    material.specular = 0.0;
    material.reflective = 0.25;
    let mut pattern = Pattern::checker(Color::new(0.75, 0.50, 0.0), Color::new(0.0, 0.0, 1.0));
    pattern.set_transform(
        Transform::rotation_y(f64::to_radians(33.0)) * Transform::scaling(0.5, 0.5, 0.5),
    );
    material.pattern = Some(pattern);
    floor.set_material(&material);
    world.objects.push(floor);

    let mut middle = Object::new_cube();
    let trans = Transform::translate(-0.5, 1.0, 0.5) * Transform::rotation_y(33_f64.to_radians());
    middle.set_transform(&trans);
    material = Material::new();
    material.color = Color::new(0.05, 0.05, 0.05);
    material.diffuse = 0.05;
    material.ambient = 0.05;
    material.specular = 1.0;
    material.shininess = 300.0;
    material.transparency = 1.0;
    material.refractive_index = 2.5;
    material.reflective = 1.0;
    middle.set_material(&material);
    world.objects.push(middle);

    world.lights.push(Light::point_light(
        &Tuple::new_point(-10.0, 10.0, -10.0),
        &Color::new(1.0, 1.0, 1.0),
    ));

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
        "images/ch12_pit/ch12_pit_{}x{}_{}-threads_{}-reflect.ppm",
        img.width(),
        img.height(),
        thread_number,
        args.reflect
    ));

    elapsed = start.elapsed();
    println!("Time elapsed: {:?}", elapsed);
}
