use crate::{colors::Color, tuples};

use super::Patterns;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Checker {
    color_a: Color,
    color_b: Color,
}

impl Checker {
    pub(super) fn new(color_a: Color, color_b: Color) -> Self {
        Self { color_a, color_b }
    }
}

impl Default for Checker {
    fn default() -> Self {
        Self {
            color_a: Color::new(1.0, 1.0, 1.0),
            color_b: Color::new(0.0, 0.0, 0.0),
        }
    }
}

impl Patterns for Checker {
    fn color_at(&self, point: tuples::Point) -> Color {
        if (point.x.floor() + point.y.floor() + point.z.floor()) as usize % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{colors::Color, patterns::Pattern, tuples::Tuple};

    const WHITE: Color = Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    };
    const BLACK: Color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };

    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = Pattern::checker(WHITE, BLACK);
        assert_eq!(pattern.pattern_at(Tuple::new_point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(Tuple::new_point(0.99, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(Tuple::new_point(1.01, 0.0, 0.0)), BLACK);
    }
    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = Pattern::checker(WHITE, BLACK);
        assert_eq!(pattern.pattern_at(Tuple::new_point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(Tuple::new_point(0.0, 0.99, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(Tuple::new_point(0.0, 1.01, 0.0)), BLACK);
    }
    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = Pattern::checker(WHITE, BLACK);
        assert_eq!(pattern.pattern_at(Tuple::new_point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(Tuple::new_point(0.0, 0.0, 0.99)), WHITE);
        assert_eq!(pattern.pattern_at(Tuple::new_point(0.0, 0.0, 1.01)), BLACK);
    }
}
