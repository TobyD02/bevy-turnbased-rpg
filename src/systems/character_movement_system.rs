use bevy::prelude::*;
use crate::components::character_component::CharacterComponent;
use crate::components::map_position_component::MapPositionComponent;
use crate::components::player_component::PlayerComponent;
use crate::resources::turn_order_resource::TurnOrderResource;

pub fn character_movement_system(
    mut query: Query<(Entity, &mut MapPositionComponent), (With<CharacterComponent>, Without<PlayerComponent>)>,
    player_query: Query<&MapPositionComponent, With<PlayerComponent>>,
    mut turn_order: ResMut<TurnOrderResource>

) {
    let (entity, mut character_map_position )= match query.single_mut() {
        Ok(pos) => pos,
        Err(_) => return,
    };

    match turn_order.get_active_entity() {
        Some(active_entity) => {
            if active_entity != entity {
                return;
            }
        }
        None => return,
    }

    let player_map_position = match player_query.single() {
        Ok(pos) => pos,
        Err(_) => return,
    };

    character_map_position.move_toward(player_map_position);
    turn_order.end_turn();
}
