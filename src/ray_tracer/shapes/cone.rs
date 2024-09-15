#![allow(unused)]
use super::{BaseShape, Debug, Object, Shapes};
use crate::ray_tracer::{
    intersections::Intersection,
    materials::Material,
    matrices::Matrix,
    rays::Ray,
    tuples::{Point, Tuple, Vector},
    utils::{is_float_equal, EPSILON},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Cone {
    base: BaseShape,
    pub(super) minimum: f64,
    pub(super) maximum: f64,
    pub(super) closed: bool,
}

impl Cone {
    #[must_use]
    pub fn new() -> Self {
        Self {
            base: BaseShape::default(),
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
            closed: false,
        }
    }

    fn check_cap(y_plane: f64, ray: &Ray, t: f64) -> bool {
        // Original:
        // let x = ray.origin.x + t * ray.direction.x;
        // let z = ray.origin.z + t * ray.direction.z;
        // (x.powi(2) + z.powi(2)) <= y_plane.powi(2)

        let x = t.mul_add(ray.direction.x, ray.origin.x);
        let z = t.mul_add(ray.direction.z, ray.origin.z);
        x.mul_add(x, z.powi(2)) <= y_plane.powi(2)
    }

    fn intersect_caps(&self, ray: &Ray, xs: &mut Vec<Intersection>) {
        if !self.closed || is_float_equal(&ray.direction.y, 0.0) {
            return;
        }

        let t = (self.minimum - ray.origin.y) / ray.direction.y;
        if Self::check_cap(self.minimum, ray, t) {
            xs.push(Intersection::new(t, Object::Cone(self.clone())));
        }

        let t = (self.maximum - ray.origin.y) / ray.direction.y;
        if Self::check_cap(self.maximum, ray, t) {
            xs.push(Intersection::new(t, Object::Cone(self.clone())));
        }
    }
}

impl Default for Cone {
    fn default() -> Self {
        Self::new()
    }
}

impl Shapes for Cone {
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
        // Compute the square of the distance from the y-axis

        // let dist = point.x.powi(2) + point.z.powi(2);
        let dist = point.x.mul_add(point.x, point.z.powi(2));

        if dist < 1.0 && point.y >= (self.maximum - EPSILON) {
            // Check top cap
            Vector::new_vector(0.0, 1.0, 0.0)
        } else if dist < 1.0 && point.y <= (self.minimum + EPSILON) {
            // Check bottom cap
            Vector::new_vector(0.0, -1.0, 0.0)
        } else {
            let mut y = f64::sqrt(dist);
            if point.y > 0.0 {
                y = -y;
            }
            Vector::new_vector(point.x, y, point.z)
        }
    }
    fn local_intersect(&self, local_ray: Ray, intersection_list: &mut Vec<Intersection>) {
        // Original version. Trying FMA.
        //
        // let a = local_ray.direction.x.powi(2) - local_ray.direction.y.powi(2)
        //     + local_ray.direction.z.powi(2);
        let a = local_ray.direction.z.mul_add(
            local_ray.direction.z,
            local_ray
                .direction
                .x
                .mul_add(local_ray.direction.x, -local_ray.direction.y.powi(2)),
        );

        // Original version. Trying FMA.
        //
        // let b = (2.0 * local_ray.origin.x * local_ray.direction.x)
        //     - (2.0 * local_ray.origin.y * local_ray.direction.y)
        //     + (2.0 * local_ray.origin.z * local_ray.direction.z);
        let b = (2.0 * local_ray.origin.z).mul_add(
            local_ray.direction.z,
            (2.0 * local_ray.origin.x).mul_add(
                local_ray.direction.x,
                -(2.0 * local_ray.origin.y * local_ray.direction.y),
            ),
        );

        // Original version. Trying FMA.
        //
        // let c =
        //     local_ray.origin.x.powi(2) - local_ray.origin.y.powi(2) + local_ray.origin.z.powi(2);
        let c = local_ray.origin.z.mul_add(
            local_ray.origin.z,
            local_ray
                .origin
                .x
                .mul_add(local_ray.origin.x, -local_ray.origin.y.powi(2)),
        );

        if is_float_equal(&a, 0.0) {
            if is_float_equal(&b, 0.0) {
                // No intersections.
                return;
            }

            // Parallel to one of the halves.
            // One intersection.
            let t = -c / (2.0 * b);
            intersection_list.push(Intersection::new(t, Object::Cone(self.clone())));
        } else {
            // TODO: Figure out which version is faster.
            #[allow(clippy::suboptimal_flops)]
            let disc = b.powi(2) - (4.0 * a * c);
            // let disc = (b * b) - (4.0 * a * c);
            // let disc = (4.0 * a).mul_add(-c, b.powi(2));
            // let disc = b.mul_add(b, -(4.0 * a * c));

            if disc < 0.0 {
                // Ray doesn't intersect the cone.
                return;
            }

            let t0 = (-b - disc.sqrt()) / (2.0 * a);
            let t1 = (-b + disc.sqrt()) / (2.0 * a);

            // let y0 = local_ray.origin.y + t0 * local_ray.direction.y;
            let y0 = t0.mul_add(local_ray.direction.y, local_ray.origin.y);
            if self.minimum < y0 && y0 < self.maximum {
                intersection_list.push(Intersection::new(t0, Object::Cone(self.clone())));
            }

            // let y1 = local_ray.origin.y + t1 * local_ray.direction.y;
            let y1 = t1.mul_add(local_ray.direction.y, local_ray.origin.y);
            if self.minimum < y1 && y1 < self.maximum {
                intersection_list.push(Intersection::new(t1, Object::Cone(self.clone())));
            }
        }

        self.intersect_caps(&local_ray, intersection_list);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersecting_a_cone_with_a_ray() {
        let examples = [
            (
                Point::new_point(0.0, 0.0, -5.0),
                Vector::new_vector(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                Point::new_point(0.0, 0.0, -5.0),
                Vector::new_vector(1.0, 1.0, 1.0),
                8.66025,
                8.66025,
            ),
            (
                Point::new_point(1.0, 1.0, -5.0),
                Vector::new_vector(-0.5, -1.0, 1.0),
                4.55006,
                49.44994,
            ),
        ];
        let shape = Cone::new();

        for example in examples {
            let direction = example.1.normalize();
            let r = Ray {
                origin: example.0,
                direction,
            };
            let mut xs = Vec::new();
            shape.local_intersect(r, &mut xs);
            assert_eq!(xs.len(), 2);
            assert!(is_float_equal(&xs[0].get_time(), example.2));
            assert!(is_float_equal(&xs[1].get_time(), example.3));
        }
    }

    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let shape = Cone::new();
        let direction = Vector::new_vector(0.0, 1.0, 1.0).normalize();
        let r = Ray::new(Point::new_point(0.0, 0.0, -1.0), direction);

        let mut xs = Vec::new();
        shape.local_intersect(r, &mut xs);
        assert_eq!(xs.len(), 1);
        assert!(is_float_equal(&xs[0].get_time(), 0.35355));
    }

    #[test]
    fn a_ray_misses_a_cone() {
        let examples = [(
            // At center height of cone, offset to one side,
            // pointing tangentially away from the cone.
            Point::new_point(0.0, 0.0, 1.0),
            Vector::new_vector(1.0, 0.0, 0.0),
        )];
        let cone = Cone::new();

        for example in examples {
            let direction = example.1;
            let ray = Ray::new(example.0, direction.normalize());
            let mut xs = Vec::new();
            cone.local_intersect(ray, &mut xs);
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn normal_vector_on_a_cone() {
        let examples = [
            (
                Point::new_point(0.0, 0.0, 0.0),
                Vector::new_vector(0.0, 0.0, 0.0),
            ),
            (
                Point::new_point(1.0, 1.0, 1.0),
                Vector::new_vector(1.0, -f64::sqrt(2.0), 1.0),
            ),
            (
                Point::new_point(-1.0, -1.0, 0.0),
                Vector::new_vector(-1.0, 1.0, 0.0),
            ),
        ];
        let cone = Cone::new();

        for example in examples {
            let n = cone.local_normal_at(example.0);
            assert_eq!(example.1, n);
        }
    }

    #[test]
    fn the_default_minimum_and_maximum_for_a_cone() {
        let cone = Cone::new();

        assert!(cone.minimum.is_infinite());
        assert!(cone.maximum.is_infinite());
    }

    #[test]
    fn intersecting_a_constrained_cone() {
        let examples = [
            (
                Point::new_point(0.0, 1.5, 0.0),
                Vector::new_vector(0.1, 1.0, 0.0),
                0,
            ),
            (
                Point::new_point(0.0, 3.0, -5.0),
                Vector::new_vector(0.0, 0.0, 1.0),
                0,
            ),
            (
                Point::new_point(0.0, 0.0, -5.0),
                Vector::new_vector(0.0, 0.0, 1.0),
                0,
            ),
            (
                Point::new_point(0.0, 2.0, -5.0),
                Vector::new_vector(0.0, 0.0, 1.0),
                0,
            ),
            (
                Point::new_point(0.0, 1.0, -5.0),
                Vector::new_vector(0.0, 0.0, 1.0),
                0,
            ),
            (
                Point::new_point(0.0, 1.5, -2.0),
                Vector::new_vector(0.0, 0.0, 1.0),
                2,
            ),
        ];

        let mut cone = Cone::new();
        cone.minimum = 1.0;
        cone.maximum = 2.0;

        for example in examples {
            let direction = example.1.normalize();
            let r = Ray::new(example.0, direction);
            let mut xs = Vec::new();
            cone.local_intersect(r, &mut xs);
            assert_eq!(example.2, xs.len());
        }
    }

    #[test]
    fn the_default_closed_value_for_a_cone() {
        let cone = Cone::new();

        assert!(!cone.closed);
    }

    #[test]
    fn intersecting_a_cones_end_caps() {
        let examples = [
            (
                Point::new_point(0.0, 0.0, -5.0),
                Vector::new_vector(0.0, 1.0, 0.0),
                0,
            ),
            (
                Point::new_point(0.0, 0.0, -0.25),
                Vector::new_vector(0.0, 1.0, 1.0),
                2,
            ),
            (
                Point::new_point(0.0, 0.0, -0.25),
                Vector::new_vector(0.0, 1.0, 0.0),
                4,
            ),
        ];

        let mut cone = Cone::new();
        cone.minimum = -0.5;
        cone.maximum = 0.5;
        cone.closed = true;

        for example in examples {
            let direction = example.1.normalize();
            let r = Ray::new(example.0, direction);
            let mut xs = Vec::new();
            cone.local_intersect(r, &mut xs);
            assert_eq!(example.2, xs.len());
        }
    }

    #[test]
    fn the_normal_vector_on_a_cones_end_caps() {
        let examples = [
            (
                Point::new_point(0.0, 1.0, 0.0),
                Vector::new_vector(0.0, -1.0, 0.0),
            ),
            (
                Point::new_point(0.5, 1.0, 0.0),
                Vector::new_vector(0.0, -1.0, 0.0),
            ),
            (
                Point::new_point(0.0, 1.0, 0.5),
                Vector::new_vector(0.0, -1.0, 0.0),
            ),
            (
                Point::new_point(0.0, 2.0, 0.0),
                Vector::new_vector(0.0, 1.0, 0.0),
            ),
            (
                Point::new_point(0.5, 2.0, 0.0),
                Vector::new_vector(0.0, 1.0, 0.0),
            ),
            (
                Point::new_point(0.0, 2.0, 0.5),
                Vector::new_vector(0.0, 1.0, 0.0),
            ),
        ];

        let mut cone = Cone::new();
        cone.minimum = 1.0;
        cone.maximum = 2.0;
        cone.closed = true;

        for example in examples {
            let n = cone.local_normal_at(example.0);
            assert_eq!(example.1, n);
        }
    }

    #[test]
    fn a_ray_misses_a_restricted_cone() {
        let mut cone = Cone::new();
        cone.minimum = -1.0;
        cone.maximum = 0.0;
        cone.closed = true;
        let examples = [
            (
                Point::new_point(0.0, 0.0, -5.0),
                Vector::new_vector(0.0, 0.0, 1.0),
            ),
            (
                Point::new_point(-2.0, 1.0, 0.0),
                Vector::new_vector(1.0, 0.0, 0.0),
            ),
        ];

        for example in examples {
            let direction = example.1.normalize();
            let r = Ray::new(example.0, direction);
            let mut xs = Vec::new();
            cone.local_intersect(r, &mut xs);
            assert_eq!(xs.len(), 0);
        }
    }
}
