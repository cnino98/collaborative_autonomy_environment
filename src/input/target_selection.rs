use bevy::prelude::*;

use crate::world::Target;

#[derive(Component)]
pub struct SelectedTarget;

pub fn handle_cursor_target_selection(
    click: On<Pointer<Click>>,
    targets: Query<(), With<Target>>,
    old_target_entity: Single<Entity, (With<SelectedTarget>, With<Target>)>,
    mut commands: Commands,
) {
    let clicked_entity = click.original_event_target();

    if targets.get(clicked_entity).is_err() {
        return;
    }

    if clicked_entity == *old_target_entity {
        return;
    }

    commands.entity(*old_target_entity).remove::<SelectedTarget>();
    commands.entity(clicked_entity).insert(SelectedTarget);
}
