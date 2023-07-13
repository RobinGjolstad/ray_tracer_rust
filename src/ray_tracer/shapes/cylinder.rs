use crate::ray_tracer::{
    intersections::Intersection,
    materials::Material,
    rays::Ray,
    tuples::{Point, Vector},
    utils::is_float_equal,
};

use super::{ShapeType, Shapes};

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct Cylinder {
    position: Point,
    material: Material,
}

impl Cylinder {
    pub(crate) fn new() -> Self {
        Self {
            position: Point::new_point(0.0, 0.0, 0.0),
            material: Material::new(),
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
        self.position = *pos;
    }
    fn get_position(&self) -> Point {
        self.position
    }
    fn get_shape_type(&self) -> ShapeType {
        ShapeType::Cylinder
    }
    fn local_normal_at(&self, point: Point) -> Vector {
        Vector::new_vector(point.x, 0.0, point.z)
    }
    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
        let a = local_ray.direction.x.powi(2) + local_ray.direction.z.powi(2);
        if is_float_equal(&a, 0.0) {
            // ray is parallell to the y axis
            return vec![];
        }

        let b = 2.0 * local_ray.origin.x * local_ray.direction.x
            + 2.0 * local_ray.origin.z * local_ray.direction.z;
        let c = local_ray.origin.x.powi(2) + local_ray.origin.z.powi(2) - 1.0;
        let disc = b.powi(2) - 4.0 * a * c;
        if disc < 0.0 {
            // Ray doesn't intersect the cylinder
            return vec![];
        }

        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let t1 = (-b + disc.sqrt()) / (2.0 * a);
        vec![
            Intersection::new(t0, super::Object::new_raw(Box::new(*self))),
            Intersection::new(t1, super::Object::new_raw(Box::new(*self))),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_ray_misses_a_cylinder() {
        let examples = [
            (
                Point::new_point(1.0, 0.0, 0.0),
                Vector::new_vector(0.0, 1.0, 0.0),
            ),
            (
                Point::new_point(0.0, 0.0, 0.0),
                Vector::new_vector(0.0, 1.0, 0.0),
            ),
            (
                Point::new_point(0.0, 0.0, -5.0),
                Vector::new_vector(1.0, 1.0, 1.0),
            ),
        ];
        let cyl = Cylinder::new();

        for example in examples {
            let direction = example.1;
            let ray = Ray::new(example.0, direction.normalize());
            let xs = cyl.local_intersect(ray);
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn a_ray_hits_a_cylinder() {
        let examples = [
            (
                Point::new_point(1.0, 0.0, -5.0),
                Vector::new_vector(0.0, 0.0, 1.0),
                5.0,
                5.0,
            ),
            (
                Point::new_point(0.0, 0.0, -5.0),
                Vector::new_vector(0.0, 0.0, 1.0),
                4.0,
                6.0,
            ),
            (
                Point::new_point(0.5, 0.0, -5.0),
                Vector::new_vector(0.1, 1.0, 1.0),
                6.80798,
                7.08872,
            ),
        ];
        let cyl = Cylinder::new();

        for example in examples {
            let direction = example.1.normalize();
            let ray = Ray::new(example.0, direction);
            let xs = cyl.local_intersect(ray);
            assert_eq!(2, xs.len());
            assert!(is_float_equal(&example.2, xs[0].get_time()));
            assert!(is_float_equal(&example.3, xs[1].get_time()));
        }
    }

    #[test]
    fn normal_vector_on_a_cylinder() {
        let examples = [
            (
                Point::new_point(1.0, 0.0, 0.0),
                Vector::new_vector(1.0, 0.0, 0.0),
            ),
            (
                Point::new_point(0.0, 5.0, -1.0),
                Vector::new_vector(0.0, 0.0, -1.0),
            ),
            (
                Point::new_point(0.0, -2.0, 1.0),
                Vector::new_vector(0.0, 0.0, 1.0),
            ),
            (
                Point::new_point(-1.0, 1.0, 0.0),
                Vector::new_vector(-1.0, 0.0, 0.0),
            ),
        ];
        let cyl = Cylinder::new();

        for example in examples {
            let n = cyl.local_normal_at(example.0);
            assert_eq!(example.1, n);
        }
    }
}
