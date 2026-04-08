use std::fmt::format;
use bevy::prelude::*;
use crate::components::map_position_component::MapPositionComponent;
use crate::components::player_component::PlayerComponent;
use crate::enums::control_mapping_enum::ControlMappingEnum;
use crate::resources::game_log_resource::GameLogResource;
use crate::resources::turn_order_resource::TurnOrderResource;

pub fn update_player_movement_system(
    mut query: Query<(Entity, &mut MapPositionComponent), With<PlayerComponent>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut turn_order: ResMut<TurnOrderResource>,
    mut logger: ResMut<GameLogResource>
) {
    if let Ok((entity, mut map_position)) = query.single_mut(){
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

        if keys.just_pressed(ControlMappingEnum::PlayerMoveUp.keycode()) {
            map_position.move_up();
            key_was_pressed = true;
        }

        if keys.just_pressed(ControlMappingEnum::PlayerMoveDown.keycode()) {
            map_position.move_down();
            key_was_pressed = true;
        }

        if keys.just_pressed(ControlMappingEnum::PlayerMoveRight.keycode()) {
            map_position.move_right();
            key_was_pressed = true;
        }

        if keys.just_pressed(ControlMappingEnum::PlayerMoveLeft.keycode()) {
            map_position.move_left();
            key_was_pressed = true;
        }

        if key_was_pressed {
            logger.log(format!("Player moved to new map position | x: {}, y: {}", map_position.get_x(), map_position.get_y()));
            turn_order.end_turn()
        }

    }
}