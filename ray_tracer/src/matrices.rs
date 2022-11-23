use std::ops::Mul;

use crate::tuples::Tuple;

#[macro_export]
macro_rules! vec2d {
    ($($i:expr),+) => { // handle numbers
        {
            let mut ret = Vec::new();
            $(ret.push($i);)*
            ret
        }
    };

    //([$($arr:tt),+]) => { // handle sets
    //    {
    //        let mut ret = Vec::new();
    //        $(ret.push(vec!($arr));)*
    //        ret
    //    }
    //};
}

#[derive(Debug)]
pub enum MatrixError {
    AsymmetricMatrix,
    InvalidSize,
}

#[derive(Debug, Clone)]
pub struct Matrix {
    matrix: Vec<Vec<f32>>,
}

impl Matrix {
    fn new(input: Vec<Vec<f32>>) -> Result<Matrix, MatrixError> {
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

        Ok(Matrix { matrix: input })
    }

    fn new_identity() -> Matrix {
        Matrix {
            matrix: vec![
                vec![1.0, 0.0, 0.0, 0.0],
                vec![0.0, 1.0, 0.0, 0.0],
                vec![0.0, 0.0, 1.0, 0.0],
                vec![0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    fn get_element(&self, x: usize, y: usize) -> f32 {
        self.matrix[x][y]
    }

    fn size(&self) -> usize {
        self.matrix.len()
    }

    fn transpose(&self) -> Result<Matrix, MatrixError> {
        let size = self.size();
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
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        let size = self.size();

        for i in 0..size {
            // outer loop
            for j in 0..size {
                // inner loop
                if self.matrix.get(i).unwrap().get(j).unwrap()
                    != other.matrix.get(i).unwrap().get(j).unwrap()
                {
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
            matrix: vec![vec![0.0; 4], vec![0.0; 4], vec![0.0; 4], vec![0.0; 4]],
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
        let mut tup = vec![0.0; 4];
        for row in 0..size {
            tup[row] = self.matrix[row][0] * rhs.x
                + self.matrix[row][1] * rhs.y
                + self.matrix[row][2] * rhs.z
                + self.matrix[row][3] * rhs.w;
        }

        Tuple::new((tup[0], tup[1], tup[2], tup[3]))
    }
}

#[cfg(test)]
mod tests {
    use crate::{tuples::Tuple, utils::is_float_equal};

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
        let b = Tuple::new((1.0, 2.0, 3.0, 1.0));

        let ab = Tuple::new((18.0, 24.0, 33.0, 1.0));

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
        let ia = a.clone() * Matrix::new_identity();
        assert_eq!(ia, a);
    }

    #[test]
    fn multiplying_a_tuple_by_the_identity_matrix() {
        let a = Tuple::new((1.0, 2.0, 3.0, 4.0));
        let ia = Matrix::new_identity() * a.clone();
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
}
