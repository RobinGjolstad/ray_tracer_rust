use crate::{
    colors::Color,
    extract_object,
    intersections::{prepare_computations, IntersectComp},
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
    pub fn shade_hit(&self, comps: &IntersectComp) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        let s = extract_object!(comps.object);
        let mat = s.get_material();
        for lights in &self.lights {
            color = color + mat.lighting(&lights, &comps.point, &comps.eyev, &comps.normalv);
        }
        color
    }

    pub fn color_at(&self, r: &crate::rays::Ray) -> Color {
        let int = r.intersect_world(&self);
        let color = match int.hit() {
            None => Color::new(0.0, 0.0, 0.0),
            Some(int) => {
                let comp = prepare_computations(&int, r);
                self.shade_hit(&comp)
            }
        };
        color
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{
        colors::Color,
        intersections::{prepare_computations, Intersection},
        rays::Ray,
        shapes::sphere::Sphere,
        transformations::Transform,
        tuples::Tuple,
        utils::is_float_equal,
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

    #[test]
    fn shading_an_intersection() {
        let w = default_world();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let shape = w.objects.first().unwrap();
        let i = Intersection::new(4.0, *shape);
        let comps = prepare_computations(&i, &r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }
    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = default_world();
        w.lights = vec![Light::point_light(
            &Tuple::new_point(0.0, 0.25, 0.0),
            &Color::new(1.0, 1.0, 1.0),
        )];
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let shape = w.objects[1];
        let i = Intersection::new(0.5, shape);
        let comps = prepare_computations(&i, &r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }
    #[test]
    fn the_color_when_a_ray_misses() {
        let w = default_world();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }
    #[test]
    fn the_color_when_a_ray_hits() {
        let w = default_world();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let c = w.color_at(&r);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }
    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let w = default_world();
        let mut objects = w.objects.iter();
        let mut outer = *objects.next().unwrap();
        match outer {
            Object::Sphere(s) => {
                let mut s = s;
                let mut mat = s.get_material();
                mat.ambient = 1.0;
                s.set_material(&mat);
                outer = Object::Sphere(s).clone();
            }
        }
        let mut inner = *objects.next().unwrap();
        let mut inner_sphere = Sphere::new();
        match inner {
            Object::Sphere(s) => {
                let mut s = s;
                let mut mat = s.get_material();
                mat.ambient = 1.0;
                s.set_material(&mat);
                inner_sphere = s;
                inner = Object::Sphere(s);
            }
        }
        let new_world = World {
            lights: default_world().lights,
            objects: vec![outer, inner],
        };
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.75),
            Tuple::new_vector(0.0, 0.0, -1.0),
        );
        let c = new_world.color_at(&r);
        assert_eq!(c, inner_sphere.get_material().color);
    }
}
