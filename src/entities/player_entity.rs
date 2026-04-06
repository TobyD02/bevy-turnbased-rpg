use bevy::prelude::*;
use crate::components::health_component::*;
use crate::components::map_position_component::MapPositionComponent;
use crate::components::player_component::PlayerComponent;
use crate::components::stats_component::*;
use crate::components::turn_taker_component::TurnTakerComponent;

#[derive(Bundle)]
pub struct PlayerEntityBundle {
    pub transform: Transform,
    pub sprite: Sprite,
    pub stats: StatsComponent,
    pub health: HealthComponent,
    pub map_position: MapPositionComponent,
    pub player: PlayerComponent,
    pub turn_taker: TurnTakerComponent
}

impl Default for PlayerEntityBundle {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            sprite: Default::default(),
            stats: Default::default(),
            health: Default::default(),
            map_position: Default::default(),
            player: PlayerComponent,
            turn_taker: TurnTakerComponent,
        }
    }
}
