mod framebuffer;
mod maze {
    pub mod reader;
    pub mod generator;
}
mod intersect;
mod movement;
mod player;
mod renderer;
mod vision;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use renderer::render;
use player::Player;
use movement::process_events;

fn main() {
    let block_size = 40;
    let (maze, start_pos) = maze::reader::load_maze("./maze.txt");

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

    // Crear al jugador en la posición inicial más 20 en cada eje
    let mut player = Player::new((start_pos.0 * block_size + 40) as f32, (start_pos.1 * block_size + 40) as f32);

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        process_events(&window, &mut player, &maze, block_size);

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
