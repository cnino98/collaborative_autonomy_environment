use bevy::prelude::*;

use crate::constants::{MOVE_AWAY_SAFE_DISTANCE, MOVE_TO_ARRIVAL_TOLERANCE};

use super::GuidanceCommand;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum Behavior {
    Idle,
    MoveTo { destination: Vec2 },
    MoveAwayFrom { threat_location: Vec2 },
}

pub enum BehaviorCompletionStatus {
    InProgress,
    Complete,
}

pub fn evaluate_behavior_completion(
    behavior: Behavior,
    agent_world_position: Vec2,
) -> BehaviorCompletionStatus {
    match behavior {
        Behavior::Idle => BehaviorCompletionStatus::InProgress,
        Behavior::MoveTo { destination } => {
            let distance_to_destination = agent_world_position.distance(destination);
            if distance_to_destination < MOVE_TO_ARRIVAL_TOLERANCE {
                BehaviorCompletionStatus::Complete
            } else {
                BehaviorCompletionStatus::InProgress
            }
        }
        Behavior::MoveAwayFrom { threat_location } => {
            let distance_from_threat = agent_world_position.distance(threat_location);
            if distance_from_threat > MOVE_AWAY_SAFE_DISTANCE {
                BehaviorCompletionStatus::Complete
            } else {
                BehaviorCompletionStatus::InProgress
            }
        }
    }
}

pub fn transition_behavior(
    current_behavior: Behavior,
    behavior_completion: BehaviorCompletionStatus,
) -> Behavior {
    match current_behavior {
        Behavior::Idle => Behavior::Idle,
        Behavior::MoveTo { destination } => match behavior_completion {
            BehaviorCompletionStatus::InProgress => Behavior::MoveTo { destination },
            BehaviorCompletionStatus::Complete => Behavior::Idle,
        },
        Behavior::MoveAwayFrom { threat_location } => match behavior_completion {
            BehaviorCompletionStatus::InProgress => Behavior::MoveAwayFrom { threat_location },
            BehaviorCompletionStatus::Complete => Behavior::Idle,
        },
    }
}

pub fn guidance_command_for_behavior(behavior: Behavior) -> GuidanceCommand {
    match behavior {
        Behavior::Idle => GuidanceCommand::StayStill,
        Behavior::MoveTo { destination } => GuidanceCommand::MoveToward { destination },
        Behavior::MoveAwayFrom { threat_location } => {
            GuidanceCommand::MoveAwayFrom { threat_location }
        }
    }
}
