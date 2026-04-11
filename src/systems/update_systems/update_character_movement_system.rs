use crate::components::character_component::CharacterComponent;
use crate::components::player_component::PlayerComponent;
use crate::enums::control_mapping_enum::ControlMappingEnum::AllowCharacterTurn;
use crate::resources::game_log_resource::GameLogResource;
use crate::resources::turn_order_resource::TurnOrderResource;
use bevy::prelude::*;
use crate::resources::map_resource::MapResource;

pub fn update_character_movement_system(
    active_query: Query<&Name, With<CharacterComponent>>,
    mut turn_order: ResMut<TurnOrderResource>,
    player_query: Query<Entity, With<PlayerComponent>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut logger: ResMut<GameLogResource>,
    mut map_resource: ResMut<MapResource>,
) {
    if !keys.pressed(AllowCharacterTurn.keycode()) {
        return;
    }

    let active_entity = match turn_order.get_active_entity() {
        Some(e) => e,
        None => return,
    };

    let active_name;
    match active_query.get(active_entity) {
        Ok(name) => {
            active_name = name.clone();
        }
        Err(_) => {
            return
        }
    }

    let active_pos;
    match map_resource.get_position(active_entity) {
        Some(pos) => active_pos = pos,
        None => {
            logger.log(format!("Failed to find entity in map {:?}", active_entity));
            turn_order.end_turn();
            return
        }
    }

    let player_pos;
    match player_query.single() {
        Ok(e) => player_pos = map_resource.get_position(e).unwrap(),
        Err(_) => return,
    };

    let direction_x = (player_pos.0 - active_pos.0).signum();
    let direction_y = (player_pos.1 - active_pos.1).signum();
    map_resource.move_tile(active_entity, active_pos.0 + direction_x, active_pos.1 + direction_y);
    turn_order.end_turn();

    let new_position = map_resource.get_position(active_entity).unwrap();

    logger.log(format!(
        "Character {:?} moved | x: {:?}, y: {:?}",
        active_name,
        new_position.0,
        new_position.1,
    ));
}
