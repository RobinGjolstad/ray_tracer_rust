
mod Tuples {
    use std::{f32::EPSILON};

    pub fn point(a: (f32, f32, f32)) -> (f32, f32, f32, f32) {
        (a.0, a.1, a.2, 1.0)
    }
    pub fn is_point(tuple: (f32, f32, f32, f32)) -> bool {
        if is_float_equal(tuple.3, 1.0) {
            true
        } else {
            false
        }
    }

    pub fn vector(a: (f32, f32, f32)) -> (f32, f32, f32, f32) {
        (a.0, a.1, a.2, 0.0)
    }
    pub fn is_vector(tuple: (f32, f32, f32, f32)) -> bool {
        if is_float_equal(tuple.3, 0.0) {
            true
        } else {
            false
        }
    }
    pub fn is_float_equal(a: f32, b: f32) -> bool {
        if (a - b).abs() < EPSILON {
            true
        } else {
            false
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn tuple_with_w_equals_1_is_a_point() {
            let a = (4.3, -4.2, 3.1, 1.0);

            assert!(is_float_equal(a.0, 4.3));
            assert!(is_float_equal(a.1, -4.2));
            assert!(is_float_equal(a.2, 3.1));
            assert!(is_float_equal(a.3, 1.0));
            assert_eq!(is_point(a), true);
            assert_ne!(is_vector(a), true);
        }

        #[test]
        fn tuple_with_w_equals_0_is_a_vector() {
            let a = (4.3, -4.2, 3.1, 0.0);

            assert!(is_float_equal(a.0, 4.3));
            assert!(is_float_equal(a.1, -4.2));
            assert!(is_float_equal(a.2, 3.1));
            assert!(is_float_equal(a.3, 0.0));
            assert_ne!(is_point(a), true);
            assert_eq!(is_vector(a), true);
        }

        #[test]
        fn function_point_creates_tuple_with_w_equal_1() {
            let p = point((4.0, -4.0, 3.0));

            assert_eq!(p, (4.0, -4.0, 3.0, 1.0));
        }

        #[test]
        fn function_vector_creates_tuple_with_w_equal_0() {
            let p = vector((4.0, -4.0, 3.0));

            assert_eq!(p, (4.0, -4.0, 3.0, 0.0));
        }
    }
}
