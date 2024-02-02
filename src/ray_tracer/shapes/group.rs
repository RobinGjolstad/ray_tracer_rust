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
    position: Point,
    transform: Matrix,
    material: Option<Material>,
    children: Option<Arc<[Object]>>,
}

impl Group {
    pub fn new() -> Self {
        Group {
            position: Point::new_point(0.0, 0.0, 0.0),
            transform: Matrix::new_identity()
                .calculate_inverse()
                .expect("Failed to calculate inverse of identity matrix."),
            material: None,
            children: None,
        }
    }

    pub(super) fn get_children(&self) -> Option<Arc<[Object]>> {
        self.children.clone()
    }

    fn has_children(&self) -> bool {
        self.children.is_some()
    }
}

impl Default for Group {
    fn default() -> Self {
        Self::new()
    }
}

impl Shapes for Group {
    fn set_position(&mut self, pos: &Point) {
        self.position = *pos;
    }
    fn get_position(&self) -> Point {
        self.position
    }
    fn set_transform(&mut self, transform: &Matrix) {
        // Apply transformation to self,
        // then recursively apply to all children.
        // If a child is a group, apply the transformation to it and its children.
        self.transform = self.transform * *transform;
    }
    fn get_transform(&self) -> Matrix {
        self.transform
    }
    fn set_material(&mut self, material: &Material) {
        self.material = Some(*material);
    }
    fn get_material(&self) -> Material {
        if self.material.is_none() {
            Material::new()
        } else {
            self.material.unwrap()
        }
    }
    fn local_normal_at(&self, point: Point) -> Vector {
        Vector::new_vector(point.x, point.y, point.z)
    }
    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
        // All children have their transformations already prepared for conversion to world space.
        // So, we can just intersect the ray with each child.
        let children = if let Some(c) = self.get_children() {
            c
        } else {
            return vec![];
        };

        let mut retval = vec![];
        children.iter().for_each(|child| {
            retval.append(&mut local_ray.intersect(child));
        });

        retval.sort_by(|a, b| {
            a.get_time()
                .partial_cmp(&b.get_time())
                .expect("Sorting intersections for group intersections failed.")
        });
        retval.dedup();

        retval
    }
}

pub(crate) struct GroupBuilder {
    children: Vec<Object>,
    transform: Matrix,
    material: Option<Material>,
}

impl GroupBuilder {
    pub fn new() -> GroupBuilder {
        GroupBuilder {
            children: Vec::new(),
            transform: Matrix::new_identity()
                .calculate_inverse()
                .expect("Failed to calculate inverse of identity matrix."),
            material: None,
        }
    }
    pub(crate) fn add(mut self, child: Object) -> GroupBuilder {
        self.children.push(child);
        self
    }
    pub(crate) fn add_children(mut self, children: Vec<Object>) -> GroupBuilder {
        self.children.append(&mut children.clone());
        self
    }
    pub fn set_transform(mut self, transform: Matrix) -> GroupBuilder {
        let mut trans = transform;
        self.transform = trans
            .calculate_inverse()
            .expect("Failed to calculate inverse.");
        self
    }
    pub fn set_material(mut self, material: Material) -> GroupBuilder {
        self.material = Some(material);
        self
    }

    /// Recursively build a group from a list of objects.
    ///
    /// Any materials or transformations applied to the group will be applied to all children.
    pub(crate) fn build(self) -> Object {
        // Transformations applied to "leaf" children should be gradually constructed from the
        // outermost group in to the leaf.
        // As an example, given group G1 with child G2 with child S1, the transformations should be
        // constructed as follows:
        // G1.transform * G2.transform * S1.transform

        let mut children = self.children;
        children.iter_mut().for_each(|child| {
            let new_transform = self.transform * child.get_transform();
            if let Object::Group(g) = child {
                // Re-build the group with the current group's transform.
                let new_g = GroupBuilder::new()
                    .add_children(g.get_children().unwrap().to_vec())
                    .set_transform(new_transform)
                    .build();
                *child = new_g;
            } else {
                child.set_transform(&new_transform);
            }
        });

        Object::Group(Group {
            position: Point::new_point(0.0, 0.0, 0.0),
            // Own transform has been applied to all children now.
            // To prevent it from being re-applied, create the group with an identity transform.
            transform: Matrix::new_identity().calculate_inverse().unwrap(),
            //transform: self.transform,
            material: self.material,
            children: Some(children.into()),
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::ray_tracer::transformations::Transform;

    use super::*;

    #[test]
    fn creating_a_group() {
        let g = Group::new();

        assert_eq!(g.get_transform(), Matrix::new_identity());
        assert!(!g.has_children());
    }
    #[test]
    fn adding_a_child_to_a_group_keeps_the_childs_transformations() {
        let mut s = new_test_shape();
        s.set_transform(&Transform::translate(5.0, 0.0, 0.0));
        let mut g = new_group(vec![s.clone()]);

        let group = if let Object::Group(g) = g {
            g
        } else {
            panic!("Failed to get group from object.");
        };

        let children = group.get_children().unwrap();

        assert_eq!(children.len(), 1);
        assert_eq!(
            children.first().unwrap().get_transform(),
            Transform::translate(5.0, 0.0, 0.0)
        );
    }
    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let g = new_group(vec![]);
        let r = Ray::new(
            Point::new_point(0.0, 0.0, 0.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );

        let xs = g.local_intersect(r);

        assert!(xs.is_empty());
    }
    #[test]
    fn intersecting_a_ray_with_a_nonempty_group() {
        let mut s1 = new_sphere();
        let mut s2 = new_sphere();
        let mut s3 = new_sphere();
        s2.set_transform(&Transform::translate(0.0, 0.0, -3.0));
        s3.set_transform(&Transform::translate(5.0, 0.0, 0.0));

        let mut g = new_group(vec![s1.clone(), s2.clone(), s3.clone()]);

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
        let mut s = new_sphere();
        s.set_transform(&Transform::translate(5.0, 0.0, 0.0));
        let g = GroupBuilder::new()
            .add(s.clone())
            .set_transform(Transform::scaling(2.0, 2.0, 2.0))
            .build();

        let r = Ray::new(
            Point::new_point(10.0, 0.0, -10.0),
            Vector::new_vector(0.0, 0.0, 1.0),
        );

        let xs = r.intersect(&g);

        assert_eq!(xs.len(), 2);
    }
}
