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
    parent: Option<BaseShape>,
    children: Vec<Object>,
}

impl Group {
    pub fn new() -> Self {
        Group {
            base: BaseShape::default(),
            parent: None,
            children: vec![],
        }
    }
    fn add_child(&mut self, child: Object) {
        todo!()
    }
    fn update_children(&mut self) {
        todo!()
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
        let mut xs: Vec<Intersection> = Vec::new();
        for child in &self.children {
            let mut child_xs = local_ray.intersect(child);
            xs.append(&mut child_xs);
        }
        xs.sort_by(|a, b| a.get_time().partial_cmp(&b.get_time()).unwrap());
        xs.dedup();

        xs
    }
}

#[cfg(test)]
mod tests {
    use crate::ray_tracer::transformations::Transform;

    use super::*;

    #[test]
    fn creating_a_group() {
        let g = Group::default();

        assert_eq!(g.get_transform(), Matrix::new_identity());
        assert!(g.children.is_empty());
    }
    #[test]
    fn adding_a_child_to_a_group() {
        let mut g = Group::new();
        let mut s = new_test_shape();
        g.add_child(s.clone());

        // Test-hack to get around the fact that `s` doesn't get its parent set.
        // Only the child cloned into the group gets its parent set.
        //s.set_parent(&g.base);
        todo!("Fix grouping and parents");

        dbg!(&g);
        //assert!(!g.children.is_empty());
        //assert!(g.children.contains(&s));
        //assert!(s.get_parent().is_some());
        //assert_eq!(s.get_parent().unwrap(), g.base);
    }
    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let g = Group::default();
        let r = Ray::new(
            Point::new_point(0.0, 0.0, 0.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );

        let xs = g.local_intersect(r);

        assert!(xs.is_empty());
    }
    #[test]
    fn intersecting_a_ray_with_a_nonempty_group() {
        let mut g = Group::default();
        let mut s1 = new_sphere();
        let mut s2 = new_sphere();
        let mut s3 = new_sphere();

        s2.set_transform(&Transform::translate(0.0, 0.0, -3.0));
        s3.set_transform(&Transform::translate(5.0, 0.0, 0.0));

        let mut g = Group::new();
        g.add_child(s1.clone());
        g.add_child(s2.clone());
        g.add_child(s3.clone());

        // Test-hack to get around the fact that `s` doesn't get its parent set.
        // Only the child cloned into the group gets its parent set.
        //s1.set_parent(&g.base);
        //s2.set_parent(&g.base);
        //s3.set_parent(&g.base);
        todo!("Fix grouping and parents");

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
