use std::ops::{Add, Sub};

use crate::{
    tuples::{Tuple, TupleTrait, TupleType},
    vectors::Vector,
};

////////////////////////////////////////////////////////////////////////////
// Point-land!
////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Point {
    pub tuple: Tuple,
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
    type Output = Point;
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
    type Output = Point;
    fn sub(self, rhs: Vector) -> Self::Output {
        Point {
            tuple: self.tuple - rhs.tuple,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn function_point_creates_tuple_with_w_equal_1() {
        let p = Point::new((4.0, -4.0, 3.0));

        assert_eq!(
            (p.tuple.x, p.tuple.y, p.tuple.z, p.tuple.w),
            (4.0, -4.0, 3.0, 1.0)
        );
    }

    #[test]
    fn subtracting_two_points_results_in_a_vector() {
        let p1 = Point::new((3.0, 2.0, 1.0));
        let p2 = Point::new((5.0, 6.0, 7.0));

        let v1 = Vector::new((-2.0, -4.0, -6.0));

        assert_eq!(p1 - p2, v1);
    }

    #[test]
    fn subtracting_a_vector_from_a_point_results_in_a_point() {
        let p1 = Point::new((3.0, 2.0, 1.0));
        let v = Vector::new((5.0, 6.0, 7.0));

        let p2 = Point::new((-2.0, -4.0, -6.0));

        assert_eq!(p1 - v, p2);
    }
}
