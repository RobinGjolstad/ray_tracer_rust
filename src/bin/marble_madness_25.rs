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
use std::{fs, path::Path, time::Instant};

#[derive(Debug, Clone, clap::Parser)]
#[command(name="Marble Madness 25", 
          author="Robin Gjølstad", 
          version="1.0.0", 
          about="Render an image with a cube of spheres, 25 at each side.", 
          long_about = None)]
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

    /// Output directory for images
    #[arg(short, long, default_value_t = String::from("images/marble_madness_25"))]
    path: String,
}

fn main() {
    // Start measuring runtime
    let start = Instant::now();

    let args = Args::parse();
    dbg!(&args);

    let mut create_dir = false;
    if let Ok(status) = Path::new(&args.path).try_exists() {
        if !status {
            create_dir = true;
        }
    } else {
        create_dir = true;
    }

    if create_dir {
        fs::create_dir_all(&args.path).expect("Failed creating output directory.");
    }

    let mut world_builder = World::builder();

    let num_spheres = 25;
    let mut sphere = glass_sphere();

    let mut material = sphere.get_material();
    material.reflective = 0.9;

    println!("Creating objects.");
    for x in 0..num_spheres {
        for y in 0..num_spheres {
            for z in 0..num_spheres {
                material.color = Color::new(
                    x as f64 / num_spheres as f64,
                    y as f64 / num_spheres as f64,
                    z as f64 / num_spheres as f64,
                );
                sphere.set_material(&material);
                let mut s = sphere.clone();
                let mut trans = Transform::translate(
                    -(num_spheres as f64) / 2.0 + x as f64,
                    -(num_spheres as f64) / 2.0 + y as f64,
                    -(num_spheres as f64) / 2.0 + z as f64,
                ) * Transform::scaling(0.33, 0.33, 0.33);
                s.set_transform(trans.inverse());
                world_builder.object(s);
            }
        }
    }

    world_builder.light(Light::point_light(
        &new_point(
            num_spheres as f64 * 2.0,
            num_spheres as f64 * 2.0,
            -(num_spheres as f64),
        ),
        &Color::new(1.0, 1.0, 1.0),
    ));

    let world = world_builder.build();

    let mut camera = Camera::new(args.x_axis, args.y_axis, 60_f64.to_radians());
    camera.set_transform(
        *Transform::view_transform(
            &new_point(40.0, 30.0, -40.0),
            &new_point(0.0, -3.0, 0.0),
            &new_vector(0.0, 1.0, 0.0),
        )
        .inverse(),
    );

    let mut elapsed = start.elapsed();
    println!("Starting render: {:?}", elapsed);

    let thread_number = args.jobs;
    let mut img = camera.render_multithreaded_improved(&world, thread_number, args.reflect);

    elapsed = start.elapsed();
    println!("Saving render: {:?}", elapsed);

    let binding = chrono::Local::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, false);
    let current_time: Vec<&str> = binding.split('+').collect();
    let current_time = current_time.first().unwrap().to_owned().replace(":", "-");

    img.save(&format!(
        "{}/marble_madness_25_{}x{}_{}-threads_{}-reflect-{}.ppm",
        args.path,
        img.width(),
        img.height(),
        thread_number,
        args.reflect,
        current_time
    ));

    elapsed = start.elapsed();
    println!("Time elapsed: {:?}", elapsed);
}
