use bevy::prelude::*;
use crate::components::character_component::CharacterComponent;
use crate::components::map_position_component::MapPositionComponent;
use crate::components::player_component::PlayerComponent;
use crate::enums::control_mapping_enum::ControlMappingEnum::AllowCharacterTurn;
use crate::resources::turn_order_resource::TurnOrderResource;

pub fn character_movement_system(
    mut query: Query<(Entity, &mut MapPositionComponent), (With<CharacterComponent>, Without<PlayerComponent>)>,
    mut turn_order: ResMut<TurnOrderResource>,
    player_query: Query<&MapPositionComponent, With<PlayerComponent>>,
    keys: Res<ButtonInput<KeyCode>>
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
}
