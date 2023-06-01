use crate::ray_tracer::{
    intersections::Intersection,
    rays::Ray,
    tuples::{Point, Vector},
};

use super::{ShapeType, Shapes};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Cylinder {}

impl Cylinder {
    pub(crate) fn new() -> Self {
        Self {}
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Self::new()
    }
}

impl Shapes for Cylinder {
    fn set_position(&mut self, pos: &Point) {
        todo!()
    }
    fn get_position(&self) -> Point {
        todo!()
    }
    fn get_shape_type(&self) -> ShapeType {
        todo!()
    }
    fn local_normal_at(&self, point: Point) -> Vector {
        todo!()
    }
    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
        todo!()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
}
