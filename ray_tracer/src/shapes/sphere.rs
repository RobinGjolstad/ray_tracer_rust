use crate::{
    materials::Material,
    matrices::Matrix,
    shapes::Shapes,
    tuples::{Point, Vector, Tuple}, intersections::Intersection, rays::Ray,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere {
    position: Point,
    transform: Matrix,
    material: Material,
}
impl Sphere {
    pub fn new() -> Self {
        Sphere {
            position: Point::new_point(0.0, 0.0, 0.0),
            transform: Matrix::new_identity(),
            material: Material::new(),
        }
    }
}

impl Shapes for Sphere {
    fn set_position(&mut self, pos: &Point) {
        self.position = *pos;
    }
    fn get_position(&self) -> Point {
        self.position
    }
    fn get_transform(&self) -> Matrix {
        self.transform
    }
    fn set_transform(&mut self, trans: &Matrix) {
        self.transform = *trans;
        self.transform.inverse().unwrap();
    }
    fn set_material(&mut self, material: &Material) {
        self.material = *material;
    }
    fn get_material(&self) -> Material {
        self.material
    }
    fn normal_at(&self, point: Point) -> Vector {
        let transform_inverse = self.get_transform().get_inverted().unwrap();
        let object_point = transform_inverse * point;
        let object_normal = object_point - Point::new_point(0.0, 0.0, 0.0);
        let mut world_normal = transform_inverse.transpose().unwrap() * object_normal;
        world_normal.w = 0.0;

        world_normal.normalize()
    }
    fn get_shape_type(&self) -> super::ShapeType {
        super::ShapeType::Sphere
    }
    fn local_intersect(&self,local_ray:Ray) -> Vec<Intersection> {
        let sphere_to_ray = local_ray.origin - self.get_position();
        let a = Tuple::dot(&local_ray.direction, &local_ray.direction);
        let b = 2.0 * Tuple::dot(&local_ray.direction, &sphere_to_ray);
        let c = Tuple::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;

        let discriminant = b.powi(2) - 4.0 * a * c;
        let discriminant_sqrt = discriminant.sqrt();

        if discriminant < 0.0 {
            Vec::new()
        } else {
            vec![
                Intersection::new((-b - discriminant_sqrt) / (2.0 * a), super::Object::new(Box::new(*self))),
                Intersection::new((-b + discriminant_sqrt) / (2.0 * a), super::Object::new(Box::new(*self))),
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{transformations::Transform, tuples::Tuple};
    use std::f64::consts::PI;

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::new_point(1.0, 0.0, 0.0));
        assert_eq!(n, Tuple::new_vector(1.0, 0.0, 0.0));
    }
    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::new_point(0.0, 1.0, 0.0));
        assert_eq!(n, Tuple::new_vector(0.0, 1.0, 0.0));
    }
    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::new_point(0.0, 0.0, 1.0));
        assert_eq!(n, Tuple::new_vector(0.0, 0.0, 1.0));
    }
    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::new_point(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));
        assert_eq!(
            n,
            Tuple::new_vector(
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0,
                f64::sqrt(3.0) / 3.0
            )
        );
    }
    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere::new();
        let n = s.normal_at(Tuple::new_point(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));
        assert_eq!(n, n.normalize())
    }
    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = Sphere::new();
        s.set_transform(&Transform::translate(0.0, 1.0, 0.0));
        let n = s.normal_at(Point::new_point(0.0, 1.70711, -0.70711));
        assert_eq!(n, Vector::new_vector(0.0, 0.70711, -0.70711));
    }
    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = Sphere::new();
        let m = Transform::scaling(1.0, 0.5, 1.0) * Transform::rotation_z(PI / 5.0);
        s.set_transform(&m);
        let n = s.normal_at(Point::new_point(
            0.0,
            f64::sqrt(2.0) / 2.0,
            -(f64::sqrt(2.0) / 2.0),
        ));
        assert_eq!(n, Vector::new_vector(0.0, 0.97014, -0.24254));
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = Sphere::new();
        let m = s.get_material();
        assert_eq!(m, Material::new());
    }
    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut s = Sphere::new();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.set_material(&m);
        assert_eq!(s.get_material(), m);
    }
}
