use crate::{colors::Color, tuples::Point};

struct Pattern {
    pub(crate) color_a: Color,
    pub(crate) color_b: Color,
}
impl Pattern {
    pub(crate) fn stripe(color_a: Color, color_b: Color) -> Self {
        Pattern { color_a, color_b }
    }
    pub(crate) fn stripe_at(&self, point: Point) -> Color {
        //
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{colors::Color, tuples::Point};

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
        assert_eq!(pattern.color_a, WHITE);
        assert_eq!(pattern.color_b, BLACK);
    }
    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = Pattern::stripe(WHITE, BLACK);
        assert_eq!(pattern.stripe_at(Point::new_point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new_point(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new_point(0.0, 2.0, 0.0)), WHITE);
    }
    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = Pattern::stripe(WHITE, BLACK);
        assert_eq!(pattern.stripe_at(Point::new_point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new_point(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new_point(0.0, 0.0, 2.0)), WHITE);
    }
    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = Pattern::stripe(WHITE, BLACK);
        assert_eq!(pattern.stripe_at(Point::new_point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new_point(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new_point(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.stripe_at(Point::new_point(-0.1, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.stripe_at(Point::new_point(-1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.stripe_at(Point::new_point(-1.1, 0.0, 0.0)), WHITE);
    }
}
