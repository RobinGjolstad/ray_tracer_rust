use crate::{colors::Color, matrices::Matrix, shapes::Object, tuples::Point};

pub mod stripes;
mod test_pattern;

pub(crate) trait Patterns {
    fn set_transform(&mut self, transformation: Matrix);
    fn get_transform(&self) -> Matrix;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pattern {
    pub(crate) color_a: Color,
    pub(crate) color_b: Color,
    pub(crate) transform: Matrix,
}
impl Pattern {
    pub fn stripe_default() -> Self {
        Pattern {
            color_a: Color::new(1.0, 1.0, 1.0),
            color_b: Color::new(0.0, 0.0, 0.0),
            transform: Matrix::new_identity().calculate_inverse().unwrap(),
        }
    }
    pub fn stripe(color_a: Color, color_b: Color) -> Self {
        Pattern {
            color_a,
            color_b,
            transform: Matrix::new_identity().calculate_inverse().unwrap(),
        }
    }
    pub(crate) fn stripe_at(&self, point: Point) -> Color {
        if point.x.floor() as isize % 2 == 0 {
            self.color_a
        } else {
            self.color_b
        }
    }

    pub(crate) fn stripe_at_object(pattern: Pattern, object: &Object, world_point: Point) -> Color {
        let object_point = object.get_transform().get_inverted().unwrap() * world_point;
        let pattern_point = pattern.get_transform().get_inverted().unwrap() * object_point;

        pattern.stripe_at(pattern_point)
    }

    pub fn set_transform(&mut self, transformation: Matrix) {
        let mut transform = transformation;
        transform.calculate_inverse().unwrap();
        self.transform = transform;
    }

    fn get_transform(&self) -> Matrix {
        self.transform
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{colors::Color, shapes::Object, transformations::Transform, tuples::Point};

    const WHITE: Color = Color {
        red: 1.0,
        green: 1.0,
        blue: 1.0,
    };
    const BLACK: Color = Color {
        red: 0.0,
        green: 0.0,
        blue: 0.0,
    };

    #[test]
    fn creating_a_stripe_pattern() {
        let pattern = Pattern::stripe(WHITE, BLACK);
        assert_eq!(pattern.color_a, WHITE);
        assert_eq!(pattern.color_b, BLACK);
    }
    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = Pattern::stripe(WHITE, BLACK);
        assert_eq!(pattern.stripe_at(Point::new_point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new_point(0.0, 1.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new_point(0.0, 2.0, 0.0)), WHITE);
    }
    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = Pattern::stripe(WHITE, BLACK);
        assert_eq!(pattern.stripe_at(Point::new_point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new_point(0.0, 0.0, 1.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new_point(0.0, 0.0, 2.0)), WHITE);
    }
    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = Pattern::stripe(WHITE, BLACK);
        assert_eq!(pattern.stripe_at(Point::new_point(0.0, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new_point(0.9, 0.0, 0.0)), WHITE);
        assert_eq!(pattern.stripe_at(Point::new_point(1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.stripe_at(Point::new_point(-0.1, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.stripe_at(Point::new_point(-1.0, 0.0, 0.0)), BLACK);
        assert_eq!(pattern.stripe_at(Point::new_point(-1.1, 0.0, 0.0)), WHITE);
    }
    #[test]
    fn stripes_with_an_object_transformation() {
        let mut object = Object::new_sphere();
        object.set_transform(
            &Transform::scaling(2.0, 2.0, 2.0)
                .calculate_inverse()
                .unwrap(),
        );
        let pattern = Pattern::stripe(WHITE, BLACK);
        let c = Pattern::stripe_at_object(pattern, &object, Point::new_point(1.5, 0.0, 0.0));
        assert_eq!(c, WHITE);
    }
    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Object::new_sphere();
        let mut pattern = Pattern::stripe(WHITE, BLACK);
        pattern.set_transform(
            Transform::scaling(2.0, 2.0, 2.0)
                .calculate_inverse()
                .unwrap(),
        );
        let c = Pattern::stripe_at_object(pattern, &object, Point::new_point(1.5, 0.0, 0.0));
        assert_eq!(c, WHITE);
    }
    #[test]
    fn stripes_with_both_and_object_and_a_pattern_transformation() {
        let mut object = Object::new_sphere();
        object.set_transform(
            &Transform::scaling(2.0, 2.0, 2.0)
                .calculate_inverse()
                .unwrap(),
        );
        let mut pattern = Pattern::stripe(WHITE, BLACK);
        pattern.set_transform(
            Transform::translate(0.5, 0.0, 0.0)
                .calculate_inverse()
                .unwrap(),
        );
        let c = Pattern::stripe_at_object(pattern, &object, Point::new_point(1.5, 0.0, 0.0));
        assert_eq!(c, WHITE);
    }
}
