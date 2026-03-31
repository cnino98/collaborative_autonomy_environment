use bevy::prelude::*;

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

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
pub enum BehaviorSelection {
    Idle,
    MoveTo,
    MoveAwayFrom,
}

pub enum GuidanceCommand {
    StayStill,
    MoveToward { destination: Vec2 },
    MoveAwayFrom { threat_location: Vec2 },
}
