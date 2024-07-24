use crate::framebuffer::Framebuffer;
use crate::maze::reader::load_maze;

fn draw_cell(framebuffer: &mut Framebuffer, xo: usize, yo: usize, block_size: usize, cell: char) {
    match cell {
        '+' => framebuffer.set_current_color(0x5A639C),
        '-' => framebuffer.set_current_color(0x5A639C),
        '|' => framebuffer.set_current_color(0x5A639C),
        ' ' => framebuffer.set_current_color(0xE2BBE9),
        'g' => framebuffer.set_current_color(0xE2BBE9),
        'p' => {
            framebuffer.set_current_color(0x00FF00); // Verde para el jugador
            draw_circle(framebuffer, xo + block_size / 2, yo + block_size / 2, (block_size / 4) as isize);
            return;
        },
        _ => framebuffer.set_current_color(0xE2BBE9),
    }

    for y in yo..yo + block_size {
        for x in xo..xo + block_size {
            framebuffer.point(x as isize, y as isize);
        }
    }
}

fn draw_circle(framebuffer: &mut Framebuffer, xo: usize, yo: usize, radius: isize) {
    let radius_squared = (radius * radius) as isize;

    for y in -radius..=radius {
        for x in -radius..=radius {
            if x * x + y * y <= radius_squared {
                framebuffer.point((xo as isize + x) as isize, (yo as isize + y) as isize);
            }
        }
    }
}

pub fn render(framebuffer: &mut Framebuffer) {
    let maze = load_maze("maze.txt");
    let block_size = framebuffer.width / maze[0].len();

    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
        }
    }
}
