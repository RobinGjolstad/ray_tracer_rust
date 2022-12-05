use crate::utils::is_float_equal;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Tuple {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

pub type Vector = Tuple;
pub type Point = Tuple;

impl Tuple {
    ////////////////////////////////////////////////////////////////////////////
    // Generic tuple things
    ////////////////////////////////////////////////////////////////////////////
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
    pub fn new_tuple(input: (f32, f32, f32, f32)) -> Self {
        Tuple {
            x: input.0,
            y: input.1,
            z: input.2,
            w: input.3,
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // Point-land!
    ////////////////////////////////////////////////////////////////////////////
    pub fn new_point(x: f32, y: f32, z: f32) -> Self {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: 1.0,
        }
    }

    ////////////////////////////////////////////////////////////////////////////
    // Vector-land!
    ////////////////////////////////////////////////////////////////////////////
    pub fn new_vector(x: f32, y: f32, z: f32) -> Self {
        Tuple {
            x: x,
            y: y,
            z: z,
            w: 0.0,
        }
    }
    pub fn magnitude(&self) -> f32 {
        assert_eq!(self.w, 0.0, "Magnitude is only valid for vectors!");
        let pow_x = f64::powi(self.x as f64, 2);
        let pow_y = f64::powi(self.y as f64, 2);
        let pow_z = f64::powi(self.z as f64, 2);
        let pow_w = f64::powi(self.w as f64, 2);
        let sum = pow_x + pow_y + pow_z + pow_w;
        f64::sqrt(sum) as f32
    }
    pub fn normalize(&self) -> Self {
        assert_eq!(self.w, 0.0, "Normalize is only valid for vectors!");
        Tuple::new(
            self.x / self.magnitude(),
            self.y / self.magnitude(),
            self.z / self.magnitude(),
            self.w / self.magnitude(),
        )
    }
    pub fn dot(a: &Self, b: &Self) -> f32 {
        assert_eq!(a.w, 0.0, "Dot-product is only valid for vectors!");
        assert_eq!(b.w, 0.0, "Dot-product is only valid for vectors!");
        let x = a.x * b.x;
        let y = a.y * b.y;
        let z = a.z * b.z;
        let w = a.w * b.w;

        return x + y + z + w;
    }
    pub fn cross(a: &Self, b: &Self) -> Self {
        assert_eq!(a.w, 0.0, "Cross-product is only valid for vectors!");
        assert_eq!(b.w, 0.0, "Cross-product is only valid for vectors!");
        Tuple::new_vector(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }

    pub fn is_point(&self) -> bool {
        if is_float_equal(&self.w, 1.0) {
            true
        } else {
            false
        }
    }

    pub fn is_vector(&self) -> bool {
        if is_float_equal(&self.w, 0.0) {
            true
        } else {
            false
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

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        if is_float_equal(&self.x, other.x)
            && is_float_equal(&self.y, other.y)
            && is_float_equal(&self.z, other.z)
            && is_float_equal(&self.w, other.w)
        {
            true
        } else {
            false
        }
    }
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_tuple_with_w_equals_1_is_a_point() {
        let a = (4.3, -4.2, 3.1, 1.0);
        let tup = Tuple::new_tuple(a);

        assert!(is_float_equal(&a.0, 4.3));
        assert!(is_float_equal(&a.1, -4.2));
        assert!(is_float_equal(&a.2, 3.1));
        assert!(is_float_equal(&a.3, 1.0));
        assert_eq!(tup.is_point(), true);
        assert_ne!(tup.is_vector(), true);
    }

    #[test]
    fn a_tuple_with_w_equals_0_is_a_vector() {
        let a = (4.3, -4.2, 3.1, 0.0);
        let tup = Tuple::new_tuple(a);

        assert!(is_float_equal(&a.0, 4.3));
        assert!(is_float_equal(&a.1, -4.2));
        assert!(is_float_equal(&a.2, 3.1));
        assert!(is_float_equal(&a.3, 0.0));
        assert_ne!(tup.is_point(), true);
        assert_eq!(tup.is_vector(), true);
    }

    #[test]
    fn function_point_creates_tuple_with_w_equal_1() {
        let p = Tuple::new_point(4.0, -4.0, 3.0);

        assert_eq!((p.x, p.y, p.z, p.w), (4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn function_vector_creates_tuple_with_w_equal_0() {
        let p = Tuple::new_vector(4.0, -4.0, 3.0);

        assert_eq!((p.x, p.y, p.z, p.w), (4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn adding_two_tuples_results_in_a_new_tuple() {
        let a1 = Tuple::new_tuple((3.0, -2.0, 5.0, 1.0));
        let a2 = Tuple::new_tuple((-2.0, 3.0, 1.0, 0.0));

        assert_eq!(a1 + a2, Tuple::new_tuple((1.0, 1.0, 6.0, 1.0)));
    }

    #[test]
    fn subtracting_two_points_results_in_a_vector() {
        let p1 = Tuple::new_point(3.0, 2.0, 1.0);
        let p2 = Tuple::new_point(5.0, 6.0, 7.0);

        let v1 = Tuple::new_vector(-2.0, -4.0, -6.0);

        assert_eq!(p1 - p2, v1);
    }

    #[test]
    fn subtracting_a_vector_from_a_point_results_in_a_point() {
        let p1 = Tuple::new_point(3.0, 2.0, 1.0);
        let v = Tuple::new_vector(5.0, 6.0, 7.0);

        let p2 = Tuple::new_point(-2.0, -4.0, -6.0);

        assert_eq!(p1 - v, p2);
    }

    #[test]
    fn subtracting_two_vectors_results_in_a_vector() {
        let v1 = Tuple::new_vector(3.0, 2.0, 1.0);
        let v2 = Tuple::new_vector(5.0, 6.0, 7.0);

        let v3 = Tuple::new_vector(-2.0, -4.0, -6.0);

        assert_eq!(v1 - v2, v3);
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector_results_in_a_negative_vector() {
        let zero = Tuple::new_vector(0.0, 0.0, 0.0);
        let v = Tuple::new_vector(1.0, -2.0, 3.0);

        let zv = Tuple::new_vector(-1.0, 2.0, -3.0);

        assert_eq!(zero - v, zv);
    }

    #[test]
    fn negating_a_tuple_reverses_the_sign_of_each_element() {
        let a = Tuple::new_tuple((1.0, -2.0, 3.0, -4.0));
        let na = Tuple::new_tuple((-1.0, 2.0, -3.0, 4.0));

        assert_eq!(-a, na);
    }

    #[test]
    fn multiplying_a_tuple_with_a_scalar_multiplies_each_element_by_a_scalar() {
        let a = Tuple::new_tuple((1.0, -2.0, 3.0, -4.0));
        let ma = Tuple::new_tuple((3.5, -7.0, 10.5, -14.0));

        assert_eq!(a * 3.5, ma);
    }

    #[test]
    fn multiplying_a_tuple_with_a_fraction_multiplies_each_element_by_a_fraction() {
        let a = Tuple::new_tuple((1.0, -2.0, 3.0, -4.0));
        let fa = Tuple::new_tuple((0.5, -1.0, 1.5, -2.0));

        assert_eq!(a * 0.5, fa);
    }

    #[test]
    fn dividing_a_tuple_with_a_scalar_divides_each_element_by_a_scalar() {
        let a = Tuple::new_tuple((1.0, -2.0, 3.0, -4.0));
        let da = Tuple::new_tuple((0.5, -1.0, 1.5, -2.0));

        assert_eq!(a / 2.0, da);
    }

    #[test]
    fn magnitude_of_vector_1_0_0_is_1() {
        let v = Tuple::new_vector(1.0, 0.0, 0.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_0_1_0_is_1() {
        let v = Tuple::new_vector(0.0, 1.0, 0.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_0_0_1_is_1() {
        let v = Tuple::new_vector(0.0, 0.0, 1.0);

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_1_2_3_is_sqrt14() {
        let v = Tuple::new_vector(1.0, 2.0, 3.0);

        assert_eq!(v.magnitude(), f32::sqrt(14.0));
    }

    #[test]
    fn magnitude_of_vector_neg_1_2_3_is_sqrt14() {
        let v = Tuple::new_vector(-1.0, -2.0, -3.0);

        assert_eq!(v.magnitude(), f32::sqrt(14.0));
    }

    #[test]
    fn normalizing_a_vector_4_0_0_gives_1_0_0() {
        let v = Tuple::new_vector(4.0, 0.0, 0.0);
        let normalized_v = v.normalize();
        let unit_v = Tuple::new_vector(1.0, 0.0, 0.0);

        assert_eq!(normalized_v, unit_v);
    }

    #[test]
    fn normalizing_a_vector_1_2_3_gives_1sqrt14_2sqrt14_3sqrt14() {
        let v = Tuple::new_vector(1.0, 2.0, 3.0);
        let normalized_v = v.normalize();

        let unit_v = Tuple::new_vector(
            1.0 / 14.0_f32.sqrt(),
            2.0 / 14.0_f32.sqrt(),
            3.0 / 14.0_f32.sqrt(),
        );

        assert_eq!(normalized_v, unit_v);
    }

    #[test]
    fn the_magnitude_of_a_normalized_vector_is_1() {
        let v = Tuple::new_vector(1.0, 2.0, 3.0);
        let norm = v.normalize();
        let norm_mag = norm.magnitude();

        assert!(is_float_equal(&norm_mag, 1.0));
    }

    #[test]
    fn the_dot_product_of_two_vectors_equals_the_sum_of_each_component_multiplied() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);

        assert!(is_float_equal(&Tuple::dot(&a, &b), 20.0));
    }

    #[test]
    fn the_cross_product_of_two_vectors() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);

        assert_eq!(Tuple::cross(&a, &b), Tuple::new_vector(-1.0, 2.0, -1.0));
        assert_eq!(Tuple::cross(&b, &a), Tuple::new_vector(1.0, -2.0, 1.0));
    }
}
