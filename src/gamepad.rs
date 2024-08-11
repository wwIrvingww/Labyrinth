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
        while let Some(Event { id: _, event, time: _ }) = self.gilrs.next_event() {
            match event {
                EventType::ButtonPressed(button, _) => match button {
                    Button::South => {
                        println!("Button 1 pressed (Enter)");
                        return Some(GamepadInput::Enter);
                    }
                    Button::East => {
                        println!("Button 2 pressed (Enter)");
                        return Some(GamepadInput::Enter);
                    }
                    Button::West => {
                        println!("Button 4 pressed (M)");
                        return Some(GamepadInput::ToggleMode);
                    }
                    Button::C => {
                        println!("Button 5 pressed (M)");
                        return Some(GamepadInput::ToggleMode);
                    }
                    Button::LeftTrigger => {
                        println!("Button 7 pressed (A)");
                        return Some(GamepadInput::MoveLeft);
                    }
                    Button::RightTrigger => {
                        println!("Button 8 pressed (D)");
                        return Some(GamepadInput::MoveRight);
                    }
                    _ => {}
                },
                EventType::AxisChanged(axis, value, _) => {
                    if axis == Axis::LeftStickY {
                        if value > 0.1 {
                            println!("D-pad moved down (S)");
                            return Some(GamepadInput::MoveBackward);
                        } else if value < -0.1 {
                            println!("D-pad moved up (W)");
                            return Some(GamepadInput::MoveForward);
                        }
                    } else if axis == Axis::LeftStickX {
                        if value > 0.1 {
                            println!("D-pad moved right (D)");
                            return Some(GamepadInput::MoveRight);
                        } else if value < -0.1 {
                            println!("D-pad moved left (A)");
                            return Some(GamepadInput::MoveLeft);
                        }
                    }
                }
                _ => (),
            }
        }
        None
    }
}
