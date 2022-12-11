use crate::{
    matrices::Matrix,
    shapes::Object,
    tuples::{Point, Tuple, Vector},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Intersection {
    t: f32,
    object: Object,
}
impl Intersection {
    pub fn new(time: f32, object: Object) -> Self {
        Intersection {
            t: time,
            object: object,
        }
    }
}
pub struct Intersections {
    list: Vec<Intersection>,
}
impl Intersections {
    pub fn new(intersect_list: &[Intersection]) -> Self {
        let mut i = Intersections {
            list: intersect_list.to_vec(),
        };
        i.list
            .sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        i
    }
    pub fn count(&self) -> usize {
        self.list.len()
    }
    pub fn get_element(&self, index: usize) -> Option<Intersection> {
        if index <= self.list.len() {
            Some(self.list[index])
        } else {
            None
        }
    }
    pub fn hit(&self) -> Option<Intersection> {
        let mut list = self.list.clone();
        list.retain(|&x| x.t.is_sign_positive());
        if let Some(int) = list.iter().min_by(|&x, &y| x.t.partial_cmp(&y.t).unwrap()) {
            Some(*int)
        } else {
            None
        }
    }
}

pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Ray {
            origin: origin,
            direction: direction,
        }
    }
    pub fn position(&self, time: f32) -> Point {
        self.origin + self.direction * time
    }

    pub fn intersect(&self, shape: &Object) -> Intersections {
        let ray = self.transform(shape.get_transform().inverse().unwrap());
        let sphere_to_ray = ray.origin - shape.position;
        let a = Tuple::dot(&ray.direction, &ray.direction);
        let b = 2.0 * Tuple::dot(&ray.direction, &sphere_to_ray);
        let c = Tuple::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;
        let discriminant_sqrt = discriminant.sqrt();

        if discriminant < 0.0 {
            Intersections::new(&[])
        } else {
            Intersections::new(&[
                Intersection {
                    t: (-b - discriminant_sqrt) / (2.0 * a),
                    object: *shape,
                },
                Intersection {
                    t: (-b + discriminant_sqrt) / (2.0 * a),
                    object: *shape,
                },
            ])
        }
    }

    pub fn transform(&self, transformation: Matrix) -> Self {
        Ray {
            origin: transformation * self.origin,
            direction: transformation * self.direction,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        matrices::Matrix, transformations::Transform, tuples::Tuple, utils::is_float_equal,
    };

    #[test]
    fn creating_and_querying_a_ray() {
        let origin = Tuple::new_point(1.0, 2.0, 3.0);
        let direction = Tuple::new_vector(4.0, 5.0, 6.0);

        let r = Ray::new(origin, direction);

        assert_eq!(r.origin, origin);
        assert_eq!(r.direction, direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(
            Tuple::new_point(2.0, 3.0, 4.0),
            Tuple::new_vector(1.0, 0.0, 0.0),
        );

        assert_eq!(r.position(0.0), Tuple::new_point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Tuple::new_point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Tuple::new_point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Tuple::new_point(4.5, 3.0, 4.0));
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Object::sphere();
        let xs = r.intersect(&s);
        assert_eq!(xs.count(), 2);
        assert!(is_float_equal(&xs.get_element(0).unwrap().t, 4.0));
        assert!(is_float_equal(&xs.get_element(1).unwrap().t, 6.0));
    }
    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(
            Tuple::new_point(0.0, 1.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Object::sphere();
        let xs = r.intersect(&s);
        assert_eq!(xs.count(), 2);
        assert!(is_float_equal(&xs.get_element(0).unwrap().t, 5.0));
        assert!(is_float_equal(&xs.get_element(1).unwrap().t, 5.0));
    }
    #[test]
    fn a_ray_misses_a_square() {
        let r = Ray::new(
            Tuple::new_point(0.0, 2.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Object::sphere();
        let xs = r.intersect(&s);
        assert_eq!(xs.count(), 0);
    }
    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Object::sphere();
        let xs = r.intersect(&s);
        assert_eq!(xs.count(), 2);
        assert!(is_float_equal(&xs.get_element(0).unwrap().t, -1.0));
        assert!(is_float_equal(&xs.get_element(1).unwrap().t, 1.0));
    }
    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Object::sphere();
        let xs = r.intersect(&s);
        assert_eq!(xs.count(), 2);
        assert!(is_float_equal(&xs.get_element(0).unwrap().t, -6.0));
        assert!(is_float_equal(&xs.get_element(1).unwrap().t, -4.0));
    }

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Object::sphere();
        let i = Intersection::new(3.5, s);

        assert!(is_float_equal(&i.t, 3.5));
        assert_eq!(i.object, s);
    }
    #[test]
    fn aggregating_intersections() {
        let s = Object::sphere();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = Intersections::new(&[i1, i2]);
        assert_eq!(xs.count(), 2);
        assert!(is_float_equal(&xs.get_element(0).unwrap().t, 1.0));
        assert!(is_float_equal(&xs.get_element(1).unwrap().t, 2.0));
    }
    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let s = Object::sphere();
        let xs = r.intersect(&s);
        assert_eq!(xs.count(), 2);
        assert_eq!(xs.get_element(0).unwrap().object, s);
        assert_eq!(xs.get_element(1).unwrap().object, s);
    }
    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Object::sphere();
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = Intersections::new(&[i2, i1]);
        let i = xs.hit();
        assert_eq!(i, Some(i1));
    }
    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Object::sphere();
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(1.0, s);
        let xs = Intersections::new(&[i1, i2]);
        let i = xs.hit();
        assert_eq!(i, Some(i2));
    }
    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Object::sphere();
        let i1 = Intersection::new(-2.0, s);
        let i2 = Intersection::new(-1.0, s);
        let xs = Intersections::new(&[i2, i1]);
        let i = xs.hit();
        assert_eq!(i, None);
    }
    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Object::sphere();
        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s);
        let xs = Intersections::new(&[i1, i2, i3, i4]);
        let i = xs.hit();
        assert_eq!(i, Some(i4));
    }
    #[test]
    fn translating_a_ray() {
        let r = Ray::new(
            Tuple::new_point(1.0, 2.0, 3.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        let m = Transform::translate(3.0, 4.0, 5.0);
        let r2 = r.transform(m);
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
        let r2 = r.transform(m);
        assert_eq!(r2.origin, Tuple::new_point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Tuple::new_vector(0.0, 3.0, 0.0));
    }
    #[test]
    fn a_spheres_default_transformation() {
        let s = Object::sphere();
        assert_eq!(s.get_transform(), Matrix::new_identity());
    }
    #[test]
    fn changing_a_spheres_transformation() {
        let mut s = Object::sphere();
        let t = Transform::translate(2.0, 3.0, 4.0);
        s.set_transform(t);
        assert_eq!(s.get_transform(), t);
    }
    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut s = Object::sphere();
        s.set_transform(Transform::scaling(2.0, 2.0, 2.0));
        let xs = r.intersect(&s);
        assert_eq!(xs.count(), 2);
        assert!(is_float_equal(&xs.get_element(0).unwrap().t, 3.0));
        assert!(is_float_equal(&xs.get_element(1).unwrap().t, 7.0));
    }
    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut s = Object::sphere();
        s.set_transform(Transform::translate(5.0, 0.0, 0.0));
        let xs = r.intersect(&s);
        assert_eq!(xs.count(), 0);
    }
}
