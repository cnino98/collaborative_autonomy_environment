use bevy::{
    picking::prelude::MeshPickingPlugin,
    prelude::*,
    window::WindowResolution,
};

mod agent;
mod constants;
mod input;
mod motion;
mod presentation;
mod setup;
mod target;
mod ui;
mod world;

use agent::{advance_agent_simulation, apply_behavior_changes, BehaviorChangeMessage};
use constants::WORLD_BACKGROUND_COLOR;
use input::handle_behavior_selection_buttons;
use presentation::sync_world_positions_to_transforms;
use setup::{spawn_camera, spawn_simulation_entities};
use target::advance_target_simulation;
use ui::{spawn_ui, update_active_behavior_status_text};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(3024, 1964),
                    ..default()
                }),
                ..default()
            }),
            MeshPickingPlugin::default(),
        ))
        .add_message::<BehaviorChangeMessage>()
        .insert_resource(ClearColor(WORLD_BACKGROUND_COLOR))
        .add_systems(
            Startup,
            (spawn_camera, spawn_simulation_entities, spawn_ui).chain(),
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
