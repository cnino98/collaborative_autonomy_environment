use bevy::{
    prelude::*,
    window::WindowResolution,
    color::palettes::basic,
};

// -----------------------------------------------------------------------------
// Global constants
// -----------------------------------------------------------------------------

const ENTITY_RENDER_SIZE: f32 = 25.0;

const AGENT_INITIAL_POSITION: Vec3 = Vec3::new(350.0, 150.0, 1.0);

const WORLD_BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);

const VELOCITY_COMMAND_GAIN: f32 = 1.0;
const AGENT_MAX_SPEED: f32 = 140.0;
const MOVE_TO_ARRIVAL_TOLERANCE: f32 = ENTITY_RENDER_SIZE * 2.0;
const MOVE_AWAY_SAFE_DISTANCE: f32 = 500.0;

const DEFAULT_BUTTON_COLOR: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON_COLOR: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON_COLOR: Color = Color::srgb(0.35, 0.75, 0.35);

// -----------------------------------------------------------------------------
// Main
// -----------------------------------------------------------------------------

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(3024,1964),
                ..default()
            }),
            ..default()
        }))
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

// -----------------------------------------------------------------------------
// Components
// -----------------------------------------------------------------------------

#[derive(Component)]
struct Agent;

#[derive(Component)]
struct Target;

#[derive(Component)]
struct SelectedTarget;

#[derive(Component)]
struct BehaviorStatusText;

#[derive(Component)]
struct WorldPosition {
    coordinates: Vec2,
}

#[derive(Component)]
struct LinearVelocity {
    units_per_second: Vec2,
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
enum Behavior {
    Idle,
    MoveTo { destination: Vec2 },
    MoveAwayFrom { threat_location: Vec2 },
}

enum BehaviorCompletionStatus {
    InProgress,
    Complete,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
enum BehaviorSelection {
    Idle,
    MoveTo,
    MoveAwayFrom,
}

#[derive(Message, Clone, Copy, Debug)]
struct BehaviorChangeMessage {
    agent_entity: Entity,
    behavior_change: BehaviorChange,
}

#[derive(Clone, Copy, Debug)]
enum BehaviorChange {
    SetBehavior(Behavior),
}

// -----------------------------------------------------------------------------
// Guidance
// -----------------------------------------------------------------------------

enum GuidanceCommand {
    StayStill,
    MoveToward { destination: Vec2 },
    MoveAwayFrom { threat_location: Vec2 },
}

// -----------------------------------------------------------------------------
// Startup systems
// -----------------------------------------------------------------------------

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn spawn_simulation_entities(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn agent
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

    // Spawn a moving target
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::from(basic::BLACK))),
        Transform::from_translation(Vec3 { x: (-100.0), y: (-100.0), z: (1.0) })
            .with_scale(Vec2::splat(ENTITY_RENDER_SIZE).extend(1.0)),
        Target,
        WorldPosition {
            coordinates: Vec2 { x: (-100.0), y: (-100.0) },
        },
        LinearVelocity {
            units_per_second: Vec2::ZERO,
        },
    ));

    // Spawn a stationary target
    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(Color::from(basic::RED))),
        Transform::from_translation(Vec3 { x: (0.0), y: (0.0), z: (1.0) })
            .with_scale(Vec2::splat(ENTITY_RENDER_SIZE).extend(1.0)),
        Target,
        SelectedTarget,
        WorldPosition {
            coordinates: Vec2 { x: (0.0), y: (0.0) },
        },
    ));
}

fn spawn_ui(mut commands: Commands) {
    // Top-left status text
    commands.spawn((
        BehaviorStatusText,
        Text::default(),
        TextColor(Color::BLACK),
        Node {
            position_type: PositionType::Absolute,
            top: px(12),
            left: px(12),
            ..default()
        },
    ));

    // Bottom-left button panel
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                left: px(12),
                bottom: px(12),
                flex_direction: FlexDirection::Column,
                row_gap: px(8),
                padding: UiRect::all(px(10)),
                ..default()
            },
            BackgroundColor(Color::srgba(1.0, 1.0, 1.0, 0.85)),
        ))
        .with_children(|button_panel| {
            button_panel
                .spawn((
                    Button,
                    BehaviorSelection::Idle,
                    Node {
                        width: px(200),
                        height: px(44),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(DEFAULT_BUTTON_COLOR),
                ))
                .with_children(|button_contents| {
                    button_contents.spawn((
                        Text::new("Idle"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            button_panel
                .spawn((
                    Button,
                    BehaviorSelection::MoveTo,
                    Node {
                        width: px(200),
                        height: px(44),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(DEFAULT_BUTTON_COLOR),
                ))
                .with_children(|button_contents| {
                    button_contents.spawn((
                        Text::new("MoveTo"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            button_panel
                .spawn((
                    Button,
                    BehaviorSelection::MoveAwayFrom,
                    Node {
                        width: px(200),
                        height: px(44),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(DEFAULT_BUTTON_COLOR),
                ))
                .with_children(|button_contents| {
                    button_contents.spawn((
                        Text::new("MoveAwayFrom"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });
        });
}

// -----------------------------------------------------------------------------
// Behavior
// -----------------------------------------------------------------------------

fn apply_behavior_changes(
    mut incoming_behavior_changes: MessageReader<BehaviorChangeMessage>,
    mut agent_behaviors: Query<&mut Behavior, With<Agent>>,
) {
    for behavior_change_message in incoming_behavior_changes.read() {
        if let Ok(mut current_behavior) =
            agent_behaviors.get_mut(behavior_change_message.agent_entity)
        {
            *current_behavior = behavior_after_applying_change(
                *current_behavior,
                behavior_change_message.behavior_change,
            );
        }
    }
}

fn behavior_after_applying_change(
    _current_behavior: Behavior,
    behavior_change: BehaviorChange,
) -> Behavior {
    match behavior_change {
        BehaviorChange::SetBehavior(requested_behavior) => requested_behavior,
    }
}

// -----------------------------------------------------------------------------
// Simulation
// -----------------------------------------------------------------------------

fn advance_target_simulation(
    mut targets: Query<
        (
            &mut WorldPosition,
            &mut LinearVelocity,
        ),
        (With<Target>, Without<Agent>),
    >,
    time: Res<Time>,
){
    for (mut world_position, mut linear_velocity) in &mut targets{
        let commanded_velocity: Vec2 = target_dynamics(world_position.coordinates);
        let next_world_position: Vec2 = integrate_world_position(
            world_position.coordinates,
            commanded_velocity,
            time.delta_secs(),
        );

    linear_velocity.units_per_second = commanded_velocity;
    world_position.coordinates = next_world_position;
    }
}

fn target_dynamics(position: Vec2) -> Vec2 {
    Vec2::new(-position.y, position.x)
}

fn advance_agent_simulation(
    mut agents: Query<
        (
            &mut WorldPosition,
            &mut LinearVelocity,
            &mut Behavior,
        ),
        With<Agent>,
    >,
    time: Res<Time>,
) {
    for (mut world_position, mut linear_velocity, mut current_behavior) in &mut agents {
        let behavior_completion: BehaviorCompletionStatus = evaluate_behavior_completion(
            *current_behavior,
            world_position.coordinates,
        );
        let next_behavior: Behavior = transition_behavior(*current_behavior, behavior_completion);
        let guidance_command: GuidanceCommand = guidance_command_for_behavior(next_behavior);
        let commanded_velocity: Vec2 = velocity_command_for_guidance(
            guidance_command,
            world_position.coordinates,
        );
        let next_world_position: Vec2 = integrate_world_position(
            world_position.coordinates,
            commanded_velocity,
            time.delta_secs(),
        );

        *current_behavior = next_behavior;
        linear_velocity.units_per_second = commanded_velocity;
        world_position.coordinates = next_world_position;
    }
}

/// Has the active behavior met its own local completion condition?
fn evaluate_behavior_completion(
    behavior: Behavior,
    agent_world_position: Vec2,
) -> BehaviorCompletionStatus {
    match behavior {
        Behavior::Idle => BehaviorCompletionStatus::InProgress,
        Behavior::MoveTo { destination } => {
            let distance_to_destination = agent_world_position.distance(destination);
            if distance_to_destination < MOVE_TO_ARRIVAL_TOLERANCE {
                BehaviorCompletionStatus::Complete
            } else {
                BehaviorCompletionStatus::InProgress
            }
        }
        Behavior::MoveAwayFrom { threat_location } => {
            let distance_from_threat =
                agent_world_position.distance(threat_location);
            if distance_from_threat > MOVE_AWAY_SAFE_DISTANCE {
                BehaviorCompletionStatus::Complete
            } else {
                BehaviorCompletionStatus::InProgress
            }
        }
    }
}

/// Given the active behavior and its completion state, what behavior should remain active?
fn transition_behavior(
    current_behavior: Behavior,
    behavior_completion: BehaviorCompletionStatus,
) -> Behavior {
    match current_behavior {
        Behavior::Idle => Behavior::Idle,
        Behavior::MoveTo { destination } => match behavior_completion {
            BehaviorCompletionStatus::InProgress => Behavior::MoveTo { destination },
            BehaviorCompletionStatus::Complete => Behavior::Idle,
        },
        Behavior::MoveAwayFrom { threat_location } => match behavior_completion {
            BehaviorCompletionStatus::InProgress => Behavior::MoveAwayFrom { threat_location },
            BehaviorCompletionStatus::Complete => Behavior::Idle,
        },
    }
}

fn guidance_command_for_behavior(behavior: Behavior) -> GuidanceCommand {
    match behavior {
        Behavior::Idle => GuidanceCommand::StayStill,
        Behavior::MoveTo { destination } => GuidanceCommand::MoveToward { destination },
        Behavior::MoveAwayFrom { threat_location } => {
            GuidanceCommand::MoveAwayFrom { threat_location }
        }
    }
}

fn velocity_command_for_guidance(
    guidance_command: GuidanceCommand,
    agent_world_position: Vec2,
) -> Vec2 {
    match guidance_command {
        GuidanceCommand::StayStill => Vec2::ZERO,
        GuidanceCommand::MoveToward { destination } => {
            let position_delta_to_destination = destination - agent_world_position;
            let desired_velocity =
                VELOCITY_COMMAND_GAIN * position_delta_to_destination;
            desired_velocity.clamp_length_max(AGENT_MAX_SPEED)
        }
        GuidanceCommand::MoveAwayFrom { threat_location } => {
            let position_delta_away_from_threat =
                agent_world_position - threat_location;
            let desired_velocity =
                VELOCITY_COMMAND_GAIN * position_delta_away_from_threat;
            desired_velocity.clamp_length_max(AGENT_MAX_SPEED)
        }
    }
}

fn integrate_world_position(
    current_world_position: Vec2,
    commanded_velocity: Vec2,
    delta_seconds: f32,
) -> Vec2 {
    current_world_position + commanded_velocity * delta_seconds
}

// -----------------------------------------------------------------------------
// UI input: emit behavior change messages
// -----------------------------------------------------------------------------

fn handle_behavior_selection_buttons(
    mut button_interactions: Query<
        (&Interaction, &BehaviorSelection, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    agent_entity: Single<Entity, With<Agent>>,
    selected_target_world_position: Single<&WorldPosition, (With<Target>, With<SelectedTarget>)>,
    mut behavior_change_writer: MessageWriter<BehaviorChangeMessage>,
) {
    for (
        interaction_state,
        behavior_selection,
        mut button_background_color,
    ) in &mut button_interactions
    {
        match *interaction_state {
            Interaction::Pressed => {
                *button_background_color = PRESSED_BUTTON_COLOR.into();

                let requested_behavior = behavior_for_selection(
                    *behavior_selection,
                    selected_target_world_position.coordinates,
                );

                behavior_change_writer.write(BehaviorChangeMessage {
                    agent_entity: *agent_entity,
                    behavior_change: BehaviorChange::SetBehavior(requested_behavior),
                });
            }
            Interaction::Hovered => {
                *button_background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *button_background_color = DEFAULT_BUTTON_COLOR.into();
            }
        }
    }
}

fn behavior_for_selection(
    behavior_selection: BehaviorSelection,
    target_world_position: Vec2,
) -> Behavior {
    match behavior_selection {
        BehaviorSelection::Idle => Behavior::Idle,
        BehaviorSelection::MoveTo => Behavior::MoveTo {
            destination: target_world_position,
        },
        BehaviorSelection::MoveAwayFrom => Behavior::MoveAwayFrom {
            threat_location: target_world_position,
        },
    }
}

// -----------------------------------------------------------------------------
// Presentation: sync simulation state into scene and UI
// -----------------------------------------------------------------------------

fn sync_world_positions_to_transforms(
    mut positioned_entities: Query<(&WorldPosition, &mut Transform)>,
) {
    for (world_position, mut transform) in &mut positioned_entities {
        transform.translation =
            world_position.coordinates.extend(transform.translation.z);
    }
}

fn update_active_behavior_status_text(
    mut status_text: Single<&mut Text, With<BehaviorStatusText>>,
    agent_behavior: Single<&Behavior, With<Agent>>,
) {
    let current_behavior = *agent_behavior;

    status_text.0 = "Active Behavior: ".to_string();

    match current_behavior {
        Behavior::Idle => status_text.push_str("Idle"),
        Behavior::MoveTo { destination } => {
            status_text.push_str(&format!(
                "MoveTo ({:.1}, {:.1})",
                destination.x, destination.y
            ));
        }
        Behavior::MoveAwayFrom { threat_location } => {
            status_text.push_str(&format!(
                "MoveAwayFrom ({:.1}, {:.1})",
                threat_location.x, threat_location.y
            ));
        }
    }
}