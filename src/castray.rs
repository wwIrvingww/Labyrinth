use crate::framebuffer::Framebuffer;
use crate::player::Player;
use nalgebra::Vector2;

pub fn cast_ray(
    framebuffer: &mut Framebuffer,
    maze: &Vec<Vec<char>>,
    player: &Player,
    angle: f32,
    block_size: usize,
) -> f32 {
    let mut d = 0.0;

    loop {
        let cos = d * angle.cos();
        let sin = d * angle.sin();
        let x = (player.pos.x + cos) as isize;
        let y = (player.pos.y + sin) as isize;

        // Verificar si las coordenadas están dentro de los límites del framebuffer
        if x < 0 || y < 0 || x >= framebuffer.width as isize || y >= framebuffer.height as isize {
            break;
        }

        // Convertir nuestras coordenadas en píxeles a índices en el array del laberinto
        let i = x as usize / block_size;
        let j = y as usize / block_size;

        // Verificar si estamos dentro de los límites del laberinto
        if i >= maze[0].len() || j >= maze.len() {
            break;
        }

        // Si el ítem actual es una pared, rompemos el bucle
        if maze[j][i] != ' ' {
            break;
        }

        // Pintar el punto en el framebuffer con un color que depende de la distancia
        framebuffer.set_current_color(0xFF0000); // Color rojo para el rayo
        framebuffer.point(x, y);

        d += 0.5; // Ajustar el incremento de 'd' para evitar saltos grandes
    }

    d // Devolver la distancia desde el jugador hasta la pared
}
