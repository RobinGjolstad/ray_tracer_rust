use crate::{colors::Color, tuples::Tuple};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Light {
    position: Tuple,
    intensity: Color,
}

impl Light {
    pub fn point_light(position: &Tuple, intensity: &Color) -> Light {
        Light {
            position: *position,
            intensity: *intensity,
        }
    }
    pub fn get_position(&self) -> Tuple {
        self.position
    }
    pub fn get_intensity(&self) -> Color {
        self.intensity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_point_light_has_a_position_and_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Tuple::new_point(0.0, 0.0, 0.0);
        let light = Light::point_light(&position, &intensity);
        assert_eq!(light.get_position(), position);
        assert_eq!(light.get_intensity(), intensity);
    }
}
