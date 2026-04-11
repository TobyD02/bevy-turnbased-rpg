use std::cmp::PartialEq;
use crate::enums::turn_group_enum::TurnGroupEnum;
use bevy::prelude::*;

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
}

impl Default for TurnOrderResource {
    fn default() -> Self {
        Self {
            entities: Vec::new(),
            active_entity: 0,
            should_reorder: true,
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

    pub fn get_active_entity(&self) -> Option<Entity> {
        self.entities.get(self.active_entity).map(|e| e.entity)
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
        let len = self.entities.len();
        if len == 0 {
            return;
        }
        self.active_entity = (self.active_entity + 1) % len;
    }

    pub fn sort(&mut self) {
        self
            .entities
            .sort_by(|a, b| b.initiative.cmp(&a.initiative));
    }
}
