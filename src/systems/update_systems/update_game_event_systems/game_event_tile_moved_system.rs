use bevy::prelude::*;
use crate::components::movement_trail_component::MovementTrailComponent;
use crate::constants::TILE_SIZE;
use crate::entities::movement_trail_entity::MovementTrailEntityBundle;
use crate::enums::game_event_enum::GameEventEnum;
use crate::enums::map_layer_enum::MapLayerEnum::{MapLayerBackground, MapLayerTiles};
use crate::resources::current_game_event_resource::CurrentGameEventResource;
use crate::resources::map_resource::MapResource;
use crate::resources::turn_order_resource::TurnOrderResource;

pub fn game_event_tile_moved_system(
    current_game_event_resource: Res<CurrentGameEventResource>,
    turn_order_resource: Res<TurnOrderResource>,
    mut commands: Commands,
) {
    let event = match current_game_event_resource.get_event() {
        Some(event) => event,
        None => return,
    };

    let GameEventEnum::GameEventTileMoved { from_tile, col } = event else {
        return;
    };

    let transform_xy = MapResource::map_to_global(*from_tile);
    commands.spawn(
        MovementTrailEntityBundle {
            transform: Transform {
                translation: Vec3::new(transform_xy.x, transform_xy.y, MapLayerBackground.float()),
                ..Default::default()
            },
            trail: MovementTrailComponent::new(turn_order_resource.get_turn_iteration()),
            sprite: Sprite {
                custom_size: Some(Vec2::new(TILE_SIZE as f32, TILE_SIZE as f32)),
                color: *col,
                ..Default::default()
            },
            ..Default::default()
        },
    );
}
