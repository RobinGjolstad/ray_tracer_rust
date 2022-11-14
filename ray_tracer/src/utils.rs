use std::f32::EPSILON;

pub fn is_float_equal(actual: &f32, comparison: f32) -> bool {
    if (actual - comparison).abs() < EPSILON {
        true
    } else {
        false
    }
}