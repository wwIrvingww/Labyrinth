use minifb::{Key, Window};

pub struct Camera {
    pub position: (f32, f32),
    pub angle: f32,
    pub speed: f32,
    pub rotation_speed: f32,
    pub last_mouse_x: Option<f32>,
    pub use_keyboard_rotation: bool,
}

impl Camera {
    pub fn new(position: (f32, f32), angle: f32, speed: f32, rotation_speed: f32) -> Self {
        Camera {
            position,
            angle,
            speed,
            rotation_speed,
            last_mouse_x: None,
            use_keyboard_rotation: false,
        }
    }

    pub fn update(&mut self, window: &Window) {
        self.use_keyboard_rotation = false;

        if window.is_key_down(Key::W) {
            self.move_forward();
        }
        if window.is_key_down(Key::S) {
            self.move_backward();
        }
        if window.is_key_down(Key::A) {
            self.rotate_left();
            self.use_keyboard_rotation = true;
        }
        if window.is_key_down(Key::D) {
            self.rotate_right();
            self.use_keyboard_rotation = true;
        }

        // Mouse movement logic
        if !self.use_keyboard_rotation {
            if let Some((mouse_x, _)) = window.get_mouse_pos(minifb::MouseMode::Pass) {
                if let Some(last_x) = self.last_mouse_x {
                    let mouse_delta = mouse_x - last_x;
                    self.angle += mouse_delta * self.rotation_speed;
                    println!("Mouse moved: delta_x = {}, new angle = {}", mouse_delta, self.angle);
                } else {
                    println!("Initial mouse position: {}", mouse_x);
                }
                self.last_mouse_x = Some(mouse_x);
            } else {
                self.last_mouse_x = None;
            }
        }
    }

    fn move_forward(&mut self) {
        self.position.0 += self.angle.cos() * self.speed;
        self.position.1 += self.angle.sin() * self.speed;
    }

    fn move_backward(&mut self) {
        self.position.0 -= self.angle.cos() * self.speed;
        self.position.1 -= self.angle.sin() * self.speed;
    }

    fn rotate_left(&mut self) {
        self.angle -= self.rotation_speed;
    }

    fn rotate_right(&mut self) {
        self.angle += self.rotation_speed;
    }
}
