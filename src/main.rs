mod bounding;
mod constants;
mod cursor_control;
mod keybinds;
mod movement_system;
mod player;
mod utils;

use crate::cursor_control::CursorControl;
use crate::keybinds::KeyBinds;
use crate::movement_system::MovementSystemPlugin;
use crate::player::PlayerPlugin;
use bevy::prelude::*;

const BASE_SPEED: f32 = 500.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<KeyBinds>()
        .add_plugins(MovementSystemPlugin)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
        .add_plugins(CursorControl)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(20.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}
