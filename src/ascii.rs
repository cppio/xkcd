use image::{DynamicImage, ImageBuffer, Pixel};

const RAMP: [char; 10] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];

fn to_char(value: u8) -> char {
    RAMP[(value as f64 / 255.0 * (RAMP.len() - 1) as f64 + 0.5) as usize]
}

fn buffer_to_string(buffer: &ImageBuffer<impl Pixel<Subpixel = u8> + 'static, Vec<u8>>) -> String {
    let mut result = String::new();

    for row in buffer.rows() {
        for pixel in row {
            result.push(to_char(pixel.to_luma().0[0]));
        }

        result.push('\n');
    }

    result
}

pub fn to_string(image: &DynamicImage) -> String {
    match image {
        DynamicImage::ImageLuma8(buffer) => buffer_to_string(buffer),
        DynamicImage::ImageLumaA8(buffer) => buffer_to_string(buffer),
        DynamicImage::ImageRgb8(buffer) => buffer_to_string(buffer),
        DynamicImage::ImageRgba8(buffer) => buffer_to_string(buffer),
        DynamicImage::ImageBgr8(buffer) => buffer_to_string(buffer),
        DynamicImage::ImageBgra8(buffer) => buffer_to_string(buffer),
    }
}
