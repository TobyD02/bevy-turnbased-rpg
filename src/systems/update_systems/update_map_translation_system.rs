use bevy::prelude::*;
use crate::resources::map_resource::MapResource;

// @todo Only update entities that have moved
pub fn update_map_translation_system(
    mut map_resource: ResMut<MapResource>,
    mut entity_query: Query<&mut Transform>,
) {
    for (e, _) in map_resource.get_entity_positions().iter() {
        let mut transform = entity_query.get_mut(*e).unwrap();
        map_resource.update_entity_transform(*e, &mut transform);
    }
}