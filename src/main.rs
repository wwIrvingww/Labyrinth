mod framebuffer;
mod renderer;
mod player;
mod castray;
mod maze;

use framebuffer::Framebuffer;
use player::Player;
use renderer::render;
use castray::cast_ray;
use nalgebra::Vector2;
use std::f32::consts::PI;
use minifb::{Window, WindowOptions};

fn find_player_start(maze: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for (row_idx, row) in maze.iter().enumerate() {
        for (col_idx, &cell) in row.iter().enumerate() {
            if cell == 'p' {
                return Some((row_idx, col_idx));
            }
        }
    }
    None
}

fn main() {
    let window_width = 800;
    let window_height = 800;

    let framebuffer_width = window_width;
    let framebuffer_height = window_height;

    let frame_delay = std::time::Duration::from_millis(16);

    let mut framebuffer = Framebuffer::new(framebuffer_width, framebuffer_height);

    let mut window = Window::new(
        "Maze Renderer",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap();

    // Cargar el laberinto
    let maze = maze::reader::load_maze("maze.txt");
    let block_size = framebuffer_width.min(framebuffer_height) / 80;

    // Encontrar la posición inicial del jugador
    let (start_row, start_col) = find_player_start(&maze).expect("No se encontró el jugador en el mapa");

    // Inicializar el jugador
    let player = Player {
        pos: Vector2::new(
            (start_col as f32 + 0.5) * block_size as f32,
            (start_row as f32 + 0.5) * block_size as f32
        ),
        a: 0.0, // 0 grados, inicial
    };

    while window.is_open() {
        // Limpiar el framebuffer
        framebuffer.clear();

        // Renderizar el laberinto
        render(&mut framebuffer);

        // Lanzar rayos desde el jugador en varias direcciones
        for i in 0..360 {
            let angle = (i as f32).to_radians();
            cast_ray(&mut framebuffer, &maze, &player, angle, block_size);
        }

        // Actualizar la ventana con el contenido del framebuffer
        window
            .update_with_buffer(
                &framebuffer.buffer.iter().map(|color| {
                    ((color.r as u32) << 16) | ((color.g as u32) << 8) | (color.b as u32)
                }).collect::<Vec<u32>>(),
                framebuffer.width,
                framebuffer.height,
            ).unwrap();

        // Esperar un poco para mantener una tasa de frames constante
        std::thread::sleep(frame_delay);
    }
}
