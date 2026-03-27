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
const TOLERANCE: f32 = 5.0;

// Main simulation loop
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, 
            (
                startup_world,
                startup_simulation,
                startup_render
            ).chain())
        .add_systems(FixedUpdate, update_simulation)
        .add_systems(
            RunFixedMainLoop,
            (
                update_render,
                update_ui,
            ).in_set(RunFixedMainLoopSystems::AfterFixedMainLoop),
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

#[derive(Component)]
enum BehaviorPrimitive {
    Idle, 
    GoTo {destination: Vec2},
}

enum GuidanceCommand{
    Hold,
    MoveTo{destination: Vec2},
}

// Setup
fn startup_simulation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::default())),
        MeshMaterial2d(materials.add(AGENT_COLOR)),
        Transform::from_translation(AGENT_INITIAL_POSITION).with_scale(Vec2::splat(ENTITY_DIAMETER).extend(1.0)),
        Agent,
        Position{value:AGENT_INITIAL_POSITION.truncate()},
        Velocity{value:Vec2::ZERO},
        BehaviorPrimitive::GoTo{destination:TARGET_INITIAL_POSITION.truncate()},
    ));

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(TARGET_COLOR)),
        Transform::from_translation(TARGET_INITIAL_POSITION).with_scale(Vec2::splat(ENTITY_DIAMETER).extend(1.0)),
        Target,
        Position{value:TARGET_INITIAL_POSITION.truncate()},
    ));
}

fn startup_world(mut commands: Commands){
    commands.spawn(Camera2d);
}

fn startup_render(mut commands: Commands){
    commands.spawn((
        Text::default(),
        TextColor(Color::srgb(0.0, 0.0, 0.0)),
        Node{
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));
}

// Simulation
fn update_simulation(
    mut agent_position: Single<&mut Position, (With<Agent>, Without<Target>)>,
    mut agent_velocity: Single<&mut Velocity, With<Agent>>,
    mut current_behavior: Single<&mut BehaviorPrimitive, With<Agent>>,
    time: Res<Time>,
) {
    // Behavior -> Guidance -> Control
    let active_behavior: BehaviorPrimitive = transition_behavior(&current_behavior, agent_position.value);
    let guidance: GuidanceCommand = compute_guidance_command(&active_behavior);
    let commanded_velocity: Vec2 = compute_velocity_command(guidance, agent_position.value);
    let next_position: Vec2 = integrate_position(agent_position.value, commanded_velocity, time.delta_secs());

    // Update variables
    **current_behavior = active_behavior;
    agent_velocity.value = commanded_velocity;
    agent_position.value = next_position;
        
}

fn transition_behavior(behavior: &BehaviorPrimitive, agent_position: Vec2) -> BehaviorPrimitive {
    match behavior {
        BehaviorPrimitive::Idle => BehaviorPrimitive::Idle,
        BehaviorPrimitive::GoTo { destination } => {
            let distance = agent_position.distance(*destination);
            if distance < TOLERANCE {
                BehaviorPrimitive::Idle
            } else {
                BehaviorPrimitive::GoTo { destination: *destination }
            }
        }
    }
}

fn compute_guidance_command(behavior: &BehaviorPrimitive) -> GuidanceCommand {
    match behavior{
        BehaviorPrimitive::Idle => GuidanceCommand::Hold,
        BehaviorPrimitive::GoTo{destination} => GuidanceCommand::MoveTo { destination: *destination }
    }
}

fn compute_velocity_command(guidance: GuidanceCommand, agent_position: Vec2) -> Vec2 {
    match guidance {
        GuidanceCommand::Hold => Vec2::ZERO,
        GuidanceCommand::MoveTo { destination } => {
            let tracking_error: Vec2 = destination - agent_position;
            let commanded_velocity: Vec2 = PROPORTIONAL_GAIN * tracking_error;
            commanded_velocity.clamp_length_max(MAX_AGENT_SPEED)
        }
    }
}

fn integrate_position(position: Vec2, velocity: Vec2, dt: f32) -> Vec2 {
    position + velocity * dt
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

// UI
fn update_ui(
    mut text: Single<&mut Text>,
    current_behavior: Single<&BehaviorPrimitive, With<Agent>>,
) {
    text.0 = "Agent State: ".to_string();
    match &*current_behavior {
        BehaviorPrimitive::Idle => text.push_str("Idle"),
        BehaviorPrimitive::GoTo { destination } => {
            text.push_str(&format!("GoTo ({:.1}, {:.1})", destination.x, destination.y));
        }
    }
}