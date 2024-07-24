use nalgebra::Vector2;

pub struct Player {
    pub pos: Vector2<f32>,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Player {
        Player {
            pos: Vector2::new(x, y),
        }
    }
}
