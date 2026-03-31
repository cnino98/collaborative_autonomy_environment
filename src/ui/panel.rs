use bevy::prelude::*;

use crate::{
    constants::DEFAULT_BUTTON_COLOR,
    input::BehaviorSelection,
};

pub fn spawn_behavior_panel(commands: &mut Commands) {
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
