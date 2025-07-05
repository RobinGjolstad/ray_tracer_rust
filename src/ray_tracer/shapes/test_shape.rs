#![allow(unused)]
use super::*;
use crate::ray_tracer::{
    intersections::Intersection,
    materials::Material,
    matrices_new::Matrix,
    rays::Ray,
    tuples_new::{new_vector, Point, Vector},
};
use std::sync::Arc;

/// Ugly hack for testing purposes
mod saved_ray {
    use crate::ray_tracer::rays::Ray;

    pub(super) static mut SAVED_RAY: Option<Ray> = None;
}
use saved_ray::SAVED_RAY;

#[cfg(test)]
#[derive(Debug, Clone, PartialEq)]
pub struct TestShape {
    pub(super) base: BaseShape,
    pub(super) parent: Option<BaseShape>,
}
impl TestShape {
    pub(super) fn new() -> Self {
        Self {
            base: BaseShape::default(),
            parent: None,
        }
    }
    pub(super) fn get_saved_ray() -> Option<Ray> {
        unsafe { SAVED_RAY }
    }
}

impl Default for TestShape {
    fn default() -> Self {
        Self::new()
    }
}

impl Shapes for TestShape {
    fn get_transform(&self) -> Matrix<4> {
        self.base.transform
    }
    fn get_material(&self) -> Material {
        self.base.material
    }
    fn local_normal_at(&self, point: Point) -> Vector {
        new_vector(point.x, point.y, point.z)
    }
    fn local_intersect(
        &self,
        object: &Object,
        local_ray: Ray,
        intersection_list: &mut Vec<Intersection>,
    ) {
        unsafe {
            SAVED_RAY = Some(local_ray);
        }
    }
}
