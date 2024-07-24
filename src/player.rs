use nalgebra::Vector2;

pub struct Player {
    pub pos: Vector2<f32>, // Posición del jugador en el mapa
    pub a: f32, // Ángulo de visión del jugador
}
