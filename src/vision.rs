use crate::framebuffer::Framebuffer;
use crate::player::Player;

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &[Vec<char>],
    player: &Player,
    a: f32,
    block_size: usize,
) {
    let mut d = 0.0;

    framebuffer.set_current_color(0xFFDDDD);

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        // Convert our coordinates in pixels to indices in the maze array
        let i = x / block_size;
        let j = y / block_size;

        // If the current item is a wall, we break the loop
        if maze[j][i] != ' ' {
            return;
        }

        framebuffer.point(x as isize, y as isize);

        d += 1.0;
    }
}
