use std::f32::consts::PI;

use ray_tracer::{canvas::Canvas, colors::Color, transformations::Transform, tuples::Tuple};

fn main() {
    let mut img = Canvas::new(256, 256);
    let p_center = Tuple::new_point(0.0, 0.0, 0.0);
    let  p_trans = Transform::translate(0.0, 40.0, 0.0) * p_center;
    let white = Color::new(1.0, 1.0, 1.0);

    for i in 0..12 {
        let p_indicator = Transform::rotation_z((PI / 6.0) * i as f32) * p_trans;
        place_pixel(&p_indicator, &mut img, &white);
    }
    img.save("ch4_pit.ppm");
}

fn place_pixel(pixel: &Tuple, img: &mut Canvas, color: &Color) {
    // Move from origin 0 to centre of image
    let x: usize = (pixel.x.round() + 128.0) as usize;
    let y: usize = (pixel.y.round() + 128.0) as usize;
    img.write_pixel(x, y, *color);
}
