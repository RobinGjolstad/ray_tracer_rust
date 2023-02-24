#![allow(unused)]
use crate::{
    materials::Material, matrices::Matrix, rays::Ray, shapes::test_shape::saved_ray::SAVED_RAY,
    tuples::Tuple,
};

use super::Shapes;

mod saved_ray {
    use crate::rays::Ray;

    pub static mut SAVED_RAY: Option<Ray> = None;
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TestShape {
    position: Tuple,
    transform: Matrix,
    material: Material,
}
impl TestShape {
    pub(super) fn new() -> TestShape {
        TestShape {
            position: Tuple::new_point(0.0, 0.0, 0.0),
            transform: Matrix::new_identity(),
            material: Material::new(),
        }
    }
    pub(super) fn get_saved_ray() -> Option<Ray> {
        unsafe { SAVED_RAY }
    }
}

impl Shapes for TestShape {
    fn set_position(&mut self, pos: &crate::tuples::Point) {
        self.position = *pos;
    }
    fn get_position(&self) -> crate::tuples::Point {
        self.position
    }
    fn set_material(&mut self, material: &Material) {
        self.material = *material;
    }
    fn set_transform(&mut self, trans: &Matrix) {
        self.transform = *trans;
    }
    fn local_normal_at(&self, point: crate::tuples::Point) -> crate::tuples::Vector {
        Tuple::new_vector(point.x, point.y, point.z)
    }
    fn get_shape_type(&self) -> super::ShapeType {
        super::ShapeType::TestShape
    }
    fn local_intersect(
        &self,
        local_ray: crate::rays::Ray,
    ) -> Vec<crate::intersections::Intersection> {
        unsafe {
            SAVED_RAY = Some(local_ray);
        }
        Vec::new()
    }
}
