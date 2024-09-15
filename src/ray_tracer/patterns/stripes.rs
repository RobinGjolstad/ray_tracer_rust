use crate::ray_tracer::{colors::Color, tuples::Point};

use super::Patterns;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Stripes {
    color_a: Color,
    color_b: Color,
}

impl Stripes {
    pub(super) const fn new(color_a: Color, color_b: Color) -> Self {
        Self { color_a, color_b }
    }
}

#[cfg(test)]
impl Stripes {
    pub(super) const fn get_colors(&self) -> (Color, Color) {
        (self.color_a, self.color_b)
    }
}

impl Default for Stripes {
    fn default() -> Self {
        Self {
            color_a: Color::new(1.0, 1.0, 1.0),
            color_b: Color::new(0.0, 0.0, 0.0),
        }
    }
}

impl Patterns for Stripes {
    fn color_at(&self, point: Point) -> Color {
        #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
        if point.x.floor() as isize % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ray_tracer::patterns::Pattern;

    use super::*;

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
    fn creating_a_stripe_pattern() {
        let pattern = Pattern::stripe(WHITE, BLACK);
        let (color_a, color_b) = pattern.get_stripe().unwrap().get_colors();
        assert_eq!(color_a, WHITE);
        assert_eq!(color_b, BLACK);
    }
    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = Pattern::stripe(WHITE, BLACK);
        assert_eq!(pattern.pattern_at(Point::new_point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(Point::new_point(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(Point::new_point(0.0, 2.0, 0.0)), WHITE);
    }
    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = Pattern::stripe(WHITE, BLACK);
        assert_eq!(pattern.pattern_at(Point::new_point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(Point::new_point(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.pattern_at(Point::new_point(0.0, 0.0, 2.0)), WHITE);
    }
    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = Pattern::stripe(WHITE, BLACK);
        assert_eq!(pattern.pattern_at(Point::new_point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(Point::new_point(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(Point::new_point(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.pattern_at(Point::new_point(-0.1, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.pattern_at(Point::new_point(-1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.pattern_at(Point::new_point(-1.1, 0.0, 0.0)), WHITE);
    }
}
