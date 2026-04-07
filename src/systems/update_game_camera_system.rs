use bevy::camera::Viewport;
use bevy::prelude::*;
use bevy::window::{Window, WindowResized};
use crate::components::game_camera_component::GameCameraComponent;

pub fn update_game_camera_system (
    windows: Query<&Window>,
    mut cameras: Query<&mut Camera, With<GameCameraComponent>>,
    mut resize_events: MessageReader<WindowResized>,
) {
    if resize_events.read().next().is_none() {
        return;
    }

    let Ok(window) = windows.single() else {
        return;
    };

    let Ok(mut camera) = cameras.single_mut() else {
        return;
    };

    let width = window.physical_width();
    let height = window.physical_height();

    let game_width = (width as f32 * 0.75) as u32;

    camera.viewport = Some(Viewport {
        physical_position: UVec2::new(0, 0),
        physical_size: UVec2::new(game_width, height),
        ..default()
    });
}