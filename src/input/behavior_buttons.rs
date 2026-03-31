use bevy::prelude::*;

use crate::{
    agent::{Behavior, BehaviorChange, BehaviorChangeMessage},
    constants::{DEFAULT_BUTTON_COLOR, HOVERED_BUTTON_COLOR, PRESSED_BUTTON_COLOR},
    world::{Agent, Target, WorldPosition},
};

use super::SelectedTarget;

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum BehaviorSelection {
    Idle,
    MoveTo,
    MoveAwayFrom,
}

pub fn handle_behavior_selection_buttons(
    mut button_interactions: Query<
        (&Interaction, &BehaviorSelection, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    agent_entity: Single<Entity, With<Agent>>,
    selected_target_world_position: Single<&WorldPosition, (With<Target>, With<SelectedTarget>)>,
    mut behavior_change_writer: MessageWriter<BehaviorChangeMessage>,
) {
    for (interaction_state, behavior_selection, mut button_background_color) in
        &mut button_interactions
    {
        match *interaction_state {
            Interaction::Pressed => {
                *button_background_color = PRESSED_BUTTON_COLOR.into();

                let requested_behavior = behavior_from_selection(
                    *behavior_selection,
                    selected_target_world_position.coordinates,
                );

                behavior_change_writer.write(BehaviorChangeMessage {
                    agent_entity: *agent_entity,
                    behavior_change: BehaviorChange::SetBehavior(requested_behavior),
                });
            }
            Interaction::Hovered => {
                *button_background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *button_background_color = DEFAULT_BUTTON_COLOR.into();
            }
        }
    }
}

fn behavior_from_selection(
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
