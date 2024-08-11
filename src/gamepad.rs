use gilrs::{Gilrs, Button, Event, EventType, Axis};

#[derive(Clone)] // Para permitir la clonación
pub enum GamepadInput {
    Enter,
    ToggleMode,
    MoveLeft,
    MoveRight,
    MoveForward,
    MoveBackward,
}

pub struct Gamepad {
    pub gilrs: Gilrs,
}

impl Gamepad {
    pub fn new() -> Self {
        let gilrs = Gilrs::new().unwrap();
        Gamepad { gilrs }
    }

    pub fn gilrs_mut(&mut self) -> &mut Gilrs {
        &mut self.gilrs
    }

    pub fn update(&mut self) -> Option<GamepadInput> {
        while let Some(Event { event, .. }) = self.gilrs.next_event() {
            match event {
                EventType::ButtonPressed(button, _) => match button {
                    Button::South => {
                        println!("Button South pressed (Enter)");
                        return Some(GamepadInput::Enter);
                    }
                    Button::East => {
                        println!("Button East pressed (Enter)");
                        return Some(GamepadInput::Enter);
                    }
                    Button::West => {
                        println!("Button West pressed (ToggleMode)");
                        return Some(GamepadInput::ToggleMode);
                    }
                    Button::C => {
                        println!("Button C pressed (ToggleMode)");
                        return Some(GamepadInput::ToggleMode);
                    }
                    Button::LeftTrigger => {
                        println!("Left Trigger pressed (MoveLeft)");
                        return Some(GamepadInput::MoveLeft);
                    }
                    Button::RightTrigger => {
                        println!("Right Trigger pressed (MoveRight)");
                        return Some(GamepadInput::MoveRight);
                    }
                    _ => {
                        println!("Unhandled button pressed: {:?}", button);
                    }
                },
                EventType::AxisChanged(axis, value, _) => {
                    if axis == Axis::LeftStickY {
                        if value > 0.1 {
                            println!("Left Stick moved down (MoveBackward)");
                            return Some(GamepadInput::MoveBackward);
                        } else if value < -0.1 {
                            println!("Left Stick moved up (MoveForward)");
                            return Some(GamepadInput::MoveForward);
                        }
                    } else if axis == Axis::LeftStickX {
                        if value > 0.1 {
                            println!("Left Stick moved right (MoveRight)");
                            return Some(GamepadInput::MoveRight);
                        } else if value < -0.1 {
                            println!("Left Stick moved left (MoveLeft)");
                            return Some(GamepadInput::MoveLeft);
                        }
                    } else {
                        println!("Unhandled axis movement: {:?} with value {}", axis, value);
                    }
                }
                _ => (),
            }
        }
        None
    }
}
