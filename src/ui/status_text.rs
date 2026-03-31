use bevy::prelude::*;

use crate::{agent::Behavior, world::Agent};

#[derive(Component)]
pub struct BehaviorStatusText;

pub fn spawn_behavior_status_text(commands: &mut Commands) {
    commands.spawn((
        BehaviorStatusText,
        Text::default(),
        TextColor(Color::BLACK),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
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
