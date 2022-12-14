use crate::{
    matrices::Matrix,
    tuples::{Point, Vector},
};

pub trait Shapes {
    fn get_position(&self) -> Point;
    fn set_transform(&mut self, trans: Matrix);
    fn get_transform(&self) -> Matrix;
    fn normal(&self, point: Point) -> Vector;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere {
    position: Point,
    transform: Matrix,
}
impl Sphere {
    pub fn new() -> Self {
        Sphere {
            position: Point::new_point(0.0, 0.0, 0.0),
            transform: Matrix::new_identity(),
        }
    }
}

impl Shapes for Sphere {
    fn get_position(&self) -> Point {
        self.position
    }
    fn get_transform(&self) -> Matrix {
        self.transform
    }
    fn set_transform(&mut self, trans: Matrix) {
        self.transform = trans
    }
    fn normal(&self, point: Point) -> Vector {
        todo!("SOON![TM]")
    }
}