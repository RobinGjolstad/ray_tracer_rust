use crate::{colors::Color, lights::Light, tuples::Tuple};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}
impl Material {
    pub fn new() -> Material {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn lighting(
        &self,
        light: &Light,
        position: &Tuple,
        eyev: &Tuple,
        normalv: &Tuple,
        in_shadow: bool
    ) -> Color {
        // Variables to combine and return
        let mut diffuse = Color::new(0.0, 0.0, 0.0);
        let mut specular = Color::new(0.0, 0.0, 0.0);
        // combine the surface color with the light's color/intensity
        let effective_color = self.color * light.get_intensity();

        // find the direction to the light source
        let mut lightv = light.get_position() - *position;
        lightv = lightv.normalize();

        // compute the ambient contribution
        let ambient = effective_color * self.ambient;

        // light_dot_normal represents the cosine of the angle between the
        // light vector and the normal vector. A negative number means the
        // light is on the other side of the surface.
        let light_dot_normal = Tuple::dot(&lightv, &normalv);
        if light_dot_normal < 0.0 {
            // diffuse and specular shall be black.
            // They are already initialized to black so we do nothing
        } else {
            // Compute the diffuse contribution
            diffuse = effective_color * self.diffuse * light_dot_normal;

            // reflect_dot_eye represents the cosine of the angle between the
            // reflection vector and the eye vector. A negative number means the
            // light reflects away from the eye.
            let reflectv = Tuple::reflect(&-lightv, normalv);
            let reflect_dot_eye = Tuple::dot(&reflectv, eyev);
            if reflect_dot_eye <= 0.0 {
                // Light reflects away from the eye, so specular must be black.
                // Do nothing since it is already initialized to black.
            } else {
                // Compute the specular contribution
                let factor = f64::powf(reflect_dot_eye, self.shininess);
                specular = light.get_intensity() * self.specular * factor;
            }
        }

        if false == in_shadow {
            // add the three contributions together to get the final shading
            ambient + diffuse + specular
        } else {
            // Only ambient lighting applies if the zone is in shadow
            ambient
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::{lights::Light, tuples::Tuple, utils::is_float_equal};

    use super::*;

    #[test]
    fn the_default_material() {
        let m = Material::new();
        assert_eq!(m.color, Color::new(1.0, 1.0, 1.0));
        assert!(is_float_equal(&m.ambient, 0.1));
        assert!(is_float_equal(&m.diffuse, 0.9));
        assert!(is_float_equal(&m.specular, 0.9));
        assert!(is_float_equal(&m.shininess, 200.0));
    }

    fn setup_lighting() -> (Material, Tuple) {
        (Material::new(), Tuple::new_point(0.0, 0.0, 0.0))
    }

    #[test]
    fn lighting_with_the_eye_between_the_light_and_the_surface() {
        let (m, position) = setup_lighting();
        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = Light::point_light(
            &Tuple::new_point(0.0, 0.0, -10.0),
            &Color::new(1.0, 1.0, 1.0),
        );
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }
    #[test]
    fn lighting_with_the_eye_between_light_and_surface_eye_offset_45_degrees() {
        let (m, position) = setup_lighting();
        let eyev = Tuple::new_vector(0.0, f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = Light::point_light(
            &Tuple::new_point(0.0, 0.0, -10.0),
            &Color::new(1.0, 1.0, 1.0),
        );
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(1.0, 1.0, 1.0));
    }
    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_degrees() {
        let (m, position) = setup_lighting();
        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = Light::point_light(
            &Tuple::new_point(0.0, 10.0, -10.0),
            &Color::new(1.0, 1.0, 1.0),
        );
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }
    #[test]
    fn lighting_with_eye_in_the_path_of_the_reflection_vector() {
        let (m, position) = setup_lighting();
        let eyev = Tuple::new_vector(0.0, -f64::sqrt(2.0) / 2.0, -f64::sqrt(2.0) / 2.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = Light::point_light(
            &Tuple::new_point(0.0, 10.0, -10.0),
            &Color::new(1.0, 1.0, 1.0),
        );
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(1.63639, 1.63639, 1.63639));
    }
    #[test]
    fn lighting_with_the_light_behind_the_surface() {
        let (m, position) = setup_lighting();
        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = Light::point_light(
            &Tuple::new_point(0.0, 0.0, 10.0),
            &Color::new(1.0, 1.0, 1.0),
        );
        let result = m.lighting(&light, &position, &eyev, &normalv, false);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_the_surface_in_shadow() {
        let (m, position) = setup_lighting();
        let eyev = Tuple::new_vector(0.0, 0.0, -1.0);
        let normalv = Tuple::new_vector(0.0, 0.0, -1.0);
        let light = Light::point_light(
            &Tuple::new_point(0.0, 0.0, -10.0),
            &Color::new(1.0, 1.0, 1.0),
        );
        let in_shadow = true;
        let result = m.lighting(&light, &position, &eyev, &normalv, in_shadow);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }
}
