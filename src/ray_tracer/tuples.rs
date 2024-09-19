#![allow(clippy::pedantic)]
use std::ops::{Add, Div, Mul, Neg, Sub};

use super::utils::is_float_equal_low_precision;

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

pub type Vector = Tuple;
pub type Point = Tuple;

pub const fn new_tuple(input: (f64, f64, f64, f64)) -> Tuple {
    Tuple {
        x: input.0,
        y: input.1,
        z: input.2,
        w: input.3,
    }
}
pub const fn new_point(x: f64, y: f64, z: f64) -> Point {
    Point { x, y, z, w: 1.0 }
}
pub const fn new_vector(x: f64, y: f64, z: f64) -> Vector {
    Vector { x, y, z, w: 0.0 }
}

impl Tuple {
    ////////////////////////////////////////////////////////////////////////////
    // Generic tuple things
    ////////////////////////////////////////////////////////////////////////////
    pub const fn new(x: f64, y: f64, z: f64, w: f64) -> Self {
        Self { x, y, z, w }
    }

    ////////////////////////////////////////////////////////////////////////////
    // Vector-land!
    ////////////////////////////////////////////////////////////////////////////
    pub fn magnitude(&self) -> f64 {
        assert_eq!(self.w, 0.0, "Magnitude is only valid for vectors!");
        let pow_x = f64::powi(self.x, 2);
        let pow_y = f64::powi(self.y, 2);
        let pow_z = f64::powi(self.z, 2);
        let pow_w = f64::powi(self.w, 2);
        let sum = pow_x + pow_y + pow_z + pow_w;
        f64::sqrt(sum)
    }
    pub fn normalize(&self) -> Self {
        assert_eq!(self.w, 0.0, "Normalize is only valid for vectors!");
        Self::new(
            self.x / self.magnitude(),
            self.y / self.magnitude(),
            self.z / self.magnitude(),
            self.w / self.magnitude(),
        )
    }
    pub fn dot(a: &Self, b: &Self) -> f64 {
        assert_eq!(a.w, 0.0, "Dot-product is only valid for vectors!");
        assert_eq!(b.w, 0.0, "Dot-product is only valid for vectors!");
        let x = a.x * b.x;
        let y = a.y * b.y;
        let z = a.z * b.z;
        let w = a.w * b.w;

        x + y + z + w
    }
    pub fn cross(a: &Self, b: &Self) -> Self {
        assert_eq!(a.w, 0.0, "Cross-product is only valid for vectors!");
        assert_eq!(b.w, 0.0, "Cross-product is only valid for vectors!");
        // Self::new_vector(
        //     a.y * b.z - a.z * b.y,
        //     a.z * b.x - a.x * b.z,
        //     a.x * b.y - a.y * b.x,
        // )
        new_vector(
            a.y.mul_add(b.z, -(a.z * b.y)),
            a.z.mul_add(b.x, -(a.x * b.z)),
            a.x.mul_add(b.y, -(a.y * b.x)),
        )
    }
    pub fn reflect(vector: &Self, normal: &Self) -> Self {
        *vector - *normal * 2.0 * Self::dot(vector, normal)
    }

    pub fn is_point(&self) -> bool {
        is_float_equal_low_precision(&self.w, 1.0)
    }

    pub fn is_vector(&self) -> bool {
        is_float_equal_low_precision(&self.w, 0.0)
    }
}

impl Add<Self> for Tuple {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl Sub<Self> for Tuple {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
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
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}
impl Mul<f64> for Tuple {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}
impl Div<f64> for Tuple {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs,
        }
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        is_float_equal_low_precision(&self.x, other.x)
            && is_float_equal_low_precision(&self.y, other.y)
            && is_float_equal_low_precision(&self.z, other.z)
            && is_float_equal_low_precision(&self.w, other.w)
    }
}

#[cfg(test)]
mod tests {
    use crate::ray_tracer::utils::is_float_equal_low_precision;

    use super::*;

    #[test]
    fn a_tuple_with_w_equals_1_is_a_point() {
        let a = (4.3, -4.2, 3.1, 1.0);
        let tup = new_tuple(a);

        assert!(is_float_equal_low_precision(&a.0, 4.3));
        assert!(is_float_equal_low_precision(&a.1, -4.2));
        assert!(is_float_equal_low_precision(&a.2, 3.1));
        assert!(is_float_equal_low_precision(&a.3, 1.0));
        assert!(tup.is_point());
        assert!(!tup.is_vector());
    }

    #[test]
    fn a_tuple_with_w_equals_0_is_a_vector() {
        let a = (4.3, -4.2, 3.1, 0.0);
        let tup = new_tuple(a);

        assert!(is_float_equal_low_precision(&a.0, 4.3));
        assert!(is_float_equal_low_precision(&a.1, -4.2));
        assert!(is_float_equal_low_precision(&a.2, 3.1));
        assert!(is_float_equal_low_precision(&a.3, 0.0));
        assert!(!tup.is_point());
        assert!(tup.is_vector());
    }

    #[test]
    fn function_point_creates_tuple_with_w_equal_1() {
        let p = new_point(4.0, -4.0, 3.0);

        assert_eq!((p.x, p.y, p.z, p.w), (4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn function_vector_creates_tuple_with_w_equal_0() {
        let p = new_vector(4.0, -4.0, 3.0);

        assert_eq!((p.x, p.y, p.z, p.w), (4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn adding_two_tuples_results_in_a_new_tuple() {
        let a1 = new_tuple((3.0, -2.0, 5.0, 1.0));
        let a2 = new_tuple((-2.0, 3.0, 1.0, 0.0));

        assert_eq!(a1 + a2, new_tuple((1.0, 1.0, 6.0, 1.0)));
    }

    #[test]
    fn subtracting_two_points_results_in_a_vector() {
        let p1 = new_point(3.0, 2.0, 1.0);
        let p2 = new_point(5.0, 6.0, 7.0);

        let v1 = new_vector(-2.0, -4.0, -6.0);

        assert_eq!(p1 - p2, v1);
    }

    #[test]
    fn subtracting_a_vector_from_a_point_results_in_a_point() {
        let p1 = new_point(3.0, 2.0, 1.0);
        let v = new_vector(5.0, 6.0, 7.0);

        let p2 = new_point(-2.0, -4.0, -6.0);

        assert_eq!(p1 - v, p2);
    }

    #[test]
    fn subtracting_two_vectors_results_in_a_vector() {
        let v1 = new_vector(3.0, 2.0, 1.0);
        let v2 = new_vector(5.0, 6.0, 7.0);

        let v3 = new_vector(-2.0, -4.0, -6.0);

        assert_eq!(v1 - v2, v3);
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector_results_in_a_negative_vector() {
        let zero = new_vector(0.0, 0.0, 0.0);
        let v = new_vector(1.0, -2.0, 3.0);

        let zv = new_vector(-1.0, 2.0, -3.0);

        assert_eq!(zero - v, zv);
    }

    #[test]
    fn negating_a_tuple_reverses_the_sign_of_each_element() {
        let a = new_tuple((1.0, -2.0, 3.0, -4.0));
        let na = new_tuple((-1.0, 2.0, -3.0, 4.0));

        assert_eq!(-a, na);
    }

    #[test]
    fn multiplying_a_tuple_with_a_scalar_multiplies_each_element_by_a_scalar() {
        let a = new_tuple((1.0, -2.0, 3.0, -4.0));
        let ma = new_tuple((3.5, -7.0, 10.5, -14.0));

        assert_eq!(a * 3.5, ma);
    }

    #[test]
    fn multiplying_a_tuple_with_a_fraction_multiplies_each_element_by_a_fraction() {
        let a = new_tuple((1.0, -2.0, 3.0, -4.0));
        let fa = new_tuple((0.5, -1.0, 1.5, -2.0));

        assert_eq!(a * 0.5, fa);
    }

    #[test]
    fn dividing_a_tuple_with_a_scalar_divides_each_element_by_a_scalar() {
        let a = new_tuple((1.0, -2.0, 3.0, -4.0));
        let da = new_tuple((0.5, -1.0, 1.5, -2.0));

        assert_eq!(a / 2.0, da);
    }

    #[test]
    fn magnitude_of_vector_1_0_0_is_1() {
        let v = new_vector(1.0, 0.0, 0.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_0_1_0_is_1() {
        let v = new_vector(0.0, 1.0, 0.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_0_0_1_is_1() {
        let v = new_vector(0.0, 0.0, 1.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_1_2_3_is_sqrt14() {
        let v = new_vector(1.0, 2.0, 3.0);

        assert_eq!(v.magnitude(), f64::sqrt(14.0));
    }

    #[test]
    fn magnitude_of_vector_neg_1_2_3_is_sqrt14() {
        let v = new_vector(-1.0, -2.0, -3.0);

        assert_eq!(v.magnitude(), f64::sqrt(14.0));
    }

    #[test]
    fn normalizing_a_vector_4_0_0_gives_1_0_0() {
        let v = new_vector(4.0, 0.0, 0.0);
        let normalized_v = v.normalize();
        let unit_v = new_vector(1.0, 0.0, 0.0);

        assert_eq!(normalized_v, unit_v);
    }

    #[test]
    fn normalizing_a_vector_1_2_3_gives_1sqrt14_2sqrt14_3sqrt14() {
        let v = new_vector(1.0, 2.0, 3.0);
        let normalized_v = v.normalize();

        let unit_v = new_vector(
            1.0 / 14.0_f64.sqrt(),
            2.0 / 14.0_f64.sqrt(),
            3.0 / 14.0_f64.sqrt(),
        );

        assert_eq!(normalized_v, unit_v);
    }

    #[test]
    fn the_magnitude_of_a_normalized_vector_is_1() {
        let v = new_vector(1.0, 2.0, 3.0);
        let norm = v.normalize();
        let norm_mag = norm.magnitude();

        assert!(is_float_equal_low_precision(&norm_mag, 1.0));
    }

    #[test]
    fn the_dot_product_of_two_vectors_equals_the_sum_of_each_component_multiplied() {
        let a = new_vector(1.0, 2.0, 3.0);
        let b = new_vector(2.0, 3.0, 4.0);

        assert!(is_float_equal_low_precision(&Tuple::dot(&a, &b), 20.0));
    }

    #[test]
    fn the_cross_product_of_two_vectors() {
        let a = new_vector(1.0, 2.0, 3.0);
        let b = new_vector(2.0, 3.0, 4.0);

        assert_eq!(Tuple::cross(&a, &b), new_vector(-1.0, 2.0, -1.0));
        assert_eq!(Tuple::cross(&b, &a), new_vector(1.0, -2.0, 1.0));
    }

    #[test]
    fn reflecting_a_vector_approaching_at_45_degrees() {
        let v = new_vector(1.0, -1.0, 0.0);
        let n = new_vector(0.0, 1.0, 0.0);
        let r = Tuple::reflect(&v, &n);
        assert_eq!(r, new_vector(1.0, 1.0, 0.0));
    }
    #[test]
    fn reflecting_a_vector_off_a_slanted_surface() {
        let v = new_vector(0.0, -1.0, 0.0);
        let n = new_vector(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0);
        let r = Tuple::reflect(&v, &n);
        assert_eq!(r, new_vector(1.0, 0.0, 0.0));
    }
}
