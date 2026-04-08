use bevy::prelude::*;
use crate::systems::update_systems::character_movement_system::character_movement_system;
use crate::systems::update_systems::draw_game_log_system::draw_game_log_system;
use crate::systems::update_systems::manage_turn_order_system::manage_turn_order_system;
use crate::systems::update_systems::player_movement_system::player_movement_system;
use crate::systems::update_systems::map_translation_system::map_translation_system;
use crate::systems::update_systems::update_game_camera_system::update_game_camera_system;

pub struct UpdateSystemsPlugin;

impl Plugin for UpdateSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_game_log_system)
            .add_systems(
                Update,
                (
                    manage_turn_order_system,
                    player_movement_system,
                    character_movement_system,
                    map_translation_system,
                ).chain(),
            )
            .add_systems(Update, update_game_camera_system);
    }
}
