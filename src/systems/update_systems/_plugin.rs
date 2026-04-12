use bevy::prelude::*;
use crate::systems::update_systems::update_character_movement_system::update_character_movement_system;
use crate::systems::update_systems::update_draw_game_log_system::update_draw_game_log_system;
use crate::systems::update_systems::update_turn_order_system::update_turn_order_system;
use crate::systems::update_systems::update_player_movement_system::update_player_movement_system;
use crate::systems::update_systems::update_map_translation_system::update_map_translation_system;
use crate::systems::update_systems::update_game_camera_system::update_game_camera_system;
use crate::systems::update_systems::update_remove_stale_entities_system::update_remove_stale_entities_system;

pub struct UpdateSystemsPlugin;

impl Plugin for UpdateSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_draw_game_log_system)
            .add_systems(
                Update,
                (
                    update_turn_order_system,
                    update_player_movement_system,
                    update_character_movement_system,
                    update_map_translation_system,
                    update_remove_stale_entities_system,
                ).chain(),
            )
            .add_systems(Update, update_game_camera_system);
    }
}
