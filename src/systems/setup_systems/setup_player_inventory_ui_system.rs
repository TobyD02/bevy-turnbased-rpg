use bevy::color::Color;
use bevy::prelude::{default, BackgroundColor, Commands, Entity, FlexDirection, Node, Overflow, PositionType, Query, Text, TextColor, TextFont, UiRect, UiTargetCamera, Val, With};
use crate::components::ui_camera_component::UiCameraComponent;
use crate::resources::game_log_resource::GameLogResource;

pub fn setup_player_inventory_ui_system(
    mut commands: Commands,
    query: Query<Entity, With<UiCameraComponent>>
) {
    commands.insert_resource(GameLogResource::default());

    let Ok(ui_camera) = query.single() else {
        return
    };
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            left: Val::Px(0.0),
            bottom: Val::Px(0.0),

            width: Val::Percent(75.0),
            height: Val::Percent(25.0),

            overflow: Overflow::clip(),
            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.3, 0.3, 0.3)),
        UiTargetCamera(ui_camera)
    )).with_children(|parent| {
        parent.spawn((
            Text::new("Inventory"), // start empty
            TextFont {
                font_size: 14.0,
                ..Default::default()
            },
            TextColor(Color::WHITE),
        ));
    });
}