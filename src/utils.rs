use bevy::math::Quat;
use bevy::prelude::Window;
use bevy::window::CursorGrabMode;

pub fn is_window_grabbed(window: &Window) -> bool {
    window.cursor_options.grab_mode == CursorGrabMode::Locked
}
