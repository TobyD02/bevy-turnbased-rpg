use std::cmp::PartialEq;
use crate::components::character_component::CharacterComponent;
use crate::components::player_component::PlayerComponent;
use crate::enums::control_mapping_enum::ControlMappingEnum::AllowCharacterTurn;
use crate::resources::game_log_resource::GameLogResource;
use crate::resources::map_resource::MapResource;
use crate::resources::turn_order_resource::TurnOrderResource;
use bevy::prelude::*;
use crate::components::mover_component::MoverComponent;
use crate::enums::turn_group_enum::TurnGroupEnum;
use crate::resources::game_event_queue_resource::GameEventQueueResource;

pub fn update_character_movement_system(
    mut active_query: Query<(&Name, &mut MoverComponent), With<CharacterComponent>>,
    mut turn_order: ResMut<TurnOrderResource>,
    player_query: Query<Entity, With<PlayerComponent>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut logger: ResMut<GameLogResource>,
    map_resource: ResMut<MapResource>,
    mut game_event_queue_resource: ResMut<GameEventQueueResource>,
) {
    // if !keys.just_pressed(AllowCharacterTurn.keycode()) {
    //     return;
    // }

    match turn_order.get_active_turn_group() {
        Some(g) => {
            if g != TurnGroupEnum::Enemy {
                return
            }
        }
        _ => return
    }

    if Some(turn_order.get_active_entity()).is_some()
    {
        if (turn_order.is_turn_committed()) {
            return
        }

        let active_entity = turn_order.get_active_entity().unwrap();
        let (active_name, mut active_mover) = match active_query.get_mut(active_entity) {
            Ok((name, mover)) => (name.clone(), mover),
            Err(_) => return,
        };

        if !active_mover.can_move() {
            turn_order.commit_turn();
            active_mover.reset_turn_movements();
            return
        }

        let active_pos;
        match map_resource.get_position(active_entity) {
            Some(pos) => active_pos = pos,
            None => {
                logger.log(format!("Failed to find entity in map {:?}", active_entity));
                turn_order.end_turn();
                return;
            }
        }

        let player_pos;
        match player_query.single() {
            Ok(e) => player_pos = map_resource.get_position(e).unwrap(),
            Err(_) => return,
        };

        let next_pos;
        match map_resource.get_path(active_pos, player_pos) {
            Some(p) => {
                next_pos = p[1];
            },
            None => {
                turn_order.end_turn();
                logger.log(format!("Character {:?} didnt move", active_name));
                return;
            }
        }

        active_mover.set_target_pos((next_pos.0, next_pos.1));
        game_event_queue_resource.move_intent(active_entity);
    }
}
