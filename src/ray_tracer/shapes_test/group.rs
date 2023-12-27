use super::{BaseShape, Object, Shapes};
use crate::ray_tracer::{
    intersections::Intersection,
    materials::Material,
    matrices::Matrix,
    rays::Ray,
    tuples::{Point, Vector},
};

#[derive(Debug, Clone, PartialEq)]
pub struct GroupBuilder {
    pub base: BaseShape,
    children: Vec<Object>,
    parent: Option<BaseShape>,
}
impl GroupBuilder {
    pub fn new() -> GroupBuilder {
        GroupBuilder {
            base: BaseShape {
                position: None,
                transform: None,
                material: None,
            },
            children: Vec::new(),
            parent: None,
        }
    }
    pub fn add_child(&mut self, child: Object) {
        self.children.push(child);
    }
    pub fn build(mut self) -> Group {
        self.children.iter_mut().for_each(|child| {
            child.set_parent(&self.base);
        });
        Group {
            base: self.base.clone(),
            children: self.children.clone(),
            parent: self.parent.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    base: BaseShape,
    children: Vec<Object>,
    parent: Option<BaseShape>,
}

impl Group {
    pub fn new() -> Self {
        todo!()
    }
}

impl Default for Group {
    fn default() -> Self {
        Self::new()
    }
}

impl Shapes for Group {
    fn set_position(&mut self, pos: &Point) {
        self.base.position = Some(*pos);
    }
    fn get_position(&self) -> Point {
        self.base.position.unwrap()
    }
    fn set_transform(&mut self, transform: &Matrix) {
        self.base.transform = Some(*transform);
    }
    fn get_transform(&self) -> Matrix {
        self.base.transform.unwrap()
    }
    fn set_material(&mut self, material: &Material) {
        self.base.material = Some(*material);
    }
    fn get_material(&self) -> Material {
        self.base.material.unwrap()
    }
    fn set_parent(&mut self, parent: &BaseShape) {
        self.parent = Some(*parent);
    }
    fn get_parent(&self) -> BaseShape {
        self.parent.unwrap()
    }
    fn local_normal_at(&self, point: Point) -> Vector {
        Vector::new_vector(point.x, point.y, point.z)
    }
    fn local_intersect(&self, _local_ray: Ray) -> Vec<Intersection> {
        Vec::new()
    }
}
