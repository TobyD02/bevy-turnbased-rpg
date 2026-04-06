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
    pub fn update_transform(&self, transform: &mut Transform) {
        transform.translation.x = self.x as f32 * MAP_WIDTH as f32;
        transform.translation.y = self.y as f32 * MAP_WIDTH as f32;
    }
    pub fn get_x(&self) -> i32 {
        self.x
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }
    pub fn move_up(&mut self) {
        self.y += 1;
    }
    pub fn move_toward(&mut self, target_map_position: &MapPositionComponent) {
        let direction_x = (target_map_position.x - self.x).signum();
        let direction_y = (target_map_position.y - self.y).signum();

        self.x += direction_x;
        self.y += direction_y;
    }
    pub fn move_down(&mut self) {
        self.y -= 1;
    }
    pub fn move_right(&mut self) {
        self.x += 1;
    }
    pub fn move_left(&mut self) {
        self.x -= 1;
    }
}
