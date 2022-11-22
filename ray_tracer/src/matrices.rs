use std::ops::Mul;

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
    fn get_element(&self, x: usize, y: usize) -> f32 {
        self.matrix[x][y]
    }
    fn size(&self) -> usize {
        self.matrix.len()
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

#[cfg(test)]
mod tests {
    use crate::utils::is_float_equal;

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
}
