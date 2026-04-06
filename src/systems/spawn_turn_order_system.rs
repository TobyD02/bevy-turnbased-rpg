use bevy::prelude::Commands;
use crate::resources::turn_order_resource::TurnOrderResource;

pub fn spawn_turn_order_system (
    mut commands: Commands,
) {
    commands.insert_resource(TurnOrderResource::default())
}