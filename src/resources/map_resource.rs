use std::collections::{HashMap, HashSet};
use std::ops::{Index, IndexMut};
use bevy::prelude::*;
use pathfinding::prelude::astar;
use crate::constants::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};

#[derive(Resource, Debug)]
pub struct MapResource {
    map_data: Vec<Option<Entity>>,
    entity_positions: HashMap<Entity, usize>,
    changed_entities: HashSet<Entity>,
    stale_entities: HashSet<Entity>,
}

impl Default for MapResource {
    fn default() -> Self {
        Self {
            map_data: vec![None; MAP_WIDTH as usize * MAP_HEIGHT as usize],
            entity_positions: HashMap::new(),
            changed_entities: HashSet::new(),
            stale_entities: HashSet::new(),
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

// @todo: Add "free_tile_exists" method, which returns false when all tiles are assigned
impl MapResource {
    pub fn get_entity_positions(&self) -> HashMap<Entity, usize> {
        self.entity_positions.clone()
    }

    pub fn get_stale_entities(&self) -> HashSet<Entity> {
        self.stale_entities.clone()
    }

    pub fn clear_stale_entities(&mut self) {
        self.stale_entities.clear();
    }


    pub fn get_changed_entity_positions(&self) -> HashMap<Entity, usize> {
        self.entity_positions
            .iter()
            .filter(|(entity, _)| self.changed_entities.contains(entity))
            .map(|(entity, &pos)| (*entity, pos))
            .collect()
    }

    pub fn clear_changed_entity_positions(&mut self) {
        self.changed_entities.clear();
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

    fn idx_to_pos(&self, idx: usize) -> (i32, i32) {
        let width = MAP_WIDTH as usize;
        let x = (idx % width) as i32;
        let y = (idx / width) as i32;
        (x, y)
    }

    fn is_walkable(&self, x: i32, y: i32) -> bool {
        match self.pos_to_idx(x, y) {
            Some(idx) => self.map_data[idx].is_none(),
            None => false,
        }
    }
    pub fn get_path(
        &self,
        start: (i32, i32),
        goal: (i32, i32),
    ) -> Option<Vec<(i32, i32)>> {

        let result = astar(
            &start,

            // neighbors
            |&(x, y)| {
                let directions = [
                    (1, 0),
                    (-1, 0),
                    (0, 1),
                    (0, -1),
                ];

                directions
                    .iter()
                    .filter_map(|(dx, dy)| {
                        let nx = x + dx;
                        let ny = y + dy;

                        if (nx, ny) == goal || self.is_walkable(nx, ny) {
                            Some(((nx, ny), 1)) // cost = 1
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            },

            // heuristic (Manhattan distance)
            |&(x, y)| (x - goal.0).abs() + (y - goal.1).abs(),

            // goal check
            |&pos| pos == goal,
        );

        result.map(|(path, _cost)| path)
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

        if self[idx].is_some() {
            self.stale_entities.insert(entity);
        }

        self.map_data[idx] = Some(entity);
        self.entity_positions.insert(entity, idx);
        self.changed_entities.insert(entity);
        true
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

            self.changed_entities.insert(entity);

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
