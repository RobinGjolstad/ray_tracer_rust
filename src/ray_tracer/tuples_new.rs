#![allow(clippy::pedantic)]

use std::{
    f64,
    ops::{Add, Div, Mul, Neg, Sub},
};

use super::utils::is_float_equal_low_precision;

pub const fn new_vector(x: f64, y: f64, z: f64) -> Vector {
    Vector::new(x, y, z)
}
pub const fn new_point(x: f64, y: f64, z: f64) -> Point {
    Point::new(x, y, z)
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vector {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn magnitude(&self) -> f64 {
        let pow_x = f64::powi(self.x, 2);
        let pow_y = f64::powi(self.y, 2);
        let pow_z = f64::powi(self.z, 2);
        let sum = pow_x + pow_y + pow_z;
        f64::sqrt(sum)
    }
    pub fn normalize(&self) -> Self {
        Self::new(
            self.x / self.magnitude(),
            self.y / self.magnitude(),
            self.z / self.magnitude(),
        )
    }
    pub fn dot(a: &Self, b: &Self) -> f64 {
        let x = a.x * b.x;
        let y = a.y * b.y;
        let z = a.z * b.z;

        x + y + z
    }
    pub fn cross(a: &Self, b: &Self) -> Self {
        // Self::new_vector(
        //     a.y * b.z - a.z * b.y,
        //     a.z * b.x - a.x * b.z,
        //     a.x * b.y - a.y * b.x,
        // )
        Self {
            x: a.y.mul_add(b.z, -(a.z * b.y)),
            y: a.z.mul_add(b.x, -(a.x * b.z)),
            z: a.x.mul_add(b.y, -(a.y * b.x)),
        }
    }
    pub fn reflect(vector: &Self, normal: &Self) -> Self {
        *vector - *normal * 2.0 * Self::dot(vector, normal)
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Point {
    pub const fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
}

impl Add<Self> for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Add<Vector> for Point {
    type Output = Self;
    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Add<Self> for Vector {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl Add<Point> for Vector {
    type Output = Point;
    fn add(self, rhs: Point) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub<Self> for Point {
    type Output = Vector;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Sub<Vector> for Point {
    type Output = Self;
    fn sub(self, rhs: Vector) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Sub<Self> for Vector {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}
impl Sub<Point> for Vector {
    type Output = Self;
    fn sub(self, rhs: Point) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for Vector {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}
impl Neg for Point {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
impl Mul<f64> for Point {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}
impl Div<f64> for Point {
    type Output = Self;
    fn div(self, rhs: f64) -> Self::Output {
        Self::Output {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        is_float_equal_low_precision(&self.x, other.x)
            && is_float_equal_low_precision(&self.y, other.y)
            && is_float_equal_low_precision(&self.z, other.z)
    }
}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        is_float_equal_low_precision(&self.x, other.x)
            && is_float_equal_low_precision(&self.y, other.y)
            && is_float_equal_low_precision(&self.z, other.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adding_two_points_results_in_a_point() {
        let p1 = new_point(1.0, 2.0, 3.0);
        let p2 = new_point(4.0, 5.0, 6.0);

        let p3 = new_point(5.0, 7.0, 9.0);

        assert_eq!(p1 + p2, p3);
        assert_eq!(p2 + p1, p3);
    }

    #[test]
    fn adding_a_point_and_a_vector_results_in_a_point() {
        let p1 = new_point(1.0, 2.0, 3.0);
        let v = new_vector(4.0, 5.0, 6.0);

        let p2 = new_point(5.0, 7.0, 9.0);

        assert_eq!(p1 + v, p2);
        assert_eq!(v + p1, p2);
    }

    #[test]
    fn adding_two_vectors_results_in_a_vector() {
        let v1 = new_vector(1.0, 2.0, 3.0);
        let v2 = new_vector(4.0, 5.0, 6.0);

        let v3 = new_vector(5.0, 7.0, 9.0);

        assert_eq!(v1 + v2, v3);
        assert_eq!(v2 + v1, v3);
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
    fn subtracting_a_point_from_a_vector_results_in_a_vector() {
        let p = new_point(3.0, 2.0, 1.0);
        let v1 = new_vector(5.0, 6.0, 7.0);

        let v2 = new_vector(2.0, 4.0, 6.0);

        assert_eq!(v1 - p, v2);
    }

    #[test]
    fn subtracting_a_vector_from_the_zero_vector_results_in_a_negative_vector() {
        let zero = new_vector(0.0, 0.0, 0.0);
        let v = new_vector(1.0, -2.0, 3.0);

        let zv = new_vector(-1.0, 2.0, -3.0);

        assert_eq!(zero - v, zv);
    }

    #[test]
    fn subtracting_two_vectors_results_in_a_vector() {
        let v1 = new_vector(3.0, 2.0, 1.0);
        let v2 = new_vector(5.0, 6.0, 7.0);

        let v3 = new_vector(-2.0, -4.0, -6.0);

        assert_eq!(v1 - v2, v3);
    }

    #[test]
    fn negating_a_vector_reverses_the_sign_of_each_element() {
        let v = new_vector(1.0, -2.0, 3.0);
        let nv = new_vector(-1.0, 2.0, -3.0);

        assert_eq!(-v, nv);
    }
    #[test]
    fn negating_a_point_reverses_the_sign_of_each_element() {
        let p = new_point(1.0, -2.0, 3.0);
        let np = new_point(-1.0, 2.0, -3.0);

        assert_eq!(-p, np);
    }

    #[test]
    fn multiplying_a_vector_with_a_scalar_multiplies_each_element_by_a_scalar() {
        let v = new_vector(1.0, -2.0, 3.0);
        let mv = new_vector(3.5, -7.0, 10.5);

        assert_eq!(v * 3.5, mv);
    }
    #[test]
    fn multiplying_a_point_with_a_scalar_multiplies_each_element_by_a_scalar() {
        let p = new_point(1.0, -2.0, 3.0);
        let mp = new_point(3.5, -7.0, 10.5);

        assert_eq!(p * 3.5, mp);
    }

    #[test]
    fn multiplying_a_vector_with_a_fraction_multiplies_each_element_by_a_fraction() {
        let a = new_vector(1.0, -2.0, 3.0);
        let fa = new_vector(0.5, -1.0, 1.5);

        assert_eq!(a * 0.5, fa);
    }
    #[test]
    fn multiplying_a_point_with_a_fraction_multiplies_each_element_by_a_fraction() {
        let a = new_point(1.0, -2.0, 3.0);
        let fa = new_point(0.5, -1.0, 1.5);

        assert_eq!(a * 0.5, fa);
    }

    #[test]
    fn dividing_a_vector_with_a_scalar_divides_each_element_by_a_scalar() {
        let a = new_vector(1.0, -2.0, 3.0);
        let da = new_vector(0.5, -1.0, 1.5);

        assert_eq!(a / 2.0, da);
    }
    #[test]
    fn dividing_a_point_with_a_scalar_divides_each_element_by_a_scalar() {
        let a = new_point(1.0, -2.0, 3.0);
        let da = new_point(0.5, -1.0, 1.5);

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

        assert!(is_float_equal_low_precision(&Vector::dot(&a, &b), 20.0));
    }

    #[test]
    fn the_cross_product_of_two_vectors() {
        let a = new_vector(1.0, 2.0, 3.0);
        let b = new_vector(2.0, 3.0, 4.0);

        assert_eq!(Vector::cross(&a, &b), new_vector(-1.0, 2.0, -1.0));
        assert_eq!(Vector::cross(&b, &a), new_vector(1.0, -2.0, 1.0));
    }

    #[test]
    fn reflecting_a_vector_approaching_at_45_degrees() {
        let v = new_vector(1.0, -1.0, 0.0);
        let n = new_vector(0.0, 1.0, 0.0);
        let r = Vector::reflect(&v, &n);
        assert_eq!(r, new_vector(1.0, 1.0, 0.0));
    }
    #[test]
    fn reflecting_a_vector_off_a_slanted_surface() {
        let v = new_vector(0.0, -1.0, 0.0);
        let n = new_vector(f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0, 0.0);
        let r = Vector::reflect(&v, &n);
        assert_eq!(r, new_vector(1.0, 0.0, 0.0));
    }
}
