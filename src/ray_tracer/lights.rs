use crate::ray_tracer::{colors::Color, tuples::Point};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Light {
    position: Point,
    intensity: Color,
}

impl Light {
    #[must_use]
    pub const fn point_light(position: &Point, intensity: &Color) -> Self {
        Self {
            position: *position,
            intensity: *intensity,
        }
    }
    #[must_use]
    pub const fn get_position(&self) -> Point {
        self.position
    }
    #[must_use]
    pub const fn get_intensity(&self) -> Color {
        self.intensity
    }
}

#[cfg(test)]
mod tests {
    use crate::ray_tracer::tuples::new_point;

    use super::*;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = new_point(0.0, 0.0, 0.0);
        let light = Light::point_light(&position, &intensity);
        assert_eq!(light.get_position(), position);
        assert_eq!(light.get_intensity(), intensity);
    }
}
