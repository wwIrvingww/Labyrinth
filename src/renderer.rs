use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::intersect::cast_ray;

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

fn draw_player(framebuffer: &mut Framebuffer, player: &Player) {
    framebuffer.set_current_color(0xFFFF00); // Color amarillo para el jugador
    framebuffer.point(player.pos.x as isize, player.pos.y as isize);
}

pub fn render(framebuffer: &mut Framebuffer, maze: &[Vec<char>], block_size: usize, player: &Player) {
    framebuffer.clear();

    // Dibujar el laberinto
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
        }
    }

    // Dibujar el jugador
    draw_player(framebuffer, player);

    // Dibujar la línea de visión del jugador
    let num_rays = 5; // Número de rayos
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32; // rayo actual dividido por total de rayos
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze.to_vec(), &player, a, block_size, true);
    }
}
