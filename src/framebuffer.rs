#[derive(Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Framebuffer {
    pub width: usize,
    pub height: usize,
    pub buffer: Vec<Color>,
    pub current_color: Color,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        let buffer = vec![Color { r: 0, g: 0, b: 0 }; width * height];
        let current_color = Color { r: 255, g: 255, b: 255 };
        Framebuffer {
            width,
            height,
            buffer,
            current_color,
        }
    }

    pub fn clear(&mut self) {
        self.buffer.fill(Color { r: 0, g: 0, b: 0 });
    }

    pub fn set_current_color(&mut self, color: u32) {
        self.current_color = Color {
            r: ((color >> 16) & 0xFF) as u8,
            g: ((color >> 8) & 0xFF) as u8,
            b: (color & 0xFF) as u8,
        };
    }

    pub fn point(&mut self, x: isize, y: isize) {
        if x >= 0 && y >= 0 && x < self.width as isize && y < self.height as isize {
            let index = (y as usize) * self.width + (x as usize);
            self.buffer[index] = self.current_color;
        }
    }

    pub fn draw_sky(&mut self, color: Color) {
        let height = self.height / 2;
        for y in 0..height {
            for x in 0..self.width {
                self.buffer[y * self.width + x] = color;
            }
        }
    }
}
