use image::ImageBuffer;
use image::Rgb;
use image::RgbImage;

use crate::tuples::Point;
use crate::tuples::TupleTrait;
use crate::tuples::Vector;

pub mod tuples;
pub mod utils;

fn main() {
    let proj = Projectile {
        position: Point::new((0.0, 1.0, 0.0)),
        velocity: Vector::new((1.5, 1.2, 0.0)),
    };
    let env = Environment {
        gravity: Vector::new((0.0, -0.1, 0.0)),
        wind: Vector::new((-0.05, 0.0, 0.0)),
    };
    fire_cannon(env, proj)
}

struct Projectile {
    position: Point,
    velocity: Vector,
}
struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn tick(env: &Environment, proj: Projectile) -> Projectile {
    let position = proj.position + proj.velocity;
    let velocity = proj.velocity + env.gravity + env.wind;

    Projectile {
        position: position,
        velocity: velocity,
    }
}

fn fire_cannon(env: Environment, proj: Projectile) {
    let mut img = RgbImage::new(256, 256);

    let mut projectile = proj;
    let mut num_ticks = 0;

    while projectile.position.tuple.y > 0.0 {
        println!(
            "Current status: Position: {:?} - Velocity: {:?}",
            projectile.position, projectile.velocity
        );
        place_pixel(&mut img, &projectile);
        projectile = tick(&env, projectile);
        num_ticks += 1;
    }

    println!(
        "Projectile traversed {} ticks with a distance of {}",
        num_ticks, projectile.position.tuple.x
    );
    place_pixel(&mut img, &projectile);
    img.save("cannon_path.png").unwrap();
}

fn place_pixel(img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>, proj: &Projectile) {
    // Pixels start in upper left corner.

    // Clamp X to be within the image
    let mut x_pixel_pos: u32 = (proj.position.tuple.x * 10.0).round() as u32;
    if x_pixel_pos >= img.width() {
        x_pixel_pos = img.width() - 1;
    }

    // To start in the lower left corner, we must "reverse" the Y-position
    // Clamp y to be within the image
    let mut y_pixel_pos: u32 = img.height() - (proj.position.tuple.y * 10.0).round() as u32;
    if y_pixel_pos >= img.height() {
        y_pixel_pos = img.height() - 1;
    }

    println!("Placing pixel ({},{})", x_pixel_pos, y_pixel_pos);

    img.put_pixel(x_pixel_pos, y_pixel_pos, Rgb([255, 255, 255]));
}
