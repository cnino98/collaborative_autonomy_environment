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
        .add_systems(FixedUpdate, (
            update_kinematics,
            sync_visual_transforms)
        .chain(),)
        .run();
}

// Components
#[derive(Component)]
struct Agent;

#[derive(Component)]
struct Target;

#[derive(Component)]
struct Kinematics {
    position: Vec2,
    velocity: Vec2,
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
        Transform::from_translation(AGENT_INITIAL_POSITION)
            .with_scale(Vec2::splat(ENTITY_DIAMETER).extend(1.0)),
        Agent,
        Kinematics {
            position: AGENT_INITIAL_POSITION.truncate(),
            velocity: Vec2::ZERO,
        },
    ));

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(TARGET_COLOR)),
        Transform::from_translation(TARGET_INITIAL_POSITION)
            .with_scale(Vec2::splat(ENTITY_DIAMETER).extend(1.0)),
        Target,
        Kinematics {
            position: TARGET_INITIAL_POSITION.truncate(),
            velocity: Vec2::ZERO,
        },
    ));
}

// Simulation
fn advance_position(position: Vec2, velocity: Vec2, delta_seconds: f32) -> Vec2 {
    position + velocity * delta_seconds
}

fn update_kinematics(
    mut target_kinematics: Single<&mut Kinematics, (With<Target>, Without<Agent>)>,
    mut agent_kinematics: Single<&mut Kinematics, (With<Agent>, Without<Target>)>,
    time: Res<Time>,
) {
    let delta_seconds = time.delta_secs();
    update_target_kinematics(&mut target_kinematics, delta_seconds);
    update_agent_kinematics(&mut agent_kinematics, &target_kinematics, delta_seconds);
}

fn update_target_kinematics(target_kinematics: &mut Kinematics, delta_seconds: f32) {
    target_kinematics.velocity = compute_target_velocity();
    target_kinematics.position = advance_position(
        target_kinematics.position,
        target_kinematics.velocity,
        delta_seconds,
    );
}

fn update_agent_kinematics(
    agent_kinematics: &mut Kinematics,
    target_kinematics: &Kinematics,
    delta_seconds: f32,
) {
    agent_kinematics.velocity =
        compute_agent_velocity(agent_kinematics.position, target_kinematics.position);

    agent_kinematics.position = advance_position(
        agent_kinematics.position,
        agent_kinematics.velocity,
        delta_seconds,
    );
}

fn compute_target_velocity() -> Vec2 {
    Vec2::ZERO
}

fn compute_agent_velocity(agent_position: Vec2, target_position: Vec2) -> Vec2 {
    let tracking_error = target_position - agent_position;
    let control_output = PROPORTIONAL_GAIN * tracking_error;
    control_output.clamp_length_max(MAX_AGENT_SPEED)
}

// Render
fn sync_visual_transforms(
    target_kinematics: Single<&Kinematics, (With<Target>, Without<Agent>)>,
    agent_kinematics: Single<&Kinematics, (With<Agent>, Without<Target>)>,
    mut target_transform: Single<&mut Transform, (With<Target>, Without<Agent>)>,
    mut agent_transform: Single<&mut Transform, (With<Agent>, Without<Target>)>,
){
    sync_transform_with_kinematics(&mut target_transform, &target_kinematics);
    sync_transform_with_kinematics(&mut agent_transform, &agent_kinematics);
}

fn sync_transform_with_kinematics(transform: &mut Transform, kinematics: &Kinematics) {
    transform.translation = kinematics.position.extend(transform.translation.z);
}