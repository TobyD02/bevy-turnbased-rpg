use std::collections::VecDeque;
use bevy::prelude::*;
use crate::enums::game_event_enum::GameEventEnum;
use crate::resources::map_resource::MapCoord;

#[derive(Resource)]
pub struct GameEventQueueResource {
    event_queue: VecDeque<GameEventEnum>
}

impl Default for GameEventQueueResource {
    fn default() -> Self {
        Self {
            event_queue: VecDeque::new()
        }
    }
}

impl GameEventQueueResource {
    pub fn push(&mut self, event: GameEventEnum) {
        self.event_queue.push_back(event);
    }

    pub fn pop(&mut self) -> Option<GameEventEnum> {
        self.event_queue.pop_front()
    }

    pub fn move_intent(&mut self, entity: Entity) {
        self.push(GameEventEnum::GameEventMoveTileIntent {entity});
    }

    pub fn tile_moved(&mut self, from_tile: MapCoord, col: Color) {
        self.push(GameEventEnum::GameEventTileMoved{from_tile, col});
    }

    pub fn attack_intent(&mut self, entity: Entity, target: Entity) {
        // @todo
    }

}