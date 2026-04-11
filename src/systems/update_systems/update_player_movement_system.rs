use bevy::prelude::*;
use crate::components::player_component::PlayerComponent;
use crate::enums::control_mapping_enum::ControlMappingEnum;
use crate::resources::game_log_resource::GameLogResource;
use crate::resources::map_resource::MapResource;
use crate::resources::turn_order_resource::TurnOrderResource;

pub fn update_player_movement_system(
    mut query: Query<(Entity), With<PlayerComponent>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut turn_order: ResMut<TurnOrderResource>,
    mut logger: ResMut<GameLogResource>,
    mut map_resource: ResMut<MapResource>,
) {
    if let Ok((entity)) = query.single_mut(){
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

        let mut key_was_pressed = false;
        let map_pos;

        match map_resource.get_position(entity) {
            Some(pos) => map_pos = pos,
            None => return,
        }

        if keys.pressed(ControlMappingEnum::PlayerMoveUp.keycode()) {
            if !map_resource.is_position_free(map_pos.0, map_pos.1) {
                map_resource.move_tile(entity, map_pos.0, map_pos.1 + 1);
            }
            key_was_pressed = true;
        }

        if keys.pressed(ControlMappingEnum::PlayerMoveDown.keycode()) {
            if !map_resource.is_position_free(map_pos.0, map_pos.1) {
                map_resource.move_tile(entity, map_pos.0, map_pos.1 - 1);
            }
            key_was_pressed = true;
        }

        if keys.pressed(ControlMappingEnum::PlayerMoveRight.keycode()) {
            if !map_resource.is_position_free(map_pos.0, map_pos.1) {
                map_resource.move_tile(entity, map_pos.0 + 1, map_pos.1);
            }
            key_was_pressed = true;
        }

        if keys.pressed(ControlMappingEnum::PlayerMoveLeft.keycode()) {
            if !map_resource.is_position_free(map_pos.0, map_pos.1) {
                map_resource.move_tile(entity, map_pos.0 - 1, map_pos.1);
            }
            key_was_pressed = true;
        }

        if key_was_pressed {
            let new_pos = map_resource.get_position(entity).unwrap();
            logger.log(format!("Player moved to new map position | x: {}, y: {}", new_pos.0, new_pos.1));
            turn_order.end_turn()
        }

    }
}