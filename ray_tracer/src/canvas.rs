use crate::colors::Color;

pub struct Canvas<'a> {
    pixels: &'a [Color],
}

impl Canvas<'_> {
    pub fn new(width: i32, height: i32) -> Self {
        const array_len: usize = width * height;
        Canvas {
            pixels: &[Color::new(0.0, 0.0, 0.0); array_len],
        }
    }
}
