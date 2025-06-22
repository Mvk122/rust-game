use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, MouseButton, Resource};

#[derive(Resource)]
pub struct KeyBinds {
    pub move_forward: KeyCode,
    pub move_left: KeyCode,
    pub move_right: KeyCode,
    pub move_backwards: KeyCode,
    pub jump: KeyCode,
    pub exit: KeyCode,
    pub primary_click: MouseButton,
    pub secondary_click: MouseButton,
    pub vertical_sensitivity: f32,
    pub horizontal_sensitivity: f32,
}

impl Default for KeyBinds {
    fn default() -> Self {
        Self {
            move_forward: KeyCode::KeyW,
            move_backwards: KeyCode::KeyS,
            move_right: KeyCode::KeyD,
            move_left: KeyCode::KeyA,
            jump: KeyCode::Space,
            exit: KeyCode::Escape,
            primary_click: MouseButton::Left,
            secondary_click: MouseButton::Right,
            vertical_sensitivity: 0.05,
            horizontal_sensitivity: 0.05,
        }
    }
}
