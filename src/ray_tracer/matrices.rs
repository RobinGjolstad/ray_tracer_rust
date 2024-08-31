// Allow using `.get(0)` on vectors to make the matrix calculations more obvious
#![allow(clippy::get_first)]

use std::{
    fmt::{Display, Formatter},
    ops::Mul,
};

use crate::ray_tracer::{tuples::Tuple, utils::is_float_equal};

use super::utils::is_float_equal_low_precision;

#[derive(Debug)]
pub enum MatrixError {
    AsymmetricMatrix,
    InvalidSize,
    NonInvertible,
    NotInverted,
}

#[derive(Debug, Clone, Copy)]
pub struct Matrix {
    matrix: [[f64; 4]; 4],
    size: usize,
    inverse: [[f64; 4]; 4],
    is_inverted: bool,
}

impl Matrix {
    pub fn new(input: Vec<Vec<f64>>) -> Result<Matrix, MatrixError> {
        // Ensure the input is a symmetrical matrix
        let x = input.len();
        for i in 0..x {
            if input.get(i).unwrap().len() != x {
                return Err(MatrixError::AsymmetricMatrix);
            }
        }

        // Ensure the matrix is a 2x2, 3x3, or 4x4
        if !(2..5).contains(&x) {
            return Err(MatrixError::InvalidSize);
        }

        let mut m = Matrix {
            matrix: [[0.0; 4]; 4],
            size: 4,
            inverse: [[0.0; 4]; 4],
            is_inverted: false,
        };
        let mut row_idx = 0;
        for row in input {
            for (col_idx, column) in row.into_iter().enumerate() {
                m.matrix[row_idx][col_idx] = column;
            }
            row_idx += 1;
        }
        m.size = row_idx;

        Ok(m)
    }

    pub fn new_empty(size: usize) -> Result<Matrix, MatrixError> {
        if size > 1 && size < 5 {
            Ok(Matrix {
                matrix: [[0.0; 4]; 4],
                size,
                inverse: [[0.0; 4]; 4],
                is_inverted: false,
            })
        } else {
            Err(MatrixError::InvalidSize)
        }
    }

    pub fn new_identity() -> Matrix {
        Matrix {
            matrix: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
            size: 4,
            inverse: [[0.0; 4]; 4],
            is_inverted: false,
        }
    }

    pub fn get_element(&self, x: usize, y: usize) -> f64 {
        self.matrix[x][y]
    }

    pub fn set_element(&mut self, x: usize, y: usize, val: f64) {
        self.matrix[x][y] = val;
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get_matrix(&self) -> [[f64; 4]; 4] {
        self.matrix
    }

    pub fn is_inverted(&self) -> bool {
        self.is_inverted
    }

    pub fn get_inverted(&self) -> Result<Matrix, MatrixError> {
        if !self.is_inverted {
            Err(MatrixError::NotInverted)
        } else {
            Ok(Matrix {
                matrix: self.inverse,
                size: self.size,
                inverse: [[0.0; 4]; 4],
                is_inverted: false,
            })
        }
    }

    pub fn transpose(&self) -> Result<Matrix, MatrixError> {
        let size = self.size;
        if size != 4 {
            return Err(MatrixError::InvalidSize);
        }

        // Start with an identity matrix, just because that doesn't require any input parameters
        let mut mat = Matrix::new_identity();
        for row in 0..size {
            for column in 0..size {
                mat.matrix[row][column] = self.matrix[column][row];
            }
        }

        Ok(mat)
    }

    pub fn determinant(&self) -> f64 {
        let mut det = 0.0;
        if self.size == 2 {
            let ad = self.matrix[0][0] * self.matrix[1][1];
            let bc = self.matrix[0][1] * self.matrix[1][0];
            det = ad - bc;
        } else {
            for column in 0..self.size() {
                det += self.matrix[0][column] * self.cofactor(0, column);
            }
        }

        det
    }

    fn submatrix(&self, row: usize, column: usize) -> Matrix {
        let size = self.size;
        let mut mat = Matrix::new_empty(size - 1).unwrap();
        let mut row_ctr = 0;

        for row_id in 0..size {
            if row_id == row {
                continue;
            }

            let mut column_ctr = 0;
            for column_id in 0..size {
                if column_id == column {
                    continue;
                }

                mat.matrix[row_ctr][column_ctr] = self.matrix[row_id][column_id];
                column_ctr += 1;
            }
            row_ctr += 1;
        }

        mat
    }

    fn minor(&self, row: usize, column: usize) -> f64 {
        let submatrix = self.submatrix(row, column);
        submatrix.determinant()
    }

    fn cofactor(&self, row: usize, column: usize) -> f64 {
        assert!(2 < self.size);

        let minor = self.minor(row, column);
        if (row + column) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    fn invertible(&self) -> bool {
        !is_float_equal(&self.determinant(), 0.0)
    }

    pub fn calculate_inverse(&mut self) -> Result<Self, MatrixError> {
        if !self.invertible() {
            return Err(MatrixError::NonInvertible);
        }

        let mut m2 = Self::new_empty(self.size).unwrap();

        for row in 0..self.size {
            for column in 0..self.size {
                let c = self.cofactor(row, column);
                m2.matrix[column][row] = c / self.determinant();
            }
        }
        self.inverse = m2.matrix;
        self.is_inverted = true;
        Ok(*self)
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        let size = self.size();

        for i in 0..size {
            // outer loop
            for j in 0..size {
                // inner loop
                if !is_float_equal_low_precision(
                    self.matrix.get(i).unwrap().get(j).unwrap(),
                    *other.matrix.get(i).unwrap().get(j).unwrap(),
                ) {
                    return false;
                }
            }
        }

        true
    }
}

impl Mul for Matrix {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let size = self.size();
        assert_eq!(size, 4, "Only 4x4 matrix is supported!"); // Only 4x4 matrix is supported
        let mut m = Matrix {
            matrix: [[0.0; 4], [0.0; 4], [0.0; 4], [0.0; 4]],
            size: 4,
            inverse: [[0.0; 4]; 4],
            is_inverted: false,
        };
        for row in 0..size {
            for column in 0..size {
                m.matrix[row][column] = (self.matrix[row][0] * rhs.matrix[0][column])
                    + (self.matrix[row][1] * rhs.matrix[1][column])
                    + (self.matrix[row][2] * rhs.matrix[2][column])
                    + (self.matrix[row][3] * rhs.matrix[3][column]);
            }
        }

        m
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;
    fn mul(self, rhs: Tuple) -> Self::Output {
        let size = self.size();
        assert_eq!(4, size);
        let mut tup = [0.0; 4];
        for (row, item) in tup.iter_mut().enumerate().take(size) {
            *item = self.matrix[row][0] * rhs.x
                + self.matrix[row][1] * rhs.y
                + self.matrix[row][2] * rhs.z
                + self.matrix[row][3] * rhs.w;
        }

        Tuple::new(tup[0], tup[1], tup[2], tup[3])
    }
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        for row in 0..self.size {
            output.push('[');
            for column in 0..self.size {
                output.push_str(&self.matrix[row][column].to_string());
                if column < self.size - 1 {
                    output.push_str(", ");
                }
            }
            output.push_str("]\n");
        }

        write!(f, "{}", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructing_and_inspecting_a_4_x_4_matrix() {
        let m = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
        ])
        .unwrap();

        assert!(is_float_equal(
            m.matrix.get(0).unwrap().get(0).unwrap(),
            1.0
        ));
        assert!(is_float_equal(
            m.matrix.get(0).unwrap().get(3).unwrap(),
            4.0
        ));
        assert!(is_float_equal(
            m.matrix.get(1).unwrap().get(0).unwrap(),
            5.5
        ));
        assert!(is_float_equal(
            m.matrix.get(1).unwrap().get(2).unwrap(),
            7.5
        ));
        assert!(is_float_equal(
            m.matrix.get(2).unwrap().get(2).unwrap(),
            11.0
        ));
        assert!(is_float_equal(
            m.matrix.get(3).unwrap().get(0).unwrap(),
            13.5
        ));
        assert!(is_float_equal(
            m.matrix.get(3).unwrap().get(2).unwrap(),
            15.5
        ));
    }

    #[test]
    fn a_2x2_matrix_ought_to_be_representable() {
        let m = Matrix::new(vec![vec![-3.0, 5.0], vec![1.0, -2.0]]).unwrap();

        assert!(is_float_equal(
            m.matrix.get(0).unwrap().get(0).unwrap(),
            -3.0
        ));
        assert!(is_float_equal(
            m.matrix.get(0).unwrap().get(1).unwrap(),
            5.0
        ));
        assert!(is_float_equal(
            m.matrix.get(1).unwrap().get(0).unwrap(),
            1.0
        ));
        assert!(is_float_equal(
            m.matrix.get(1).unwrap().get(1).unwrap(),
            -2.0
        ));
    }

    #[test]
    fn a_3x3_matrix_ought_to_be_representable() {
        let m = Matrix::new(vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ])
        .unwrap();

        assert!(is_float_equal(
            m.matrix.get(0).unwrap().get(0).unwrap(),
            -3.0
        ));
        assert!(is_float_equal(
            m.matrix.get(1).unwrap().get(1).unwrap(),
            -2.0
        ));
        assert!(is_float_equal(
            m.matrix.get(2).unwrap().get(2).unwrap(),
            1.0
        ));
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ])
        .unwrap();
        let b = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ])
        .unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ])
        .unwrap();
        let b = Matrix::new(vec![
            vec![2.0, 3.0, 4.0, 5.0],
            vec![6.0, 7.0, 8.0, 9.0],
            vec![8.0, 7.0, 6.0, 5.0],
            vec![4.0, 3.0, 2.0, 1.0],
        ])
        .unwrap();

        assert_ne!(a, b);
    }

    #[test]
    fn multiplying_two_matrices() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ])
        .unwrap();
        let b = Matrix::new(vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ])
        .unwrap();

        let ab = Matrix::new(vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
        ])
        .unwrap();

        assert_eq!(a * b, ab);
    }

    #[test]
    fn a_matrix_multiplied_by_a_tuple() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ])
        .unwrap();
        let b = Tuple::new(1.0, 2.0, 3.0, 1.0);

        let ab = Tuple::new(18.0, 24.0, 33.0, 1.0);

        assert_eq!(a * b, ab);
    }

    #[test]
    fn multiplying_a_matrix_by_the_identity_matrix() {
        let a = Matrix::new(vec![
            vec![0.0, 1.0, 2.0, 4.0],
            vec![1.0, 2.0, 4.0, 8.0],
            vec![2.0, 4.0, 8.0, 16.0],
            vec![4.0, 8.0, 16.0, 32.0],
        ])
        .unwrap();
        let ia = a * Matrix::new_identity();
        assert_eq!(ia, a);
    }

    #[test]
    fn multiplying_a_tuple_by_the_identity_matrix() {
        let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
        let ia = Matrix::new_identity() * a;
        assert_eq!(ia, a);
    }

    #[test]
    fn transposing_a_matrix() {
        let a = Matrix::new(vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ])
        .unwrap();
        let ta = Matrix::new(vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0],
        ])
        .unwrap();

        assert_eq!(a.transpose().unwrap(), ta);
    }

    #[test]
    fn transposing_the_identity_matrix() {
        let a = Matrix::new_identity();

        assert_eq!(a, a.transpose().unwrap());
    }

    #[test]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        let a = Matrix::new(vec![vec![1.0, 5.0], vec![-3.0, 2.0]]).unwrap();
        assert!(is_float_equal(&17.0, a.determinant()));
    }

    #[test]
    fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        let a = Matrix::new(vec![
            vec![1.0, 5.0, 0.0],
            vec![-3.0, 2.0, 7.0],
            vec![0.0, 6.0, -3.0],
        ])
        .unwrap();
        let sub_a = Matrix::new(vec![vec![-3.0, 2.0], vec![0.0, 6.0]]).unwrap();

        assert_eq!(a.submatrix(0, 2), sub_a);
    }

    #[test]
    fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        let a = Matrix::new(vec![
            vec![-6.0, 1.0, 1.0, 6.0],
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![-1.0, 0.0, 8.0, 2.0],
            vec![-7.0, 1.0, -1.0, 1.0],
        ])
        .unwrap();
        let sub_a = Matrix::new(vec![
            vec![-6.0, 1.0, 6.0],
            vec![-8.0, 8.0, 6.0],
            vec![-7.0, -1.0, 1.0],
        ])
        .unwrap();

        assert_eq!(a.submatrix(2, 1), sub_a);
    }

    #[test]
    fn calculating_a_minor_of_a_3x3_matrix() {
        let a = Matrix::new(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ])
        .unwrap();
        let b = a.submatrix(1, 0);

        assert!(is_float_equal(&25.0, b.determinant()));
        assert!(is_float_equal(&25.0, a.minor(1, 0)));
    }

    #[test]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        let a = Matrix::new(vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ])
        .unwrap();

        assert!(is_float_equal(&-12.0, a.minor(0, 0)));
        assert!(is_float_equal(&-12.0, a.cofactor(0, 0)));
        assert!(is_float_equal(&25.0, a.minor(1, 0)));
        assert!(is_float_equal(&-25.0, a.cofactor(1, 0)));
    }

    #[test]
    fn calculating_the_determinant_of_a_3x3_matrix() {
        let a = Matrix::new(vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0],
        ])
        .unwrap();

        assert!(is_float_equal(&a.cofactor(0, 0), 56.0));
        assert!(is_float_equal(&a.cofactor(0, 1), 12.0));
        assert!(is_float_equal(&a.cofactor(0, 2), -46.0));
        assert!(is_float_equal(&a.determinant(), -196.0));
    }

    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        let a = Matrix::new(vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ])
        .unwrap();

        assert!(is_float_equal(&a.cofactor(0, 0), 690.0));
        assert!(is_float_equal(&a.cofactor(0, 1), 447.0));
        assert!(is_float_equal(&a.cofactor(0, 2), 210.0));
        assert!(is_float_equal(&a.cofactor(0, 3), 51.0));
        assert!(is_float_equal(&a.determinant(), -4071.0));
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let a = Matrix::new(vec![
            vec![6.0, 4.0, 4.0, 4.0],
            vec![5.0, 5.0, 7.0, 6.0],
            vec![4.0, -9.0, 3.0, -7.0],
            vec![9.0, 1.0, 7.0, -6.0],
        ])
        .unwrap();

        assert!(is_float_equal(&a.determinant(), -2120.0));
        assert!(a.invertible());
    }

    #[test]
    fn testing_a_noninvertible_matrix_for_invertibility() {
        let a = Matrix::new(vec![
            vec![-4.0, 2.0, -2.0, -3.0],
            vec![9.0, 6.0, 2.0, 6.0],
            vec![0.0, -5.0, 1.0, -5.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ])
        .unwrap();

        assert!(is_float_equal(&a.determinant(), 0.0));
        assert!(!a.invertible());
    }

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        let mut a = Matrix::new(vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0],
        ])
        .unwrap();

        let b = Matrix {
            matrix: a.calculate_inverse().unwrap().inverse,
            size: a.size(),
            inverse: Matrix::new_empty(a.size()).unwrap().matrix,
            is_inverted: false,
        };

        let b_comp = Matrix::new(vec![
            vec![0.21805, 0.45113, 0.24060, -0.04511],
            vec![-0.80827, -1.45677, -0.44361, 0.52068],
            vec![-0.07895, -0.22368, -0.05263, 0.19737],
            vec![-0.52256, -0.81391, -0.30075, 0.30639],
        ])
        .unwrap();

        assert!(is_float_equal(&a.determinant(), 532.0));
        assert!(is_float_equal(&a.cofactor(2, 3), -160.0));
        assert!(is_float_equal(&b.matrix[3][2], -160.0 / 532.0));
        assert!(is_float_equal(&a.cofactor(3, 2), 105.0));
        assert!(is_float_equal(&b.matrix[2][3], 105.0 / 532.0));
        assert_eq!(b, b_comp);
    }

    #[test]
    fn calculating_the_inverse_of_another_matrix() {
        let mut a = Matrix::new(vec![
            vec![8.0, -5.0, 9.0, 2.0],
            vec![7.0, 5.0, 6.0, 1.0],
            vec![-6.0, 0.0, 9.0, 6.0],
            vec![-3.0, 0.0, -9.0, -4.0],
        ])
        .unwrap();
        let inv_a = Matrix::new(vec![
            vec![-0.15385, -0.15385, -0.28205, -0.53846],
            vec![-0.07692, 0.12308, 0.02564, 0.03077],
            vec![0.35897, 0.35897, 0.43590, 0.92308],
            vec![-0.69231, -0.69231, -0.76923, -1.92308],
        ])
        .unwrap();

        let b = Matrix {
            matrix: a.calculate_inverse().unwrap().inverse,
            size: a.size(),
            inverse: Matrix::new_empty(a.size()).unwrap().matrix,
            is_inverted: false,
        };

        assert_eq!(b, inv_a);
    }

    #[test]
    fn calculating_the_inverse_of_a_third_matrix() {
        let mut a = Matrix::new(vec![
            vec![9.0, 3.0, 0.0, 9.0],
            vec![-5.0, -2.0, -6.0, -3.0],
            vec![-4.0, 9.0, 6.0, 4.0],
            vec![-7.0, 6.0, 6.0, 2.0],
        ])
        .unwrap();
        let inv_a = Matrix::new(vec![
            vec![-0.04074, -0.07778, 0.14444, -0.22222],
            vec![-0.07778, 0.03333, 0.36667, -0.33333],
            vec![-0.02901, -0.14630, -0.10926, 0.12963],
            vec![0.17778, 0.06667, -0.26667, 0.33333],
        ])
        .unwrap();

        let b = Matrix {
            matrix: a.calculate_inverse().unwrap().inverse,
            size: a.size(),
            inverse: Matrix::new_empty(a.size()).unwrap().matrix,
            is_inverted: false,
        };

        assert_eq!(b, inv_a);
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let a = Matrix::new(vec![
            vec![3.0, -9.0, 7.0, 3.0],
            vec![3.0, -8.0, 2.0, -9.0],
            vec![-4.0, 4.0, 4.0, 1.0],
            vec![-6.0, 5.0, -1.0, 1.0],
        ])
        .unwrap();
        let mut b = Matrix::new(vec![
            vec![8.0, 2.0, 2.0, 2.0],
            vec![3.0, -1.0, 7.0, 0.0],
            vec![7.0, 0.0, 5.0, 4.0],
            vec![6.0, -2.0, 0.0, 5.0],
        ])
        .unwrap();
        let c = a * b;

        assert_eq!(
            c * b.calculate_inverse().unwrap().get_inverted().unwrap(),
            a
        );
    }
}
