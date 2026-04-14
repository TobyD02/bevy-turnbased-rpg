use bevy::prelude::*;
use crate::components::mover_component::MoverComponent;
use crate::components::player_component::PlayerComponent;
use crate::enums::control_mapping_enum::ControlMappingEnum;
use crate::resources::game_event_queue_resource::GameEventQueueResource;
use crate::resources::game_log_resource::GameLogResource;
use crate::resources::map_resource::MapResource;
use crate::resources::turn_order_resource::TurnOrderResource;

pub fn update_player_movement_system(
    mut query: Query<(Entity, &mut MoverComponent), With<PlayerComponent>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut turn_order: ResMut<TurnOrderResource>,
    mut logger: ResMut<GameLogResource>,
    mut game_event_queue_resource: ResMut<GameEventQueueResource>,
    mut map_resource: ResMut<MapResource>,
) {
    if let Ok((entity, mut mover)) = query.single_mut(){
        match turn_order.get_active_entity() {
            Some(active_entity) => {
                if active_entity != entity {
                    return;
                }
            }
            None =>  {
                return
            },
        }

        let map_pos;

        match map_resource.get_position(entity) {
            Some(pos) => map_pos = pos,
            None => return,
        }

        if keys.just_pressed(ControlMappingEnum::PlayerMoveUp.keycode()) {
            // did_move = map_resource.move_tile(entity, map_pos.0, map_pos.1 + 1);
            mover.set_target_pos((map_pos.0, map_pos.1 + 1));
            game_event_queue_resource.move_intent(entity);
        }

        if keys.just_pressed(ControlMappingEnum::PlayerMoveDown.keycode()) {
            // did_move = map_resource.move_tile(entity, map_pos.0, map_pos.1 - 1);
            mover.set_target_pos((map_pos.0, map_pos.1 - 1));
            game_event_queue_resource.move_intent(entity);
        }

        if keys.just_pressed(ControlMappingEnum::PlayerMoveRight.keycode()) {
            // did_move = map_resource.move_tile(entity, map_pos.0 + 1, map_pos.1);
            mover.set_target_pos((map_pos.0 + 1, map_pos.1));
            game_event_queue_resource.move_intent(entity);
        }

        if keys.just_pressed(ControlMappingEnum::PlayerMoveLeft.keycode()) {
            // did_move = map_resource.move_tile(entity, map_pos.0 - 1, map_pos.1);
            mover.set_target_pos((map_pos.0 - 1, map_pos.1));
            game_event_queue_resource.move_intent(entity);
        }

        if keys.just_pressed(ControlMappingEnum::PlayerEndTurn.keycode()) {
            turn_order.commit_turn();
            mover.reset_turn_movements()
        }

    }
}