#![allow(unused)]
use crate::ray_tracer::{
    intersections::Intersection,
    materials::Material,
    matrices_new::Matrix,
    rays::Ray,
    tuples_new::{new_point, Point, Vector},
};

use super::{BaseShape, Object, Shapes};

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    base: BaseShape,
    parent: Option<BaseShape>,
}

impl Sphere {
    #[must_use]
    pub fn new() -> Self {
        Self {
            base: BaseShape::default(),
            parent: None,
        }
    }
}

impl Default for Sphere {
    fn default() -> Self {
        Self::new()
    }
}

impl Shapes for Sphere {
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
        point - new_point(0.0, 0.0, 0.0)
    }
    fn local_intersect(&self, local_ray: Ray, intersection_list: &mut Vec<Intersection>) {
        let sphere_to_ray = local_ray.origin - self.get_position();
        let a = Vector::dot(&local_ray.direction, &local_ray.direction);
        let b = 2.0 * Vector::dot(&local_ray.direction, &sphere_to_ray);
        let c = Vector::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        // let discriminant = b.powi(2) - 4.0 * a * c;
        let discriminant = b.mul_add(b, -(4.0 * a * c));
        let discriminant_sqrt = discriminant.sqrt();

        if discriminant < 0.0 {
        } else {
            intersection_list.push(Intersection::new(
                (-b - discriminant_sqrt) / (2.0 * a),
                Object::Sphere(self.clone()),
            ));
            intersection_list.push(Intersection::new(
                (-b + discriminant_sqrt) / (2.0 * a),
                Object::Sphere(self.clone()),
            ));
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ray_tracer::tuples_new::new_vector;

    use super::*;

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();
        let n = s.local_normal_at(new_point(1.0, 0.0, 0.0));
        assert_eq!(n, new_vector(1.0, 0.0, 0.0));
    }
    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new();
        let n = s.local_normal_at(new_point(0.0, 1.0, 0.0));
        assert_eq!(n, new_vector(0.0, 1.0, 0.0));
    }
    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new();
        let n = s.local_normal_at(new_point(0.0, 0.0, 1.0));
        assert_eq!(n, new_vector(0.0, 0.0, 1.0));
    }
    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::new();
        let n = s.local_normal_at(new_point(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));
        assert_eq!(
            n,
            new_vector(
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0
            )
        );
    }
    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere::new();
        let n = s.local_normal_at(new_point(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));
        assert_eq!(n, n.normalize());
    }
}
