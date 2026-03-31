use bevy::prelude::*;

use crate::{motion::integrate_world_position, world::{Agent, LinearVelocity, WorldPosition}};

use super::{
    evaluate_behavior_completion, guidance_command_for_behavior, transition_behavior,
    velocity_command_for_guidance, Behavior,
};

pub fn advance_agent_simulation(
    mut agents: Query<
        (&mut WorldPosition, &mut LinearVelocity, &mut Behavior),
        With<Agent>,
    >,
    time: Res<Time>,
) {
    for (mut world_position, mut linear_velocity, mut current_behavior) in &mut agents {
        let behavior_completion =
            evaluate_behavior_completion(*current_behavior, world_position.coordinates);
        let next_behavior = transition_behavior(*current_behavior, behavior_completion);
        let guidance_command = guidance_command_for_behavior(next_behavior);
        let commanded_velocity =
            velocity_command_for_guidance(guidance_command, world_position.coordinates);
        let next_world_position = integrate_world_position(
            world_position.coordinates,
            commanded_velocity,
            time.delta_secs(),
        );

        *current_behavior = next_behavior;
        linear_velocity.units_per_second = commanded_velocity;
        world_position.coordinates = next_world_position;
    }
}
