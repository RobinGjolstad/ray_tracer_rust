pub mod tuples;

mod tuptup {
    use std::{f32::EPSILON, ops::Add};

    use crate::tuples::tuples;

    enum TupleType {
        Point,
        Vector,
    }
    trait TupTup {
        const type_of: TupleType;
        type tuple;
        fn new(input: Self::tuple) -> Self;
    }

    struct Point {
        x: f32,
        y: f32,
        z: f32,
        w: f32,
    }

    impl TupTup for Point {
        const type_of: TupleType = TupleType::Point;
        type tuple = (f32, f32, f32, f32);
        fn new(input: Self::tuple) -> Self {
            Point { x: input.0, y: input.1, z: input.2, w: input.3 }
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
            let a = Tuple::new(4.3, -4.2, 3.1, 1.0);


        }
    }
}
