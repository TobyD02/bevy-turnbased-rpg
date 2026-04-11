use bevy::prelude::{Commands, Entity, Name, Query, ResMut, Transform};
use crate::resources::game_log_resource::GameLogResource;
use crate::resources::map_resource::MapResource;

pub fn update_remove_stale_entities_system(
    mut commands: Commands,
    mut map_resource: ResMut<MapResource>,
    mut logger: ResMut<GameLogResource>
) {
    for stale_entity in map_resource.get_stale_entities().iter() {
        commands.entity(*stale_entity).despawn();

        logger.log(format!("Removed stale entity {:?}", stale_entity))
    }
    map_resource.clear_stale_entities();
}