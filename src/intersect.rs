use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
    pub texture_coord: f32,
}

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &[Vec<char>],
    player: &Player,
    a: f32,
    block_size: usize,
    draw_line: bool,
) -> Intersect {
    let mut d = 0.0;

    framebuffer.set_current_color(0xFDDDDD);

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = (player.pos.x + cos) as isize;
        let y = (player.pos.y + sin) as isize;

        let i = (x as usize) / block_size;
        let j = (y as usize) / block_size;

        // Corregir la comparación de caracteres vacíos
        if maze[j][i] != ' ' && maze[j][i] != 'p' && maze[j][i] != 'g' {
            let texture_coord = if maze[j][i] == '|' {
                (y as f32 % block_size as f32) / block_size as f32
            } else {
                (x as f32 % block_size as f32) / block_size as f32
            };
            return Intersect {
                distance: d,
                impact: maze[j][i],
                texture_coord,
            };
        }

        if draw_line {
            framebuffer.point(x, y);
        }

        // Corregir la comparación de caracteres vacíos
        if maze[j][i] != ' ' && maze[j][i] != 'p' && maze[j][i] != 'g' && maze[j][i] != '|' {
            let texture_coord = if maze[j][i] == '|' {
                (y as f32 % block_size as f32) / block_size as f32
            } else {
                (x as f32 % block_size as f32) / block_size as f32
            };
            return Intersect {
                distance: d,
                impact: maze[j][i],
                texture_coord,
            };
        }

        d += 0.1;
    }
}
