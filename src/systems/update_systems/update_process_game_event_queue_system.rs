use crate::resources::current_game_event_resource::CurrentGameEventResource;
use crate::resources::game_event_queue_resource::GameEventQueueResource;
use crate::resources::turn_order_resource::TurnOrderResource;
use bevy::prelude::*;

pub fn update_process_game_queue_system(
    mut game_event_queue_resource: ResMut<GameEventQueueResource>,
    mut current_game_event_resource: ResMut<CurrentGameEventResource>,
    mut turn_order_resource: ResMut<TurnOrderResource>,
) {

    if let Some(event) = game_event_queue_resource.pop() {
        current_game_event_resource.set_event(Some(event))
    } else {
        current_game_event_resource.set_event(None);
        if turn_order_resource.is_turn_committed() {
            turn_order_resource.end_turn() // sets turn_commited to false, and sets turn to next entity
        }
    }
}
