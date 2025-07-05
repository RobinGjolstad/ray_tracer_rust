use std::sync::Arc;

use super::{
    colors::Color,
    intersections::{prepare_computations, schlick, IntersectComp, Intersections},
    lights::Light,
    rays::Ray,
    shapes::Object,
    tuples_new::{Point, Vector},
    utils::is_float_equal,
};

#[allow(clippy::module_name_repetitions)]
#[derive(Default, Debug, PartialEq, Clone)]
pub struct WorldBuilder {
    objects: Vec<Object>,
    lights: Vec<Light>,
}

impl WorldBuilder {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn object(&mut self, object: Object) -> &mut Self {
        self.objects.push(object);
        self
    }

    pub fn light(&mut self, light: Light) -> &mut Self {
        self.lights.push(light);
        self
    }

    #[must_use]
    pub fn build(self) -> World {
        World {
            objects: self.objects.into(),
            lights: self.lights.into(),
        }
    }
}

#[derive(Default, Debug, PartialEq)]
pub struct World {
    pub objects: Arc<[Object]>,
    pub lights: Arc<[Light]>,
}

impl World {
    #[must_use]
    pub fn builder() -> WorldBuilder {
        WorldBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> WorldBuilder {
        WorldBuilder {
            objects: self.objects.to_vec(),
            lights: self.lights.to_vec(),
        }
    }

    #[cfg(test)]
    #[must_use]
    pub fn new_default_world() -> Self {
        use crate::ray_tracer::{shapes::new_sphere, tuples_new::new_point};

        let s1 = new_sphere()
            .color(&Color::new(0.8, 1.0, 0.6))
            .diffuse(0.7)
            .specular(0.2)
            .build();

        let s2 = new_sphere().scale(0.5, 0.5, 0.5).build();

        let objects = vec![s1, s2];
        let lights = vec![Light::point_light(
            &new_point(-10.0, 10.0, -10.0),
            &Color::new(1.0, 1.0, 1.0),
        )];
        Self {
            objects: objects.into(),
            lights: lights.into(),
        }
    }
    pub(crate) fn shade_hit(&self, comps: &IntersectComp, remaining: usize) -> Color {
        let shadowed = self.is_shadowed(&comps.over_point);

        let surface = comps.object.get_material().lighting(
            &comps.object.clone(),
            &self.lights[0],
            &comps.over_point,
            &comps.eyev,
            &comps.normalv,
            shadowed,
        );

        let reflected = self.reflected_color(comps, remaining);
        let refracted = self.refracted_color(comps, remaining);

        let material = comps.object.get_material();
        if material.reflective > 0.0 && material.transparency > 0.0 {
            let reflectance = schlick(comps);

            surface + reflected * reflectance + refracted * (1.0 - reflectance)
        } else {
            surface + reflected + refracted
        }
    }

    pub(crate) fn color_at(&self, r: &Ray, remaining: usize) -> Color {
        let int = r.intersect_world(self);
        int.hit().map_or_else(
            || Color::new(0.0, 0.0, 0.0),
            |int_hit| {
                let comp = prepare_computations(&int_hit, r, &int);
                self.shade_hit(&comp, remaining)
            },
        )
    }

    pub(crate) fn is_shadowed(&self, point: &Point) -> bool {
        let v = self.lights.first().unwrap().get_position() - *point; // TODO: Support multiple lights
        let distance = v.magnitude();
        let direction = v.normalize();

        let r = Ray::new(*point, direction);
        let mut intersections = Intersections::default();

        // Iterate over all objects, but stop on first valid intersection.
        for object in self.objects.iter() {
            r.intersect(object, &mut intersections.list);
            if intersections.hit().is_some_and(|h| h.get_time() < distance) {
                return true;
            }
        }

        false
    }

    pub(crate) fn reflected_color(&self, comps: &IntersectComp, remaining: usize) -> Color {
        if is_float_equal(&comps.object.get_material().reflective, 0.0) || remaining < 1 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let reflect_ray = Ray::new(comps.over_point, comps.reflectv);

        self.color_at(&reflect_ray, remaining - 1) * comps.object.get_material().reflective
    }

    fn refracted_color(&self, comps: &IntersectComp, remaining: usize) -> Color {
        if remaining == 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        if is_float_equal(&comps.object.get_material().transparency, 0.0) {
            return Color::new(0.0, 0.0, 0.0);
        }

        // Snell's Law:
        // sin(theta_i) / sin(theta_t) == n_2 / n_1
        let n_ratio = comps.n1 / comps.n2;
        let cos_i = Vector::dot(&comps.eyev, &comps.normalv);
        // let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));
        let sin2_t = n_ratio.powi(2) * cos_i.mul_add(-cos_i, 1.0);
        if sin2_t > 1.0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let cos_t = (1.0 - sin2_t).sqrt();
        // let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;
        let direction = comps.normalv * n_ratio.mul_add(cos_i, -cos_t) - comps.eyev * n_ratio;
        let refract_ray = Ray::new(comps.under_point, direction);

        self.color_at(&refract_ray, remaining - 1) * comps.object.get_material().transparency
    }
}

#[cfg(test)]
mod tests {
    

    use crate::ray_tracer::{
        intersections::{Intersection, Intersections},
        patterns::Pattern,
        shapes::{new_plane, new_sphere, ShapeBuilder, Sphere, TypeSpecified},
        tuples_new::{new_point, new_vector},
        utils::is_float_equal,
    };

    use super::*;

    /// Default collection of objects and lights for the default world.
    ///
    /// This function should be used when you want the default world, but want to modify one of the components.
    /// In this case, do something like this:
    /// ```rust
    /// // Get the default set of components.
    /// let (objects, lights) = default_world_components();
    ///
    /// // Modify the objects or lights as needed.
    /// let first_object = objects.first().unwrap().clone();
    /// let modified_object = first_object.scale(2.0, 2.0, 2.0);
    ///
    /// // Create a new world with the modified components.
    /// let w = WorldBuilder::new()
    ///    .object(modified_object.build())
    ///    .lights(lights)
    ///    .build();
    ///
    /// // Use the world as needed.
    /// ```
    fn default_world_components() -> (Vec<ShapeBuilder<Sphere, TypeSpecified>>, Vec<Light>) {
        let s1 = new_sphere()
            .color(&Color::new(0.8, 1.0, 0.6))
            .diffuse(0.7)
            .specular(0.2);

        let s2 = new_sphere().scale(0.5, 0.5, 0.5);

        let objects = vec![s1, s2];
        let lights = vec![Light::point_light(
            &new_point(-10.0, 10.0, -10.0),
            &Color::new(1.0, 1.0, 1.0),
        )];

        (objects, lights)
    }

    /// Get an immutable default world with two spheres and a point-light.
    ///
    /// If you wish to modify any of the components, use `default_world_components()` instead.
    fn default_world() -> World {
        let (objects, lights) = default_world_components();
        let objects = objects.into_iter().map(|s| s.build()).collect::<Vec<_>>();
        World {
            objects: objects.into(),
            lights: lights.into(),
        }
    }

    #[test]
    fn creating_a_world() {
        let w = World::default();
        assert_eq!(w.objects.len(), 0);
        assert_eq!(w.lights.len(), 0);
    }

    #[test]
    fn the_default_world() {
        let light = Light::point_light(&new_point(-10.0, 10.0, -10.0), &Color::new(1.0, 1.0, 1.0));
        let s1 = new_sphere()
            .color(&Color::new(0.8, 1.0, 0.6))
            .diffuse(0.7)
            .specular(0.2)
            .build();

        let s2 = new_sphere().scale(0.5, 0.5, 0.5).build();

        let (objects, lights) = default_world_components();
        let objects = objects.into_iter().map(|s| s.build()).collect::<Vec<_>>();

        assert!(lights.contains(&light));
        assert!(objects.contains(&s1));
        assert!(objects.contains(&s2));
    }

    #[test]
    fn intersect_a_world_with_a_ray() {
        let w = default_world();
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
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
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let shape = w.objects.first().unwrap();
        let i = Intersection::new(4.0, shape.clone());
        let binding = Intersections::new(&[i.clone()]);
        let comps = prepare_computations(&i, &r, &binding);
        let c = w.shade_hit(&comps, 1);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }
    #[test]
    fn shading_an_intersection_from_the_inside() {
        let (objects, _) = default_world_components();
        let objects = objects.into_iter().map(|s| s.build()).collect::<Vec<_>>();
        let light = Light::point_light(&new_point(0.0, 0.25, 0.0), &Color::new(1.0, 1.0, 1.0));
        let mut w = WorldBuilder::new();
        for o in &objects {
            w.object(o.clone());
        }
        w.light(light);
        let w = w.build();
        let r = Ray::new(new_point(0.0, 0.0, 0.0), new_vector(0.0, 0.0, 1.0));
        let shape = w.objects[1].clone();
        let i = Intersection::new(0.5, shape);
        let binding = Intersections::new(&[i.clone()]);
        let comps = prepare_computations(&i, &r, &binding);
        let c = w.shade_hit(&comps, 1);
        assert_eq!(c, Color::new(0.90498, 0.90498, 0.90498));
    }
    #[test]
    fn shade_hit_is_given_an_intersection_in_shadow() {
        let mut w = WorldBuilder::new();
        w.lights = vec![Light::point_light(
            &new_point(0.0, 0.0, -10.0),
            &Color::new(1.0, 1.0, 1.0),
        )];

        let s1 = new_sphere().build();
        w.objects.push(s1);

        let s2 = new_sphere().translate(0.0, 0.0, 10.0).build();
        w.objects.push(s2.clone());

        let w = w.build();

        let r = Ray::new(new_point(0.0, 0.0, 5.0), new_vector(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, s2);
        let binding = Intersections::new(&[i.clone()]);
        let comps = prepare_computations(&i, &r, &binding);
        let c = w.shade_hit(&comps, 1);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }
    #[test]
    fn the_color_when_a_ray_misses() {
        let w = default_world();
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 1.0, 0.0));
        let c = w.color_at(&r, 1);
        assert_eq!(c, Color::new(0.0, 0.0, 0.0));
    }
    #[test]
    fn the_color_when_a_ray_hits() {
        let w = default_world();
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let c = w.color_at(&r, 1);
        assert_eq!(c, Color::new(0.38066, 0.47583, 0.2855));
    }
    #[test]
    fn the_color_with_an_intersection_behind_the_ray() {
        let (objects, lights) = default_world_components();

        // Set `ambient` to 1.0 for both objects.
        let objects: Vec<Object> = objects
            .into_iter()
            .map(|o| o.ambient(1.0).build())
            .collect();

        // Grab the inner object.
        let inner_sphere = objects[1].clone();

        let new_world = World {
            lights: lights.into(),
            objects: objects.into(),
        };
        let r = Ray::new(new_point(0.0, 0.0, 0.75), new_vector(0.0, 0.0, -1.0));
        let c = new_world.color_at(&r, 1);
        assert_eq!(c, inner_sphere.get_material().color);
    }

    #[test]
    fn there_is_no_shadow_when_nothing_is_collinear_with_point_and_light() {
        let w = default_world();
        let p = new_point(0.0, 10.0, 0.0);
        assert!(!w.is_shadowed(&p));
    }
    #[test]
    fn the_shadow_when_an_object_is_between_the_point_and_the_light() {
        let w = default_world();
        let p = new_point(10.0, -10.0, 10.0);
        assert!(w.is_shadowed(&p));
    }
    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_light() {
        let w = default_world();
        let p = new_point(-20.0, 20.0, -20.0);
        assert!(!w.is_shadowed(&p));
    }
    #[test]
    fn there_is_no_shadow_when_an_object_is_behind_the_point() {
        let w = default_world();
        let p = new_point(-2.0, 2.0, -2.0);
        assert!(!w.is_shadowed(&p));
    }

    #[test]
    fn the_reflected_color_for_a_nonreflective_material() {
        let (objects, _) = default_world_components();
        let shape = objects[1].clone().ambient(1.0).build();

        let mut w = default_world().into_builder();
        let _ = std::mem::replace(w.objects.get_mut(1).unwrap(), shape.clone());
        let w = w.build();

        let r = Ray::new(new_point(0.0, 0.0, 0.0), new_vector(0.0, 0.0, 1.0));
        let i = Intersection::new(1.0, shape);
        let binding = Intersections::new(&[i.clone()]);
        let comps = prepare_computations(&i, &r, &binding);
        let color = w.reflected_color(&comps, 1);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }
    #[test]
    fn the_reflected_color_for_a_reflective_material() {
        let mut w = default_world().into_builder();
        let shape = new_plane()
            .reflective(0.5)
            .translate(0.0, -1.0, 0.0)
            .build();
        w.objects.push(shape.clone());
        let w = w.build();

        let r = Ray::new(
            new_point(0.0, 0.0, -3.0),
            new_vector(0.0, -(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), shape);
        let binding = Intersections::new(&[i.clone()]);
        let comps = prepare_computations(&i, &r, &binding);
        let color = w.reflected_color(&comps, 1);
        assert_eq!(color, Color::new(0.19032, 0.2379, 0.14274));
    }
    #[test]
    fn shade_hit_with_a_reflective_material() {
        let mut w = default_world().into_builder();
        let shape = new_plane()
            .reflective(0.5)
            .translate(0.0, -1.0, 0.0)
            .build();
        w.objects.push(shape.clone());
        let w = w.build();

        let r = Ray::new(
            new_point(0.0, 0.0, -3.0),
            new_vector(0.0, -(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), shape);
        let binding = Intersections::new(&[i.clone()]);
        let comps = prepare_computations(&i, &r, &binding);
        let color = w.shade_hit(&comps, 1);
        assert_eq!(color, Color::new(0.87677, 0.92436, 0.82918));
    }
    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = World::builder();
        w.light(Light::point_light(
            &new_point(0.0, 0.0, 0.0),
            &Color::new(1.0, 1.0, 1.0),
        ));

        let builder = new_plane().reflective(1.0);
        let lower = builder.clone().translate(0.0, -1.0, 0.0).build();
        let upper = builder.translate(0.0, 1.0, 0.0).build();

        w.objects.push(lower);
        w.objects.push(upper);
        let w = w.build();

        let r = Ray::new(new_point(0.0, 0.0, 0.0), new_vector(0.0, 1.0, 0.0));

        // Simply test that the function returns when the ray is locked between two mirrors.
        #[allow(unused_assignments)]
        let mut color = Color::new(0.0, 0.0, 0.0);
        color = w.color_at(&r, 1);
        assert_ne!(color, Color::new(0.0, 0.0, 0.0));
    }
    #[test]
    fn the_reflected_color_at_the_maximum_recursive_depth() {
        let mut w = default_world().into_builder();

        let shape = new_plane()
            .reflective(0.5)
            .translate(0.0, -1.0, 0.0)
            .build();
        w.object(shape.clone());
        let w = w.build();

        let r = Ray::new(
            new_point(0.0, 0.0, -3.0),
            new_vector(0.0, -(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2.0_f64.sqrt(), shape);
        let binding = Intersections::new(&[i.clone()]);
        let comps = prepare_computations(&i, &r, &binding);
        let color = w.reflected_color(&comps, 0);
        assert_eq!(color, Color::new(0.0, 0.0, 0.0));
    }
    #[test]
    fn the_refracted_color_with_an_opaque_surface() {
        let w = default_world();
        let shape = w.objects[0].clone();
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
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
        let w = default_world();
        let shape = w.objects[0].clone();
        shape.get_material().transparency = 1.0;
        shape.get_material().refractive_index = 1.5;
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
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
        let w = default_world();
        let shape = w.objects[0].clone();
        shape.get_material().transparency = 1.0;
        shape.get_material().refractive_index = 1.5;
        let r = Ray::new(
            new_point(0.0, 0.0, 2.0_f64.sqrt() / 2.0),
            new_vector(0.0, 1.0, 0.0),
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
        let (objects, lights) = default_world_components();
        let first = objects[0]
            .clone()
            .ambient(1.0)
            .pattern(Pattern::test_pattern_default())
            .build();
        let second = objects[1]
            .clone()
            .transparency(1.0)
            .refractive_index(1.5)
            .build();

        let A = first.clone();
        let B = second.clone();

        let mut wb = WorldBuilder::new();
        wb.light(lights[0]).object(first).object(second);
        let w = wb.build();

        let r = Ray::new(new_point(0.0, 0.0, 0.1), new_vector(0.0, 1.0, 0.0));
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
        let floor = new_plane()
            .translate(0.0, -1.0, 0.0)
            .transparency(0.5)
            .refractive_index(1.5)
            .build();

        let ball = new_sphere()
            .translate(0.0, -3.5, -0.5)
            .color(&Color::new(1.0, 0.0, 0.0))
            .ambient(0.5)
            .build();

        let mut w = default_world().into_builder();
        w.object(floor.clone()).object(ball);
        let w = w.build();

        let r = Ray::new(
            new_point(0.0, 0.0, -3.0),
            new_vector(0.0, -(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let xs = Intersections::new(&[Intersection::new(2.0_f64.sqrt(), floor)]);
        let comps = prepare_computations(&xs.list[0], &r, &xs);
        let color = w.shade_hit(&comps, 5);
        assert_eq!(color, Color::new(0.93642, 0.68642, 0.68642));
    }
    #[test]
    fn shade_hit_with_a_reflective_transparent_material() {
        let floor = new_plane()
            .translate(0.0, -1.0, 0.0)
            .reflective(0.5)
            .transparency(0.5)
            .refractive_index(1.5)
            .build();

        let ball = new_sphere()
            .translate(0.0, -3.5, -0.5)
            .color(&Color::new(1.0, 0.0, 0.0))
            .ambient(0.5)
            .build();

        let mut w = default_world().into_builder();
        w.object(floor.clone()).object(ball);
        let w = w.build();

        let r = Ray::new(
            new_point(0.0, 0.0, -3.0),
            new_vector(0.0, -(2.0_f64.sqrt()) / 2.0, 2.0_f64.sqrt() / 2.0),
        );
        let xs = Intersections::new(&[Intersection::new(2.0_f64.sqrt(), floor)]);
        let comps = prepare_computations(&xs.list[0], &r, &xs);
        let color = w.shade_hit(&comps, 5);
        assert_eq!(color, Color::new(0.93391, 0.69643, 0.69243));
    }
}
