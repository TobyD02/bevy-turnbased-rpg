use bevy::prelude::*;
#[derive(Component)]
pub struct HealthComponent {
    pub max_health: f32,
    pub current_health: f32
}

impl Default for HealthComponent {
    fn default() -> Self {
        Self {
            max_health: 20.,
            current_health: 10.
        }
    }
}