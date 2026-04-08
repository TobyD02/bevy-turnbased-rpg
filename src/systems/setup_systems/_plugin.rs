use bevy::prelude::*;
use crate::systems::setup_systems::spawn_camera_system::spawn_camera_system;
use crate::systems::setup_systems::spawn_character_system::spawn_character_system;
use crate::systems::setup_systems::spawn_game_log_system::spawn_game_log_system;
use crate::systems::setup_systems::spawn_player_system::spawn_player_system;
use crate::systems::setup_systems::spawn_turn_order_system::spawn_turn_order_system;
use crate::systems::setup_systems::spawn_ui_camera_system::spawn_ui_camera_system;

pub struct SetupSystemsPlugin;

impl Plugin for SetupSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                spawn_camera_system,
                spawn_turn_order_system,
                spawn_player_system,
                spawn_character_system,
            ).chain(),
        )
        .add_systems(
            Startup,
            (spawn_ui_camera_system, spawn_game_log_system).chain(),
        );
    }
}
