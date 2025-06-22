use crate::keybinds::KeyBinds;
use bevy::app::{App, Plugin, Startup};
use bevy::prelude::{ButtonInput, KeyCode, MouseButton, Query, Res, Update, With};
use bevy::window::{CursorGrabMode, PrimaryWindow, Window};

pub struct CursorControl;

impl Plugin for CursorControl {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, grab_cursor_at_startup_system)
            .add_systems(Update, control_cursor_grab_system);
    }
}

fn grab_cursor_at_startup_system(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut primary_window = q_windows.single_mut();
    grab_cursor(&mut primary_window);
}

fn control_cursor_grab_system(
    kb: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    keybinds: Res<KeyBinds>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if kb.just_pressed(keybinds.exit) {
        release_cursor(&mut window_query.single_mut())
    }
    if mouse.just_pressed(keybinds.primary_click) {
        grab_cursor(&mut window_query.single_mut())
    }
}

fn grab_cursor(window: &mut Window) {
    window.cursor_options.grab_mode = CursorGrabMode::Locked;
    window.cursor_options.visible = false;
}

fn release_cursor(window: &mut Window) {
    window.cursor_options.grab_mode = CursorGrabMode::None;
    window.cursor_options.visible = true;
}
