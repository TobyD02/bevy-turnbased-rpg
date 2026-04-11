use bevy::prelude::*;
use crate::resources::map_resource::MapResource;

pub fn setup_map_system(
    mut commands: Commands
) {
    commands.insert_resource(MapResource::default())
}