#![allow(unused)]
use super::*;
use crate::ray_tracer::{
    intersections::Intersection,
    materials::Material,
    matrices::Matrix,
    rays::Ray,
    tuples::{Point, Vector},
};

/// Ugly hack for testing purposes
mod saved_ray {
    use crate::ray_tracer::rays::Ray;

    pub(super) static mut SAVED_RAY: Option<Ray> = None;
}
use saved_ray::SAVED_RAY;

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TestShape {
    base: BaseShape,
    parent: Option<BaseShape>,
}
impl TestShape {
    pub(super) fn new() -> TestShape {
        TestShape {
            base: BaseShape {
                position: Some(Point::new_point(0.0, 0.0, 0.0)),
                transform: Some(Matrix::new_identity().calculate_inverse().unwrap()),
                material: Some(Material::new()),
            },
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
    fn set_position(&mut self, pos: &Point) {
        self.base.position = Some(*pos);
    }
    fn get_position(&self) -> Point {
        self.base.position.unwrap()
    }
    fn set_transform(&mut self, transform: &Matrix) {
        let transform = transform.clone().calculate_inverse().unwrap();
        self.base.transform = Some(transform);
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
    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
        unsafe {
            SAVED_RAY = Some(local_ray);
        }
        Vec::new()
    }
}
