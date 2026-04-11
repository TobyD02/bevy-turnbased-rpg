use std::cmp::PartialEq;
use crate::components::character_component::CharacterComponent;
use crate::components::player_component::PlayerComponent;
use crate::enums::control_mapping_enum::ControlMappingEnum::AllowCharacterTurn;
use crate::resources::game_log_resource::GameLogResource;
use crate::resources::map_resource::MapResource;
use crate::resources::turn_order_resource::TurnOrderResource;
use bevy::prelude::*;
use crate::enums::turn_group_enum::TurnGroupEnum;

pub fn update_character_movement_system(
    active_query: Query<&Name, With<CharacterComponent>>,
    mut turn_order: ResMut<TurnOrderResource>,
    player_query: Query<Entity, With<PlayerComponent>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut logger: ResMut<GameLogResource>,
    mut map_resource: ResMut<MapResource>,
) {
    // if !keys.pressed(AllowCharacterTurn.keycode()) {
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

    for active_entity in turn_order.get_active_entities_in_turn_group()
    {
        let active_name;
        match active_query.get(active_entity) {
            Ok(name) => {
                active_name = name.clone();
            }
            Err(_) => return,
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

        map_resource.move_tile(
            active_entity,
            next_pos.0,
            next_pos.1,
        );
        turn_order.end_turn();

        let new_position = map_resource.get_position(active_entity).unwrap();

        logger.log(format!(
            "Character {:?} moved | x: {:?}, y: {:?}",
            active_name, new_position.0, new_position.1,
        ));
    }
}
