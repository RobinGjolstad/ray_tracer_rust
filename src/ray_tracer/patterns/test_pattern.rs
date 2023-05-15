use crate::ray_tracer::{colors::Color, tuples::Point};

use super::Patterns;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) struct TestPattern {}
impl TestPattern {}

impl Patterns for TestPattern {
    fn color_at(&self, point: Point) -> Color {
        Color::new(point.x, point.y, point.z)
    }
}

impl Default for TestPattern {
    fn default() -> Self {
        Self {}
    }
}

#[cfg(test)]
mod tests {
    //
}
