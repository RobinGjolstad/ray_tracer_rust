use std::ops::{Add, Sub};

use crate::{
    points::Point,
    tuples::{Tuple, TupleTrait, TupleType},
};

////////////////////////////////////////////////////////////////////////////
// Vector-land!
////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Vector {
    pub tuple: Tuple,
}

impl Vector {
    pub fn magnitude(&self) -> f32 {
        let pow_x = f64::powi(self.tuple.x as f64, 2);
        let pow_y = f64::powi(self.tuple.y as f64, 2);
        let pow_z = f64::powi(self.tuple.z as f64, 2);
        let pow_w = f64::powi(self.tuple.w as f64, 2);
        let sum = pow_x + pow_y + pow_z + pow_w;
        f64::sqrt(sum) as f32
    }
    pub fn normalize(&self) -> Self {
        Vector {
            tuple: Tuple::new((
                self.tuple.x / self.magnitude(),
                self.tuple.y / self.magnitude(),
                self.tuple.z / self.magnitude(),
                self.tuple.w / self.magnitude(),
            )),
        }
    }
    pub fn dot(a: &Self, b: &Self) -> f32 {
        let x = a.tuple.x * b.tuple.x;
        let y = a.tuple.y * b.tuple.y;
        let z = a.tuple.z * b.tuple.z;
        let w = a.tuple.w * b.tuple.w;

        return x + y + z + w;
    }
    pub fn cross(a: &Self, b: &Self) -> Self {
        Vector::new((
            a.tuple.y * b.tuple.z - a.tuple.z * b.tuple.y,
            a.tuple.z * b.tuple.x - a.tuple.x * b.tuple.z,
            a.tuple.x * b.tuple.y - a.tuple.y * b.tuple.x,
        ))
    }
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
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            tuple: self.tuple - rhs.tuple,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::is_float_equal;

    #[test]
    fn function_vector_creates_tuple_with_w_equal_0() {
        let p = Vector::new((4.0, -4.0, 3.0));

        assert_eq!(
            (p.tuple.x, p.tuple.y, p.tuple.z, p.tuple.w),
            (4.0, -4.0, 3.0, 0.0)
        );
    }

    #[test]
    fn subtracting_two_vectors_results_in_a_vector() {
        let v1 = Vector::new((3.0, 2.0, 1.0));
        let v2 = Vector::new((5.0, 6.0, 7.0));

        let v3 = Vector::new((-2.0, -4.0, -6.0));

        assert_eq!(v1 - v2, v3);
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector_results_in_a_negative_vector() {
        let zero = Vector::new((0.0, 0.0, 0.0));
        let v = Vector::new((1.0, -2.0, 3.0));

        let zv = Vector::new((-1.0, 2.0, -3.0));

        assert_eq!(zero - v, zv);
    }

    #[test]
    fn magnitude_of_vector_1_0_0_is_1() {
        let v = Vector::new((1.0, 0.0, 0.0));

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_0_1_0_is_1() {
        let v = Vector::new((0.0, 1.0, 0.0));

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_0_0_1_is_1() {
        let v = Vector::new((0.0, 0.0, 1.0));

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_1_2_3_is_sqrt14() {
        let v = Vector::new((1.0, 2.0, 3.0));

        assert_eq!(v.magnitude(), f32::sqrt(14.0));
    }

    #[test]
    fn magnitude_of_vector_neg_1_2_3_is_sqrt14() {
        let v = Vector::new((-1.0, -2.0, -3.0));

        assert_eq!(v.magnitude(), f32::sqrt(14.0));
    }

    #[test]
    fn normalizing_a_vector_4_0_0_gives_1_0_0() {
        let v = Vector::new((4.0, 0.0, 0.0));
        let normalized_v = v.normalize();
        let unit_v = Vector::new((1.0, 0.0, 0.0));

        assert_eq!(normalized_v, unit_v);
    }

    #[test]
    fn normalizing_a_vector_1_2_3_gives_1sqrt14_2sqrt14_3sqrt14() {
        let v = Vector::new((1.0, 2.0, 3.0));
        let normalized_v = v.normalize();

        let unit_v = Vector::new((
            1.0 / 14.0_f32.sqrt(),
            2.0 / 14.0_f32.sqrt(),
            3.0 / 14.0_f32.sqrt(),
        ));

        assert_eq!(normalized_v, unit_v);
    }

    #[test]
    fn the_magnitude_of_a_normalized_vector_is_1() {
        let v = Vector::new((1.0, 2.0, 3.0));
        let norm = v.normalize();
        let norm_mag = norm.magnitude();

        assert!(is_float_equal(&norm_mag, 1.0));
    }

    #[test]
    fn the_dot_product_of_two_vectors_equals_the_sum_of_each_component_multiplied() {
        let a = Vector::new((1.0, 2.0, 3.0));
        let b = Vector::new((2.0, 3.0, 4.0));

        assert!(is_float_equal(&Vector::dot(&a, &b), 20.0));
    }

    #[test]
    fn the_cross_product_of_two_vectors() {
        let a = Vector::new((1.0, 2.0, 3.0));
        let b = Vector::new((2.0, 3.0, 4.0));

        assert_eq!(Vector::cross(&a, &b), Vector::new((-1.0, 2.0, -1.0)));
        assert_eq!(Vector::cross(&b, &a), Vector::new((1.0, -2.0, 1.0)));
    }
}
