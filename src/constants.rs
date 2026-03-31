use bevy::prelude::*;

pub const ENTITY_RENDER_SIZE: f32 = 25.0;

pub const AGENT_INITIAL_POSITION: Vec3 = Vec3::new(350.0, 150.0, 1.0);

pub const WORLD_BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

pub const VELOCITY_COMMAND_GAIN: f32 = 1.0;
pub const AGENT_MAX_SPEED: f32 = 140.0;
pub const MOVE_TO_ARRIVAL_TOLERANCE: f32 = ENTITY_RENDER_SIZE * 2.0;
pub const MOVE_AWAY_SAFE_DISTANCE: f32 = 500.0;

pub const DEFAULT_BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.35, 0.75, 0.35);
