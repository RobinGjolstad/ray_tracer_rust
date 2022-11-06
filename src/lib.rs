mod Tuples {
    use std::{
        f32::EPSILON,
        ops::{Add, Sub, Neg},
    };

    enum TupleType {
        Point,
        Vector,
        Undefined,
    }

    trait TupleTrait {
        const TYPE_OF: TupleType;
        fn new(input: (f32, f32, f32)) -> Self;
    }

    #[derive(Debug, PartialEq)]
    pub struct Tuple {
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    }

    impl Tuple {
        fn new(input: (f32, f32, f32, f32)) -> Self {
            Tuple {
                x: input.0,
                y: input.1,
                z: input.2,
                w: input.3,
            }
        }
        fn type_of(&self) -> TupleType {
            if is_float_equal(&self.w, 0.0) {
                TupleType::Vector
            } else if is_float_equal(&self.w, 1.0) {
                TupleType::Point
            } else {
                TupleType::Undefined
            }
        }
    }
    impl Add<Tuple> for Tuple {
        type Output = Self;
        fn add(self, rhs: Tuple) -> Self::Output {
            Tuple {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
                w: self.w + rhs.w,
            }
        }
    }
    impl Sub<Tuple> for Tuple {
        type Output = Self;
        fn sub(self, rhs: Tuple) -> Self::Output {
            Tuple {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z,
                w: self.w - rhs.w,
            }
        }
    }
    impl Neg for Tuple {
        type Output = Self;
        fn neg(self) -> Self::Output {
            Tuple {
                x: -self.x,
                y: -self.y,
                z: -self.z,
                w: -self.w,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Point {
        tuple: Tuple,
    }

    impl TupleTrait for Point {
        const TYPE_OF: TupleType = TupleType::Point;

        fn new(input: (f32, f32, f32)) -> Self {
            Point {
                tuple: Tuple {
                    x: input.0,
                    y: input.1,
                    z: input.2,
                    w: 1.0,
                },
            }
        }
    }

    impl Add<Vector> for Point {
        type Output = Self;
        fn add(self, rhs: Vector) -> Self::Output {
            Point {
                tuple: self.tuple + rhs.tuple,
            }
        }
    }
    impl Sub<Point> for Point {
        type Output = Vector;
        fn sub(self, rhs: Point) -> Vector {
            Vector {
                tuple: self.tuple - rhs.tuple,
            }
        }
    }
    impl Sub<Vector> for Point {
        type Output = Self;
        fn sub(self, rhs: Vector) -> Self::Output {
            Point {
                tuple: self.tuple - rhs.tuple,
            }
        }
    }

    #[derive(Debug, PartialEq)]
    pub struct Vector {
        tuple: Tuple,
    }

    impl TupleTrait for Vector {
        const TYPE_OF: TupleType = TupleType::Vector;

        fn new(input: (f32, f32, f32)) -> Self {
            Vector {
                tuple: Tuple {
                    x: input.0,
                    y: input.1,
                    z: input.2,
                    w: 0.0,
                },
            }
        }
    }

    impl Add<Vector> for Vector {
        type Output = Vector;
        fn add(self, rhs: Vector) -> Self::Output {
            Vector {
                tuple: self.tuple + rhs.tuple,
            }
        }
    }

    impl Add<Point> for Vector {
        type Output = Point;
        fn add(self, rhs: Point) -> Point {
            Point {
                tuple: self.tuple + rhs.tuple,
            }
        }
    }

    impl Sub<Vector> for Vector {
        type Output = Self;
        fn sub(self, rhs: Vector) -> Self::Output {
            Vector {
                tuple: self.tuple - rhs.tuple,
            }
        }
    }

    pub fn point(a: (f32, f32, f32)) -> Point {
        Point::new(a)
    }
    pub fn is_point(tuple: &Tuple) -> bool {
        if is_float_equal(&tuple.w, 1.0) {
            true
        } else {
            false
        }
    }

    pub fn vector(a: (f32, f32, f32)) -> Vector {
        Vector::new(a)
    }
    pub fn is_vector(tuple: &Tuple) -> bool {
        if is_float_equal(&tuple.w, 0.0) {
            true
        } else {
            false
        }
    }
    pub fn is_float_equal(actual: &f32, comparison: f32) -> bool {
        if (actual - comparison).abs() < EPSILON {
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
            let tup = Tuple::new(a);

            assert!(is_float_equal(&a.0, 4.3));
            assert!(is_float_equal(&a.1, -4.2));
            assert!(is_float_equal(&a.2, 3.1));
            assert!(is_float_equal(&a.3, 1.0));
            assert_eq!(is_point(&tup), true);
            assert_ne!(is_vector(&tup), true);
        }

        #[test]
        fn tuple_with_w_equals_0_is_a_vector() {
            let a = (4.3, -4.2, 3.1, 0.0);
            let tup = Tuple::new(a);

            assert!(is_float_equal(&a.0, 4.3));
            assert!(is_float_equal(&a.1, -4.2));
            assert!(is_float_equal(&a.2, 3.1));
            assert!(is_float_equal(&a.3, 0.0));
            assert_ne!(is_point(&tup), true);
            assert_eq!(is_vector(&tup), true);
        }

        #[test]
        fn function_point_creates_tuple_with_w_equal_1() {
            let p = point((4.0, -4.0, 3.0));

            assert_eq!(
                (p.tuple.x, p.tuple.y, p.tuple.z, p.tuple.w),
                (4.0, -4.0, 3.0, 1.0)
            );
        }

        #[test]
        fn function_vector_creates_tuple_with_w_equal_0() {
            let p = vector((4.0, -4.0, 3.0));

            assert_eq!(
                (p.tuple.x, p.tuple.y, p.tuple.z, p.tuple.w),
                (4.0, -4.0, 3.0, 0.0)
            );
        }

        #[test]
        fn adding_two_tuples_results_in_a_new_tuple() {
            let a1 = Tuple::new((3.0, -2.0, 5.0, 1.0));
            let a2 = Tuple::new((-2.0, 3.0, 1.0, 0.0));

            assert_eq!(a1 + a2, Tuple::new((1.0, 1.0, 6.0, 1.0)));
        }

        #[test]
        fn subtracting_two_points_results_in_a_vector() {
            let p1 = point((3.0, 2.0, 1.0));
            let p2 = point((5.0, 6.0, 7.0));

            let v1 = vector((-2.0, -4.0, -6.0));

            assert_eq!(p1 - p2, v1);
        }

        #[test]
        fn subtracting_a_vector_from_a_point_results_in_a_point() {
            let p1 = point((3.0, 2.0, 1.0));
            let v = vector((5.0, 6.0, 7.0));

            let p2 = point((-2.0, -4.0, -6.0));

            assert_eq!(p1 - v, p2);
        }

        #[test]
        fn subtracting_two_vectors_results_in_a_vector() {
            let v1 = vector((3.0, 2.0, 1.0));
            let v2 = vector((5.0, 6.0, 7.0));

            let v3 = vector((-2.0, -4.0, -6.0));

            assert_eq!(v1 - v2, v3);
        }

        #[test]
        fn subtracting_a_vector_from_the_zero_vector_results_in_a_negative_vector() {
            let zero = vector((0.0, 0.0, 0.0));
            let v = vector((1.0, -2.0, 3.0));

            let zv = vector((-1.0, 2.0, -3.0));

            assert_eq!(zero - v, zv);
        }

        #[test]
        fn negating_a_vector_reverses_the_sign_of_each_element() {
            let a = Tuple::new((1.0, -2.0, 3.0, -4.0));
            let na = Tuple::new((-1.0, 2.0, -3.0, 4.0));

            assert_eq!(-a, na);
        }
    }
}
