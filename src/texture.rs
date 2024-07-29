use image::{DynamicImage, GenericImageView};
use std::path::Path;

pub struct Texture {
    pub image: DynamicImage,
    pub width: u32,
    pub height: u32,
}

impl Texture {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self, image::ImageError> {
        let img = image::open(path)?;
        let (width, height) = img.dimensions();
        Ok(Texture {
            image: img,
            width,
            height,
        })
    }

    pub fn get_color(&self, x: u32, y: u32) -> u32 {
        let pixel = self.image.get_pixel(x, y);
        ((pixel[0] as u32) << 16) | ((pixel[1] as u32) << 8) | (pixel[2] as u32)
    }
}
