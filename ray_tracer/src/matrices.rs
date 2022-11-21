

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

pub struct Matrix {
    matrix: Vec<Vec<f32>>,
}

impl  Matrix {
    fn new(input: Vec<Vec<f32>>) -> Matrix {
        Matrix {
            matrix: input, 
        }
    }
    fn get_element(&self, x: usize, y: usize) -> f32 {
        self.matrix[x][y]
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
        ]);

        assert!(is_float_equal(m.matrix.get(0).unwrap().get(0).unwrap(), 1.0));
        assert!(is_float_equal(m.matrix.get(0).unwrap().get(3).unwrap(), 4.0));
        assert!(is_float_equal(m.matrix.get(1).unwrap().get(0).unwrap(), 5.5));
        assert!(is_float_equal(m.matrix.get(1).unwrap().get(2).unwrap(), 7.5));
        assert!(is_float_equal(m.matrix.get(2).unwrap().get(2).unwrap(), 11.0));
        assert!(is_float_equal(m.matrix.get(3).unwrap().get(0).unwrap(), 13.5));
        assert!(is_float_equal(m.matrix.get(3).unwrap().get(2).unwrap(), 15.5));
    }
}
