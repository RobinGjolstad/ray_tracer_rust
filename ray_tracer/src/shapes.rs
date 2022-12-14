use crate::{
    matrices::Matrix,
    transformations::Transform,
    tuples::{Point, Tuple, Vector},
};

#[derive(Debug, PartialEq, Clone, Copy)]
enum Objects {
    Sphere(),
}
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Object {
    pub position: Point,
    object: Objects,
    transform: Matrix,
}
impl Object {
    pub fn sphere() -> Self {
        Object {
            position: Tuple::new_point(0.0, 0.0, 0.0),
            object: Objects::Sphere(),
            transform: Matrix::new_identity(),
        }
    }
    pub fn get_position(&self) -> Point {
        self.position
    }
    pub fn set_transform(&mut self, trans: Matrix) {
        self.transform = trans;
    }
    pub fn get_transform(&self) -> Matrix {
        self.transform
    }
    pub fn normal(&self, point: Point) -> Vector {
        match self.object {
            Sphere => {
                let transform_inverse = self.get_transform().inverse().unwrap();
                let object_point = transform_inverse * point;
                let object_normal = object_point - Point::new_point(0.0, 0.0, 0.0);
                let mut world_normal = transform_inverse.transpose().unwrap() * object_normal;
                world_normal.w = 0.0;

                world_normal.normalize()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use crate::transformations::Transform;

    use super::*;
    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Object::sphere();
        let n = s.normal(Tuple::new_point(1.0, 0.0, 0.0));
        assert_eq!(n, Tuple::new_vector(1.0, 0.0, 0.0));
    }
    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Object::sphere();
        let n = s.normal(Tuple::new_point(0.0, 1.0, 0.0));
        assert_eq!(n, Tuple::new_vector(0.0, 1.0, 0.0));
    }
    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Object::sphere();
        let n = s.normal(Tuple::new_point(0.0, 0.0, 1.0));
        assert_eq!(n, Tuple::new_vector(0.0, 0.0, 1.0));
    }
    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Object::sphere();
        let n = s.normal(Tuple::new_point(
            f32::sqrt(3.0) / 3.0,
            f32::sqrt(3.0) / 3.0,
            f32::sqrt(3.0) / 3.0,
        ));
        assert_eq!(
            n,
            Tuple::new_vector(
                f32::sqrt(3.0) / 3.0,
                f32::sqrt(3.0) / 3.0,
                f32::sqrt(3.0) / 3.0
            )
        );
    }
    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Object::sphere();
        let n = s.normal(Tuple::new_point(
            f32::sqrt(3.0) / 3.0,
            f32::sqrt(3.0) / 3.0,
            f32::sqrt(3.0) / 3.0,
        ));
        assert_eq!(n, n.normalize())
    }
    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = Object::sphere();
        s.set_transform(Transform::translate(0.0, 1.0, 0.0));
        let n = s.normal(Point::new_point(0.0, 1.70711, -0.70711));
        assert_eq!(n, Vector::new_vector(0.0, 0.70711, -0.70711));
    }
    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = Object::sphere();
        let m = Transform::scaling(1.0, 0.5, 1.0) * Transform::rotation_z(PI / 5.0);
        s.set_transform(m);
        let n = s.normal(Point::new_point(
            0.0,
            f32::sqrt(2.0) / 2.0,
            -(f32::sqrt(2.0) / 2.0),
        ));
        assert_eq!(n, Vector::new_vector(0.0, 0.97014, -0.24254));
    }
}
