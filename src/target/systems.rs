use bevy::prelude::*;

use crate::{
    motion::integrate_world_position,
    world::{Agent, LinearVelocity, Target, WorldPosition},
};

pub fn advance_target_simulation(
    mut targets: Query<
        (&mut WorldPosition, &mut LinearVelocity),
        (With<Target>, Without<Agent>),
    >,
    time: Res<Time>,
) {
    for (mut world_position, mut linear_velocity) in &mut targets {
        let commanded_velocity = target_dynamics(world_position.coordinates);
        let next_world_position = integrate_world_position(
            world_position.coordinates,
            commanded_velocity,
            time.delta_secs(),
        );

        linear_velocity.units_per_second = commanded_velocity;
        world_position.coordinates = next_world_position;
    }
}

fn target_dynamics(position: Vec2) -> Vec2 {
    Vec2::new(-position.y, position.x)
}
