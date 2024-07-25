use minifb::Window;
use crate::player::Player;
use nalgebra::Vector2;

pub fn process_events(window: &Window, player: &mut Player, maze: &[Vec<char>], block_size: usize) {
    let mut new_pos = player.pos.clone();
    let speed = 2.0;

    if window.is_key_down(minifb::Key::W) {
        new_pos.x += player.a.cos() * speed;
        new_pos.y += player.a.sin() * speed;
    }
    if window.is_key_down(minifb::Key::S) {
        new_pos.x -= player.a.cos() * speed;
        new_pos.y -= player.a.sin() * speed;
    }
    if window.is_key_down(minifb::Key::A) {
        player.a -= 0.1;
    }
    if window.is_key_down(minifb::Key::D) {
        player.a += 0.1;
    }

    // Verificar colisiones
    let i = new_pos.x as usize / block_size;
    let j = new_pos.y as usize / block_size;
    if maze[j][i] == ' ' {
        player.pos = new_pos;
    }
}
