use bevy::prelude::{ButtonInput, KeyCode, MouseButton, Res};

pub enum GameControl {
    Up,
    Down,
    Left,
    Right,
    ZoomIn,
    ZoomOut,
    MainAttack
}

impl GameControl {
    pub fn pressed(
        &self,
        keyboard_input: &Res<ButtonInput<KeyCode>>, 
        mouse_input: &Res<ButtonInput<MouseButton>>
    ) -> bool {
        match self {
            GameControl::Up => {
                keyboard_input.pressed(KeyCode::KeyW) || keyboard_input.pressed(KeyCode::ArrowUp)
            }
            GameControl::Down => {
                keyboard_input.pressed(KeyCode::KeyS) || keyboard_input.pressed(KeyCode::ArrowDown)
            }
            GameControl::Left => {
                keyboard_input.pressed(KeyCode::KeyA) || keyboard_input.pressed(KeyCode::ArrowLeft)
            }
            GameControl::Right => {
                keyboard_input.pressed(KeyCode::KeyD) || keyboard_input.pressed(KeyCode::ArrowRight)
            }
            GameControl::ZoomIn => keyboard_input.pressed(KeyCode::KeyQ),
            GameControl::ZoomOut => keyboard_input.pressed(KeyCode::KeyE),
            GameControl::MainAttack => mouse_input.pressed(MouseButton::Left),
        }
    }
}

pub fn get_control_pressed(
    control: GameControl,
    input: &Res<ButtonInput<KeyCode>>,
    mouse_input: &Res<ButtonInput<MouseButton>>,
) -> f32 {
    if control.pressed(input, mouse_input) {
        1.0
    } else {
        0.0
    }
}
