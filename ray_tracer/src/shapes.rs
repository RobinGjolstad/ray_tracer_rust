use crate::{tuples::{Point, Vector}, matrices::Matrix, materials::Material};

pub mod sphere;

pub trait Shapes {
    fn get_position(&self) -> Point;
    fn set_transform(&mut self, trans: &Matrix);
    fn get_transform(&self) -> Matrix;
    fn set_material(&mut self, material: &Material);
    fn get_material(&self) -> Material;
    fn normal(&self, point: Point) -> Vector;
}
