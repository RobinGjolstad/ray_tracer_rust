use crate::colors::Color;

pub struct Canvas {
    pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            pixels: vec![vec![Color::new(0.0, 0.0, 0.0); width]; height],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn access_nested_vec() {
        let canvas = Canvas::new(10, 10);

        let element = canvas.pixels.get(2).unwrap().get(4).unwrap();
        assert_eq!(element, Color::new(0.0, 0.0, 0.0));
        assert_eq!( Color::new(0.0, 0.0, 0.0), element);

    }
}