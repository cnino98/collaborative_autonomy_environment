use bevy::prelude::*;

use crate::world::Agent;

use super::Behavior;

#[derive(Message, Clone, Copy, Debug)]
pub struct BehaviorChangeMessage {
    pub agent_entity: Entity,
    pub behavior_change: BehaviorChange,
}

#[derive(Clone, Copy, Debug)]
pub enum BehaviorChange {
    SetBehavior(Behavior),
}

pub fn apply_behavior_changes(
    mut incoming_behavior_changes: MessageReader<BehaviorChangeMessage>,
    mut agent_behaviors: Query<&mut Behavior, With<Agent>>,
) {
    for behavior_change_message in incoming_behavior_changes.read() {
        if let Ok(mut current_behavior) =
            agent_behaviors.get_mut(behavior_change_message.agent_entity)
        {
            match behavior_change_message.behavior_change {
                BehaviorChange::SetBehavior(requested_behavior) => {
                    *current_behavior = requested_behavior;
                }
            }
        }
    }
}
