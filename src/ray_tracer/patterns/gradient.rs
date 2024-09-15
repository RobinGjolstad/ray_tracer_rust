use crate::ray_tracer::{colors::Color, tuples};

use super::Patterns;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Gradient {
    color_a: Color,
    color_b: Color,
}

impl Gradient {
    pub(super) const fn new(color_a: Color, color_b: Color) -> Self {
        Self { color_a, color_b }
    }
}

impl Default for Gradient {
    fn default() -> Self {
        Self {
            color_a: Color::new(1.0, 1.0, 1.0),
            color_b: Color::new(0.0, 0.0, 0.0),
        }
    }
}

impl Patterns for Gradient {
    fn color_at(&self, point: tuples::Point) -> Color {
        let distance = self.color_b - self.color_a;
        let fraction = point.x - f64::floor(point.x);

        self.color_a + distance * fraction
    }
}

#[cfg(test)]
mod tests {
    use crate::ray_tracer::{patterns::Pattern, tuples::Tuple};

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
    fn a_gradient_linearly_interpolates_between_colors() {
        let pattern = Pattern::gradient(WHITE, BLACK);
        assert_eq!(pattern.pattern_at(Tuple::new_point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(
            pattern.pattern_at(Tuple::new_point(0.25, 0.0, 0.0)),
            Color::new(0.75, 0.75, 0.75)
        );
        assert_eq!(
            pattern.pattern_at(Tuple::new_point(0.5, 0.0, 0.0)),
            Color::new(0.5, 0.5, 0.5)
        );
        assert_eq!(
            pattern.pattern_at(Tuple::new_point(0.75, 0.0, 0.0)),
            Color::new(0.25, 0.25, 0.25)
        );
    }
}
