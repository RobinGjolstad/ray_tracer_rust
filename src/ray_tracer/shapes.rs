//! # Shapes
//!
//! ## Notes regarding restructuring shapes:
//!
//! Builder.
//! Gradually specify the shape.
//!
//! ```rust
//! let mut material = Material::new();
//! // Do things with material.
//!
//! let some_sphere = Shape::new() // or Shape::sphere() ?
//!     .sphere()
//!     .translate(x,y,z)
//!     .scale(x,y,z)
//!     .rotate(x,y,z)
//!     .translate(x,y,z)
//!     .material(&material) // Creates owned version internally.
//!     .build();
//! ```
//!
//! Options:
//! - `Shape::new().sphere()`
//! - `Shape::sphere()`
//! - `new_sphere()`
//! - `ShapeBuilder::new().sphere()`
//! - `ShapeBuilder::sphere()`
//!
//! All should chain.
//! Avoid having to `build`?
//! All methods take and return `&mut Self`?
//!
//! Shape enum contain reference or owned object?
//! Is it faster with dynamic dispatch or passing through an enum?
//! Implement two parallell versions?

#![allow(unused, clippy::approx_constant)]
use crate::ray_tracer::{
    colors::Color,
    intersections::Intersection,
    materials::Material,
    matrices_new::Matrix,
    patterns::Pattern,
    rays::Ray,
    tuples_new::{new_point, Point, Vector},
};
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
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

pub trait Shapes: Debug + Default + Sync {
    fn set_transform(&mut self, transform: &Matrix<4>);
    fn get_transform(&self) -> Matrix<4>;
    fn set_material(&mut self, material: &Material);
    fn get_material(&self) -> Material;
    fn local_normal_at(&self, point: Point) -> Vector;
    fn local_intersect(&self, local_ray: Ray, intersection_list: &mut Vec<Intersection>);
}

#[derive(Debug, Clone, PartialEq)]
pub struct BaseShape {
    transform: Matrix<4>,
    material: Material,
}
impl BaseShape {
    #[must_use]
    pub fn new() -> Self {
        let mut mat = Matrix::<4>::identity();
        mat.inverse();
        Self {
            transform: mat,
            material: Material::new(),
        }
    }
}
impl Default for BaseShape {
    fn default() -> Self {
        Self::new()
    }
}

pub enum RotationAxis {
    X,
    Y,
    Z,
}

trait Type {}
struct TypeNotSpecified;
impl Type for TypeNotSpecified {}
struct TypeSpecified;
impl Type for TypeSpecified {}

pub struct ShapeBuilder<S, T> {
    base: BaseShape,
    shape: Option<S>,
    type_specified: PhantomData<T>,
}

impl Default for ShapeBuilder<(), TypeNotSpecified> {
    fn default() -> Self {
        Self {
            base: BaseShape::default(),
            shape: None,
            type_specified: PhantomData,
        }
    }
}

impl ShapeBuilder<(), TypeNotSpecified> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            base: BaseShape::default(),
            shape: None,
            type_specified: PhantomData,
        }
    }
}

impl<S, T> ShapeBuilder<S, T> {
    #[must_use]
    pub fn cube(&mut self) -> ShapeBuilder<Cube, TypeSpecified> {
        todo!()
    }
    #[must_use]
    pub fn cylinder(&mut self) -> ShapeBuilder<Cylinder, TypeSpecified> {
        todo!()
    }
    #[must_use]
    pub fn cone(&mut self) -> ShapeBuilder<Cone, TypeSpecified> {
        todo!()
    }
    #[must_use]
    pub fn plane(&mut self) -> ShapeBuilder<Plane, TypeSpecified> {
        todo!()
    }
    #[must_use]
    pub fn sphere(&mut self) -> ShapeBuilder<Sphere, TypeSpecified> {
        todo!()
    }
    #[must_use]
    pub fn group(&mut self) -> GroupBuilder {
        todo!()
    }
    #[cfg(test)]
    pub fn test_shape(&mut self) -> ShapeBuilder<TestShape, TypeSpecified> {
        ShapeBuilder {
            base: self.base.clone(),
            shape: Some(TestShape::new()),
            type_specified: PhantomData,
        }
    }
}

#[cfg(test)]
impl ShapeBuilder<TestShape, TypeSpecified> {
    #[must_use]
    pub fn build(&self) -> Object {
        let mut shape = self.shape.clone().unwrap();
        shape.base = self.base.clone();
        Object::TestShape(shape)
    }
}

impl<S, T> ShapeBuilder<S, T> {
    pub fn translate(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
        self.base.transform = self.base.transform
            * Matrix::new([
                [1.0, 0.0, 0.0, x],
                [0.0, 1.0, 0.0, y],
                [0.0, 0.0, 1.0, z],
                [0.0, 0.0, 0.0, 1.0],
            ]);

        self
    }
    pub fn scale(&mut self, x: f64, y: f64, z: f64) -> &mut Self {
        todo!("Apply scaling.")
    }
    pub fn shear(
        &mut self,
        x_y: f64,
        x_z: f64,
        y_x: f64,
        y_z: f64,
        z_x: f64,
        z_y: f64,
    ) -> &mut Self {
        todo!()
    }
    pub fn rotate(&mut self, axis: &RotationAxis, angle: f64) -> &mut Self {
        todo!()
    }
    pub fn color(&mut self, color: &Color) -> &mut Self {
        todo!()
    }
    pub fn ambient(&mut self, ambient: f64) -> &mut Self {
        todo!()
    }
    pub fn diffuse(&mut self, diffuse: f64) -> &mut Self {
        todo!()
    }
    pub fn specular(&mut self, specular: f64) -> &mut Self {
        todo!()
    }
    pub fn shininess(&mut self, shininess: f64) -> &mut Self {
        todo!()
    }
    pub fn pattern(&mut self, pattern: Pattern) -> &mut Self {
        todo!()
    }
    pub fn reflective(&mut self, reflective: f64) -> &mut Self {
        todo!()
    }
    pub fn transparency(&mut self, transparency: f64) -> &mut Self {
        todo!()
    }
    pub fn refractive_index(&mut self, refractive_index: f64) -> &mut Self {
        todo!()
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
            Self::Group(g) => g.get_transform().inverse.expect(
                "Shape's transformation matrix should be inverted and transposed before use.",
            ),
            Self::Sphere(s) => s.get_transform().inverse.expect(
                "Shape's transformation matrix should be inverted and transposed before use.",
            ),
            Self::Plane(p) => p.get_transform().inverse.expect(
                "Shape's transformation matrix should be inverted and transposed before use.",
            ),
            Self::Cube(c) => c.get_transform().inverse.expect(
                "Shape's transformation matrix should be inverted and transposed before use.",
            ),
            Self::Cylinder(c) => c.get_transform().inverse.expect(
                "Shape's transformation matrix should be inverted and transposed before use.",
            ),
            Self::Cone(c) => c.get_transform().inverse.expect(
                "Shape's transformation matrix should be inverted and transposed before use.",
            ),

            #[cfg(test)]
            Self::TestShape(s) => s.get_transform().inverse.expect(
                "Shape's transformation matrix should be inverted and transposed before use.",
            ),
        };

        inverted * *point
    }
    pub(crate) fn normal_at(&self, world_point: Point) -> Vector {
        let local_point = self.world_to_object(&world_point);
        let local_normal = match self {
            Self::Group(g) => g.local_normal_at(local_point),
            Self::Sphere(s) => s.local_normal_at(local_point),
            Self::Plane(p) => p.local_normal_at(local_point),
            Self::Cube(c) => c.local_normal_at(local_point),
            Self::Cylinder(c) => c.local_normal_at(local_point),
            Self::Cone(c) => c.local_normal_at(local_point),

            #[cfg(test)]
            Self::TestShape(s) => s.local_normal_at(local_point),
        };

        self.normal_to_world(&local_normal)
    }
    fn local_vector_to_world(&self, local_vector: &Vector) -> Vector {
        let inverted_mat = match self {
            Self::Group(g) => g.get_transform().inverse.expect(
                "Shape's transformation matrix should be inverted and transposed before use.",
            ),
            Self::Sphere(s) => s.get_transform().inverse.expect(
                "Shape's transformation matrix should be inverted and transposed before use.",
            ),
            Self::Plane(p) => p.get_transform().inverse.expect(
                "Shape's transformation matrix should be inverted and transposed before use.",
            ),
            Self::Cube(c) => c.get_transform().inverse.expect(
                "Shape's transformation matrix should be inverted and transposed before use.",
            ),
            Self::Cylinder(c) => c.get_transform().inverse.expect(
                "Shape's transformation matrix should be inverted and transposed before use.",
            ),
            Self::Cone(c) => c.get_transform().inverse.expect(
                "Shape's transformation matrix should be inverted and transposed before use.",
            ),

            #[cfg(test)]
            Self::TestShape(s) => s.get_transform().inverse.expect(
                "Shape's transformation matrix should be inverted and transposed before use.",
            ),
        };

        let mut world_vector = inverted_mat.transpose() * *local_vector;
        world_vector.normalize()
    }

    pub fn set_transform(&mut self, transform: &Matrix<4>) {
        debug_assert!(
            transform.inverse.is_some() && transform.inverse_transpose.is_some(),
            "Transformation matrix should be inverted before assignment."
        );

        match self {
            Self::Group(g) => g.set_transform(transform),
            Self::Sphere(s) => s.set_transform(transform),
            Self::Plane(p) => p.set_transform(transform),
            Self::Cube(c) => c.set_transform(transform),
            Self::Cylinder(c) => c.set_transform(transform),
            Self::Cone(c) => c.set_transform(transform),

            #[cfg(test)]
            Self::TestShape(s) => s.set_transform(transform),
        }
    }
    #[must_use]
    pub fn get_transform(&self) -> Matrix<4> {
        match self {
            Self::Group(g) => g.get_transform(),
            Self::Sphere(s) => s.get_transform(),
            Self::Plane(p) => p.get_transform(),
            Self::Cube(c) => c.get_transform(),
            Self::Cylinder(c) => c.get_transform(),
            Self::Cone(c) => c.get_transform(),

            #[cfg(test)]
            Self::TestShape(s) => s.get_transform(),
        }
    }
    pub fn set_material(&mut self, material: &Material) {
        match self {
            Self::Group(g) => g.set_material(material),
            Self::Sphere(s) => s.set_material(material),
            Self::Plane(p) => p.set_material(material),
            Self::Cube(c) => c.set_material(material),
            Self::Cylinder(c) => c.set_material(material),
            Self::Cone(c) => c.set_material(material),

            #[cfg(test)]
            Self::TestShape(s) => s.set_material(material),
        }
    }
    #[must_use]
    pub fn get_material(&self) -> Material {
        match self {
            Self::Group(g) => g.get_material(),
            Self::Sphere(s) => s.get_material(),
            Self::Plane(p) => p.get_material(),
            Self::Cube(c) => c.get_material(),
            Self::Cylinder(c) => c.get_material(),
            Self::Cone(c) => c.get_material(),

            #[cfg(test)]
            Self::TestShape(s) => s.get_material(),
        }
    }
    pub(crate) fn local_intersect(
        &self,
        local_ray: Ray,
        intersection_list: &mut Vec<Intersection>,
    ) {
        match self {
            Self::Group(g) => g.local_intersect(local_ray, intersection_list),
            Self::Sphere(s) => s.local_intersect(local_ray, intersection_list),
            Self::Plane(p) => p.local_intersect(local_ray, intersection_list),
            Self::Cube(c) => c.local_intersect(local_ray, intersection_list),
            Self::Cylinder(c) => c.local_intersect(local_ray, intersection_list),
            Self::Cone(c) => c.local_intersect(local_ray, intersection_list),

            #[cfg(test)]
            Self::TestShape(s) => s.local_intersect(local_ray, intersection_list),
        }
    }

    pub(crate) fn world_to_object(&self, world_point: &Point) -> Point {
        // The shape's transform has already combined with its parent's transform
        // if it is part of a group.
        self.get_transform()
            .inverse
            .expect("Shape's transformation matrix should be inverted and transposed before use.")
            * *world_point
    }

    fn normal_to_world(&self, local_normal: &Vector) -> Vector {
        let mut normal =
            self.get_transform().inverse_transpose.expect(
                "Shape's transformation matrix should be inverted and transposed before use.",
            ) * *local_normal;
        normal.normalize()
    }
}

#[must_use]
pub fn new_sphere() -> Object {
    (Object::Sphere(Sphere::default()))
}
#[must_use]
pub fn glass_sphere() -> Object {
    let mut s = Sphere::default();
    let mut material = s.get_material();
    material.transparency = 1.0;
    material.refractive_index = 1.5;
    s.set_material(&material);

    Object::Sphere(s)
}
#[must_use]
pub fn new_plane() -> Object {
    Object::Plane(Plane::default())
}
#[must_use]
pub fn new_cube() -> Object {
    Object::Cube(Cube::default())
}
#[must_use]
pub fn new_cylinder(max_min: Option<(f64, f64)>) -> Object {
    let mut cyl = Cylinder::default();
    if let Some(max_min) = max_min {
        cyl.maximum = max_min.0;
        cyl.minimum = max_min.1;
        cyl.closed = true;
    }

    Object::Cylinder(cyl)
}
#[must_use]
pub fn new_cone(max_min: Option<(f64, f64)>) -> Object {
    let mut cone = Cone::default();
    if let Some(max_min) = max_min {
        cone.maximum = max_min.0;
        cone.minimum = max_min.1;
        cone.closed = true;
    }

    (Object::Cone(cone))
}
#[must_use]
pub fn new_group(children: Vec<Object>) -> Object {
    let mut gb = GroupBuilder::new();
    children
        .into_iter()
        .fold(gb, group::GroupBuilder::add)
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
    use crate::ray_tracer::{
        transformations::Transform, tuples_new::new_vector, utils::is_float_equal,
    };

    #[test]
    fn the_default_transformation() {
        let s = new_test_shape();
        assert_eq!(s.get_transform(), *Matrix::<4>::identity().inverse());
    }
    #[test]
    fn assigning_a_transformation() {
        let mut s = new_test_shape();
        let mut trans = Transform::translate(2.0, 3.0, 4.0);
        trans.inverse();
        s.set_transform(&trans);
        assert_eq!(
            s.get_transform().matrix,
            Transform::translate(2.0, 3.0, 4.0).matrix
        );
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
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let mut s = new_test_shape();
        let mut trans = Transform::scaling(2.0, 2.0, 2.0);
        trans.inverse();
        s.set_transform(&trans);
        let mut xs = Vec::new();
        r.intersect(&s, &mut xs);
        let saved_ray = TestShape::get_saved_ray().unwrap();
        assert_eq!(saved_ray.origin, new_point(0.0, 0.0, -2.5));
        assert_eq!(saved_ray.direction, new_vector(0.0, 0.0, 0.5));
    }
    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let mut s = new_test_shape();
        let mut trans = Transform::translate(5.0, 0.0, 0.0);
        trans.inverse();
        s.set_transform(&trans);
        let mut xs = Vec::new();
        r.intersect(&s, &mut xs);
        let saved_ray = TestShape::get_saved_ray().unwrap();
        assert_eq!(saved_ray.origin, new_point(-5.0, 0.0, -5.0));
        assert_eq!(saved_ray.direction, new_vector(0.0, 0.0, 1.0));
    }
    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let mut s = new_test_shape();
        let mut trans = Transform::translate(0.0, 1.0, 0.0);
        trans.inverse();
        s.set_transform(&trans);
        let n = s.normal_at(new_point(0.0, 1.70711, -0.70711));
        assert_eq!(n, new_vector(0.0, 0.70711, -0.70711));
    }
    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let mut s = new_test_shape();
        let mut m = Transform::scaling(1.0, 0.5, 1.0) * Transform::rotation_z(PI / 5.0);
        m.inverse();
        s.set_transform(&m);
        let n = s.normal_at(new_point(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0));
        assert_eq!(n, new_vector(0.0, 0.97014, -0.24254));
    }
    #[test]
    fn a_helper_for_producing_a_sphere_with_a_glassy_material() {
        let s = glass_sphere();
        assert_eq!(s.get_transform().matrix, Matrix::<4>::identity().matrix);
        assert!(is_float_equal(&s.get_material().transparency, 1.0));
        assert!(is_float_equal(&s.get_material().refractive_index, 1.5));
    }
    #[test]
    fn converting_a_point_from_world_to_object_space() {
        let mut s = new_sphere();
        s.set_transform(Transform::translate(5.0, 0.0, 0.0).inverse());
        let g2 = GroupBuilder::new()
            .add(s)
            .set_transform(Transform::scaling(2.0, 2.0, 2.0).inverse())
            .build();
        let g1 = GroupBuilder::new()
            .add(g2)
            .set_transform(Transform::rotation_y(PI / 2.0).inverse())
            .build();

        // Get the sphere from the groups
        let Object::Group(g1) = g1 else {
            panic!("Expected a group");
        };
        let Object::Group(g2) = g1.get_children().unwrap()[0].clone() else {
            panic!("Expected a group");
        };
        let s = g2.get_children().unwrap()[0].clone();

        let p = s.world_to_object(&new_point(-2.0, 0.0, -10.0));

        assert_eq!(p, new_point(0.0, 0.0, -1.0));
    }
    #[test]
    fn converting_a_normal_from_object_to_world_space() {
        let mut s = new_sphere();
        s.set_transform(Transform::translate(5.0, 0.0, 0.0).inverse());
        let g2 = GroupBuilder::new()
            .set_transform(Transform::scaling(1.0, 2.0, 3.0).inverse())
            .add(s)
            .build();
        let g1 = GroupBuilder::new()
            .set_transform(Transform::rotation_y(PI / 2.0).inverse())
            .add(g2)
            .build();

        // Get the sphere from the groups
        let Object::Group(g1) = g1 else {
            panic!("Expected a group");
        };
        let Object::Group(g2) = g1.get_children().unwrap()[0].clone() else {
            panic!("Expected a group");
        };
        let s = g2.get_children().unwrap()[0].clone();

        let n = s.normal_to_world(&new_vector(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));

        assert_eq!(n, new_vector(0.2857, 0.4286, -0.8571));
    }
    #[test]
    fn finding_the_normal_on_a_child_object() {
        let mut s = new_sphere();
        s.set_transform(Transform::translate(5.0, 0.0, 0.0).inverse());
        let g2 = GroupBuilder::new()
            .set_transform(Transform::scaling(1.0, 2.0, 3.0).inverse())
            .add(s)
            .build();
        let g1 = GroupBuilder::new()
            .set_transform(Transform::rotation_y(PI / 2.0).inverse())
            .add(g2)
            .build();

        // Get the sphere from the groups
        let Object::Group(g1) = g1 else {
            panic!("Expected a group");
        };
        let Object::Group(g2) = g1.get_children().unwrap()[0].clone() else {
            panic!("Expected a group");
        };
        let s = g2.get_children().unwrap()[0].clone();

        let p = new_point(1.7321, 1.1547, -5.5774);
        let n = s.normal_at(p);

        assert_eq!(n, new_vector(0.2857, 0.4286, -0.8571));
    }
    #[test]
    fn creating_a_shape_with_a_shape_builder() {
        let s = ShapeBuilder::default().test_shape().build();
        assert_eq!(s.get_transform(), *Matrix::<4>::identity().inverse());
    }
    #[test]
    fn assigning_a_transformation_with_a_shape_builder() {
        let mut s = ShapeBuilder::default()
            .test_shape()
            .translate(2.0, 3.0, 4.0)
            .build();

        assert_eq!(
            s.get_transform().matrix,
            Transform::translate(2.0, 3.0, 4.0).matrix
        );
    }
}
