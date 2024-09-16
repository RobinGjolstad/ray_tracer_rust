// Allow using `.get(0)` on vectors to make the matrix calculations more obvious
#![allow(clippy::get_first, clippy::missing_errors_doc)]

use std::{
    fmt::{Display, Formatter},
    iter::zip,
    ops::Mul,
};

use rayon::iter::IntoParallelRefIterator;

use crate::ray_tracer::utils::is_float_equal;

use super::{
    tuples::{new_point, new_vector, Point, Vector},
    utils::is_float_equal_low_precision,
};

/*
#[derive(Debug, Default, Copy, Clone)]
pub struct Matrix2 {
    matrix: [[f64; 2]; 2],
    inverse: Option<[[f64; 2]; 2]>,
    inverse_transpose: Option<[[f64; 2]; 2]>,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Matrix3 {
    matrix: [[f64; 3]; 3],
    inverse: Option<[[f64; 3]; 3]>,
    inverse_transpose: Option<[[f64; 3]; 3]>,
}
*/

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Matrix<const S: usize> {
    pub matrix: [[f64; S]; S],
    pub inverse: Option<[[f64; S]; S]>,
    pub inverse_transpose: Option<[[f64; S]; S]>,
}

impl<const S: usize> Matrix<S> {
    pub const fn new(matrix: [[f64; S]; S]) -> Self {
        Self {
            matrix,
            inverse: None,
            inverse_transpose: None,
        }
    }
    pub const fn new_empty() -> Self {
        Self {
            matrix: [[0.0; S]; S],
            inverse: None,
            inverse_transpose: None,
        }
    }
    pub fn get_element(&self, x: usize, y: usize) -> f64 {
        self.matrix[x][y]
    }
}
impl Matrix<3> {
    pub fn identity() -> Self {
        Self {
            matrix: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            inverse: None,
            inverse_transpose: None,
        }
    }
}

impl<const S: usize> Default for Matrix<S> {
    fn default() -> Self {
        Self::new_empty()
    }
}

impl Mul for Matrix<3> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut mat: [[f64; 3]; 3] = [[0.0; 3]; 3];

        for row in 0..3 {
            for column in 0..3 {
                // mat[row][column] =
                //       (self.matrix[row][0] * rhs.matrix[0][column])
                //     + (self.matrix[row][1] * rhs.matrix[1][column])
                //     + (self.matrix[row][2] * rhs.matrix[2][column])

                mat[row][column] = self.matrix[row][0].mul_add(
                    rhs.matrix[0][column],
                    self.matrix[row][1].mul_add(
                        rhs.matrix[1][column],
                        self.matrix[row][2] * rhs.matrix[2][column],
                    ),
                );
            }
        }

        Self {
            matrix: mat,
            inverse: None,
            inverse_transpose: None,
        }
    }
}

impl Mul for Matrix<4> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mut mat: [[f64; 4]; 4] = [[0.0; 4]; 4];

        for row in 0..4 {
            for column in 0..4 {
                // mat[row][column] =
                //       (self.matrix[row][0] * rhs.matrix[0][column])
                //     + (self.matrix[row][1] * rhs.matrix[1][column])
                //     + (self.matrix[row][2] * rhs.matrix[2][column])
                //     + (self.matrix[row][3] * rhs.matrix[3][column]);

                mat[row][column] = self.matrix[row][0].mul_add(
                    rhs.matrix[0][column],
                    self.matrix[row][1].mul_add(
                        rhs.matrix[1][column],
                        self.matrix[row][2].mul_add(
                            rhs.matrix[2][column],
                            self.matrix[row][3] * rhs.matrix[3][column],
                        ),
                    ),
                );
            }
        }

        Self {
            matrix: mat,
            inverse: None,
            inverse_transpose: None,
        }
    }
}

impl Mul<Point> for Matrix<3> {
    type Output = Point;
    fn mul(self, rhs: Point) -> Self::Output {
        let mut tup: [f64; 3] = [0.0; 3];

        for (row, item) in tup.iter_mut().enumerate().take(3) {
            *item = self.matrix[row][0] * rhs.x
                + self.matrix[row][1] * rhs.y
                + self.matrix[row][2] * rhs.z;
        }

        new_point(tup[0], tup[1], tup[2])
    }
}

impl Mul<Vector> for Matrix<3> {
    type Output = Vector;
    fn mul(self, rhs: Vector) -> Self::Output {
        let mut tup: [f64; 3] = [0.0; 3];

        for (row, item) in tup.iter_mut().enumerate().take(3) {
            *item = self.matrix[row][0] * rhs.x
                + self.matrix[row][1] * rhs.y
                + self.matrix[row][2] * rhs.z;
        }

        new_vector(tup[0], tup[1], tup[2])
    }
}

#[cfg(test)]
mod tests {
    use crate::ray_tracer::tuples::{new_point, new_vector};

    use super::*;

    #[test]
    fn constructing_and_inspecting_a_4_x_4_matrix() {
        let m = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert!(is_float_equal(&m.get_element(0, 0), 1.0));
        assert!(is_float_equal(&m.get_element(0, 3), 4.0));
        assert!(is_float_equal(&m.get_element(1, 0), 5.5));
        assert!(is_float_equal(&m.get_element(1, 2), 7.5));
        assert!(is_float_equal(&m.get_element(2, 2), 11.0));
        assert!(is_float_equal(&m.get_element(3, 0), 13.5));
        assert!(is_float_equal(&m.get_element(3, 2), 15.5));
    }

    #[test]
    fn a_2x2_matrix_ought_to_be_representable() {
        let m = Matrix::new([[-3.0, 5.0], [1.0, -2.0]]);

        assert!(is_float_equal(&m.get_element(0, 0), -3.0));
        assert!(is_float_equal(&m.get_element(0, 1), 5.0));
        assert!(is_float_equal(&m.get_element(1, 0), 1.0));
        assert!(is_float_equal(&m.get_element(1, 1), -2.0));
    }

    #[test]
    fn a_3x3_matrix_ought_to_be_representable() {
        let m = Matrix::new([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

        assert!(is_float_equal(&m.get_element(0, 0), -3.0));
        assert!(is_float_equal(&m.get_element(1, 1), -2.0));
        assert!(is_float_equal(&m.get_element(2, 2), 1.0));
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(a, b);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);

        assert_ne!(a, b);
    }

    #[test]
    fn multiplying_two_4x4_matrices() {
        let a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let b = Matrix::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let ab = Matrix::new([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        assert_eq!(a * b, ab);
    }

    #[test]
    fn multiplying_two_3x3_matrices() {
        let a = Matrix::new([[1.0, 2.0, 3.0], [4.0, 5.0, 6.0], [7.0, 8.0, 9.0]]);
        let b = Matrix::new([[-2.0, 1.0, 2.0], [3.0, 2.0, 1.0], [4.0, 3.0, 6.0]]);

        let ab = Matrix::new([[16.0, 14.0, 22.0], [31.0, 32.0, 49.0], [46.0, 50.0, 76.0]]);

        assert_eq!(a * b, ab);
    }

    #[test]
    fn a_matrix_multiplied_by_a_point() {
        let a = Matrix::new([[1.0, 2.0, 3.0], [2.0, 4.0, 4.0], [8.0, 6.0, 4.0]]);
        let b = new_point(1.0, 2.0, 3.0);

        let ab = new_point(18.0, 24.0, 33.0);

        assert_eq!(a * b, ab);
    }
    #[test]
    fn a_matrix_multiplied_by_a_vector() {
        let a = Matrix::new([[1.0, 2.0, 3.0], [2.0, 4.0, 4.0], [8.0, 6.0, 4.0]]);
        let b = new_vector(1.0, 2.0, 3.0);

        let ab = new_vector(18.0, 24.0, 33.0);

        assert_eq!(a * b, ab);
    }
}
