#![allow(unused)]
use super::{BaseShape, Object, Shapes};
use crate::ray_tracer::{
    intersections::Intersection,
    materials::Material,
    matrices::Matrix,
    rays::Ray,
    shapes::*,
    tuples::{Point, Vector},
};

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
    base: BaseShape,
}

impl Group {
    pub fn new() -> Self {
        Group {
            base: BaseShape::default(),
        }
    }
}

impl Default for Group {
    fn default() -> Self {
        Self::new()
    }
}

impl Shapes for Group {
    fn set_position(&mut self, pos: &Point) {
        self.base.position = *pos;
    }
    fn get_position(&self) -> Point {
        self.base.position
    }
    fn set_transform(&mut self, transform: &Matrix) {
        self.base.transform = *transform;
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
        Vector::new_vector(point.x, point.y, point.z)
    }
    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
        todo!()
    }
}

pub(crate) fn group_local_intersect(obj: &Object, local_ray: Ray) -> Vec<Intersection> {
    let children = obj.get_children();
    if children.is_none() {
        return vec![];
    }

    let mut retval = vec![];
    let children = children.unwrap();
    children.iter().for_each(|c| {
        let child = c.read().unwrap();
        //retval.append(&mut child.value.intersect(local_ray));
        retval.append(&mut local_ray.intersect(obj));
    });

    retval.sort_by(|a, b| {
        a.get_time()
            .partial_cmp(&b.get_time())
            .expect("Sorting intersections for group intersections failed.")
    });
    retval.dedup();

    retval
}

#[cfg(test)]
mod tests {
    use crate::ray_tracer::transformations::Transform;

    use super::*;

    #[test]
    fn creating_a_group() {
        let g = new_group();

        assert_eq!(g.get_transform(), Matrix::new_identity());
        assert!(!g.has_children());
    }
    #[test]
    fn adding_a_child_to_a_group() {
        let mut g = new_group();
        let mut s = new_test_shape();
        g.add_child(&s);

        let children = g.get_children().unwrap();

        let shape_ref = s.get_ref();

        let mut child_contained = false;
        children.iter().for_each(|c| {
            if Arc::ptr_eq(c, &shape_ref) {
                child_contained = true;
            }
        });

        assert_eq!(children.len(), 1);
        assert!(child_contained);
        assert!(Arc::ptr_eq(&s.get_parent().unwrap(), &g.get_ref()));
    }
    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let g = new_group();
        let r = Ray::new(
            Point::new_point(0.0, 0.0, 0.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );

        let xs = g.local_intersect(r);

        assert!(xs.is_empty());
    }
    #[test]
    fn intersecting_a_ray_with_a_nonempty_group() {
        let mut g = new_group();
        let mut s1 = new_sphere();
        let mut s2 = new_sphere();
        let mut s3 = new_sphere();

        s2.set_transform(&Transform::translate(0.0, 0.0, -3.0));
        s3.set_transform(&Transform::translate(5.0, 0.0, 0.0));

        g.add_child(&s1);
        g.add_child(&s2);
        g.add_child(&s3);

        let r = Ray::new(
            Point::new_point(0.0, 0.0, -5.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );

        let xs = g.local_intersect(r);

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].get_object(), &s2);
        assert_eq!(xs[1].get_object(), &s2);
        assert_eq!(xs[2].get_object(), &s1);
        assert_eq!(xs[3].get_object(), &s1);
    }
    #[test]
    fn intersecting_a_transformed_group() {
        todo!()
    }
}
