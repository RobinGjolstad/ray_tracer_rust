use crate::ray_tracer::{
    intersections::Intersection,
    rays::Ray,
    shapes::{Object, ShapeType, Shapes},
    tuples::{Point, Vector},
    utils::is_float_equal,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cube {
    position: Point,
}
impl Cube {
    pub fn new() -> Self {
        Self {
            position: Point::new_point(0.0, 0.0, 0.0),
        }
    }

    fn check_axis(&self, origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;

        let mut tmin = tmin_numerator / direction;
        let mut tmax = tmax_numerator / direction;

        if tmin > tmax {
            (tmin, tmax) = (tmax, tmin);
        }

        (tmin, tmax)
    }
}

impl Shapes for Cube {
    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
        let (xtmin, xtmax): (f64, f64) = self.check_axis(local_ray.origin.x, local_ray.direction.x);
        let (ytmin, ytmax): (f64, f64) = self.check_axis(local_ray.origin.y, local_ray.direction.y);
        let (ztmin, ztmax): (f64, f64) = self.check_axis(local_ray.origin.z, local_ray.direction.z);

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
                Intersection::new(tmin, Object::new_raw(Box::new(*self))),
                Intersection::new(tmax, Object::new_raw(Box::new(*self))),
            ]
        }
    }

    fn set_position(&mut self, pos: &Point) {
        self.position = *pos;
    }

    fn get_position(&self) -> Point {
        self.position
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

    fn get_shape_type(&self) -> ShapeType {
        super::ShapeType::Cube
    }
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
