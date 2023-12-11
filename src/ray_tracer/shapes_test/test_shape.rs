use super::*;
use crate::ray_tracer::{
    materials::Material,
    matrices::Matrix,
    shapes_test::BaseShape,
    tuples::{Point, Vector},
};

#[derive(Debug, Clone, Copy)]
struct TestShape {
    base: BaseShape,
    parent: Option<BaseShape>,
}
impl TestShape {
    fn new() -> TestShape {
        TestShape {
            base: BaseShape {
                position: Some(Point::new_point(0.0, 0.0, 0.0)),
                transform: None,
                material: Some(Material::new()),
            },
            parent: None,
        }
    }
}

impl Shapes for TestShape {
    fn set_position(&mut self, pos: &Point) {
        self.base.position = Some(*pos);
    }
    fn get_position(&self) -> Point {
        self.base.position.unwrap()
    }
    fn set_transform(&mut self, transform: Matrix) {
        self.base.transform = Some(transform);
    }
    fn get_transform(&self) -> Matrix {
        self.base.transform.unwrap()
    }
    fn set_material(&mut self, material: Material) {
        self.base.material = Some(material);
    }
    fn get_material(&self) -> Material {
        self.base.material.unwrap()
    }
    fn set_parent(&mut self, parent: BaseShape) {
        self.parent = Some(parent);
    }
    fn get_parent(&self) -> BaseShape {
        self.parent.unwrap()
    }
    fn local_normal_at(&self, point: Point) -> Vector {
        Vector::new_vector(point.x, point.y, point.z)
    }
    fn get_shape_type(&self) -> ShapeType {
        ShapeType::TestShape
    }
    fn local_intersect(&self, _local_ray: (f64, f64)) -> Vec<f64> {
        Vec::new()
    }
}
