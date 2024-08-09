use minifb::{Key, Window};
use crate::framebuffer::{Framebuffer, Color}; // Importa el framebuffer y Color

pub struct Menu {
    pub selected: usize, // Indica el cuadro seleccionado (0, 1 o 2)
}

impl Menu {
    pub fn new() -> Self {
        Menu {
            selected: 0, // Comienza con el primer cuadro seleccionado
        }
    }

    pub fn update(&mut self, window: &Window) -> Option<String> {
        if window.is_key_pressed(Key::D, minifb::KeyRepeat::No) {
            self.selected = (self.selected + 1) % 3; // Avanza al siguiente cuadro
        }
        if window.is_key_pressed(Key::A, minifb::KeyRepeat::No) {
            if self.selected == 0 {
                self.selected = 2; // Mueve la selección al último cuadro
            } else {
                self.selected -= 1; // Retrocede al cuadro anterior
            }
        }

        if window.is_key_pressed(Key::Enter, minifb::KeyRepeat::No) {
            return Some(format!("maze{}.txt", self.selected + 1)); // Selecciona el nivel basado en el cuadro
        }

        None
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer) {
        framebuffer.clear();

        let box_width = 100;
        let box_height = 100;
        let spacing = 50;
        let start_x = (framebuffer.width - 3 * box_width - 2 * spacing) / 2;
        let y_position = framebuffer.height / 3;

        for i in 0..3 {
            let x = start_x + i * (box_width + spacing);
            let color = if i == self.selected {
                Color { r: 0, g: 255, b: 0 } // Verde para el cuadro seleccionado
            } else {
                Color { r: 255, g: 255, b: 255 } // Blanco para los demás cuadros
            };

            // Dibujar cuadro
            self.draw_rectangle(framebuffer, x, y_position, box_width, box_height, color);

            // Dibujar número del nivel en el centro del cuadro
            let level_text = format!("{}", i + 1);
            let text_x = x + (box_width - (level_text.len() * (5 + 1)) * 10) / 2;
            let text_y = y_position + (box_height - 7 * 10) / 2;
            draw_text(framebuffer, &level_text, text_x, text_y, 10, Color { r: 0, g: 0, b: 0 });
        }

        // Dibujar flecha debajo del cuadro seleccionado
        let arrow_x = start_x + self.selected * (box_width + spacing) + box_width / 2 - 5;
        let arrow_y = y_position + box_height + 20;
        self.draw_arrow(framebuffer, arrow_x, arrow_y);
    }

    fn draw_rectangle(&self, framebuffer: &mut Framebuffer, x: usize, y: usize, width: usize, height: usize, color: Color) {
        for i in 0..width {
            for j in 0..height {
                framebuffer.set_pixel((x + i) as isize, (y + j) as isize, ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32));
            }
        }
    }

    fn draw_arrow(&self, framebuffer: &mut Framebuffer, x: usize, y: usize) {
        let arrow = [
            (0, 0),
            (1, 0),
            (2, 0),
            (3, 0),
            (4, 0),
            (1, 1),
            (2, 1),
            (3, 1),
            (2, 2),
        ];
        for &(dx, dy) in arrow.iter() {
            framebuffer.set_pixel((x + dx) as isize, (y + dy) as isize, (255 << 16) | (255 << 8) | 255); // Flecha blanca
        }
    }
}

fn draw_text(
    framebuffer: &mut Framebuffer,
    text: &str,
    x: usize,
    y: usize,
    scale: usize,
    color: Color,
) {
    let chars: Vec<char> = text.chars().collect();
    let mut cursor_x = x;

    for ch in chars {
        if let Some(font_char) = crate::renderer::FONT.iter().find(|&&(c, _)| c == ch.to_string()) {
            for (row, bits) in font_char.1.iter().enumerate() {
                for col in 0..crate::renderer::FONT_WIDTH {
                    if (bits >> (crate::renderer::FONT_WIDTH - 1 - col)) & 1 == 1 {
                        // Dibujar píxeles según la escala
                        for sx in 0..scale {
                            for sy in 0..scale {
                                let px = (cursor_x + col * scale + sx).try_into().unwrap();
                                let py = (y + row * scale + sy).try_into().unwrap();
                                if (px as usize) < framebuffer.width && (py as usize) < framebuffer.height {
                                    framebuffer.set_pixel(px, py, ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32));
                                }
                            }
                        }
                    }
                }
            }
            cursor_x += (crate::renderer::FONT_WIDTH + 1) * scale; // Espacio entre caracteres
        }
    }
}
