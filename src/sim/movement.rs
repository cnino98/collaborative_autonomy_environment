use bevy::prelude::*;

use crate::{
    constants::{AGENT_MAX_SPEED, VELOCITY_COMMAND_GAIN},
    model::GuidanceCommand,
};

pub fn velocity_command_for_guidance(
    guidance_command: GuidanceCommand,
    agent_world_position: Vec2,
) -> Vec2 {
    match guidance_command {
        GuidanceCommand::StayStill => Vec2::ZERO,
        GuidanceCommand::MoveToward { destination } => {
            let position_delta_to_destination = destination - agent_world_position;
            let desired_velocity = VELOCITY_COMMAND_GAIN * position_delta_to_destination;
            desired_velocity.clamp_length_max(AGENT_MAX_SPEED)
        }
        GuidanceCommand::MoveAwayFrom { threat_location } => {
            let position_delta_away_from_threat = agent_world_position - threat_location;
            let desired_velocity = VELOCITY_COMMAND_GAIN * position_delta_away_from_threat;
            desired_velocity.clamp_length_max(AGENT_MAX_SPEED)
        }
    }
}

pub fn integrate_world_position(
    current_world_position: Vec2,
    commanded_velocity: Vec2,
    delta_seconds: f32,
) -> Vec2 {
    current_world_position + commanded_velocity * delta_seconds
}
