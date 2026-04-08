use crate::components::character_component::CharacterComponent;
use crate::components::map_position_component::MapPositionComponent;
use crate::components::player_component::PlayerComponent;
use crate::enums::control_mapping_enum::ControlMappingEnum::AllowCharacterTurn;
use crate::resources::game_log_resource::GameLogResource;
use crate::resources::turn_order_resource::TurnOrderResource;
use bevy::prelude::*;

pub fn update_character_movement_system(
    mut query: Query<
        (Entity, &mut MapPositionComponent),
        (With<CharacterComponent>, Without<PlayerComponent>),
    >,
    mut turn_order: ResMut<TurnOrderResource>,
    player_query: Query<&MapPositionComponent, With<PlayerComponent>>,
    keys: Res<ButtonInput<KeyCode>>,
    mut logger: ResMut<GameLogResource>,
) {
    if !keys.pressed(AllowCharacterTurn.keycode()) {
        return;
    }

    let active_entity = match turn_order.get_active_entity() {
        Some(e) => e,
        None => return,
    };

    let Ok((_, mut character_map_position)) = query.get_mut(active_entity) else {
        return; // active entity isn't a character (e.g. it's the player's turn)
    };

    let player_map_position = match player_query.single() {
        Ok(pos) => pos,
        Err(_) => return,
    };

    character_map_position.move_toward(player_map_position);
    turn_order.end_turn();
    logger.log(format!(
        "Character {:?} moved | x: {:?}, y: {:?}",
        active_entity,
        character_map_position.get_x(),
        character_map_position.get_y()
    ));
}
