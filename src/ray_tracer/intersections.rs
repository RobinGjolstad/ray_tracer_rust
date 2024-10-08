use crate::ray_tracer::{
    rays::Ray,
    shapes::Object,
    tuples::{Point, Tuple, Vector},
    utils,
};

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Intersection {
    t: f64,
    object: Object,
}
impl Intersection {
    pub(crate) fn new(time: f64, object: Object) -> Self {
        Intersection { t: time, object }
    }
    pub(crate) fn get_time(&self) -> f64 {
        self.t
    }
    #[cfg(test)]
    pub(crate) fn get_object_raw(&self) -> &Object {
        &self.object
    }
    pub(crate) fn get_object(&self) -> &Object {
        &self.object
    }
}

#[derive(Debug, PartialEq, Clone)]
pub(crate) struct Intersections {
    pub(crate) list: Vec<Intersection>,
}
impl Intersections {
    #[cfg(test)]
    pub(crate) fn new(intersect_list: &[Intersection]) -> Self {
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
    #[cfg(test)]
    pub(crate) fn count(&self) -> usize {
        self.list.len()
    }
    #[cfg(test)]
    pub(crate) fn get_element(&self, index: usize) -> Option<Intersection> {
        if index <= self.list.len() {
            Some(self.list[index].clone())
        } else {
            None
        }
    }
    pub(crate) fn hit(&self) -> Option<Intersection> {
        let mut list = self.list.clone();
        list.retain(|x| x.t.is_sign_positive());
        list.iter()
            .min_by(|&x, &y| x.t.partial_cmp(&y.t).unwrap())
            .cloned()
    }
    pub(crate) fn put_elements(&mut self, intersection: &[Intersection]) {
        self.list.extend(intersection.to_owned());
        self.sort();
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct IntersectComp {
    pub(crate) t: f64,
    pub(crate) object: Object,
    pub(crate) point: Point,
    pub(crate) eyev: Vector,
    pub(crate) normalv: Vector,
    pub(crate) reflectv: Vector,
    pub(crate) inside: bool,
    pub(crate) over_point: Point,
    pub(crate) under_point: Point,
    pub(crate) n1: f64,
    pub(crate) n2: f64,
}
pub(crate) fn prepare_computations(
    intersection: &Intersection,
    ray: &Ray,
    intersections: &Intersections,
) -> IntersectComp {
    let mut comps = IntersectComp {
        t: intersection.t,
        object: intersection.object.clone(),
        point: ray.position(intersection.t),
        eyev: -(ray.get_direction()),
        normalv: intersection
            .get_object()
            .normal_at(ray.position(intersection.t)),
        reflectv: Vector::new_vector(0.0, 0.0, 0.0),
        inside: false,
        over_point: Point::new_point(0.0, 0.0, 0.0),
        under_point: Point::new_point(0.0, 0.0, 0.0),
        n1: 0.0,
        n2: 0.0,
    };

    let point = ray.position(intersection.t);
    let normalv = intersection.get_object().normal_at(point);
    let eyev = -(ray.get_direction());
    if Tuple::dot(&normalv, &eyev) < 0.0 {
        comps.inside = true;
        comps.normalv = -comps.normalv;
    }

    comps.reflectv = Vector::reflect(&ray.direction, &comps.normalv);

    comps.over_point = comps.point + comps.normalv * utils::EPSILON;
    comps.under_point = comps.point - comps.normalv * utils::EPSILON;

    (comps.n1, comps.n2) = get_refractive_index_from_intersections(intersection, intersections);

    comps
}

/// Get the refractive index of two objects at an intersection.
/// n1: The object where a ray is "leaving".
/// n2: The object where a ray is "entering".
///
/// Returns (n1, n2)
fn get_refractive_index_from_intersections(
    intersected_object: &Intersection,
    intersection_collection: &Intersections,
) -> (f64, f64) {
    let mut containers: Vec<Object> = Vec::new();
    let mut n1 = 1.0;
    let mut n2 = 1.0;

    for i in &intersection_collection.list {
        if i == intersected_object && !containers.is_empty() {
            n1 = containers.last().unwrap().get_material().refractive_index;
        }

        if containers.contains(&i.object) {
            containers.retain(|item| *item != i.object);
        } else {
            containers.push(i.object.clone());
        }

        if i == intersected_object {
            if !containers.is_empty() {
                n2 = containers.last().unwrap().get_material().refractive_index;
            }

            break;
        }
    }

    (n1, n2)
}

/// Calculate the `reflectance` of an intersection.
/// Reflectance describes a fraction of light being reflected.
pub(crate) fn schlick(comps: &IntersectComp) -> f64 {
    // Find the cosine of the angle between the eye and normal vectors
    let mut cos = Tuple::dot(&comps.eyev, &comps.normalv);

    // Total internal reflection can only occur if n1 > n2
    if comps.n1 > comps.n2 {
        let n = comps.n1 / comps.n2;
        let sin2_t = n.powi(2) * (1.0 - cos.powi(2));

        if sin2_t > 1.0 {
            return 1.0;
        } else {
            // Compute cosine of theta_t using trig identity
            let cos_t = (1.0 - sin2_t).sqrt();

            // when n1 > n2, use cos(theta_t) instead
            cos = cos_t;
        }
    }

    let r0 = ((comps.n1 - comps.n2) / (comps.n1 + comps.n2)).powi(2);

    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
}

#[cfg(test)]
mod tests {

    use crate::ray_tracer::{
        shapes::*,
        transformations::Transform,
        utils::{is_float_equal, EPSILON},
    };

    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = new_sphere();
        let i = Intersection::new(3.5, s.clone());

        assert!(is_float_equal(&i.t, 3.5));
        assert_eq!(i.object, s);
    }
    #[test]
    fn aggregating_intersections() {
        let s = new_sphere();
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s);
        let xs = Intersections::new(&vec![i1, i2]);
        assert_eq!(xs.count(), 2);
        assert!(is_float_equal(&xs.get_element(0).unwrap().t, 1.0));
        assert!(is_float_equal(&xs.get_element(1).unwrap().t, 2.0));
    }
    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = new_sphere();
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s);
        let xs = Intersections::new(&vec![i2, i1.clone()]);
        let i = xs.hit();
        assert_eq!(i, Some(i1));
    }
    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = new_sphere();
        let i1 = Intersection::new(-1.0, s.clone());
        let i2 = Intersection::new(1.0, s);
        let xs = Intersections::new(&vec![i1, i2.clone()]);
        let i = xs.hit();
        assert_eq!(i, Some(i2));
    }
    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = new_sphere();
        let i1 = Intersection::new(-2.0, s.clone());
        let i2 = Intersection::new(-1.0, s);
        let xs = Intersections::new(&vec![i2, i1]);
        let i = xs.hit();
        assert_eq!(i, None);
    }
    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = new_sphere();
        let i1 = Intersection::new(5.0, s.clone());
        let i2 = Intersection::new(7.0, s.clone());
        let i3 = Intersection::new(-3.0, s.clone());
        let i4 = Intersection::new(2.0, s);
        let xs = Intersections::new(&vec![i1, i2, i3, i4.clone()]);
        let i = xs.hit();
        assert_eq!(i, Some(i4));
    }
    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let shape = new_sphere();
        let i = Intersection::new(4.0, shape);
        let comps = prepare_computations(
            &i,
            &r,
            &Intersections {
                list: vec![i.clone()],
            },
        );
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
        let shape = new_sphere();
        let i = Intersection::new(4.0, shape);
        let comps = prepare_computations(&i.clone(), &r, &Intersections { list: vec![i] });
        assert!(!comps.inside);
    }
    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let shape = new_sphere();
        let i = Intersection::new(1.0, shape);
        let comps = prepare_computations(&i.clone(), &r, &Intersections { list: vec![i] });
        assert_eq!(comps.point, Tuple::new_point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Tuple::new_vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::new_vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut shape = new_sphere();
        shape.set_transform(&Transform::translate(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, shape);
        let comps = prepare_computations(&i.clone(), &r, &Intersections { list: vec![i] });
        assert!(comps.over_point.z < -utils::EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let shape = new_plane();
        let r = Ray::new(
            Point::new_point(0.0, 1.0, -1.0),
            Vector::new_vector(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let i = Intersection::new(f64::sqrt(2.0), shape);
        let comps = prepare_computations(&i.clone(), &r, &Intersections { list: vec![i] });
        assert_eq!(
            comps.reflectv,
            Vector::new_vector(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)
        );
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        #[allow(non_snake_case)]
        let mut A = glass_sphere();
        A.set_transform(&Transform::scaling(2.0, 2.0, 2.0));
        A.get_material().refractive_index = 1.5;

        #[allow(non_snake_case)]
        let mut B = glass_sphere();
        B.set_transform(&Transform::translate(0.0, 0.0, -0.25));
        B.get_material().refractive_index = 2.0;

        #[allow(non_snake_case)]
        let mut C = glass_sphere();
        C.set_transform(&Transform::translate(0.0, 0.0, 0.25));
        C.get_material().refractive_index = 2.5;

        let r = Ray::new(
            Point::new_point(0.0, 0.0, -4.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );
        let xs = Intersections {
            list: vec![
                Intersection::new(2.0, A.clone()),
                Intersection::new(2.75, B.clone()),
                Intersection::new(3.25, C.clone()),
                Intersection::new(4.75, B.clone()),
                Intersection::new(5.25, C.clone()),
                Intersection::new(6.0, A.clone()),
            ],
        };

        let results = [
            [1.0, 1.5],
            [1.5, 2.0],
            [2.0, 2.5],
            [2.5, 2.5],
            [2.5, 1.5],
            [1.5, 1.0],
        ];

        for (i, result) in results.iter().enumerate() {
            let comps = prepare_computations(&xs.list[i], &r, &xs);
            assert_eq!(comps.n1, result[0]);
            assert_eq!(comps.n2, result[1]);
        }
    }

    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let r = Ray::new(
            Point::new_point(0.0, 0.0, -5.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );
        let mut shape = glass_sphere();
        shape.set_transform(&Transform::translate(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, shape);
        let xs = Intersections::new(&[i.clone()]);
        let comps = prepare_computations(&i, &r, &xs);
        assert!(comps.under_point.z > EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }
    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let shape = glass_sphere();
        let r = Ray::new(
            Point::new_point(0.0, 0.0, 2.0_f64.sqrt() / 2.0),
            Vector::new_vector(0.0, 1.0, 0.0),
        );
        let xs = Intersections::new(&[
            Intersection::new(-(2.0_f64.sqrt()) / 2.0, shape.clone()),
            Intersection::new(2.0_f64.sqrt() / 2.0, shape),
        ]);
        let comps = prepare_computations(&xs.list[1], &r, &xs);
        let reflectance = schlick(&comps);
        assert!(is_float_equal(&reflectance, 1.0));
    }
    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let shape = glass_sphere();
        let r = Ray::new(
            Point::new_point(0.0, 0.0, 0.0),
            Vector::new_vector(0.0, 1.0, 0.0),
        );
        let xs = Intersections::new(&[
            Intersection::new(-1.0, shape.clone()),
            Intersection::new(2.0, shape),
        ]);
        let comps = prepare_computations(&xs.list[1], &r, &xs);
        let reflectance = schlick(&comps);
        assert!(is_float_equal(&reflectance, 0.04));
    }
    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
        let shape = glass_sphere();
        let r = Ray::new(
            Point::new_point(0.0, 0.99, -2.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );
        let xs = Intersections::new(&[Intersection::new(1.8589, shape)]);
        let comps = prepare_computations(&xs.list[0], &r, &xs);
        let reflectance = schlick(&comps);
        assert!(is_float_equal(&reflectance, 0.48873));
    }
}
