use crate::components::mover_component::MoverComponent;
use crate::enums::game_event_enum::GameEventEnum;
use crate::resources::current_game_event_resource::CurrentGameEventResource;
use crate::resources::game_log_resource::GameLogResource;
use crate::resources::map_resource::MapResource;
use bevy::prelude::*;
use crate::resources::game_event_queue_resource::GameEventQueueResource;

pub fn game_event_move_tile_intent_system(
    current_game_event_resource: Res<CurrentGameEventResource>,
    mut mover_query: Query<(&mut MoverComponent, Option<&Name>, Option<&Sprite>)>,
    mut game_event_queue_resource: ResMut<GameEventQueueResource>,
    mut map_resource: ResMut<MapResource>,
    mut logger: ResMut<GameLogResource>,
) {
    let event = match current_game_event_resource.get_event() {
        Some(event) => event,
        None => return,
    };

    let GameEventEnum::GameEventMoveTileIntent { entity } = event else {
        return;
    };

    let Ok((mut mover, name, sprite)) = mover_query.get_mut(*entity) else {
        return;
    };

    let entity_name;
    match Some(name) {
        Some(n) => entity_name = n.unwrap().to_string(),
        None => entity_name = entity.to_string() ,
    }

    let entity_color;
    match Some(sprite) {
        Some(s) => entity_color = s.unwrap().color,
        None => entity_color = Color::srgb(1., 1., 1.),
    }

    if !mover.can_move() {
        logger.log(format!(
            "Cannot move {:?} {:?}",
            entity_name,
            mover.get_move_speed()
        ));
        return;
    }

    // get map position
    let map_pos;

    match map_resource.get_position(*entity) {
        Some(pos) => map_pos = pos,
        None => return,
    }
    if mover.get_target_pos().is_none() {
        return;
    }

    let target_pos = mover.get_target_pos().unwrap();

    match map_resource.get_path(map_pos, target_pos) {
        Some(p) => {
            if p.len() > 2 {
                return;
            }
            map_resource.move_tile(*entity, target_pos);
            mover.increment_turn_movements();

            logger.log(format!(
                "{:?} moved, {:?} / {:?} used",
                entity_name,
                mover.get_turn_movements(),
                mover.get_move_speed()
            ));

            game_event_queue_resource.tile_moved(map_pos, entity_color)
        }
        None => return,
    }
}
