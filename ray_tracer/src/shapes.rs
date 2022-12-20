use crate::{
    materials::Material,
    matrices::Matrix,
    tuples::{Point, Vector},
};

pub mod sphere;

use self::sphere::Sphere;

pub trait Shapes {
    fn get_position(&self) -> Point;
    fn set_transform(&mut self, trans: &Matrix);
    fn get_transform(&self) -> Matrix;
    fn set_material(&mut self, material: &Material);
    fn get_material(&self) -> Material;
    fn normal(&self, point: Point) -> Vector;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Object {
    Sphere(Sphere),
}

#[macro_export]
macro_rules! extract_object {
    ($e:expr) => {
        match $e {
            Object::Sphere(s) => s,
            _ => todo!("This variant is not possible to extract!"),
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extracting_an_unknown_value_from_an_enum() {
        let sphere = Sphere::new();
        let obj = Object::Sphere(sphere);
        let extract = extract_object!(obj);
        assert_eq!(sphere, extract);
    }
}
