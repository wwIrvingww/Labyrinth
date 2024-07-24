mod framebuffer;
mod maze {
    pub mod reader;
    pub mod generator;
}
mod renderer;
mod player;
mod vision;
mod movement;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use renderer::render;
use player::Player;
use movement::handle_player_movement;

fn main() {
    let block_size = 40;
    let (maze, player_start) = maze::reader::load_maze("./maze.txt");

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

    // Crear al jugador en la posición inicial y sumarle 20 en x y 20 en y
    let mut player = Player::new(
        player_start.0 as f32 * block_size as f32 + 20.0,
        player_start.1 as f32 * block_size as f32 + 20.0,
    );

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        framebuffer.clear();

        // Mover al jugador basado en la entrada del teclado
        handle_player_movement(&mut player, &window, &maze, block_size);

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
