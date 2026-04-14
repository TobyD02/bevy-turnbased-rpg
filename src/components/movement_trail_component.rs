use bevy::prelude::*;

#[derive(Component)]
pub struct MovementTrailComponent {
    pub decay: i32,
    pub decay_max: i32,
    pub start_turn_iter: i32
}

impl Default for MovementTrailComponent {
    fn default() -> Self {
        Self {
            decay: 0,
            decay_max: 5,
            start_turn_iter: 0
        }
    }
}

impl MovementTrailComponent {
    pub fn new(start_turn_iter: i32) -> Self {
        Self {
            start_turn_iter,
            ..Default::default()
        }
    }
}