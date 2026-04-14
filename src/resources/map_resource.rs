use std::collections::{HashMap, HashSet};
use std::ops::{Index, IndexMut, Sub, Add};
use bevy::prelude::*;
use bevy::prelude::ops::sqrt;
use pathfinding::prelude::astar;
use crate::constants::{MAP_HEIGHT, MAP_WIDTH, TILE_SIZE};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct MapCoord {
    pub x: i32,
    pub y: i32,
}

impl MapCoord {
    pub fn length(&self) -> i32 {
        sqrt((self.x * self.x + self.y * self.y) as f32) as i32
    }
    pub fn up() -> MapCoord {
        MapCoord { x: 0, y: 1 }
    }

    pub fn down() -> MapCoord {
        MapCoord { x: 0, y: -1 }
    }

    pub fn left() -> MapCoord {
        MapCoord { x: -1, y: 0 }
    }

    pub fn right() -> MapCoord {
        MapCoord { x: 1, y: 0 }
    }
}

impl Sub for MapCoord {
    type Output = MapCoord;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add for MapCoord {
    type Output = MapCoord;

    fn add(self, other: MapCoord) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

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

    pub fn get_position(&self, entity: Entity) -> Option<MapCoord> {
        self.entity_positions.get(&entity).map(|&idx| {
            let width = MAP_WIDTH as usize;
            MapCoord {
                x: (idx % width) as i32,
                y: (idx / width) as i32,
            }
        })
    }

    pub fn map_to_global(map_coord: MapCoord) -> Vec2 {
        Vec2::new(
            map_coord.x as f32 * TILE_SIZE as f32,
            map_coord.y as f32 * TILE_SIZE as f32,
        )
    }

    fn pos_to_idx(&self, map_coord: MapCoord) -> Option<usize> {
        if map_coord.x < 0
            || map_coord.x >= MAP_WIDTH
            || map_coord.y < 0
            || map_coord.y >= MAP_HEIGHT
        {
            return None;
        }

        let width = MAP_WIDTH as usize;
        Some(map_coord.x as usize + map_coord.y as usize * width)
    }

    fn idx_to_pos(&self, idx: usize) -> MapCoord {
        let width = MAP_WIDTH as usize;
        MapCoord {
            x: (idx % width) as i32,
            y: (idx / width) as i32,
        }
    }

    fn is_walkable(&self, map_coord: MapCoord) -> bool {
        match self.pos_to_idx(map_coord) {
            Some(idx) => self.map_data[idx].is_none(),
            None => false,
        }
    }

    pub fn get_path(
        &self,
        start: MapCoord,
        goal: MapCoord,
    ) -> Option<Vec<MapCoord>> {

        let result = astar(
            &(start.x, start.y),

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
                        let next = MapCoord {
                            x: x + dx,
                            y: y + dy,
                        };

                        if next == goal || self.is_walkable(next) {
                            Some(((next.x, next.y), 1))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            },

            |&(x, y)| (x - goal.x).abs() + (y - goal.y).abs(),

            |&pos| pos == (goal.x, goal.y),
        );

        result.map(|(path, _)| {
            path.into_iter()
                .map(|(x, y)| MapCoord { x, y })
                .collect()
        })
    }

    pub fn is_position_free(&self, coord: MapCoord) -> bool {
        match self.pos_to_idx(coord) {
            Some(idx) => self.map_data[idx].is_none(),
            None => false,
        }
    }

    pub fn set_tile(&mut self, entity: Entity, coord: MapCoord) -> bool {
        let idx = match self.pos_to_idx(coord) {
            Some(idx) => idx,
            None => return false,
        };

        if let Some(existing) = self[idx] {
            self.stale_entities.insert(existing);
        }

        self.map_data[idx] = Some(entity);
        self.entity_positions.insert(entity, idx);
        self.changed_entities.insert(entity);
        true
    }

    pub fn set_background_tile(&mut self, entity: Entity, coord: MapCoord) -> bool {
        self.set_tile(entity, coord)
    }

    pub fn move_tile(&mut self, entity: Entity, coord: MapCoord) -> bool {
        let new_idx = match self.pos_to_idx(coord) {
            Some(idx) => idx,
            None => return false,
        };

        let current_idx = match self.entity_positions.get(&entity).copied() {
            Some(idx) => idx,
            None => return false,
        };

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
        let pos = match self.get_position(entity) {
            Some(pos) => pos,
            None => return false,
        };

        let world = Self::map_to_global(pos);

        transform.translation.x = world.x;
        transform.translation.y = world.y;

        true
    }
}
