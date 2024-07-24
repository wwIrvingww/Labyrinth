use crate::framebuffer::Framebuffer;
use crate::player::Player;
use nalgebra::Vector2;

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &[Vec<char>],
    player: &Player,
    a: f32,
    block_size: usize,
) {
    let mut d = 0.0;

    framebuffer.set_current_color(0xFFDDDD);

    println!("Cast ray from ({}, {}) with angle {}", player.pos.x, player.pos.y, a);

    loop {
        let cos = d * a.cos();
        let sin = d * a.sin();
        let x = (player.pos.x + cos) as usize;
        let y = (player.pos.y + sin) as usize;

        // Convertir nuestras coordenadas en píxeles a índices en el array del laberinto
        let i = x / block_size;
        let j = y / block_size;

        // Si el elemento actual es una pared, rompemos el bucle
        if maze[j][i] != ' ' {
            println!("Hit wall at ({}, {})", x, y);
            return;
        }

        // Verificar si las coordenadas están dentro de los límites de la pantalla
        if x >= 0 && y >= 0 && x < framebuffer.width && y < framebuffer.height {
            println!("Drawing point at ({}, {})", x, y);
            framebuffer.point(x as isize, y as isize);
        }

        d += 0.1;
    }
}