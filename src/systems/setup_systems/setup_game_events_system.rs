use bevy::prelude::*;
use crate::resources::current_game_event_resource::CurrentGameEventResource;
use crate::resources::game_event_queue_resource::GameEventQueueResource;

pub fn setup_game_events_system(
    mut commands: Commands,
) {
    commands.insert_resource(GameEventQueueResource::default());
    commands.insert_resource(CurrentGameEventResource::default())
}