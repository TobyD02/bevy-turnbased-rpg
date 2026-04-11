use bevy::prelude::*;
use crate::components::stats_component::StatsComponent;
use crate::components::wall_component::WallComponent;

#[derive(Bundle)]
pub struct WallEntityBundle {
    pub wall: WallComponent,
    pub transform: Transform,
    pub sprite: Sprite,
    pub stats: StatsComponent,
}

impl Default for WallEntityBundle {
    fn default() -> Self {
        Self {
            wall: WallComponent,
            transform: Default::default(),
            sprite: Default::default(),
            stats: Default::default(),
        }
    }
}