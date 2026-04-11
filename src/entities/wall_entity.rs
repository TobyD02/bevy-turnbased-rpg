use bevy::prelude::*;
use crate::components::health_component::HealthComponent;
use crate::components::wall_component::WallComponent;

#[derive(Bundle)]
pub struct WallEntityBundle {
    pub wall: WallComponent,
    pub transform: Transform,
    pub sprite: Sprite,
    pub health: HealthComponent,
}

impl Default for WallEntityBundle {
    fn default() -> Self {
        Self {
            wall: WallComponent,
            transform: Default::default(),
            sprite: Default::default(),
            health: Default::default(),
        }
    }
}