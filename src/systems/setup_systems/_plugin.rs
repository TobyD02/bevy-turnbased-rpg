use crate::systems::setup_systems::setup_camera_system::setup_camera_system;
use crate::systems::setup_systems::setup_character_system::setup_character_system;
use crate::systems::setup_systems::setup_game_log_ui_system::setup_game_log_ui_system;
use crate::systems::setup_systems::setup_map_system::setup_map_system;
use crate::systems::setup_systems::setup_player_system::setup_player_system;
use crate::systems::setup_systems::setup_turn_order_system::setup_turn_order_system;
use crate::systems::setup_systems::setup_ui_camera_system::setup_ui_camera_system;
use bevy::prelude::*;
use crate::systems::setup_systems::setup_player_inventory_ui_system::setup_player_inventory_ui_system;
use crate::systems::setup_systems::setup_player_equipment_ui_system::setup_player_equipment_ui_system;

pub struct SetupSystemsPlugin;

impl Plugin for SetupSystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                setup_ui_camera_system,
                setup_game_log_ui_system,
                setup_player_inventory_ui_system,
                setup_player_equipment_ui_system,
                setup_map_system,
                setup_camera_system,
                setup_turn_order_system,
                setup_player_system,
                setup_character_system,
            )
                .chain()
        );
    }
}
