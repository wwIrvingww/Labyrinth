use minifb::Key;
use nalgebra::Vector2;

pub struct Player {
    pub pos: Vector2<f32>,
    pub a: f32, // Angle of view
    pub fov: f32, // Field of view
}

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        Player {
            pos: Vector2::new(x, y),
            a: 0.0, // Inicializamos el ángulo en 0
            fov: std::f32::consts::PI / 3.0, // Field of view inicial
        }
    }
}

pub fn process_events(window: &minifb::Window, player: &mut Player, maze: &[Vec<char>], block_size: usize) {
    let movement_speed = 2.0;
    let rotation_speed = 0.1;

    let mut new_pos = player.pos.clone();

    if window.is_key_down(Key::W) {
        new_pos.x += player.a.cos() * movement_speed;
        new_pos.y += player.a.sin() * movement_speed;
    }
    if window.is_key_down(Key::S) {
        new_pos.x -= player.a.cos() * movement_speed;
        new_pos.y -= player.a.sin() * movement_speed;
    }
    if window.is_key_down(Key::A) {
        player.a -= rotation_speed;
    }
    if window.is_key_down(Key::D) {
        player.a += rotation_speed;
    }

    // Verificar colisiones con las paredes
    let i = (new_pos.x as usize) / block_size;
    let j = (new_pos.y as usize) / block_size;

    if maze[j][i] == ' ' || maze[j][i] == 'p' || maze[j][i] == 'g' {
        player.pos = new_pos;
    }
}
