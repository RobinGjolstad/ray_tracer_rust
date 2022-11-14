use std::{
    ops::{Add, Div, Mul, Neg, Sub},
};
use crate::utils::is_float_equal as is_float_equal;

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
impl Add<&Tuple> for Tuple {
    type Output = Self;
    fn add(self, rhs: &Tuple) -> Self::Output {
        Tuple {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}
impl Add<&Tuple> for &Tuple {
    type Output = Tuple;
    fn add(self, rhs: &Tuple) -> Self::Output {
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
impl Sub<Tuple> for &Tuple {
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
impl Sub<&Tuple> for Tuple {
    type Output = Tuple;
    fn sub(self, rhs: &Tuple) -> Self::Output {
        Tuple {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}
impl Sub<&Tuple> for &Tuple {
    type Output = Tuple;
    fn sub(self, rhs: &Tuple) -> Self::Output {
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
impl Add<&Vector> for Point {
    type Output = Point;
    fn add(self, rhs: &Vector) -> Self::Output {
        Point {
            tuple: self.tuple + rhs.tuple,
        }
    }
}
impl Add<Vector> for &Point {
    type Output = Point;
    fn add(self, rhs: Vector) -> Self::Output {
        Point {
            tuple: self.tuple + rhs.tuple,
        }
    }
}
impl Add<&Vector> for &Point {
    type Output = Point;
    fn add(self, rhs: &Vector) -> Self::Output {
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
impl Sub<&Point> for Point {
    type Output = Vector;
    fn sub(self, rhs: &Point) -> Vector {
        Vector {
            tuple: self.tuple - rhs.tuple,
        }
    }
}
impl Sub<Point> for &Point {
    type Output = Vector;
    fn sub(self, rhs: Point) -> Vector {
        Vector {
            tuple: self.tuple - rhs.tuple,
        }
    }
}
impl Sub<&Point> for &Point {
    type Output = Vector;
    fn sub(self, rhs: &Point) -> Vector {
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
impl Sub<&Vector> for Point {
    type Output = Point;
    fn sub(self, rhs: &Vector) -> Self::Output {
        Point {
            tuple: self.tuple - rhs.tuple,
        }
    }
}
impl Sub<Vector> for &Point {
    type Output = Point;
    fn sub(self, rhs: Vector) -> Self::Output {
        Point {
            tuple: self.tuple - rhs.tuple,
        }
    }
}
impl Sub<&Vector> for &Point {
    type Output = Point;
    fn sub(self, rhs: &Vector) -> Self::Output {
        Point {
            tuple: self.tuple - rhs.tuple,
        }
    }
}

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
        vector((
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
impl Add<&Vector> for Vector {
    type Output = Vector;
    fn add(self, rhs: &Vector) -> Self::Output {
        Vector {
            tuple: self.tuple + rhs.tuple,
        }
    }
}
impl Add<Vector> for &Vector {
    type Output = Vector;
    fn add(self, rhs: Vector) -> Self::Output {
        Vector {
            tuple: self.tuple + rhs.tuple,
        }
    }
}
impl Add<&Vector> for &Vector {
    type Output = Vector;
    fn add(self, rhs: &Vector) -> Self::Output {
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
impl Add<&Point> for Vector {
    type Output = Point;
    fn add(self, rhs: &Point) -> Point {
        Point {
            tuple: self.tuple + rhs.tuple,
        }
    }
}
impl Add<Point> for &Vector {
    type Output = Point;
    fn add(self, rhs: Point) -> Point {
        Point {
            tuple: self.tuple + rhs.tuple,
        }
    }
}
impl Add<&Point> for &Vector {
    type Output = Point;
    fn add(self, rhs: &Point) -> Point {
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
impl Sub<&Vector> for Vector {
    type Output = Vector;
    fn sub(self, rhs: &Vector) -> Self::Output {
        Vector {
            tuple: self.tuple - rhs.tuple,
        }
    }
}
impl Sub<Vector> for &Vector {
    type Output = Vector;
    fn sub(self, rhs: Vector) -> Self::Output {
        Vector {
            tuple: self.tuple - rhs.tuple,
        }
    }
}
impl Sub<&Vector> for &Vector {
    type Output = Vector;
    fn sub(self, rhs: &Vector) -> Self::Output {
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

    #[test]
    fn magnitude_of_vector_1_0_0_is_1() {
        let v = vector((1.0, 0.0, 0.0));

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_0_1_0_is_1() {
        let v = vector((0.0, 1.0, 0.0));

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_0_0_1_is_1() {
        let v = vector((0.0, 0.0, 1.0));

        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_1_2_3_is_sqrt14() {
        let v = vector((1.0, 2.0, 3.0));

        assert_eq!(v.magnitude(), f32::sqrt(14.0));
    }

    #[test]
    fn magnitude_of_vector_neg_1_2_3_is_sqrt14() {
        let v = vector((-1.0, -2.0, -3.0));

        assert_eq!(v.magnitude(), f32::sqrt(14.0));
    }

    #[test]
    fn normalizing_a_vector_4_0_0_gives_1_0_0() {
        let v = vector((4.0, 0.0, 0.0));
        let normalized_v = v.normalize();
        let unit_v = vector((1.0, 0.0, 0.0));

        assert_eq!(normalized_v, unit_v);
    }

    #[test]
    fn normalizing_a_vector_1_2_3_gives_1sqrt14_2sqrt14_3sqrt14() {
        let v = vector((1.0, 2.0, 3.0));
        let normalized_v = v.normalize();

        let unit_v = vector((
            1.0 / 14.0_f32.sqrt(),
            2.0 / 14.0_f32.sqrt(),
            3.0 / 14.0_f32.sqrt(),
        ));

        assert_eq!(normalized_v, unit_v);
    }

    #[test]
    fn the_magnitude_of_a_normalized_vector_is_1() {
        let v = vector((1.0, 2.0, 3.0));
        let norm = v.normalize();
        let norm_mag = norm.magnitude();

        assert!(is_float_equal(&norm_mag, 1.0));
    }

    #[test]
    fn the_dot_product_of_two_vectors_equals_the_sum_of_each_component_multiplied() {
        let a = vector((1.0, 2.0, 3.0));
        let b = vector((2.0, 3.0, 4.0));

        assert!(is_float_equal(&Vector::dot(&a, &b), 20.0));
    }

    #[test]
    fn the_cross_product_of_two_vectors() {
        let a = vector((1.0, 2.0, 3.0));
        let b = vector((2.0, 3.0, 4.0));

        assert_eq!(Vector::cross(&a, &b), vector((-1.0, 2.0, -1.0)));
        assert_eq!(Vector::cross(&b, &a), vector((1.0, -2.0, 1.0)));
    }
}
