use crate::ray_tracer::{
    intersections::{Intersection, Intersections},
    matrices::Matrix,
    shapes::Object,
    tuples::{Point, Vector},
    world::World,
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
        self.transform(&object.get_transform().get_inverted().unwrap())
    }

    pub(crate) fn intersect(&self, object: &Object, intersection_list: &mut Vec<Intersection>) {
        let local_ray = if let Object::Group(_) = object {
            // Do not convert ray to local space if object is a group.
            // Conversions are taken care of in the group's intersect method.
            *self
        } else {
            self.global_to_local(object)
        };

        object.local_intersect(local_ray, intersection_list);
    }

    pub(crate) fn intersect_world(&self, world: &World) -> Intersections {
        let mut intersections = Intersections::default();
        world
            .objects
            .iter()
            .for_each(|object| self.intersect(object, &mut intersections.list));

        intersections.sort();
        intersections
    }

    pub(crate) fn intersect_world_first(&self, world: &World) -> Intersections {
        let mut intersections = Intersections::default();

        // "Manual" loop to be able to break out early.
        for object in world.objects.iter() {
            self.intersect(object, &mut intersections.list);
            if !intersections.list.is_empty() {
                // Stop on first intersection.
                break;
            }
        }

        intersections.sort();
        intersections
    }

    pub(crate) fn transform(&self, transformation: &Matrix) -> Self {
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
        shapes::new_sphere, transformations::Transform, tuples::{new_point, new_vector}, utils::is_float_equal
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
        let r = Ray::new(
            new_point(2.0, 3.0, 4.0),
            new_vector(1.0, 0.0, 0.0),
        );

        assert_eq!(r.position(0.0), new_point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), new_point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), new_point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), new_point(4.5, 3.0, 4.0));
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(
            new_point(0.0, 0.0, -5.0),
            new_vector(0.0, 0.0, 1.0),
        );
        let s = new_sphere();
        let mut xs = Intersections::default();
        r.intersect(&s, &mut xs.list);
        assert_eq!(xs.count(), 2);
        assert!(is_float_equal(&xs.get_element(0).unwrap().get_time(), 4.0));
        assert!(is_float_equal(&xs.get_element(1).unwrap().get_time(), 6.0));
    }
    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(
            new_point(0.0, 1.0, -5.0),
            new_vector(0.0, 0.0, 1.0),
        );
        let s = new_sphere();
        let mut xs = Intersections::default();
        r.intersect(&s, &mut xs.list);
        assert_eq!(xs.count(), 2);
        assert!(is_float_equal(&xs.get_element(0).unwrap().get_time(), 5.0));
        assert!(is_float_equal(&xs.get_element(1).unwrap().get_time(), 5.0));
    }
    #[test]
    fn a_ray_misses_a_square() {
        let r = Ray::new(
            Tuple::new_point(0.0, 2.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = new_sphere();
        let mut xs = Intersections::default();
        r.intersect(&s, &mut xs.list);
        assert_eq!(xs.count(), 0);
    }
    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = new_sphere();
        let mut xs = Intersections::default();
        r.intersect(&s, &mut xs.list);
        assert_eq!(xs.count(), 2);
        assert!(is_float_equal(&xs.get_element(0).unwrap().get_time(), -1.0));
        assert!(is_float_equal(&xs.get_element(1).unwrap().get_time(), 1.0));
    }
    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = new_sphere();
        let mut xs = Intersections::default();
        r.intersect(&s, &mut xs.list);
        assert_eq!(xs.count(), 2);
        assert!(is_float_equal(&xs.get_element(0).unwrap().get_time(), -6.0));
        assert!(is_float_equal(&xs.get_element(1).unwrap().get_time(), -4.0));
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = new_sphere();
        let mut xs = Intersections::default();
        r.intersect(&s, &mut xs.list);
        assert_eq!(xs.count(), 2);
        dbg!(&s);
        dbg!(&xs);
        assert_eq!(*xs.get_element(0).unwrap().get_object_raw(), s);
        assert_eq!(*xs.get_element(1).unwrap().get_object_raw(), s);
    }
    #[test]
    fn translating_a_ray() {
        let r = Ray::new(
            Tuple::new_point(1.0, 2.0, 3.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        let m = Transform::translate(3.0, 4.0, 5.0);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, Tuple::new_point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Tuple::new_vector(0.0, 1.0, 0.0));
    }
    #[test]
    fn scaling_a_ray() {
        let r = Ray::new(
            Tuple::new_point(1.0, 2.0, 3.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        let m = Transform::scaling(2.0, 3.0, 4.0);
        let r2 = r.transform(&m);
        assert_eq!(r2.origin, Tuple::new_point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Tuple::new_vector(0.0, 3.0, 0.0));
    }
    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut s = new_sphere();
        s.set_transform(&Transform::scaling(2.0, 2.0, 2.0));
        let mut xs = Intersections::default();
        r.intersect(&s, &mut xs.list);
        assert_eq!(xs.count(), 2);
        assert!(is_float_equal(&xs.get_element(0).unwrap().get_time(), 3.0));
        assert!(is_float_equal(&xs.get_element(1).unwrap().get_time(), 7.0));
    }
    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut s = new_sphere();
        s.set_transform(&Transform::translate(5.0, 0.0, 0.0));
        let mut xs = Intersections::default();
        r.intersect(&s, &mut xs.list);
        assert_eq!(xs.count(), 0);
    }
}
