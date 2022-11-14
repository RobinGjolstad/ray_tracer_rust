
pub struct Color {
    red: f32,
    green: f32,
    blue: f32,
}
impl Color {
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::utils::is_float_equal;

    use super::*;

    #[test]
    fn colors_are_red_green_blue_tuples() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert!(is_float_equal(&c.red, -0.5));
        assert!(is_float_equal(&c.green, 0.4));
        assert!(is_float_equal(&c.blue, 1.7));
    }
}
