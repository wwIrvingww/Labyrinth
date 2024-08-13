use crate::framebuffer::Framebuffer;
use image::GenericImageView;
use std::sync::Arc;

pub struct Sprite {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub width: usize,
    pub height: usize,
    texture_data: Arc<image::DynamicImage>,
}

impl Sprite {
    pub fn new(x: f32, y: f32, z: f32, texture_path: &str, width: usize, height: usize) -> Self {
        let img = image::open(texture_path).expect("Failed to load texture");
        Sprite {
            x,
            y,
            z,
            width,
            height,
            texture_data: Arc::new(img),
        }
    }

    pub fn get_color(&self, u: f32, v: f32) -> (u8, u8, u8) {
        let (width, height) = self.texture_data.dimensions();
        let x = ((u.clamp(0.0, 1.0)) * (width - 1) as f32) as u32; // Restar 1 para evitar out-of-bounds
        let y = ((v.clamp(0.0, 1.0)) * (height - 1) as f32) as u32; // Restar 1 para evitar out-of-bounds
        let pixel = self.texture_data.get_pixel(x, y);
        (pixel[0], pixel[1], pixel[2])
    }

    pub fn render(&self, framebuffer: &mut Framebuffer, camera_x: f32, camera_y: f32, camera_angle: f32) {
        let dx = self.x - camera_x;
        let dy = self.y - camera_y;
        let distance = (dx * dx + dy * dy).sqrt();
        let sprite_angle = dy.atan2(dx) - camera_angle;
    
        if sprite_angle.abs() > std::f32::consts::PI / 2.0 {
            return;
        }
    
        let screen_x = framebuffer.width as f32 / 2.0 * (1.0 + sprite_angle.tan());
        
        // Reducir el tamaño del sprite a la mitad
        let sprite_size = (framebuffer.height as f32 / distance / 4.0) as usize;
    
        let floor_offset = framebuffer.height as isize / 2;
    
        let half_width = sprite_size / 2;
        let half_height = sprite_size / 2;
    
        for y in 0..sprite_size {
            for x in 0..sprite_size {
                let framebuffer_x = screen_x as isize + x as isize - half_width as isize;
                let framebuffer_y = floor_offset + y as isize - half_height as isize;
    
                if framebuffer_x >= 0 && framebuffer_x < framebuffer.width as isize && framebuffer_y >= 0 && framebuffer_y < framebuffer.height as isize {
                    let texture_u = (x as f32 / sprite_size as f32).clamp(0.0, 1.0);
                    let texture_v = (y as f32 / sprite_size as f32).clamp(0.0, 1.0);
                    let (r, g, b) = self.get_color(texture_u, texture_v);
                    framebuffer.set_pixel(framebuffer_x, framebuffer_y, ((r as u32) << 16) | ((g as u32) << 8) | (b as u32));
                }
            }
        }
    }
}
