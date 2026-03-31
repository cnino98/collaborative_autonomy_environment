use bevy::{
    color::palettes::basic,
    prelude::*,
};

use crate::{
    constants::{AGENT_INITIAL_POSITION, ENTITY_RENDER_SIZE},
    model::{
        Agent,
        Behavior,
        LinearVelocity,
        SelectedTarget,
        Target,
        WorldPosition,
    },
};

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn spawn_simulation_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::default())),
        MeshMaterial2d(materials.add(Color::from(basic::BLUE))),
        Transform::from_translation(AGENT_INITIAL_POSITION)
            .with_scale(Vec2::splat(ENTITY_RENDER_SIZE).extend(1.0)),
        Agent,
        WorldPosition {
            coordinates: AGENT_INITIAL_POSITION.truncate(),
        },
        LinearVelocity {
            units_per_second: Vec2::ZERO,
        },
        Behavior::Idle,
    ));

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::from(basic::BLACK))),
        Transform::from_translation(Vec3::new(-100.0, -100.0, 1.0))
            .with_scale(Vec2::splat(ENTITY_RENDER_SIZE).extend(1.0)),
        Target,
        WorldPosition {
            coordinates: Vec2::new(-100.0, -100.0),
        },
        LinearVelocity {
            units_per_second: Vec2::ZERO,
        },
    ));

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::from(basic::RED))),
        Transform::from_translation(Vec3::new(0.0, 0.0, 1.0))
            .with_scale(Vec2::splat(ENTITY_RENDER_SIZE).extend(1.0)),
        Target,
        SelectedTarget,
        WorldPosition {
            coordinates: Vec2::new(0.0, 0.0),
        },
    ));
}
