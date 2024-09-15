use dyn_clonable::clonable;
use std::fmt::Debug;

#[cfg(test)]
use self::test_pattern::TestPattern;
#[cfg(test)]
use std::io::ErrorKind;

use crate::ray_tracer::{colors::Color, matrices::Matrix, shapes::Object, tuples::Point};

use self::{checker::Checker, gradient::Gradient, rings::Ring, solid::Solid, stripes::Stripes};

pub mod checker;
pub mod gradient;
pub mod rings;
pub mod solid;
pub mod stripes;

#[cfg(test)]
pub(crate) mod test_pattern;

#[derive(Debug, Clone, Copy, PartialEq)]
enum PatternType {
    Stripes(Stripes),
    Gradient(Gradient),
    Ring(Ring),
    Checker(Checker),
    Solid(Solid),

    #[cfg(test)]
    TestPattern(TestPattern),
}

#[clonable]
pub(crate) trait Patterns: Debug + Clone + Sync {
    fn color_at(&self, point: Point) -> Color;
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pattern {
    pattern: PatternType,
    transform: Matrix,
}
impl Pattern {
    #[must_use]
    pub fn stripe_default() -> Self {
        Self {
            pattern: PatternType::Stripes(Stripes::default()),
            transform: Matrix::new_identity().calculate_inverse().unwrap(),
        }
    }
    #[must_use]
    pub fn stripe(color_a: Color, color_b: Color) -> Self {
        Self {
            pattern: PatternType::Stripes(Stripes::new(color_a, color_b)),
            transform: Matrix::new_identity().calculate_inverse().unwrap(),
        }
    }
    #[must_use]
    pub fn gradient(color_a: Color, color_b: Color) -> Self {
        Self {
            pattern: PatternType::Gradient(Gradient::new(color_a, color_b)),
            transform: Matrix::new_identity().calculate_inverse().unwrap(),
        }
    }
    #[must_use]
    pub fn gradient_default() -> Self {
        Self {
            pattern: PatternType::Gradient(Gradient::default()),
            transform: Matrix::new_identity().calculate_inverse().unwrap(),
        }
    }

    #[must_use]
    pub fn ring(color_a: Color, color_b: Color) -> Self {
        Self {
            pattern: PatternType::Ring(Ring::new(color_a, color_b)),
            transform: Matrix::new_identity().calculate_inverse().unwrap(),
        }
    }
    #[must_use]
    pub fn ring_default() -> Self {
        Self {
            pattern: PatternType::Ring(Ring::default()),
            transform: Matrix::new_identity().calculate_inverse().unwrap(),
        }
    }

    #[must_use]
    pub fn checker(color_a: Color, color_b: Color) -> Self {
        Self {
            pattern: PatternType::Checker(Checker::new(color_a, color_b)),
            transform: Matrix::new_identity().calculate_inverse().unwrap(),
        }
    }
    #[must_use]
    pub fn checker_default() -> Self {
        Self {
            pattern: PatternType::Checker(Checker::default()),
            transform: Matrix::new_identity().calculate_inverse().unwrap(),
        }
    }

    #[must_use]
    pub fn solid(color: Color) -> Self {
        Self {
            pattern: PatternType::Solid(Solid::new(color)),
            transform: Matrix::new_identity().calculate_inverse().unwrap(),
        }
    }
    #[must_use]
    pub fn solid_default() -> Self {
        Self {
            pattern: PatternType::Solid(Solid::default()),
            transform: Matrix::new_identity().calculate_inverse().unwrap(),
        }
    }

    fn pattern_at(&self, point: Point) -> Color {
        match self.pattern {
            PatternType::Stripes(s) => s.color_at(point),
            PatternType::Gradient(g) => g.color_at(point),
            PatternType::Ring(r) => r.color_at(point),
            PatternType::Checker(c) => c.color_at(point),
            PatternType::Solid(s) => s.color_at(point),

            #[cfg(test)]
            PatternType::TestPattern(tp) => tp.color_at(point),
        }
    }
    pub(crate) fn pattern_at_object(pattern: Self, object: &Object, world_point: Point) -> Color {
        let object_point = object.world_to_object(&world_point);
        let pattern_point = pattern.get_transform().get_inverted().unwrap() * object_point;

        pattern.pattern_at(pattern_point)
    }

    pub fn set_transform(&mut self, transformation: Matrix) {
        let mut transform = transformation;
        transform.calculate_inverse().unwrap();
        self.transform = transform;
    }

    #[must_use]
    pub const fn get_transform(&self) -> Matrix {
        self.transform
    }
}

#[cfg(test)]
impl Pattern {
    const fn get_stripe(&self) -> Result<Stripes, ErrorKind> {
        match self.pattern {
            PatternType::Stripes(s) => Ok(s),
            _ => Err(ErrorKind::NotFound),
        }
    }

    pub(crate) fn test_pattern_default() -> Self {
        Self {
            pattern: PatternType::TestPattern(TestPattern::default()),
            transform: Matrix::new_identity().calculate_inverse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray_tracer::shapes::new_sphere;
    use crate::ray_tracer::transformations::Transform;

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let mut object = new_sphere();
        object.set_transform(
            &Transform::scaling(2.0, 2.0, 2.0)
                .calculate_inverse()
                .unwrap(),
        );
        let pattern = Pattern::test_pattern_default();
        let c = Pattern::pattern_at_object(pattern, &object, Point::new_point(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }
    #[test]
    fn a_pattern_with_a_pattern_transformation() {
        let object = new_sphere();
        let mut pattern = Pattern::test_pattern_default();
        pattern.set_transform(
            Transform::scaling(2.0, 2.0, 2.0)
                .calculate_inverse()
                .unwrap(),
        );
        let c = Pattern::pattern_at_object(pattern, &object, Point::new_point(2.0, 3.0, 4.0));
        assert_eq!(c, Color::new(1.0, 1.5, 2.0));
    }
    #[test]
    fn a_pattern_with_both_and_object_and_a_pattern_transformation() {
        let mut object = new_sphere();
        object.set_transform(
            &Transform::scaling(2.0, 2.0, 2.0)
                .calculate_inverse()
                .unwrap(),
        );
        let mut pattern = Pattern::test_pattern_default();
        pattern.set_transform(
            Transform::translate(0.5, 1.0, 1.5)
                .calculate_inverse()
                .unwrap(),
        );
        let c = Pattern::pattern_at_object(pattern, &object, Point::new_point(2.5, 3.0, 3.5));
        assert_eq!(c, Color::new(0.75, 0.5, 0.25));
    }

    #[test]
    fn the_default_pattern_transformation() {
        let pattern = Pattern::test_pattern_default();
        assert_eq!(pattern.transform, Matrix::new_identity());
    }
    #[test]
    fn assigning_a_transformation() {
        let mut pattern = Pattern::test_pattern_default();
        pattern.set_transform(Transform::translate(1.0, 2.0, 3.0));
        assert_eq!(pattern.transform, Transform::translate(1.0, 2.0, 3.0));
    }
}
