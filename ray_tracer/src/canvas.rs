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
