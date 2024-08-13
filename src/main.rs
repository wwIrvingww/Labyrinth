use std::time::{Duration, Instant};
use minifb::{Key, Window, WindowOptions};
use image::{GenericImageView, imageops::FilterType}; // Importa el tipo de filtro para el escalado
use rand::Rng; // Importa el rasgo Rng para usar gen_range

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
mod audio_player;

use framebuffer::Framebuffer;
use player::Player;
use camera::Camera;
use minimap::Minimap;
use renderer::{render2d, render3d, render_sprite};
use menu::Menu;
use sprites::Sprite;
use gamepad::{Gamepad, GamepadInput};
use crate::movement::process_events;

use audio_player::AudioPlayer;

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
    let mut trigger_time = rng.gen_range(5..60);

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
    
    let audio_file_path = "C:/Users/irvin/UVG/Sexto_Semestre/Graficas/Labyrinth/song.mp3"; // Reemplaza con la ruta a tu archivo de música
    let audio_player = AudioPlayer::new(audio_file_path); // Inicializa el reproductor de audio

    let mut sprites = vec![]; // Lista de fantasmas
    let mut gamepad = Gamepad::new(); // Inicializar el GamePad
    let mut mode = "2D";  // Inicializar la variable `mode`

    while window.is_open() && !window.is_key_down(Key::Escape) {
        match &mut current_screen {
            ScreenState::Menu => {
                framebuffer.clear();  // Limpia el framebuffer antes de dibujar el menú

                if let Some(selected_level) = menu.update(&window, &mut gamepad.gilrs) {
                    current_screen = ScreenState::Game(selected_level);
                    audio_player.play(); // Comienza a reproducir la música al iniciar el laberinto
                    continue;
                }

                menu.draw(&mut framebuffer);

                window
                    .update_with_buffer(&framebuffer.buffer.iter().map(|color| {
                        ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
                    }).collect::<Vec<u32>>(), window_width, window_height)
                    .unwrap();
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
                    // Lógica para cambiar entre modo 2D y 3D con la tecla 'm' o el botón 'B'/'Círculo' del gamepad
                    if window.is_key_pressed(Key::M, minifb::KeyRepeat::No) {
                        mode = if mode == "2D" { "3D" } else { "2D" };
                    }

                    if let Some(gamepad_input) = gamepad.update() {
                        match gamepad_input {
                            GamepadInput::ToggleMode => {
                                mode = if mode == "2D" { "3D" } else { "2D" };
                            }
                            _ => {}
                        }
                    }

                    let reached_goal = process_events(&window, &mut player, &maze, block_size, &mut framebuffer, &mut gamepad.gilrs);

                    if reached_goal {
                        success = true;
                        break;
                    }

                    let elapsed_time = sprite_timer.elapsed().as_secs();

                    if mode == "3D" {
                        // Solo disparar sprites si está en modo 3D
                        if elapsed_time >= trigger_time {
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

                        render3d(&mut framebuffer, &maze, block_size, &player, success);

                        let mut z_buffer = vec![f32::MAX; framebuffer.width];

                        for sprite in &sprites {
                            render_sprite(&mut framebuffer, &player, sprite, &mut z_buffer);
                        }
                    } else {
                        render2d(&mut framebuffer, &maze, block_size, &player, success);
                    }

                    minimap.draw(&mut framebuffer, &maze, &player, &sprites, block_size);

                    window
                        .update_with_buffer(&framebuffer.buffer.iter().map(|color| {
                            ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
                        }).collect::<Vec<u32>>(), maze_width, maze_height)
                        .unwrap();

                    std::thread::sleep(frame_delay);
                }

                if success {
                    current_screen = ScreenState::Success;
                    audio_player.stop(); // Detener la música al llegar a la pantalla de éxito

                    // Re-inicializa el framebuffer y la ventana para la pantalla de éxito
                    framebuffer = Framebuffer::new(window_width, window_height); 
                    window = Window::new(
                        "Maze Renderer",
                        window_width,
                        window_height,
                        WindowOptions::default(),
                    ).unwrap();
                }
            }
            ScreenState::Success => {
                framebuffer.clear();  // Limpia el framebuffer correctamente

                // Cargar y redimensionar la imagen de éxito para que coincida con el framebuffer
                let img = image::open("C:/Users/irvin/UVG/Sexto_Semestre/Graficas/Labyrinth/success.png")
                    .expect("Failed to load image")
                    .resize_exact(window_width as u32, window_height as u32, FilterType::Lanczos3);

                let (img_width, img_height) = img.dimensions();

                // Dibujar la imagen en el framebuffer
                for (x, y, pixel) in img.pixels() {
                    let rgba = pixel.0;
                    let color = ((rgba[0] as u32) << 16) | ((rgba[1] as u32) << 8) | (rgba[2] as u32);
                    framebuffer.set_pixel(x as isize, y as isize, color);
                }

                window
                    .update_with_buffer(&framebuffer.buffer.iter().map(|color| {
                        ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
                    }).collect::<Vec<u32>>(), window_width, window_height)
                    .unwrap();

                // Pausa breve antes de regresar al menú
                std::thread::sleep(Duration::from_secs(2));

                // Re-inicializa el framebuffer y la ventana antes de regresar al menú
                framebuffer = Framebuffer::new(window_width, window_height);
                window = Window::new(
                    "Maze Renderer",
                    window_width,
                    window_height,
                    WindowOptions::default(),
                ).unwrap();

                current_screen = ScreenState::Menu;
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
