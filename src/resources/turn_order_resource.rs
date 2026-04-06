use std::cmp::Reverse;
use std::collections::{BTreeMap};
use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct TurnOrderResource {
    entities: BTreeMap<Reverse<i32>, Entity>,
    active_entity: usize,
    should_reorder: bool
}

impl Default for TurnOrderResource {
    fn default() -> Self{
        Self {
            entities: BTreeMap::new(),
            active_entity: 0,
            should_reorder: true
        }
    }
}

impl TurnOrderResource {
    pub fn is_should_reorder(&self) -> bool {
        self.should_reorder
    }
    pub fn set_should_reorder(&mut self, should_reorder: bool) {
        self.should_reorder = should_reorder;
    }
    pub fn get_entities(&self) -> &BTreeMap<Reverse<i32>, Entity> {
        &self.entities
    }
    pub fn get_entities_mut(&mut self) -> &mut BTreeMap<Reverse<i32>, Entity> {
        &mut self.entities
    }
    pub fn add_entity(&mut self, initiative: i32, entity: Entity) {
        self.entities.insert(Reverse(initiative), entity);
    }
    pub fn get_active_entity(&self) -> Option<Entity> {
        self.entities.values().nth(self.active_entity).copied()
    }

    pub fn end_turn(&mut self) {
        let len = self.entities.len();
        if len == 0 {
            return;
        }

        self.active_entity = (self.active_entity+ 1) % len;
    }
}