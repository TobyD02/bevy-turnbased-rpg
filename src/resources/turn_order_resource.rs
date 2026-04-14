use crate::enums::turn_group_enum::TurnGroupEnum;
use bevy::prelude::*;
use std::cmp::PartialEq;

#[derive(Debug, PartialEq)]
pub struct TurnEntity {
    pub(crate) initiative: i32,
    pub(crate) entity: Entity,
    turn_group: TurnGroupEnum,
}

#[derive(Resource, Debug)]
pub struct TurnOrderResource {
    entities: Vec<(TurnEntity)>,
    active_entity: usize,
    should_reorder: bool,
    turn_commited: bool,
    turn_iteration: i32,
}

impl Default for TurnOrderResource {
    fn default() -> Self {
        Self {
            entities: Vec::new(),
            active_entity: 0,
            should_reorder: true,
            turn_commited: false,
            turn_iteration: 0,
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

    pub fn get_entities(&self) -> &Vec<TurnEntity> {
        &self.entities
    }

    pub fn get_entities_mut(&mut self) -> &mut Vec<TurnEntity> {
        &mut self.entities
    }

    pub fn add_entity(&mut self, initiative: i32, entity: Entity, turn_group: TurnGroupEnum) {
        self.entities.push(TurnEntity {
            initiative,
            entity,
            turn_group,
        });
        self.entities
            .sort_by(|a, b| b.initiative.cmp(&a.initiative));
    }

    pub fn is_turn_committed(&self) -> bool {
        self.turn_commited
    }

    pub fn commit_turn(&mut self) {
        self.turn_commited = true;
    }

    pub fn get_active_entity(&self) -> Option<Entity> {
        self.entities.get(self.active_entity).map(|e| e.entity)
    }

    pub fn get_turn_iteration(&self) -> i32 {
        self.turn_iteration
    }

    pub fn get_active_turn_group(&self) -> Option<TurnGroupEnum> {
        self.entities.get(self.active_entity).map(|e| e.turn_group)
    }

    pub fn get_active_entities_in_turn_group(&self) -> Vec<Entity> {
        let mut result = Vec::new();

        if self.entities.is_empty() {
            return result;
        }

        let start = self.active_entity;
        let active_group = &self.entities[start].turn_group;

        for i in start..self.entities.len() {
            let turn_entity = &self.entities[i];

            if turn_entity.turn_group != *active_group {
                break;
            }

            result.push(turn_entity.entity);
        }

        result
    }

    pub fn end_turn(&mut self) {
        self.turn_commited = false;
        let len = self.entities.len();
        if len == 0 {
            return;
        }
        self.active_entity = (self.active_entity + 1);
        if self.active_entity >= len {
            self.active_entity = 0;
            self.turn_iteration += 1;
        }
    }

    pub fn sort(&mut self) {
        self.entities
            .sort_by(|a, b| b.initiative.cmp(&a.initiative));
    }
}
