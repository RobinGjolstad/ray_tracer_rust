use crate::{colors::Color, tuples};

use super::Patterns;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Ring {
    color_a: Color,
    color_b: Color,
}

impl Ring {
    pub(super) fn new(color_a: Color, color_b: Color) -> Self {
        Self { color_a, color_b }
    }
}

impl Default for Ring {
    fn default() -> Self {
        Self {
            color_a: Color::new(1.0, 1.0, 1.0),
            color_b: Color::new(0.0, 0.0, 0.0),
        }
    }
}

impl Patterns for Ring {
    fn color_at(&self, point: tuples::Point) -> Color {
        let inside = point.x.powi(2) + point.z.powi(2);
        let magnitude = inside.sqrt();
        if magnitude.floor() as usize % 2 == 0 {
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
    fn a_ring_should_extend_in_both_x_and_z() {
        let pattern = Pattern::ring(WHITE, BLACK);
        assert_eq!(pattern.pattern_at(Tuple::new_point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(Tuple::new_point(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.pattern_at(Tuple::new_point(0.0, 0.0, 1.0)), BLACK);
        assert_eq!(
            pattern.pattern_at(Tuple::new_point(0.708, 0.0, 0.708)),
            BLACK
        );
    }
}
