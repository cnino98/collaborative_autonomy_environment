use bevy::prelude::*;

#[derive(Component)]
pub struct Agent;

#[derive(Component)]
pub struct Target;

#[derive(Component)]
pub struct WorldPosition {
    pub coordinates: Vec2,
}

#[derive(Component)]
pub struct LinearVelocity {
    pub units_per_second: Vec2,
}
