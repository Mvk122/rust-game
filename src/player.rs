use crate::movement_system::{Velocity};
use bevy::app::{App, Plugin};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{
    Commands, Component, Cuboid, KeyCode, Mesh, Mesh3d, Query, Res, ResMut, Startup, Transform,
    Update, With,
};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_keyboard_event_system);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn((
            Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
            MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
            Transform::from_xyz(0.0, 0.5, 0.0),
        ))
        .insert(Player)
        .insert(Velocity(Vec3::new(0., 0., 0.)));
}

fn player_keyboard_event_system(
    kb: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>,
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.0.x = if kb.pressed(KeyCode::KeyD) {
            1.
        } else if kb.pressed(KeyCode::KeyA) {
            -1.
        } else {
            0.
        };
        velocity.0.z = if kb.pressed(KeyCode::KeyS) {
            1.
        } else if kb.pressed(KeyCode::KeyW) {
            -1.
        } else {
            0.
        };

        if kb.pressed(KeyCode::Space) {
            velocity.0.y += 20.;
        }
    }
}
