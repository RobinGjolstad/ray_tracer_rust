#![allow(unused)]
use crate::ray_tracer::{
    intersections::{Intersection, IntersectionsSoA},
    materials::Material,
    matrices_new::Matrix,
    rays::Ray,
    tuples_new::{new_vector, Point, Vector},
    utils::EPSILON,
};

use super::{BaseShape, Object, Shapes};

#[derive(Debug, Clone, PartialEq)]
pub struct Plane {
    pub(super) base: BaseShape,
    parent: Option<BaseShape>,
}

impl Plane {
    #[must_use]
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
    fn get_transform(&self) -> Matrix<4> {
        self.base.transform
    }
    fn get_material(&self) -> Material {
        self.base.material
    }
    #[allow(unused_variables)]
    fn local_normal_at(&self, point: Point) -> Vector {
        new_vector(0.0, 1.0, 0.0)
    }
    fn local_intersect(
        &self,
        object: &Object,
        local_ray: Ray,
        intersection_list: &mut crate::ray_tracer::intersections::IntersectionsSoA,
    ) {
        if f64::abs(local_ray.direction.y) < EPSILON {
            return;
        }
        let t = -local_ray.origin.y / local_ray.direction.y;
        intersection_list.push(t, object.clone());
    }
}

#[cfg(test)]
mod tests {
    use crate::ray_tracer::{
        shapes::ShapeBuilder, tuples_new::new_point, utils::is_float_equal_low_precision,
    };

    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Plane::new();
        let n1 = p.local_normal_at(new_point(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(new_point(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(new_point(-5.0, 0.0, 150.0));
        let expected_normal = new_vector(0.0, 1.0, 0.0);
        assert_eq!(expected_normal, n1);
        assert_eq!(expected_normal, n2);
        assert_eq!(expected_normal, n3);
    }
    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Plane::new();
        let obj_plane = ShapeBuilder::from_plane(p.clone()).build();
        let r = Ray::new(new_point(0.0, 10.0, 1.0), new_vector(0.0, 0.0, 1.0));
        let mut xs = IntersectionsSoA::default();
        p.local_intersect(&obj_plane, r, &mut xs);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Plane::new();
        let obj_plane = ShapeBuilder::from_plane(p.clone()).build();
        let r = Ray::new(new_point(0.0, 0.0, 0.0), new_vector(0.0, 0.0, 1.0));
        let mut xs = IntersectionsSoA::default();
        p.local_intersect(&obj_plane, r, &mut xs);
        assert_eq!(xs.len(), 0);
    }
    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Plane::new();
        let p_o = ShapeBuilder::from_plane(p.clone()).build();
        let r = Ray::new(new_point(0.0, 1.0, 0.0), new_vector(0.0, -1.0, 0.0));
        let mut xs = IntersectionsSoA::default();
        p.local_intersect(&p_o, r, &mut xs);
        assert_eq!(xs.len(), 1);
        assert!(is_float_equal_low_precision(&xs.ts[0], 1.0));
        assert_eq!(&xs.objects[0], &p_o);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Plane::new();
        let p_o = ShapeBuilder::from_plane(p.clone()).build();
        let r = Ray::new(new_point(0.0, -1.0, 0.0), new_vector(0.0, 1.0, 0.0));
        let mut xs = IntersectionsSoA::default();
        p.local_intersect(&p_o, r, &mut xs);
        assert_eq!(xs.len(), 1);
        assert!(is_float_equal_low_precision(&xs.ts[0], 1.0));
        assert_eq!(&xs.objects[0], &p_o);
    }
}
