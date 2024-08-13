use crate::framebuffer::{Framebuffer, Color}; 
use crate::player::Player;
use crate::intersect::cast_ray;
use crate::texture::Texture;
use once_cell::sync::Lazy;
use std::sync::Arc;
use crate::sprites::Sprite;

pub const FONT_WIDTH: usize = 5;
pub const FONT_HEIGHT: usize = 7;

pub const FONT: &[(&str, [u8; FONT_HEIGHT])] = &[
    ("S", [
        0b01110,
        0b10000,
        0b10000,
        0b01110,
        0b00010,
        0b00010,
        0b11100,
    ]),
    ("U", [
        0b10010,
        0b10010,
        0b10010,
        0b10010,
        0b10010,
        0b10010,
        0b01100,
    ]),
    ("C", [
        0b01110,
        0b10010,
        0b10000,
        0b10000,
        0b10000,
        0b10010,
        0b01110,
    ]),
    ("E", [
        0b11110,
        0b10000,
        0b10000,
        0b11100,
        0b10000,
        0b10000,
        0b11110,
    ]),
    ("!", [
        0b00100,
        0b00100,
        0b00100,
        0b00100,
        0b00000,
        0b00000,
        0b00100,
    ]),
    (" ", [
        0b00000,
        0b00000,
        0b00000,
        0b00000,
        0b00000,
        0b00000,
        0b00000,
    ]),
];

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

pub fn draw_text(
    framebuffer: &mut Framebuffer,
    text: &str,
    x: usize,
    y: usize,
    scale: usize,
    color: Color,
) {
    let chars: Vec<char> = text.chars().collect();
    let mut cursor_x = x;

    for ch in chars {
        if let Some(font_char) = FONT.iter().find(|&&(c, _)| c == ch.to_string()) {
            for (row, bits) in font_char.1.iter().enumerate() {
                for col in 0..FONT_WIDTH {
                    if (bits >> (FONT_WIDTH - 1 - col)) & 1 == 1 {
                        for sx in 0..scale {
                            for sy in 0..scale {
                                let px = (cursor_x + col * scale + sx).try_into().unwrap();
                                let py = (y + row * scale + sy).try_into().unwrap();
                                if (px as usize) < framebuffer.width && (py as usize) < framebuffer.height {
                                    framebuffer.set_pixel(px, py, ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32));
                                }
                            }
                        }
                    }
                }
            }
            cursor_x += (FONT_WIDTH + 1) * scale;
        }
    }
}

// static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| Arc::new(Texture::new("wall1.png")));

// Crear una textura procedural en lugar de cargar desde un archivo
static WALL1: Lazy<Arc<Texture>> = Lazy::new(|| {
    Arc::new(Texture::from_procedural_pattern(512, 512))
});

pub fn render2d(framebuffer: &mut Framebuffer, maze: &[Vec<char>], block_size: usize, player: &Player, success: bool) {
    framebuffer.clear();

    for row in 0..maze.len() {
        for col in 0..maze[row].len() {
            draw_cell(framebuffer, col * block_size, row * block_size, block_size, maze[row][col]);
        }
    }

    draw_player(framebuffer, player);

    if success {
        draw_text(framebuffer, "SUCCESS", framebuffer.width / 2 - 50, framebuffer.height / 2 - 10, 2, Color { r: 255, g: 0, b: 255 });
    }

    let num_rays = 5;
    for i in 0..num_rays {
        let current_ray = i as f32 / num_rays as f32;
        let a = player.a - (player.fov / 2.0) + (player.fov * current_ray);
        cast_ray(framebuffer, &maze.to_vec(), &player, a, block_size, true);
    }
}



pub fn render3d(framebuffer: &mut Framebuffer, maze: &[Vec<char>], block_size: usize, player: &Player, success: bool) {
    let num_rays = framebuffer.width;
    let hh = framebuffer.height as f32 / 2.0;

    // Establece el color del cielo
    framebuffer.set_current_color(0x3F4670); // Color del cielo

    for y in 0..hh as usize {
        for x in 0..framebuffer.width {
            framebuffer.point(x as isize, y as isize);
        }
    }

    // Establece el color del suelo
    framebuffer.set_current_color(0x565680); // Color del suelo

    for y in hh as usize..framebuffer.height {
        for x in 0..framebuffer.width {
            framebuffer.point(x as isize, y as isize);
        }
    }

    if success {
        draw_text(framebuffer, "SUCCESS", framebuffer.width / 2 - 50, framebuffer.height / 2 - 10, 2, Color { r: 255, g: 0, b: 255 });
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

pub fn render_sprite(
    framebuffer: &mut Framebuffer,
    player: &Player,
    sprite: &Sprite,
    z_buffer: &mut [f32],
) {
    let sprite_a = (sprite.y - player.pos.y).atan2(sprite.x - player.pos.x) - player.a;

    if sprite_a < -player.fov / 2.0 || sprite_a > player.fov / 2.0 {
        return;
    }

    let sprite_d = ((player.pos.x - sprite.x).powi(2) + (player.pos.y - sprite.y).powi(2)).sqrt();

    let screen_height = framebuffer.height as f32;
    let screen_width = framebuffer.width as f32;

    let sprite_size = (screen_height / sprite_d) * 100.0;
    let start_x = (sprite_a * (screen_height / player.fov) + screen_width / 2.0) - sprite_size / 2.0;
    let start_y = (screen_height / 2.0) - sprite_size / 2.0;

    let end_x = ((start_x + sprite_size) as usize).min(framebuffer.width);
    let end_y = ((start_y + sprite_size) as usize).min(framebuffer.height);
    let start_x = start_x.max(0.0) as usize;
    let start_y = start_y.max(0.0) as usize;

    if start_x < framebuffer.width && sprite_d < z_buffer[start_x] {
        for x in start_x..end_x {
            for y in start_y..end_y {
                let tx = ((x - start_x) * (sprite.width - 1) / sprite_size as usize) as u32;
                let ty = ((y - start_y) * (sprite.height - 1) / sprite_size as usize) as u32;
                let color = sprite.get_color(tx as f32 / sprite.width as f32, ty as f32 / sprite.height as f32);

                if color != (0x98, 0x00, 0x88) { 
                    framebuffer.set_current_color((color.0 as u32) << 16 | (color.1 as u32) << 8 | color.2 as u32);
                    framebuffer.point(x as isize, y as isize);
                }
                z_buffer[x] = sprite_d;
            }
        }
    }
}

