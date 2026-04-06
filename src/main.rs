pub mod components;
pub mod enums;
pub mod systems;
mod constants;
pub mod entities;
pub mod resources;

use bevy::prelude::*;
use crate::systems::character_movement_system::character_movement_system;
use crate::systems::manage_turn_order_system::manage_turn_order_system;
use crate::systems::map_translation_system::map_translation_system;
use crate::systems::player_movement_system::player_movement_system;
use crate::systems::spawn_character_system::spawn_character_system;
use crate::systems::spawn_player_system::spawn_player_system;
use crate::systems::spawn_turn_order_system::spawn_turn_order_system;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, (spawn_turn_order_system, spawn_player_system, spawn_character_system).chain())
        .add_systems(Update, (manage_turn_order_system, player_movement_system, character_movement_system, map_translation_system).chain())
        .run();
}
