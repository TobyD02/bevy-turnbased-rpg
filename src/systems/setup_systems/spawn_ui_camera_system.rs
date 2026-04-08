use bevy::camera::visibility::RenderLayers;
use bevy::prelude::*;
use crate::components::ui_camera_component::UiCameraComponent;

pub fn spawn_ui_camera_system(
    mut commands: Commands
) {
    commands.spawn((
        Camera2d,
        Camera {
            order: 1, // render AFTER game
            ..default()
        },
        RenderLayers::layer(1),
        IsDefaultUiCamera,
        UiCameraComponent
    ),);

}