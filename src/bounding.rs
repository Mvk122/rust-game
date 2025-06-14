use bevy::math::Vec3;
use bevy::prelude::Transform;

pub enum BoundingBox {
    Cube(Vec3),
}

pub trait GroundCollision {
    fn object_on_ground_y_world_coord(&self) -> f32;
    fn object_bottom_y_world_coord(&self, transform: &Transform) -> f32;
    fn object_is_grounded(&self, transform: &Transform) -> bool;
}

impl GroundCollision for BoundingBox {
    fn object_on_ground_y_world_coord(&self) -> f32 {
        match self {
            BoundingBox::Cube(vector) => vector.y / 2.,
        }
    }

    fn object_bottom_y_world_coord(&self, transform: &Transform) -> f32 {
        match self {
            BoundingBox::Cube(vector) => transform.translation.y - (vector.y / 2.),
        }
    }

    fn object_is_grounded(&self, transform: &Transform) -> bool {
        return transform.translation.y <= self.object_on_ground_y_world_coord();
    }
}
