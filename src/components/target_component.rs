use bevy::prelude::{Component, Entity};

#[derive(Component)]
pub struct TargetComponent {
    target: Option<Entity>
}

impl Default for TargetComponent {
    fn default() -> Self {
        Self {
            target: None,
        }
    }
}

impl TargetComponent {
    pub fn get_target_entity(&self) -> Option<Entity> {
        self.target
    }
    
    pub fn set_target_entity(&mut self, entity: Option<Entity>) {
        self.target = entity;
    }
}