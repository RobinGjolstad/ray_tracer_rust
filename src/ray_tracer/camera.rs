use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

use crate::ray_tracer::{canvas::Canvas, colors::Color, matrices::Matrix, rays::Ray, world::World};

use super::tuples::new_point;

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
    #[must_use]
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Self {
        let half_view = f64::tan(field_of_view / 2.0);
        #[allow(clippy::cast_precision_loss)]
        let aspect: f64 = hsize as f64 / vsize as f64;

        let (half_width, half_height) = if aspect >= 1.0 {
            (half_view, half_view / aspect)
        } else {
            (half_view * aspect, half_view)
        };

        let mut transform = Matrix::new_identity();
        transform.calculate_inverse().unwrap();
        Self {
            hsize,
            vsize,
            field_of_view,
            transform,
            #[allow(clippy::cast_precision_loss)]
            pixel_size: (half_width * 2.0) / hsize as f64,
            half_height,
            half_width,
        }
    }

    pub fn set_transform(&mut self, transformation: Matrix) {
        self.transform = transformation;
        self.transform.calculate_inverse().unwrap();
    }

    #[must_use]
    #[allow(clippy::cast_precision_loss)]
    pub fn ray_for_pixel(&self, px: usize, py: usize) -> Ray {
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
        let pixel = self.transform.get_inverted().unwrap() * new_point(world_x, world_y, -1.0);
        let origin = self.transform.get_inverted().unwrap() * new_point(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    #[must_use]
    pub fn render(&self, w: &World, num_reflections: usize) -> Canvas {
        let mut image = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x, y);
                let color = w.color_at(&ray, num_reflections);
                image.write_pixel(x, y, color);
            }
        }

        image
    }

    #[must_use]
    pub fn render_multithreaded(
        &self,
        w: &World,
        thread_num: usize,
        num_reflections: usize,
    ) -> Canvas {
        let image = Arc::new(Mutex::new(Canvas::new(self.hsize, self.vsize)));

        let pixels_per_thread = if thread_num == 0 {
            self.vsize
        } else {
            self.vsize / thread_num
        };

        let (tx, rx) = mpsc::channel();
        thread::scope(|s| {
            let mut thread_handles = Vec::new();
            let mut pixels_not_allocated = self.vsize;
            let mut last_allocated_pixels = 0;

            for thread in 0..thread_num {
                let tx_clone = tx.clone();
                let start_pixels = thread * pixels_per_thread;
                let end_pixels = start_pixels + pixels_per_thread;
                last_allocated_pixels = end_pixels;
                pixels_not_allocated -= pixels_per_thread;
                let handle = s.spawn(move || {
                    for y in start_pixels..end_pixels {
                        for x in 0..self.hsize {
                            let ray = self.ray_for_pixel(x, y);
                            let color = w.color_at(&ray, num_reflections);
                            tx_clone.send((x, y, color)).unwrap();
                        }
                    }
                });
                thread_handles.push(handle);
            }

            // If there are more pixels left to start a thread for, create one for that.
            if pixels_not_allocated > 0 {
                let start_pixels = last_allocated_pixels;
                let end_pixels = last_allocated_pixels + pixels_not_allocated;
                let tx_clone = tx.clone();
                let handle = s.spawn(move || {
                    for y in start_pixels..end_pixels {
                        for x in 0..self.hsize {
                            let ray = self.ray_for_pixel(x, y);
                            let color = w.color_at(&ray, num_reflections);
                            tx_clone.send((x, y, color)).unwrap();
                        }
                    }
                });
                thread_handles.push(handle);
            }

            let thread_image = Arc::clone(&image);
            s.spawn(move || loop {
                if thread_handles.len() > thread_handles.iter().filter(|x| x.is_finished()).count()
                {
                    let values: Result<(usize, usize, Color), mpsc::TryRecvError> = rx.try_recv();
                    match values {
                        Ok((x, y, color)) => {
                            let mut internal_image = thread_image.lock().unwrap();
                            internal_image.write_pixel(x, y, color);
                        }
                        Err(mpsc::TryRecvError::Disconnected) => {
                            break;
                        }
                        Err(mpsc::TryRecvError::Empty) => {
                            continue;
                        }
                    }
                } else {
                    break;
                }
            });
        });

        let ret_img = image.lock().unwrap().clone();
        ret_img
    }

    #[must_use]
    pub fn render_multithreaded_improved(
        &self,
        w: &World,
        thread_num: usize,
        num_reflections: usize,
    ) -> Canvas {
        let image = Arc::new(Mutex::new(Canvas::new(self.hsize, self.vsize)));

        let (tx, rx) = mpsc::channel();
        thread::scope(|s| {
            // List of pixel rows that need to be rendered.
            // Each thread will take a row from this list and render it.
            // When a thread has taken the row, it should be removed from the list.
            let pixel_rows_to_render: Vec<usize> = (0..self.vsize).collect();
            let pixel_rows_to_render = Arc::new(Mutex::new(pixel_rows_to_render));

            let mut thread_handles = Vec::new();
            for _thread in 0..thread_num {
                let tx_clone = tx.clone();
                let pixel_rows = Arc::clone(&pixel_rows_to_render);
                let handle = s.spawn(move || loop {
                    //let thread_id = thread::current().id();
                    // While there are still pixel rows to render, render them.
                    // Otherwise, break out of the loop.
                    let mut pixel_rows_to_render = pixel_rows.lock().unwrap();
                    if pixel_rows_to_render.len() > 0 {
                        let row = pixel_rows_to_render.pop().unwrap();
                        drop(pixel_rows_to_render);
                        //println!("Thread {:?} - Rendering row: {row}", thread_id);
                        for x in 0..self.hsize {
                            let ray = self.ray_for_pixel(x, row);
                            let color = w.color_at(&ray, num_reflections);
                            tx_clone.send((x, row, color)).unwrap();
                        }
                        //println!("Thread {:?} - Finished rendering row: {row}", thread_id);
                    } else {
                        //println!("Thread {:?} terminating.", thread_id);
                        break;
                    }
                });
                thread_handles.push(handle);
            }

            let thread_image = Arc::clone(&image);
            s.spawn(move || loop {
                if thread_handles.len() > thread_handles.iter().filter(|x| x.is_finished()).count()
                {
                    let values: Result<(usize, usize, Color), mpsc::TryRecvError> = rx.try_recv();
                    match values {
                        Ok((x, y, color)) => {
                            let mut internal_image = thread_image.lock().unwrap();
                            internal_image.write_pixel(x, y, color);
                        }
                        Err(mpsc::TryRecvError::Disconnected) => {
                            break;
                        }
                        Err(mpsc::TryRecvError::Empty) => {
                            continue;
                        }
                    }
                } else {
                    break;
                }
            });
        });

        let ret_img = image.lock().unwrap().clone();
        ret_img
    }
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::ray_tracer::{
        canvas::Canvas, colors::Color, matrices::Matrix, transformations::Transform,
        tuples::new_vector, utils::is_float_equal, world::World,
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
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, new_point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, new_vector(0.0, 0.0, -1.0));
    }
    #[test]
    fn constructing_a_ray_through_a_corner_of_the_canvas() {
        let c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0, 0);
        assert_eq!(r.origin, new_point(0.0, 0.0, 0.0));
        assert_eq!(r.direction, new_vector(0.66519, 0.33259, -0.66851));
    }
    #[test]
    fn constructing_a_ray_when_the_camera_is_transformed() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.set_transform(Transform::rotation_y(PI / 4.0) * Transform::translate(0.0, -2.0, 5.0));
        let r = c.ray_for_pixel(100, 50);
        assert_eq!(r.origin, new_point(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            new_vector(f64::sqrt(2.0) / 2.0, 0.0, -f64::sqrt(2.0) / 2.0)
        );
    }
    #[test]
    fn rendering_a_world_with_a_camera() {
        let w = World::new_default_world();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = new_point(0.0, 0.0, -5.0);
        let to = new_point(0.0, 0.0, 0.0);
        let up = new_vector(0.0, 1.0, 0.0);
        c.set_transform(Transform::view_transform(&from, &to, &up));
        let image: Canvas = c.render(&w, 1);
        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
    #[test]
    fn rendering_a_world_with_a_camera_with_one_thread() {
        let w = World::new_default_world();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = new_point(0.0, 0.0, -5.0);
        let to = new_point(0.0, 0.0, 0.0);
        let up = new_vector(0.0, 1.0, 0.0);
        c.set_transform(Transform::view_transform(&from, &to, &up));
        let image: Canvas = c.render_multithreaded(&w, 1, 1);
        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
    #[test]
    fn rendering_a_world_with_a_camera_with_two_threads() {
        let w = World::new_default_world();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = new_point(0.0, 0.0, -5.0);
        let to = new_point(0.0, 0.0, 0.0);
        let up = new_vector(0.0, 1.0, 0.0);
        c.set_transform(Transform::view_transform(&from, &to, &up));
        let image: Canvas = c.render_multithreaded(&w, 2, 1);
        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
    #[test]
    fn rendering_a_world_with_a_camera_with_two_threads_improved() {
        let w = World::new_default_world();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = new_point(0.0, 0.0, -5.0);
        let to = new_point(0.0, 0.0, 0.0);
        let up = new_vector(0.0, 1.0, 0.0);
        c.set_transform(Transform::view_transform(&from, &to, &up));
        let image: Canvas = c.render_multithreaded_improved(&w, 2, 1);
        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
