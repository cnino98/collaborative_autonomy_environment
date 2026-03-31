use bevy::prelude::*;

use crate::model::Behavior;

#[derive(Message, Clone, Copy, Debug)]
pub struct BehaviorChangeMessage {
    pub agent_entity: Entity,
    pub behavior_change: BehaviorChange,
}

#[derive(Clone, Copy, Debug)]
pub enum BehaviorChange {
    SetBehavior(Behavior),
}
