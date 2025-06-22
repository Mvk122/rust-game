use crate::bounding::GroundCollision;
use crate::keybinds::KeyBinds;
use crate::movement_system::{BoundingBox, GravityAffected, Physics, Velocity};
use crate::utils::is_window_grabbed;
use bevy::app::{App, Plugin};
use bevy::asset::Assets;
use bevy::color::Color;
use bevy::input::mouse::MouseMotion;
use bevy::input::ButtonInput;
use bevy::math::{Isometry3d, Quat, Vec2, Vec3};
use bevy::pbr::{MeshMaterial3d, StandardMaterial};
use bevy::prelude::{
    Camera3d, Commands, Component, Cuboid, EventReader, IntoSystemConfigs, KeyCode, Mesh, Mesh3d,
    Query, Res, ResMut, Startup, Transform, Update, Window, With, Without,
};
use bevy::window::PrimaryWindow;
use std::f32::consts::{FRAC_PI_2, TAU};

pub struct PlayerPlugin;

#[derive(Component)]
pub struct Player {
    pub jump_system: JumpSystem,
}

#[derive(Component)]
pub struct PlayerCamera {
    pub lag_amount: f32,
    pub orbit_distance: f32,
    pub pitch: f32,
    pub yaw: f32,
}

impl PlayerCamera {
    pub fn get_orbit_translation(&self, origin: &Vec3) -> Vec3 {
        let rotation = Quat::from_rotation_x(self.pitch) * Quat::from_rotation_y(self.yaw);
        let orbit_pos = rotation * (Vec3::Z * self.orbit_distance);
        origin + orbit_pos
    }

    pub fn orbit_and_look_at_origin_transformation(&self, origin: &Vec3) -> Transform {
        Transform::from_translation(self.get_orbit_translation(&origin))
            .looking_at(*origin, Vec3::Y)
    }
}

impl Default for PlayerCamera {
    fn default() -> Self {
        PlayerCamera {
            lag_amount: 0.1,
            orbit_distance: 10.,
            pitch: (-40.0f32).to_radians(),
            yaw: 45.0f32.to_radians(),
        }
    }
}

pub struct JumpSystem {
    pub max_jumps: u8,
    pub jumps_remaining: u8,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_player, spawn_camera).chain())
            .add_systems(Update, jump_system)
            .add_systems(Update, player_keyboard_event_system)
            .add_systems(Update, camera_movement_system);
    }
}

fn spawn_camera(
    mut commands: Commands,
    player_query: Query<&Transform, (With<Player>, Without<PlayerCamera>)>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_camera: PlayerCamera = PlayerCamera::default();
        commands
            .spawn((
                Camera3d::default(),
                player_camera
                    .orbit_and_look_at_origin_transformation(&player_transform.translation),
            ))
            .insert(PlayerCamera::default())
            .insert(Velocity(Vec3::ZERO));
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
    keybinds: Res<KeyBinds>,
    mut query: Query<(&mut Velocity, &mut Transform, &mut Player), With<Player>>,
) {
    if let Ok((mut velocity, mut transform, mut player)) = query.get_single_mut() {
        velocity.0.x = if kb.pressed(keybinds.move_right) {
            1.
        } else if kb.pressed(keybinds.move_left) {
            -1.
        } else {
            0.
        };
        velocity.0.z = if kb.pressed(keybinds.move_backwards) {
            1.
        } else if kb.pressed(keybinds.move_forward) {
            -1.
        } else {
            0.
        };

        if kb.just_pressed(keybinds.jump) && player.jump_system.jumps_remaining > 0 {
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

fn adjust_camera_rotation(
    camera: &mut PlayerCamera,
    motion: Vec2,
    vertical_sensitivity: f32,
    horizontal_sensitivity: f32,
) {
    camera.yaw = camera.yaw - motion.x * horizontal_sensitivity;
    camera.pitch = (camera.pitch - motion.y * vertical_sensitivity).clamp(-FRAC_PI_2, FRAC_PI_2);
    println!("{}", camera.pitch);
}

fn camera_movement_system(
    mut mouse_motion: EventReader<MouseMotion>,
    keybinds: Res<KeyBinds>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    player_query: Query<&Transform, (With<Player>, Without<PlayerCamera>)>,
    mut camera_query: Query<
        (&mut Transform, &mut PlayerCamera),
        (With<PlayerCamera>, Without<Player>),
    >,
) {
    if let (Ok(window), Ok(player_transform), Ok((mut camera_transform, mut camera))) = (
        window_query.get_single(),
        player_query.get_single(),
        camera_query.get_single_mut(),
    ) {
        if is_window_grabbed(window) {
            let total_mouse_motion = mouse_motion
                .read()
                .fold(Vec2::ZERO, |acc, ev| acc + ev.delta);
            adjust_camera_rotation(
                camera.as_mut(),
                total_mouse_motion,
                keybinds.vertical_sensitivity,
                keybinds.horizontal_sensitivity,
            );
        }

        let target_pos = camera.get_orbit_translation(&player_transform.translation);
        camera_transform.translation = camera_transform
            .translation
            .lerp(target_pos, camera.lag_amount);
        camera_transform.look_at(player_transform.translation, Vec3::Y);
    }
}
