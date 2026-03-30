use bevy::prelude::*;

// -----------------------------------------------------------------------------
// Global constants
// -----------------------------------------------------------------------------

const ENTITY_DIAMETER: f32 = 25.0;

const AGENT_INITIAL_POSITION: Vec3 = Vec3::new(350.0, 150.0, 1.0);
const TARGET_INITIAL_POSITION: Vec3 = Vec3::new(-100.0, -100.0, 1.0);

const BACKGROUND_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const AGENT_COLOR: Color = Color::srgb(0.3, 0.3, 0.7);
const TARGET_COLOR: Color = Color::srgb(1.0, 0.5, 0.5);

const PROPORTIONAL_GAIN: f32 = 1.0;
const MAX_AGENT_SPEED: f32 = 140.0;
const TOLERANCE: f32 = 5.0;

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

// -----------------------------------------------------------------------------
// App entry point
// -----------------------------------------------------------------------------

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_message::<BehaviorRequestMessage>()
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(
            Startup,
            (
                startup_world,
                startup_simulation,
                startup_render,
            )
                .chain(),
        )
        .add_systems(
            FixedUpdate,
            (
                apply_behavior_requests,
                simulate_agents,
            )
                .chain(),
        )
        .add_systems(Update, handle_behavior_buttons)
        .add_systems(
            RunFixedMainLoop,
            (
                sync_positions_to_transforms,
                update_status_text,
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
struct ControlledAgent;

#[derive(Component)]
struct Target;

#[derive(Component)]
struct StatusText;

#[derive(Component)]
struct Position {
    value: Vec2,
}

#[derive(Component)]
struct Velocity {
    value: Vec2,
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
enum BehaviorPrimitive {
    Idle,
    GoTo { destination: Vec2 },
}


#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
enum PrimitiveEvaluation {
    Running,
    Finished,
    Waiting,
}

#[derive(Component, Clone, Copy, Debug, PartialEq, Eq)]
enum BehaviorButtonAction {
    Idle,
    GoTo,
}

// -----------------------------------------------------------------------------
// Buffered behavior requests
// -----------------------------------------------------------------------------
#[derive(Message, Clone, Copy, Debug)]
struct BehaviorRequestMessage {
    agent: Entity,
    request: BehaviorRequest,
}

#[derive(Clone, Copy, Debug)]
enum BehaviorRequest {
    SetPrimitive(BehaviorPrimitive),
}

// -----------------------------------------------------------------------------
// Guidance
// -----------------------------------------------------------------------------

enum GuidanceCommand {
    Hold,
    MoveTo { destination: Vec2 },
}

// -----------------------------------------------------------------------------
// Startup systems
// -----------------------------------------------------------------------------

fn startup_world(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn startup_simulation(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Mesh2d(meshes.add(Triangle2d::default())),
        MeshMaterial2d(materials.add(AGENT_COLOR)),
        Transform::from_translation(AGENT_INITIAL_POSITION)
            .with_scale(Vec2::splat(ENTITY_DIAMETER).extend(1.0)),
        Agent,
        ControlledAgent,
        Position {
            value: AGENT_INITIAL_POSITION.truncate(),
        },
        Velocity { value: Vec2::ZERO },
        BehaviorPrimitive::Idle,
        PrimitiveEvaluation::Waiting,
    ));

    commands.spawn((
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(TARGET_COLOR)),
        Transform::from_translation(TARGET_INITIAL_POSITION)
            .with_scale(Vec2::splat(ENTITY_DIAMETER).extend(1.0)),
        Target,
        Position {
            value: TARGET_INITIAL_POSITION.truncate(),
        },
    ));
}

fn startup_render(mut commands: Commands) {
    // Top-left status text
    commands.spawn((
        StatusText,
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
        .with_children(|parent| {
            parent
                .spawn((
                    Button,
                    BehaviorButtonAction::Idle,
                    Node {
                        width: px(120),
                        height: px(44),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("Idle"),
                        TextFont {
                            font_size: 20.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));
                });

            parent
                .spawn((
                    Button,
                    BehaviorButtonAction::GoTo,
                    Node {
                        width: px(120),
                        height: px(44),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(NORMAL_BUTTON),
                ))
                .with_children(|button| {
                    button.spawn((
                        Text::new("GoTo"),
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
// Fixed-step executive: commit requests into authoritative behavior state
// -----------------------------------------------------------------------------

fn apply_behavior_requests(
    mut requests: MessageReader<BehaviorRequestMessage>,
    mut behaviors: Query<&mut BehaviorPrimitive, With<Agent>>,
) {
    for message in requests.read() {
        if let Ok(mut behavior) = behaviors.get_mut(message.agent) {
            *behavior = commit_behavior_request(*behavior, message.request);
        }
    }
}

fn commit_behavior_request(
    _current_behavior: BehaviorPrimitive,
    request: BehaviorRequest,
) -> BehaviorPrimitive {
    match request {
        BehaviorRequest::SetPrimitive(next_behavior) => next_behavior,
    }
}

// -----------------------------------------------------------------------------
// Fixed-step simulation
// -----------------------------------------------------------------------------

fn simulate_agents(
    mut agents: Query<
        (
            &mut Position,
            &mut Velocity,
            &mut BehaviorPrimitive,
            &mut PrimitiveEvaluation,
        ),
        With<Agent>,
    >,
    time: Res<Time>,
) {
    for (mut position, mut velocity, mut behavior, mut evaluation) in &mut agents {
        let primitive_evaluation = evaluate_primitive(*behavior, position.value);
        let next_behavior = resolve_behavior_transition(*behavior, primitive_evaluation);
        let guidance = compute_guidance_command(next_behavior);
        let commanded_velocity = compute_velocity_command(guidance, position.value);
        let next_position = integrate_position(position.value, commanded_velocity, time.delta_secs());

        *evaluation = primitive_evaluation;
        *behavior = next_behavior;
        velocity.value = commanded_velocity;
        position.value = next_position;
    }
}

/// Has the currently active primitive achieved its local condition?
fn evaluate_primitive(
    behavior: BehaviorPrimitive,
    agent_position: Vec2,
) -> PrimitiveEvaluation {
    match behavior {
        BehaviorPrimitive::Idle => PrimitiveEvaluation::Waiting,
        BehaviorPrimitive::GoTo { destination } => {
            let distance_to_destination = agent_position.distance(destination);

            if distance_to_destination < TOLERANCE {
                PrimitiveEvaluation::Finished
            } else {
                PrimitiveEvaluation::Running
            }
        }
    }
}

/// Given the current primitive's local condition, what should remain active?
///
/// In the current manual-only setup:
/// - `Idle` persists until a human request changes it.
/// - `GoTo` persists while running.
/// - `GoTo` falls back to `Idle` once finished.
fn resolve_behavior_transition(
    current_behavior: BehaviorPrimitive,
    evaluation: PrimitiveEvaluation,
) -> BehaviorPrimitive {
    match current_behavior {
        BehaviorPrimitive::Idle => BehaviorPrimitive::Idle,
        BehaviorPrimitive::GoTo { destination } => match evaluation {
            PrimitiveEvaluation::Running => BehaviorPrimitive::GoTo { destination },
            PrimitiveEvaluation::Finished | PrimitiveEvaluation::Waiting => BehaviorPrimitive::Idle,
        },
    }
}

fn compute_guidance_command(behavior: BehaviorPrimitive) -> GuidanceCommand {
    match behavior {
        BehaviorPrimitive::Idle => GuidanceCommand::Hold,
        BehaviorPrimitive::GoTo { destination } => GuidanceCommand::MoveTo { destination },
    }
}

fn compute_velocity_command(
    guidance: GuidanceCommand,
    agent_position: Vec2,
) -> Vec2 {
    match guidance {
        GuidanceCommand::Hold => Vec2::ZERO,
        GuidanceCommand::MoveTo { destination } => {
            let tracking_error = destination - agent_position;
            let commanded_velocity = PROPORTIONAL_GAIN * tracking_error;
            commanded_velocity.clamp_length_max(MAX_AGENT_SPEED)
        }
    }
}

fn integrate_position(
    position: Vec2,
    velocity: Vec2,
    dt: f32,
) -> Vec2 {
    position + velocity * dt
}

// -----------------------------------------------------------------------------
// UI input: produce behavior requests, do not mutate behavior directly
// -----------------------------------------------------------------------------

fn handle_behavior_buttons(
    mut interactions: Query<
        (&Interaction, &BehaviorButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    controlled_agent: Single<Entity, (With<Agent>, With<ControlledAgent>)>,
    target_position: Single<&Position, With<Target>>,
    mut request_writer: MessageWriter<BehaviorRequestMessage>,
) {
    for (interaction, action, mut color) in &mut interactions {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();

                let requested_behavior =
                    requested_behavior_from_button(*action, target_position.value);

                request_writer.write(BehaviorRequestMessage {
                    agent: *controlled_agent,
                    request: BehaviorRequest::SetPrimitive(requested_behavior),
                });
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn requested_behavior_from_button(
    action: BehaviorButtonAction,
    target_position: Vec2,
) -> BehaviorPrimitive {
    match action {
        BehaviorButtonAction::Idle => BehaviorPrimitive::Idle,
        BehaviorButtonAction::GoTo => BehaviorPrimitive::GoTo {
            destination: target_position,
        },
    }
}

// -----------------------------------------------------------------------------
// Render / UI presentation
// -----------------------------------------------------------------------------

fn sync_positions_to_transforms(
    mut query: Query<(&Position, &mut Transform)>,
) {
    for (position, mut transform) in &mut query {
        transform.translation = position.value.extend(transform.translation.z);
    }
}

fn update_status_text(
    mut text: Single<&mut Text, With<StatusText>>,
    controlled_agent: Single<
        (&BehaviorPrimitive, &PrimitiveEvaluation),
        (With<Agent>, With<ControlledAgent>),
    >,
) {
    let (behavior, evaluation) = *controlled_agent;

    text.0 = "Active Behavior: ".to_string();

    match behavior {
        BehaviorPrimitive::Idle => text.push_str("Idle"),
        BehaviorPrimitive::GoTo { destination } => {
            text.push_str(&format!(
                "GoTo ({:.1}, {:.1})",
                destination.x, destination.y
            ));
        }
    }

    match evaluation {
        PrimitiveEvaluation::Finished => text.push_str("\nBehavior Status: Finished."),
        PrimitiveEvaluation::Waiting => text.push_str("\nBehavior Status: Waiting."),
        PrimitiveEvaluation::Running => text.push_str("\nBehavior Status: Running."),
    }
}

/*
Future directions
- Add Pickup, Dropoff, Herd, Evade, etc.
- Add targetable requests for multiple agents from the same UI.
- Add explicit sequencing / planning above primitive-level execution.
- Add selection UI so BehaviorRequestMessage.agent is chosen dynamically.
*/