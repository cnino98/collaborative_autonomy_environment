use bevy::prelude::*;

const ENTITY_DIAMETER: f32 = 25.0;
const TARGET_STARTING_POSITION: Vec3 = Vec3::new(-100.0, -100.0, 1.0);
const AGENT_STARTING_POSITION: Vec3 = Vec3::new(350.0, 150.0, 1.0);
const GOAL_LOCATION: Vec3 = Vec3::new(0.0, 0.0, 1.0);

const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const AGENT_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
const TARGET_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);
const GOAL_COLOR: Color = Color::srgb(0.5, 0.8, 0.5);

const F_LOWER: f32 = 0.10;
const F_UPPER: f32 = 0.30;
const K1: f32 = 0.10;
const K2: f32 = 1.20;

const TARGET_MIN_SPEED: f32 = 5.0;
const TARGET_MAX_SPEED: f32 = 90.0;
const REPULSION_DECAY_LENGTH: f32 = 120.0;

const MAX_AGENT_SPEED: f32 = 140.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (update_target, update_agent).chain())
        .run();
}

#[derive(Component)]
struct Agent;

#[derive(Component)]
struct Target;

#[derive(Component)]
struct Goal;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(TARGET_COLOR)),
        Transform::from_translation(TARGET_STARTING_POSITION)
            .with_scale(Vec2::splat(ENTITY_DIAMETER).extend(1.0)),
        Target,
    ));

    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::default())),
        MeshMaterial2d(materials.add(AGENT_COLOR)),
        Transform::from_translation(AGENT_STARTING_POSITION)
            .with_scale(Vec2::splat(ENTITY_DIAMETER).extend(1.0)),
        Agent,
    ));

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(GOAL_COLOR)),
        Transform::from_translation(GOAL_LOCATION)
            .with_scale(Vec2::splat(ENTITY_DIAMETER).extend(1.0)),
        Goal,
    ));
}

fn target_flee_speed(distance: f32) -> f32 {
    TARGET_MIN_SPEED
        + (TARGET_MAX_SPEED - TARGET_MIN_SPEED) * (-distance / REPULSION_DECAY_LENGTH).exp()
}

fn update_target(
    mut target_transform: Single<&mut Transform, (With<Target>, Without<Agent>, Without<Goal>)>,
    agent_transform: Single<&Transform, (With<Agent>, Without<Target>, Without<Goal>)>,
    time: Res<Time>,
) {
    let target_position = target_transform.translation.truncate();
    let agent_position = agent_transform.translation.truncate();

    let away_from_agent = target_position - agent_position;
    let direction = away_from_agent.normalize_or_zero();
    let distance = away_from_agent.length();

    let speed = target_flee_speed(distance);
    let target_velocity = direction * speed;

    let new_target_position = target_position + target_velocity * time.delta_secs();

    target_transform.translation.x = new_target_position.x;
    target_transform.translation.y = new_target_position.y;
}

fn update_agent(
    mut agent_transform: Single<&mut Transform, (With<Agent>, Without<Target>, Without<Goal>)>,
    target_transform: Single<&Transform, (With<Target>, Without<Agent>, Without<Goal>)>,
    goal_transform: Single<&Transform, (With<Goal>, Without<Agent>, Without<Target>)>,
    time: Res<Time>,
) {
    let agent_position = agent_transform.translation.truncate();
    let target_position = target_transform.translation.truncate();
    let goal_position = goal_transform.translation.truncate();

    let e = target_position - goal_position;
    let a = K1 / F_LOWER;
    let x_d = target_position + a * e;
    let r = x_d - agent_position;

    let control_gain = (1.0 + a) * F_UPPER + K2;
    let u = (control_gain * r).clamp_length_max(MAX_AGENT_SPEED);

    let new_agent_position = agent_position + u * time.delta_secs();

    agent_transform.translation.x = new_agent_position.x;
    agent_transform.translation.y = new_agent_position.y;
}