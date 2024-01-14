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

pub(super) trait Shapes: Debug + Default + Sync {
    fn set_position(&mut self, pos: &Point);
    fn get_position(&self) -> Point;
    fn set_transform(&mut self, transform: &Matrix);
    fn get_transform(&self) -> Matrix;
    fn set_material(&mut self, material: &Material);
    fn get_material(&self) -> Material;
    fn local_normal_at(&self, point: Point) -> Vector;
    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection>;
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
pub enum ObjectEnum {
    Group(Group),
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
    Cone(Cone),

    #[cfg(test)]
    TestShape(TestShape),
}
impl ObjectEnum {
    fn world_point_to_local(&self, point: &Point) -> Point {
        let inverted = match self {
            ObjectEnum::Group(g) => g.get_transform().get_inverted().unwrap(),
            ObjectEnum::Sphere(s) => s.get_transform().get_inverted().unwrap(),
            ObjectEnum::Plane(p) => p.get_transform().get_inverted().unwrap(),
            ObjectEnum::Cube(c) => c.get_transform().get_inverted().unwrap(),
            ObjectEnum::Cylinder(c) => c.get_transform().get_inverted().unwrap(),
            ObjectEnum::Cone(c) => c.get_transform().get_inverted().unwrap(),

            #[cfg(test)]
            ObjectEnum::TestShape(s) => s.get_transform().get_inverted().unwrap(),
        };

        inverted * *point
    }
    pub(crate) fn normal_at(&self, world_point: Point) -> Vector {
        let local_point = self.world_point_to_local(&world_point);
        let local_normal = match self {
            ObjectEnum::Group(g) => g.local_normal_at(local_point),
            ObjectEnum::Sphere(s) => s.local_normal_at(local_point),
            ObjectEnum::Plane(p) => p.local_normal_at(local_point),
            ObjectEnum::Cube(c) => c.local_normal_at(local_point),
            ObjectEnum::Cylinder(c) => c.local_normal_at(local_point),
            ObjectEnum::Cone(c) => c.local_normal_at(local_point),

            #[cfg(test)]
            ObjectEnum::TestShape(s) => s.local_normal_at(local_point),
        };

        self.local_vector_to_world(&local_normal)
    }
    fn local_vector_to_world(&self, local_vector: &Vector) -> Vector {
        let inverted = match self {
            ObjectEnum::Group(g) => g.get_transform().get_inverted().unwrap(),
            ObjectEnum::Sphere(s) => s.get_transform().get_inverted().unwrap(),
            ObjectEnum::Plane(p) => p.get_transform().get_inverted().unwrap(),
            ObjectEnum::Cube(c) => c.get_transform().get_inverted().unwrap(),
            ObjectEnum::Cylinder(c) => c.get_transform().get_inverted().unwrap(),
            ObjectEnum::Cone(c) => c.get_transform().get_inverted().unwrap(),

            #[cfg(test)]
            ObjectEnum::TestShape(s) => s.get_transform().get_inverted().unwrap(),
        };

        let mut world_vector = inverted.transpose().unwrap() * *local_vector;
        world_vector.w = 0.0;
        world_vector.normalize()
    }

    pub fn set_transform(&mut self, transform: &Matrix) {
        match self {
            ObjectEnum::Group(g) => g.set_transform(transform),
            ObjectEnum::Sphere(s) => s.set_transform(transform),
            ObjectEnum::Plane(p) => p.set_transform(transform),
            ObjectEnum::Cube(c) => c.set_transform(transform),
            ObjectEnum::Cylinder(c) => c.set_transform(transform),
            ObjectEnum::Cone(c) => c.set_transform(transform),

            #[cfg(test)]
            ObjectEnum::TestShape(s) => s.set_transform(transform),
        }
    }
    pub fn get_transform(&self) -> Matrix {
        match self {
            ObjectEnum::Group(g) => g.get_transform(),
            ObjectEnum::Sphere(s) => s.get_transform(),
            ObjectEnum::Plane(p) => p.get_transform(),
            ObjectEnum::Cube(c) => c.get_transform(),
            ObjectEnum::Cylinder(c) => c.get_transform(),
            ObjectEnum::Cone(c) => c.get_transform(),

            #[cfg(test)]
            ObjectEnum::TestShape(s) => s.get_transform(),
        }
    }
    pub fn set_material(&mut self, material: &Material) {
        match self {
            ObjectEnum::Group(g) => g.set_material(material),
            ObjectEnum::Sphere(s) => s.set_material(material),
            ObjectEnum::Plane(p) => p.set_material(material),
            ObjectEnum::Cube(c) => c.set_material(material),
            ObjectEnum::Cylinder(c) => c.set_material(material),
            ObjectEnum::Cone(c) => c.set_material(material),

            #[cfg(test)]
            ObjectEnum::TestShape(s) => s.set_material(material),
        }
    }
    pub fn get_material(&self) -> Material {
        match self {
            ObjectEnum::Group(g) => g.get_material(),
            ObjectEnum::Sphere(s) => s.get_material(),
            ObjectEnum::Plane(p) => p.get_material(),
            ObjectEnum::Cube(c) => c.get_material(),
            ObjectEnum::Cylinder(c) => c.get_material(),
            ObjectEnum::Cone(c) => c.get_material(),

            #[cfg(test)]
            ObjectEnum::TestShape(s) => s.get_material(),
        }
    }
    pub(crate) fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
        match self {
            ObjectEnum::Group(g) => g.local_intersect(local_ray),
            ObjectEnum::Sphere(s) => s.local_intersect(local_ray),
            ObjectEnum::Plane(p) => p.local_intersect(local_ray),
            ObjectEnum::Cube(c) => c.local_intersect(local_ray),
            ObjectEnum::Cylinder(c) => c.local_intersect(local_ray),
            ObjectEnum::Cone(c) => c.local_intersect(local_ray),

            #[cfg(test)]
            ObjectEnum::TestShape(s) => s.local_intersect(local_ray),
        }
    }
}

pub fn new_sphere() -> Object {
    Object::new(ObjectEnum::Sphere(Sphere::default()))
}
pub fn glass_sphere() -> Object {
    let mut s = Sphere::default();
    let mut material = s.get_material();
    material.transparency = 1.0;
    material.refractive_index = 1.5;
    s.set_material(&material);

    Object::new(ObjectEnum::Sphere(s))
}
pub fn new_plane() -> Object {
    Object::new(ObjectEnum::Plane(Plane::default()))
}
pub fn new_cube() -> Object {
    Object::new(ObjectEnum::Cube(Cube::default()))
}
pub fn new_cylinder(max_min: Option<(f64, f64)>) -> Object {
    let mut cyl = Cylinder::default();
    if let Some(max_min) = max_min {
        cyl.maximum = max_min.0;
        cyl.minimum = max_min.1;
        cyl.closed = true;
    }

    Object::new(ObjectEnum::Cylinder(cyl))
}
pub fn new_cone(max_min: Option<(f64, f64)>) -> Object {
    let mut cone = Cone::default();
    if let Some(max_min) = max_min {
        cone.maximum = max_min.0;
        cone.minimum = max_min.1;
        cone.closed = true;
    }

    Object::new(ObjectEnum::Cone(cone))
}
pub fn new_group() -> Object {
    let group = Group::default();
    Object::new(ObjectEnum::Group(group))
}

#[cfg(test)]
fn new_test_shape() -> Object {
    Object::new(ObjectEnum::TestShape(TestShape::default()))
}

// Shapes tree structure START

type ObjectDataRef = Arc<RwLock<ObjectData>>;
type WeakObjectDataRef = Weak<RwLock<ObjectData>>;
/// Parent relationship is one of non-ownership.
type Parent = RwLock<WeakObjectDataRef>; // not `RwLock<ObjectDataRef>` which would cause memory leak.
/// Children relationship is one of ownership.
type Children = RwLock<Vec<Child>>;
type Child = ObjectDataRef;

// ====== ObjectNode ======

#[derive(Debug)]
pub struct ObjectData {
    value: ObjectEnum,
    parent: Parent,
    children: Children,
}
impl ObjectData {
    fn new(value: ObjectEnum) -> Self {
        ObjectData {
            value,
            parent: RwLock::new(Weak::new()),
            children: RwLock::new(Vec::new()),
        }
    }

    fn world_point_to_local(&self, world_point: &Point) -> Point {
        self.value.world_point_to_local(world_point)
    }

    fn normal_at(&self, world_point: &Point) -> Vector {
        self.value.normal_at(*world_point)
    }

    fn local_vector_to_world(&self, local_vector: &Vector) -> Vector {
        self.value.local_vector_to_world(local_vector)
    }

    fn set_transform(&mut self, transform: &Matrix) {
        self.value.set_transform(transform);
    }

    fn get_transform(&self) -> Matrix {
        self.value.get_transform()
    }

    fn set_material(&mut self, material: &Material) {
        self.value.set_material(material);
    }

    fn get_material(&self) -> Material {
        self.value.get_material()
    }

    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
        self.value.local_intersect(local_ray)
    }
}
impl Deref for ObjectData {
    type Target = ObjectEnum;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl DerefMut for ObjectData {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.value
    }
}
impl PartialEq for ObjectData {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl Default for ObjectData {
    fn default() -> Self {
        ObjectData {
            value: ObjectEnum::Group(Group::default()),
            parent: RwLock::new(Weak::new()),
            children: RwLock::new(Vec::new()),
        }
    }
}

// ====== Object ======

#[derive(Clone, Debug)]
pub struct Object {
    arc_ref: ObjectDataRef,
}
impl Object {
    pub fn new(value: ObjectEnum) -> Object {
        let new_node = ObjectData {
            value,
            parent: RwLock::new(Weak::new()),
            children: RwLock::new(Vec::new()),
        };
        //let arc_ref = Arc::new(RwLock::new(new_node));
        let arc_ref = Arc::new(RwLock::new(new_node));
        //let arc_ref = Arc::new(new_node);
        Object { arc_ref }
    }

    pub fn world_point_to_local(&self, world_point: &Point) -> Point {
        self.arc_ref
            .read()
            .unwrap()
            .world_point_to_local(world_point)
    }

    pub fn normal_at(&self, world_point: &Point) -> Vector {
        self.arc_ref.read().unwrap().normal_at(world_point)
    }

    pub fn local_vector_to_world(&self, local_vector: &Vector) -> Vector {
        self.arc_ref
            .read()
            .unwrap()
            .local_vector_to_world(local_vector)
    }

    pub fn set_transform(&mut self, transform: &Matrix) {
        self.arc_ref.write().unwrap().set_transform(transform);
    }

    pub fn get_transform(&self) -> Matrix {
        self.arc_ref.read().unwrap().get_transform()
    }

    pub fn set_material(&mut self, material: &Material) {
        self.arc_ref.write().unwrap().set_material(material);
    }

    pub fn get_material(&self) -> Material {
        self.arc_ref.read().unwrap().get_material()
    }

    pub(crate) fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
        self.arc_ref.read().unwrap().local_intersect(local_ray)
    }

    pub(crate) fn get_copy_of_internal_arc(&self) -> ObjectDataRef {
        self.arc_ref.clone()
    }

    /// Create a new child and add it to the children list.
    /// Return a reference to the new child.
    ///
    /// NOTE: This method is only available for `Group` objects.
    pub fn create_and_add_child(&self, value: ObjectEnum) -> ObjectDataRef {
        if let ObjectEnum::Group(_) = self.arc_ref.read().unwrap().value {
            let new_child = Object::new(value);
            self.add_child_and_update_its_parent(&new_child);
            new_child.get_copy_of_internal_arc()
        } else {
            panic!("This method is only available for `Group` objects.");
        }
    }

    /// Add an existing child to the children list.
    ///
    /// NOTE: This method is only available for `Group` objects.
    pub fn add_child_and_update_its_parent(&self, child: &Object) {
        if let ObjectEnum::Group(_) = self.arc_ref.read().unwrap().value {
        } else {
            panic!("This method is only available for `Group` objects.");
        }

        {
            let mut own_ref = self.arc_ref.write().unwrap();
            let mut my_children = own_ref.children.write().unwrap();
            my_children.push(child.get_copy_of_internal_arc());
        } // `my_children` guard dropped.

        {
            let mut child_ref = child.arc_ref.write().unwrap();
            let mut childs_parent = child_ref.parent.write().unwrap();
            *childs_parent = Arc::downgrade(&self.get_copy_of_internal_arc());
        } // `my_parent` guard dropped.
    }

    pub fn has_parent(&self) -> bool {
        self.get_parent().is_some()
    }

    pub(crate) fn get_parent(&self) -> Option<ObjectDataRef> {
        let own_ref = self.arc_ref.read().unwrap();
        let my_parent = own_ref.parent.read().unwrap();
        my_parent.upgrade()
    }
}
impl Default for Object {
    fn default() -> Self {
        Object::new(ObjectEnum::Group(Group::default()))
    }
}
impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        let own_ref = self.arc_ref.read().unwrap();
        let other_ref = other.arc_ref.read().unwrap();

        let same_value = own_ref.value == other_ref.value;

        let own_parent = own_ref.parent.read().unwrap().upgrade();
        let other_parent = other_ref.parent.read().unwrap().upgrade();
        let mut same_parent = false;
        if own_parent.is_some() && other_parent.is_some() {
            same_parent =
                *own_parent.unwrap().read().unwrap() == *other_parent.unwrap().read().unwrap();
        }

        let own_children = own_ref.children.read().unwrap();
        let other_children = other_ref.children.read().unwrap();
        let mut same_children = false;
        if own_children.len() == other_children.len() {
            same_children = true;
            for i in 0..own_children.len() {
                if *own_children[i].read().unwrap() != *other_children[i].read().unwrap() {
                    same_children = false;
                    break;
                }
            }
        }

        same_value && same_parent && same_children
    }
}

// Shapes tree structure END

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
        let mut s = new_test_shape();
        s.set_transform(&Transform::translate(5.0, 0.0, 0.0));
        let _xs = r.intersect(&s);
        let saved_ray = TestShape::get_saved_ray().unwrap();
        assert_eq!(saved_ray.origin, Tuple::new_point(-5.0, 0.0, -5.0));
        assert_eq!(saved_ray.direction, Tuple::new_vector(0.0, 0.0, 1.0));
    }
    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let mut s = new_test_shape();
        s.set_transform(&Transform::translate(0.0, 1.0, 0.0));
        let n = s.normal_at(&Tuple::new_point(0.0, 1.70711, -0.70711));
        assert_eq!(n, Tuple::new_vector(0.0, 0.70711, -0.70711));
    }
    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let mut s = new_test_shape();
        let m = Transform::scaling(1.0, 0.5, 1.0) * Transform::rotation_z(PI / 5.0);
        s.set_transform(&m);
        let n = s.normal_at(&Tuple::new_point(
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
    fn a_shape_has_a_parent_attribute() {
        let s = new_test_shape();
        todo!("Fix grouping and parents");
        //assert!(s.get_parent().is_none());
    }
}
