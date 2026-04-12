use bevy::camera::Viewport;
use bevy::prelude::*;
use bevy::window::{Window, WindowResized};
use crate::components::game_camera_component::GameCameraComponent;
use crate::components::target_component::TargetComponent;

pub fn update_game_camera_system (
    windows: Query<&Window>,
    mut cameras: Query<(&mut Camera, &TargetComponent, &mut Transform), With<GameCameraComponent>>,
    mut resize_events: MessageReader<WindowResized>,
    target_query: Query<&Transform, Without<GameCameraComponent>>,
) {
    let should_resize = resize_events.read().next().is_some() && windows.single().is_ok();

    let Ok(window) = windows.single() else {
        return;
    };

    let Ok((mut camera, target, mut camera_position)) = cameras.single_mut() else {
        return;
    };

    if should_resize {
        let width = window.physical_width();
        let height = window.physical_height();

        let game_width = (width as f32 * 0.75) as u32;
        let game_height = (height as f32 * 0.75) as u32;

        camera.viewport = Some(Viewport {
            physical_position: UVec2::new(0, 0),
            physical_size: UVec2::new(game_width, game_height),
            ..default()
        });
    }

    match target.get_target_entity() {
        Some(e) => {
            if let Ok (target_position) = target_query.get(e) {
                let t = 0.1; // smoothing factor (tweak this)

                // Position
                camera_position.translation = camera_position
                    .translation
                    .lerp(target_position.translation, t);

                // Rotation
                camera_position.rotation = camera_position
                    .rotation
                    .slerp(target_position.rotation, t);

            }
        },
        _ => return
    }
}