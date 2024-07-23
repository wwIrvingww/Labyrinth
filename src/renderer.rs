use crate::framebuffer::Framebuffer;
use crate::maze::reader::load_maze;

fn draw_cell(framebuffer: &mut Framebuffer, x0: usize, y0: usize, block_size: usize, cell: char) {
    let color = match cell {
        '+' | '|' | '-' => 0x5A639C, // Paredes
        ' ' => 0xE2BBE9, // Caminos
        'p' => 0xF7E7DC, // Jugador 
        'g' => 0x9B86BD, // Meta 
        _ => 0x000000,  // Por defecto, negro
    };

    framebuffer.set_current_color(color);

    for x in 0..block_size {
        for y in 0..block_size {
            framebuffer.point((x0 + x) as isize, (y0 + y) as isize);
        }
    }
}

pub fn render(framebuffer: &mut Framebuffer, maze: &Vec<Vec<char>>, block_size: usize) {
    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
        }
    }
}