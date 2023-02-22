use crate::{
    intersections::Intersection,
    materials::Material,
    matrices::Matrix,
    transformations::Transform,
    tuples::{Point, Vector},
    utils::{is_float_equal, EPSILON},
};

use super::{Object, Shapes};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Plane {
    position: Point,
    transform: Matrix,
    material: Material,
}
impl Plane {
    pub fn new() -> Plane {
        Plane {
            position: Point::new_point(0.0, 0.0, 0.0),
            transform: Matrix::new_identity(),
            material: Material::new(),
        }
    }
}

impl Shapes for Plane {
    fn set_position(&mut self, pos: &Point) {
        self.position = *pos;
    }
    fn get_position(&self) -> Point {
        self.position
    }
    fn set_material(&mut self, material: &Material) {
        self.material = *material;
    }
    fn get_material(&self) -> Material {
        self.material
    }
    fn set_transform(&mut self, trans: &Matrix) {
        self.transform = *trans;
    }
    fn get_transform(&self) -> Matrix {
        self.transform
    }
    fn get_shape_type(&self) -> super::ShapeType {
        super::ShapeType::Plane
    }
    #[allow(unused_variables)]
    fn local_normal_at(&self, point: Point) -> crate::tuples::Vector {
        Vector::new_vector(0.0, 1.0, 0.0)
    }
    fn local_intersect(
        &self,
        local_ray: crate::rays::Ray,
    ) -> Vec<crate::intersections::Intersection> {
        if f64::abs(local_ray.direction.y) < EPSILON {
            return Vec::new();
        }

        let t = -local_ray.origin.y / local_ray.direction.y;
        vec![Intersection::new(t, Object::new(Box::new(*self)))]
    }
}

#[cfg(test)]
mod tests {
    use crate::{rays::Ray, shapes::Object, tuples::Vector};

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
        let p_o = Object::new(Box::new(p));
        let r = Ray::new(
            Point::new_point(0.0, 1.0, 0.0),
            Vector::new_vector(0.0, -1.0, 0.0),
        );
        let xs = p.local_intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs.get(0).unwrap().get_time(), 1.0);
        assert_eq!(xs.get(0).unwrap().get_object(), p_o);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane::new();
        let p_o = Object::new(Box::new(p));
        let r = Ray::new(
            Point::new_point(0.0, -1.0, 0.0),
            Vector::new_vector(0.0, 1.0, 0.0),
        );
        let xs = p.local_intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs.get(0).unwrap().get_time(), 1.0);
        assert_eq!(xs.get(0).unwrap().get_object(), p_o);
    }
}
