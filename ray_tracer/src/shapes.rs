use crate::{tuples::{Point, Tuple}, matrices::Matrix};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Objects {
    /// Sphere(x_scale, y_scale, z_scale)
    Sphere(f32, f32, f32),
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Object {
    pub position: Point,
    object: Objects,
    transform: Matrix,
}
impl Object {
    pub fn sphere() -> Self {
        Object {
            position: Tuple::new_point(0.0, 0.0, 0.0),
            object: Objects::Sphere(1.0, 1.0, 1.0),
            transform: Matrix::new_identity(),
        }
    }
    pub fn set_transform(&mut self, trans: Matrix) {
        self.transform = trans;
    }
    pub fn get_transform(&self) -> Matrix {
        self.transform
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere {
    pub position: Point,
    x_scale: f32,
    y_scale: f32,
    z_scale: f32,
}

impl Sphere {
    pub fn new() -> Self {
        Sphere {
            position: Tuple::new_point(0.0, 0.0, 0.0),
            x_scale: 1.0,
            y_scale: 1.0,
            z_scale: 1.0,
        }
    }
}
