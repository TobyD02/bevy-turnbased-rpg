use bevy::prelude::*;
use crate::components::character_component::CharacterComponent;
use crate::components::health_component::*;
use crate::components::stats_component::*;
use crate::components::turn_taker_component::TurnTakerComponent;

#[derive(Bundle)]
pub struct CharacterEntityBundle {
    pub transform: Transform,
    pub sprite: Sprite,
    pub stats: StatsComponent,
    pub health: HealthComponent,
    pub character: CharacterComponent,
    pub turn_taker: TurnTakerComponent,
    pub name: Name
}

impl Default for CharacterEntityBundle {
    fn default() -> Self {
        Self {
            transform: Default::default(),
            sprite: Default::default(),
            stats: Default::default(),
            health: Default::default(),
            character: CharacterComponent,
            turn_taker: TurnTakerComponent,
            name: Name::new("Some Character")
        }
    }
}