use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use crate::components::game_log_component::GameLogComponent;
use crate::components::game_log_text_component::GameLogTextComponent;
use crate::components::ui_camera_component::UiCameraComponent;
use crate::resources::game_log_resource::GameLogResource;

pub fn setup_game_log_system(
    mut commands: Commands,
    mut asset_server: ResMut<AssetServer>,
    query: Query<Entity, With<UiCameraComponent>>
) {
    commands.insert_resource(GameLogResource::default());

    let Ok(ui_camera) = query.single() else {
        return
    };

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            right: Val::Px(0.0),
            top: Val::Px(0.0),

            width: Val::Percent(25.0),
            height: Val::Percent(100.0),

            flex_direction: FlexDirection::Column,
            padding: UiRect::all(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(Color::srgb(0.08, 0.08, 0.08)),
        GameLogComponent,
        UiTargetCamera(ui_camera)
    )).with_children(|parent| {
        parent.spawn((
            Text::new(""), // start empty
            TextFont {
                font_size: 14.0,
                ..Default::default()
            },
            TextColor(Color::WHITE),
            GameLogTextComponent,
        ));
    });
}
