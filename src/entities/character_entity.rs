use bevy::prelude::*;
use crate::components::health_component::*;
use crate::components::map_position_component::MapPositionComponent;
use crate::components::stats_component::*;

#[derive(Bundle)]
pub struct CharacterEntityBundle {
    pub transform: Transform,
    pub sprite: Sprite,
    pub stats: StatsComponent,
    pub health: HealthComponent,
    pub map_position: MapPositionComponent
}