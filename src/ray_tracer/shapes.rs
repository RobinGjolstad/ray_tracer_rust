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
    transformations::Transform,
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
#[cfg(test)]
use crate::ray_tracer::intersections::IntersectionsSoA;

use self::group::GroupBuilder;

pub trait Shapes: Debug + Default + Sync {
    fn get_transform(&self) -> Matrix<4>;
    fn get_material(&self) -> Material;
    fn local_normal_at(&self, point: Point) -> Vector;
    fn local_intersect(
        &self,
        object: &Object,
        local_ray: Ray,
        intersection_list: &mut crate::ray_tracer::intersections::IntersectionsSoA,
    );
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationAxis {
    X,
    Y,
    Z,
}

trait Type {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeNotSpecified;
impl Type for TypeNotSpecified {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TypeSpecified;
impl Type for TypeSpecified {}

#[derive(Debug, Clone, Copy, PartialEq)]
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
    #[must_use]
    pub const fn from_cube(cube: Cube) -> ShapeBuilder<Cube, TypeSpecified> {
        ShapeBuilder {
            base: cube.base,
            shape: Some(cube),
            type_specified: PhantomData,
        }
    }
    #[must_use]
    pub const fn from_cylinder(cylinder: Cylinder) -> ShapeBuilder<Cylinder, TypeSpecified> {
        ShapeBuilder {
            base: cylinder.base,
            shape: Some(cylinder),
            type_specified: PhantomData,
        }
    }
    #[must_use]
    pub const fn from_cone(cone: Cone) -> ShapeBuilder<Cone, TypeSpecified> {
        ShapeBuilder {
            base: cone.base,
            shape: Some(cone),
            type_specified: PhantomData,
        }
    }
    #[must_use]
    pub const fn from_plane(plane: Plane) -> ShapeBuilder<Plane, TypeSpecified> {
        ShapeBuilder {
            base: plane.base,
            shape: Some(plane),
            type_specified: PhantomData,
        }
    }
    #[must_use]
    pub const fn from_sphere(sphere: Sphere) -> ShapeBuilder<Sphere, TypeSpecified> {
        ShapeBuilder {
            base: sphere.base,
            shape: Some(sphere),
            type_specified: PhantomData,
        }
    }

    #[cfg(test)]
    #[must_use]
    pub const fn from_test_shape(test_shape: TestShape) -> ShapeBuilder<TestShape, TypeSpecified> {
        ShapeBuilder {
            base: test_shape.base,
            shape: Some(test_shape),
            type_specified: PhantomData,
        }
    }
}

impl<S, T> ShapeBuilder<S, T> {
    #[must_use]
    pub fn cube(self) -> ShapeBuilder<Cube, TypeSpecified> {
        ShapeBuilder {
            base: BaseShape::default(),
            shape: Some(Cube::default()),
            type_specified: PhantomData,
        }
    }
    #[must_use]
    pub fn cylinder(self) -> ShapeBuilder<Cylinder, TypeSpecified> {
        ShapeBuilder {
            base: BaseShape::default(),
            shape: Some(Cylinder::default()),
            type_specified: PhantomData,
        }
    }
    #[must_use]
    pub fn cone(self) -> ShapeBuilder<Cone, TypeSpecified> {
        ShapeBuilder {
            base: BaseShape::default(),
            shape: Some(Cone::default()),
            type_specified: PhantomData,
        }
    }
    #[must_use]
    pub fn plane(self) -> ShapeBuilder<Plane, TypeSpecified> {
        ShapeBuilder {
            base: BaseShape::default(),
            shape: Some(Plane::default()),
            type_specified: PhantomData,
        }
    }
    #[must_use]
    pub fn sphere(self) -> ShapeBuilder<Sphere, TypeSpecified> {
        ShapeBuilder {
            base: BaseShape::default(),
            shape: Some(Sphere::default()),
            type_specified: PhantomData,
        }
    }
    #[must_use]
    pub fn group(self) -> GroupBuilder {
        GroupBuilder::default()
    }
    #[must_use]
    pub const fn set_transform(mut self, transformation: Matrix<4>) -> Self {
        self.base.transform = transformation;

        self
    }

    #[cfg(test)]
    #[must_use]
    pub fn test_shape(self) -> ShapeBuilder<TestShape, TypeSpecified> {
        ShapeBuilder {
            base: self.base,
            shape: Some(TestShape::new()),
            type_specified: PhantomData,
        }
    }
}

impl ShapeBuilder<Cube, TypeSpecified> {
    #[must_use]
    pub fn build(&self) -> Object {
        let mut shape = self.shape.clone().unwrap();
        shape.base = self.base;
        shape.base.transform.inverse();
        Object::Cube(shape)
    }
}

impl ShapeBuilder<Cylinder, TypeSpecified> {
    #[must_use]
    pub fn build(&self) -> Object {
        let mut shape = self.shape.clone().unwrap();
        shape.base = self.base;
        shape.base.transform.inverse();
        Object::Cylinder(shape)
    }

    #[must_use]
    pub const fn caps(mut self, max: f64, min: f64) -> Self {
        if let Some(shape) = self.shape.as_mut() {
            shape.maximum = max;
            shape.minimum = min;
            shape.closed = true;
        }

        self
    }
}

impl ShapeBuilder<Cone, TypeSpecified> {
    #[must_use]
    pub fn build(&self) -> Object {
        let mut shape = self.shape.clone().unwrap();
        shape.base = self.base;
        shape.base.transform.inverse();
        Object::Cone(shape)
    }

    #[must_use]
    pub const fn caps(mut self, max: f64, min: f64) -> Self {
        if let Some(shape) = self.shape.as_mut() {
            shape.maximum = max;
            shape.minimum = min;
            shape.closed = true;
        }

        self
    }
}

impl ShapeBuilder<Plane, TypeSpecified> {
    #[must_use]
    pub fn build(&self) -> Object {
        let mut shape = self.shape.clone().unwrap();
        shape.base = self.base;
        shape.base.transform.inverse();
        Object::Plane(shape)
    }
}

impl ShapeBuilder<Sphere, TypeSpecified> {
    #[must_use]
    pub fn build(&self) -> Object {
        let mut shape = self.shape.clone().unwrap();
        shape.base = self.base;
        shape.base.transform.inverse();
        Object::Sphere(shape)
    }
}

#[cfg(test)]
impl ShapeBuilder<TestShape, TypeSpecified> {
    #[must_use]
    pub fn build(&self) -> Object {
        let mut shape = self.shape.clone().unwrap();
        shape.base = self.base;
        shape.base.transform.inverse();
        Object::TestShape(shape)
    }
}

impl<S, T> ShapeBuilder<S, T> {
    #[must_use]
    pub fn translate(mut self, x: f64, y: f64, z: f64) -> Self {
        self.base.transform = self.base.transform * Transform::translate(x, y, z);

        self
    }
    #[must_use]
    pub fn scale(mut self, x: f64, y: f64, z: f64) -> Self {
        self.base.transform = self.base.transform * Transform::scaling(x, y, z);

        self
    }
    #[must_use]
    pub fn shear(mut self, x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Self {
        self.base.transform =
            self.base.transform * Transform::shearing(x_y, x_z, y_x, y_z, z_x, z_y);

        self
    }
    #[must_use]
    pub fn rotate(mut self, axis: &RotationAxis, angle: f64) -> Self {
        self.base.transform = match axis {
            RotationAxis::X => self.base.transform * Transform::rotation_x(angle),
            RotationAxis::Y => self.base.transform * Transform::rotation_y(angle),
            RotationAxis::Z => self.base.transform * Transform::rotation_z(angle),
        };

        self
    }
    #[must_use]
    pub const fn color(mut self, color: &Color) -> Self {
        self.base.material.color = *color;

        self
    }
    #[must_use]
    pub const fn ambient(mut self, ambient: f64) -> Self {
        self.base.material.ambient = ambient;

        self
    }
    #[must_use]
    pub const fn diffuse(mut self, diffuse: f64) -> Self {
        self.base.material.diffuse = diffuse;

        self
    }
    #[must_use]
    pub const fn specular(mut self, specular: f64) -> Self {
        self.base.material.specular = specular;

        self
    }
    #[must_use]
    pub const fn shininess(mut self, shininess: f64) -> Self {
        self.base.material.shininess = shininess;

        self
    }
    #[must_use]
    pub const fn pattern(mut self, pattern: Pattern) -> Self {
        self.base.material.pattern = Some(pattern);

        self
    }
    #[must_use]
    pub const fn reflective(mut self, reflective: f64) -> Self {
        self.base.material.reflective = reflective;

        self
    }
    #[must_use]
    pub const fn transparency(mut self, transparency: f64) -> Self {
        self.base.material.transparency = transparency;

        self
    }
    #[must_use]
    pub const fn refractive_index(mut self, refractive_index: f64) -> Self {
        self.base.material.refractive_index = refractive_index;

        self
    }
}

impl TryFrom<Object> for ShapeBuilder<Cube, TypeSpecified> {
    type Error = String;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        if let Object::Cube(cube) = value {
            Ok(ShapeBuilder::from_cube(cube))
        } else {
            Err("Object is not a Cube".to_string())
        }
    }
}
impl TryFrom<Object> for ShapeBuilder<Cylinder, TypeSpecified> {
    type Error = String;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        if let Object::Cylinder(cylinder) = value {
            Ok(ShapeBuilder::from_cylinder(cylinder))
        } else {
            Err("Object is not a Cylinder".to_string())
        }
    }
}
impl TryFrom<Object> for ShapeBuilder<Cone, TypeSpecified> {
    type Error = String;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        if let Object::Cone(cone) = value {
            Ok(ShapeBuilder::from_cone(cone))
        } else {
            Err("Object is not a Cone".to_string())
        }
    }
}
impl TryFrom<Object> for ShapeBuilder<Plane, TypeSpecified> {
    type Error = String;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        if let Object::Plane(plane) = value {
            Ok(ShapeBuilder::from_plane(plane))
        } else {
            Err("Object is not a Plane".to_string())
        }
    }
}
impl TryFrom<Object> for ShapeBuilder<Sphere, TypeSpecified> {
    type Error = String;

    fn try_from(value: Object) -> Result<Self, Self::Error> {
        if let Object::Sphere(sphere) = value {
            Ok(ShapeBuilder::from_sphere(sphere))
        } else {
            Err("Object is not a Sphere".to_string())
        }
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
        intersection_list: &mut crate::ray_tracer::intersections::IntersectionsSoA,
    ) {
        match self {
            Self::Cone(c) => c.local_intersect(self, local_ray, intersection_list),
            Self::Cube(c) => c.local_intersect(self, local_ray, intersection_list),
            Self::Cylinder(c) => c.local_intersect(self, local_ray, intersection_list),
            Self::Group(g) => g.local_intersect(self, local_ray, intersection_list),
            Self::Plane(p) => p.local_intersect(self, local_ray, intersection_list),
            Self::Sphere(s) => s.local_intersect(self, local_ray, intersection_list),

            #[cfg(test)]
            Self::TestShape(s) => s.local_intersect(self, local_ray, intersection_list),
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
pub fn new_sphere() -> ShapeBuilder<Sphere, TypeSpecified> {
    ShapeBuilder::new().sphere()
}
#[must_use]
pub fn glass_sphere() -> ShapeBuilder<Sphere, TypeSpecified> {
    ShapeBuilder::new()
        .sphere()
        .transparency(1.0)
        .refractive_index(1.5)
}
#[must_use]
pub fn new_plane() -> ShapeBuilder<Plane, TypeSpecified> {
    ShapeBuilder::new().plane()
}
#[must_use]
pub fn new_cube() -> ShapeBuilder<Cube, TypeSpecified> {
    ShapeBuilder::new().cube()
}
#[must_use]
pub fn new_cylinder(max_min: Option<(f64, f64)>) -> ShapeBuilder<Cylinder, TypeSpecified> {
    let mut shape = ShapeBuilder::new().cylinder();
    if let Some((max, min)) = max_min {
        shape = shape.caps(max, min);
    }

    shape
}
#[must_use]
pub fn new_cone(max_min: Option<(f64, f64)>) -> ShapeBuilder<Cone, TypeSpecified> {
    let mut shape = ShapeBuilder::new().cone();
    if let Some((max, min)) = max_min {
        shape = shape.caps(max, min);
    }

    shape
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
fn new_test_shape() -> ShapeBuilder<TestShape, TypeSpecified> {
    ShapeBuilder::new().test_shape()
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
        let s = new_test_shape().build();
        assert_eq!(s.get_transform(), *Matrix::<4>::identity().inverse());
    }
    #[test]
    fn assigning_a_transformation() {
        let mut s = new_test_shape().translate(2.0, 3.0, 4.0).build();

        assert_eq!(
            s.get_transform().matrix,
            Transform::translate(2.0, 3.0, 4.0).matrix
        );
    }
    #[test]
    fn the_default_material() {
        let s = new_test_shape().build();
        let m = s.get_material();
        assert_eq!(m, Material::new());
    }
    #[test]
    fn assigning_a_material() {
        let mut s = new_test_shape().ambient(1.0).build();

        let mut m = Material::new();
        m.ambient = 1.0;

        assert_eq!(s.get_material(), m);
    }
    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let mut s = new_test_shape().scale(2.0, 2.0, 2.0).build();
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let mut xs = IntersectionsSoA::default();
        r.intersect(&s, &mut xs);
        let saved_ray = TestShape::get_saved_ray().unwrap();
        assert_eq!(saved_ray.origin, new_point(0.0, 0.0, -2.5));
        assert_eq!(saved_ray.direction, new_vector(0.0, 0.0, 0.5));
    }
    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let mut s = new_test_shape().translate(5.0, 0.0, 0.0).build();
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let mut xs = IntersectionsSoA::default();
        r.intersect(&s, &mut xs);
        let saved_ray = TestShape::get_saved_ray().unwrap();
        assert_eq!(saved_ray.origin, new_point(-5.0, 0.0, -5.0));
        assert_eq!(saved_ray.direction, new_vector(0.0, 0.0, 1.0));
    }
    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let mut s = new_test_shape().translate(0.0, 1.0, 0.0).build();

        let n = s.normal_at(new_point(0.0, 1.70711, -0.70711));
        assert_eq!(n, new_vector(0.0, 0.70711, -0.70711));
    }
    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let mut s = new_test_shape()
            .scale(1.0, 0.5, 1.0)
            .rotate(&RotationAxis::Z, (PI / 5.0))
            .build();

        let n = s.normal_at(new_point(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0));
        assert_eq!(n, new_vector(0.0, 0.97014, -0.24254));
    }
    #[test]
    fn a_helper_for_producing_a_sphere_with_a_glassy_material() {
        let s = glass_sphere().build();

        assert_eq!(s.get_transform().matrix, Matrix::<4>::identity().matrix);
        assert!(is_float_equal(&s.get_material().transparency, 1.0));
        assert!(is_float_equal(&s.get_material().refractive_index, 1.5));
    }
    #[test]
    fn converting_a_point_from_world_to_object_space() {
        let mut s = new_sphere().translate(5.0, 0.0, 0.0).build();
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
        let mut s = new_sphere().translate(5.0, 0.0, 0.0).build();
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
        let mut s = new_sphere().translate(5.0, 0.0, 0.0).build();
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
