use image::GenericImageView;
use std::sync::Arc;
use std::env;

pub struct Texture {
    pub data: Arc<image::DynamicImage>,
}

impl Texture {
    pub fn new(path: &str) -> Self {
        println!("Directorio de trabajo actual: {:?}", env::current_dir());
        println!("Cargando textura desde: {}", path);
        let img = image::open("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/Labyrinth/wall1.png").expect("Failed to load texture");        Texture {
            data: Arc::new(img),
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