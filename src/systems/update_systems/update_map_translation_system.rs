use bevy::prelude::*;
use crate::components::map_position_component::MapPositionComponent;

pub fn update_map_translation_system(
    mut query: Query<(&MapPositionComponent, &mut Transform)>
) {
    for (map_position, mut transform) in query.iter_mut() {
        map_position.update_transform(&mut *transform);
    }
}