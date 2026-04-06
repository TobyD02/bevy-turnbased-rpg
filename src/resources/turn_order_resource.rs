use bevy::prelude::*;

#[derive(Resource, Debug)]
pub struct TurnOrderResource {
    entities: Vec<(i32, Entity)>,
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

    pub fn get_entities(&self) -> &[(i32, Entity)] {
        &self.entities
    }

    pub fn get_entities_mut(&mut self) -> &mut Vec<(i32, Entity)> {
        &mut self.entities
    }

    pub fn add_entity(&mut self, initiative: i32, entity: Entity) {
        self.entities.push((initiative, entity));
        self.entities.sort_by(|(a, _), (b, _)| b.cmp(a));
    }

    pub fn get_active_entity(&self) -> Option<Entity> {
        self.entities.get(self.active_entity).map(|(_, e)| *e)
    }

    pub fn end_turn(&mut self) {
        let len = self.entities.len();
        if len == 0 {
            return;
        }
        self.active_entity = (self.active_entity + 1) % len;
    }

    pub fn sort(&mut self) {
        &self.entities.sort_by(|(a, _), (b, _)| b.cmp(a));
    }
}
