use image::{ImageBuffer, RgbImage, Rgb, GenericImageView};
use noise::{NoiseFn, Perlin};
use std::sync::Arc;

pub struct Texture {
    pub data: Arc<image::DynamicImage>,
}

impl Texture {
    pub fn new(path: &str) -> Self {
        let img = image::open(path).expect("Failed to load texture");
        Texture {
            data: Arc::new(img),
        }
    }

    pub fn from_procedural_pattern(width: u32, height: u32) -> Self {
        let perlin = Perlin::new();
        let mut img: RgbImage = ImageBuffer::new(width, height);
    
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let value = perlin.get([x as f64 / width as f64, y as f64 / height as f64]);
            let intensity = ((value + 1.0) * 128.0) as u8;
    
            // Mezcla de color hacia #E2BBE9 (R: 226, G: 187, B: 233)
            let red_target = 226;
            let green_target = 187;
            let blue_target = 233;
    
            *pixel = Rgb([
                ((intensity as u32 + red_target as u32) / 2).min(255) as u8,   // Canal rojo
                ((intensity as u32 + green_target as u32) / 2).min(255) as u8, // Canal verde
                ((intensity as u32 + blue_target as u32) / 2).min(255) as u8   // Canal azul
            ]);
        }
    
        Texture {
            data: Arc::new(image::DynamicImage::ImageRgb8(img)),
        }
    }
    

    pub fn get_color(&self, u: f32, v: f32) -> (u8, u8, u8) {
        let (width, height) = self.data.dimensions();
        let x = (u * width as f32) as u32;
        let y = (v * height as f32) as u32;
        let pixel = self.data.get_pixel(x, y);
        (pixel[0], pixel[1], pixel[2])
    }
}
