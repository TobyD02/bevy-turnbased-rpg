use bevy::prelude::*;
use crate::components::health_component::*;
use crate::components::player_component::PlayerComponent;
use crate::components::stats_component::*;
use crate::components::turn_taker_component::TurnTakerComponent;

#[derive(Bundle)]
pub struct PlayerEntityBundle {
    pub transform: Transform,
    pub sprite: Sprite,
    pub stats: StatsComponent,
    pub health: HealthComponent,
    pub player: PlayerComponent,
    pub turn_taker: TurnTakerComponent,
    pub name: Name
}

impl Default for PlayerEntityBundle {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            sprite: Default::default(),
            stats: Default::default(),
            health: Default::default(),
            player: PlayerComponent,
            turn_taker: TurnTakerComponent,
            name: Name::new("Player")
        }
    }
}
