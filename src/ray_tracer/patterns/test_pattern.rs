use crate::ray_tracer::{colors::Color, tuples::Point};

use super::Patterns;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub(super) struct TestPattern {}
impl TestPattern {}

impl Patterns for TestPattern {
    fn color_at(&self, point: Point) -> Color {
        Color::new(point.x, point.y, point.z)
    }
}

#[cfg(test)]
mod tests {
    //
}
