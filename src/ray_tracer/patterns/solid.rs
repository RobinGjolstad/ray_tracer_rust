use crate::ray_tracer::{colors::Color, tuples_new::Point};

use super::Patterns;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct Solid {
    color: Color,
}

impl Solid {
    pub(super) const fn new(color: Color) -> Self {
        Self { color }
    }
}

impl Default for Solid {
    fn default() -> Self {
        Self {
            color: Color::new(0.0, 0.0, 0.0),
        }
    }
}

impl Patterns for Solid {
    fn color_at(&self, _point: Point) -> Color {
        self.color
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray_tracer::{patterns::Pattern, tuples_new::new_point};

    const WHITE: Color = Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    };

    #[test]
    fn a_solid_pattern_is_the_same_color_in_all_directions() {
        let pattern = Pattern::solid(WHITE);
        assert_eq!(pattern.pattern_at(new_point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(new_point(1.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(new_point(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(pattern.pattern_at(new_point(1.0, 1.0, 0.0)), WHITE);
    }
}
