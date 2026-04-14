use bevy::prelude::*;
use crate::components::character_component::CharacterComponent;
use crate::components::health_component::*;
use crate::components::movement_trail_component::MovementTrailComponent;
use crate::components::mover_component::MoverComponent;
use crate::components::stats_component::*;
use crate::components::turn_taker_component::TurnTakerComponent;
use crate::constants::TILE_SIZE;

#[derive(Bundle)]
pub struct MovementTrailEntityBundle {
    pub transform: Transform,
    pub sprite: Sprite,
    pub trail: MovementTrailComponent
}

impl Default for MovementTrailEntityBundle {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                ..Default::default()
            },
            trail: Default::default()
        }
    }
}
