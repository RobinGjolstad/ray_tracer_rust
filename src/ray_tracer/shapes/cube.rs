#![allow(unused)]
use crate::ray_tracer::{
    intersections::Intersection,
    materials::Material,
    matrices_new::Matrix,
    rays::Ray,
    tuples_new::{new_vector, Point, Vector},
    utils::is_float_equal,
};

use super::{BaseShape, Object, Shapes};

#[derive(Debug, Clone, PartialEq)]
pub struct Cube {
    base: BaseShape,
    parent: Option<BaseShape>,
}

impl Cube {
    #[must_use]
    pub fn new() -> Self {
        Self {
            base: BaseShape::default(),
            parent: None,
        }
    }
}

impl Default for Cube {
    fn default() -> Self {
        Self::new()
    }
}

impl Shapes for Cube {
    fn set_position(&mut self, pos: &Point) {
        self.base.position = *pos;
    }
    fn get_position(&self) -> Point {
        self.base.position
    }
    fn set_transform(&mut self, transform: &Matrix<4>) {
        debug_assert!(
            transform.inverse.is_some() && transform.inverse_transpose.is_some(),
            "Transformation matrices must be inverted before applying it to an object."
        );

        self.base.transform = *transform;
    }
    fn get_transform(&self) -> Matrix<4> {
        self.base.transform
    }
    fn set_material(&mut self, material: &Material) {
        self.base.material = *material;
    }
    fn get_material(&self) -> Material {
        self.base.material
    }
    fn local_normal_at(&self, point: Point) -> Vector {
        let maxc = [point.x.abs(), point.y.abs(), point.z.abs()]
            .iter()
            .max_by(|a, b| a.total_cmp(b))
            .unwrap()
            .to_owned();

        if is_float_equal(&maxc, point.x.abs()) {
            new_vector(point.x, 0.0, 0.0)
        } else if is_float_equal(&maxc, point.y.abs()) {
            new_vector(0.0, point.y, 0.0)
        } else if is_float_equal(&maxc, point.z.abs()) {
            new_vector(0.0, 0.0, point.z)
        } else {
            panic!("Intersection did not match any axis")
        }
    }
    fn local_intersect(&self, local_ray: Ray, intersection_list: &mut Vec<Intersection>) {
        let (xtmin, xtmax): (f64, f64) = check_axis(local_ray.origin.x, local_ray.direction.x);
        let (ytmin, ytmax): (f64, f64) = check_axis(local_ray.origin.y, local_ray.direction.y);
        let (ztmin, ztmax): (f64, f64) = check_axis(local_ray.origin.z, local_ray.direction.z);

        let tmin = [xtmin, ytmin, ztmin]
            .iter()
            .max_by(|a, b| a.total_cmp(b))
            .unwrap()
            .to_owned();
        let tmax = [xtmax, ytmax, ztmax]
            .iter()
            .min_by(|a, b| a.total_cmp(b))
            .unwrap()
            .to_owned();

        if tmin > tmax {
        } else {
            intersection_list.push(Intersection::new(tmin, Object::Cube(self.clone())));
            intersection_list.push(Intersection::new(tmax, Object::Cube(self.clone())));
        }
    }
}

/// Check which plane of a given axis is hit first and returns the time difference between the
/// intersections.
fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
    let tmin_numerator = -1.0 - origin;
    let tmax_numerator = 1.0 - origin;

    let mut tmin = tmin_numerator / direction;
    let mut tmax = tmax_numerator / direction;

    if tmin > tmax {
        (tmin, tmax) = (tmax, tmin);
    }

    (tmin, tmax)
}

#[cfg(test)]
mod tests {
    use crate::ray_tracer::{tuples_new::new_point, utils::is_float_equal_low_precision};

    use super::*;

    #[test]
    fn a_ray_intersects_a_cube() {
        let c = Cube::new();

        let examples = [
            // +x
            (
                Ray::new(new_point(5.0, 0.5, 0.0), new_vector(-1.0, 0.0, 0.0)),
                4.0,
                6.0,
            ),
            // -x
            (
                Ray::new(new_point(-5.0, 0.5, 0.0), new_vector(1.0, 0.0, 0.0)),
                4.0,
                6.0,
            ),
            // +y
            (
                Ray::new(new_point(0.5, 5.0, 0.0), new_vector(0.0, -1.0, 0.0)),
                4.0,
                6.0,
            ),
            // -y
            (
                Ray::new(new_point(0.5, -5.0, 0.0), new_vector(0.0, 1.0, 0.0)),
                4.0,
                6.0,
            ),
            // +z
            (
                Ray::new(new_point(0.5, 0.0, 5.0), new_vector(0.0, 0.0, -1.0)),
                4.0,
                6.0,
            ),
            // -z
            (
                Ray::new(new_point(0.5, 0.0, -5.0), new_vector(0.0, 0.0, 1.0)),
                4.0,
                6.0,
            ),
            // inside
            (
                Ray::new(new_point(0.0, 0.5, 0.0), new_vector(0.0, 0.0, 1.0)),
                -1.0,
                1.0,
            ),
        ];

        for intersection in examples {
            let mut xs = Vec::new();
            c.local_intersect(intersection.0, &mut xs);
            assert_eq!(xs.len(), 2);
            assert!(is_float_equal_low_precision(
                &xs[0].get_time(),
                intersection.1
            ));
            assert!(is_float_equal_low_precision(
                &xs[1].get_time(),
                intersection.2
            ));
        }
    }

    #[test]
    fn a_ray_misses_a_cube() {
        let c = Cube::new();

        let examples = [
            Ray::new(
                new_point(-2.0, 0.0, 0.0),
                new_vector(0.2673, 0.5345, 0.8018),
            ),
            Ray::new(
                new_point(0.0, -2.0, 0.0),
                new_vector(0.8018, 0.2673, 0.5345),
            ),
            Ray::new(
                new_point(0.0, 0.0, -2.0),
                new_vector(0.5345, 0.8018, 0.2673),
            ),
            Ray::new(new_point(2.0, 0.0, 2.0), new_vector(0.0, 0.0, -1.0)),
            Ray::new(new_point(0.0, 2.0, 2.0), new_vector(0.0, -1.0, 0.0)),
            Ray::new(new_point(2.0, 2.0, 0.0), new_vector(-1.0, 0.0, 0.0)),
        ];

        for ray in examples {
            let mut xs = Vec::new();
            c.local_intersect(ray, &mut xs);
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        let c = Cube::new();

        let examples = [
            (new_point(1.0, 0.5, -0.8), new_vector(1.0, 0.0, 0.0)),
            (new_point(-1.0, -0.2, 0.9), new_vector(-1.0, 0.0, 0.0)),
            (new_point(-0.4, 1.0, -0.1), new_vector(0.0, 1.0, 0.0)),
            (new_point(0.3, -1.0, -0.7), new_vector(0.0, -1.0, 0.0)),
            (new_point(-0.6, 0.3, 1.0), new_vector(0.0, 0.0, 1.0)),
            (new_point(0.4, 0.4, -1.0), new_vector(0.0, 0.0, -1.0)),
            (new_point(1.0, 1.0, 1.0), new_vector(1.0, 0.0, 0.0)),
            (new_point(-1.0, -1.0, -1.0), new_vector(-1.0, 0.0, 0.0)),
        ];

        for ex in examples {
            let normal = c.local_normal_at(ex.0);
            assert_eq!(normal, ex.1);
        }
    }
}
