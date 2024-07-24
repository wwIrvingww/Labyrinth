mod framebuffer;
mod maze {
    pub mod reader;
    pub mod generator;
}
mod renderer;
mod player;
mod vision;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use renderer::render;
use player::Player;

fn main() {
    let block_size = 40;
    let (maze, player_pos) = maze::reader::load_maze("./maze.txt");

    let window_width = maze[0].len() * block_size;
    let window_height = maze.len() * block_size;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(window_width, window_height);

    let mut window = Window::new(
        "Maze Renderer",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // Crear al jugador en la posición inicial encontrada en el laberinto
    let player_x = player_pos.1 as f32 * block_size as f32 + block_size as f32 / 2.0;
    let player_y = player_pos.0 as f32 * block_size as f32 + block_size as f32 / 2.0;
    let mut player = Player::new(player_x, player_y);

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        // Movimiento del jugador
        if window.is_key_down(Key::W) {
            move_player(&mut player, -1.0, 0.0, &maze, block_size);
        }
        if window.is_key_down(Key::S) {
            move_player(&mut player, 1.0, 0.0, &maze, block_size);
        }
        if window.is_key_down(Key::A) {
            move_player(&mut player, 0.0, -1.0, &maze, block_size);
        }
        if window.is_key_down(Key::D) {
            move_player(&mut player, 0.0, 1.0, &maze, block_size);
        }

        framebuffer.clear();

        // Renderizar el laberinto y la línea de visión
        render(&mut framebuffer, &maze, block_size, &player);

        window
            .update_with_buffer(&framebuffer.buffer.iter().map(|color| {
                ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
            }).collect::<Vec<u32>>(), window_width, window_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}

fn move_player(player: &mut Player, dy: f32, dx: f32, maze: &[Vec<char>], block_size: usize) {
    let new_x = player.pos.x + dx * block_size as f32;
    let new_y = player.pos.y + dy * block_size as f32;

    let col = (new_x / block_size as f32) as usize;
    let row = (new_y / block_size as f32) as usize;

    if maze[row][col] == ' ' || maze[row][col] == 'p' || maze[row][col] == 'g' {
        player.pos.x = new_x;
        player.pos.y = new_y;
    }
}
