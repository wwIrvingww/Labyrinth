mod framebuffer;
mod maze {
    pub mod reader;
    pub mod generator;
}
mod renderer;
mod player;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use renderer::render;
use player::Player;

fn main() {
    let block_size = 40;
    let maze = maze::reader::load_maze("./maze.txt");

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

    // Inicializar el jugador en la posición 'p' del archivo del laberinto
    let mut player = Player::new(0.0, 0.0);
    for (row, line) in maze.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            if cell == 'p' {
                player = Player::new(col as f32, row as f32);
                break;
            }
        }
    }

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        framebuffer.clear();

        render(&mut framebuffer, &maze, block_size, &player);

        window
            .update_with_buffer(&framebuffer.buffer.iter().map(|color| {
                ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
            }).collect::<Vec<u32>>(), window_width, window_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
