use crate::colors::Color;

struct PixelColorString {}

pub struct Canvas {
    pixels: Vec<Vec<Color>>,
    width: usize,
    height: usize,
    ppm: String,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        // Instantiate the canvas with defined dimensions and empty fields
        Canvas {
            pixels: vec![vec![Color::new(0.0, 0.0, 0.0); width]; height],
            width: width,
            height: height,
            ppm: "".to_string(),
        }
    }
    pub fn pixel_at(&self, x: usize, y: usize) -> &Color {
        self.pixels.get(y).unwrap().get(x).unwrap()
    }
    fn get_mut_pixel(&mut self, x: usize, y: usize) -> &mut Color {
        self.pixels.get_mut(y).unwrap().get_mut(x).unwrap()
    }
    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        let pixel = self.get_mut_pixel(x, y);
        *pixel = color;
    }

    /// Save the canvas to a file
    fn canvas_to_ppm(&mut self) {
        // Set up the PPM header
        self.ppm.push_str("P3\n");
        self.ppm
            .push_str(format!("{} {}\n", self.width, self.height).as_str());
        self.ppm.push_str("255\n");

        // Insert pixel elements from the canvas
        for row in &self.pixels {
            for column in row {
                self.ppm.push_str(format!(
                    "{} {} {} ",
                    Color::float_to_u8(&column.red),
                    Color::float_to_u8(&column.green),
                    Color::float_to_u8(&column.blue)
                ).as_str());
            }
            self.ppm = self.ppm.trim().to_string();
            self.ppm.push_str("\n");
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
        assert_eq!(Color::new(0.0, 0.0, 0.0), element);
    }

    #[test]
    fn a_new_canvas_has_all_black_pixels() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        for vector in c.pixels {
            for color in vector {
                assert_eq!(color, Color::new(0.0, 0.0, 0.0));
            }
        }
    }

    #[test]
    fn writing_a_color_to_a_pixel_sets_colors_for_that_pixel() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1.0, 0.0, 0.0);

        c.write_pixel(2, 3, red);
        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn constructing_a_ppm_header() {
        let mut c = Canvas::new(5, 3);
        c.canvas_to_ppm();
        let comp_string = "P3\n5 3\n255\n".to_string();

        let mut ppm_iter = c.ppm.split("\n");
        let mut comp_iter = comp_string.split("\n");

        // Test each line separately
        assert_eq!(ppm_iter.next().unwrap(), comp_iter.next().unwrap());
        assert_eq!(ppm_iter.next().unwrap(), comp_iter.next().unwrap());
        assert_eq!(ppm_iter.next().unwrap(), comp_iter.next().unwrap());
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);
        c.canvas_to_ppm();

        let mut comp_string = "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n".to_string();
        comp_string.push_str("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n");
        comp_string.push_str("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n");
        let mut comp_string_iter = comp_string.split("\n");

        let ppm_iter = c.ppm.split("\n");
        let mut ppm_start_comp = ppm_iter.skip(3);

        assert_eq!(ppm_start_comp.next(), comp_string_iter.next());
    }
}
