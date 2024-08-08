use minifb::Window;
use crate::player::Player;
use crate::framebuffer::{Framebuffer, Color};

pub fn process_events(window: &Window, player: &mut Player, maze: &[Vec<char>], block_size: usize, framebuffer: &mut Framebuffer) -> bool {
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
    if maze[j][i] == 'p' {
        println!("Estás en el inicio");
        framebuffer.draw_sky(Color { r: 255, g: 0, b: 0 }); // Rojo
    } else if maze[j][i] == 'g' {
        println!("Estás en la meta");
        framebuffer.draw_sky(Color { r: 0, g: 255, b: 0 }); // Verde
        return true; // Indica que se ha llegado a la meta
    } else {
        framebuffer.draw_sky(Color { r: 135, g: 206, b: 235 }); // Color cielo por defecto (Sky Blue)
    }

    // Corregir la comparación de caracteres vacíos
    if maze[j][i] == ' ' {
        player.pos = new_pos;
    }

    false
}
