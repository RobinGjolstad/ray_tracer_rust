
pub mod tuples {
    use std::{f32::EPSILON, ops::Add};

    pub fn is_float_equal(a: f32, b: f32) -> bool {
        if (a - b).abs() < EPSILON {
            true
        } else {
            false
        }
    }

    #[derive(Debug)]
    enum TupleError {
        AddError,
    }

    #[derive(Debug, PartialEq)]
    enum TupleType {
        Point,
        Vector,
    }

    #[derive(Debug, PartialEq)]
    struct Tuple {
        x: f32,
        y: f32,
        z: f32,
        w: f32,
        tuple_type: TupleType,
    }
    impl Tuple {
        fn new((x, y, z, w): (f32, f32, f32, f32)) -> Tuple {
            // Initialize struct
            // Type is determined by `w`
            let mut tuple = Tuple {
                x: x,
                y: y,
                z: z,
                w: w,
                tuple_type: TupleType::Point,
            };
            if is_float_equal(w, 1.0) {
                tuple.tuple_type = TupleType::Point;
            } else {
                tuple.tuple_type = TupleType::Vector;
            }
            tuple
        }
        fn point((x, y, z): (f32, f32, f32)) -> Tuple {
            Tuple {
                x: x,
                y: y,
                z: z,
                w: 1.0,
                tuple_type: TupleType::Point,
            }
        }
        fn vector((x, y, z): (f32, f32, f32)) -> Tuple {
            Tuple {
                x: x,
                y: y,
                z: z,
                w: 0.0,
                tuple_type: TupleType::Vector,
            }
        }
    }

    impl Add for Tuple {
        type Output = Self;

        fn add(self, rhs: Self) -> Self {
            let mut tuple = Self {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
                w: self.w + rhs.w,
                tuple_type: self.tuple_type,
            };
            if is_float_equal(tuple.w, 1.0) {
                tuple.tuple_type = TupleType::Point;
            } else if is_float_equal(tuple.w, 0.0) {
                tuple.tuple_type = TupleType::Vector;
            } else {
                panic!("Addition of two points makes no sense!");
            }

            tuple
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn tuple_with_w_equals_1_is_a_point() {
            let a: Tuple = Tuple::new((4.3, -4.2, 3.1, 1.0));

            assert!(is_float_equal(a.x, 4.3));
            assert!(is_float_equal(a.y, -4.2));
            assert!(is_float_equal(a.z, 3.1));
            assert!(is_float_equal(a.w, 1.0));
            assert_eq!(a.tuple_type, TupleType::Point);
            assert_ne!(a.tuple_type, TupleType::Vector);
        }

        #[test]
        fn tuple_with_w_equals_0_is_a_vector() {
            let a: Tuple = Tuple::new((4.3, -4.2, 3.1, 0.0));

            assert!(is_float_equal(a.x, 4.3));
            assert!(is_float_equal(a.y, -4.2));
            assert!(is_float_equal(a.z, 3.1));
            assert!(is_float_equal(a.w, 0.0));
            assert_ne!(a.tuple_type, TupleType::Point);
            assert_eq!(a.tuple_type, TupleType::Vector);
        }

        #[test]
        fn function_point_creates_tuple_with_w_equal_1() {
            let p = Tuple::point((4.0, -4.0, 3.0));

            assert_eq!(
                p,
                Tuple {
                    x: 4.0,
                    y: -4.0,
                    z: 3.0,
                    w: 1.0,
                    tuple_type: TupleType::Point
                }
            );
        }

        #[test]
        fn function_vector_creates_tuple_with_w_equal_0() {
            let p = Tuple::vector((4.0, -4.0, 3.0));

            assert_eq!(
                p,
                Tuple {
                    x: 4.0,
                    y: -4.0,
                    z: 3.0,
                    w: 0.0,
                    tuple_type: TupleType::Vector
                }
            );
        }

        #[test]
        fn adding_a_point_and_a_vector_creates_a_point() {
            let a1 = Tuple::new((3.0, -2.0, 5.0, 1.0));
            let a2 = Tuple::new((-2.0, 3.0, 1.0, 0.0));

            assert_eq!(a1+a2, Tuple::point((1.0, 1.0, 6.0)));
        }

        #[test]
        fn adding_two_vectors_creates_a_vector(){
            let a1 = Tuple::new((3.0, -2.0, 5.0, 0.0));
            let a2 = Tuple::new((-2.0, 3.0, 1.0, 0.0));

            assert_eq!(a1+a2, Tuple::vector((1.0, 1.0, 6.0)));
        }

        #[test]
        #[should_panic]
        fn adding_two_points_panics(){

            let a1 = Tuple::new((3.0, -2.0, 5.0, 1.0));
            let a2 = Tuple::new((-2.0, 3.0, 1.0, 1.0));

            _ = a1 + a2;
        }
    }
}
