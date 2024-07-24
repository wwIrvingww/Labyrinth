use nalgebra::Vector2;

pub struct Player {
    pub pos: Vector2<f32>,
    pub a: f32, // Angle of view
}

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        Player {
            pos: Vector2::new(x, y),
            a: 0.0, // Inicializamos el ángulo en 0
        }
    }
}
