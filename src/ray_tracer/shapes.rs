//! # Shapes
//!
//! ## Groups and trees
//!
//! The tree structure used for grouping is inspired by the crate::utils::tree module.

#![allow(unused, clippy::approx_constant)]
use crate::ray_tracer::{
    intersections::Intersection,
    materials::Material,
    matrices::Matrix,
    rays::Ray,
    tuples::{Point, Vector},
};
use std::{
    fmt::{Debug, Display},
    ops::{Deref, DerefMut},
    sync::{Arc, Mutex, RwLock, Weak},
};

mod cylinder;
pub use cylinder::Cylinder;
mod cone;
pub use cone::Cone;
mod group;
use group::Group;
mod sphere;
pub use sphere::Sphere;
mod cube;
pub use cube::Cube;
mod plane;
pub use plane::Plane;

#[cfg(test)]
mod test_shape;
#[cfg(test)]
use test_shape::TestShape;

use self::group::GroupBuilder;

pub(super) trait Shapes: Debug + Default + Sync {
    fn set_position(&mut self, pos: &Point);
    fn get_position(&self) -> Point;
    fn set_transform(&mut self, transform: &Matrix);
    fn get_transform(&self) -> Matrix;
    fn set_material(&mut self, material: &Material);
    fn get_material(&self) -> Material;
    fn local_normal_at(&self, point: Point) -> Vector;
    fn local_intersect(&self, local_ray: Ray, intersection_list: &mut Vec<Intersection>);
}

#[derive(Debug, Clone, PartialEq)]
pub struct BaseShape {
    position: Point,
    transform: Matrix,
    material: Material,
}
impl BaseShape {
    pub fn new() -> Self {
        Self {
            position: Point::new_point(0.0, 0.0, 0.0),
            transform: Matrix::new_identity().calculate_inverse().unwrap(),
            material: Material::new(),
        }
    }
}
impl Default for BaseShape {
    fn default() -> Self {
        BaseShape::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Group(Group),
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
    Cone(Cone),

    #[cfg(test)]
    TestShape(TestShape),
}
impl Object {
    fn world_point_to_local(&self, point: &Point) -> Point {
        let inverted = match self {
            Object::Group(g) => g.get_transform().get_inverted().unwrap(),
            Object::Sphere(s) => s.get_transform().get_inverted().unwrap(),
            Object::Plane(p) => p.get_transform().get_inverted().unwrap(),
            Object::Cube(c) => c.get_transform().get_inverted().unwrap(),
            Object::Cylinder(c) => c.get_transform().get_inverted().unwrap(),
            Object::Cone(c) => c.get_transform().get_inverted().unwrap(),

            #[cfg(test)]
            Object::TestShape(s) => s.get_transform().get_inverted().unwrap(),
        };

        inverted * *point
    }
    pub(crate) fn normal_at(&self, world_point: Point) -> Vector {
        let local_point = self.world_to_object(&world_point);
        let local_normal = match self {
            Object::Group(g) => g.local_normal_at(local_point),
            Object::Sphere(s) => s.local_normal_at(local_point),
            Object::Plane(p) => p.local_normal_at(local_point),
            Object::Cube(c) => c.local_normal_at(local_point),
            Object::Cylinder(c) => c.local_normal_at(local_point),
            Object::Cone(c) => c.local_normal_at(local_point),

            #[cfg(test)]
            Object::TestShape(s) => s.local_normal_at(local_point),
        };

        self.normal_to_world(&local_normal)
    }
    fn local_vector_to_world(&self, local_vector: &Vector) -> Vector {
        let inverted = match self {
            Object::Group(g) => g.get_transform().get_inverted().unwrap(),
            Object::Sphere(s) => s.get_transform().get_inverted().unwrap(),
            Object::Plane(p) => p.get_transform().get_inverted().unwrap(),
            Object::Cube(c) => c.get_transform().get_inverted().unwrap(),
            Object::Cylinder(c) => c.get_transform().get_inverted().unwrap(),
            Object::Cone(c) => c.get_transform().get_inverted().unwrap(),

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
            Object::Sphere(s) => s.set_transform(transform),
            Object::Plane(p) => p.set_transform(transform),
            Object::Cube(c) => c.set_transform(transform),
            Object::Cylinder(c) => c.set_transform(transform),
            Object::Cone(c) => c.set_transform(transform),

            #[cfg(test)]
            Object::TestShape(s) => s.set_transform(transform),
        }
    }
    pub fn get_transform(&self) -> Matrix {
        match self {
            Object::Group(g) => g.get_transform(),
            Object::Sphere(s) => s.get_transform(),
            Object::Plane(p) => p.get_transform(),
            Object::Cube(c) => c.get_transform(),
            Object::Cylinder(c) => c.get_transform(),
            Object::Cone(c) => c.get_transform(),

            #[cfg(test)]
            Object::TestShape(s) => s.get_transform(),
        }
    }
    pub fn set_material(&mut self, material: &Material) {
        match self {
            Object::Group(g) => g.set_material(material),
            Object::Sphere(s) => s.set_material(material),
            Object::Plane(p) => p.set_material(material),
            Object::Cube(c) => c.set_material(material),
            Object::Cylinder(c) => c.set_material(material),
            Object::Cone(c) => c.set_material(material),

            #[cfg(test)]
            Object::TestShape(s) => s.set_material(material),
        }
    }
    pub fn get_material(&self) -> Material {
        match self {
            Object::Group(g) => g.get_material(),
            Object::Sphere(s) => s.get_material(),
            Object::Plane(p) => p.get_material(),
            Object::Cube(c) => c.get_material(),
            Object::Cylinder(c) => c.get_material(),
            Object::Cone(c) => c.get_material(),

            #[cfg(test)]
            Object::TestShape(s) => s.get_material(),
        }
    }
    pub(crate) fn local_intersect(
        &self,
        local_ray: Ray,
        intersection_list: &mut Vec<Intersection>,
    ) {
        match self {
            Object::Group(g) => g.local_intersect(local_ray, intersection_list),
            Object::Sphere(s) => s.local_intersect(local_ray, intersection_list),
            Object::Plane(p) => p.local_intersect(local_ray, intersection_list),
            Object::Cube(c) => c.local_intersect(local_ray, intersection_list),
            Object::Cylinder(c) => c.local_intersect(local_ray, intersection_list),
            Object::Cone(c) => c.local_intersect(local_ray, intersection_list),

            #[cfg(test)]
            Object::TestShape(s) => s.local_intersect(local_ray, intersection_list),
        }
    }

    pub(crate) fn world_to_object(&self, world_point: &Point) -> Point {
        // The shape's transform has already combined with its parent's transform
        // if it is part of a group.
        self.get_transform().get_inverted().unwrap() * *world_point
    }

    fn normal_to_world(&self, local_normal: &Vector) -> Vector {
        let mut normal = self
            .get_transform()
            .get_inverted()
            .unwrap()
            .transpose()
            .unwrap()
            * *local_normal;
        normal.w = 0.0;
        normal.normalize()
    }
}

pub fn new_sphere() -> Object {
    (Object::Sphere(Sphere::default()))
}
pub fn glass_sphere() -> Object {
    let mut s = Sphere::default();
    let mut material = s.get_material();
    material.transparency = 1.0;
    material.refractive_index = 1.5;
    s.set_material(&material);

    Object::Sphere(s)
}
pub fn new_plane() -> Object {
    Object::Plane(Plane::default())
}
pub fn new_cube() -> Object {
    Object::Cube(Cube::default())
}
pub fn new_cylinder(max_min: Option<(f64, f64)>) -> Object {
    let mut cyl = Cylinder::default();
    if let Some(max_min) = max_min {
        cyl.maximum = max_min.0;
        cyl.minimum = max_min.1;
        cyl.closed = true;
    }

    Object::Cylinder(cyl)
}
pub fn new_cone(max_min: Option<(f64, f64)>) -> Object {
    let mut cone = Cone::default();
    if let Some(max_min) = max_min {
        cone.maximum = max_min.0;
        cone.minimum = max_min.1;
        cone.closed = true;
    }

    (Object::Cone(cone))
}
pub fn new_group(children: Vec<Object>) -> Object {
    let mut gb = GroupBuilder::new();
    children
        .into_iter()
        .fold(gb, |gb, child| gb.add(child))
        .build()
}

#[cfg(test)]
fn new_test_shape() -> Object {
    Object::TestShape(TestShape::default())
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use super::*;
    use crate::ray_tracer::{transformations::Transform, tuples::Tuple, utils::is_float_equal};

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
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut s = new_test_shape();
        s.set_transform(&Transform::scaling(2.0, 2.0, 2.0));
        let mut _xs = Vec::new();
        r.intersect(&s, &mut _xs);
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
        let mut s = new_test_shape();
        s.set_transform(&Transform::translate(5.0, 0.0, 0.0));
        let mut _xs = Vec::new();
        r.intersect(&s, &mut _xs);
        let saved_ray = TestShape::get_saved_ray().unwrap();
        assert_eq!(saved_ray.origin, Tuple::new_point(-5.0, 0.0, -5.0));
        assert_eq!(saved_ray.direction, Tuple::new_vector(0.0, 0.0, 1.0));
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
        let s = glass_sphere();
        assert_eq!(
            s.get_transform().get_matrix(),
            Matrix::new_identity().get_matrix()
        );
        assert!(is_float_equal(&s.get_material().transparency, 1.0));
        assert!(is_float_equal(&s.get_material().refractive_index, 1.5));
    }
    #[test]
    fn converting_a_point_from_world_to_object_space() {
        let mut s = new_sphere();
        s.set_transform(&Transform::translate(5.0, 0.0, 0.0));
        let g2 = GroupBuilder::new()
            .add(s)
            .set_transform(Transform::scaling(2.0, 2.0, 2.0))
            .build();
        let g1 = GroupBuilder::new()
            .add(g2)
            .set_transform(Transform::rotation_y(PI / 2.0))
            .build();

        // Get the sphere from the groups
        let g1 = if let Object::Group(g) = g1 {
            g
        } else {
            panic!("Expected a group");
        };
        let g2 = if let Object::Group(g) = g1.get_children().unwrap()[0].clone() {
            g
        } else {
            panic!("Expected a group");
        };
        let s = g2.get_children().unwrap()[0].clone();

        let p = s.world_to_object(&Tuple::new_point(-2.0, 0.0, -10.0));

        assert_eq!(p, Tuple::new_point(0.0, 0.0, -1.0));
    }
    #[test]
    fn converting_a_normal_from_object_to_world_space() {
        let mut s = new_sphere();
        s.set_transform(&Transform::translate(5.0, 0.0, 0.0));
        let g2 = GroupBuilder::new()
            .set_transform(Transform::scaling(1.0, 2.0, 3.0))
            .add(s)
            .build();
        let g1 = GroupBuilder::new()
            .set_transform(Transform::rotation_y(PI / 2.0))
            .add(g2)
            .build();

        // Get the sphere from the groups
        let g1 = if let Object::Group(g) = g1 {
            g
        } else {
            panic!("Expected a group");
        };
        let g2 = if let Object::Group(g) = g1.get_children().unwrap()[0].clone() {
            g
        } else {
            panic!("Expected a group");
        };
        let s = g2.get_children().unwrap()[0].clone();

        let n = s.normal_to_world(&Vector::new_vector(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));

        assert_eq!(n, Vector::new_vector(0.2857, 0.4286, -0.8571));
    }
    #[test]
    fn finding_the_normal_on_a_child_object() {
        let mut s = new_sphere();
        s.set_transform(&Transform::translate(5.0, 0.0, 0.0));
        let g2 = GroupBuilder::new()
            .set_transform(Transform::scaling(1.0, 2.0, 3.0))
            .add(s)
            .build();
        let g1 = GroupBuilder::new()
            .set_transform(Transform::rotation_y(PI / 2.0))
            .add(g2)
            .build();

        // Get the sphere from the groups
        let g1 = if let Object::Group(g) = g1 {
            g
        } else {
            panic!("Expected a group");
        };
        let g2 = if let Object::Group(g) = g1.get_children().unwrap()[0].clone() {
            g
        } else {
            panic!("Expected a group");
        };
        let s = g2.get_children().unwrap()[0].clone();

        let p = Point::new_point(1.7321, 1.1547, -5.5774);
        let n = s.normal_at(p);

        assert_eq!(n, Vector::new_vector(0.2857, 0.4286, -0.8571));
    }
}
