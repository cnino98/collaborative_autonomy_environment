use bevy::{
    prelude::*,
    window::WindowResolution,
    picking::prelude::MeshPickingPlugin,
};

mod constants;
mod model;
mod setup;
mod sim;
mod ui;

use constants::WORLD_BACKGROUND_COLOR;
use model::BehaviorChangeMessage;
use setup::{spawn_camera, spawn_simulation_entities};
use sim::{
    advance_agent_simulation,
    advance_target_simulation,
    apply_behavior_changes,
};
use ui::{
    handle_behavior_selection_buttons,
    spawn_ui,
    sync_world_positions_to_transforms,
    update_active_behavior_status_text,
};

fn main() {
    App::new()
        .add_plugins(
            (
                DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(3024, 1964),
                ..default()
            }),
            ..default()
        }), MeshPickingPlugin::default())
    )
        .add_message::<BehaviorChangeMessage>()
        .insert_resource(ClearColor(WORLD_BACKGROUND_COLOR))
        .add_systems(
            Startup,
            (
                spawn_camera,
                spawn_simulation_entities,
                spawn_ui,
            )
                .chain(),
        )
        .add_systems(
            FixedUpdate,
            (
                apply_behavior_changes,
                advance_agent_simulation,
                advance_target_simulation,
            )
                .chain(),
        )
        .add_systems(Update, handle_behavior_selection_buttons)
        .add_systems(
            RunFixedMainLoop,
            (
                sync_world_positions_to_transforms,
                update_active_behavior_status_text,
            )
                .in_set(RunFixedMainLoopSystems::AfterFixedMainLoop),
        )
        .run();
}
