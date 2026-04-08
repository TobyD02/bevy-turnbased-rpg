use bevy::camera::{ScalingMode, Viewport};
use bevy::prelude::*;
use crate::components::game_camera_component::GameCameraComponent;

pub fn setup_camera_system(
    mut commands: Commands,
) {
    commands.spawn((
        Camera2d,
        Camera::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            scale: 0.2,
            ..OrthographicProjection::default_2d()
        }),
        GameCameraComponent
    ));
}