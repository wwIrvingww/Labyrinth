use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub struct Intersect {
    pub distance: f32,
    pub impact: char,
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

    framebuffer.set_current_color(0xFFDDDD);

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = (player.pos.x + cos) as isize;
        let y = (player.pos.y + sin) as isize;

        // Convert our coordinates in pixels to indices in the maze array
        let i = (x as usize) / block_size;
        let j = (y as usize) / block_size;

        // If the current item is a wall, we break the loop
        if maze[j][i] != ' ' {
            return Intersect {
                distance: d,
                impact: maze[j][i],
            };
        }

        if draw_line {
            framebuffer.point(x, y);
        }

        d += 1.0;
    }
}

pub fn render3d(framebuffer: &mut Framebuffer, maze: &[Vec<char>], player: &Player, block_size: usize) {
    framebuffer.clear();
    
    let num_rays = 2; // Número de rayos a lanzar
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32; // rayo actual dividido por el total de rayos
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, maze, player, a, block_size, true);
    }
}