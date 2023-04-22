use crate::ray_tracer::{
    colors::Color,
    intersections::{prepare_computations, IntersectComp},
    lights::Light,
    rays::Ray,
    shapes::Object,
    transformations::Transform,
    tuples::Tuple,
};

use super::utils::is_float_equal;

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
        let mut s1 = Object::new_sphere();
        let mut s1_mat = s1.get_material();
        s1_mat.color = Color::new(0.8, 1.0, 0.6);
        s1_mat.diffuse = 0.7;
        s1_mat.specular = 0.2;
        s1.set_material(&s1_mat);

        let mut s2 = Object::new_sphere();
        s2.set_transform(&Transform::scaling(0.5, 0.5, 0.5));

        World {
            objects: vec![s1, s2],
            lights: vec![Light::point_light(
                &Tuple::new_point(-10.0, 10.0, -10.0),
                &Color::new(1.0, 1.0, 1.0),
            )],
        }
    }
    pub(crate) fn shade_hit(&self, comps: &IntersectComp, remaining: usize) -> Color {
        let mut surface = Color::new(0.0, 0.0, 0.0);
        let s = comps.object.clone();
        let mat = s.get_material();

        for lights in &self.lights {
            surface = surface
                + mat.lighting(
                    &s,
                    lights,
                    &comps.over_point,
                    &comps.eyev,
                    &comps.normalv,
                    self.is_shadowed(&comps.over_point),
                );
        }
        let reflected = self.reflected_color(comps, remaining);

        surface + reflected
    }

    pub(crate) fn color_at(&self, r: &Ray, remaining: usize) -> Color {
        let int = r.intersect_world(self);
        match int.hit() {
            None => Color::new(0.0, 0.0, 0.0),
            Some(int) => {
                let comp = prepare_computations(&int, r);
                self.shade_hit(&comp, remaining)
            }
        }
    }

    pub(crate) fn is_shadowed(&self, point: &Tuple) -> bool {
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

    pub(crate) fn reflected_color(&self, comps: &IntersectComp, remaining: usize) -> Color {
        if is_float_equal(&comps.object.material.reflective, 0.0) || remaining < 1 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
        let color = self.color_at(&reflect_ray, remaining - 1) * comps.object.material.reflective;

        color
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use crate::ray_tracer::{
        intersections::{self, Intersection},
        utils::is_float_equal,
    };

    use super::*;

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
        let mut s1 = Object::new_sphere();
        let mut s1_mat = s1.get_material();
        s1_mat.color = Color::new(0.8, 1.0, 0.6);
        s1_mat.diffuse = 0.7;
        s1_mat.specular = 0.2;
        s1.set_material(&s1_mat);

        let mut s2 = Object::new_sphere();
        s2.set_transform(&Transform::scaling(0.5, 0.5, 0.5));

        let w = default_world();

        assert!(w.lights.contains(&light));
        assert!(w.objects.contains(&s1));
        assert!(w.objects.contains(&s2));
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
        let c = w.shade_hit(&comps, 1);
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
        let c = w.shade_hit(&comps, 1);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }
    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = World::new();
        w.lights = vec![Light::point_light(
            &Tuple::new_point(0.0, 0.0, -10.0),
            &Color::new(1.0, 1.0, 1.0),
        )];

        let s1 = Object::new_sphere();
        w.objects.push(s1);

        let mut s2 = Object::new_sphere();
        s2.set_transform(&Transform::translate(0.0, 0.0, 10.0));
        w.objects.push(s2.clone());

        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let i = Intersection::new(4.0, s2);
        let comps = prepare_computations(&i, &r);
        let c = w.shade_hit(&comps, 1);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
    #[test]
    fn the_color_when_a_ray_misses() {
        let w = default_world();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        let c = w.color_at(&r, 1);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }
    #[test]
    fn the_color_when_a_ray_hits() {
        let w = default_world();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let c = w.color_at(&r, 1);
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
        outer.set_material(&mat);

        // Grabs the inner sphere
        let mut inner = objects.next().unwrap().clone();
        let mut _inner_sphere = Object::new_sphere();
        let mut mat = inner.get_material();
        mat.ambient = 1.0;
        inner.set_material(&mat);
        _inner_sphere = inner.clone();

        let new_world = World {
            lights: default_world().lights,
            objects: vec![outer, inner],
        };
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.75),
            Tuple::new_vector(0.0, 0.0, -1.0),
        );
        let c = new_world.color_at(&r, 1);
        assert_eq!(c, _inner_sphere.get_material().color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::new_default_world();
        let p = Tuple::new_point(0.0, 10.0, 0.0);
        assert!(!w.is_shadowed(&p));
    }
    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::new_default_world();
        let p = Tuple::new_point(10.0, -10.0, 10.0);
        assert!(w.is_shadowed(&p));
    }
    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::new_default_world();
        let p = Tuple::new_point(-20.0, 20.0, -20.0);
        assert!(!w.is_shadowed(&p));
    }
    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::new_default_world();
        let p = Tuple::new_point(-2.0, 2.0, -2.0);
        assert!(!w.is_shadowed(&p));
    }

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let mut w = World::new_default_world();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut shape = w.objects.get_mut(1).unwrap();
        shape.material.ambient = 1.0;
        let i = Intersection::new(1.0, shape.clone());
        let comps = intersections::prepare_computations(&i, &r);
        let color = w.reflected_color(&comps, 1);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }
    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let mut w = World::new_default_world();
        let mut shape = Object::new_plane();
        shape.material.reflective = 0.5;
        shape.set_transform(&Transform::translate(0.0, -1.0, 0.0));
        w.objects.push(shape.clone());
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -3.0),
            Tuple::new_vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), shape);
        let comps = intersections::prepare_computations(&i, &r);
        let color = w.reflected_color(&comps, 1);
        assert_eq!(color, Color::new(0.19032, 0.2379, 0.14274));
    }
    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut w = World::new_default_world();
        let mut shape = Object::new_plane();
        shape.material.reflective = 0.5;
        shape.set_transform(&Transform::translate(0.0, -1.0, 0.0));
        w.objects.push(shape.clone());
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -3.0),
            Tuple::new_vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), shape);
        let comps = intersections::prepare_computations(&i, &r);
        let color = w.shade_hit(&comps, 1);
        assert_eq!(color, Color::new(0.87677, 0.92436, 0.82918));
    }
    #[test]
    #[ignore = "Recursive reflection is not bounded yet."]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = World::new();
        w.lights.push(Light::point_light(
            &Tuple::new_point(0.0, 0.0, 0.0),
            &Color::new(1.0, 1.0, 1.0),
        ));
        let mut lower = Object::new_plane();
        lower.material.reflective = 1.0;
        lower.set_transform(&Transform::translate(0.0, -1.0, 0.0));
        let mut upper = lower.clone();
        upper.set_transform(&Transform::translate(0.0, 1.0, 0.0));
        w.objects.push(lower);
        w.objects.push(upper);
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        let color = w.color_at(&r, 1);
    }
    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let mut w = World::new_default_world();
        let mut shape = Object::new_plane();
        shape.material.reflective = 0.5;
        shape.set_transform(&Transform::translate(0.0, -1.0, 0.0));
        w.objects.push(shape.clone());
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -3.0),
            Tuple::new_vector(0.0, -2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), shape);
        let comps = intersections::prepare_computations(&i, &r);
        let color = w.reflected_color(&comps, 0);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }
}
