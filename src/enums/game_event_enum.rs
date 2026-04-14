use bevy::prelude::*;
use crate::resources::map_resource::MapCoord;

#[derive(Debug)]
pub enum GameEventEnum {
    GameEventMoveTileIntent {
        entity: Entity,
    },
    GameEventTileMoved {
        from_tile: MapCoord,
        col: Color
    },
}
