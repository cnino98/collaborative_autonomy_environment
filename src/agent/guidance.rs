use bevy::prelude::*;

pub enum GuidanceCommand {
    StayStill,
    MoveToward { destination: Vec2 },
    MoveAwayFrom { threat_location: Vec2 },
}
