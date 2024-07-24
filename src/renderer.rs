use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::vision::cast_ray;

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

pub fn render(framebuffer: &mut Framebuffer, maze: &[Vec<char>], block_size: usize, player: &Player) {
    framebuffer.clear();

    // Dibujar el laberinto
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
        }
    }

    // Dibujar el jugador como un punto
    framebuffer.set_current_color(0xFFFF00); // Color amarillo para el jugador
    if player.pos.x >= 0.0 && player.pos.y >= 0.0 && player.pos.x < framebuffer.width as f32 && player.pos.y < framebuffer.height as f32 {
        framebuffer.point(player.pos.x as isize, player.pos.y as isize);
    }

    // Dibujar la línea de visión del jugador
    cast_ray(framebuffer, maze, player, player.a, block_size);

    println!("Player angle: {}", player.a);

    cast_ray(framebuffer, maze, player, player.a, block_size);
}