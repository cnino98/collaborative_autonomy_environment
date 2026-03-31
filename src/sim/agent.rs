use bevy::prelude::*;

use crate::{
    constants::{MOVE_AWAY_SAFE_DISTANCE, MOVE_TO_ARRIVAL_TOLERANCE},
    model::{
        Agent,
        Behavior,
        BehaviorChange,
        BehaviorChangeMessage,
        BehaviorCompletionStatus,
        BehaviorSelection,
        GuidanceCommand,
        LinearVelocity,
        WorldPosition,
    },
    sim::{integrate_world_position, velocity_command_for_guidance},
};

pub fn apply_behavior_changes(
    mut incoming_behavior_changes: MessageReader<BehaviorChangeMessage>,
    mut agent_behaviors: Query<&mut Behavior, With<Agent>>,
) {
    for behavior_change_message in incoming_behavior_changes.read() {
        if let Ok(mut current_behavior) =
            agent_behaviors.get_mut(behavior_change_message.agent_entity)
        {
            *current_behavior = behavior_after_applying_change(
                *current_behavior,
                behavior_change_message.behavior_change,
            );
        }
    }
}

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

pub fn behavior_from_selection(
    behavior_selection: BehaviorSelection,
    target_world_position: Vec2,
) -> Behavior {
    match behavior_selection {
        BehaviorSelection::Idle => Behavior::Idle,
        BehaviorSelection::MoveTo => Behavior::MoveTo {
            destination: target_world_position,
        },
        BehaviorSelection::MoveAwayFrom => Behavior::MoveAwayFrom {
            threat_location: target_world_position,
        },
    }
}

fn behavior_after_applying_change(
    _current_behavior: Behavior,
    behavior_change: BehaviorChange,
) -> Behavior {
    match behavior_change {
        BehaviorChange::SetBehavior(requested_behavior) => requested_behavior,
    }
}

fn evaluate_behavior_completion(
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

fn transition_behavior(
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

fn guidance_command_for_behavior(behavior: Behavior) -> GuidanceCommand {
    match behavior {
        Behavior::Idle => GuidanceCommand::StayStill,
        Behavior::MoveTo { destination } => GuidanceCommand::MoveToward { destination },
        Behavior::MoveAwayFrom { threat_location } => {
            GuidanceCommand::MoveAwayFrom { threat_location }
        }
    }
}
