use minifb::{Key, Window};
use crate::player::Player;

pub fn handle_player_movement(player: &mut Player, window: &Window, maze: &Vec<Vec<char>>, block_size: usize) {
    let movement_speed = 1.0;
    let rotation_speed = 0.1;

    // Calcular nuevas posiciones potenciales
    let mut new_x = player.pos.x;
    let mut new_y = player.pos.y;

    if window.is_key_down(Key::W) {
        new_x += player.a.cos() * movement_speed;
        new_y += player.a.sin() * movement_speed;
    }
    if window.is_key_down(Key::S) {
        new_x -= player.a.cos() * movement_speed;
        new_y -= player.a.sin() * movement_speed;
    }
    if window.is_key_down(Key::A) {
        player.a -= rotation_speed;
    }
    if window.is_key_down(Key::D) {
        player.a += rotation_speed;
    }

    // Verificar colisiones
    let maze_x = (new_x / block_size as f32) as usize;
    let maze_y = (new_y / block_size as f32) as usize;

    if maze[maze_y][maze_x] != '+' && maze[maze_y][maze_x] != '|' && maze[maze_y][maze_x] != '-' {
        player.pos.x = new_x;
        player.pos.y = new_y;
    }
}
