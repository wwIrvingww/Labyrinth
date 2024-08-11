use minifb::{Key, Window};
use gilrs::{Gilrs, Button, EventType, Event};

const GAMEPAD_SENSITIVITY: f32 = 5.0; // ajusta este valor según tus necesidades
const MOUSE_SENSITIVITY: f32 = 0.02; // sensibilidad para la rotación con el mouse 0.02
const KEY_SENSITIVITY: f32 = 4.0; // sensibilidad para la rotación con el mouse 0.02


pub struct Camera {
    pub position: (f32, f32),
    pub angle: f32,
    pub speed: f32,
    pub rotation_speed: f32,
    pub last_mouse_x: Option<f32>,
    pub moving_forward: bool,
    pub moving_backward: bool,
    pub rotating_left: bool,
    pub rotating_right: bool,
}

impl Camera {
    pub fn new(position: (f32, f32), angle: f32, speed: f32, rotation_speed: f32) -> Self {
        Camera {
            position,
            angle,
            speed,
            rotation_speed,
            last_mouse_x: None,
            moving_forward: false,
            moving_backward: false,
            rotating_left: false,
            rotating_right: false,
        }
    }

    pub fn update(&mut self, window: &Window, gamepad: &mut Gilrs) {
        // Procesar entrada del teclado
        if window.is_key_down(Key::W) {
            self.moving_forward = true;
        } else {
            self.moving_forward = false;
        }
        if window.is_key_down(Key::S) {
            self.moving_backward = true;
        } else {
            self.moving_backward = false;
        }
        if window.is_key_down(Key::A) {
            self.rotating_left = true;
        } else {
            self.rotating_left = false;
        }
        if window.is_key_down(Key::D) {
            self.rotating_right = true;
        } else {
            self.rotating_right = false;
        }

        // Procesar entrada del gamepad
        while let Some(Event { event, .. }) = gamepad.next_event() {
            match event {
                EventType::ButtonPressed(Button::DPadUp, _) => {
                    self.moving_forward = true;
                }
                EventType::ButtonReleased(Button::DPadUp, _) => {
                    self.moving_forward = false;
                }
                EventType::ButtonPressed(Button::DPadDown, _) => {
                    self.moving_backward = true;
                }
                EventType::ButtonReleased(Button::DPadDown, _) => {
                    self.moving_backward = false;
                }
                EventType::ButtonPressed(Button::DPadLeft, _) => {
                    self.rotating_left = true;
                }
                EventType::ButtonReleased(Button::DPadLeft, _) => {
                    self.rotating_left = false;
                }
                EventType::ButtonPressed(Button::DPadRight, _) => {
                    self.rotating_right = true;
                }
                EventType::ButtonReleased(Button::DPadRight, _) => {
                    self.rotating_right = false;
                }
                _ => {}
            }
        }

        // Procesar entrada del mouse para la rotación
        if let Some(mouse_pos) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            if let Some(last_x) = self.last_mouse_x {
                let delta_x = mouse_pos.0 - last_x;
                self.angle += delta_x * MOUSE_SENSITIVITY;
            }
            self.last_mouse_x = Some(mouse_pos.0);
        }

        // Actualizar posición y ángulo
        if self.moving_forward {
            self.move_forward();
        }
        if self.moving_backward {
            self.move_backward();
        }
        if self.rotating_left {
            self.rotate_left();
        }
        if self.rotating_right {
            self.rotate_right();
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
        self.angle -= self.rotation_speed * KEY_SENSITIVITY;
    }

    fn rotate_right(&mut self) {
        self.angle += self.rotation_speed * KEY_SENSITIVITY;
    }

    fn rotate_left_gamepad(&mut self) {
        self.angle -= self.rotation_speed * GAMEPAD_SENSITIVITY;
    }

    fn rotate_right_gamepad(&mut self) {
        self.angle += self.rotation_speed * GAMEPAD_SENSITIVITY;
    }
}


