use crate::constants::{GAME_SPEED, GRAVITY};
use bevy::app::{App, Plugin};
use bevy::math::Vec3;
use bevy::prelude::{Component, Query, Res, Time, Transform, Update, With};

pub struct MovementSystemPlugin;

// Velocity component needs to be added to an entity for it to have movement applied to it.
#[derive(Component)]
pub struct Velocity(pub(crate) Vec3);

#[derive(Component)]
pub struct GravityAffected;

pub enum BoundingBox {
    Cube(Vec3),
}

#[derive(Component)]
pub struct Physics {
    pub(crate) bounding_box: BoundingBox,
}

impl Plugin for MovementSystemPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, apply_gravity);
        app.add_systems(Update, apply_velocity);
    }
}

fn apply_gravity(
    mut query: Query<(&mut Velocity, &mut Transform, &Physics), With<GravityAffected>>,
    time: Res<Time>,
) {
    let delta = time.delta_secs();

    for (mut velocity, mut transform, physics) in &mut query {
        let bottom_y_level = object_bottom_y_world_coord(&physics.bounding_box, &transform);

        if bottom_y_level > 0. {
            velocity.0 += Vec3::new(0.0, -GRAVITY * delta * GAME_SPEED, 0.0);
        }

        if bottom_y_level < 0. {
            transform.translation.y = object_on_ground_y_world_coord(&physics.bounding_box);
            velocity.0.y = 0.
        }
    }
}

fn apply_velocity(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    let delta = time.delta_secs();

    for (velocity, mut transform) in &mut query {
        transform.translation += velocity.0 * delta * GAME_SPEED;
    }
}

fn object_bottom_y_world_coord(bounding_box: &BoundingBox, transform: &Transform) -> f32 {
    match bounding_box {
        BoundingBox::Cube(vector) => transform.translation.y - (vector.y / 2.),
    }
}

fn object_on_ground_y_world_coord(bounding_box: &BoundingBox) -> f32 {
    match bounding_box {
        BoundingBox::Cube(vector) => vector.y / 2.,
    }
}
