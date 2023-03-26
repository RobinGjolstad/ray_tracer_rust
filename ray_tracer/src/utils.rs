/// A fixed value used for comparing f64
pub const EPSILON: f64 = 0.00005;

/// Compares two f64 and asserts whether they are "equal" or not.
pub fn is_float_equal(actual: &f64, comparison: f64) -> bool {
     (actual - comparison).abs() < EPSILON 
}
