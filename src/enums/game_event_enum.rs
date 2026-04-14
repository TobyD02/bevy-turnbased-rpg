use bevy::prelude::*;

#[derive(Debug)]
pub enum GameEventEnum {
    GameEventMoveTileIntent {
        entity: Entity,
    },
    GameEventTileMoved {
        from_tile: (i32, i32),
        col: Color
    },
}
