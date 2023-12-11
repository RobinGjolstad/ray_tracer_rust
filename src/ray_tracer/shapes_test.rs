#![allow(clippy::approx_constant)]
use crate::ray_tracer::{
    materials::Material,
    matrices::Matrix,
    tuples::{Point, Vector},
};
use std::fmt::Debug;

mod group;
use group::Group;

#[cfg(test)]
mod test_shape;
#[cfg(test)]
use test_shape::TestShape;

use super::{intersections::Intersection, rays::Ray};

pub(super) trait Shapes: Debug + Sync {
    fn set_position(&mut self, pos: &Point);
    fn get_position(&self) -> Point;
    fn set_transform(&mut self, transform: &Matrix);
    fn get_transform(&self) -> Matrix;
    fn set_material(&mut self, material: &Material);
    fn get_material(&self) -> Material;
    fn set_parent(&mut self, parent: &BaseShape);
    fn get_parent(&self) -> BaseShape;
    fn local_normal_at(&self, point: Point) -> Vector;
    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection>;
}

#[derive(Debug, Clone, Copy)]
pub struct BaseShape {
    position: Option<Point>,
    transform: Option<Matrix>,
    material: Option<Material>,
}

#[derive(Debug, Clone)]
pub enum Object {
    Group(Group),

    #[cfg(test)]
    TestShape(TestShape),
}
impl Object {
    fn world_point_to_local(&self, point: &Point) -> Point {
        let inverted = match self {
            Object::Group(g) => g.get_transform().get_inverted().unwrap(),

            #[cfg(test)]
            Object::TestShape(s) => s.get_transform().get_inverted().unwrap(),
        };

        inverted * *point
    }
    fn normal_at(&self, world_point: Point) -> Vector {
        let local_point = self.world_point_to_local(&world_point);
        let local_normal = match self {
            Object::Group(g) => g.local_normal_at(local_point),

            #[cfg(test)]
            Object::TestShape(s) => s.local_normal_at(local_point),
        };

        self.local_vector_to_world(&local_normal)
    }
    fn local_vector_to_world(&self, local_vector: &Vector) -> Vector {
        let inverted = match self {
            Object::Group(g) => g.get_transform().get_inverted().unwrap(),

            #[cfg(test)]
            Object::TestShape(s) => s.get_transform().get_inverted().unwrap(),
        };

        let mut world_vector = inverted.transpose().unwrap() * *local_vector;
        world_vector.w = 0.0;
        world_vector.normalize()
    }

    pub fn set_transform(&mut self, transform: &Matrix) {
        match self {
            Object::Group(g) => g.set_transform(transform),

            #[cfg(test)]
            Object::TestShape(s) => s.set_transform(transform),
        }
    }
    pub fn get_transform(&self) -> Matrix {
        match self {
            Object::Group(g) => g.get_transform(),

            #[cfg(test)]
            Object::TestShape(s) => s.get_transform(),
        }
    }
    pub fn set_material(&mut self, material: &Material) {
        match self {
            Object::Group(g) => g.set_material(material),

            #[cfg(test)]
            Object::TestShape(s) => s.set_material(material),
        }
    }
    pub fn get_material(&self) -> Material {
        match self {
            Object::Group(g) => g.get_material(),

            #[cfg(test)]
            Object::TestShape(s) => s.get_material(),
        }
    }
    fn set_parent(&mut self, parent: &BaseShape) {
        match self {
            Object::Group(g) => g.set_parent(parent),

            #[cfg(test)]
            Object::TestShape(s) => s.set_parent(parent),
        }
    }
}

pub fn new_group(group: Group) -> Object {
    Object::Group(group)
}
#[cfg(test)]
fn new_test_shape() -> Object {
    Object::TestShape(TestShape::new())
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;
    use crate::ray_tracer::{transformations::Transform, tuples::Tuple};

    #[test]
    fn the_default_transformation() {
        let s = new_test_shape();
        assert_eq!(s.get_transform(), Matrix::new_identity());
    }
    #[test]
    fn assigning_a_transformation() {
        let mut s = new_test_shape();
        s.set_transform(&Transform::translate(2.0, 3.0, 4.0));
        assert_eq!(s.get_transform(), Transform::translate(2.0, 3.0, 4.0));
    }
    #[test]
    fn the_default_material() {
        let s = new_test_shape();
        let m = s.get_material();
        assert_eq!(m, Material::new());
    }
    #[test]
    fn assigning_a_material() {
        let mut s = new_test_shape();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.set_material(&m);
        assert_eq!(s.get_material(), m);
    }
    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        todo!("Convert Ray to accept new objects")
    }
    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        todo!("Convert Ray to accept new objects")
    }
    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let mut s = new_test_shape();
        s.set_transform(&Transform::translate(0.0, 1.0, 0.0));
        let n = s.normal_at(Tuple::new_point(0.0, 1.70711, -0.70711));
        assert_eq!(n, Tuple::new_vector(0.0, 0.70711, -0.70711));
    }
    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let mut s = new_test_shape();
        let m = Transform::scaling(1.0, 0.5, 1.0) * Transform::rotation_z(PI / 5.0);
        s.set_transform(&m);
        let n = s.normal_at(Tuple::new_point(
            0.0,
            f64::sqrt(2.0) / 2.0,
            -f64::sqrt(2.0) / 2.0,
        ));
        assert_eq!(n, Tuple::new_vector(0.0, 0.97014, -0.24254));
    }
    #[test]
    fn a_helper_for_producing_a_sphere_with_a_glassy_material() {
        todo!("Implement spheres")
    }
}
