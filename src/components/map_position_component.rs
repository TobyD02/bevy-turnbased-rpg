use crate::constants::MAP_WIDTH;
use bevy::prelude::*;

#[derive(Component)]
pub struct MapPositionComponent {
    x: i32,
    y: i32,
}

impl Default for MapPositionComponent {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

impl MapPositionComponent {
    fn update_transform(&self, transform: &mut Transform) {
        transform.translation.x = self.x as f32 * MAP_WIDTH as f32;
        transform.translation.y = self.y as f32 * MAP_WIDTH as f32;
    }
}
