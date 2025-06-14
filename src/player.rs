use crate::bounding::GroundCollision;
use crate::movement_system::{BoundingBox, GravityAffected, Physics, Velocity};
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
pub struct Player {
    pub jump_system: JumpSystem,
}

pub struct JumpSystem {
    pub max_jumps: u8,
    pub jumps_remaining: u8,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, jump_system)
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
        .insert(Player {
            jump_system: JumpSystem {
                max_jumps: 2,
                jumps_remaining: 2,
            },
        })
        .insert(GravityAffected)
        .insert(Physics {
            bounding_box: BoundingBox::Cube(Vec3::new(1.0, 1.0, 1.0)),
        })
        .insert(Velocity(Vec3::new(0., 0., 0.)));
}

fn player_keyboard_event_system(
    kb: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Velocity, &mut Transform, &mut Player), With<Player>>,
) {
    if let Ok((mut velocity, mut transform, mut player)) = query.get_single_mut() {
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

        if kb.just_pressed(KeyCode::Space) && player.jump_system.jumps_remaining > 0 {
            player.jump_system.jumps_remaining -= 1;
            velocity.0.y = 5.;
        }
    }
}

fn jump_system(mut query: Query<(&mut Player, &Physics, &Transform)>) {
    for (mut player, physics, transform) in &mut query {
        if (physics.bounding_box.object_is_grounded(transform)) {
            player.jump_system.jumps_remaining = player.jump_system.max_jumps;
        }
    }
}
