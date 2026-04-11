use std::collections::HashMap;
use std::ops::{Index, IndexMut};
use bevy::prelude::*;
use crate::constants::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};

//@todo Mark when an entity has moved
struct EntityPositions {
    map_data_index: usize,
    has_changed: bool
}

#[derive(Resource, Debug)]
pub struct MapResource {
    map_data: [Option<Entity>; MAP_WIDTH as usize * MAP_HEIGHT as usize],
    entity_positions: HashMap<Entity, usize>,
}

impl Default for MapResource {
    fn default() -> Self {
        Self {
            map_data: [None; MAP_WIDTH as usize * MAP_HEIGHT as usize],
            entity_positions: HashMap::new(),
        }
    }
}

impl Index<usize> for MapResource {
    type Output = Option<Entity>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.map_data[index]
    }
}

impl IndexMut<usize> for MapResource {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.map_data[index]
    }
}

impl MapResource {
    pub fn get_entity_positions(&self) -> HashMap<Entity, usize> {
        self.entity_positions.clone()
    }

    pub fn get_position(&self, entity: Entity) -> Option<(i32, i32)> {
        self.entity_positions.get(&entity).map(|&idx| {
            let width = MAP_WIDTH as usize;

            let x = (idx % width) as i32;
            let y = (idx / width) as i32;

            (x, y)
        })
    }

    fn pos_to_idx(&self, x: i32, y: i32) -> Option<usize> {
        if x < 0 || x >= MAP_WIDTH || y < 0 || y >= MAP_HEIGHT {
            return None;
        }

        let width = MAP_WIDTH as usize;
        Some(x as usize + y as usize * width)
    }

    pub fn is_position_free(&self, x: i32, y: i32) -> bool {
        match self.pos_to_idx(x, y) {
            Some(idx) => self.map_data[idx].is_none(),
            None => false,
        }
    }

    pub fn set_tile(&mut self, entity: Entity, x: i32, y: i32) -> bool {
        let idx = match self.pos_to_idx(x, y) {
            Some(idx) => idx,
            None => {
                return false;
            }
        };

        if self.map_data[idx].is_none() {
            self.map_data[idx] = Some(entity);
            self.entity_positions.insert(entity, idx);
            return true;
        }

        false
    }

    pub fn move_tile(&mut self, entity: Entity, x: i32, y: i32) -> bool {
        let new_idx = match self.pos_to_idx(x, y) {
            Some(idx) => idx,
            None => {
                return false;
            }
        };

        let current_idx = match self.entity_positions.get(&entity).copied() {
            Some(idx) => idx,
            None => {
                return false;
            }
        };

        // Only move if destination is free
        if self.map_data[new_idx].is_none() {
            self.map_data[new_idx] = Some(entity);
            self.map_data[current_idx] = None;
            self.entity_positions.insert(entity, new_idx);

            return true;
        }

        false
    }

    pub fn update_entity_transform(&self, entity: Entity, transform: &mut Transform) -> bool {
        let (x, y) = match self.get_position(entity) {
            Some(pos) => pos,
            None => return false,
        };

        // NOTE: You probably want TILE_SIZE here instead of MAP_WIDTH
        transform.translation.x = x as f32 * TILE_SIZE as f32;
        transform.translation.y = y as f32 * TILE_SIZE as f32;

        true
    }
}
