use crate::tuples::Tuple as Tuple;
use crate::tuples::Point as Point;
use crate::tuples::Vector as Vector;

pub mod tuples;

fn main() {
    println!("Hello!");

    let proj = Projectile {
        position: tuples::point((0.0, 1.0, 0.0)),
        velocity: tuples::vector((1.0, 1.0, 0.0)),
    };
    let env = Environment {
        gravity: tuples::vector((0.0, -0.1, 0.0)),
        wind: tuples::vector((-0.01, 0.0, 0.0)),
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

    Projectile { position: position, velocity: velocity }
}

fn fire_cannon(env: Environment, proj: Projectile) {
    let mut projectile = proj;
    let mut num_ticks = 0;
    while projectile.position.tuple.y > 0.0 {
        println!("Current status: Position: {:?} - Velocity: {:?}", projectile.position, projectile.velocity);
        projectile = tick(&env, projectile);
        num_ticks += 1;
    }

    println!("Projectile traversed {} ticks with a distance of {}", num_ticks, projectile.position.tuple.x);
}