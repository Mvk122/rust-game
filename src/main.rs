mod components;

use bevy::prelude::*;
use crate::components::Velocity;

const BASE_SPEED: f32 = 500.;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, movable_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Mesh3d(meshes.add(Circle::new(4.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::srgb_u8(124, 144, 255))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // light
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn movable_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
    let delta = time.delta_secs();

    for (velocity, mut transform) in &mut query {
        let translation = &mut transform.translation;
        translation.x += velocity.x * delta * BASE_SPEED;
        translation.y += velocity.y * delta * BASE_SPEED;
        translation.z += velocity.z * delta * BASE_SPEED;
    }
}

