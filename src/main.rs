use std::time::{Duration, Instant};
use minifb::{Key, Window, WindowOptions};
use rand::Rng;

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
mod sprites;
mod gamepad;

use framebuffer::Framebuffer;
use player::Player;
use camera::Camera;
use minimap::Minimap;
use renderer::{render2d, render3d, render_sprite};
use menu::Menu;
use sprites::Sprite;
use gamepad::{Gamepad};
use crate::movement::process_events;

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
    let mut rng = rand::thread_rng();
    let mut sprite_timer = Instant::now();
    let mut trigger_time = rng.gen_range(0..15);

    let window_width = 640;
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

    let sprite_texture_path = "C:/Users/irvin/UVG/Sexto_Semestre/Graficas/Labyrinth/ghost.png";

    let mut sprites = vec![]; // Lista de fantasmas
    let mut gamepad = Gamepad::new(); // Inicializar el GamePad
    let mut mode = "2D";  // Inicializar la variable `mode`

    while window.is_open() && !window.is_key_down(Key::Escape) {
        match &mut current_screen {
            ScreenState::Menu => {
                framebuffer.clear(); 

                if let Some(selected_level) = menu.update(&window, &mut gamepad.gilrs) {
                    current_screen = ScreenState::Game(selected_level);
                    continue;
                }

                menu.draw(&mut framebuffer);
            }
            ScreenState::Game(level) => {
                let (maze, start_pos) = maze::reader::load_maze(level);

                let maze_width = maze[0].len() * block_size;
                let maze_height = maze.len() * block_size;

                framebuffer = Framebuffer::new(maze_width, maze_height); 
                window = Window::new(
                    "Maze Renderer",
                    maze_width,
                    maze_height,
                    WindowOptions::default(),
                ).unwrap();

                let mut player = Player::new((start_pos.0 * block_size + 40) as f32, (start_pos.1 * block_size + 40) as f32);
                let mut camera = Camera::new((start_pos.0 as f32, start_pos.1 as f32), 0.0, 0.1, 0.005);
                let minimap = Minimap::new(5, maze_width - 110, 5);
                let mut success = false;

                while window.is_open() && !window.is_key_down(Key::Escape) {
                    let moved = process_events(&window, &mut player, &maze, block_size, &mut framebuffer, &mut gamepad.gilrs);

                    let elapsed_time = sprite_timer.elapsed().as_secs();

                    // Solo disparar sprites si está en modo 3D
                    if mode == "3D" && elapsed_time >= trigger_time {
                        let ghost_x = player.pos.x + player.a.cos() * block_size as f32;
                        let ghost_y = player.pos.y + player.a.sin() * block_size as f32;

                        if sprites.len() >= 3 {
                            sprites.remove(0);
                        }

                        let sprite = Sprite::new(ghost_x, ghost_y, 0.0, sprite_texture_path, 16, 16);
                        sprites.push(sprite);

                        sprite_timer = Instant::now();
                        trigger_time = rng.gen_range(0..15);
                    }

                    if success {
                        current_screen = ScreenState::Success;
                        break;
                    }

                    camera.update(&window, &mut gamepad.gilrs);
                    player.a = camera.angle;

                    framebuffer.clear();

                    let mut z_buffer = vec![f32::MAX; framebuffer.width];

                    if mode == "2D" {
                        render2d(&mut framebuffer, &maze, block_size, &player, success);
                    } else {
                        render3d(&mut framebuffer, &maze, block_size, &player, success);
                    }

                    minimap.draw(&mut framebuffer, &maze, &player, &sprites, block_size);

                    for sprite in &sprites {
                        render_sprite(&mut framebuffer, &player, sprite, &mut z_buffer);
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
                framebuffer.clear();

                for y in 0..framebuffer.height {
                    for x in 0..framebuffer.width {
                        framebuffer.set_pixel(x as isize, y as isize, 0);
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

                if window.get_keys().len() > 0 || window.get_mouse_down(minifb::MouseButton::Left) || window.get_mouse_down(minifb::MouseButton::Right) {
                    current_screen = ScreenState::Menu;
                    framebuffer = Framebuffer::new(window_width, window_height); 
                    menu = Menu::new(); 
                    window = Window::new(
                        "Maze Renderer",
                        window_width,
                        window_height,
                        WindowOptions::default(),
                    ).unwrap();
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
