mod Tuples {
    use std::{f32::EPSILON, ops::Add};

    enum TupleType {
        Point,
        Vector,
        Undefined
    }

    trait TupleTrait {
        const TYPE_OF: TupleType;
        fn new(input: (f32, f32, f32)) -> Self;
    }

    #[derive(Debug)]
    pub struct Tuple {
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    }


    impl Tuple {
        fn new(input: (f32, f32, f32, f32)) -> Self {
            Tuple { x: input.0, y: input.1, z: input.2, w: input.3 }
        }
    }
    
    
    #[derive(Debug)]
    pub struct Point {
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    }

    impl TupleTrait for Point {
        const TYPE_OF: TupleType = TupleType::Point;

        fn new(input: (f32, f32, f32)) -> Self {
            Point {
                x: input.0,
                y: input.1,
                z: input.2,
                w: 1.0,
            }
        }
    }

    impl Add<Vector> for Point {
        type Output = Self;
        fn add(self, rhs: Vector) -> Self::Output {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
                w: self.w + rhs.w,
            }
        }
    }

    #[derive(Debug)]
    pub struct Vector {
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    }

    impl TupleTrait for Vector {
        const TYPE_OF: TupleType = TupleType::Vector;

        fn new(input: (f32, f32, f32)) -> Self {
            Vector {
                x: input.0,
                y: input.1,
                z: input.2,
                w: 0.0,
            }
        }
    }

    impl Add<Vector> for Vector {
        type Output = Vector;
        fn add(self, rhs: Vector) -> Self::Output {
            Vector {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
                w: self.w + rhs.w,
            }
        }
    }

    impl Add<Point> for Vector {
        type Output = Point;
        fn add(self, rhs: Point) -> Point {
            Point {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
                w: self.w + rhs.w,
            }
        }
    }

    pub fn point(a: (f32, f32, f32)) -> Point {
        Point::new(a)
    }
    pub fn is_point(tuple: (f32, f32, f32, f32)) -> bool {
        if is_float_equal(tuple.3, 1.0) {
            true
        } else {
            false
        }
    }

    pub fn vector(a: (f32, f32, f32)) -> Vector {
        Vector::new(a)
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

            assert_eq!((p.x, p.y, p.z, p.w), (4.0, -4.0, 3.0, 1.0));
        }

        #[test]
        fn function_vector_creates_tuple_with_w_equal_0() {
            let p = vector((4.0, -4.0, 3.0));

            assert_eq!((p.x, p.y, p.z, p.w), (4.0, -4.0, 3.0, 0.0));
        }

        #[test]
        fn adding_two_tuples_results_in_a_new_tuple() {
            let a1 = 
        }
    }
}
