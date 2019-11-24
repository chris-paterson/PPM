use crate::color::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Color>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let pixels = vec![Color::new(0., 0., 0.); width * height];

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn with_pixels(mut self, pixels: Vec<Color>) -> Self {
        self.pixels = pixels;
        self
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Option<&Color> {
        if !self.index_is_valid(x, y) {
            return None;
        }

        Some(&self.pixels[self.width * y + x])
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[self.width * y + x] = color;
    }

    fn index_is_valid(&self, x: usize, y: usize) -> bool {
        x <= self.width && y < self.height
    }
}
