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
        .add_systems(
            FixedUpdate,
            (
                update_target,
                update_goal,
                update_agent,
                sync_kinematic_state_to_transforms,
            )
                .chain(),
        )
        .run();
}

#[derive(Component)]
struct Agent;

#[derive(Component)]
struct Target;

#[derive(Component)]
struct Goal;

#[derive(Component)]
struct KinematicState {
    position: Vec2,
    velocity: Vec2,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    // Target entity
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(TARGET_COLOR)),
        Transform::from_translation(TARGET_STARTING_POSITION)
            .with_scale(Vec2::splat(ENTITY_DIAMETER).extend(1.0)),
        KinematicState {
            position: TARGET_STARTING_POSITION.truncate(),
            velocity: Vec2::ZERO,
        },
        Target,
    ));

    // Agent entity
    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::default())),
        MeshMaterial2d(materials.add(AGENT_COLOR)),
        Transform::from_translation(AGENT_STARTING_POSITION)
            .with_scale(Vec2::splat(ENTITY_DIAMETER).extend(1.0)),
        KinematicState {
            position: AGENT_STARTING_POSITION.truncate(),
            velocity: Vec2::ZERO,
        },
        Agent,
    ));

    // Goal entity
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(GOAL_COLOR)),
        Transform::from_translation(GOAL_LOCATION)
            .with_scale(Vec2::splat(ENTITY_DIAMETER).extend(1.0)),
        KinematicState {
            position: GOAL_LOCATION.truncate(),
            velocity: Vec2::ZERO,
        },
        Goal,
    ));
}

fn target_dynamics(target_position: Vec2, agent_position: Vec2) -> Vec2 {
    let away_from_agent = target_position - agent_position;
    let direction = away_from_agent.normalize_or_zero();
    let distance = away_from_agent.length();

    let speed = TARGET_MIN_SPEED
        + (TARGET_MAX_SPEED - TARGET_MIN_SPEED) * (-distance / REPULSION_DECAY_LENGTH).exp();

    direction * speed
}

fn goal_dynamics(_goal_position: Vec2) -> Vec2 {
    Vec2::ZERO
}

fn agent_dynamics(agent_position: Vec2, target_position: Vec2, goal_position: Vec2) -> Vec2 {
    let e = target_position - goal_position;
    let a = K1 / F_LOWER;
    let x_d = target_position + a * e;
    let r = x_d - agent_position;

    let control_gain = (1.0 + a) * F_UPPER + K2;

    (control_gain * r).clamp_length_max(MAX_AGENT_SPEED)
}

fn integrate_position(position: Vec2, velocity: Vec2, dt: f32) -> Vec2 {
    position + velocity * dt
}

fn update_target(
    mut target_state: Single<&mut KinematicState, (With<Target>, Without<Agent>)>,
    agent_state: Single<&KinematicState, (With<Agent>, Without<Target>)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    let target_velocity = target_dynamics(target_state.position, agent_state.position);

    target_state.velocity = target_velocity;
    target_state.position = integrate_position(target_state.position, target_state.velocity, dt);
}

fn update_goal(
    mut goal_state: Single<&mut KinematicState, With<Goal>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    let goal_velocity = goal_dynamics(goal_state.position);

    goal_state.velocity = goal_velocity;
    goal_state.position = integrate_position(goal_state.position, goal_state.velocity, dt);
}

fn update_agent(
    mut agent_state: Single<&mut KinematicState, (With<Agent>, Without<Target>, Without<Goal>)>,
    target_state: Single<&KinematicState, (With<Target>, Without<Agent>)>,
    goal_state: Single<&KinematicState, (With<Goal>, Without<Agent>)>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();

    let agent_velocity = agent_dynamics(
        agent_state.position,
        target_state.position,
        goal_state.position,
    );

    agent_state.velocity = agent_velocity;
    agent_state.position = integrate_position(agent_state.position, agent_state.velocity, dt);
}

fn sync_kinematic_state_to_transforms(mut query: Query<(&KinematicState, &mut Transform)>) {
    for (state, mut transform) in &mut query {
        transform.translation.x = state.position.x;
        transform.translation.y = state.position.y;
    }
}