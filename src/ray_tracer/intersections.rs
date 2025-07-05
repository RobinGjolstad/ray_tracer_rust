use crate::ray_tracer::{
    rays::Ray,
    shapes::Object,
    tuples_new::{new_point, new_vector, Point, Vector},
    utils::{self, is_float_equal},
};

#[derive(Debug, PartialEq, Clone)]
pub struct Intersection {
    t: f64,
    object: Object,
}
impl Intersection {
    #[must_use]
    pub const fn new(time: f64, object: Object) -> Self {
        Self { t: time, object }
    }
    pub(crate) const fn get_time(&self) -> f64 {
        self.t
    }
    #[cfg(test)]
    pub(crate) const fn get_object_raw(&self) -> &Object {
        &self.object
    }
    pub(crate) const fn get_object(&self) -> &Object {
        &self.object
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Intersections {
    pub list: Vec<Intersection>,
}
impl Intersections {
    #[must_use]
    pub fn new(intersect_list: &[Intersection]) -> Self {
        let mut i = Self {
            list: intersect_list.to_vec(),
        };
        i.sort();
        i
    }
    pub fn sort(&mut self) {
        self.list
            .sort_unstable_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    }
    #[cfg(test)]
    pub(crate) const fn count(&self) -> usize {
        self.list.len()
    }
    #[cfg(test)]
    pub(crate) fn get_element(&self, index: usize) -> Option<Intersection> {
        if index < self.list.len() {
            Some(self.list[index].clone())
        } else {
            None
        }
    }
    #[must_use]
    pub fn hit(&self) -> Option<Intersection> {
        self.list
            .iter()
            .filter(|x| x.t.is_sign_positive())
            .min_by(|&x, &y| x.t.partial_cmp(&y.t).unwrap())
            .cloned()
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct IntersectionsSoA {
    pub ts: Vec<f64>,
    pub objects: Vec<Object>,
}

impl IntersectionsSoA {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            ts: Vec::new(),
            objects: Vec::new(),
        }
    }

    #[must_use]
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            ts: Vec::with_capacity(cap),
            objects: Vec::with_capacity(cap),
        }
    }

    pub fn push(&mut self, t: f64, object: Object) {
        self.ts.push(t);
        self.objects.push(object);
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.ts.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.ts.is_empty()
    }

    pub fn sort(&mut self) {
        // Sort both arrays by t, keeping them in sync
        let mut indices: Vec<usize> = (0..self.ts.len()).collect();
        indices.sort_by(|&i, &j| self.ts[i].partial_cmp(&self.ts[j]).unwrap());
        let mut new_ts = Vec::with_capacity(self.ts.len());
        let mut new_objects = Vec::with_capacity(self.objects.len());
        for &i in &indices {
            new_ts.push(self.ts[i]);
            new_objects.push(self.objects[i].clone());
        }
        self.ts = new_ts;
        self.objects = new_objects;
    }

    #[must_use]
    pub fn hit(&self) -> Option<(f64, &Object)> {
        let mut min_t = None;
        let mut min_idx = None;
        for (i, &t) in self.ts.iter().enumerate() {
            if t.is_sign_positive() && min_t.is_none_or(|mt| t < mt) {
                min_t = Some(t);
                min_idx = Some(i);
            }
        }
        min_idx.map(|i| (self.ts[i], &self.objects[i]))
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct IntersectComp {
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
#[must_use]
pub fn prepare_computations(
    intersection: &Intersection,
    ray: &Ray,
    intersections: &IntersectionsSoA,
) -> IntersectComp {
    let mut comps = IntersectComp {
        t: intersection.t,
        object: intersection.object.clone(),
        point: ray.position(intersection.t),
        eyev: -(ray.get_direction()),
        normalv: intersection
            .get_object()
            .normal_at(ray.position(intersection.t)),
        reflectv: new_vector(0.0, 0.0, 0.0),
        inside: false,
        over_point: new_point(0.0, 0.0, 0.0),
        under_point: new_point(0.0, 0.0, 0.0),
        n1: 0.0,
        n2: 0.0,
    };

    let point = ray.position(intersection.t);
    let normalv = intersection.get_object().normal_at(point);
    let eyev = -(ray.get_direction());
    if Vector::dot(&normalv, &eyev) < 0.0 {
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
    intersection_collection: &IntersectionsSoA,
) -> (f64, f64) {
    let mut containers: Vec<Object> = Vec::new();
    let mut n1 = 1.0;
    let mut n2 = 1.0;

    for idx in 0..intersection_collection.len() {
        let t = intersection_collection.ts[idx];
        let object = &intersection_collection.objects[idx];
        
        let time_is_equal = is_float_equal(&t, intersected_object.get_time());
        let object_is_equal = object == intersected_object.get_object();

        if time_is_equal && object_is_equal && !containers.is_empty() {
            n1 = containers.last().unwrap().get_material().refractive_index;
        }
        if containers.contains(object) {
            containers.retain(|item| *item != *object);
        } else {
            containers.push(object.clone());
        }
        if time_is_equal && object_is_equal {
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
#[must_use]
pub fn schlick(comps: &IntersectComp) -> f64 {
    // Find the cosine of the angle between the eye and normal vectors
    let mut cos = Vector::dot(&comps.eyev, &comps.normalv);

    // Total internal reflection can only occur if n1 > n2
    if comps.n1 > comps.n2 {
        let n = comps.n1 / comps.n2;
        let sin2_t = n.powi(2) * cos.mul_add(-cos, 1.0);

        if sin2_t > 1.0 {
            return 1.0;
        }

        // Compute cosine of theta_t using trig identity
        let cos_t = (1.0 - sin2_t).sqrt();

        // when n1 > n2, use cos(theta_t) instead
        cos = cos_t;
    }

    let r0 = ((comps.n1 - comps.n2) / (comps.n1 + comps.n2)).powi(2);

    (1.0 - r0).mul_add((1.0 - cos).powi(5), r0)
}

#[cfg(test)]
mod tests {

    use utils::is_float_equal_low_precision;

    use crate::ray_tracer::{shapes::*, utils::is_float_equal};

    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = new_sphere().build();
        let i = Intersection::new(3.5, s.clone());
        let mut xs = IntersectionsSoA::default();
        xs.push(i.get_time(), i.get_object().clone());
        assert!(is_float_equal(&i.get_time(), 3.5));
        assert_eq!(i.get_object(), &s);
    }
    #[test]
    fn aggregating_intersections() {
        let s = new_sphere().build();
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s);
        let mut xs = IntersectionsSoA::default();
        xs.push(i1.get_time(), i1.get_object().clone());
        xs.push(i2.get_time(), i2.get_object().clone());
        assert_eq!(xs.len(), 2);
        assert!(is_float_equal(&xs.ts[0], 1.0));
        assert!(is_float_equal(&xs.ts[1], 2.0));
    }
    #[test]
    fn the_hit_when_all_intersections_have_positive_t() {
        let s = new_sphere().build();
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s);
        let mut xs = IntersectionsSoA::default();
        xs.push(i1.get_time(), i1.get_object().clone());
        xs.push(i2.get_time(), i2.get_object().clone());
        xs.sort();
        let hit = xs.hit();
        assert_eq!(hit, Some((1.0, &i1.get_object().clone())));
    }
    #[test]
    fn the_hit_when_some_intersections_have_negative_t() {
        let s = new_sphere().build();
        let i1 = Intersection::new(-1.0, s.clone());
        let i2 = Intersection::new(1.0, s);
        let mut xs = IntersectionsSoA::default();
        xs.push(i1.get_time(), i1.get_object().clone());
        xs.push(i2.get_time(), i2.get_object().clone());
        xs.sort();
        let hit = xs.hit();
        assert_eq!(hit, Some((1.0, &i2.get_object().clone())));
    }
    #[test]
    fn the_hit_when_all_intersections_have_negative_t() {
        let s = new_sphere().build();
        let i1 = Intersection::new(-2.0, s.clone());
        let i2 = Intersection::new(-1.0, s);
        let mut xs = IntersectionsSoA::default();
        xs.push(i1.get_time(), i1.get_object().clone());
        xs.push(i2.get_time(), i2.get_object().clone());
        xs.sort();
        let hit = xs.hit();
        assert_eq!(hit, None);
    }
    #[test]
    fn the_hit_is_always_the_lowest_nonnegative_intersection() {
        let s = new_sphere().build();
        let i1 = Intersection::new(5.0, s.clone());
        let i2 = Intersection::new(7.0, s.clone());
        let i3 = Intersection::new(-3.0, s.clone());
        let i4 = Intersection::new(2.0, s);
        let mut xs = IntersectionsSoA::default();
        xs.push(i1.get_time(), i1.get_object().clone());
        xs.push(i2.get_time(), i2.get_object().clone());
        xs.push(i3.get_time(), i3.get_object().clone());
        xs.push(i4.get_time(), i4.get_object().clone());
        xs.sort();
        let hit = xs.hit();
        assert_eq!(hit, Some((2.0, &i4.get_object().clone())));
    }
    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let shape = new_sphere().build();
        let i = Intersection::new(4.0, shape);
        let mut xs = IntersectionsSoA::default();
        xs.push(i.get_time(), i.get_object().clone());
        let comps = prepare_computations(&i, &r, &xs);
        assert!(is_float_equal(&comps.t, i.get_time()));
        assert_eq!(comps.object, i.get_object().clone());
        assert_eq!(comps.point, new_point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, new_vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, new_vector(0.0, 0.0, -1.0));
    }
    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let shape = new_sphere().build();
        let i = Intersection::new(4.0, shape);
        let mut xs = IntersectionsSoA::default();
        xs.push(i.get_time(), i.get_object().clone());
        let comps = prepare_computations(&i, &r, &xs);
        assert!(!comps.inside);
    }
    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(new_point(0.0, 0.0, 0.0), new_vector(0.0, 0.0, 1.0));
        let shape = new_sphere().build();
        let i = Intersection::new(1.0, shape);
        let mut xs = IntersectionsSoA::default();
        xs.push(i.get_time(), i.get_object().clone());
        let comps = prepare_computations(&i, &r, &xs);
        assert_eq!(comps.point, new_point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, new_vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, new_vector(0.0, 0.0, -1.0));
        assert!(comps.inside);
    }
    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let shape = new_sphere().translate(0.0, 0.0, 1.0).build();
        let i = Intersection::new(5.0, shape);
        let mut xs = IntersectionsSoA::default();
        xs.push(i.get_time(), i.get_object().clone());
        let comps = prepare_computations(&i, &r, &xs);
        assert!(comps.over_point.z < -utils::EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }
    #[test]
    fn precomputing_the_reflection_vector() {
        let shape = new_plane().build();
        let r = Ray::new(
            new_point(0.0, 1.0, -1.0),
            new_vector(0.0, -f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0),
        );
        let i = Intersection::new(f64::sqrt(2.0), shape);
        let mut xs = IntersectionsSoA::default();
        xs.push(i.get_time(), i.get_object().clone());
        let comps = prepare_computations(&i, &r, &xs);
        assert_eq!(
            comps.reflectv,
            new_vector(0.0, f64::sqrt(2.0) / 2.0, f64::sqrt(2.0) / 2.0)
        );
    }
    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        #[allow(non_snake_case)]
        let A = glass_sphere()
            .scale(2.0, 2.0, 2.0)
            .refractive_index(1.5)
            .build();
        #[allow(non_snake_case)]
        let B = glass_sphere()
            .translate(0.0, 0.0, -0.25)
            .refractive_index(2.0)
            .build();
        #[allow(non_snake_case)]
        let C = glass_sphere()
            .translate(0.0, 0.0, 0.25)
            .refractive_index(2.5)
            .build();
        let r = Ray::new(new_point(0.0, 0.0, -4.0), new_vector(0.0, 0.0, 1.0));
        let mut xs = IntersectionsSoA::default();
        xs.push(2.0, A.clone());
        xs.push(2.75, B.clone());
        xs.push(3.25, C.clone());
        xs.push(4.75, B);
        xs.push(5.25, C);
        xs.push(6.0, A);
        let results = [
            [1.0, 1.5],
            [1.5, 2.0],
            [2.0, 2.5],
            [2.5, 2.5],
            [2.5, 1.5],
            [1.5, 1.0],
        ];
        for (i, result) in results.iter().enumerate() {
            let intersection = Intersection::new(xs.ts[i], xs.objects[i].clone());
            let comps = prepare_computations(&intersection, &r, &xs);
            assert!(is_float_equal_low_precision(&comps.n1, result[0]));
            assert!(is_float_equal_low_precision(&comps.n2, result[1]));
        }
    }
    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let r = Ray::new(new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0));
        let shape = glass_sphere().translate(0.0, 0.0, 1.0).build();
        let i = Intersection::new(5.0, shape);
        let mut xs = IntersectionsSoA::default();
        xs.push(i.get_time(), i.get_object().clone());
        let comps = prepare_computations(&i, &r, &xs);
        assert!(comps.under_point.z > utils::EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }
    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let shape = glass_sphere().build();
        let r = Ray::new(
            new_point(0.0, 0.0, 2.0_f64.sqrt() / 2.0),
            new_vector(0.0, 1.0, 0.0),
        );
        let mut xs = IntersectionsSoA::default();
        xs.push(-(2.0_f64.sqrt()) / 2.0, shape.clone());
        xs.push(2.0_f64.sqrt() / 2.0, shape.clone());
        let comps = prepare_computations(&Intersection::new(2.0_f64.sqrt() / 2.0, shape), &r, &xs);
        let reflectance = schlick(&comps);
        assert!(is_float_equal(&reflectance, 1.0));
    }
    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let shape = glass_sphere().build();
        let r = Ray::new(new_point(0.0, 0.0, 0.0), new_vector(0.0, 1.0, 0.0));
        let mut xs = IntersectionsSoA::default();
        xs.push(-1.0, shape.clone());
        xs.push(2.0, shape.clone());
        let comps = prepare_computations(&Intersection::new(2.0, shape), &r, &xs);
        let reflectance = schlick(&comps);
        assert!(is_float_equal(&reflectance, 0.04));
    }
    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
        let shape = glass_sphere().build();
        let r = Ray::new(new_point(0.0, 0.99, -2.0), new_vector(0.0, 0.0, 1.0));
        let mut xs = IntersectionsSoA::default();
        xs.push(1.8589, shape.clone());
        let comps = prepare_computations(&Intersection::new(1.8589, shape), &r, &xs);
        let reflectance = schlick(&comps);
        assert!(is_float_equal(&reflectance, 0.48873));
    }
}
