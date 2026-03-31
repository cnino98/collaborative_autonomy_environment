use bevy::prelude::*;

use crate::world::WorldPosition;

pub fn sync_world_positions_to_transforms(
    mut positioned_entities: Query<(&WorldPosition, &mut Transform)>,
) {
    for (world_position, mut transform) in &mut positioned_entities {
        transform.translation = world_position.coordinates.extend(transform.translation.z);
    }
}
