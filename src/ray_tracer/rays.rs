use crate::ray_tracer::{
    matrices_new::Mat,
    shapes::Object,
    tuples_new::{Point, Vector},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    #[must_use]
    pub const fn new(origin: Point, direction: Vector) -> Self {
        Self { origin, direction }
    }
    pub(crate) const fn get_direction(&self) -> Vector {
        self.direction
    }
    pub(crate) fn position(&self, time: f64) -> Point {
        self.origin + self.direction * time
    }
    fn global_to_local(&self, object: &Object) -> Self {
        self.transform(
            object
                .get_transform()
                .inverse
                .as_ref()
                .expect("The objects transformation matrix should be inverted."),
        )
    }

    pub fn intersect(&self, object: &crate::ray_tracer::shapes::Object, intersection_list: &mut crate::ray_tracer::intersections::IntersectionsSoA) {
        let local_ray = self.transform(&object.get_transform().inverse.expect("Shape's transformation matrix should be inverted and transposed before use."));
        object.local_intersect(local_ray, intersection_list);
    }

    #[must_use] pub fn intersect_world(&self, world: &crate::ray_tracer::world::World) -> crate::ray_tracer::intersections::IntersectionsSoA {
        let mut xs = crate::ray_tracer::intersections::IntersectionsSoA::default();
        for object in world.objects.iter() {
            self.intersect(object, &mut xs);
        }
        xs.sort();
        xs
    }

    pub(crate) fn transform(&self, transformation: &Mat<4>) -> Self {
        Self {
            origin: *transformation * self.origin,
            direction: *transformation * self.direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray_tracer::{
        shapes::new_sphere,
        transformations::Transform,
        tuples_new::{new_point, new_vector},
        utils::is_float_equal,
        intersections::IntersectionsSoA,
    };

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = new_point(1.0, 2.0, 3.0);
        let direction = new_vector(4.0, 5.0, 6.0);

        let r = Ray::new(origin, direction);

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(new_point(2.0, 3.0, 4.0), new_vector(1.0, 0.0, 0.0));

        assert_eq!(r.position(0.0), new_point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), new_point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), new_point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), new_point(4.5, 3.0, 4.0));
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let s = new_sphere().build();
        let mut xs = IntersectionsSoA::default();
        r.intersect(&s, &mut xs);
        assert_eq!(xs.len(), 2);
        assert!(is_float_equal(&xs.ts[0], 4.0));
        assert!(is_float_equal(&xs.ts[1], 6.0));
    }
    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(new_point(0.0, 1.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let s = new_sphere().build();
        let mut xs = IntersectionsSoA::default();
        r.intersect(&s, &mut xs);
        assert_eq!(xs.len(), 2);
        assert!(is_float_equal(&xs.ts[0], 5.0));
        assert!(is_float_equal(&xs.ts[1], 5.0));
    }
    #[test]
    fn a_ray_misses_a_square() {
        let r = Ray::new(new_point(0.0, 2.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let s = new_sphere().build();
        let mut xs = IntersectionsSoA::default();
        r.intersect(&s, &mut xs);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(new_point(0.0, 0.0, 0.0), new_vector(0.0, 0.0, 1.0));
        let s = new_sphere().build();
        let mut xs = IntersectionsSoA::default();
        r.intersect(&s, &mut xs);
        assert_eq!(xs.len(), 2);
        assert!(is_float_equal(&xs.ts[0], -1.0));
        assert!(is_float_equal(&xs.ts[1], 1.0));
    }
    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(new_point(0.0, 0.0, 5.0), new_vector(0.0, 0.0, 1.0));
        let s = new_sphere().build();
        let mut xs = IntersectionsSoA::default();
        r.intersect(&s, &mut xs);
        assert_eq!(xs.len(), 2);
        assert!(is_float_equal(&xs.ts[0], -6.0));
        assert!(is_float_equal(&xs.ts[1], -4.0));
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let s = new_sphere().build();
        let mut xs = IntersectionsSoA::default();
        r.intersect(&s, &mut xs);
        assert_eq!(xs.len(), 2);
        assert_eq!(&xs.objects[0], &s);
        assert_eq!(&xs.objects[1], &s);
    }
    #[test]
    fn translating_a_ray() {
        let r = Ray::new(new_point(1.0, 2.0, 3.0), new_vector(0.0, 1.0, 0.0));
        let m = Transform::translate(3.0, 4.0, 5.0);
        let r2 = r.transform(&m.matrix);
        assert_eq!(r2.origin, new_point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, new_vector(0.0, 1.0, 0.0));
    }
    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(new_point(1.0, 2.0, 3.0), new_vector(0.0, 1.0, 0.0));
        let m = Transform::scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(&m.matrix);
        assert_eq!(r2.origin, new_point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, new_vector(0.0, 3.0, 0.0));
    }
    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let s = new_sphere().scale(2.0, 2.0, 2.0).build();
        let mut xs = IntersectionsSoA::default();
        r.intersect(&s, &mut xs);
        assert_eq!(xs.len(), 2);
        assert!(is_float_equal(&xs.ts[0], 3.0));
        assert!(is_float_equal(&xs.ts[1], 7.0));
    }
    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let s = new_sphere().translate(5.0, 0.0, 0.0).build();
        let mut xs = IntersectionsSoA::default();
        r.intersect(&s, &mut xs);
        assert_eq!(xs.len(), 0);
    }
}
