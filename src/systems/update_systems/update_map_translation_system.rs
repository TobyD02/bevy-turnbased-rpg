use bevy::prelude::*;
use crate::resources::game_log_resource::GameLogResource;
use crate::resources::map_resource::MapResource;

pub fn update_map_translation_system(
    mut map_resource: ResMut<MapResource>,
    mut entity_query: Query<(&mut Transform, Option<&Name>, Entity)>,
    mut logger: ResMut<GameLogResource>
) {
    for (e, _) in map_resource.get_changed_entity_positions().iter() {
        let (mut transform, name, entity) = entity_query.get_mut(*e).unwrap();
        map_resource.update_entity_transform(*e, &mut transform);

        // match name{
        //     Some(n) => logger.log(format!("Moved {:?}", name)),
        //     None => logger.log(format!("Moved {:?}", entity))
        // }
    }

    map_resource.clear_changed_entity_positions();
}