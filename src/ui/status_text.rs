use bevy::prelude::*;

use crate::model::{Behavior, BehaviorStatusText, Agent, WorldPosition};

pub fn sync_world_positions_to_transforms(
    mut positioned_entities: Query<(&WorldPosition, &mut Transform)>,
) {
    for (world_position, mut transform) in &mut positioned_entities {
        transform.translation =
            world_position.coordinates.extend(transform.translation.z);
    }
}

pub fn update_active_behavior_status_text(
    mut status_text: Single<&mut Text, With<BehaviorStatusText>>,
    agent_behavior: Single<&Behavior, With<Agent>>,
) {
    let current_behavior = *agent_behavior;

    status_text.0 = "Active Behavior: ".to_string();

    match current_behavior {
        Behavior::Idle => status_text.push_str("Idle"),
        Behavior::MoveTo { destination } => {
            status_text.push_str(&format!(
                "MoveTo ({:.1}, {:.1})",
                destination.x, destination.y
            ));
        }
        Behavior::MoveAwayFrom { threat_location } => {
            status_text.push_str(&format!(
                "MoveAwayFrom ({:.1}, {:.1})",
                threat_location.x, threat_location.y
            ));
        }
    }
}
