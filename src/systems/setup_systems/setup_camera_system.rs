use crate::components::game_camera_component::GameCameraComponent;
use crate::constants::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};
use bevy::camera::{ScalingMode, Viewport};
use bevy::prelude::*;
use crate::components::target_component::TargetComponent;

pub fn setup_camera_system(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera::default(),
        Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            scale: 0.5,
            ..OrthographicProjection::default_2d()
        }),
        GameCameraComponent,
        Transform::from_translation(Vec3::new(
            MAP_WIDTH as f32 * TILE_SIZE as f32 / 2.,
            MAP_HEIGHT as f32 * TILE_SIZE as f32 / 2.,
            0.0,
        )),
        TargetComponent::default(),
    ));
}
