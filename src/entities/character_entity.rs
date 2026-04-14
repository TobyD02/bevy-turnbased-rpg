use bevy::prelude::*;
use crate::components::attacker_component::AttackerComponent;
use crate::components::character_component::CharacterComponent;
use crate::components::health_component::*;
use crate::components::mover_component::MoverComponent;
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
    pub mover: MoverComponent,
    pub attacker: AttackerComponent,
    pub name: Name
}

impl Default for CharacterEntityBundle {
    fn default() -> Self {
        Self {
            name: Name::new("Some Character"),
            transform: Default::default(),
            sprite: Default::default(),
            stats: Default::default(),
            health: Default::default(),
            character: CharacterComponent,
            turn_taker: TurnTakerComponent,
            mover: MoverComponent::new(1),
            attacker: AttackerComponent,
        }
    }
}