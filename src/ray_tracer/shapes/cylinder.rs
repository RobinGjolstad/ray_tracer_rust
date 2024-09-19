#![allow(unused)]
use super::{BaseShape, Debug, Object, Shapes};
use crate::ray_tracer::{
    intersections::Intersection,
    materials::Material,
    matrices::Matrix,
    rays::Ray,
    tuples::{new_vector, Point, Vector},
    utils::{is_float_equal, EPSILON},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Cylinder {
    base: BaseShape,
    parent: Option<BaseShape>,
    pub(super) minimum: f64,
    pub(super) maximum: f64,
    pub(super) closed: bool,
}

impl Cylinder {
    #[must_use]
    pub fn new() -> Self {
        Self {
            base: BaseShape::default(),
            parent: None,
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
            closed: false,
        }
    }

    fn check_cap(ray: &Ray, t: f64) -> bool {
        // let x = ray.origin.x + t * ray.direction.x;
        // let z = ray.origin.z + t * ray.direction.z;
        // (x.powi(2) + z.powi(2)) <= 1.0

        let x = t.mul_add(ray.direction.x, ray.origin.x);
        let z = t.mul_add(ray.direction.z, ray.origin.z);

        x.mul_add(x, z.powi(2)) <= 1.0
    }

    fn intersect_caps(&self, ray: &Ray, xs: &mut Vec<Intersection>) {
        if !self.closed || is_float_equal(&ray.direction.y, 0.0) {
            return;
        }

        let t = (self.minimum - ray.origin.y) / ray.direction.y;
        if Self::check_cap(ray, t) {
            xs.push(Intersection::new(t, Object::Cylinder(self.clone())));
        }

        let t = (self.maximum - ray.origin.y) / ray.direction.y;
        if Self::check_cap(ray, t) {
            xs.push(Intersection::new(t, Object::Cylinder(self.clone())));
        }
    }
}

impl Default for Cylinder {
    fn default() -> Self {
        Self::new()
    }
}

impl Shapes for Cylinder {
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
            new_vector(0.0, 1.0, 0.0)
        } else if dist < 1.0 && point.y <= (self.minimum + EPSILON) {
            new_vector(0.0, -1.0, 0.0)
        } else {
            new_vector(point.x, 0.0, point.z)
        }
    }
    fn local_intersect(&self, local_ray: Ray, intersection_list: &mut Vec<Intersection>) {
        // let a = local_ray.direction.x.powi(2) + local_ray.direction.z.powi(2);
        let a = local_ray
            .direction
            .x
            .mul_add(local_ray.direction.x, local_ray.direction.z.powi(2));
        if is_float_equal(&a, 0.0) {
            self.intersect_caps(&local_ray, intersection_list);

            return;
        }

        // let b = 2.0 * local_ray.origin.x * local_ray.direction.x
        //     + 2.0 * local_ray.origin.z * local_ray.direction.z;
        let b = (2.0 * local_ray.origin.x).mul_add(
            local_ray.direction.x,
            2.0 * local_ray.origin.z * local_ray.direction.z,
        );
        // let c = local_ray.origin.x.powi(2) + local_ray.origin.z.powi(2) - 1.0;
        let c = local_ray
            .origin
            .x
            .mul_add(local_ray.origin.x, local_ray.origin.z.powi(2))
            - 1.0;

        #[allow(clippy::suboptimal_flops)]
        // let disc = b.powi(2) - 4.0 * a * c;
        // let disc = (4.0 * a).mul_add(-c, b.powi(2));
        let disc = b.mul_add(b, -(4.0 * a * c));

        if disc < 0.0 {
            // Ray doesn't intersect the cylinder
            return;
        }

        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let t1 = (-b + disc.sqrt()) / (2.0 * a);

        // let y0 = local_ray.origin.y + t0 * local_ray.direction.y;
        let y0 = t0.mul_add(local_ray.direction.y, local_ray.origin.y);
        if self.minimum < y0 && y0 < self.maximum {
            intersection_list.push(Intersection::new(t0, Object::Cylinder(self.clone())));
        }

        // let y1 = local_ray.origin.y + t1 * local_ray.direction.y;
        let y1 = t1.mul_add(local_ray.direction.y, local_ray.origin.y);
        if self.minimum < y1 && y1 < self.maximum {
            intersection_list.push(Intersection::new(t1, Object::Cylinder(self.clone())));
        }

        self.intersect_caps(&local_ray, intersection_list);
    }
}

#[cfg(test)]
mod tests {
    use crate::ray_tracer::tuples::new_point;

    use super::*;

    #[test]
    fn a_ray_misses_a_cylinder() {
        let examples = [
            (new_point(1.0, 0.0, 0.0), new_vector(0.0, 1.0, 0.0)),
            (new_point(0.0, 0.0, 0.0), new_vector(0.0, 1.0, 0.0)),
            (new_point(0.0, 0.0, -5.0), new_vector(1.0, 1.0, 1.0)),
        ];
        let cyl = Cylinder::new();

        for example in examples {
            let direction = example.1;
            let ray = Ray::new(example.0, direction.normalize());
            let mut xs = Vec::new();
            cyl.local_intersect(ray, &mut xs);
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn a_ray_hits_a_cylinder() {
        let examples = [
            (
                new_point(1.0, 0.0, -5.0),
                new_vector(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                new_point(0.0, 0.0, -5.0),
                new_vector(0.0, 0.0, 1.0),
                4.0,
                6.0,
            ),
            (
                new_point(0.5, 0.0, -5.0),
                new_vector(0.1, 1.0, 1.0),
                6.80798,
                7.08872,
            ),
        ];
        let cyl = Cylinder::new();

        for example in examples {
            let direction = example.1.normalize();
            let ray = Ray::new(example.0, direction);
            let mut xs = Vec::new();
            cyl.local_intersect(ray, &mut xs);
            assert_eq!(2, xs.len());
            assert!(is_float_equal(&example.2, xs[0].get_time()));
            assert!(is_float_equal(&example.3, xs[1].get_time()));
        }
    }

    #[test]
    fn normal_vector_on_a_cylinder() {
        let examples = [
            (new_point(1.0, 0.0, 0.0), new_vector(1.0, 0.0, 0.0)),
            (new_point(0.0, 5.0, -1.0), new_vector(0.0, 0.0, -1.0)),
            (new_point(0.0, -2.0, 1.0), new_vector(0.0, 0.0, 1.0)),
            (new_point(-1.0, 1.0, 0.0), new_vector(-1.0, 0.0, 0.0)),
        ];
        let cyl = Cylinder::new();

        for example in examples {
            let n = cyl.local_normal_at(example.0);
            assert_eq!(example.1, n);
        }
    }

    #[test]
    fn the_default_minimum_and_maximum_for_a_cylinder() {
        let cyl = Cylinder::new();

        assert!(cyl.minimum.is_infinite());
        assert!(cyl.maximum.is_infinite());
    }

    #[test]
    fn intersecting_a_connstrained_cylinder() {
        let examples = [
            (new_point(0.0, 1.5, 0.0), new_vector(0.1, 1.0, 0.0), 0),
            (new_point(0.0, 3.0, -5.0), new_vector(0.0, 0.0, 1.0), 0),
            (new_point(0.0, 0.0, -5.0), new_vector(0.0, 0.0, 1.0), 0),
            (new_point(0.0, 2.0, -5.0), new_vector(0.0, 0.0, 1.0), 0),
            (new_point(0.0, 1.0, -5.0), new_vector(0.0, 0.0, 1.0), 0),
            (new_point(0.0, 1.5, -2.0), new_vector(0.0, 0.0, 1.0), 2),
        ];

        let mut cyl = Cylinder::new();
        cyl.minimum = 1.0;
        cyl.maximum = 2.0;

        for example in examples {
            let direction = example.1.normalize();
            let r = Ray::new(example.0, direction);
            let mut xs = Vec::new();
            cyl.local_intersect(r, &mut xs);
            assert_eq!(example.2, xs.len());
        }
    }

    #[test]
    fn the_default_closed_value_for_a_cylinder() {
        let cyl = Cylinder::new();

        assert!(!cyl.closed);
    }

    #[test]
    fn intersecting_the_caps_of_a_closed_cylinder() {
        let examples = [
            (new_point(0.0, 3.0, 0.0), new_vector(0.0, -1.0, 0.0), 2),
            (new_point(0.0, 3.0, -2.0), new_vector(0.0, -1.0, 2.0), 2),
            (new_point(0.0, 4.0, -2.0), new_vector(0.0, -1.0, 1.0), 2),
            (new_point(0.0, 0.0, -2.0), new_vector(0.0, 1.0, 2.0), 2),
            (new_point(0.0, -1.0, -2.0), new_vector(0.0, 1.0, 1.0), 2),
        ];

        let mut cyl = Cylinder::new();
        cyl.minimum = 1.0;
        cyl.maximum = 2.0;
        cyl.closed = true;

        for example in examples {
            let direction = example.1.normalize();
            let r = Ray::new(example.0, direction);
            let mut xs = Vec::new();
            cyl.local_intersect(r, &mut xs);
            assert_eq!(example.2, xs.len());
        }
    }

    #[test]
    fn the_normal_vector_on_a_cylinders_end_caps() {
        let examples = [
            (new_point(0.0, 1.0, 0.0), new_vector(0.0, -1.0, 0.0)),
            (new_point(0.5, 1.0, 0.0), new_vector(0.0, -1.0, 0.0)),
            (new_point(0.0, 1.0, 0.5), new_vector(0.0, -1.0, 0.0)),
            (new_point(0.0, 2.0, 0.0), new_vector(0.0, 1.0, 0.0)),
            (new_point(0.5, 2.0, 0.0), new_vector(0.0, 1.0, 0.0)),
            (new_point(0.0, 2.0, 0.5), new_vector(0.0, 1.0, 0.0)),
        ];

        let mut cyl = Cylinder::new();
        cyl.minimum = 1.0;
        cyl.maximum = 2.0;
        cyl.closed = true;

        for example in examples {
            let n = cyl.local_normal_at(example.0);
            assert_eq!(example.1, n);
        }
    }
}
