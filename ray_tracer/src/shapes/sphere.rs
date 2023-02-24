use crate::{
    intersections::Intersection,
    materials::Material,
    matrices::Matrix,
    rays::Ray,
    shapes::Shapes,
    tuples::{Point, Tuple, Vector},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub(super) struct Sphere {
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
    pub(super) fn set_transform(&mut self, trans: &Matrix) {
        self.transform = *trans;
    }
}

impl Shapes for Sphere {
    fn set_position(&mut self, pos: &Point) {
        self.position = *pos;
    }
    fn get_position(&self) -> Point {
        self.position
    }
    fn local_normal_at(&self, point: Point) -> Vector {
        point - Point::new_point(0.0, 0.0, 0.0)
    }
    fn get_shape_type(&self) -> super::ShapeType {
        super::ShapeType::Sphere
    }
    fn set_material(&mut self, material: &Material) {
        self.material = *material;
    }
    fn set_transform(&mut self, trans: &Matrix) {
        self.transform = *trans;
    }
    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
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
                Intersection::new(
                    (-b - discriminant_sqrt) / (2.0 * a),
                    super::Object::new(Box::new(*self)),
                ),
                Intersection::new(
                    (-b + discriminant_sqrt) / (2.0 * a),
                    super::Object::new(Box::new(*self)),
                ),
            ]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuples::Tuple;

    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
        let s = Sphere::new();
        let n = s.local_normal_at(Tuple::new_point(1.0, 0.0, 0.0));
        assert_eq!(n, Tuple::new_vector(1.0, 0.0, 0.0));
    }
    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
        let s = Sphere::new();
        let n = s.local_normal_at(Tuple::new_point(0.0, 1.0, 0.0));
        assert_eq!(n, Tuple::new_vector(0.0, 1.0, 0.0));
    }
    #[test]
    fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
        let s = Sphere::new();
        let n = s.local_normal_at(Tuple::new_point(0.0, 0.0, 1.0));
        assert_eq!(n, Tuple::new_vector(0.0, 0.0, 1.0));
    }
    #[test]
    fn the_normal_on_a_sphere_at_a_nonaxial_point() {
        let s = Sphere::new();
        let n = s.local_normal_at(Tuple::new_point(
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
        let n = s.local_normal_at(Tuple::new_point(
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
            f64::sqrt(3.0) / 3.0,
        ));
        assert_eq!(n, n.normalize())
    }
}
