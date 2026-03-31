use bevy::prelude::*;

pub fn integrate_world_position(
    current_world_position: Vec2,
    commanded_velocity: Vec2,
    delta_seconds: f32,
) -> Vec2 {
    current_world_position + commanded_velocity * delta_seconds
}
