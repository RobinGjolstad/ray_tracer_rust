use crate::ray_tracer::{
    colors::Color,
    intersections::{prepare_computations, schlick, IntersectComp},
    lights::Light,
    rays::Ray,
    shapes::Object,
    transformations::Transform,
    tuples::Point,
};

use super::{intersections, tuples::Tuple, utils::is_float_equal};

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
                &Point::new_point(-10.0, 10.0, -10.0),
                &Color::new(1.0, 1.0, 1.0),
            )],
        }
    }
    pub(crate) fn shade_hit(&self, comps: &IntersectComp, remaining: usize) -> Color {
        let mut return_color = Color::new(0.0, 0.0, 0.0);
        let shadowed = self.is_shadowed(&comps.over_point);

        let surface = comps.object.material.lighting(
            &comps.object.clone(),
            &self.lights[0],
            &comps.over_point,
            &comps.eyev,
            &comps.normalv,
            shadowed,
        );

        let reflected = self.reflected_color(comps, remaining);
        let refracted = self.refracted_color(comps, remaining);

        let material = comps.object.material;
        if material.reflective > 0.0 && material.transparency > 0.0 {
            let reflectance = schlick(comps);
            return_color = surface + reflected * reflectance + refracted * (1.0 - reflectance);
        } else {
            return_color = surface + reflected + refracted;
        }

        return_color
    }

    pub(crate) fn color_at(&self, r: &Ray, remaining: usize) -> Color {
        let int = r.intersect_world(self);
        match int.hit() {
            None => Color::new(0.0, 0.0, 0.0),
            Some(int_hit) => {
                let comp = prepare_computations(&int_hit, r, &int);
                self.shade_hit(&comp, remaining)
            }
        }
    }

    pub(crate) fn is_shadowed(&self, point: &Point) -> bool {
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

        self.color_at(&reflect_ray, remaining - 1) * comps.object.material.reflective
    }

    fn refracted_color(&self, comps: &IntersectComp, remaining: usize) -> Color {
        if remaining == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if is_float_equal(&comps.object.material.transparency, 0.0) {
            return Color::new(0.0, 0.0, 0.0);
        }

        // Snell's Law:
        // sin(theta_i) / sin(theta_t) == n_2 / n_1
        let n_ratio = comps.n1 / comps.n2;
        let cos_i = Tuple::dot(&comps.eyev, &comps.normalv);
        let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));
        if sin2_t > 1.0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;
        let refract_ray = Ray::new(comps.under_point, direction);

        self.color_at(&refract_ray, remaining - 1) * comps.object.material.transparency
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
        intersections::{Intersection, Intersections},
        patterns::Pattern,
        tuples::Vector,
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
            &Point::new_point(-10.0, 10.0, -10.0),
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
            Point::new_point(0.0, 0.0, -5.0),
            Vector::new_vector(0.0, 0.0, 1.0),
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
            Point::new_point(0.0, 0.0, -5.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );
        let shape = w.objects.first().unwrap();
        let i = Intersection::new(4.0, shape.clone());
        let comps = prepare_computations(&i.clone(), &r, &Intersections::new(&vec![i]));
        let c = w.shade_hit(&comps, 1);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }
    #[test]
    fn shading_an_intersection_from_the_inside() {
        let mut w = default_world();
        w.lights = vec![Light::point_light(
            &Point::new_point(0.0, 0.25, 0.0),
            &Color::new(1.0, 1.0, 1.0),
        )];
        let r = Ray::new(
            Point::new_point(0.0, 0.0, 0.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );
        let shape = w.objects[1].clone();
        let i = Intersection::new(0.5, shape);
        let comps = prepare_computations(&i.clone(), &r, &Intersections::new(&vec![i]));
        let c = w.shade_hit(&comps, 1);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }
    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = World::new();
        w.lights = vec![Light::point_light(
            &Point::new_point(0.0, 0.0, -10.0),
            &Color::new(1.0, 1.0, 1.0),
        )];

        let s1 = Object::new_sphere();
        w.objects.push(s1);

        let mut s2 = Object::new_sphere();
        s2.set_transform(&Transform::translate(0.0, 0.0, 10.0));
        w.objects.push(s2.clone());

        let r = Ray::new(
            Point::new_point(0.0, 0.0, 5.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );
        let i = Intersection::new(4.0, s2);
        let comps = prepare_computations(&i.clone(), &r, &Intersections::new(&vec![i]));
        let c = w.shade_hit(&comps, 1);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
    #[test]
    fn the_color_when_a_ray_misses() {
        let w = default_world();
        let r = Ray::new(
            Point::new_point(0.0, 0.0, -5.0),
            Vector::new_vector(0.0, 1.0, 0.0),
        );
        let c = w.color_at(&r, 1);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }
    #[test]
    fn the_color_when_a_ray_hits() {
        let w = default_world();
        let r = Ray::new(
            Point::new_point(0.0, 0.0, -5.0),
            Vector::new_vector(0.0, 0.0, 1.0),
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
            Point::new_point(0.0, 0.0, 0.75),
            Vector::new_vector(0.0, 0.0, -1.0),
        );
        let c = new_world.color_at(&r, 1);
        assert_eq!(c, _inner_sphere.get_material().color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = World::new_default_world();
        let p = Point::new_point(0.0, 10.0, 0.0);
        assert!(!w.is_shadowed(&p));
    }
    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = World::new_default_world();
        let p = Point::new_point(10.0, -10.0, 10.0);
        assert!(w.is_shadowed(&p));
    }
    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = World::new_default_world();
        let p = Point::new_point(-20.0, 20.0, -20.0);
        assert!(!w.is_shadowed(&p));
    }
    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = World::new_default_world();
        let p = Point::new_point(-2.0, 2.0, -2.0);
        assert!(!w.is_shadowed(&p));
    }

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let mut w = World::new_default_world();
        let r = Ray::new(
            Point::new_point(0.0, 0.0, 0.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );
        let mut shape = w.objects.get_mut(1).unwrap();
        shape.material.ambient = 1.0;
        let i = Intersection::new(1.0, shape.clone());
        let comps = prepare_computations(&i.clone(), &r, &Intersections::new(&vec![i]));
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
            Point::new_point(0.0, 0.0, -3.0),
            Vector::new_vector(0.0, -(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), shape);
        let comps = prepare_computations(&i.clone(), &r, &Intersections::new(&vec![i]));
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
            Point::new_point(0.0, 0.0, -3.0),
            Vector::new_vector(0.0, -(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), shape);
        let comps = prepare_computations(&i.clone(), &r, &Intersections::new(&vec![i]));
        let color = w.shade_hit(&comps, 1);
        assert_eq!(color, Color::new(0.87677, 0.92436, 0.82918));
    }
    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = World::new();
        w.lights.push(Light::point_light(
            &Point::new_point(0.0, 0.0, 0.0),
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
            Point::new_point(0.0, 0.0, 0.0),
            Vector::new_vector(0.0, 1.0, 0.0),
        );

        // Simply test that the function returns when the ray is locked between two mirrors.
        #[allow(unused_assignments)]
        let mut color = Color::new(0.0, 0.0, 0.0);
        color = w.color_at(&r, 1);
        assert_ne!(color, Color::new(0.0, 0.0, 0.0));
    }
    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let mut w = World::new_default_world();
        let mut shape = Object::new_plane();
        shape.material.reflective = 0.5;
        shape.set_transform(&Transform::translate(0.0, -1.0, 0.0));
        w.objects.push(shape.clone());
        let r = Ray::new(
            Point::new_point(0.0, 0.0, -3.0),
            Vector::new_vector(0.0, -(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), shape);
        let comps = prepare_computations(&i.clone(), &r, &Intersections::new(&vec![i]));
        let color = w.reflected_color(&comps, 0);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }

    #[test]
    fn the_refracted_color_with_an_opaque_surface() {
        let w = World::new_default_world();
        let shape = w.objects[0].clone();
        let r = Ray::new(
            Point::new_point(0.0, 0.0, -5.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );
        let xs = Intersections::new(&[
            Intersection::new(4.0, shape.clone()),
            Intersection::new(6.0, shape),
        ]);
        let comps = prepare_computations(&xs.list[0], &r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }
    #[test]
    fn the_refracted_color_at_the_maximum_recursive_depth() {
        let w = World::new_default_world();
        let mut shape = w.objects[0].clone();
        shape.material.transparency = 1.0;
        shape.material.refractive_index = 1.5;
        let r = Ray::new(
            Point::new_point(0.0, 0.0, -5.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );
        let xs = Intersections::new(&[
            Intersection::new(4.0, shape.clone()),
            Intersection::new(6.0, shape),
        ]);
        let comps = prepare_computations(&xs.list[0], &r, &xs);
        let c = w.refracted_color(&comps, 0);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }
    #[test]
    fn the_refracted_color_under_total_internal_reflection() {
        let w = World::new_default_world();
        let mut shape = w.objects[0].clone();
        shape.material.transparency = 1.0;
        shape.material.refractive_index = 1.5;
        let r = Ray::new(
            Point::new_point(0.0, 0.0, 2.0_f64.sqrt() / 2.0),
            Vector::new_vector(0.0, 1.0, 0.0),
        );
        let xs = Intersections::new(&[
            Intersection::new(-(2.0_f64.sqrt()) / 2.0, shape.clone()),
            Intersection::new(2.0_f64.sqrt() / 2.0, shape),
        ]);
        let comps = prepare_computations(&xs.list[1], &r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }
    #[test]
    #[allow(non_snake_case)]
    fn the_refracted_color_with_a_refracted_ray() {
        let mut w = default_world();
        w.objects[0].material.ambient = 1.0;
        w.objects[0].material.pattern = Some(Pattern::test_pattern_default());
        let A = w.objects[0].clone();

        w.objects[1].material.transparency = 1.0;
        w.objects[1].material.refractive_index = 1.5;
        let B = w.objects[1].clone();

        let r = Ray::new(
            Point::new_point(0.0, 0.0, 0.1),
            Vector::new_vector(0.0, 1.0, 0.0),
        );
        let xs = Intersections::new(&[
            Intersection::new(-0.9899, A.clone()),
            Intersection::new(-0.4899, B.clone()),
            Intersection::new(0.4899, B),
            Intersection::new(0.9899, A),
        ]);
        let comps = prepare_computations(&xs.list[2], &r, &xs);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, Color::new(0.0, 0.99888, 0.04725));
    }
    #[test]
    fn shade_hit_with_a_transparent_material() {
        let mut w = default_world();

        let mut floor = Object::new_plane();
        floor.set_transform(&Transform::translate(0.0, -1.0, 0.0));
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        w.objects.push(floor.clone());

        let mut ball = Object::new_sphere();
        ball.set_transform(&Transform::translate(0.0, -3.5, -0.5));
        ball.material.color = Color::new(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        w.objects.push(ball.clone());

        let r = Ray::new(
            Point::new_point(0.0, 0.0, -3.0),
            Vector::new_vector(0.0, -(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let xs = Intersections::new(&[Intersection::new(2.0_f64.sqrt(), floor)]);
        let comps = prepare_computations(&xs.list[0], &r, &xs);
        let color = w.shade_hit(&comps, 5);
        assert_eq!(color, Color::new(0.93642, 0.68642, 0.68642));
    }
    #[test]
    fn shade_hit_with_a_reflective_transparent_material() {
        let mut w = default_world();

        let mut floor = Object::new_plane();
        floor.set_transform(&Transform::translate(0.0, -1.0, 0.0));
        floor.material.reflective = 0.5;
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        w.objects.push(floor.clone());

        let mut ball = Object::new_sphere();
        ball.set_transform(&Transform::translate(0.0, -3.5, -0.5));
        ball.material.color = Color::new(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        w.objects.push(ball.clone());

        let r = Ray::new(
            Point::new_point(0.0, 0.0, -3.0),
            Vector::new_vector(0.0, -(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let xs = Intersections::new(&[Intersection::new(2.0_f64.sqrt(), floor)]);
        let comps = prepare_computations(&xs.list[0], &r, &xs);
        let color = w.shade_hit(&comps, 5);
        assert_eq!(color, Color::new(0.93391, 0.69643, 0.69243));
    }
}
