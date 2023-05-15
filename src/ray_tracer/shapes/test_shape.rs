#![allow(unused)]
use crate::ray_tracer::{
    intersections::Intersection,
    materials::Material,
    matrices::Matrix,
    rays::Ray,
    shapes::test_shape::saved_ray::SAVED_RAY,
    tuples::{Point, Tuple, Vector},
};

use super::Shapes;

mod saved_ray {
    use crate::ray_tracer::rays::Ray;

    pub(super) static mut SAVED_RAY: Option<Ray> = None;
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub(super) struct TestShape {
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
    fn set_position(&mut self, pos: &Point) {
        self.position = *pos;
    }
    fn get_position(&self) -> Point {
        self.position
    }
    fn local_normal_at(&self, point: Point) -> Vector {
        Tuple::new_vector(point.x, point.y, point.z)
    }
    fn get_shape_type(&self) -> super::ShapeType {
        super::ShapeType::TestShape
    }
    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
        unsafe {
            SAVED_RAY = Some(local_ray);
        }
        Vec::new()
    }
}
