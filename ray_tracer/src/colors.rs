use std::ops::{Add, Mul, Sub};

use crate::utils::is_float_equal;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub(crate) red: f64,
    pub(crate) green: f64,
    pub(crate) blue: f64,
}
impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }

    pub fn float_to_u8(color: &f64) -> u8 {
        let col = 255 as f64 * *color;
        col.ceil() as u8
    }
}

impl PartialEq<Color> for Color {
    fn eq(&self, other: &Color) -> bool {
        if is_float_equal(&self.red, other.red)
            && is_float_equal(&self.green, other.green)
            && is_float_equal(&self.blue, other.blue)
        {
            true
        } else {
            false
        }
    }
}
impl PartialEq<Color> for &Color {
    fn eq(&self, other: &Color) -> bool {
        if is_float_equal(&self.red, other.red)
            && is_float_equal(&self.green, other.green)
            && is_float_equal(&self.blue, other.blue)
        {
            true
        } else {
            false
        }
    }
}
impl PartialEq<&Color> for Color {
    fn eq(&self, other: &&Color) -> bool {
        if is_float_equal(&self.red, other.red)
            && is_float_equal(&self.green, other.green)
            && is_float_equal(&self.blue, other.blue)
        {
            true
        } else {
            false
        }
    }
}
impl Add<Color> for Color {
    type Output = Self;
    fn add(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl Sub<Color> for Color {
    type Output = Color;
    fn sub(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            red: self.red * rhs.red,
            green: self.green * rhs.green,
            blue: self.blue * rhs.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
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

    #[test]
    fn adding_colors_adds_each_element() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let c12 = Color::new(1.6, 0.7, 1.0);
        assert_eq!(c1 + c2, c12);
    }

    #[test]
    fn subtracting_colors_subtracts_each_element() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);

        let c12 = Color::new(0.2, 0.5, 0.5);
        assert_eq!(c1 - c2, c12);
    }

    #[test]
    fn multiplying_a_color_with_a_scalar_multiplies_each_element_with_a_scalar() {
        let c = Color::new(0.2, 0.3, 0.4);

        let c2 = Color::new(0.4, 0.6, 0.8);
        assert_eq!(c * 2.0, c2);
    }

    #[test]
    fn multiplying_two_colors_multiplies_their_elements() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);

        let c12 = Color::new(0.9, 0.2, 0.04);
        assert_eq!(c1 * c2, c12);
    }
}
