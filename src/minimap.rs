use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub struct Minimap {
    pub scale: usize,
    pub offset_x: usize,
    pub offset_y: usize,
}

impl Minimap {
    pub fn new(scale: usize, offset_x: usize, offset_y: usize) -> Self {
        Minimap {
            scale,
            offset_x,
            offset_y,
        }
    }

    pub fn draw(&self, framebuffer: &mut Framebuffer, maze: &[Vec<char>], player: &Player, block_size: usize) {
        for (j, row) in maze.iter().enumerate() {
            for (i, &cell) in row.iter().enumerate() {
                let color = match cell {
                    ' ' => 0xFFFFFF, // Color blanco para caminos
                    '|' | '-' => 0x000000, // Color negro para paredes
                    'p' => 0x0000FF, // Color azul para el inicio
                    'g' => 0x00FF00, // Color verde para la meta
                    _ => 0xFF0000, // Color rojo para cualquier otro caracter
                };

                let x = i * self.scale + self.offset_x;
                let y = j * self.scale + self.offset_y;

                for dx in 0..self.scale {
                    for dy in 0..self.scale {
                        framebuffer.set_pixel((x + dx) as isize, (y + dy) as isize, color);
                    }
                }
            }
        }

        // Dibujar al jugador en color rosa
        let player_x = (player.pos.x as usize / block_size) * self.scale + self.offset_x;
        let player_y = (player.pos.y as usize / block_size) * self.scale + self.offset_y;
        for dx in 0..self.scale {
            for dy in 0..self.scale {
                framebuffer.set_pixel((player_x + dx) as isize, (player_y + dy) as isize, 0xFF00FF); // Rosa para el jugador
            }
        }
    }
}
