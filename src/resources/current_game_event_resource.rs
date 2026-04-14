use bevy::prelude::*;
use crate::enums::game_event_enum::GameEventEnum;

#[derive(Resource)]
pub struct CurrentGameEventResource {
    game_event: Option<GameEventEnum>,
}

impl Default for CurrentGameEventResource {
    fn default() -> Self {
        Self {
            game_event: None,
        }
    }
}

impl CurrentGameEventResource {
    pub fn get_event(&self) -> &Option<GameEventEnum> {
        &self.game_event
    }

    pub fn set_event(&mut self, event: Option<GameEventEnum>) {
        self.game_event = event;
    }
    
}
