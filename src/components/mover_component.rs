use bevy::prelude::*;
use crate::resources::map_resource::MapCoord;

#[derive(Component)]
pub struct MoverComponent {
    move_speed: i32,
    turn_movements: i32,
    target_pos: Option<MapCoord>
}

impl Default for MoverComponent {
    fn default() -> Self {
        Self {
            move_speed: 5,
            turn_movements: 0,
            target_pos: None
        }
    }
}

impl MoverComponent {
    pub fn new(move_speed: i32) -> Self {
        Self {
            move_speed,
            ..Default::default()
        }
    }
    pub fn get_move_speed(&self) -> i32{
        self.move_speed
    }
    pub fn get_target_pos(&self) -> Option<MapCoord> {
        self.target_pos.clone()
    }
    pub fn set_target_pos(&mut self, target_pos: MapCoord) {
        self.target_pos = Some(target_pos);
    }
    pub fn get_turn_movements(&self) -> i32 {
        self.turn_movements
    }

    pub fn reset_turn_movements(&mut self) {
        self.turn_movements = 0;
        self.target_pos = None;
    }

    pub fn can_move(&self) -> bool {
        self.turn_movements < self.move_speed
    }

    pub fn increment_turn_movements(&mut self) {
        self.turn_movements += 1;
    }
}