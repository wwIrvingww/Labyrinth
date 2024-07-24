use crate::framebuffer::Framebuffer;
use crate::player::Player;

fn draw_cell(framebuffer: &mut Framebuffer, x0: usize, y0: usize, block_size: usize, cell: char) {
    let color = match cell {
        '+' | '|' | '-' => 0x5A639C, // Paredes
        ' ' => 0xE2BBE9, // Caminos
        'g' => 0x00FF00, // Meta
        'p' => 0xE2BBE9, // Punto de inicio
        _ => 0xE2BBE9,  // Por defecto, color de fondo
    };

    framebuffer.set_current_color(color);

    for x in 0..block_size {
        for y in 0..block_size {
            framebuffer.point((x0 + x) as isize, (y0 + y) as isize);
        }
    }
}

pub fn render(framebuffer: &mut Framebuffer, maze: &Vec<Vec<char>>, block_size: usize, player: &Player) {
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
        }
    }

    // Dibujar el jugador como un punto
    framebuffer.set_current_color(0xFFFF00); // Color amarillo para el jugador
    let player_x = (player.pos.x as usize) * block_size + block_size / 2;
    let player_y = (player.pos.y as usize) * block_size + block_size / 2;
    framebuffer.point(player_x as isize, player_y as isize);
}
