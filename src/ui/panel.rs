use bevy::prelude::*;

use crate::{
    constants::{
        DEFAULT_BUTTON_COLOR,
        HOVERED_BUTTON_COLOR,
        PRESSED_BUTTON_COLOR,
    },
    model::{
        Agent,
        BehaviorChange,
        BehaviorChangeMessage,
        BehaviorSelection,
        BehaviorStatusText,
        SelectedTarget,
        Target,
        WorldPosition,
    },
    sim::behavior_from_selection,
};

pub fn spawn_ui(mut commands: Commands) {
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
            for (behavior_selection, label) in [
                (BehaviorSelection::Idle, "Idle"),
                (BehaviorSelection::MoveTo, "MoveTo"),
                (BehaviorSelection::MoveAwayFrom, "MoveAwayFrom"),
            ] {
                button_panel
                    .spawn((
                        Button,
                        behavior_selection,
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
                            Text::new(label),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::WHITE),
                        ));
                    });
            }
        });
}

pub fn handle_behavior_selection_buttons(
    mut button_interactions: Query<
        (&Interaction, &BehaviorSelection, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    agent_entity: Single<Entity, With<Agent>>,
    selected_target_world_position: Single<&WorldPosition, (With<Target>, With<SelectedTarget>)>,
    mut behavior_change_writer: MessageWriter<BehaviorChangeMessage>,
) {
    for (interaction_state, behavior_selection, mut button_background_color) in
        &mut button_interactions
    {
        match *interaction_state {
            Interaction::Pressed => {
                *button_background_color = PRESSED_BUTTON_COLOR.into();

                let requested_behavior = behavior_from_selection(
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

pub fn handle_cursor_target_selection(
    click: On<Pointer<Click>>,
    targets: Query<(), With<Target>>,
    old_target_entity: Single<Entity, (With<SelectedTarget>, With<Target>)>,
    mut commands: Commands,
) {
    let clicked_entity = click.original_event_target();

    if targets.get(clicked_entity).is_err() {
        return;
    }

    if clicked_entity == *old_target_entity {
        return;
    }

    commands.entity(*old_target_entity).remove::<SelectedTarget>();
    commands.entity(clicked_entity).insert(SelectedTarget);
}