use std::f64::consts::PI;

use ray_tracer::{
    canvas::Canvas,
    colors::Color,
    intersections::Intersections,
    rays::Ray,
    shapes::{sphere::Sphere, Object, Shapes},
    transformations::Transform,
    tuples::Tuple,
};

fn main() {
    println!("Hello");

    let sphere = Sphere::new();
    let mut squash = sphere;
    let transform = Transform::scaling(1.0, 0.5, 1.0)
        * Transform::scaling(0.5, 1.0, 1.0)
        * Transform::rotation_z(PI / 4.0)
        * Transform::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    squash.set_transform(&transform);
    let ray_origin = Tuple::new_point(0.0, 0.0, -5.0);
    let wall_z = 10.0_f64;
    let wall_size = 7.0_f64;
    let canvas_pixels = 100;
    let pixel_size = wall_size / canvas_pixels as f64;
    let half = wall_size / 2.0;
    let mut img = Canvas::new(canvas_pixels, canvas_pixels);
    let color = Color::new(1.0, 0.0, 0.0);

    for y in 0..canvas_pixels {
        // Calculate "world y coordinate"
        // Top = +half
        // Bottom = -half
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_pixels {
            // Calculate "world x coordinate"
            // Left = -half
            // Right = +half
            let world_x = -half + pixel_size * x as f64;

            // Describe the "wall" location the ray will target
            let position = Tuple::new_point(world_x, world_y, wall_z);
            let r = Ray::new(ray_origin, (position - ray_origin).normalize());
            let xs = Intersections::new(&r.intersect(&Object::Sphere(squash)));

            if let Some(_) = xs.hit() {
                img.write_pixel(x, y, color);
            } else {
            }
        }
    }

    img.save("ch5_pit_squash.ppm");
}
