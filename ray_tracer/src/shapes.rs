use dyn_clonable::*;
use std::fmt::Debug;

use crate::{
    materials::Material,
    matrices::Matrix,
    tuples::{Point, Vector},
};

pub mod sphere;
mod test_shape;

use self::{sphere::Sphere, test_shape::TestShape};

#[derive(Debug, PartialEq)]
pub enum ShapeType {
    Sphere,
    TestShape,
}

#[clonable]
pub trait Shapes: Debug + Clone {
    fn get_position(&self) -> Point;
    fn set_transform(&mut self, trans: &Matrix);
    fn get_transform(&self) -> Matrix;
    fn set_material(&mut self, material: &Material);
    fn get_material(&self) -> Material;
    fn normal(&self, point: Point) -> Vector;
    fn get_shape_type(&self) -> ShapeType;
}

#[derive(Debug, Clone)]
pub struct Object {
    object: Box<dyn Shapes>,
}

impl Object {
    pub fn new(obj: Box<dyn Shapes>) -> Object {
        Object { object: obj }
    }
    pub fn normal(&self, point: Point) -> Vector {
        self.object.normal(point)
    }
    pub fn get_transform(&self) -> Matrix {
        self.object.get_transform()
    }
    pub fn get_position(&self) -> Point {
        self.object.get_position()
    }
    pub fn get_material(&self) -> Material {
        self.object.get_material()
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        if self.object.get_shape_type() != other.object.get_shape_type() {
            return false;
        } else if self.object.get_position() != other.object.get_position() {
            return false;
        } else if self.object.get_transform() != other.object.get_transform() {
            return false;
        } else if self.object.get_material() != other.object.get_material() {
            return false;
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_default_transformation() {
        let s = TestShape::new();
        assert_eq!(s.get_transform(), Matrix::new_identity());
    }
}
