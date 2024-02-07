#![allow(unused)]
use super::*;
use crate::ray_tracer::{
    intersections::Intersection,
    materials::Material,
    matrices::Matrix,
    rays::Ray,
    tuples::{Point, Tuple, Vector},
    utils::EPSILON,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Plane {
    base: BaseShape,
    parent: Option<BaseShape>,
}

impl Plane {
    pub fn new() -> Self {
        Self {
            base: BaseShape::default(),
            parent: None,
        }
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self::new()
    }
}

impl Shapes for Plane {
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
    #[allow(unused_variables)]
    fn local_normal_at(&self, point: Point) -> Vector {
        Vector::new_vector(0.0, 1.0, 0.0)
    }
    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
        if f64::abs(local_ray.direction.y) < EPSILON {
            return Vec::new();
        }

        let t = -local_ray.origin.y / local_ray.direction.y;
        vec![Intersection::new(t, Object::Plane(self.clone()))]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::new();
        let n1 = p.local_normal_at(Point::new_point(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(Point::new_point(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(Point::new_point(-5.0, 0.0, 150.0));
        let expected_normal = Vector::new_vector(0.0, 1.0, 0.0);
        assert_eq!(expected_normal, n1);
        assert_eq!(expected_normal, n2);
        assert_eq!(expected_normal, n3);
    }
    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Plane::new();
        let r = Ray::new(
            Point::new_point(0.0, 10.0, 1.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );
        let xs = p.local_intersect(r);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Plane::new();
        let r = Ray::new(
            Point::new_point(0.0, 0.0, 0.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );
        let xs = p.local_intersect(r);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Plane::new();
        let p_o = Object::Plane(p.clone());
        let r = Ray::new(
            Point::new_point(0.0, 1.0, 0.0),
            Vector::new_vector(0.0, -1.0, 0.0),
        );
        let xs = p.local_intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs.first().unwrap().get_time(), 1.0);
        assert_eq!(*xs.first().unwrap().get_object(), p_o);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane::new();
        let p_o = Object::Plane(p.clone());
        let r = Ray::new(
            Point::new_point(0.0, -1.0, 0.0),
            Vector::new_vector(0.0, 1.0, 0.0),
        );
        let xs = p.local_intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs.first().unwrap().get_time(), 1.0);
        assert_eq!(*xs.first().unwrap().get_object(), p_o);
    }
}
