use dyn_clonable::*;
use std::fmt::Debug;

use crate::{
    intersections::Intersection,
    materials::Material,
    matrices::Matrix,
    rays::Ray,
    tuples::{Point, Tuple, Vector},
};

use self::sphere::Sphere;

pub mod plane;
pub mod sphere;
mod test_shape;

#[derive(Debug, PartialEq)]
pub enum ShapeType {
    Sphere,
    TestShape,
    Plane,
}

#[clonable]
pub trait Shapes: Debug + Clone {
    fn set_position(&mut self, pos: &Point);
    fn get_position(&self) -> Point;
    fn set_transform(&mut self, trans: &Matrix);
    fn get_transform(&self) -> Matrix;
    fn set_material(&mut self, material: &Material);
    fn get_material(&self) -> Material;
    fn local_normal_at(&self, point: Point) -> Vector;
    fn get_shape_type(&self) -> ShapeType;
    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection>;
}

#[derive(Debug, Clone)]
pub struct Object {
    object: Box<dyn Shapes>,
}

impl Object {
    pub fn new(obj: Box<dyn Shapes>) -> Object {
        Object { object: obj }
    }
    pub fn new_sphere() -> Object {
        Object::new(Box::new(Sphere::new()))
    }
    pub fn normal_at(&self, point: Point) -> Vector {
        let local_point = self.object.get_transform().get_inverted().unwrap() * point;
        let local_normal = self.object.local_normal_at(local_point);
        let mut world_normal = self
            .object
            .get_transform()
            .get_inverted()
            .unwrap()
            .transpose()
            .unwrap()
            * local_normal;
        world_normal.w = 0.0;

        world_normal.normalize()
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
    pub fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
        self.object.local_intersect(local_ray)
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

    use std::f64::consts::PI;

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
        let s_obj = Object::new(Box::new(s));
        assert_eq!(s_obj.get_transform(), Matrix::new_identity());
    }
    #[test]
    fn assigning_a_transformation() {
        let s = TestShape::new();
        let mut s_obj = Object::new(Box::new(s));
        s_obj.set_transform(&Transform::translate(2.0, 3.0, 4.0));
        assert_eq!(s_obj.get_transform(), Transform::translate(2.0, 3.0, 4.0));
    }
    #[test]
    fn the_default_material() {
        let s = TestShape::new();
        let s_obj = Object::new(Box::new(s));
        let m = s_obj.get_material();
        assert_eq!(m, Material::new());
    }
    #[test]
    fn assigning_a_material() {
        let s = TestShape::new();
        let mut s_obj = Object::new(Box::new(s));
        let mut m = Material::new();
        m.ambient = 1.0;
        s_obj.set_material(&m);
        assert_eq!(s_obj.get_material(), m);
    }
    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut s = Object::new(Box::new(TestShape::new()));
        s.set_transform(&Transform::scaling(2.0, 2.0, 2.0));
        let _xs = r.intersect(&s);
        let saved_ray = TestShape::get_saved_ray().unwrap();
        assert_eq!(saved_ray.origin, Tuple::new_point(0.0, 0.0, -2.5));
        assert_eq!(saved_ray.direction, Tuple::new_vector(0.0, 0.0, 0.5));
    }
    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut s = Object::new(Box::new(TestShape::new()));
        s.set_transform(&Transform::translate(5.0, 0.0, 0.0));
        let _xs = r.intersect(&s);
        let saved_ray = TestShape::get_saved_ray().unwrap();
        assert_eq!(saved_ray.origin, Tuple::new_point(-5.0, 0.0, -5.0));
        assert_eq!(saved_ray.direction, Tuple::new_vector(0.0, 0.0, 1.0));
    }
    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let mut s = Object::new(Box::new(TestShape::new()));
        s.set_transform(&Transform::translate(0.0, 1.0, 0.0));
        let n = s.normal_at(Tuple::new_point(0.0, 1.70711, -0.70711));
        assert_eq!(n, Tuple::new_vector(0.0, 0.70711, -0.70711));
    }
    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let mut s = Object::new(Box::new(TestShape::new()));
        let m = Transform::scaling(1.0, 0.5, 1.0) * Transform::rotation_z(PI / 5.0);
        s.set_transform(&m);
        let n = s.normal_at(Tuple::new_point(
            0.0,
            f64::sqrt(2.0) / 2.0,
            -f64::sqrt(2.0) / 2.0,
        ));
        assert_eq!(n, Tuple::new_vector(0.0, 0.97014, -0.24254));
    }
}
