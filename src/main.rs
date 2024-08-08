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
mod texture;
mod camera;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use renderer::{render2d, render3d};
use player::Player;
use movement::process_events;
use camera::Camera;

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

    let mut player = Player::new((start_pos.0 * block_size + 40) as f32, (start_pos.1 * block_size + 40) as f32);
    let mut camera = Camera::new((start_pos.0 as f32, start_pos.1 as f32), 0.0, 0.1, 0.005); // Ajuste del rotation_speed
    let mut mode = "2D";

    while window.is_open() {
        if window.is_key_down(Key::Escape) {
            break;
        }

        if window.is_key_down(Key::M) {
            mode = if mode == "2D" { "3D" } else { "2D" };
        }

        process_events(&window, &mut player, &maze, block_size, &mut framebuffer);

        camera.update(&window);
        player.a = camera.angle;  // Sincronizar el ángulo del jugador con el ángulo de la cámara

        framebuffer.clear();

        if mode == "2D" {
            render2d(&mut framebuffer, &maze, block_size, &player);
        } else {
            render3d(&mut framebuffer, &maze, block_size, &player);
        }

        window
            .update_with_buffer(&framebuffer.buffer.iter().map(|color| {
                ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
            }).collect::<Vec<u32>>(), window_width, window_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
