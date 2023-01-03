use std::ops::Deref;

use crate::{
    rays::Ray,
    shapes::{Object, Shapes},
    tuples::{Point, Tuple, Vector},
    utils,
};

#[derive(Debug, PartialEq, Clone)]
pub struct Intersection {
    t: f64,
    object: Object,
}
impl Intersection {
    pub fn new(time: f64, object: Object) -> Self {
        Intersection {
            t: time,
            object: object,
        }
    }
    pub fn get_time(&self) -> f64 {
        self.t
    }
    pub fn get_object_raw(&self) -> Object {
        self.object.clone()
    }
    pub fn get_object(&self) -> Object {
        self.object.clone()
    }
}

#[derive(Debug, PartialEq)]
pub struct Intersections {
    pub list: Vec<Intersection>,
}
impl Intersections {
    pub fn new(intersect_list: &Vec<Intersection>) -> Self {
        let mut i = Intersections {
            list: intersect_list.to_vec(),
        };
        i.sort();
        i
    }
    fn sort(&mut self) {
        self.list
            .sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    }
    pub fn count(&self) -> usize {
        self.list.len()
    }
    pub fn get_element(&self, index: usize) -> Option<Intersection> {
        if index <= self.list.len() {
            Some(self.list[index].clone())
        } else {
            None
        }
    }
    pub fn hit(&self) -> Option<Intersection> {
        let mut list = self.list.clone();
        list.retain(|x| x.t.is_sign_positive());
        if let Some(int) = list.iter().min_by(|&x, &y| x.t.partial_cmp(&y.t).unwrap()) {
            Some(int.clone())
        } else {
            None
        }
    }
    pub fn put_elements(&mut self, intersection: &Vec<Intersection>) {
        self.list.extend(*intersection.clone());
        self.sort();
    }
}

#[derive(Debug, PartialEq)]
pub struct IntersectComp {
    pub t: f64,
    pub object: Object,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
    pub over_point: Tuple,
}
pub fn prepare_computations(intersection: &Intersection, ray: &Ray) -> IntersectComp {
    let mut comps = IntersectComp {
        t: intersection.t,
        object: intersection.object,
        point: ray.position(intersection.t),
        eyev: -(ray.get_direction()),
        normalv: intersection
            .get_object()
            .normal(ray.position(intersection.t)),
        inside: false,
        over_point: Tuple::new(0.0, 0.0, 0.0, 0.0),
    };
    let point = ray.position(intersection.t);
    let normalv = intersection.get_object().normal(point);
    let eyev = -(ray.get_direction());
    if Tuple::dot(&normalv, &eyev) < 0.0 {
        comps.inside = true;
        comps.normalv = -comps.normalv;
    }
    comps.over_point = comps.point + comps.normalv * utils::EPSILON;

    comps
}

#[cfg(test)]
mod tests {
    use crate::{
        rays::Ray,
        shapes::sphere::Sphere,
        transformations::Transform,
        tuples::Tuple,
        utils::{self, is_float_equal},
    };

    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Object::Sphere(Sphere::new());
        let i = Intersection::new(3.5, s);

        assert!(is_float_equal(&i.t, 3.5));
        assert_eq!(i.object, s);
    }
    #[test]
    fn aggregating_intersections() {
        let s = Object::Sphere(Sphere::new());
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = Intersections::new(&vec![i1, i2]);
        assert_eq!(xs.count(), 2);
        assert!(is_float_equal(&xs.get_element(0).unwrap().t, 1.0));
        assert!(is_float_equal(&xs.get_element(1).unwrap().t, 2.0));
    }
    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = Object::Sphere(Sphere::new());
        let i1 = Intersection::new(1.0, s);
        let i2 = Intersection::new(2.0, s);
        let xs = Intersections::new(&vec![i2, i1]);
        let i = xs.hit();
        assert_eq!(i, Some(i1));
    }
    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = Object::Sphere(Sphere::new());
        let i1 = Intersection::new(-1.0, s);
        let i2 = Intersection::new(1.0, s);
        let xs = Intersections::new(&vec![i1, i2]);
        let i = xs.hit();
        assert_eq!(i, Some(i2));
    }
    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = Object::Sphere(Sphere::new());
        let i1 = Intersection::new(-2.0, s);
        let i2 = Intersection::new(-1.0, s);
        let xs = Intersections::new(&vec![i2, i1]);
        let i = xs.hit();
        assert_eq!(i, None);
    }
    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Object::Sphere(Sphere::new());
        let i1 = Intersection::new(5.0, s);
        let i2 = Intersection::new(7.0, s);
        let i3 = Intersection::new(-3.0, s);
        let i4 = Intersection::new(2.0, s);
        let xs = Intersections::new(&vec![i1, i2, i3, i4]);
        let i = xs.hit();
        assert_eq!(i, Some(i4));
    }
    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let shape = Sphere::new();
        let i = Intersection::new(4.0, Object::Sphere(shape));
        let comps = prepare_computations(&i, &r);
        assert!(is_float_equal(&comps.t, i.t));
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Tuple::new_point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Tuple::new_vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::new_vector(0.0, 0.0, -1.0));
    }
    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let shape = Sphere::new();
        let i = Intersection::new(4.0, Object::Sphere(shape));
        let comps = prepare_computations(&i, &r);
        assert_eq!(comps.inside, false);
    }
    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let shape = Sphere::new();
        let i = Intersection::new(1.0, Object::Sphere(shape));
        let comps = prepare_computations(&i, &r);
        assert_eq!(comps.point, Tuple::new_point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Tuple::new_vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::new_vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut shape = Sphere::new();
        shape.set_transform(&Transform::translate(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, Object::Sphere(shape));
        let comps = prepare_computations(&i, &r);
        assert!(comps.over_point.z < -utils::EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
}