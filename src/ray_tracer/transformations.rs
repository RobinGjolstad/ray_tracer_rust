use crate::ray_tracer::{
    matrices_new::Matrix,
    tuples_new::{Point, Vector},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Transform;
impl Transform {
    #[must_use]
    pub const fn translate(x: f64, y: f64, z: f64) -> Matrix<4> {
        Matrix::new([
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    #[must_use]
    pub const fn scaling(x: f64, y: f64, z: f64) -> Matrix<4> {
        Matrix::new([
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    #[must_use]
    pub fn rotation_x(angle: f64) -> Matrix<4> {
        Matrix::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, f64::cos(angle), -f64::sin(angle), 0.0],
            [0.0, f64::sin(angle), f64::cos(angle), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    #[must_use]
    pub fn rotation_y(angle: f64) -> Matrix<4> {
        Matrix::new([
            [f64::cos(angle), 0.0, f64::sin(angle), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-f64::sin(angle), 0.0, f64::cos(angle), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    #[must_use]
    pub fn rotation_z(angle: f64) -> Matrix<4> {
        Matrix::new([
            [f64::cos(angle), -f64::sin(angle), 0.0, 0.0],
            [f64::sin(angle), f64::cos(angle), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    #[must_use]
    pub const fn shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix<4> {
        Matrix::new([
            [1.0, x_y, x_z, 0.0],
            [y_x, 1.0, y_z, 0.0],
            [z_x, z_y, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    #[must_use]
    pub fn view_transform(from: &Point, to: &Point, up: &Vector) -> Matrix<4> {
        let forward = (*to - *from).normalize();
        let up_norm = up.normalize();
        let left = Vector::cross(&forward, &up_norm);
        let true_up = Vector::cross(&left, &forward);
        let orientation = Matrix::new([
            [left.x, left.y, left.z, 0.0],
            [true_up.x, true_up.y, true_up.z, 0.0],
            [-forward.x, -forward.y, -forward.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        orientation * Self::translate(-from.x, -from.y, -from.z)
    }
}

// Tests should be allowed to contain single-char variables.
#[allow(clippy::many_single_char_names)]
#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::ray_tracer::tuples_new::{new_point, new_vector};

    use super::*;

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = Transform::translate(5.0, -3.0, 2.0);
        let p = new_point(-3.0, 4.0, 5.0);

        assert_eq!(transform * p, new_point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let mut transform = Transform::translate(5.0, -3.0, 2.0);
        let inv = transform.inverse().inverse.unwrap();
        let p = new_point(-3.0, 4.0, 5.0);

        assert_eq!(inv * p, new_point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Transform::translate(5.0, -3.0, 2.0);
        let v = new_vector(-3.0, 4.0, 5.0);

        assert_eq!(transform * v, v);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let transform = Transform::scaling(2.0, 3.0, 4.0);
        let p = new_point(-4.0, 6.0, 8.0);

        assert_eq!(transform * p, new_point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let transform = Transform::scaling(2.0, 3.0, 4.0);
        let p = new_vector(-4.0, 6.0, 8.0);

        assert_eq!(transform * p, new_vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let mut transform = Transform::scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse().inverse.unwrap();
        let v = new_vector(-4.0, 6.0, 8.0);

        assert_eq!(inv * v, new_vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = Transform::scaling(-1.0, 1.0, 1.0);
        let p = new_point(2.0, 3.0, 4.0);

        assert_eq!(transform * p, new_point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = new_point(0.0, 1.0, 0.0);
        let half_quarter = Transform::rotation_x(PI / 4.0);
        let full_quarter = Transform::rotation_x(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            new_point(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)
        );
        assert_eq!(full_quarter * p, new_point(0.0, 0.0, 1.0));
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = new_point(0.0, 1.0, 0.0);
        let mut half_quarter = Transform::rotation_x(PI / 4.0);
        let inv = half_quarter.inverse().inverse.unwrap();

        assert_eq!(
            inv * p,
            new_point(0.0, f64::sqrt(2.0) / 2.0, -(f64::sqrt(2.0) / 2.0))
        );
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = new_point(0.0, 0.0, 1.0);
        let half_quarter = Transform::rotation_y(PI / 4.0);
        let full_quarter = Transform::rotation_y(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            new_point(f64::sqrt(2.0) / 2.0, 0.0, f64::sqrt(2.0) / 2.0)
        );
        assert_eq!(full_quarter * p, new_point(1.0, 0.0, 0.0));
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = new_point(0.0, 1.0, 0.0);
        let half_quarter = Transform::rotation_z(PI / 4.0);
        let full_quarter = Transform::rotation_z(PI / 2.0);

        assert_eq!(
            half_quarter * p,
            new_point(-(f64::sqrt(2.0) / 2.0), f64::sqrt(2.0) / 2.0, 0.0)
        );
        assert_eq!(full_quarter * p, new_point(-1.0, 0.0, 0.0));
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Transform::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = new_point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, new_point(5.0, 3.0, 4.0));
    }
    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Transform::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = new_point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, new_point(6.0, 3.0, 4.0));
    }
    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Transform::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = new_point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, new_point(2.0, 5.0, 4.0));
    }
    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Transform::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = new_point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, new_point(2.0, 7.0, 4.0));
    }
    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Transform::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = new_point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, new_point(2.0, 3.0, 6.0));
    }
    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Transform::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = new_point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, new_point(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = new_point(1.0, 0.0, 1.0);
        let a = Transform::rotation_x(PI / 2.0);
        let b = Transform::scaling(5.0, 5.0, 5.0);
        let c = Transform::translate(10.0, 5.0, 7.0);
        // Apply rotation first
        let p2 = a * p;
        assert_eq!(p2, new_point(1.0, -1.0, 0.0));
        // Then apply scaling
        let p3 = b * p2;
        assert_eq!(p3, new_point(5.0, -5.0, 0.0));
        // Then apply translation
        let p4 = c * p3;
        assert_eq!(p4, new_point(15.0, 0.0, 7.0));
    }
    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = new_point(1.0, 0.0, 1.0);
        let a = Transform::rotation_x(PI / 2.0);
        let b = Transform::scaling(5.0, 5.0, 5.0);
        let c = Transform::translate(10.0, 5.0, 7.0);
        let t = c * b * a;
        assert_eq!(t * p, new_point(15.0, 0.0, 7.0));
    }

    #[test]
    fn the_transformation_matrix_for_the_default_orientation() {
        let from = new_point(0.0, 0.0, 0.0);
        let to = new_point(0.0, 0.0, -1.0);
        let up = new_vector(0.0, 1.0, 0.0);
        let t = Transform::view_transform(&from, &to, &up);
        assert_eq!(t, Matrix::<4>::identity());
    }
    #[test]
    fn a_view_transformation_matrix_looking_in_positive_z_direction() {
        let from = new_point(0.0, 0.0, 0.0);
        let to = new_point(0.0, 0.0, 1.0);
        let up = new_vector(0.0, 1.0, 0.0);
        let t = Transform::view_transform(&from, &to, &up);
        assert_eq!(t, Transform::scaling(-1.0, 1.0, -1.0));
    }
    #[test]
    fn the_view_transformation_moves_the_world() {
        let from = new_point(0.0, 0.0, 8.0);
        let to = new_point(0.0, 0.0, 0.0);
        let up = new_vector(0.0, 1.0, 0.0);
        let t = Transform::view_transform(&from, &to, &up);
        assert_eq!(t, Transform::translate(0.0, 0.0, -8.0));
    }
    #[test]
    fn an_arbitrary_view_transformation() {
        let from = new_point(1.0, 3.0, 2.0);
        let to = new_point(4.0, -2.0, 8.0);
        let up = new_vector(1.0, 1.0, 0.0);
        let t = Transform::view_transform(&from, &to, &up);
        assert_eq!(
            t,
            Matrix::new([
                [-0.50709, 0.50709, 0.67612, -2.36643],
                [0.76772, 0.60609, 0.12122, -2.82843],
                [-0.35857, 0.59761, -0.71714, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ])
        );
    }
}
