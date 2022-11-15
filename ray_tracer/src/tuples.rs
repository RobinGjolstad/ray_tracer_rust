use crate::utils::is_float_equal;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub enum TupleType {
    Point,
    Vector,
    Undefined,
}

pub trait TupleTrait {
    const TYPE_OF: TupleType;
    fn new(input: (f32, f32, f32)) -> Self;
}

////////////////////////////////////////////////////////////////////////////
// Tuple-land
////////////////////////////////////////////////////////////////////////////
#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Tuple {
    pub fn new(input: (f32, f32, f32, f32)) -> Self {
        Tuple {
            x: input.0,
            y: input.1,
            z: input.2,
            w: input.3,
        }
    }
    pub fn type_of(&self) -> TupleType {
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
    type Output = Tuple;
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
    type Output = Tuple;
    fn neg(self) -> Self::Output {
        Tuple {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}
impl Mul<f32> for Tuple {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Tuple {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}
impl Div<f32> for Tuple {
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Tuple {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

pub fn is_point(tuple: &Tuple) -> bool {
    if is_float_equal(&tuple.w, 1.0) {
        true
    } else {
        false
    }
}

pub fn is_vector(tuple: &Tuple) -> bool {
    if is_float_equal(&tuple.w, 0.0) {
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
    fn adding_two_tuples_results_in_a_new_tuple() {
        let a1 = Tuple::new((3.0, -2.0, 5.0, 1.0));
        let a2 = Tuple::new((-2.0, 3.0, 1.0, 0.0));

        assert_eq!(a1 + a2, Tuple::new((1.0, 1.0, 6.0, 1.0)));
    }

    #[test]
    fn negating_a_tuple_reverses_the_sign_of_each_element() {
        let a = Tuple::new((1.0, -2.0, 3.0, -4.0));
        let na = Tuple::new((-1.0, 2.0, -3.0, 4.0));

        assert_eq!(-a, na);
    }

    #[test]
    fn multiplying_a_tuple_with_a_scalar_multiplies_each_element_by_a_scalar() {
        let a = Tuple::new((1.0, -2.0, 3.0, -4.0));
        let ma = Tuple::new((3.5, -7.0, 10.5, -14.0));

        assert_eq!(a * 3.5, ma);
    }

    #[test]
    fn multiplying_a_tuple_with_a_fraction_multiplies_each_element_by_a_fraction() {
        let a = Tuple::new((1.0, -2.0, 3.0, -4.0));
        let fa = Tuple::new((0.5, -1.0, 1.5, -2.0));

        assert_eq!(a * 0.5, fa);
    }

    #[test]
    fn dividing_a_tuple_with_a_scalar_divides_each_element_by_a_scalar() {
        let a = Tuple::new((1.0, -2.0, 3.0, -4.0));
        let da = Tuple::new((0.5, -1.0, 1.5, -2.0));

        assert_eq!(a / 2.0, da);
    }
}
