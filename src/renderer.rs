use crate::framebuffer::Framebuffer;
use crate::player::Player;
use crate::intersect::cast_ray;
use crate::texture::Texture;
use once_cell::sync::Lazy;
use std::sync::Arc;

static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("wall1.png")));

fn draw_cell(framebuffer: &mut Framebuffer, x0: usize, y0: usize, block_size: usize, cell: char) {
    let color = match cell {
        '+' | '|' | '-' => 0x5A639C,
        ' ' => 0xE2BBE9,
        'g' => 0x00FF00,
        'p' => 0xE2BBE9,
        _ => 0xE2BBE9,
    };

    framebuffer.set_current_color(color);

    for x in 0..block_size {
        for y in 0..block_size {
            framebuffer.point((x0 + x) as isize, (y0 + y) as isize);
        }
    }
}

fn draw_player(framebuffer: &mut Framebuffer, player: &Player) {
    framebuffer.set_current_color(0xFFFF00);
    framebuffer.point(player.pos.x as isize, player.pos.y as isize);
}

pub fn render2d(framebuffer: &mut Framebuffer, maze: &[Vec<char>], block_size: usize, player: &Player) {
    framebuffer.clear();

    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
        }
    }

    draw_player(framebuffer, player);

    let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze.to_vec(), &player, a, block_size, true);
    }
}

pub fn render3d(framebuffer: &mut Framebuffer, maze: &[Vec<char>], block_size: usize, player: &Player) {
    let num_rays = framebuffer.width;
    let hw = framebuffer.width as f32 / 2.0;
    let hh = framebuffer.height as f32 / 2.0;

    framebuffer.set_current_color(0x87CEEB); // Color azul para el cielo

    for y in 0..hh as usize {
        for x in 0..framebuffer.width {
            framebuffer.point(x as isize, y as isize);
        }
    }

    framebuffer.set_current_color(0x8B4513); // Color marrón para el suelo

    for y in hh as usize..framebuffer.height {
        for x in 0..framebuffer.width {
            framebuffer.point(x as isize, y as isize);
        }
    }

    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        let intersect = cast_ray(framebuffer, &maze.to_vec(), &player, a, block_size, false);

        let distance_to_wall = intersect.distance * (a - player.a).cos();
        let distance_to_projection_plane = 100.0;
        let stake_height = (hh / distance_to_wall) * distance_to_projection_plane;

        let stake_top = (hh - (stake_height / 2.0)) as usize;
        let stake_bottom = (hh + (stake_height / 2.0)) as usize;

        let texture = &*WALL1;
        for y in stake_top..stake_bottom {
            let tex_y = (y as f32 - stake_top as f32) / (stake_bottom as f32 - stake_top as f32);
            let (r, g, b) = texture.get_color(intersect.texture_coord, tex_y);
            framebuffer.set_current_color((r as u32) << 16 | (g as u32) << 8 | b as u32);
            framebuffer.point(i as isize, y as isize);
        }
    }
}
