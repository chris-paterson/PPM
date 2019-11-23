mod canvas;
mod color;

use canvas::Canvas;
use color::Color;

use std::fs;

// TODO(chris): Return an actual error
pub fn load(filename: &str) -> Option<Canvas> {
    let content = match fs::read_to_string(&filename) {
        Ok(file_content) => file_content,
        Err(_) => return None,
    };

    let lines = content.split('\n').collect();

    // Header
    let (width, height, scale_max) = match parse_header(&lines) {
        Some((w, h, max)) => (w, h, max),
        None => return None,
    };

    // Image
    let pixels = match parse_body(&lines, scale_max) {
        Some(pixels) => pixels,
        None => return None,
    };

    Some(Canvas::new(width, height).with_pixels(pixels))
}

fn parse_header(lines: &Vec<&str>) -> Option<(usize, usize, f32)> {
    let dimensions: Vec<&str> = match lines.get(1) {
        Some(dimensions) => dimensions.split(' ').collect(),
        None => return None,
    };

    let width = dimensions[0].parse::<usize>().unwrap();
    let height = dimensions[1].parse::<usize>().unwrap();

    let scale_max: f32 = match lines.get(2) {
        Some(size) => size.parse::<f32>().unwrap(),
        None => return None,
    };

    Some((width, height, scale_max))
}

fn parse_body(lines: &Vec<&str>, scale_max: f32) -> Option<Vec<Color>> {
    let content: Vec<f32> = lines[3..]
        .join(" ") // Merge into one string
        .trim_end()
        .split(' ') // Split into vec of r, g, b values
        .map(|x| x.parse::<f32>().unwrap())
        .collect();

    assert!(content.len() % 3 == 0);

    let mut pixels = Vec::new();
    for i in (0..content.len()).step_by(3) {
        let r = round_decimal(content[i] / scale_max);
        let g = round_decimal(content[i + 1] / scale_max);
        let b = round_decimal(content[i + 2] / scale_max);

        pixels.push(Color::new(r, g, b));
    }

    Some(pixels)
}

fn round_decimal(num: f32) -> f32 {
    (num * 100.0).round() / 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edit_canvas() {
        let mut canvas = Canvas::new(8, 8);
        let pixel = canvas.pixel_at(0, 0).unwrap();
        assert_eq!(*pixel, Color::new(0.0, 0.0, 0.0));

        let new_color = Color::new(0.5, 0.5, 0.5);
        canvas.set_pixel(0, 0, new_color);
        assert_eq!(*canvas.pixel_at(0, 0).unwrap(), Color::new(0.5, 0.5, 0.5));
    }

    #[test]
    fn test_parse_header() {
        let input = "P3
32 64
255
";

        let lines = input.split('\n').collect();
        let (width, height, scale_max) = parse_header(&lines).unwrap();
        assert!(width == 32);
        assert!(height == 64);
        assert!(scale_max == 255.0);
    }

    #[test]
    fn test_parse_body() {
        let input = "P3
8 8
255
255 127 255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 255 127 255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 255 127 255 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 255 127 255 0 0 0 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 255 127 255 0 0 0 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 127 255 0 0 0 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 127 255 0 0 0
0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 255 127 255
";

        let lines = input.split('\n').collect();

        let pixels = parse_body(&lines, 255.0).unwrap();
        assert!(pixels[0] == Color::new(1.0, 0.5, 1.0))
    }

    #[test]
    fn test_parse_body_different_scale() {
        let input = "P3
2 2
16
16 4 8 0 0 0
0 0 0 0 0 0
";

        let lines = input.split('\n').collect();

        let pixels = parse_body(&lines, 16.0).unwrap();
        assert!(pixels[0] == Color::new(1.0, 0.25, 0.5))
    }
}
