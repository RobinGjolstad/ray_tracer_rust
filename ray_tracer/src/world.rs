use crate::{
    colors::Color,
    intersections::{prepare_computations, IntersectComp, Intersections},
    lights::Light,
    rays::Ray,
    shapes::{sphere::Sphere, Object, Shapes},
    transformations::Transform,
    tuples::Tuple,
};

#[derive(Debug, PartialEq)]
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
    pub fn new_default_world() -> World {
        let mut s1 = Sphere::new();
        let mut s1_mat = s1.get_material();
        s1_mat.color = Color::new(0.8, 1.0, 0.6);
        s1_mat.diffuse = 0.7;
        s1_mat.specular = 0.2;
        s1.set_material(&s1_mat);

        let mut s2 = Sphere::new();
        s2.set_transform(&Transform::scaling(0.5, 0.5, 0.5));

        World {
            objects: vec![Object::new(Box::new(s1)), Object::new(Box::new(s2))],
            lights: vec![Light::point_light(
                &Tuple::new_point(-10.0, 10.0, -10.0),
                &Color::new(1.0, 1.0, 1.0),
            )],
        }
    }
    pub fn shade_hit(&self, comps: &IntersectComp) -> Color {
        let mut color = Color::new(0.0, 0.0, 0.0);
        let s = comps.object.clone();
        let mat = s.get_material();
        for lights in &self.lights {
            color = color
                + mat.lighting(
                    &lights,
                    &comps.over_point,
                    &comps.eyev,
                    &comps.normalv,
                    self.is_shadowed(&comps.over_point),
                );
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

    pub fn is_shadowed(&self, point: &Tuple) -> bool {
        let v = self.lights.first().unwrap().get_position() - *point; // TODO: Support multiple lights
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(*point, direction);
        let intersections = r.intersect_world(self);
        let h = intersections.hit();

        if let Some(hit) = h {
            if hit.get_time() < distance {
                return true;
            }
        }
        false
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
        World::new_default_world()
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
        assert!(w.objects.contains(&Object::new(Box::new(s1))));
        assert!(w.objects.contains(&Object::new(Box::new(s2))));
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
        let i = Intersection::new(4.0, shape.clone());
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
        let shape = w.objects[1].clone();
        let i = Intersection::new(0.5, shape);
        let comps = prepare_computations(&i, &r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }
    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = World::new();
        w.lights = vec![Light::point_light(
            &Tuple::new_point(0.0, 0.0, -10.0),
            &Color::new(1.0, 1.0, 1.0),
        )];

        let s1 = Sphere::new();
        w.objects.push(Object::new(Box::new(s1)));

        let mut s2 = Sphere::new();
        s2.set_transform(&Transform::translate(0.0, 0.0, 10.0));
        w.objects.push(Object::new(Box::new(s2)));

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let i = Intersection::new(4.0, Object::new(Box::new(s2)));
        let comps = prepare_computations(&i, &r);
        let c = w.shade_hit(&comps);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
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

        // Grabs the outer sphere
        let mut outer = objects.next().unwrap().clone();

        let mut mat = outer.get_material();
        mat.ambient = 1.0;
        outer.set_material(mat);

        // Grabs the inner sphere
        let mut inner = objects.next().unwrap().clone();
        let mut inner_sphere = Object::new(Box::new(Sphere::new()));
        let mut mat = inner.get_material();
        mat.ambient = 1.0;
        inner.set_material(mat);
        inner_sphere = inner.clone();

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

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::new_default_world();
        let p = Tuple::new_point(0.0, 10.0, 0.0);
        assert_eq!(w.is_shadowed(&p), false);
    }
    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::new_default_world();
        let p = Tuple::new_point(10.0, -10.0, 10.0);
        assert_eq!(w.is_shadowed(&p), true);
    }
    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::new_default_world();
        let p = Tuple::new_point(-20.0, 20.0, -20.0);
        assert_eq!(w.is_shadowed(&p), false);
    }
    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::new_default_world();
        let p = Tuple::new_point(-2.0, 2.0, -2.0);
        assert_eq!(w.is_shadowed(&p), false);
    }
}
