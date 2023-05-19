/// A fixed value used for comparing f64
pub const EPSILON: f64 = 0.00005;

/// Compares two f64 and asserts whether they are within a difference defined by `utils::EPSILON`
pub fn is_float_equal(actual: &f64, comparison: f64) -> bool {
    (actual - comparison).abs() < EPSILON
}

pub(crate) struct F64 {
    float: f64,
}

impl PartialEq for F64 {
    fn eq(&self, other: &Self) -> bool {
        is_float_equal(&self.float, other.float)
    }
}
