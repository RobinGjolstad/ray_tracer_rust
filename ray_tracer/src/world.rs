use crate::{
    lights::Light,
    shapes::{Object, Shapes},
};

#[derive(Debug, PartialEq, Clone)]
pub struct World {
    pub objects: Vec<Object>,
    pub lights: Vec<Light>,
}

impl World {
    pub fn new() -> Self {
        World {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        colors::Color, rays::Ray, shapes::sphere::Sphere, transformations::Transform,
        tuples::Tuple, utils::is_float_equal,
    };

    fn default_world() -> World {
        let mut s1 = Sphere::new();
        let mut s1_mat = s1.get_material();
        s1_mat.color = Color::new(0.8, 1.0, 0.6);
        s1_mat.diffuse = 0.7;
        s1_mat.specular = 0.2;
        s1.set_material(&s1_mat);

        let mut s2 = Sphere::new();
        s2.set_transform(&Transform::scaling(0.5, 0.5, 0.5));

        World {
            objects: vec![Object::Sphere(s1), Object::Sphere(s2)],
            lights: vec![Light::point_light(
                &Tuple::new_point(-10.0, 10.0, -10.0),
                &Color::new(1.0, 1.0, 1.0),
            )],
        }
    }

    #[test]
    fn creating_a_world() {
        let w = World::new();
        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.lights.len(), 0);
    }

    #[test]
    fn the_default_world() {
        let light = Light::point_light(
            &Tuple::new_point(-10.0, 10.0, -10.0),
            &Color::new(1.0, 1.0, 1.0),
        );
        let mut s1 = Sphere::new();
        let mut s1_mat = s1.get_material();
        s1_mat.color = Color::new(0.8, 1.0, 0.6);
        s1_mat.diffuse = 0.7;
        s1_mat.specular = 0.2;
        s1.set_material(&s1_mat);

        let mut s2 = Sphere::new();
        s2.set_transform(&Transform::scaling(0.5, 0.5, 0.5));

        let w = default_world();

        assert!(w.lights.contains(&light));
        assert!(w.objects.contains(&Object::Sphere(s1)));
        assert!(w.objects.contains(&Object::Sphere(s2)));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = default_world();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let xs = r.intersect_world(&w);

        assert_eq!(xs.count(), 4);
        assert!(is_float_equal(&xs.get_element(0).unwrap().get_time(), 4.0));
        assert!(is_float_equal(&xs.get_element(1).unwrap().get_time(), 4.5));
        assert!(is_float_equal(&xs.get_element(2).unwrap().get_time(), 5.5));
        assert!(is_float_equal(&xs.get_element(3).unwrap().get_time(), 6.0));
    }
}
