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
mod menu;

use framebuffer::Framebuffer;
use minifb::{Key, Window, WindowOptions};
use std::time::Duration;
use renderer::{render2d, render3d};
use player::Player;
use movement::process_events;
use camera::Camera;
use minimap::Minimap;
use menu::Menu;

enum ScreenState {
    Menu,
    Game(String),
    Success,
}

fn main() {
    loop {
        run_game();
    }
}

fn run_game() {
    let block_size = 40;

    let window_width = 640; // Tamaño fijo para la pantalla del menú
    let window_height = 480;
    let frame_delay = Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(window_width, window_height);

    let mut window = Window::new(
        "Maze Renderer",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    let mut current_screen = ScreenState::Menu;
    let mut menu = Menu::new();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        match &mut current_screen {
            ScreenState::Menu => {
                framebuffer.clear();  // Asegúrate de limpiar el framebuffer antes de dibujar el menú

                if let Some(selected_level) = menu.update(&window) {
                    current_screen = ScreenState::Game(selected_level);
                    continue;
                }

                menu.draw(&mut framebuffer);
            }
            ScreenState::Game(level) => {
                let (maze, start_pos) = maze::reader::load_maze(level);

                let maze_width = maze[0].len() * block_size;
                let maze_height = maze.len() * block_size;

                framebuffer = Framebuffer::new(maze_width, maze_height); // Ajustar el framebuffer al tamaño del maze
                window = Window::new(
                    "Maze Renderer",
                    maze_width,
                    maze_height,
                    WindowOptions::default(),
                ).unwrap();

                let mut player = Player::new((start_pos.0 * block_size + 40) as f32, (start_pos.1 * block_size + 40) as f32);
                let mut camera = Camera::new((start_pos.0 as f32, start_pos.1 as f32), 0.0, 0.1, 0.005);
                let minimap = Minimap::new(5, maze_width - 70, 5);
                let mut mode = "2D";
                let mut success = false;

                while window.is_open() && !window.is_key_down(Key::Escape) {
                    match current_screen {
                        ScreenState::Game(_) => {
                            if window.is_key_pressed(Key::M, minifb::KeyRepeat::No) {
                                mode = if mode == "2D" { "3D" } else { "2D" };
                            }

                            success = process_events(&window, &mut player, &maze, block_size, &mut framebuffer);

                            if success {
                                current_screen = ScreenState::Success;
                                break; // Romper el bucle interno para evitar seguir procesando el juego
                            }

                            camera.update(&window);
                            player.a = camera.angle;  // Sincronizar el ángulo del jugador con el ángulo de la cámara

                            framebuffer.clear();

                            if mode == "2D" {
                                render2d(&mut framebuffer, &maze, block_size, &player, success);
                            } else {
                                render3d(&mut framebuffer, &maze, block_size, &player, success);
                            }

                            minimap.draw(&mut framebuffer, &maze, &player, block_size);
                        }
                        _ => {}
                    }

                    window
                        .update_with_buffer(&framebuffer.buffer.iter().map(|color| {
                            ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
                        }).collect::<Vec<u32>>(), maze_width, maze_height)
                        .unwrap();

                    std::thread::sleep(frame_delay);
                }
            }
            ScreenState::Success => {
                framebuffer.clear(); // Limpiar la pantalla

                // Dibujar fondo manualmente
                for y in 0..framebuffer.height {
                    for x in 0..framebuffer.width {
                        framebuffer.set_pixel(x as isize, y as isize, 0); // Negro
                    }
                }

                let text = "SUCCESS!";
                let scale = 10;
                let text_width = (text.len() * (renderer::FONT_WIDTH + 1)) * scale;
                let text_height = renderer::FONT_HEIGHT * scale;
                let x = (framebuffer.width - text_width) / 2;
                let y = (framebuffer.height - text_height) / 2;

                renderer::draw_text(&mut framebuffer, text, x, y, scale, framebuffer::Color { r: 255, g: 255, b: 255 });

                window
                    .update_with_buffer(&framebuffer.buffer.iter().map(|color| {
                        ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
                    }).collect::<Vec<u32>>(), window_width, window_height)
                    .unwrap();

                // Verificar interacción del usuario
                if window.get_keys().len() > 0 || window.get_mouse_down(minifb::MouseButton::Left) || window.get_mouse_down(minifb::MouseButton::Right) {
                    // Reiniciar al estado inicial (menú)
                    current_screen = ScreenState::Menu;
                    framebuffer = Framebuffer::new(window_width, window_height); // Restablecer framebuffer al tamaño original del menú
                    menu = Menu::new(); // Reiniciar el menú
                    window = Window::new(
                        "Maze Renderer",
                        window_width,
                        window_height,
                        WindowOptions::default(),
                    ).unwrap(); // Reiniciar la ventana
                    continue; // Saltar de inmediato al menú
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
