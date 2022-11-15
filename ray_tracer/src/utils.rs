/// A fixed value used for comparing f32
const FLOAT_COMPARISON: f32 = 0.0000005;

/// Compares two f32 and asserts whether they are "equal" or not.
pub fn is_float_equal(actual: &f32, comparison: f32) -> bool {
    if (actual - comparison).abs() < FLOAT_COMPARISON {
        true
    } else {
        false
    }
}
