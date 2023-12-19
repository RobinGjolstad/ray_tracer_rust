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

    /// Number of times light can reflect
    #[arg(short, long, default_value_t = 5)]
    reflect: usize,

    /// Number of spheres, cubed, to render
    #[arg(short, long, default_value_t = 3)]
    num: usize,
}

fn main() {
    // Start measuring runtime
    let start = Instant::now();

    let args = Args::parse();
    dbg!(args);

    let mut world = World::new();

    let mut sphere = Object::glass_sphere();

    let mut material = sphere.get_material();
    material.reflective = 0.9;

    println!("Adding spheres");
    for x in 0..args.num {
        for y in 0..args.num {
            for z in 0..args.num {
                material.color = Color::new(
                    x as f64 / args.num as f64,
                    y as f64 / args.num as f64,
                    z as f64 / args.num as f64,
                );
                sphere.set_material(&material);
                let mut s = sphere.clone();
                let trans = Transform::translate(
                    -(args.num as f64) / 2.0 + x as f64,
                    -(args.num as f64) / 2.0 + y as f64,
                    -(args.num as f64) / 2.0 + z as f64,
                ) * Transform::scaling(0.33, 0.33, 0.33);
                s.set_transform(&trans);
                world.objects.push(s);
            }
        }
    }

    world.lights.push(Light::point_light(
        &Tuple::new_point(
            args.num as f64 * 2.0,
            args.num as f64 * 2.0,
            -(args.num as f64),
        ),
        &Color::new(1.0, 1.0, 1.0),
    ));

    let mut camera = Camera::new(args.x_axis, args.y_axis, 60_f64.to_radians());
    camera.set_transform(Transform::view_transform(
        &Tuple::new_point(40.0, 30.0, -40.0),
        &Tuple::new_point(0.0, -3.0, 0.0),
        &Tuple::new_vector(0.0, 1.0, 0.0),
    ));

    let mut elapsed = start.elapsed();
    println!("Starting render: {:?}", elapsed);

    let thread_number = args.jobs;
    let mut img = camera.render_multithreaded_improved(&world, thread_number, args.reflect);

    elapsed = start.elapsed();
    println!("Saving render: {:?}", elapsed);
    img.save(&format!(
        "images/many_spheres/many_spheres_{}x{}_{}-threads_{}-reflect_{}-spheres.ppm",
        img.width(),
        img.height(),
        thread_number,
        args.reflect,
        args.num
    ));

    elapsed = start.elapsed();
    println!("Time elapsed: {:?}", elapsed);
}
