pub mod panel;
pub mod status_text;

pub use status_text::*;

use bevy::prelude::*;

pub fn spawn_ui(mut commands: Commands) {
    status_text::spawn_behavior_status_text(&mut commands);
    panel::spawn_behavior_panel(&mut commands);
}
