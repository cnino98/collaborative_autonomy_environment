// Imports
use bevy::prelude::*;

// Global constants
const ENTITY_DIAMETER: f32 = 25.0;

const AGENT_INITIAL_POSITION: Vec3 = Vec3::new(350.0, 150.0, 1.0);
const TARGET_INITIAL_POSITION: Vec3 = Vec3::new(-100.0, -100.0, 1.0);

const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const AGENT_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
const TARGET_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

const PROPORTIONAL_GAIN: f32 = 1.0;
const MAX_AGENT_SPEED: f32 = 140.0;

// Main simulation loop
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, spawn_world_entities)
        .add_systems(FixedUpdate, update_simulation)
        .add_systems(
            RunFixedMainLoop,
            update_render.in_set(RunFixedMainLoopSystems::AfterFixedMainLoop),
        )
        .run();
}

// Components
#[derive(Component)]
struct Agent;

#[derive(Component)]
struct Target;

#[derive(Component)]
struct Position {
    value: Vec2,
}

#[derive(Component)]
struct Velocity {
    value: Vec2,
}

// Setup
fn spawn_world_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::default())),
        MeshMaterial2d(materials.add(AGENT_COLOR)),
        Transform::from_translation(AGENT_INITIAL_POSITION).with_scale(Vec2::splat(ENTITY_DIAMETER).extend(1.0)),
        Agent,
        Position{value:AGENT_INITIAL_POSITION.truncate(),},
        Velocity{value:Vec2::ZERO},
    ));

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(TARGET_COLOR)),
        Transform::from_translation(TARGET_INITIAL_POSITION).with_scale(Vec2::splat(ENTITY_DIAMETER).extend(1.0)),
        Target,
        Position{value:TARGET_INITIAL_POSITION.truncate(),},
    ));
}

// Simulation
fn advance_position(position: Vec2, velocity: Vec2, delta_seconds: f32) -> Vec2 {
    position + velocity * delta_seconds
}

fn update_simulation(
    target_position: Single<&Position, (With<Target>, Without<Agent>)>,
    mut agent_position: Single<&mut Position, (With<Agent>, Without<Target>)>,
    mut agent_velocity: Single<&mut Velocity, With<Agent>>,
    time: Res<Time>,
) {
    agent_velocity.value = compute_agent_velocity(agent_position.value, target_position.value);
    agent_position.value = advance_position(agent_position.value, agent_velocity.value, time.delta_secs());
}

fn compute_agent_velocity(agent_position: Vec2, target_position: Vec2) -> Vec2 {
    let tracking_error: Vec2 = target_position - agent_position;
    let control_output: Vec2 = PROPORTIONAL_GAIN * tracking_error;
    control_output.clamp_length_max(MAX_AGENT_SPEED)
}

// Render
fn update_render(
    target_position: Single<&Position, With<Target>>,
    agent_position: Single<&Position, With<Agent>>,
    mut target_transform: Single<&mut Transform, (With<Target>, Without<Agent>)>,
    mut agent_transform: Single<&mut Transform, (With<Agent>, Without<Target>)>,
){
    update_transform(&mut target_transform, target_position.value);
    update_transform(&mut agent_transform, agent_position.value);
}

fn update_transform(transform: &mut Transform, position: Vec2) {
    transform.translation = position.extend(transform.translation.z);
}