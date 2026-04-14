use bevy::ecs::schedule::ScheduleConfigs;
use bevy::ecs::system::ScheduleSystem;
use crate::systems::update_systems::update_game_event_systems::game_event_move_tile_intent_system::game_event_move_tile_intent_system;
use bevy::prelude::*;
use crate::systems::update_systems::update_game_event_systems::game_event_tile_moved_system::game_event_tile_moved_system;

pub fn update_game_event_systems() -> ScheduleConfigs<ScheduleSystem> {
    (
        game_event_move_tile_intent_system,
        game_event_tile_moved_system
    ).chain()
}
