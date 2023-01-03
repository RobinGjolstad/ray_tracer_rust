use crate::{materials::Material, matrices::Matrix, tuples::Tuple};

use super::Shapes;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct TestShape {
    position: Tuple,
    transform: Matrix,
    material: Material,
}
impl TestShape {
    pub(crate) fn new() -> TestShape {
        TestShape {
            position: Tuple::new_point(0.0, 0.0, 0.0),
            transform: Matrix::new_identity(),
            material: Material::new(),
        }
    }
}

impl Shapes for TestShape {
    fn get_material(&self) -> crate::materials::Material {
        self.material
    }
    fn set_material(&mut self, material: &crate::materials::Material) {
        self.material = *material;
    }
    fn get_position(&self) -> crate::tuples::Point {
        self.position
    }
    fn get_transform(&self) -> crate::matrices::Matrix {
        self.transform
    }
    fn set_transform(&mut self, trans: &crate::matrices::Matrix) {
        self.transform = *trans;
    }
    fn normal(&self, point: crate::tuples::Point) -> crate::tuples::Vector {
        todo!()
    }
    fn get_shape_type(&self) -> super::ShapeType {
        super::ShapeType::TestShape
    }
}
