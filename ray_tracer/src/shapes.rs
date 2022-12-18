use crate::{
    materials::Material,
    matrices::Matrix,
    tuples::{Point, Vector},
};

pub mod sphere;

use self::sphere::Sphere;

pub trait Shapes {
    fn get_position(&self) -> Point;
    fn set_transform(&mut self, trans: &Matrix);
    fn get_transform(&self) -> Matrix;
    fn set_material(&mut self, material: &Material);
    fn get_material(&self) -> Material;
    fn normal(&self, point: Point) -> Vector;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Object {
    Sphere(Sphere),
}
