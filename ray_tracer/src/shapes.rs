use dyn_clonable::*;
use std::fmt::Debug;

use crate::{
    intersections::Intersection,
    materials::Material,
    matrices::Matrix,
    rays::Ray,
    tuples::{Point, Tuple, Vector},
};

use self::{plane::Plane, sphere::Sphere};

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
    fn set_material(&mut self, material: &Material);
    fn set_transform(&mut self, trans: &Matrix);
    fn local_normal_at(&self, point: Point) -> Vector;
    fn get_shape_type(&self) -> ShapeType;
    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection>;
}

#[derive(Debug, Clone)]
pub struct Object {
    object: Box<dyn Shapes>,
    transform: Matrix,
    material: Material,
}

impl Object {
    pub fn new(obj: Box<dyn Shapes>) -> Object {
        Object {
            object: obj,
            transform: Matrix::new_identity(),
            material: Material::new(),
        }
    }
    pub fn new_sphere() -> Object {
        Object::new(Box::new(Sphere::new()))
    }
    pub fn new_plane() -> Object {
        Object::new(Box::new(Plane::new()))
    }
    fn world_point_to_local(&self, point: &Point) -> Point {
        self.get_transform().get_inverted().unwrap() * *point
    }
    fn local_vector_to_world(&self, vector: &Vector) -> Vector {
        let mut world_vector = self
            .get_transform()
            .get_inverted()
            .unwrap()
            .transpose()
            .unwrap()
            * *vector;
        world_vector.w = 0.0;

        world_vector.normalize()
    }
    pub fn normal_at(&self, point: Point) -> Vector {
        let local_point = self.world_point_to_local(&point);
        let local_normal = self.object.local_normal_at(local_point);
        self.local_vector_to_world(&local_normal)
    }
    pub fn set_transform(&mut self, trans: &Matrix) {
        self.transform = *trans;
        self.transform.calculate_inverse().unwrap();
        self.object.set_transform(&self.transform);
    }
    pub fn get_transform(&self) -> Matrix {
        self.transform
    }
    pub fn get_position(&self) -> Point {
        self.object.get_position()
    }
    pub fn get_material(&self) -> Material {
        self.material
    }
    pub fn set_material(&mut self, mat: &Material) {
        self.material = *mat;
        self.object.set_material(&self.material);
    }
    // Used in comparisons between objects
    #[allow(unused)]
    fn get_shape_type(&self) -> ShapeType {
        self.object.get_shape_type()
    }
    pub fn set_position(&mut self, pos: &Tuple) {
        self.object.set_position(pos);
    }
    pub fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
        let detected_intersections = self.object.local_intersect(local_ray);
        let mut return_intersections: Vec<Intersection> = Vec::new();
        for intersection in 0..detected_intersections.len() {
            return_intersections.push(Intersection::new(
                detected_intersections[intersection].get_time(),
                self.clone(),
            ));
        }

        return_intersections
    }
}
impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        if self.object.get_shape_type() != other.object.get_shape_type() {
            return false;
        } else if self.object.get_position() != other.object.get_position() {
            return false;
        } else if self.get_transform() != other.get_transform() {
            return false;
        } else if self.get_material() != other.get_material() {
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
