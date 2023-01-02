use crate::{canvas::Canvas, matrices::Matrix, rays::Ray, tuples::Tuple, world::World};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    transform: Matrix,
    pixel_size: f64,
    half_width: f64,
    half_height: f64,
}

impl Camera {
    pub fn new(horizontal_size: usize, vertical_size: usize, field_of_view: f64) -> Self {
        let half_view = f64::tan(field_of_view / 2.0);
        let aspect: f64 = horizontal_size as f64 / vertical_size as f64;
        let mut half_width = 0.0;
        let mut half_height = 0.0;
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }
        Camera {
            hsize: horizontal_size,
            vsize: vertical_size,
            field_of_view: field_of_view,
            transform: Matrix::new_identity(),
            pixel_size: (half_width * 2.0) / horizontal_size as f64,
            half_height: half_height,
            half_width: half_width,
        }
    }

    pub fn set_transform(&mut self, transformation: Matrix) {
        self.transform = transformation;
    }

    pub fn ray_for_pixel(&mut self, px: usize, py: usize) -> Ray {
        // The offset from the edge of the canvas to the pixel's center
        let xoffset = (px as f64 + 0.5) * self.pixel_size; 
        let yoffset = (py as f64 + 0.5) * self.pixel_size;

        // The untransformed coordinates of the pixel in world space.
        // (Remember that the camera looks toward -z, so +x is to the *left*)
        let world_x = self.half_width - xoffset;
        let world_y = self.half_height - yoffset;

        // Using the camera matrix, transform the canvas point and the origin,
        // and then compute the ray's direction vector.
        // (remember that the canvas is at z=-1)
        let pixel =
            self.transform.get_inverted().unwrap() * Tuple::new_point(world_x, world_y, -1.0);
        let origin = self.transform.get_inverted().unwrap() * Tuple::new_point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&mut self, w: &World) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = w.color_at(&ray);
                image.write_pixel(x, y, color);
            }
        }

        image
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{
        canvas::Canvas, colors::Color, matrices::Matrix, transformations::Transform, tuples::Tuple,
        utils::is_float_equal, world::World,
    };

    use super::*;

    #[test]
    fn constructing_a_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;
        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
        assert!(is_float_equal(&c.field_of_view, PI / 2.0));
        assert_eq!(c.transform, Matrix::new_identity());
    }
    #[test]
    fn the_pixel_size_for_a_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);
        assert!(is_float_equal(&c.pixel_size, 0.01));
    }
    #[test]
    fn the_pixel_size_for_a_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);
        assert!(is_float_equal(&c.pixel_size, 0.01));
    }
    #[test]
    fn constructing_a_ray_through_the_center_of_the_canvas() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Tuple::new_point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::new_vector(0.0, 0.0, -1.0));
    }
    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin, Tuple::new_point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Tuple::new_vector(0.66519, 0.33259, -0.66851));
    }
    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform = Transform::rotation_y(PI / 4.0) * Transform::translate(0.0, -2.0, 5.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, Tuple::new_point(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            Tuple::new_vector(f64::sqrt(2.0) / 2.0, 0.0, -f64::sqrt(2.0) / 2.0)
        );
    }
    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = World::new_default_world();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = Tuple::new_point(0.0, 0.0, -5.0);
        let to = Tuple::new_point(0.0, 0.0, 0.0);
        let up = Tuple::new_vector(0.0, 1.0, 0.0);
        c.transform = Transform::view_transform(&from, &to, &up);
        let image: Canvas = c.render(&w);
        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
