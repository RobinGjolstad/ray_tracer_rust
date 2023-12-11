use crate::ray_tracer::{
    materials::Material,
    matrices::Matrix,
    shapes::ShapeType,
    tuples::{Point, Vector},
};
use std::fmt::Debug;

mod test_shape;

pub(super) trait Shapes: Debug + Sync {
    fn set_position(&mut self, pos: &Point);
    fn get_position(&self) -> Point;
    fn set_transform(&mut self, transform: Matrix);
    fn get_transform(&self) -> Matrix;
    fn set_material(&mut self, material: Material);
    fn get_material(&self) -> Material;
    fn set_parent(&mut self, parent: BaseShape);
    fn get_parent(&self) -> BaseShape;
    fn local_normal_at(&self, point: Point) -> Vector;
    fn get_shape_type(&self) -> ShapeType;
    fn local_intersect(&self, local_ray: (f64, f64)) -> Vec<f64>;
}

#[derive(Debug, Clone, Copy)]
pub(super) struct BaseShape {
    position: Option<Point>,
    transform: Option<Matrix>,
    material: Option<Material>,
}

#[derive(Debug, Clone)]
enum Object {
    TestShape(TestShape),
    Group(Group),
}
impl Object {
    fn world_point_to_local(&self, point: &Point) -> Point {
        self.get_transform().get_inverted().unwrap() * *point
    }
}

impl Shapes for Object {
    fn set_position(&mut self, pos: &Point) {
        match self {
            Object::TestShape(test_shape) => test_shape.set_position(pos),
            Object::Group(group) => group.set_position(pos),
        }
    }
    fn get_position(&self) -> Point {
        match self {
            Object::TestShape(test_shape) => test_shape.get_position(),
            Object::Group(group) => group.get_position(),
        }
    }
    fn set_transform(&mut self, transform: Matrix) {
        match self {
            Object::TestShape(test_shape) => test_shape.set_transform(transform),
            Object::Group(group) => group.set_transform(transform),
        }
    }
    fn get_transform(&self) -> Matrix {
        match self {
            Object::TestShape(test_shape) => test_shape.get_transform(),
            Object::Group(group) => group.get_transform(),
        }
    }
    fn set_material(&mut self, material: Material) {
        match self {
            Object::TestShape(test_shape) => test_shape.set_material(material),
            Object::Group(group) => group.set_material(material),
        }
    }
    fn get_material(&self) -> Material {
        match self {
            Object::TestShape(test_shape) => test_shape.get_material(),
            Object::Group(group) => group.get_material(),
        }
    }
    fn set_parent(&mut self, parent: BaseShape) {
        match self {
            Object::TestShape(test_shape) => test_shape.set_parent(parent),
            Object::Group(group) => group.set_parent(parent),
        }
    }
    fn get_parent(&self) -> BaseShape {
        match self {
            Object::TestShape(test_shape) => test_shape.get_parent(),
            Object::Group(group) => group.get_parent(),
        }
    }
    fn local_normal_at(&self, point: Point) -> Vector {
        match self {
            Object::TestShape(test_shape) => test_shape.local_normal_at(point),
            Object::Group(group) => group.local_normal_at(point),
        }
    }
    fn get_shape_type(&self) -> ShapeType {
        match self {
            Object::TestShape(test_shape) => test_shape.get_shape_type(),
            Object::Group(group) => group.get_shape_type(),
        }
    }
    fn local_intersect(&self, local_ray: (f64, f64)) -> Vec<f64> {
        match self {
            Object::TestShape(test_shape) => test_shape.local_intersect(local_ray),
            Object::Group(group) => group.local_intersect(local_ray),
        }
    }
}

fn new_test_shape() -> Object {
    Object::TestShape(TestShape::new())
}
fn new_group(group: Group) -> Object {
    Object::Group(group)
}
