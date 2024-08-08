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
mod minimap;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use renderer::{render2d, render3d, draw_text, FONT_WIDTH, FONT_HEIGHT}; // Importar las constantes necesarias
use player::Player;
use movement::process_events;
use camera::Camera;
use minimap::Minimap;

enum ScreenState {
    Game,
    Success,
}

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
    let minimap = Minimap::new(5, window_width - 70, 5); // Escala de 5 y offset en la esquina superior derecha
    let mut mode = "2D";

    let mut current_screen = ScreenState::Game;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        match current_screen {
            ScreenState::Game => {
                if window.is_key_pressed(Key::M, minifb::KeyRepeat::No) {
                    mode = if mode == "2D" { "3D" } else { "2D" };
                }

                let success = process_events(&window, &mut player, &maze, block_size, &mut framebuffer);

                if success {
                    current_screen = ScreenState::Success;
                    continue; // Saltar al siguiente ciclo para procesar la pantalla de éxito
                }

                camera.update(&window);
                player.a = camera.angle;  // Sincronizar el ángulo del jugador con el ángulo de la cámara

                framebuffer.clear();

                if mode == "2D" {
                    render2d(&mut framebuffer, &maze, block_size, &player, success);
                } else {
                    render3d(&mut framebuffer, &maze, block_size, &player, success);
                }

                // Dibujar el minimapa
                minimap.draw(&mut framebuffer, &maze, &player, block_size);
            }
            ScreenState::Success => {
                framebuffer.clear(); // Limpiar la pantalla

                // Dibujar fondo manualmente
                for y in 0..framebuffer.height {
                    for x in 0..framebuffer.width {
                        framebuffer.set_pixel(x as isize, y as isize, ((0 as u32) << 16) | ((0 as u32) << 8) | (0 as u32)); // Negro
                    }
                }

                // Dibujar texto "SUCCESS!" en el centro
                let text = "SUCCESS!";
                let scale = 10; // Escala del texto
                let text_width = (text.len() * (FONT_WIDTH + 1)) * scale; // 5 es FONT_WIDTH
                let text_height = FONT_HEIGHT * scale; // 7 es FONT_HEIGHT
                let x = (framebuffer.width - text_width) / 2;
                let y = (framebuffer.height - text_height) / 2;

                draw_text(
                    &mut framebuffer,
                    text,
                    x,
                    y,
                    scale,
                    framebuffer::Color { r: 255, g: 255, b: 255 }, // Blanco
                );

                // Esperar a que el usuario presione cualquier tecla
                if window.is_key_pressed(Key::Enter, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::Space, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::W, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::A, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::S, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::D, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::M, minifb::KeyRepeat::No)
                    || window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No)
                {
                    // Reiniciar la posición del jugador o realizar cualquier otra acción necesaria
                    // Por simplicidad, simplemente volvemos al juego sin reiniciar
                    current_screen = ScreenState::Game;
                    continue;
                }
            }
        }

        window
            .update_with_buffer(&framebuffer.buffer.iter().map(|color| {
                ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
            }).collect::<Vec<u32>>(), window_width, window_height)
            .unwrap();

        std::thread::sleep(frame_delay);
    }
}
