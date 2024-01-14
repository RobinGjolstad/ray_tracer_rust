#![allow(unused)]
use super::*;
use crate::ray_tracer::{
    intersections::Intersection,
    materials::Material,
    matrices::Matrix,
    rays::Ray,
    tuples::{Point, Tuple, Vector},
    utils::is_float_equal,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Cube {
    base: BaseShape,
    parent: Option<BaseShape>,
}

impl Cube {
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
    fn set_transform(&mut self, transform: &Matrix) {
        let mut trans = *transform;
        trans.calculate_inverse().unwrap();
        self.base.transform = trans;
    }
    fn get_transform(&self) -> Matrix {
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
            Vector::new_vector(point.x, 0.0, 0.0)
        } else if is_float_equal(&maxc, point.y.abs()) {
            Vector::new_vector(0.0, point.y, 0.0)
        } else if is_float_equal(&maxc, point.z.abs()) {
            Vector::new_vector(0.0, 0.0, point.z)
        } else {
            panic!("Intersection did not match any axis")
        }
    }
    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
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
            Vec::new()
        } else {
            vec![
                Intersection::new(tmin, Object::new(ObjectEnum::Cube(self.clone()))),
                Intersection::new(tmax, Object::new(ObjectEnum::Cube(self.clone()))),
            ]
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
    use super::*;

    #[test]
    fn a_ray_intersects_a_cube() {
        let c = Cube::new();

        let examples = [
            // +x
            (
                Ray::new(
                    Point::new_point(5.0, 0.5, 0.0),
                    Vector::new_vector(-1.0, 0.0, 0.0),
                ),
                4.0,
                6.0,
            ),
            // -x
            (
                Ray::new(
                    Point::new_point(-5.0, 0.5, 0.0),
                    Vector::new_vector(1.0, 0.0, 0.0),
                ),
                4.0,
                6.0,
            ),
            // +y
            (
                Ray::new(
                    Point::new_point(0.5, 5.0, 0.0),
                    Vector::new_vector(0.0, -1.0, 0.0),
                ),
                4.0,
                6.0,
            ),
            // -y
            (
                Ray::new(
                    Point::new_point(0.5, -5.0, 0.0),
                    Vector::new_vector(0.0, 1.0, 0.0),
                ),
                4.0,
                6.0,
            ),
            // +z
            (
                Ray::new(
                    Point::new_point(0.5, 0.0, 5.0),
                    Vector::new_vector(0.0, 0.0, -1.0),
                ),
                4.0,
                6.0,
            ),
            // -z
            (
                Ray::new(
                    Point::new_point(0.5, 0.0, -5.0),
                    Vector::new_vector(0.0, 0.0, 1.0),
                ),
                4.0,
                6.0,
            ),
            // inside
            (
                Ray::new(
                    Point::new_point(0.0, 0.5, 0.0),
                    Vector::new_vector(0.0, 0.0, 1.0),
                ),
                -1.0,
                1.0,
            ),
        ];

        for intersection in examples {
            let xs = c.local_intersect(intersection.0);
            assert_eq!(xs.len(), 2);
            assert_eq!(xs[0].get_time(), intersection.1);
            assert_eq!(xs[1].get_time(), intersection.2);
        }
    }

    #[test]
    fn a_ray_misses_a_cube() {
        let c = Cube::new();

        let examples = [
            Ray::new(
                Point::new_point(-2.0, 0.0, 0.0),
                Vector::new_vector(0.2673, 0.5345, 0.8018),
            ),
            Ray::new(
                Point::new_point(0.0, -2.0, 0.0),
                Vector::new_vector(0.8018, 0.2673, 0.5345),
            ),
            Ray::new(
                Point::new_point(0.0, 0.0, -2.0),
                Vector::new_vector(0.5345, 0.8018, 0.2673),
            ),
            Ray::new(
                Point::new_point(2.0, 0.0, 2.0),
                Vector::new_vector(0.0, 0.0, -1.0),
            ),
            Ray::new(
                Point::new_point(0.0, 2.0, 2.0),
                Vector::new_vector(0.0, -1.0, 0.0),
            ),
            Ray::new(
                Point::new_point(2.0, 2.0, 0.0),
                Vector::new_vector(-1.0, 0.0, 0.0),
            ),
        ];

        for ray in examples {
            let xs = c.local_intersect(ray);
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        let c = Cube::new();

        let examples = [
            (
                Point::new_point(1.0, 0.5, -0.8),
                Vector::new_vector(1.0, 0.0, 0.0),
            ),
            (
                Point::new_point(-1.0, -0.2, 0.9),
                Vector::new_vector(-1.0, 0.0, 0.0),
            ),
            (
                Point::new_point(-0.4, 1.0, -0.1),
                Vector::new_vector(0.0, 1.0, 0.0),
            ),
            (
                Point::new_point(0.3, -1.0, -0.7),
                Vector::new_vector(0.0, -1.0, 0.0),
            ),
            (
                Point::new_point(-0.6, 0.3, 1.0),
                Vector::new_vector(0.0, 0.0, 1.0),
            ),
            (
                Point::new_point(0.4, 0.4, -1.0),
                Vector::new_vector(0.0, 0.0, -1.0),
            ),
            (
                Point::new_point(1.0, 1.0, 1.0),
                Vector::new_vector(1.0, 0.0, 0.0),
            ),
            (
                Point::new_point(-1.0, -1.0, -1.0),
                Vector::new_vector(-1.0, 0.0, 0.0),
            ),
        ];

        for ex in examples {
            let normal = c.local_normal_at(ex.0);
            assert_eq!(normal, ex.1);
        }
    }
}
