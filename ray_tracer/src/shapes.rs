use dyn_clonable::*;
use std::fmt::Debug;

use crate::{
    materials::Material,
    matrices::Matrix,
    tuples::{Point, Tuple, Vector},
};

pub mod sphere;
mod test_shape;

#[derive(Debug, PartialEq)]
pub enum ShapeType {
    Sphere,
    TestShape,
}

#[clonable]
pub trait Shapes: Debug + Clone {
    fn set_position(&mut self, pos: &Point);
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
    pub fn set_transform(&mut self, trans: &Matrix) {
        self.object.set_transform(trans);
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
    pub fn set_material(&mut self, mat: &Material) {
        self.object.set_material(mat);
    }
    pub fn get_shape_type(&self) -> ShapeType {
        self.object.get_shape_type()
    }
    pub fn set_position(&mut self, pos: &Tuple) {
        self.object.set_position(pos);
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

    use crate::{
        shapes::{sphere::Sphere, test_shape::TestShape},
        transformations::Transform,
    };

    use super::*;

    #[test]
    fn different_object_types_are_different_even_with_same_parameters() {
        let trans =
            Transform::scaling(1.0, 2.0, 3.0) * Transform::rotation_x(180.0_f64.to_radians());
        let mut mat = Material::new();
        mat.ambient = 0.7;
        let pos = Tuple::new_point(1.0, 0.0, 1.0);

        let mut sphere = Object::new(Box::new(Sphere::new()));
        let mut testshape = Object::new(Box::new(TestShape::new()));
        sphere.set_material(&mat);
        testshape.set_material(&mat);
        sphere.set_transform(&trans);
        testshape.set_transform(&trans);
        sphere.set_position(&pos);
        testshape.set_position(&pos);

        assert_ne!(sphere, testshape);
    }
    #[test]
    fn the_default_transformation() {
        let s = TestShape::new();
        assert_eq!(s.get_transform(), Matrix::new_identity());
    }
}
