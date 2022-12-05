use crate::{matrices::Matrix, tuples::Tuple};

enum RotationAxis {
    X,
    Y,
    Z,
}

pub struct Transform;
impl Transform {
    pub fn translate(x: f32, y: f32, z: f32) -> Matrix {
        Matrix::new(vec![
            vec![1.0, 0.0, 0.0, x],
            vec![0.0, 1.0, 0.0, y],
            vec![0.0, 0.0, 1.0, z],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
        .unwrap()
    }
    pub fn scaling(x: f32, y: f32, z: f32) -> Matrix {
        Matrix::new(vec![
            vec![x, 0.0, 0.0, 0.0],
            vec![0.0, y, 0.0, 0.0],
            vec![0.0, 0.0, z, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
        .unwrap()
    }
    pub fn rotation_x(angle: f32) -> Matrix {
        Matrix::new(vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, f32::cos(angle), -f32::sin(angle), 0.0],
            vec![0.0, f32::sin(angle), f32::cos(angle), 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
        .unwrap()
    }
    pub fn rotation_y(angle: f32) -> Matrix {
        Matrix::new(vec![
            vec![f32::cos(angle), 0.0, f32::sin(angle), 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![-f32::sin(angle), 0.0, f32::cos(angle), 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
        .unwrap()
    }
    pub fn rotation_z(angle: f32) -> Matrix {
        Matrix::new(vec![
            vec![f32::cos(angle), -f32::sin(angle), 0.0, 0.0],
            vec![f32::sin(angle), f32::cos(angle), 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
        .unwrap()
    }
    pub fn shearing(x_y: f32, x_z: f32, y_x: f32, y_z: f32, z_x: f32, z_y: f32) -> Matrix {
        Matrix::new(vec![
            vec![1.0, x_y, x_z, 0.0],
            vec![y_x, 1.0, y_z, 0.0],
            vec![z_x, z_y, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
        .unwrap()
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use super::*;
    use crate::tuples::Tuple;

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = Transform::translate(5.0, -3.0, 2.0);
        let p = Tuple::new_point(-3.0, 4.0, 5.0);

        assert_eq!(transform * p, Tuple::new_point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = Transform::translate(5.0, -3.0, 2.0);
        let inv = transform.inverse().unwrap();
        let p = Tuple::new_point(-3.0, 4.0, 5.0);

        assert_eq!(inv * p, Tuple::new_point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Transform::translate(5.0, -3.0, 2.0);
        let v = Tuple::new_vector(-3.0, 4.0, 5.0);

        assert_eq!(transform * v, v);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let transform = Transform::scaling(2.0, 3.0, 4.0);
        let p = Tuple::new_point(-4.0, 6.0, 8.0);

        assert_eq!(transform * p, Tuple::new_point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let transform = Transform::scaling(2.0, 3.0, 4.0);
        let p = Tuple::new_vector(-4.0, 6.0, 8.0);

        assert_eq!(transform * p, Tuple::new_vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = Transform::scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse().unwrap();
        let v = Tuple::new_vector(-4.0, 6.0, 8.0);

        assert_eq!(inv * v, Tuple::new_vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = Transform::scaling(-1.0, 1.0, 1.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, Tuple::new_point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = Transform::rotation_x(PI / 4.0);
        let full_quarter = Transform::rotation_x(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            Tuple::new_point(0.0, f32::sqrt(2.0) / 2.0, f32::sqrt(2.0) / 2.0)
        );
        assert_eq!(full_quarter * p, Tuple::new_point(0.0, 0.0, 1.0));
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = Transform::rotation_x(PI / 4.0);
        let inv = half_quarter.inverse().unwrap();

        assert_eq!(
            inv * p,
            Tuple::new_point(0.0, f32::sqrt(2.0) / 2.0, -(f32::sqrt(2.0) / 2.0))
        );
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Tuple::new_point(0.0, 0.0, 1.0);
        let half_quarter = Transform::rotation_y(PI / 4.0);
        let full_quarter = Transform::rotation_y(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            Tuple::new_point(f32::sqrt(2.0) / 2.0, 0.0, f32::sqrt(2.0) / 2.0)
        );
        assert_eq!(full_quarter * p, Tuple::new_point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = Transform::rotation_z(PI / 4.0);
        let full_quarter = Transform::rotation_z(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            Tuple::new_point(-(f32::sqrt(2.0) / 2.0), f32::sqrt(2.0) / 2.0, 0.0)
        );
        assert_eq!(full_quarter * p, Tuple::new_point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Transform::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::new_point(5.0, 3.0, 4.0));
    }
    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Transform::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::new_point(6.0, 3.0, 4.0));
    }
    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Transform::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::new_point(2.0, 5.0, 4.0));
    }
    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Transform::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::new_point(2.0, 7.0, 4.0));
    }
    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Transform::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::new_point(2.0, 3.0, 6.0));
    }
    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Transform::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::new_point(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Tuple::new_point(1.0, 0.0, 1.0);
        let a = Transform::rotation_x(PI / 2.0);
        let b = Transform::scaling(5.0, 5.0, 5.0);
        let c = Transform::translate(10.0, 5.0, 7.0);
        // Apply rotation first
        let p2 = a * p;
        assert_eq!(p2, Tuple::new_point(1.0, -1.0, 0.0));
        // Then apply scaling
        let p3 = b * p2;
        assert_eq!(p3, Tuple::new_point(5.0, -5.0, 0.0));
        // Then apply translation
        let p4 = c * p3;
        assert_eq!(p4, Tuple::new_point(15.0, 0.0, 7.0));
    }
    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Tuple::new_point(1.0, 0.0, 1.0);
        let a = Transform::rotation_x(PI / 2.0);
        let b = Transform::scaling(5.0, 5.0, 5.0);
        let c = Transform::translate(10.0, 5.0, 7.0);
        let t = c * b * a;
        assert_eq!(t * p, Tuple::new_point(15.0, 0.0, 7.0));
    }
}
